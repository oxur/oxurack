use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use oxurack::{
    CableTransform, LabelKind, LabelTarget, LabelValue, MidiDelivery, MidiOutModule, Oxurack,
    ParameterValue, ScopeRecord, ScopeSelector, SimulationDriver, Value,
};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;

#[derive(Debug, Parser)]
#[command(about = "Edit a live rack parameter and save its current configuration")]
struct Args {
    /// RON patch to load.
    #[arg(long)]
    patch: PathBuf,
    /// Destination RON patch to create or overwrite.
    #[arg(long)]
    save: PathBuf,
    /// Exercise resettable whole-rack lifetimes and emit the replay MIDI trace.
    #[arg(long)]
    lifecycle: bool,
}

fn run_with_output(
    args: Args,
    output: &mut impl Write,
    status: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.lifecycle {
        return run_lifecycle(args, output, status);
    }
    let mut rack = Oxurack::new();
    let result = (|| {
        rack.try_install::<ClockModule>()?;
        rack.try_install::<LfoModule>()?;
        rack.try_install::<TuringMachineModule>()?;
        rack.try_install::<MidiOutModule>()?;
        rack.load_patch(&args.patch)?;
        rack.set_driver_mode(SimulationDriver::Manual).start();

        advance_and_drain(&mut rack, 200)?;
        let previous = rack.parameter("lfo_main", "frequency_hz")?;
        rack.set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(1.0))?;
        rack.save_patch(&args.save)?;
        advance_and_drain(&mut rack, 100)?;
        let completed_samples = rack.sample_time().completed_samples();
        Ok::<_, Box<dyn std::error::Error>>((previous, completed_samples))
    })();
    rack.stop();
    let (previous, completed_samples) = result?;
    writeln!(
        output,
        "lfo_main.frequency_hz: {previous:?} -> 1.0\nsaved={}\ncompleted_samples={completed_samples}",
        args.save.display()
    )?;
    Ok(())
}

