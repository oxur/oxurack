use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use oxurack::{
    CommandResult, CoreCommand, MidiDelivery, MidiOutModule, MidiOutputEvent, Oxurack,
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

struct ReservedMissingLoad {
    directory: PathBuf,
    path: PathBuf,
}

impl Drop for ReservedMissingLoad {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir(&self.directory);
    }
}

fn reserve_missing_load() -> Result<ReservedMissingLoad, Box<dyn std::error::Error>> {
    for ordinal in 0..1024 {
        let directory = std::env::temp_dir().join(format!(
            "oxurack-missing-load-{}-{ordinal}",
            std::process::id()
        ));
        match std::fs::create_dir(&directory) {
            Ok(()) => {
                return Ok(ReservedMissingLoad {
                    path: directory.join("not-created.ron"),
                    directory,
                });
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error.into()),
        }
    }
    Err("could not reserve a private missing-load directory after 1024 attempts".into())
}

fn final_cleanup(
    rack: &mut Oxurack,
    status: &mut impl Write,
    phase: &str,
) -> Result<Vec<MidiOutputEvent>, Box<dyn std::error::Error>> {
    rack.reset()?;
    let cleanup: Vec<_> = rack.drain_midi().collect();
    for event in cleanup.iter().copied() {
        let [a, b, c] = event.message.to_bytes();
        writeln!(
            status,
            "{phase}: sample={} {a:02X} {b:02X} {c:02X}",
            event.sample
        )?;
        rack.record_scope_midi(event, MidiDelivery::Generated);
    }
    rack.reset()?;
    if rack.drain_midi().next().is_some() {
        return Err(format!("{phase}: repeated reset emitted duplicate cleanup").into());
    }
    Ok(cleanup)
}

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

    let missing = reserve_missing_load()?;
    let ids = [
        rack.enqueue_command(CoreCommand::SavePatch(args.save.clone()))?,
        rack.enqueue_command(CoreCommand::LoadPatch(missing.path.clone()))?,
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
    if !matches!(outcomes[1].result, CommandResult::Completed(Err(_)))
        || outcomes.iter().enumerate().any(|(index, result)| {
            index != 1 && !matches!(result.result, CommandResult::Completed(Ok(())))
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
    drop(missing);

    let (trace, signals, midi) = play(&mut rack, args.samples, output)?;
    let mut fresh = install()?;
    fresh.load_patch(&args.save)?;
    let (fresh_trace, fresh_signals, fresh_midi) = play(&mut fresh, args.samples, &mut Vec::new())?;
    if trace != fresh_trace || signals != fresh_signals || midi != fresh_midi {
        return Err("fresh seeded replay differs from final saved patch".into());
    }
    let final_events = final_cleanup(&mut rack, status, "final-cleanup")?;
    let reference_events = final_cleanup(&mut fresh, status, "reference-cleanup")?;
    if final_events != reference_events {
        return Err("final and reference cleanup differ".into());
    }
    let scope_records = rack.scope().ok_or("Scope missing")?.records().count();
    writeln!(
        status,
        "replay: samples={}; midi={}; scope_records_after_cleanup={scope_records}; fresh comparison=equal; BPM={}; Clock BPM={}",
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
    fn session_preserves_adjacent_file_and_reports_exact_final_cleanup() {
        let patch = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        let directory = temp_path("session-dir").with_extension("dir");
        std::fs::create_dir(&directory).unwrap();
        let save = directory.join("saved.ron");
        let former_missing_sibling = save.with_extension("guaranteed-missing.ron");
        let sibling_bytes = std::fs::read(&patch).unwrap();
        std::fs::write(&former_missing_sibling, &sibling_bytes).unwrap();
        let mut output = Vec::new();
        let mut status = Vec::new();
        run_with_output(
            Args {
                patch,
                save: save.clone(),
                samples: 300,
            },
            &mut output,
            &mut status,
        )
        .unwrap();
        let trace = String::from_utf8(output).unwrap();
        let status = String::from_utf8(status).unwrap();
        assert_eq!(trace, "251|90 3E 15\n");
        let final_cleanup = "final-cleanup: sample=300 80 3E 00\nfinal-cleanup: sample=300 B0 7B 00\nfinal-cleanup: sample=300 B0 78 00\n";
        let reference_cleanup = "reference-cleanup: sample=300 80 3E 00\nreference-cleanup: sample=300 B0 7B 00\nreference-cleanup: sample=300 B0 78 00\n";
        assert!(status.contains(final_cleanup), "{status}");
        assert!(status.contains(reference_cleanup), "{status}");
        assert!(status.contains("fresh comparison=equal"));
        assert!(status.contains("surviving TM handle valid"));
        assert!(status.contains("scope_records_after_cleanup="));
        assert!(save.exists());
        assert_eq!(
            std::fs::read(&former_missing_sibling).unwrap(),
            sibling_bytes
        );
        std::fs::remove_dir_all(directory).unwrap();
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
        let directory = temp_path("unwritable-dir").with_extension("dir");
        std::fs::create_dir(&directory).unwrap();
        let error = run_with_output(
            Args {
                patch: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("examples/patches/turing-machine-lfo.ron"),
                save: directory.clone(),
                samples: 1,
            },
            &mut Vec::new(),
            &mut Vec::new(),
        )
        .unwrap_err();
        assert!(!error.to_string().is_empty());
        let _ = std::fs::remove_file(save);
        std::fs::remove_dir_all(directory).unwrap();
    }
}
