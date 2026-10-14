use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, TryRecvError};
use std::time::{Duration, Instant};

use oxurack::{CoreCommand, MidiDelivery, MidiOutputEvent, Oxurack, ParameterValue};

use super::input::Input;
use super::{playback, workbench};

/// The rack's authoritative fixed sample cadence is 1 kHz.
const SAMPLE_PERIOD: Duration = Duration::from_millis(1);
/// Bound idle-input latency without spinning while the rack is paused.
const PAUSED_POLL: Duration = Duration::from_millis(20);
/// Bound command bursts so a queued terminal stream cannot starve due samples.
const COMMANDS_PER_TURN: usize = 8;

pub(super) fn run<S, W, E>(
    rack: &mut Oxurack,
    patch: &Path,
    session: Option<&mut playback::Session<S>>,
    trace: &mut W,
    diagnostics: &mut E,
    receiver: &Receiver<Input>,
    cancellation: &AtomicBool,
) -> Result<(), Box<dyn Error>>
where
    S: playback::MidiSender,
    W: Write,
    E: Write,
{
    let mut session = session;
    let mut deadline = Instant::now() + SAMPLE_PERIOD;
    let mut pending = None;
    let mut failure = LiveFailure::default();

    loop {
        let mut clock = Instant::now;
        match owner_turn(
            rack,
            patch,
            &mut session,
            trace,
            diagnostics,
            receiver,
            cancellation,
            &mut pending,
            &mut deadline,
            &mut clock,
        ) {
            Ok(OwnerTurn::Continue) => {}
            Ok(OwnerTurn::Exit) => break,
            Ok(OwnerTurn::Wait(timeout)) => match receiver.recv_timeout(timeout) {
                Ok(input) => pending = Some(input),
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => pending = Some(Input::Disconnected),
            },
            Err(error) => {
                failure.push("owner turn", error);
                break;
            }
        }
    }

    rack.stop();
    if let Err(error) = rack.reset() {
        failure.push("shutdown reset", boxed(error.to_string()));
    }
    if let Err(error) = deliver_drained(rack, &mut session, trace) {
        failure.push("shutdown MIDI delivery", error);
    }
    if let Some(session) = &mut session {
        let cleanup = session.finish().to_vec();
        for error in &cleanup {
            failure.push("MIDI session finish", boxed(error.clone()));
            if let Err(diagnostic_error) = writeln!(
                diagnostics,
                "rack-live: MIDI shutdown cleanup failed: {error}"
            ) {
                failure.push(
                    "MIDI cleanup diagnostic write",
                    boxed(diagnostic_error.to_string()),
                );
            }
        }
    }
    if let Err(error) = trace.flush() {
        failure.push("MIDI trace flush", boxed(error.to_string()));
    }
    if let Err(error) = diagnostics.flush() {
        failure.push("diagnostic flush", boxed(error.to_string()));
    }
    if failure.is_empty() {
        Ok(())
    } else {
        Err(Box::new(failure))
    }
}

#[derive(Default, Debug)]
struct LiveFailure {
    causes: Vec<FailureCause>,
}

#[derive(Debug)]
struct FailureCause {
    stage: String,
    error: Box<dyn Error>,
}

impl LiveFailure {
    fn push(&mut self, stage: impl Into<String>, error: Box<dyn Error>) {
        self.causes.push(FailureCause {
            stage: stage.into(),
            error,
        });
    }

    fn is_empty(&self) -> bool {
        self.causes.is_empty()
    }
}

impl fmt::Display for LiveFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, cause) in self.causes.iter().enumerate() {
            if index == 0 {
                write!(formatter, "{}: {}", cause.stage, cause.error)?;
            } else {
                write!(formatter, "; {}: {}", cause.stage, cause.error)?;
            }
        }
        Ok(())
    }
}

impl Error for LiveFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.causes.first().map(|cause| cause.error.as_ref())
    }
}

pub(super) fn combine_results(
    primary: Result<(), Box<dyn Error>>,
    cleanup: Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    match (primary, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
        (Err(primary), Err(cleanup)) => {
            let mut failure = LiveFailure::default();
            failure.push("live session", primary);
            failure.push("stdin reader shutdown", cleanup);
            Err(Box::new(failure))
        }
    }
}

#[derive(Clone, Copy)]
enum InputAction {
    Continue,
    Started,
    Exit,
}

struct CommandBatch {
    consumed: usize,
    exit: bool,
    terminal_error: Option<Box<dyn Error>>,
}

fn process_ready_commands(
    first: Option<Input>,
    receiver: &Receiver<Input>,
    mut apply: impl FnMut(Input) -> Result<InputAction, Box<dyn Error>>,
) -> CommandBatch {
    let mut batch = CommandBatch {
        consumed: 0,
        exit: false,
        terminal_error: None,
    };
    let mut next = first;
    while !batch.exit && batch.consumed < COMMANDS_PER_TURN {
        let input = match next.take() {
            Some(input) => input,
            None => match receiver.try_recv() {
                Ok(input) => input,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    batch.terminal_error =
                        Some(boxed("stdin reader disconnected without sending EOF"));
                    batch.exit = true;
                    break;
                }
            },
        };
        batch.consumed += 1;
        match apply(input) {
            Ok(InputAction::Continue | InputAction::Started) => {}
            Ok(InputAction::Exit) => batch.exit = true,
            Err(error) => {
                batch.terminal_error = Some(error);
                batch.exit = true;
            }
        }
    }
    batch
}

#[derive(Debug, PartialEq, Eq)]
enum OwnerTurn {
    Continue,
    Wait(Duration),
    Exit,
}

#[allow(clippy::too_many_arguments)]
fn owner_turn<S, W, E, C>(
    rack: &mut Oxurack,
    patch: &Path,
    session: &mut Option<&mut playback::Session<S>>,
    trace: &mut W,
    diagnostics: &mut E,
    receiver: &Receiver<Input>,
    cancellation: &AtomicBool,
    pending: &mut Option<Input>,
    deadline: &mut Instant,
    now: &mut C,
) -> Result<OwnerTurn, Box<dyn Error>>
where
    S: playback::MidiSender,
    W: Write,
    E: Write,
    C: FnMut() -> Instant,
{
    if cancellation.load(Ordering::SeqCst) {
        return Ok(OwnerTurn::Exit);
    }

    let first = match pending.take() {
        Some(input) => Some(input),
        None => match receiver.try_recv() {
            Ok(input) => Some(input),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                return Err(boxed("stdin reader disconnected without sending EOF"));
            }
        },
    };
    let batch = process_ready_commands(first, receiver, |input| {
        let was_running = rack.is_running();
        let action = handle_input(rack, patch, input, session, trace, diagnostics)?;
        if !was_running && rack.is_running() && matches!(action, InputAction::Started) {
            *deadline = now() + SAMPLE_PERIOD;
        }
        Ok(action)
    });
    if let Some(error) = batch.terminal_error {
        return Err(error);
    }
    if batch.exit || cancellation.load(Ordering::SeqCst) {
        return Ok(OwnerTurn::Exit);
    }

    if rack.is_running() && now() >= *deadline {
        tick_once(rack, session, trace)?;
        *deadline = now() + SAMPLE_PERIOD;
        return Ok(OwnerTurn::Continue);
    }

    let wait = if rack.is_running() {
        deadline.saturating_duration_since(now())
    } else {
        PAUSED_POLL
    };
    Ok(OwnerTurn::Wait(wait))
}

