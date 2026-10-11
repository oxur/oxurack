use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

#[cfg(test)]
use oxurack::ScopeRecord;
use oxurack::{MidiDelivery, MidiOutputEvent, Oxurack};

const SAMPLE_PERIOD: Duration = Duration::from_millis(1);

pub(crate) trait MidiSender {
    fn send(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub(crate) struct PlaybackFailure {
    primary: Option<Box<dyn Error>>,
    cleanup: Vec<String>,
}

impl PlaybackFailure {
    fn new(primary: Option<Box<dyn Error>>, cleanup: Vec<String>) -> Self {
        Self { primary, cleanup }
    }
}

impl fmt::Display for PlaybackFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(primary) = &self.primary {
            write!(f, "{primary}")?;
        } else {
            f.write_str("MIDI cleanup failed")?;
        }
        for failure in &self.cleanup {
            write!(f, "; cleanup: {failure}")?;
        }
        Ok(())
    }
}

impl Error for PlaybackFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.primary.as_deref()
    }
}

#[derive(Debug)]
struct SendContext {
    sample: u64,
    source: Box<dyn Error>,
}

impl fmt::Display for SendContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "sample {} MIDI send failed: {}",
            self.sample, self.source
        )
    }
}

impl Error for SendContext {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventKind {
    NoteOn(u8),
    NoteOff(u8),
    ControlChange,
    Other,
}

fn classify(status: u8, data1: u8, data2: u8) -> (usize, EventKind) {
    let channel = usize::from(status & 0x0f);
    match status & 0xf0 {
        0x90 if data2 > 0 => (channel, EventKind::NoteOn(data1)),
        0x80 => (channel, EventKind::NoteOff(data1)),
        0x90 => (channel, EventKind::NoteOff(data1)),
        0xb0 => (channel, EventKind::ControlChange),
        _ => (channel, EventKind::Other),
    }
}

pub(crate) struct Session<S> {
    sender: S,
    active: [Option<u8>; 16],
    used: [bool; 16],
    terminal: bool,
    cleanup_errors: Vec<String>,
}

impl<S: MidiSender> Session<S> {
    pub(crate) fn new(sender: S) -> Self {
        Self {
            sender,
            active: [None; 16],
            used: [false; 16],
            terminal: false,
            cleanup_errors: Vec::new(),
        }
    }

    fn send_performance(&mut self, event: MidiOutputEvent) -> Result<(), Box<dyn Error>> {
        if self.terminal {
            return Err("playback session is already finished".into());
        }
        let message = event.message;
        let (channel, kind) = classify(message.status, message.data1, message.data2);
        self.used[channel] = true;
        let [status, data1, data2] = event.message.to_bytes();
        self.sender
            .send(&[status, data1, data2])
            .map_err(|source| -> Box<dyn Error> {
                Box::new(SendContext {
                    sample: event.sample,
                    source,
                })
            })?;
        match kind {
            EventKind::NoteOn(note) => self.active[channel] = Some(note),
            EventKind::NoteOff(note) if self.active[channel] == Some(note) => {
                self.active[channel] = None;
            }
            EventKind::NoteOff(_) | EventKind::ControlChange | EventKind::Other => {}
        }
        Ok(())
    }

    pub(crate) fn finish(&mut self) -> &[String] {
        if self.terminal {
            return &self.cleanup_errors;
        }
        self.terminal = true;
        for channel in 0..16 {
            if let Some(note) = self.active[channel] {
                self.cleanup_send(channel, [0x80 | channel as u8, note, 0]);
            }
        }
        for channel in 0..16 {
            if self.used[channel] {
                self.cleanup_send(channel, [0xB0 | channel as u8, 123, 0]);
                self.cleanup_send(channel, [0xB0 | channel as u8, 120, 0]);
            }
        }
        &self.cleanup_errors
    }

    fn cleanup_send(&mut self, channel: usize, bytes: [u8; 3]) {
        if let Err(error) = self.sender.send(&bytes) {
            self.cleanup_errors.push(format!(
                "channel {channel} {:02X} send failed: {error}",
                bytes[0] & 0xf0
            ));
        }
    }
}

fn boxed(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::other(message.into()))
}

