use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use oxurack::{
    CommandStatus, CoreCommand, MidiDelivery, MidiOutModule, MidiOutputEvent, Oxurack,
    ParameterValue, ScopeRecord, ScopeSelector, SimulationDriver, Value,
};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;

#[derive(Debug, Parser)]
#[command(about = "Exercise live rack handles, removal, commands and seeded replay")]
struct Args {
    #[arg(long)]
    patch: PathBuf,
    #[arg(long)]
    save: PathBuf,
    #[arg(long, default_value_t = 2000)]
    samples: u64,
}

type Trace = Vec<(u64, u8, u8, u8)>;
type Signals = Vec<(u64, usize, Value)>;

fn install() -> Result<Oxurack, oxurack::CoreError> {
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>()?;
    rack.try_install::<LfoModule>()?;
    rack.try_install::<TuringMachineModule>()?;
    rack.try_install::<MidiOutModule>()?;
    Ok(rack)
}

fn drain_one(
    rack: &mut Oxurack,
    status: &mut impl Write,
    phase: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    rack.advance_samples(1)?;
    let events: Vec<_> = rack.drain_midi().collect();
    for event in events.iter().copied() {
        let [a, b, c] = event.message.to_bytes();
        writeln!(
            status,
            "{phase}: midi sample={} {a:02X} {b:02X} {c:02X}",
            event.sample
        )?;
        rack.record_scope_midi(event, MidiDelivery::Generated);
    }
    Ok(())
}

fn play(
    rack: &mut Oxurack,
    samples: u64,
    output: &mut impl Write,
) -> Result<(Trace, Signals, Vec<MidiOutputEvent>), Box<dyn std::error::Error>> {
    rack.set_driver_mode(SimulationDriver::Manual);
    rack.configure_scope(
        &[
            ScopeSelector::new("clock_main", "pulse"),
            ScopeSelector::new("tm_main", "write_probability"),
        ],
        (samples as usize).saturating_mul(4).max(8),
    )?;
    rack.start();
    let mut trace = Vec::new();
    let mut all_midi = Vec::new();
    for _ in 0..samples {
        rack.advance_samples(1)?;
        let events: Vec<_> = rack.drain_midi().collect();
        for event in events.iter().copied() {
            let [a, b, c] = event.message.to_bytes();
            writeln!(output, "{}|{a:02X} {b:02X} {c:02X}", event.sample)?;
            trace.push((event.sample, a, b, c));
            all_midi.push(event);
            rack.record_scope_midi(event, MidiDelivery::Generated);
        }
    }
    rack.stop();
    let signals = rack
        .scope()
        .ok_or("Scope missing")?
        .records()
        .filter_map(|record| match record {
            ScopeRecord::Signal {
                sample,
                selection,
                value,
            } => Some((*sample, *selection, *value)),
            _ => None,
        })
        .collect();
    Ok((trace, signals, all_midi))
}