fn handle_input<S, W, E>(
    rack: &mut Oxurack,
    patch: &Path,
    input: Input,
    session: &mut Option<&mut playback::Session<S>>,
    trace: &mut W,
    diagnostics: &mut E,
) -> Result<InputAction, Box<dyn Error>>
where
    S: playback::MidiSender,
    W: Write,
    E: Write,
{
    let line = match input {
        Input::Line(line) => line,
        Input::RejectedLine(reason) => {
            writeln!(diagnostics, "rack-live: {reason}")?;
            diagnostics.flush()?;
            return Ok(InputAction::Continue);
        }
        Input::Eof => return Ok(InputAction::Exit),
        Input::ReadError(error) => return Err(boxed(format!("stdin read failed: {error}"))),
        Input::Disconnected => {
            return Err(boxed("stdin reader disconnected without sending EOF"));
        }
    };
    let command = line.trim();
    if command.is_empty() {
        return Ok(InputAction::Continue);
    }
    let mut words = command.split_whitespace();
    let verb = words.next().unwrap_or_default();
    let rest = command[verb.len()..].trim_start();
    match verb {
        "help" if rest.is_empty() => {
            writeln!(
                diagnostics,
                "help | play | pause | reset | get MODULE PARAM | set MODULE PARAM VALUE | status | scope | save PATH | scope-save PATH | quit\n\
                 examples: get clock_main bpm; set clock_main bpm 90; save /tmp/live-patch.ron\n\
                 pause stops and silences at the current sample without rewinding; reset silences while preserving running state"
            )?;
        }
        "play" if rest.is_empty() => {
            let was_running = rack.is_running();
            rack.start();
            writeln!(
                diagnostics,
                "rack-live: playing from sample {}",
                rack.sample_time().completed_samples()
            )?;
            if !was_running {
                diagnostics.flush()?;
                return Ok(InputAction::Started);
            }
        }
        "pause" if rest.is_empty() => {
            if rack.is_running() {
                rack.stop();
                rack.reset()
                    .map_err(|error| boxed(format!("pause reset failed: {error}")))?;
                deliver_drained(rack, session, trace)?;
                writeln!(
                    diagnostics,
                    "rack-live: paused at sample {}",
                    rack.sample_time().completed_samples()
                )?;
            } else {
                writeln!(
                    diagnostics,
                    "rack-live: already paused at sample {}",
                    rack.sample_time().completed_samples()
                )?;
            }
        }
        "reset" if rest.is_empty() => {
            rack.execute_command(&CoreCommand::Panic)
                .map_err(|error| boxed(format!("reset failed: {error}")))?;
            deliver_drained(rack, session, trace)?;
            writeln!(
                diagnostics,
                "rack-live: reset at sample {}",
                rack.sample_time().completed_samples()
            )?;
        }
        "get" => match parse_pair(rest) {
            Ok((module, parameter)) => match rack.parameter(module, parameter) {
                Ok(value) => writeln!(
                    diagnostics,
                    "{module}.{parameter}={}",
                    display_parameter(&value)
                )?,
                Err(error) => writeln!(diagnostics, "rack-live: get failed: {error}")?,
            },
            Err(message) => writeln!(diagnostics, "rack-live: {message}")?,
        },
        "set" => match parse_set(rest) {
            Ok((module, parameter, raw)) => {
                match set_live_parameter(rack, module, parameter, raw) {
                    Ok(value) => writeln!(
                        diagnostics,
                        "{module}.{parameter}={}",
                        display_parameter(&value)
                    )?,
                    Err(error) => writeln!(diagnostics, "rack-live: set failed: {error}")?,
                }
            }
            Err(message) => writeln!(diagnostics, "rack-live: {message}")?,
        },
        "status" if rest.is_empty() => match rack.snapshot_patch() {
            Ok(snapshot) => {
                writeln!(
                    diagnostics,
                    "sample={} running={} global_bpm={}",
                    rack.sample_time().completed_samples(),
                    rack.is_running(),
                    snapshot.bpm
                )?;
                writeln!(
                    diagnostics,
                    "global BPM is patch metadata and differs from a Clock module's audible bpm parameter"
                )?;
                let mut modules: Vec<_> = snapshot.modules.iter().collect();
                modules.sort_by(|left, right| left.instance_name.cmp(&right.instance_name));
                for module in modules {
                    writeln!(
                        diagnostics,
                        "module={} kind={}",
                        module.instance_name, module.kind
                    )?;
                    let mut parameters: Vec<_> = module.parameters.iter().collect();
                    parameters.sort_by(|left, right| left.0.cmp(right.0));
                    for (name, value) in parameters {
                        writeln!(diagnostics, "  {name}={}", display_parameter(value))?;
                    }
                }
            }
            Err(error) => writeln!(diagnostics, "rack-live: status failed: {error}")?,
        },
        "scope" if rest.is_empty() => match rack.scope() {
            Some(capture) => {
                workbench::write_view(diagnostics, patch, rack.sample_time(), capture, 12)?
            }
            None => writeln!(diagnostics, "rack-live: Scope capture is unavailable")?,
        },
        "save" => match one_path(rest) {
            Ok(path) => match rack.execute_command(&CoreCommand::SavePatch(path.clone())) {
                Ok(()) => writeln!(
                    diagnostics,
                    "rack-live: saved live configuration to {}",
                    path.display()
                )?,
                Err(error) => writeln!(diagnostics, "rack-live: save failed: {error}")?,
            },
            Err(message) => writeln!(diagnostics, "rack-live: {message}")?,
        },
        "scope-save" => match one_path(rest) {
            Ok(path) => match save_scope(rack, &path) {
                Ok(()) => writeln!(
                    diagnostics,
                    "rack-live: saved Scope CSV to {}",
                    path.display()
                )?,
                Err(error) => writeln!(diagnostics, "rack-live: Scope save failed: {error}")?,
            },
            Err(message) => writeln!(diagnostics, "rack-live: {message}")?,
        },
        "quit" if rest.is_empty() => return Ok(InputAction::Exit),
        _ => writeln!(
            diagnostics,
            "rack-live: unknown command or extra arguments; enter help"
        )?,
    }
    diagnostics.flush()?;
    Ok(InputAction::Continue)
}

fn deliver_drained<S, W>(
    rack: &mut Oxurack,
    session: &mut Option<&mut playback::Session<S>>,
    trace: &mut W,
) -> Result<(), Box<dyn Error>>
where
    S: playback::MidiSender,
    W: Write,
{
    let events: Vec<MidiOutputEvent> = rack.drain_midi().collect();
    for event in &events {
        rack.record_scope_midi(*event, MidiDelivery::Generated);
    }
    let mut failure = LiveFailure::default();
    for event in events {
        if let Some(session) = session.as_deref_mut() {
            if let Err(error) = session.send_performance(event) {
                failure.push("native MIDI send", error);
                break;
            }
            rack.record_scope_midi(event, MidiDelivery::Sent);
        }
        if let Err(error) = write_trace(trace, event) {
            failure.push("MIDI trace write", Box::new(error));
            break;
        }
    }
    if let Err(error) = trace.flush() {
        failure.push("MIDI trace flush", Box::new(error));
    }
    if failure.is_empty() {
        Ok(())
    } else {
        Err(Box::new(failure))
    }
}

fn tick_once<S, W>(
    rack: &mut Oxurack,
    session: &mut Option<&mut playback::Session<S>>,
    trace: &mut W,
) -> Result<(), Box<dyn Error>>
where
    S: playback::MidiSender,
    W: Write,
{
    let mut failure = LiveFailure::default();
    if let Err(error) = rack.advance_samples(1) {
        failure.push("sample advance", boxed(error.to_string()));
    }
    if let Err(error) = deliver_drained(rack, session, trace) {
        failure.push("sample output delivery", error);
    }
    if failure.is_empty() {
        Ok(())
    } else {
        Err(Box::new(failure))
    }
}

fn write_trace(writer: &mut impl Write, event: MidiOutputEvent) -> io::Result<()> {
    let [status, data1, data2] = event.message.to_bytes();
    writeln!(
        writer,
        "{}|{status:02X} {data1:02X} {data2:02X}",
        event.sample
    )
}

fn parse_pair(raw: &str) -> Result<(&str, &str), &'static str> {
    let mut parts = raw.split_whitespace();
    let module = parts.next().ok_or("usage: get MODULE PARAM")?;
    let parameter = parts.next().ok_or("usage: get MODULE PARAM")?;
    if parts.next().is_some() {
        return Err("usage: get MODULE PARAM");
    }
    Ok((module, parameter))
}