fn run_lifecycle(
    args: Args,
    output: &mut impl Write,
    status: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>()?;
    rack.try_install::<LfoModule>()?;
    rack.try_install::<TuringMachineModule>()?;
    rack.try_install::<MidiOutModule>()?;
    rack.load_patch(&args.patch)?;
    rack.set_driver_mode(SimulationDriver::Manual).start();
    advance_and_drain(&mut rack, 200)?;
    rack.set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(1.0))?;
    rack.set_parameter("tm_main", "write_probability", ParameterValue::Float(0.25))?;
    let source = ("lfo_main", "bipolar_out");
    let target = ("tm_main", "write_modulation");
    rack.unpatch(source, target, 0)?;
    let occurrence = rack.patch(
        source,
        target,
        Some(CableTransform::Affine {
            factor: 0.0,
            offset: 0.25,
        }),
    )?;
    let selector = LabelTarget::Cable {
        source: (source.0.into(), source.1.into()),
        target: (target.0.into(), target.1.into()),
        occurrence,
    };
    rack.set_label(
        &selector,
        LabelKind::Description,
        LabelValue::Text("Constant +0.25 write modulation".into()),
    )?;
    rack.save_patch(&args.save)?;
    let saved = rack.snapshot_patch()?;
    let cable = saved
        .cables
        .iter()
        .find(|c| {
            c.source == (source.0.into(), source.1.into())
                && c.target == (target.0.into(), target.1.into())
        })
        .ok_or("edited cable missing from snapshot")?;
    if cable.transform
        != Some(CableTransform::Affine {
            factor: 0.0,
            offset: 0.25,
        })
        || cable.description != "Constant +0.25 write modulation"
    {
        return Err("saved cable transform or label differs from requested edit".into());
    }
    for module in &saved.modules {
        if module.instance_name == "lfo_main"
            && module.parameters.get("frequency_hz") != Some(&ParameterValue::Float(1.0))
        {
            return Err("saved LFO frequency differs".into());
        }
        if module.instance_name == "tm_main"
            && module.parameters.get("write_probability") != Some(&ParameterValue::Float(0.25))
        {
            return Err("saved TM write probability differs".into());
        }
    }
    advance_and_drain(&mut rack, 100)?;
    rack.stop();
    let retired_sample = rack.sample_time().completed_samples();
    rack.unload_patch()?;
    let cleanup: Vec<_> = rack.drain_midi().collect();
    writeln!(
        status,
        "lifecycle: retired at sample {}; cleanup events={}",
        retired_sample,
        cleanup.len()
    )?;
    if retired_sample != 300 {
        return Err(format!("retired lifetime ended at unexpected sample {retired_sample}").into());
    }
    if rack.sample_time().completed_samples() != 0
        || rack.scope().is_some()
        || rack.snapshot_patch().is_ok()
    {
        return Err("unload did not produce an empty zero-sample lifetime".into());
    }
    rack.load_patch(&args.save)?;
    if rack.sample_time().completed_samples() != 0 {
        return Err("reload did not start at sample zero".into());
    }
    writeln!(
        status,
        "lifecycle: new lifetime sample={}",
        rack.sample_time().completed_samples()
    )?;
    rack.configure_scope(
        &[
            ScopeSelector::new("lfo_main", "bipolar_out"),
            ScopeSelector::new("tm_main", "write_probability"),
        ],
        21_000,
    )?;
    rack.start();
    let mut drained = Vec::with_capacity(128);
    let mut printed_trace = Vec::new();
    for _ in 0..10_000 {
        rack.advance_samples(1)?;
        drained.clear();
        drained.extend(rack.drain_midi());
        for &event in &drained {
            let [status, data1, data2] = event.message.to_bytes();
            writeln!(
                output,
                "{}|{status:02X} {data1:02X} {data2:02X}",
                event.sample
            )?;
            printed_trace.push((event.sample, status, data1, data2));
            rack.record_scope_midi(event, MidiDelivery::Generated);
        }
    }
    rack.stop();
    let capture = rack.scope().ok_or("Scope disappeared during replay")?;
    let values: Vec<_> = capture
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
    if capture.overwritten() != 0 || values.len() != 20_000 {
        return Err(format!(
            "Scope capture incomplete: records={}, overwritten={}",
            values.len(),
            capture.overwritten()
        )
        .into());
    }
    let (records, remainder) = values.as_chunks::<2>();
    if !remainder.is_empty() {
        return Err("Scope capture ended with an incomplete signal pair".into());
    }
    for (sample, record) in records.iter().enumerate() {
        if record[0].0 != sample as u64
            || record[0].1 != 0
            || record[1].0 != sample as u64
            || record[1].1 != 1
        {
            return Err(format!("Scope signal order/sample mismatch at sample {sample}").into());
        }
        let expected_probability = if sample == 0 { 0.0 } else { 0.5 };
        if record[1].2 != Value::Float(expected_probability) {
            return Err(format!("Scope probability mismatch at sample {sample}").into());
        }
    }
    let midi_records: Vec<_> = capture
        .records()
        .filter_map(|record| match record {
            ScopeRecord::Midi { event, delivery } => Some((*event, *delivery)),
            _ => None,
        })
        .collect();
    if capture.records().count() != 20_072 {
        return Err(format!(
            "Scope mixed capture incomplete: records={}",
            capture.records().count()
        )
        .into());
    }
    let trace_records = midi_records
        .iter()
        .map(|(event, _)| {
            let [status, data1, data2] = event.message.to_bytes();
            (event.sample, status, data1, data2)
        })
        .collect::<Vec<_>>();
    if midi_records.len() != 72
        || midi_records
            .iter()
            .any(|(_, delivery)| *delivery != MidiDelivery::Generated)
        || trace_records != printed_trace
    {
        return Err(format!(
            "Scope MIDI capture mismatch: records={}",
            midi_records.len()
        )
        .into());
    }
    rack.reset()?;
    let final_silence: Vec<_> = rack.drain_midi().collect();
    writeln!(
        status,
        "lifecycle: replay samples=10000; scope signals={}; final silence events={}",
        values.len(),
        final_silence.len()
    )?;
    Ok(())
}

fn advance_and_drain(rack: &mut Oxurack, samples: u64) -> Result<(), oxurack::CoreError> {
    for _ in 0..samples {
        rack.advance_samples(1)?;
        for _event in rack.drain_midi() {}
    }
    Ok(())
}