fn run_with_output(
    args: Args,
    output: &mut impl Write,
    status: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.samples == 0 {
        return Err("--samples must be positive".into());
    }
    let mut rack = install()?;
    rack.load_patch(&args.patch)?;
    rack.set_driver_mode(SimulationDriver::Manual);
    let lfo = rack.module::<LfoModule>("lfo_main")?;
    let lfo_frequency = lfo.knob("frequency_hz");
    let tm = rack.module::<TuringMachineModule>("tm_main")?;
    rack.configure_scope(
        &[
            ScopeSelector::new("lfo_main", "bipolar_out"),
            ScopeSelector::new("tm_main", "write_modulation"),
        ],
        4000,
    )?;
    rack.start();
    for _ in 0..300 {
        drain_one(&mut rack, status, "initial")?;
    }
    rack.stop();
    rack.set_knob(&lfo_frequency, ParameterValue::Float(0.25))?;
    rack.remove_module("lfo_main")?;
    if rack.knob(&lfo_frequency).is_ok() {
        return Err("removed LFO knob remained valid".into());
    }
    if tm.parameter(&rack, "write_probability").is_err() {
        return Err("surviving TM handle became stale".into());
    }
    writeln!(
        status,
        "removed lfo_main at sample {}; surviving TM handle valid",
        rack.sample_time().completed_samples()
    )?;

    let missing = args.save.with_extension("guaranteed-missing.ron");
    let ids = [
        rack.enqueue_command(CoreCommand::SavePatch(args.save.clone()))?,
        rack.enqueue_command(CoreCommand::LoadPatch(missing))?,
        rack.enqueue_command(CoreCommand::LoadPatch(args.save.clone()))?,
        rack.enqueue_command(CoreCommand::SetParameter {
            module: "tm_main".into(),
            param: "write_probability".into(),
            value: ParameterValue::Float(0.5),
        })?,
        rack.enqueue_command(CoreCommand::SetBpm(133.0))?,
        rack.enqueue_command(CoreCommand::SavePatch(args.save.clone()))?,
    ];
    let outcomes = rack.process_commands();
    if outcomes.len() != 6 || outcomes.iter().map(|result| result.id).collect::<Vec<_>>() != ids {
        return Err("queued command outcomes were not FIFO".into());
    }
    if !matches!(outcomes[1].status, CommandStatus::Completed(Err(_)))
        || outcomes.iter().enumerate().any(|(index, result)| {
            index != 1 && !matches!(result.status, CommandStatus::Completed(Ok(())))
        })
    {
        return Err(format!("unexpected command results: {outcomes:?}").into());
    }
    let snapshot = rack.snapshot_patch()?;
    if snapshot.bpm != 133.0
        || snapshot
            .modules
            .iter()
            .find(|m| m.instance_name == "tm_main")
            .and_then(|m| m.parameters.get("write_probability"))
            != Some(&ParameterValue::Float(0.5))
    {
        return Err("final saved configuration has unexpected BPM or TM parameter".into());
    }
    if rack.bpm()? != 133.0 || clock_bpm(&rack)? != 120.0 {
        return Err("global BPM changed Clock parameter".into());
    }
    for event in rack.drain_midi() {
        let [a, b, c] = event.message.to_bytes();
        writeln!(
            status,
            "carried-cleanup: sample={} {a:02X} {b:02X} {c:02X}",
            event.sample
        )?;
    }

    let (trace, signals, midi) = play(&mut rack, args.samples, output)?;
    let mut fresh = install()?;
    fresh.load_patch(&args.save)?;
    let (fresh_trace, fresh_signals, fresh_midi) = play(&mut fresh, args.samples, &mut Vec::new())?;
    if trace != fresh_trace || signals != fresh_signals || midi != fresh_midi {
        return Err("fresh seeded replay differs from final saved patch".into());
    }
    let scope_records = rack.scope().ok_or("Scope missing")?.records().count();
    writeln!(
        status,
        "replay: samples={}; midi={}; scope_records={scope_records}; fresh comparison=equal; BPM={}; Clock BPM={}",
        args.samples,
        trace.len(),
        rack.bpm()?,
        clock_bpm(&rack)?
    )?;
    Ok(())
}

fn clock_bpm(rack: &Oxurack) -> Result<f32, oxurack::CoreError> {
    match rack.parameter("clock_main", "bpm")? {
        ParameterValue::Float(value) => Ok(value),
        _ => Err(oxurack::CoreError::NoPatchLoaded),
    }
}

fn main() -> ExitCode {
    match run_with_output(
        Args::parse(),
        &mut std::io::stdout().lock(),
        &mut std::io::stderr().lock(),
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("rack-session: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT: AtomicU64 = AtomicU64::new(0);

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "oxurack-rack-session-{}-{}-{name}.ron",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ))
    }

    #[test]
    fn session_writes_only_final_trace_and_compares_fresh_seeded_replay() {
        let patch = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        let save = temp_path("saved");
        let mut output = Vec::new();
        let mut status = Vec::new();
        run_with_output(
            Args {
                patch,
                save: save.clone(),
                samples: 32,
            },
            &mut output,
            &mut status,
        )
        .unwrap();
        let trace = String::from_utf8(output).unwrap();
        let status = String::from_utf8(status).unwrap();
        assert!(trace.lines().all(|line| line.contains('|')));
        assert!(status.contains("fresh comparison=equal"));
        assert!(status.contains("surviving TM handle valid"));
        assert!(save.exists());
        let _ = std::fs::remove_file(save);
    }

    #[test]
    fn session_rejects_zero_samples_and_missing_fixture() {
        let save = temp_path("invalid");
        let error = run_with_output(
            Args {
                patch: temp_path("missing"),
                save: save.clone(),
                samples: 0,
            },
            &mut Vec::new(),
            &mut Vec::new(),
        )
        .unwrap_err();
        assert!(error.to_string().contains("positive"));
        let error = run_with_output(
            Args {
                patch: temp_path("missing"),
                save: save.clone(),
                samples: 1,
            },
            &mut Vec::new(),
            &mut Vec::new(),
        )
        .unwrap_err();
        assert!(!error.to_string().is_empty());
        let _ = std::fs::remove_file(save);
    }
}