fn parse_set(raw: &str) -> Result<(&str, &str, &str), &'static str> {
    let (module, fields) = raw
        .split_once(char::is_whitespace)
        .ok_or("usage: set MODULE PARAM VALUE")?;
    let fields = fields.trim_start();
    let (parameter, value) = fields
        .split_once(char::is_whitespace)
        .ok_or("usage: set MODULE PARAM VALUE")?;
    let value = value.trim_start();
    if module.is_empty() || parameter.is_empty() || value.is_empty() {
        return Err("usage: set MODULE PARAM VALUE");
    }
    Ok((module, parameter, value))
}

fn set_live_parameter(
    rack: &mut Oxurack,
    module: &str,
    parameter: &str,
    raw: &str,
) -> Result<ParameterValue, Box<dyn Error>> {
    let current = rack.parameter(module, parameter)?;
    let value = match current {
        ParameterValue::Float(_) => {
            let value = raw.parse::<f32>()?;
            if !value.is_finite() {
                return Err(boxed("float value must be finite"));
            }
            ParameterValue::Float(value)
        }
        ParameterValue::Int(_) => ParameterValue::Int(raw.parse::<i64>()?),
        ParameterValue::Bool(_) => match raw {
            "true" => ParameterValue::Bool(true),
            "false" => ParameterValue::Bool(false),
            _ => return Err(boxed("boolean value must be true or false")),
        },
        ParameterValue::String(_) => {
            ParameterValue::String(if parameter == "waveform" && raw == "triangle" {
                "tri".to_owned()
            } else {
                raw.to_owned()
            })
        }
        ParameterValue::Scale(_) => {
            return Err(boxed(
                "Scale parameters are not editable in the terminal; edit the patch file",
            ));
        }
        _ => return Err(boxed("unsupported parameter type")),
    };
    rack.execute_command(&CoreCommand::SetParameter {
        module: module.to_owned(),
        param: parameter.to_owned(),
        value,
    })?;
    Ok(rack.parameter(module, parameter)?)
}

fn display_parameter(value: &ParameterValue) -> String {
    match value {
        ParameterValue::Float(value) => value.to_string(),
        ParameterValue::Int(value) => value.to_string(),
        ParameterValue::Bool(value) => value.to_string(),
        ParameterValue::String(value) => value.clone(),
        ParameterValue::Scale(value) => format!("Scale({value:?})"),
        _ => "unsupported".to_owned(),
    }
}

fn one_path(raw: &str) -> Result<PathBuf, &'static str> {
    if raw.is_empty() {
        return Err("usage: command PATH");
    }
    Ok(PathBuf::from(raw))
}

fn save_scope(rack: &Oxurack, path: &Path) -> io::Result<()> {
    let capture = rack
        .scope()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Scope capture is disabled"))?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    capture.write_csv(&mut file)?;
    file.flush()
}