pub(crate) fn select_name(names: &[String], requested: &str) -> Result<usize, String> {
    if requested.is_empty() {
        return Err("output name must not be empty".to_owned());
    }
    let matches: Vec<_> = names
        .iter()
        .enumerate()
        .filter(|(_, name)| name.as_str() == requested)
        .map(|(index, _)| index)
        .collect();
    match matches.as_slice() {
        [index] => Ok(*index),
        [] => Err(format!(
            "no MIDI output named {requested:?}; run --list-outputs to inspect current names"
        )),
        _ => Err(format!(
            "MIDI output name {requested:?} is ambiguous; select a zero-based index from --list-outputs"
        )),
    }
}

pub(crate) fn select_index(names: &[String], index: usize) -> Result<usize, String> {
    if index < names.len() {
        Ok(index)
    } else {
        Err(format!(
            "MIDI output index {index} is out of range; run --list-outputs for a fresh index"
        ))
    }
}

pub(crate) fn play<S, W>(
    rack: &mut Oxurack,
    session: &mut Session<S>,
    samples: u64,
    cancellation: &AtomicBool,
    mut trace: W,
) -> Result<(), PlaybackFailure>
where
    S: MidiSender,
    W: Write,
{
    let mut primary = run_samples(rack, session, samples, cancellation, &mut trace).err();
    rack.stop();
    let _ = drain_generated(rack);
    let cleanup = session.finish().to_vec();
    if let Err(error) = trace.flush()
        && primary.is_none()
    {
        primary = Some(boxed(format!("trace flush failed: {error}")));
    }
    if primary.is_some() || !cleanup.is_empty() {
        Err(PlaybackFailure::new(primary, cleanup))
    } else {
        Ok(())
    }
}

/// Drains exactly the runner-owned batch, then copies it after the rack borrow ends.
pub(crate) fn drain_generated(rack: &mut Oxurack) -> Vec<MidiOutputEvent> {
    let events: Vec<_> = rack.drain_midi().collect();
    for event in &events {
        rack.record_scope_midi(*event, MidiDelivery::Generated);
    }
    events
}

fn run_samples<S, W>(
    rack: &mut Oxurack,
    session: &mut Session<S>,
    samples: u64,
    cancellation: &AtomicBool,
    trace: &mut W,
) -> Result<(), Box<dyn Error>>
where
    S: MidiSender,
    W: Write,
{
    let mut deadline = Instant::now();
    for _ in 0..samples {
        wait_until(deadline, cancellation)?;
        if cancellation.load(Ordering::SeqCst) {
            break;
        }
        let dispatch_started = Instant::now();
        if rack.advance_samples(1)? == 0 {
            break;
        }
        let events = drain_generated(rack);
        for event in events {
            if cancellation.load(Ordering::SeqCst) {
                break;
            }
            let [status, data1, data2] = event.message.to_bytes();
            session.send_performance(event)?;
            rack.record_scope_midi(event, MidiDelivery::Sent);
            writeln!(
                trace,
                "{}|{status:02X} {data1:02X} {data2:02X}",
                event.sample
            )
            .map_err(|error| {
                boxed(format!(
                    "sample {} trace write failed: {error}",
                    event.sample
                ))
            })?;
            if cancellation.load(Ordering::SeqCst) {
                break;
            }
        }
        let finished = Instant::now();
        deadline = next_deadline(deadline, dispatch_started, finished)?;
        if cancellation.load(Ordering::SeqCst) {
            break;
        }
    }
    Ok(())
}

fn wait_until(deadline: Instant, cancellation: &AtomicBool) -> Result<(), Box<dyn Error>> {
    loop {
        if cancellation.load(Ordering::SeqCst) {
            return Ok(());
        }
        let now = Instant::now();
        if now >= deadline {
            return Ok(());
        }
        std::thread::sleep(deadline.duration_since(now));
    }
}