fn main() -> ExitCode {
    match run_with_output(
        Args::parse(),
        &mut std::io::stdout().lock(),
        &mut std::io::stderr().lock(),
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("save-live-patch: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    fn path(label: &str, extension: &str) -> PathBuf {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "oxurack-save-live-{label}-{}-{id}.{extension}",
            std::process::id()
        ))
    }

    fn args(patch: PathBuf, save: PathBuf, lifecycle: bool) -> Args {
        let mut argv = vec![
            std::ffi::OsString::from("save-live-patch"),
            std::ffi::OsString::from("--patch"),
            patch.into_os_string(),
            std::ffi::OsString::from("--save"),
            save.into_os_string(),
        ];
        if lifecycle {
            argv.push("--lifecycle".into());
        }
        Args::try_parse_from(argv).unwrap()
    }

    #[test]
    fn parsed_default_dispatch_preserves_summary_and_reports_writer_failure() {
        let patch = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        let save = path("default", "ron");
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        run_with_output(args(patch, save.clone(), false), &mut stdout, &mut stderr).unwrap();
        let stdout = String::from_utf8(stdout).unwrap();
        assert!(stdout.contains("lfo_main.frequency_hz:"));
        assert!(stdout.contains("completed_samples=300"));
        assert!(stderr.is_empty());
        assert!(save.exists());
        let _ = std::fs::remove_file(save);

        struct RejectWriter;
        impl Write for RejectWriter {
            fn write(&mut self, _bytes: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("controlled writer failure"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let save = path("writer-error", "ron");
        let error = run_with_output(
            args(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("examples/patches/turing-machine-lfo.ron"),
                save.clone(),
                false,
            ),
            &mut RejectWriter,
            &mut Vec::new(),
        )
        .unwrap_err();
        assert!(error.to_string().contains("controlled writer failure"));
        let _ = std::fs::remove_file(save);
    }

    #[test]
    fn parsed_lifecycle_dispatch_captures_complete_trace_and_retired_boundary() {
        let patch = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        let save = path("lifecycle", "ron");
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        run_with_output(args(patch, save.clone(), true), &mut stdout, &mut stderr).unwrap();
        let stdout = String::from_utf8(stdout).unwrap();
        let stderr = String::from_utf8(stderr).unwrap();
        assert_eq!(stdout.lines().count(), 72);
        assert!(!stdout.contains("completed_samples=300"));
        assert!(stderr.contains("retired at sample 300; cleanup events=3"));
        assert!(stderr.contains("new lifetime sample=0"));
        assert!(stderr.contains("scope signals=20000"));
        assert!(save.exists());
        let _ = std::fs::remove_file(save);
    }

    #[test]
    fn clap_rejects_missing_arguments_and_unknown_flags() {
        assert!(Args::try_parse_from(["save-live-patch", "--patch", "x"]).is_err());
        assert!(
            Args::try_parse_from(["save-live-patch", "--patch", "x", "--save", "y", "--wat"])
                .is_err()
        );
    }

    #[test]
    fn actual_dispatch_reports_missing_malformed_and_unwritable_inputs_in_both_modes() {
        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        for lifecycle in [false, true] {
            let missing = path("missing-patch", "ron");
            let error = run_with_output(
                args(missing, path("missing-save", "ron"), lifecycle),
                &mut Vec::new(),
                &mut Vec::new(),
            )
            .unwrap_err();
            assert!(
                error.to_string().contains("No such file")
                    || error.to_string().contains("cannot find")
            );

            let malformed = path("malformed-patch", "ron");
            std::fs::write(&malformed, "(not valid RON").unwrap();
            let error = run_with_output(
                args(malformed.clone(), path("malformed-save", "ron"), lifecycle),
                &mut Vec::new(),
                &mut Vec::new(),
            )
            .unwrap_err();
            assert!(
                error.to_string().to_lowercase().contains("ron")
                    || error.to_string().to_lowercase().contains("parse")
            );
            let _ = std::fs::remove_file(malformed);

            let missing_parent = path("unwritable-parent", "dir").join("save.ron");
            let error = run_with_output(
                args(fixture.clone(), missing_parent, lifecycle),
                &mut Vec::new(),
                &mut Vec::new(),
            )
            .unwrap_err();
            assert!(
                error.to_string().contains("No such file")
                    || error.to_string().contains("cannot find")
            );
        }
    }
}