fn boxed(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::other(message.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU64;
    use std::sync::{Arc, Condvar, Mutex, mpsc};
    use std::thread;

    use oxurack::{ScopeSelector, SimulationDriver};
    use oxurack_core::{load_patch_from_file, save_patch_to_file};
    use oxurack_mod_clock::ClockModule;
    use oxurack_mod_lfo::LfoModule;
    use oxurack_mod_turingmachine::TuringMachineModule;

    static NEXT: AtomicU64 = AtomicU64::new(0);

    fn patch_path(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches")
            .join(name)
    }

    fn temp_path(suffix: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "oxurack-live-{}-{}-{suffix}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ))
    }

    fn build_rack(patch: &Path, scope: bool) -> Oxurack {
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<LfoModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<oxurack::MidiOutModule>().unwrap();
        rack.load_patch(patch).unwrap();
        rack.set_driver_mode(SimulationDriver::Manual);
        if scope {
            rack.configure_scope(&[ScopeSelector::new("lfo_main", "bipolar_out")], 4096)
                .unwrap();
        }
        rack
    }

    fn fast_midi_fixture() -> PathBuf {
        let source = patch_path("turing-machine-midi.ron");
        let mut patch = load_patch_from_file(&source).unwrap();
        let clock = patch
            .modules
            .iter_mut()
            .find(|module| module.instance_name == "clock_main")
            .unwrap();
        clock
            .parameters
            .insert("bpm".into(), ParameterValue::Float(1000.0));
        clock
            .parameters
            .insert("ppqn".into(), ParameterValue::Int(30));
        let path = temp_path("fast-midi.ron");
        save_patch_to_file(&patch, &path).unwrap();
        path
    }

    fn command(rack: &mut Oxurack, patch: &Path, line: &str, diagnostic: &mut Vec<u8>) {
        let mut no_session: Option<&mut playback::Session<RecordingSender>> = None;
        let mut trace = Vec::new();
        let action = handle_input(
            rack,
            patch,
            Input::Line(line.to_owned()),
            &mut no_session,
            &mut trace,
            diagnostic,
        )
        .unwrap();
        assert!(matches!(
            action,
            InputAction::Continue | InputAction::Started
        ));
    }

    fn take_sample(rack: &mut Oxurack) -> Vec<(u64, [u8; 3])> {
        rack.advance_samples(1).unwrap();
        rack.drain_midi()
            .map(|event| (event.sample, event.message.to_bytes()))
            .collect()
    }

    fn trace_rows(bytes: &[u8]) -> Vec<(u64, [u8; 3])> {
        String::from_utf8(bytes.to_vec())
            .unwrap()
            .lines()
            .map(|line| {
                let (sample, message) = line.split_once('|').unwrap();
                let mut bytes = message
                    .split_whitespace()
                    .map(|byte| u8::from_str_radix(byte, 16).unwrap());
                (
                    sample.parse().unwrap(),
                    [
                        bytes.next().unwrap(),
                        bytes.next().unwrap(),
                        bytes.next().unwrap(),
                    ],
                )
            })
            .collect()
    }

    fn direct_sample_with_scope(rack: &mut Oxurack) -> Vec<(u64, [u8; 3])> {
        rack.advance_samples(1).unwrap();
        let events: Vec<_> = rack.drain_midi().collect();
        for event in &events {
            rack.record_scope_midi(*event, MidiDelivery::Generated);
        }
        events
            .into_iter()
            .map(|event| (event.sample, event.message.to_bytes()))
            .collect()
    }

    struct RecordingSender {
        sent: Arc<std::sync::Mutex<Vec<[u8; 3]>>>,
    }

    impl Default for RecordingSender {
        fn default() -> Self {
            Self {
                sent: Arc::new(std::sync::Mutex::new(Vec::new())),
            }
        }
    }

    impl playback::MidiSender for RecordingSender {
        fn send(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
            self.sent
                .lock()
                .map_err(|error| boxed(error.to_string()))?
                .push([bytes[0], bytes[1], bytes[2]]);
            Ok(())
        }
    }

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("injected trace write failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::other("injected trace flush failure"))
        }
    }

    #[derive(Default)]
    struct FailOnceWriter {
        failed: bool,
        later_bytes: Vec<u8>,
    }

    impl Write for FailOnceWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            if !self.failed {
                self.failed = true;
                return Err(io::Error::other("injected first trace write failure"));
            }
            self.later_bytes.extend_from_slice(buffer);
            Ok(buffer.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct FailingSender(Arc<std::sync::atomic::AtomicUsize>);

    impl playback::MidiSender for FailingSender {
        fn send(&mut self, _bytes: &[u8]) -> Result<(), Box<dyn Error>> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Err(boxed("injected MIDI send failure"))
        }
    }

    #[derive(Clone, Default)]
    struct TickCounter(Arc<(Mutex<usize>, Condvar)>);

    impl TickCounter {
        fn count(&self) -> usize {
            *self.0.0.lock().unwrap()
        }

        fn wait_for(&self, target: usize) {
            let (count, changed) = &*self.0;
            let mut count = count.lock().unwrap();
            while *count < target {
                count = changed.wait(count).unwrap();
            }
        }
    }

    impl Write for TickCounter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            Ok(buffer.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            let (count, changed) = &*self.0;
            *count
                .lock()
                .map_err(|error| io::Error::other(error.to_string()))? += 1;
            changed.notify_all();
            Ok(())
        }
    }

    #[derive(Clone)]
    struct FakeClock(Arc<Mutex<Instant>>);

    impl FakeClock {
        fn now(&self) -> Instant {
            *self.0.lock().unwrap()
        }

        fn advance(&self, duration: Duration) {
            let mut now = self.0.lock().unwrap();
            *now += duration;
        }
    }

    struct AdvancingWriter {
        clock: FakeClock,
        step: Duration,
    }

    impl Write for AdvancingWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            Ok(buffer.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            self.clock.advance(self.step);
            Ok(())
        }
    }

    #[test]
    fn command_grammars_preserve_string_tail_and_reject_extras() {
        assert_eq!(parse_pair("clock_main bpm").unwrap(), ("clock_main", "bpm"));
        assert!(parse_pair("clock_main bpm extra").is_err());
        assert_eq!(
            parse_set("lfo_main waveform triangle").unwrap(),
            ("lfo_main", "waveform", "triangle")
        );
        assert_eq!(
            parse_set("lfo_main label saw wave").unwrap(),
            ("lfo_main", "label", "saw wave")
        );
        assert!(parse_set("lfo_main waveform").is_err());
    }

    #[test]
    fn production_turn_services_commands_then_admits_one_due_sample_without_burst() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut rack = build_rack(&fixture, false);
        rack.start();
        let base = Instant::now();
        let clock = FakeClock(Arc::new(Mutex::new(base)));
        let mut deadline = base + SAMPLE_PERIOD;
        let (sender, receiver) = mpsc::sync_channel(COMMANDS_PER_TURN + 2);
        for _ in 0..COMMANDS_PER_TURN + 2 {
            sender.send(Input::Line("status".into())).unwrap();
        }
        let mut trace = Vec::new();
        let mut diagnostics = AdvancingWriter {
            clock: clock.clone(),
            step: SAMPLE_PERIOD * 2,
        };
        let cancellation = AtomicBool::new(false);
        let mut session = None;
        let mut pending = None;
        let mut now = || clock.now();
        let turn = owner_turn::<RecordingSender, _, _, _>(
            &mut rack,
            &fixture,
            &mut session,
            &mut trace,
            &mut diagnostics,
            &receiver,
            &cancellation,
            &mut pending,
            &mut deadline,
            &mut now,
        )
        .unwrap();
        assert_eq!(turn, OwnerTurn::Continue);
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert!(deadline > clock.now());
        assert_eq!(
            receiver.try_iter().count(),
            2,
            "one production turn must cap ready work at eight commands"
        );

        // A second late turn admits one sample, then schedules from after that
        // sample instead of replaying missed periods in a catch-up burst.
        clock.advance(SAMPLE_PERIOD * 20);
        let before = rack.sample_time().completed_samples();
        let turn = owner_turn::<RecordingSender, _, _, _>(
            &mut rack,
            &fixture,
            &mut session,
            &mut trace,
            &mut diagnostics,
            &receiver,
            &cancellation,
            &mut pending,
            &mut deadline,
            &mut now,
        )
        .unwrap();
        assert_eq!(turn, OwnerTurn::Continue);
        assert_eq!(rack.sample_time().completed_samples(), before + 1);
        assert_eq!(deadline, clock.now() + SAMPLE_PERIOD);

        // A due pause is serviced before sampling and stops that turn.
        let (pause_sender, pause_receiver) = mpsc::sync_channel(1);
        pause_sender.send(Input::Line("pause".into())).unwrap();
        deadline = clock.now();
        let before_pause = rack.sample_time().completed_samples();
        let _ = owner_turn::<RecordingSender, _, _, _>(
            &mut rack,
            &fixture,
            &mut session,
            &mut trace,
            &mut diagnostics,
            &pause_receiver,
            &cancellation,
            &mut pending,
            &mut deadline,
            &mut now,
        )
        .unwrap();
        assert!(!rack.is_running());
        assert_eq!(rack.sample_time().completed_samples(), before_pause);

        // A paused turn cannot sample; a due quit also wins over sampling.
        deadline = clock.now();
        let paused = owner_turn::<RecordingSender, _, _, _>(
            &mut rack,
            &fixture,
            &mut session,
            &mut trace,
            &mut diagnostics,
            &pause_receiver,
            &cancellation,
            &mut pending,
            &mut deadline,
            &mut now,
        )
        .unwrap();
        assert_eq!(paused, OwnerTurn::Wait(PAUSED_POLL));
        rack.start();
        deadline = clock.now();
        let (quit_sender, quit_receiver) = mpsc::sync_channel(1);
        quit_sender.send(Input::Line("quit".into())).unwrap();
        let before_quit = rack.sample_time().completed_samples();
        let quit = owner_turn::<RecordingSender, _, _, _>(
            &mut rack,
            &fixture,
            &mut session,
            &mut trace,
            &mut diagnostics,
            &quit_receiver,
            &cancellation,
            &mut pending,
            &mut deadline,
            &mut now,
        )
        .unwrap();
        assert_eq!(quit, OwnerTurn::Exit);
        assert_eq!(rack.sample_time().completed_samples(), before_quit);
        drop(sender);
        drop(pause_sender);
        drop(quit_sender);
    }

    #[test]
    fn production_turn_preserves_due_sample_across_repeated_play_and_long_status() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut rack = build_rack(&fixture, true);
        rack.start();
        let base = Instant::now();
        let clock = FakeClock(Arc::new(Mutex::new(base)));
        let mut deadline = base;
        let (sender, receiver) = mpsc::sync_channel(COMMANDS_PER_TURN);
        for _ in 0..COMMANDS_PER_TURN {
            sender.send(Input::Line("play".into())).unwrap();
        }
        let mut trace = Vec::new();
        let mut diagnostics = AdvancingWriter {
            clock: clock.clone(),
            step: SAMPLE_PERIOD * 2,
        };
        let cancellation = AtomicBool::new(false);
        let mut session = None;
        let mut pending = None;
        let mut now = || clock.now();
        let turn = owner_turn::<RecordingSender, _, _, _>(
            &mut rack,
            &fixture,
            &mut session,
            &mut trace,
            &mut diagnostics,
            &receiver,
            &cancellation,
            &mut pending,
            &mut deadline,
            &mut now,
        )
        .unwrap();
        assert_eq!(turn, OwnerTurn::Continue);
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert_eq!(deadline, clock.now() + SAMPLE_PERIOD);
        drop(sender);
    }

    #[test]
    fn ready_input_is_capped_at_eight_fifo_messages() {
        let (sender, receiver) = mpsc::sync_channel(COMMANDS_PER_TURN + 1);
        for index in 0..=COMMANDS_PER_TURN {
            sender.send(Input::Line(format!("help {index}"))).unwrap();
        }
        let first = receiver.try_recv().unwrap();
        let mut handled = Vec::new();
        let batch = process_ready_commands(Some(first), &receiver, |input| {
            if let Input::Line(line) = input {
                handled.push(line);
            }
            Ok(InputAction::Continue)
        });
        assert_eq!(batch.consumed, COMMANDS_PER_TURN);
        assert_eq!(handled.len(), COMMANDS_PER_TURN);
        assert!(
            matches!(receiver.try_recv().unwrap(), Input::Line(line) if line == format!("help {COMMANDS_PER_TURN}"))
        );
    }

    #[test]
    fn combining_owner_and_reader_shutdown_errors_keeps_both_causes() {
        let error = combine_results(
            Err(boxed("primary owner failure")),
            Err(boxed("completed reader panic")),
        )
        .unwrap_err();
        let text = error.to_string();
        assert!(text.contains("primary owner failure"));
        assert!(text.contains("completed reader panic"));
    }

    #[test]
    fn native_delivery_keeps_trace_and_records_only_successful_sends() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut native = build_rack(&fixture, true);
        let mut trace_only = build_rack(&fixture, true);
        native.start();
        trace_only.start();
        let sent = Arc::new(Mutex::new(Vec::new()));
        let mut session = playback::Session::new(RecordingSender {
            sent: Arc::clone(&sent),
        });
        let mut session_option = Some(&mut session);
        let mut native_trace = Vec::new();
        let mut trace_only_option: Option<&mut playback::Session<RecordingSender>> = None;
        let mut trace_only_trace = Vec::new();
        for _ in 0..1200 {
            tick_once(&mut native, &mut session_option, &mut native_trace).unwrap();
            tick_once(
                &mut trace_only,
                &mut trace_only_option,
                &mut trace_only_trace,
            )
            .unwrap();
        }

        let mut native_diagnostics = Vec::new();
        let _ = handle_input(
            &mut native,
            &fixture,
            Input::Line("pause".into()),
            &mut session_option,
            &mut native_trace,
            &mut native_diagnostics,
        )
        .unwrap();
        let _ = handle_input(
            &mut trace_only,
            &fixture,
            Input::Line("pause".into()),
            &mut trace_only_option,
            &mut trace_only_trace,
            &mut Vec::new(),
        )
        .unwrap();

        let native_rows = trace_rows(&native_trace);
        let trace_rows = trace_rows(&trace_only_trace);
        assert!(!native_rows.is_empty());
        assert_eq!(native_rows, trace_rows);
        let successful_sends = sent.lock().unwrap().clone();
        assert_eq!(
            successful_sends,
            native_rows.iter().map(|row| row.1).collect::<Vec<_>>()
        );

        let native_records: Vec<_> = native.scope().unwrap().records().copied().collect();
        let trace_records: Vec<_> = trace_only.scope().unwrap().records().copied().collect();
        let generated = |records: &[oxurack::ScopeRecord]| {
            records
                .iter()
                .filter_map(|record| match record {
                    oxurack::ScopeRecord::Midi {
                        event,
                        delivery: MidiDelivery::Generated,
                    } => Some((event.sample, event.message.to_bytes())),
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        let sent_records = native_records
            .iter()
            .filter(|record| {
                matches!(
                    record,
                    oxurack::ScopeRecord::Midi {
                        delivery: MidiDelivery::Sent,
                        ..
                    }
                )
            })
            .count();
        assert_eq!(generated(&native_records), generated(&trace_records));
        assert_eq!(sent_records, successful_sends.len());
        assert!(trace_records.iter().all(|record| !matches!(
            record,
            oxurack::ScopeRecord::Midi {
                delivery: MidiDelivery::Sent,
                ..
            }
        )));
    }

    #[test]
    fn live_get_set_and_save_use_typed_live_values_and_bad_commands_recover() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut rack = build_rack(&fixture, true);
        let mut output = Vec::new();
        command(&mut rack, &fixture, "set clock_main bpm 90", &mut output);
        command(
            &mut rack,
            &fixture,
            "set lfo_main frequency_hz 1",
            &mut output,
        );
        assert_eq!(
            rack.parameter("lfo_main", "frequency_hz").unwrap(),
            ParameterValue::Float(1.0)
        );
        command(
            &mut rack,
            &fixture,
            "set lfo_main frequency_hz 0",
            &mut output,
        );
        assert_eq!(
            rack.parameter("lfo_main", "frequency_hz").unwrap(),
            ParameterValue::Float(0.0)
        );
        command(
            &mut rack,
            &fixture,
            "set lfo_main frequency_hz 1",
            &mut output,
        );
        command(
            &mut rack,
            &fixture,
            "set tm_main write_probability 0.25",
            &mut output,
        );
        command(
            &mut rack,
            &fixture,
            "set lfo_main waveform triangle",
            &mut output,
        );
        command(&mut rack, &fixture, "get lfo_main waveform", &mut output);
        assert_eq!(
            rack.parameter("clock_main", "bpm").unwrap(),
            ParameterValue::Float(90.0)
        );
        assert_eq!(
            rack.parameter("lfo_main", "frequency_hz").unwrap(),
            ParameterValue::Float(1.0)
        );
        assert_eq!(
            rack.parameter("tm_main", "write_probability").unwrap(),
            ParameterValue::Float(0.25)
        );
        assert_eq!(
            rack.parameter("lfo_main", "waveform").unwrap(),
            ParameterValue::String("tri".into())
        );

        let before = rack.snapshot_patch().unwrap();
        let before_time = rack.sample_time();
        command(&mut rack, &fixture, "get clock_main bpm extra", &mut output);
        command(
            &mut rack,
            &fixture,
            "set lfo_main frequency_hz NaN",
            &mut output,
        );
        command(
            &mut rack,
            &fixture,
            "set lfo_main frequency_hz -1",
            &mut output,
        );
        command(&mut rack, &fixture, "set unknown bpm 90", &mut output);
        command(&mut rack, &fixture, "set tm_main scale Major", &mut output);
        assert_eq!(rack.snapshot_patch().unwrap(), before);
        assert_eq!(rack.sample_time(), before_time);
        command(&mut rack, &fixture, "set clock_main bpm 90", &mut output);
        command(&mut rack, &fixture, "get clock_main bpm", &mut output);
        assert_eq!(rack.snapshot_patch().unwrap(), before);
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("get failed") || output.contains("usage: get"));
        assert!(output.contains("finite"));
        assert!(output.contains("set failed"));
        assert!(output.contains("terminal; edit the patch file"));

        let saved = temp_path("live.ron");
        command(
            &mut rack,
            &fixture,
            &format!("save {}", saved.display()),
            &mut Vec::new(),
        );
        let snapshot = load_patch_from_file(&saved).unwrap();
        let clock = snapshot
            .modules
            .iter()
            .find(|module| module.instance_name == "clock_main")
            .unwrap();
        let lfo = snapshot
            .modules
            .iter()
            .find(|module| module.instance_name == "lfo_main")
            .unwrap();
        let tm = snapshot
            .modules
            .iter()
            .find(|module| module.instance_name == "tm_main")
            .unwrap();
        assert_eq!(clock.parameters["bpm"], ParameterValue::Float(90.0));
        assert_eq!(
            lfo.parameters["waveform"],
            ParameterValue::String("tri".into())
        );
        assert_eq!(lfo.parameters["frequency_hz"], ParameterValue::Float(1.0));
        assert_eq!(
            tm.parameters["write_probability"],
            ParameterValue::Float(0.25)
        );
        let _ = std::fs::remove_file(saved);
    }

    #[test]
    fn status_and_help_report_sorted_live_parameters_and_clock_bpm_distinction() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut rack = build_rack(&fixture, true);
        rack.set_parameter("clock_main", "bpm", ParameterValue::Float(90.0))
            .unwrap();
        let time = rack.sample_time();
        let records: Vec<_> = rack.scope().unwrap().records().copied().collect();
        let mut output = Vec::new();
        command(&mut rack, &fixture, "status", &mut output);
        command(&mut rack, &fixture, "help", &mut output);
        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("sample=0 running=false global_bpm=120"));
        assert!(text.contains("global BPM is patch metadata"));
        assert!(text.contains("module=clock_main kind=clock"));
        assert!(text.contains("bpm=90"));
        assert!(text.contains("module=lfo_main kind=lfo"));
        assert!(text.contains("module=midi_main kind=midi_out"));
        assert!(text.contains("module=tm_main kind=turing_machine"));
        assert!(text.contains("get clock_main bpm"));
        assert!(text.contains("set clock_main bpm 90"));
        assert!(text.contains("save /tmp/live-patch.ron"));
        assert!(text.contains("pause stops and silences"));
        assert!(text.contains("reset silences while preserving running state"));
        let names = ["clock_main", "lfo_main", "midi_main", "tm_main"];
        let positions: Vec<_> = names.iter().map(|name| text.find(name).unwrap()).collect();
        assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(rack.sample_time(), time);
        assert_eq!(
            rack.scope().unwrap().records().copied().collect::<Vec<_>>(),
            records
        );
    }

    #[test]
    fn command_subject_and_direct_control_match_while_unchanged_control_diverges() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut subject = build_rack(&fixture, true);
        let mut direct = build_rack(&fixture, true);
        let mut unchanged = build_rack(&fixture, true);
        subject.start();
        direct.start();
        unchanged.start();
        let mut subject_trace = Vec::new();
        let mut direct_trace = Vec::new();
        let mut unchanged_trace = Vec::new();
        for _ in 0..200 {
            subject_trace.extend(take_sample(&mut subject));
            direct_trace.extend(take_sample(&mut direct));
            unchanged_trace.extend(take_sample(&mut unchanged));
        }
        command(
            &mut subject,
            &fixture,
            "set lfo_main frequency_hz 1",
            &mut Vec::new(),
        );
        direct
            .set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(1.0))
            .unwrap();
        for _ in 0..1000 {
            subject_trace.extend(take_sample(&mut subject));
            direct_trace.extend(take_sample(&mut direct));
            unchanged_trace.extend(take_sample(&mut unchanged));
        }
        assert_eq!(subject_trace, direct_trace);
        assert_eq!(subject.sample_time().completed_samples(), 1200);
        assert_eq!(direct.sample_time().completed_samples(), 1200);
        assert_eq!(unchanged.sample_time().completed_samples(), 1200);
        let subject_records: Vec<_> = subject.scope().unwrap().records().copied().collect();
        let direct_records: Vec<_> = direct.scope().unwrap().records().copied().collect();
        let unchanged_records: Vec<_> = unchanged.scope().unwrap().records().copied().collect();
        assert_eq!(subject_records, direct_records);
        assert_ne!(subject_records, unchanged_records);
        assert_eq!(subject_trace, direct_trace);
    }

    #[test]
    fn pause_reset_and_resume_have_one_cleanup_boundary_and_keep_the_rack_running_state() {
        let fixture = patch_path("turing-machine-midi.ron");
        let mut rack = build_rack(&fixture, false);
        rack.set_parameter("clock_main", "bpm", ParameterValue::Float(1000.0))
            .unwrap();
        rack.set_parameter("clock_main", "ppqn", ParameterValue::Int(30))
            .unwrap();
        let sent = Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut session = playback::Session::new(RecordingSender {
            sent: Arc::clone(&sent),
        });
        rack.start();
        let mut found_note = false;
        let mut active_note = 0;
        for _ in 0..64 {
            rack.advance_samples(1).unwrap();
            let events: Vec<_> = rack.drain_midi().collect();
            for event in events {
                session.send_performance(event).unwrap();
                if event.message.status & 0xf0 == 0x90 && event.message.data2 > 0 {
                    active_note = event.message.data1;
                    found_note = true;
                }
            }
            if found_note {
                break;
            }
        }
        assert!(found_note);
        let paused_at = rack.sample_time();
        let mut output = Vec::new();
        let mut session_option = Some(&mut session);
        let mut trace = Vec::new();
        let action = handle_input(
            &mut rack,
            &fixture,
            Input::Line("pause".into()),
            &mut session_option,
            &mut trace,
            &mut output,
        )
        .unwrap();
        assert!(matches!(action, InputAction::Continue));
        assert!(!rack.is_running());
        assert_eq!(rack.sample_time(), paused_at);
        let after_pause = sent.lock().unwrap().clone();
        assert!(after_pause.ends_with(&[[0x80, active_note, 0], [0xB0, 123, 0], [0xB0, 120, 0]]));
        let after_first_pause = sent.lock().unwrap().len();
        let _ = handle_input(
            &mut rack,
            &fixture,
            Input::Line("pause".into()),
            &mut session_option,
            &mut trace,
            &mut output,
        )
        .unwrap();
        assert_eq!(sent.lock().unwrap().len(), after_first_pause);
        rack.start();
        let before_reset = rack.sample_time();
        let _ = handle_input(
            &mut rack,
            &fixture,
            Input::Line("reset".into()),
            &mut session_option,
            &mut trace,
            &mut output,
        )
        .unwrap();
        assert!(rack.is_running());
        assert_eq!(rack.sample_time(), before_reset);
        rack.stop();
        assert_eq!(rack.sample_time(), before_reset);
    }

    #[test]
    fn frontend_pause_and_resumed_engines_match_a_public_control_after_1000_samples() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut subject = build_rack(&fixture, true);
        let mut control = build_rack(&fixture, true);
        for rack in [&mut subject, &mut control] {
            rack.set_parameter("clock_main", "bpm", ParameterValue::Float(1000.0))
                .unwrap();
            rack.set_parameter("clock_main", "ppqn", ParameterValue::Int(30))
                .unwrap();
            rack.set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(5.0))
                .unwrap();
            rack.start();
        }

        let mut session_option: Option<&mut playback::Session<RecordingSender>> = None;
        let mut subject_trace = Vec::new();
        let mut control_trace = Vec::new();
        for _ in 0..1000 {
            tick_once(&mut subject, &mut session_option, &mut subject_trace).unwrap();
            control_trace.extend(direct_sample_with_scope(&mut control));
            if trace_rows(&subject_trace)
                .iter()
                .any(|(_, bytes)| bytes[0] & 0xf0 == 0x90 && bytes[2] > 0)
            {
                break;
            }
        }
        let initial_subject = trace_rows(&subject_trace);
        assert!(
            initial_subject
                .iter()
                .any(|(_, bytes)| bytes[0] & 0xf0 == 0x90 && bytes[2] > 0)
        );
        assert_eq!(initial_subject, control_trace);

        subject_trace.clear();
        let paused_at = subject.sample_time();
        let mut diagnostics = Vec::new();
        let action = handle_input(
            &mut subject,
            &fixture,
            Input::Line("pause".into()),
            &mut session_option,
            &mut subject_trace,
            &mut diagnostics,
        )
        .unwrap();
        assert!(matches!(action, InputAction::Continue));
        control.stop();
        control.reset().unwrap();
        let control_cleanup: Vec<_> = control.drain_midi().collect();
        for event in &control_cleanup {
            control.record_scope_midi(*event, MidiDelivery::Generated);
        }
        let control_cleanup: Vec<_> = control_cleanup
            .iter()
            .map(|event| (event.sample, event.message.to_bytes()))
            .collect();
        assert_eq!(trace_rows(&subject_trace), control_cleanup);
        assert!(
            control_cleanup
                .iter()
                .any(|(_, bytes)| bytes[0] & 0xf0 == 0x80)
        );
        assert!(control_cleanup.contains(&(paused_at.completed_samples(), [0xB0, 123, 0])));
        assert!(control_cleanup.contains(&(paused_at.completed_samples(), [0xB0, 120, 0])));
        assert!(!subject.is_running());
        assert!(!control.is_running());
        assert_eq!(subject.sample_time(), paused_at);
        assert_eq!(control.sample_time(), paused_at);

        subject_trace.clear();
        control_trace.clear();
        command(&mut subject, &fixture, "play", &mut diagnostics);
        control.start();
        for _ in 0..1000 {
            tick_once(&mut subject, &mut session_option, &mut subject_trace).unwrap();
            control_trace.extend(direct_sample_with_scope(&mut control));
        }
        let resumed_subject = trace_rows(&subject_trace);
        assert!(!resumed_subject.is_empty());
        assert_eq!(resumed_subject, control_trace);
        assert_eq!(
            subject.sample_time().completed_samples(),
            paused_at.completed_samples() + 1000
        );
        assert_eq!(subject.sample_time(), control.sample_time());
        assert_eq!(
            subject
                .scope()
                .unwrap()
                .records()
                .copied()
                .collect::<Vec<_>>(),
            control
                .scope()
                .unwrap()
                .records()
                .copied()
                .collect::<Vec<_>>()
        );
        let distinct_lfo_values: std::collections::HashSet<_> = subject
            .scope()
            .unwrap()
            .records()
            .filter_map(|record| match record {
                oxurack::ScopeRecord::Signal {
                    selection: 0,
                    value,
                    ..
                } => Some(format!("{value:?}")),
                _ => None,
            })
            .collect();
        assert!(distinct_lfo_values.len() > 1, "LFO output did not change");
    }

    #[test]
    fn scope_export_collision_and_save_keep_current_capture_and_sample_cursor() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut rack = build_rack(&fixture, true);
        rack.configure_scope(&[ScopeSelector::new("lfo_main", "bipolar_out")], 4)
            .unwrap();
        rack.start();
        for _ in 0..25 {
            take_sample(&mut rack);
        }
        let time = rack.sample_time();
        let records: Vec<_> = rack.scope().unwrap().records().copied().collect();
        let mut view = Vec::new();
        command(&mut rack, &fixture, "scope", &mut view);
        let view = String::from_utf8(view).unwrap();
        assert!(view.contains("completed_samples=25 capacity=4 retained=4 overwritten=21"));
        assert_eq!(rack.sample_time(), time);
        assert_eq!(
            rack.scope().unwrap().records().copied().collect::<Vec<_>>(),
            records
        );

        rack.set_parameter("clock_main", "bpm", ParameterValue::Float(90.0))
            .unwrap();
        let csv = temp_path("capture.csv");
        save_scope(&rack, &csv).unwrap();
        let original = std::fs::read(&csv).unwrap();
        assert!(save_scope(&rack, &csv).is_err());
        assert_eq!(std::fs::read(&csv).unwrap(), original);
        assert_eq!(rack.sample_time(), time);
        assert_eq!(
            rack.scope().unwrap().records().copied().collect::<Vec<_>>(),
            records
        );
        assert_eq!(original.iter().filter(|byte| **byte == b'\n').count(), 5);
        assert_eq!(take_sample(&mut rack).len(), 0);
        assert_eq!(rack.sample_time().completed_samples(), 26);

        let save = temp_path("snapshot.ron");
        rack.execute_command(&CoreCommand::SavePatch(save.clone()))
            .unwrap();
        assert_eq!(rack.sample_time().completed_samples(), 26);
        assert_eq!(
            rack.scope().unwrap().records().copied().collect::<Vec<_>>(),
            records
                .iter()
                .copied()
                .skip(1)
                .chain(rack.scope().unwrap().records().copied().last())
                .collect::<Vec<_>>()
        );
        let saved = load_patch_from_file(&save).unwrap();
        let mut fresh_rack = build_rack(&fixture, false);
        fresh_rack.load_patch(&save).unwrap();
        assert_eq!(fresh_rack.sample_time().completed_samples(), 0);
        assert_eq!(saved.modules.len(), 4);
        let mut independent = build_rack(&fixture, false);
        independent
            .set_parameter("clock_main", "bpm", ParameterValue::Float(90.0))
            .unwrap();
        fresh_rack.start();
        independent.start();
        for _ in 0..100 {
            assert_eq!(take_sample(&mut fresh_rack), take_sample(&mut independent));
        }
        assert_eq!(fresh_rack.sample_time().completed_samples(), 100);
        assert_eq!(independent.sample_time().completed_samples(), 100);
        rack.stop();
        fresh_rack.stop();
        independent.stop();
        let _ = std::fs::remove_file(csv);
        let _ = std::fs::remove_file(save);
    }

    #[test]
    fn scope_and_save_leave_live_control_continuous_and_replay_matches_for_2000_samples() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut subject = build_rack(&fixture, true);
        let mut control = build_rack(&fixture, true);
        let mut subject_trace = Vec::new();
        let mut control_trace = Vec::new();
        subject.start();
        control.start();
        for _ in 0..200 {
            subject_trace.extend(direct_sample_with_scope(&mut subject));
            control_trace.extend(direct_sample_with_scope(&mut control));
        }

        let mut diagnostics = Vec::new();
        command(
            &mut subject,
            &fixture,
            "set clock_main bpm 90",
            &mut diagnostics,
        );
        command(
            &mut subject,
            &fixture,
            "set lfo_main frequency_hz 1",
            &mut diagnostics,
        );
        control
            .set_parameter("clock_main", "bpm", ParameterValue::Float(90.0))
            .unwrap();
        control
            .set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(1.0))
            .unwrap();

        let save = temp_path("continuing-live.ron");
        let csv = temp_path("continuing-scope.csv");
        let mut view = Vec::new();
        command(&mut subject, &fixture, "scope", &mut view);
        command(
            &mut subject,
            &fixture,
            &format!("save {}", save.display()),
            &mut diagnostics,
        );
        command(
            &mut subject,
            &fixture,
            &format!("scope-save {}", csv.display()),
            &mut diagnostics,
        );
        assert!(
            String::from_utf8(view)
                .unwrap()
                .contains("completed_samples=200")
        );
        assert!(std::fs::read_to_string(&csv).unwrap().lines().count() > 200);
        assert_eq!(subject.sample_time().completed_samples(), 200);
        assert_eq!(control.sample_time().completed_samples(), 200);

        for _ in 0..1000 {
            subject_trace.extend(direct_sample_with_scope(&mut subject));
            control_trace.extend(direct_sample_with_scope(&mut control));
        }
        assert_eq!(subject.sample_time().completed_samples(), 1200);
        assert_eq!(control.sample_time().completed_samples(), 1200);
        assert_eq!(subject_trace, control_trace);
        assert_eq!(
            subject
                .scope()
                .unwrap()
                .records()
                .copied()
                .collect::<Vec<_>>(),
            control
                .scope()
                .unwrap()
                .records()
                .copied()
                .collect::<Vec<_>>()
        );
        assert!(!subject_trace.is_empty());

        let saved_patch = load_patch_from_file(&save).unwrap();
        assert_eq!(saved_patch.modules.len(), 4);
        let mut replay = build_rack(&save, true);
        let mut independent_patch = load_patch_from_file(&fixture).unwrap();
        independent_patch
            .modules
            .iter_mut()
            .find(|module| module.instance_name == "clock_main")
            .unwrap()
            .parameters
            .insert("bpm".into(), ParameterValue::Float(90.0));
        independent_patch
            .modules
            .iter_mut()
            .find(|module| module.instance_name == "lfo_main")
            .unwrap()
            .parameters
            .insert("frequency_hz".into(), ParameterValue::Float(1.0));
        let independent_fixture = temp_path("independent-config.ron");
        save_patch_to_file(&independent_patch, &independent_fixture).unwrap();
        let mut independent = build_rack(&independent_fixture, true);
        let mut unchanged = build_rack(&fixture, true);
        assert_eq!(saved_patch, independent.snapshot_patch().unwrap());
        assert_eq!(replay.sample_time().completed_samples(), 0);
        replay.start();
        independent.start();
        unchanged.start();
        let mut replay_trace = Vec::new();
        let mut independent_trace = Vec::new();
        let mut unchanged_trace = Vec::new();
        for _ in 0..2000 {
            replay_trace.extend(direct_sample_with_scope(&mut replay));
            independent_trace.extend(direct_sample_with_scope(&mut independent));
            unchanged_trace.extend(direct_sample_with_scope(&mut unchanged));
        }
        assert_eq!(replay.sample_time().completed_samples(), 2000);
        assert_eq!(
            replay_trace,
            independent_trace,
            "replay clock={:?} lfo={:?}; independent clock={:?} lfo={:?}",
            replay.parameter("clock_main", "bpm"),
            replay.parameter("lfo_main", "frequency_hz"),
            independent.parameter("clock_main", "bpm"),
            independent.parameter("lfo_main", "frequency_hz"),
        );
        assert_eq!(
            replay
                .scope()
                .unwrap()
                .records()
                .copied()
                .collect::<Vec<_>>(),
            independent
                .scope()
                .unwrap()
                .records()
                .copied()
                .collect::<Vec<_>>()
        );
        assert!(!replay_trace.is_empty());
        assert_ne!(replay_trace, unchanged_trace);
        assert_ne!(
            replay
                .scope()
                .unwrap()
                .records()
                .copied()
                .collect::<Vec<_>>(),
            unchanged
                .scope()
                .unwrap()
                .records()
                .copied()
                .collect::<Vec<_>>()
        );

        let _ = std::fs::remove_file(save);
        let _ = std::fs::remove_file(csv);
        let _ = std::fs::remove_file(independent_fixture);
    }

    #[test]
    fn owner_ticks_while_terminal_reader_is_blocked_in_stdin() {
        struct BlockingAfterPlay {
            first: bool,
            release: mpsc::Receiver<()>,
        }
        impl std::io::Read for BlockingAfterPlay {
            fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
                if !self.first {
                    self.first = true;
                    buffer[..5].copy_from_slice(b"play\n");
                    return Ok(5);
                }
                let _ = self.release.recv();
                Ok(0)
            }
        }
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut rack = build_rack(&fixture, true);
        let (release_sender, release_receiver) = mpsc::channel();
        let reader = super::super::input::Reader::spawn(BlockingAfterPlay {
            first: false,
            release: release_receiver,
        });
        let cancellation = Arc::new(AtomicBool::new(false));
        let cancel = Arc::clone(&cancellation);
        let tick_counter = TickCounter::default();
        let observer_counter = tick_counter.clone();
        let stopper = thread::spawn(move || {
            observer_counter.wait_for(10);
            cancel.store(true, Ordering::SeqCst);
        });
        let mut trace = tick_counter;
        let mut diagnostics = Vec::new();
        let result = run::<RecordingSender, _, _>(
            &mut rack,
            &fixture,
            None,
            &mut trace,
            &mut diagnostics,
            reader.receiver(),
            &cancellation,
        );
        let samples = rack.sample_time().completed_samples();
        reader.shutdown().unwrap();
        assert!(result.is_ok());
        assert!(samples >= 10, "owner completed only {samples} samples");
        release_sender.send(()).unwrap();
        stopper.join().unwrap();
    }

    #[test]
    fn quit_and_eof_stop_and_cleanup_the_active_owner() {
        for close_with_eof in [false, true] {
            let fixture = fast_midi_fixture();
            let mut rack = build_rack(&fixture, false);
            let (sender, receiver) = mpsc::sync_channel(32);
            sender.send(Input::Line("play".into())).unwrap();
            let tick_counter = TickCounter::default();
            let producer_counter = tick_counter.clone();
            let producer = thread::spawn(move || {
                producer_counter.wait_for(10);
                if close_with_eof {
                    sender.send(Input::Eof).unwrap();
                } else {
                    sender.send(Input::Line("quit".into())).unwrap();
                }
            });
            let cancellation = AtomicBool::new(false);
            let mut trace = tick_counter;
            let mut diagnostics = Vec::new();
            run::<RecordingSender, _, _>(
                &mut rack,
                &fixture,
                None,
                &mut trace,
                &mut diagnostics,
                &receiver,
                &cancellation,
            )
            .unwrap();
            producer.join().unwrap();
            assert!(!rack.is_running());
            assert!(rack.sample_time().completed_samples() > 0);
            assert!(trace.count() >= 10);
            let _ = std::fs::remove_file(fixture);
        }
    }

    #[test]
    fn failures_preserve_primary_and_cleanup_causes_and_still_silence_dirty_native_voice() {
        let fixture = fast_midi_fixture();
        let mut rack = build_rack(&fixture, false);
        rack.configure_scope(&[], 4096).unwrap();
        let (sender, receiver) = mpsc::sync_channel(32);
        sender
            .send(Input::ReadError(io::Error::other("CRC_PRIMARY_READ")))
            .unwrap();
        let cancellation = AtomicBool::new(false);
        let mut trace = FailingWriter;
        let mut diagnostics = Vec::new();
        let error = run::<RecordingSender, _, _>(
            &mut rack,
            &fixture,
            None,
            &mut trace,
            &mut diagnostics,
            &receiver,
            &cancellation,
        )
        .unwrap_err();
        let details = error.to_string();
        assert!(details.contains("CRC_PRIMARY_READ"));
        assert!(details.contains("MIDI trace flush"));
        assert!(details.contains("injected trace flush failure"));
        assert!(!rack.is_running());
        assert_eq!(rack.sample_time().completed_samples(), 0);
        drop(sender);

        let mut rack = build_rack(&fixture, false);
        let (sender, receiver) = mpsc::sync_channel(32);
        sender.send(Input::Line("play".into())).unwrap();
        let sent = Arc::new(Mutex::new(Vec::new()));
        let mut session = playback::Session::new(RecordingSender {
            sent: Arc::clone(&sent),
        });
        let mut trace = FailOnceWriter::default();
        let mut diagnostics = Vec::new();
        let error = run(
            &mut rack,
            &fixture,
            Some(&mut session),
            &mut trace,
            &mut diagnostics,
            &receiver,
            &cancellation,
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("injected first trace write failure")
        );
        assert!(!rack.is_running());
        let sent = sent.lock().unwrap().clone();
        assert!(sent.iter().any(|message| message[0] & 0xf0 == 0x90));
        assert!(sent.iter().any(|message| message[0] & 0xf0 == 0x80));
        assert!(sent.contains(&[0xB0, 123, 0]));
        assert!(sent.contains(&[0xB0, 120, 0]));
        let cleanup_trace = trace_rows(&trace.later_bytes);
        assert!(
            cleanup_trace
                .iter()
                .any(|(_, message)| message[0] & 0xf0 == 0x80)
        );

        let mut rack = build_rack(&fixture, false);
        rack.configure_scope(&[], 4096).unwrap();
        let (sender, receiver) = mpsc::sync_channel(1);
        sender.send(Input::Line("play".into())).unwrap();
        let cancellation = AtomicBool::new(false);
        let mut session = playback::Session::new(FailingSender(Arc::new(
            std::sync::atomic::AtomicUsize::new(0),
        )));
        let mut trace = Vec::new();
        let mut diagnostics = Vec::new();
        let error = run(
            &mut rack,
            &fixture,
            Some(&mut session),
            &mut trace,
            &mut diagnostics,
            &receiver,
            &cancellation,
        )
        .unwrap_err();
        assert!(error.to_string().contains("injected MIDI send failure"));
        assert!(rack.scope().unwrap().records().any(|record| matches!(
            record,
            oxurack::ScopeRecord::Midi {
                delivery: MidiDelivery::Generated,
                ..
            }
        )));
        assert!(!rack.scope().unwrap().records().any(|record| matches!(
            record,
            oxurack::ScopeRecord::Midi {
                delivery: MidiDelivery::Sent,
                ..
            }
        )));
        drop(sender);
        let _ = std::fs::remove_file(fixture);
    }

    #[test]
    fn quit_while_reader_is_blocked_on_full_bounded_channel_does_not_hang() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut rack = build_rack(&fixture, false);
        let lines = format!(
            "quit\n{}",
            "help\n".repeat(super::super::input::INPUT_CAPACITY * 4)
        );
        let reader = super::super::input::Reader::spawn(std::io::Cursor::new(lines));
        let cancellation = AtomicBool::new(false);
        let mut trace = Vec::new();
        let mut diagnostics = Vec::new();
        run::<RecordingSender, _, _>(
            &mut rack,
            &fixture,
            None,
            &mut trace,
            &mut diagnostics,
            reader.receiver(),
            &cancellation,
        )
        .unwrap();
        reader.shutdown().unwrap();
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert!(!rack.is_running());
    }

    #[test]
    fn reader_error_is_visible_and_uses_common_cleanup() {
        let fixture = patch_path("turing-machine-lfo.ron");
        let mut rack = build_rack(&fixture, false);
        rack.start();
        let (sender, receiver) = mpsc::sync_channel(32);
        sender
            .send(Input::ReadError(io::Error::other(
                "injected stdin read failure",
            )))
            .unwrap();
        let cancellation = AtomicBool::new(false);
        let mut trace = Vec::new();
        let mut diagnostics = Vec::new();
        let error = run::<RecordingSender, _, _>(
            &mut rack,
            &fixture,
            None,
            &mut trace,
            &mut diagnostics,
            &receiver,
            &cancellation,
        )
        .unwrap_err();
        assert!(error.to_string().contains("injected stdin read failure"));
        assert!(!rack.is_running());
        assert_eq!(rack.sample_time().completed_samples(), 0);
        drop(sender);

        let mut rack = build_rack(&fixture, false);
        rack.start();
        let (sender, receiver) = mpsc::sync_channel(32);
        drop(sender);
        let error = run::<RecordingSender, _, _>(
            &mut rack,
            &fixture,
            None,
            &mut Vec::new(),
            &mut Vec::new(),
            &receiver,
            &cancellation,
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("disconnected without sending EOF")
        );
        assert!(!rack.is_running());
    }
}
