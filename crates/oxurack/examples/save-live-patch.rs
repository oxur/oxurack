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

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.lifecycle {
        return run_lifecycle(args);
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
    println!(
        "lfo_main.frequency_hz: {previous:?} -> 1.0\nsaved={}\ncompleted_samples={completed_samples}",
        args.save.display()
    );
    Ok(())
}

fn run_lifecycle(args: Args) -> Result<(), Box<dyn std::error::Error>> {
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
    rack.unload_patch()?;
    let cleanup: Vec<_> = rack.drain_midi().collect();
    eprintln!(
        "lifecycle: retired at sample {}; cleanup events={}",
        rack.sample_time().completed_samples(),
        cleanup.len()
    );
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
    rack.configure_scope(
        &[
            ScopeSelector::new("lfo_main", "bipolar_out"),
            ScopeSelector::new("tm_main", "write_probability"),
        ],
        21_000,
    )?;
    rack.start();
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    let mut drained = Vec::with_capacity(128);
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
            rack.record_scope_midi(event, MidiDelivery::Generated);
        }
    }
    drop(output);
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
    if !values
        .iter()
        .any(|(_, selection, value)| *selection == 1 && *value == Value::Float(0.5))
    {
        return Err("Scope did not observe effective write probability 0.5".into());
    }
    rack.reset()?;
    let final_silence: Vec<_> = rack.drain_midi().collect();
    eprintln!(
        "lifecycle: replay samples=10000; scope signals={}; final silence events={}",
        values.len(),
        final_silence.len()
    );
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
    match run(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("save-live-patch: {error}");
            ExitCode::FAILURE
        }
    }
}