fn next_deadline(
    previous: Instant,
    dispatch_started: Instant,
    finished: Instant,
) -> Result<Instant, Box<dyn Error>> {
    let normal_next = previous
        .checked_add(SAMPLE_PERIOD)
        .ok_or_else(|| boxed("sample deadline overflow"))?;
    if dispatch_started > previous || finished > normal_next {
        finished
            .checked_add(SAMPLE_PERIOD)
            .ok_or_else(|| boxed("sample deadline rebase overflow"))
    } else {
        Ok(normal_next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::AtomicU64;

    #[derive(Default)]
    struct Recorder {
        sent: Vec<[u8; 3]>,
        fail_at: Vec<usize>,
        cancel_after_success: Option<(Arc<AtomicBool>, usize)>,
        observed: Option<Arc<std::sync::Mutex<Vec<[u8; 3]>>>>,
    }

    impl MidiSender for Recorder {
        fn send(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
            let call = self.sent.len();
            let message = [bytes[0], bytes[1], bytes[2]];
            self.sent.push(message);
            if let Some(observed) = &self.observed {
                observed.lock().unwrap().push(message);
            }
            if self.fail_at.contains(&call) {
                Err(format!("injected failure at {call}").into())
            } else {
                if let Some((flag, after)) = &self.cancel_after_success
                    && call == *after
                {
                    flag.store(true, Ordering::SeqCst);
                }
                Ok(())
            }
        }
    }

    fn template_event() -> MidiOutputEvent {
        let (mut rack, path) = fast_rack();
        let event = (0..24)
            .find_map(|_| {
                rack.advance_samples(1).unwrap();
                rack.drain_midi().next()
            })
            .expect("fast fixture produces a MIDI event within 24 samples");
        rack.stop();
        let _ = std::fs::remove_file(path);
        event
    }

    fn event(mut template: MidiOutputEvent, bytes: [u8; 3], sample: u64) -> MidiOutputEvent {
        template.sample = sample;
        template.message.status = bytes[0];
        template.message.data1 = bytes[1];
        template.message.data2 = bytes[2];
        template
    }

    #[test]
    fn failed_note_off_keeps_note_active_for_cleanup_retry() {
        let template = template_event();
        let mut session = Session::new(Recorder {
            sent: vec![],
            fail_at: vec![1],
            cancel_after_success: None,
            observed: None,
        });
        session
            .send_performance(event(template, [0x90, 60, 90], 2))
            .unwrap();
        assert!(
            session
                .send_performance(event(template, [0x80, 60, 0], 3))
                .is_err()
        );
        session.finish();
        assert_eq!(session.sender.sent[2], [0x80, 60, 0]);
    }

    #[test]
    fn successful_note_off_clears_ownership_before_a_later_failed_note_on() {
        let template = template_event();
        let mut session = Session::new(Recorder {
            sent: vec![],
            fail_at: vec![2],
            cancel_after_success: None,
            observed: None,
        });
        session
            .send_performance(event(template, [0x90, 60, 90], 0))
            .unwrap();
        session
            .send_performance(event(template, [0x80, 60, 0], 1))
            .unwrap();
        assert!(
            session
                .send_performance(event(template, [0x90, 62, 90], 2))
                .is_err()
        );
        session.finish();
        assert_eq!(session.sender.sent[3], [0xB0, 123, 0]);
        assert_eq!(session.sender.sent[4], [0xB0, 120, 0]);
        assert_eq!(session.sender.sent.len(), 5);
    }

    #[test]
    fn failed_first_note_on_marks_channel_used_but_not_active() {
        let template = template_event();
        let mut session = Session::new(Recorder {
            sent: vec![],
            fail_at: vec![0],
            cancel_after_success: None,
            observed: None,
        });
        assert!(
            session
                .send_performance(event(template, [0x92, 60, 90], 4))
                .is_err()
        );
        session.finish();
        assert_eq!(session.sender.sent[1], [0xB2, 123, 0]);
        assert_eq!(session.sender.sent[2], [0xB2, 120, 0]);
    }

    #[test]
    fn finish_orders_cleanup_attempts_continues_and_is_idempotent() {
        let template = template_event();
        let mut session = Session::new(Recorder {
            sent: vec![],
            fail_at: vec![2],
            cancel_after_success: None,
            observed: None,
        });
        session
            .send_performance(event(template, [0x94, 61, 90], 0))
            .unwrap();
        session
            .send_performance(event(template, [0x91, 62, 90], 1))
            .unwrap();
        let first = session.finish().to_vec();
        let sent = session.sender.sent.clone();
        assert_eq!(
            sent[2..],
            [
                [0x81, 62, 0],
                [0x84, 61, 0],
                [0xB1, 123, 0],
                [0xB1, 120, 0],
                [0xB4, 123, 0],
                [0xB4, 120, 0]
            ]
        );
        assert_eq!(first.len(), 1);
        assert_eq!(session.finish(), first.as_slice());
        assert_eq!(session.sender.sent, sent);
        assert!(
            session
                .send_performance(event(template, [0x94, 64, 90], 2))
                .is_err()
        );
        assert_eq!(session.sender.sent, sent);
    }

    #[test]
    fn exact_name_selector_requires_one_match() {
        let names = vec!["A".to_owned(), "AB".to_owned(), "A".to_owned()];
        assert_eq!(select_name(&names, "AB"), Ok(1));
        assert!(select_name(&names, "A").is_err());
        assert!(select_name(&names, "B").is_err());
        assert_eq!(select_index(&names, 2), Ok(2));
        assert!(select_index(&names, 3).is_err());
        assert!(select_name(&names, "").is_err());
    }

    fn fast_rack() -> (Oxurack, std::path::PathBuf) {
        use oxurack_core::{deserialize_patch, save_patch_to_file};
        use oxurack_mod_clock::ClockModule;
        use oxurack_mod_turingmachine::TuringMachineModule;
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-midi.ron");
        let mut patch = deserialize_patch(&std::fs::read_to_string(fixture).unwrap()).unwrap();
        patch.modules[0]
            .parameters
            .insert("bpm".into(), oxurack::ParameterValue::Float(1000.0));
        patch.modules[0]
            .parameters
            .insert("ppqn".into(), oxurack::ParameterValue::Int(30));
        let path = std::env::temp_dir().join(format!(
            "oxurack-playback-{}-{}.ron",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        save_patch_to_file(&patch, &path).unwrap();
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<oxurack::MidiOutModule>().unwrap();
        rack.load_patch(&path).unwrap();
        rack.set_driver_mode(oxurack::SimulationDriver::Manual)
            .start();
        (rack, path)
    }

    #[test]
    fn production_runner_stops_drain_on_cancel_and_fresh_run_replays_start() {
        let cancellation = Arc::new(AtomicBool::new(false));
        let sender = Recorder {
            sent: Vec::new(),
            fail_at: Vec::new(),
            cancel_after_success: Some((Arc::clone(&cancellation), 0)),
            observed: None,
        };
        let (mut rack, path) = fast_rack();
        let mut session = Session::new(sender);
        let mut trace = Vec::new();
        play(&mut rack, &mut session, 200, &cancellation, &mut trace).unwrap();
        let (sample, bytes) = super::super::oracle::expected_trace(200)[0];
        assert_eq!(
            trace,
            format!(
                "{sample}|{:02X} {:02X} {:02X}\n",
                bytes[0], bytes[1], bytes[2]
            )
            .as_bytes()
        );
        assert_eq!(rack.sample_time().completed_samples(), sample + 1);
        assert_eq!(rack.advance_samples(1).unwrap(), 0);
        let _ = std::fs::remove_file(path);

        let cancellation = Arc::new(AtomicBool::new(false));
        let sender = Recorder {
            sent: Vec::new(),
            fail_at: Vec::new(),
            cancel_after_success: Some((Arc::clone(&cancellation), 0)),
            observed: None,
        };
        let (mut rack, path) = fast_rack();
        let mut session = Session::new(sender);
        let mut second_trace = Vec::new();
        play(
            &mut rack,
            &mut session,
            200,
            &cancellation,
            &mut second_trace,
        )
        .unwrap();
        assert_eq!(trace, second_trace);
        assert_eq!(rack.sample_time().completed_samples(), sample + 1);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn production_runner_matches_independent_oracle_for_24_samples() {
        let (mut rack, path) = fast_rack();
        let mut session = Session::new(Recorder::default());
        let cancellation = AtomicBool::new(false);
        let mut trace = Vec::new();
        play(&mut rack, &mut session, 24, &cancellation, &mut trace).unwrap();

        let expected = super::super::oracle::expected_trace(24);
        let expected_text: Vec<_> = expected
            .iter()
            .map(|(sample, bytes)| {
                format!(
                    "{sample}|{:02X} {:02X} {:02X}\n",
                    bytes[0], bytes[1], bytes[2]
                )
            })
            .collect();
        assert_eq!(trace, expected_text.concat().as_bytes());
        assert_eq!(rack.sample_time().completed_samples(), 24);
        assert_eq!(rack.drain_midi().count(), 0);
        assert_eq!(rack.advance_samples(1).unwrap(), 0);
        let expected_performance: Vec<_> = expected.iter().map(|(_, bytes)| *bytes).collect();
        assert_eq!(
            &session.sender.sent[..expected_performance.len()],
            expected_performance
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn capture_copies_generated_and_successfully_sent_events_without_changing_trace() {
        let (mut baseline, baseline_path) = fast_rack();
        let mut baseline_session = Session::new(Recorder::default());
        let cancellation = AtomicBool::new(false);
        let mut baseline_trace = Vec::new();
        play(
            &mut baseline,
            &mut baseline_session,
            200,
            &cancellation,
            &mut baseline_trace,
        )
        .unwrap();

        let (mut captured, captured_path) = fast_rack();
        captured.stop();
        captured.configure_scope(&[], 2048).unwrap();
        captured.start();
        let mut session = Session::new(Recorder::default());
        let mut trace = Vec::new();
        play(
            &mut captured,
            &mut session,
            200,
            &AtomicBool::new(false),
            &mut trace,
        )
        .unwrap();
        assert_eq!(trace, baseline_trace);
        let records: Vec<_> = captured.scope().unwrap().records().copied().collect();
        let generated: Vec<_> = records
            .iter()
            .filter_map(|record| match record {
                ScopeRecord::Midi {
                    event,
                    delivery: MidiDelivery::Generated,
                } => Some(*event),
                _ => None,
            })
            .collect();
        let sent: Vec<_> = records
            .iter()
            .filter_map(|record| match record {
                ScopeRecord::Midi {
                    event,
                    delivery: MidiDelivery::Sent,
                } => Some(*event),
                _ => None,
            })
            .collect();
        let expected = super::super::oracle::expected_trace(200);
        assert_eq!(
            generated
                .iter()
                .map(|event| (event.sample, event.message.to_bytes()))
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(sent, generated);
        assert_eq!(trace, baseline_trace);
        assert_eq!(
            trace.iter().filter(|byte| **byte == b'\n').count(),
            generated.len()
        );
        let mut csv = Vec::new();
        captured.scope().unwrap().write_csv(&mut csv).unwrap();
        let csv = String::from_utf8(csv).unwrap();
        assert!(csv.starts_with("sample,record,module,port,value_kind,value,status,data1,data2\n"));
        let first = generated[0];
        let [status, data1, data2] = first.message.to_bytes();
        assert!(csv.contains(&format!(
            "{},midi_generated,,,midi,,{status:02X},{data1:02X},{data2:02X}\n",
            first.sample
        )));
        let _ = std::fs::remove_file(baseline_path);
        let _ = std::fs::remove_file(captured_path);
    }

    #[test]
    fn failed_send_marks_only_successful_prefix_sent_and_excludes_cleanup() {
        let (mut rack, path) = fast_rack();
        rack.stop();
        rack.configure_scope(&[], 128).unwrap();
        rack.start();
        let mut session = Session::new(Recorder {
            sent: Vec::new(),
            fail_at: vec![1],
            cancel_after_success: None,
            observed: None,
        });
        let error = play(
            &mut rack,
            &mut session,
            200,
            &AtomicBool::new(false),
            &mut Vec::new(),
        )
        .unwrap_err();
        assert!(error.to_string().contains("injected failure at 1"));
        let records: Vec<_> = rack.scope().unwrap().records().copied().collect();
        let generated: Vec<_> = records
            .iter()
            .filter(|record| {
                matches!(
                    record,
                    ScopeRecord::Midi {
                        delivery: MidiDelivery::Generated,
                        ..
                    }
                )
            })
            .collect();
        let sent: Vec<_> = records
            .iter()
            .filter(|record| {
                matches!(
                    record,
                    ScopeRecord::Midi {
                        delivery: MidiDelivery::Sent,
                        ..
                    }
                )
            })
            .collect();
        assert!(generated.len() >= 2);
        assert_eq!(sent.len(), 1);
        assert!(
            session.sender.sent.len() > sent.len(),
            "cleanup sends stay outside Scope"
        );
        assert_eq!(rack.drain_midi().count(), 0);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn cancellation_after_success_keeps_generated_batch_and_sent_prefix() {
        let (mut rack, path) = fast_rack();
        rack.stop();
        rack.configure_scope(&[], 128).unwrap();
        rack.start();
        let cancellation = Arc::new(AtomicBool::new(false));
        let sender = Recorder {
            sent: Vec::new(),
            fail_at: Vec::new(),
            cancel_after_success: Some((Arc::clone(&cancellation), 0)),
            observed: None,
        };
        let mut session = Session::new(sender);
        play(&mut rack, &mut session, 200, &cancellation, &mut Vec::new()).unwrap();
        let records: Vec<_> = rack.scope().unwrap().records().copied().collect();
        let generated = records
            .iter()
            .filter(|record| {
                matches!(
                    record,
                    ScopeRecord::Midi {
                        delivery: MidiDelivery::Generated,
                        ..
                    }
                )
            })
            .count();
        let sent = records
            .iter()
            .filter(|record| {
                matches!(
                    record,
                    ScopeRecord::Midi {
                        delivery: MidiDelivery::Sent,
                        ..
                    }
                )
            })
            .count();
        assert!(generated >= sent);
        assert_eq!(sent, 1);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn zero_sample_runner_finishes_without_sends() {
        let (mut rack, path) = fast_rack();
        let mut session = Session::new(Recorder::default());
        let cancellation = AtomicBool::new(false);
        play(&mut rack, &mut session, 0, &cancellation, &mut Vec::new()).unwrap();
        assert!(session.sender.sent.is_empty());
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert_eq!(rack.advance_samples(1).unwrap(), 0);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn production_runner_keeps_primary_send_failure_and_runs_cleanup() {
        let observed = Arc::new(std::sync::Mutex::new(Vec::new()));
        let sender = Recorder {
            sent: Vec::new(),
            fail_at: vec![1, 2],
            cancel_after_success: None,
            observed: Some(Arc::clone(&observed)),
        };
        let cancellation = AtomicBool::new(false);
        let (mut rack, path) = fast_rack();
        let mut session = Session::new(sender);
        let mut trace = Vec::new();
        let error = play(&mut rack, &mut session, 200, &cancellation, &mut trace).unwrap_err();
        assert!(error.to_string().contains("sample"));
        assert!(error.to_string().contains("injected failure at 1"));
        assert!(error.to_string().contains("cleanup"));
        let sent = observed.lock().unwrap();
        assert!(
            sent.len() >= 4,
            "performance failure and cleanup attempted: {sent:?}"
        );
        assert!(trace.starts_with(b"5|90 "));
        assert_eq!(trace.iter().filter(|byte| **byte == b'\n').count(), 1);
        assert_eq!(rack.drain_midi().count(), 0);
        assert_eq!(rack.advance_samples(1).unwrap(), 0);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn deadline_rebases_from_dispatch_and_finish_times() {
        let origin = Instant::now();
        let on_time = next_deadline(origin, origin, origin + Duration::from_micros(200)).unwrap();
        assert_eq!(on_time, origin + SAMPLE_PERIOD);
        let late_start = origin + Duration::from_millis(5);
        let late_finish = late_start + Duration::from_micros(200);
        assert_eq!(
            next_deadline(origin, late_start, late_finish).unwrap(),
            late_finish + SAMPLE_PERIOD
        );
    }
}
