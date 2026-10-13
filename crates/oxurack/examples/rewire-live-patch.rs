use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use oxurack::{
    CableTransform, LabelKind, LabelTarget, LabelValue, MidiOutModule, Oxurack, SimulationDriver,
};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;

#[derive(Debug, Parser)]
#[command(about = "Rewire and save the live Turing Machine rack")]
struct Args {
    /// RON patch to load.
    #[arg(long)]
    patch: PathBuf,
    /// Destination RON patch to create or overwrite.
    #[arg(long)]
    save: PathBuf,
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut rack = Oxurack::new();
    let result = (|| {
        rack.try_install::<ClockModule>()?;
        rack.try_install::<LfoModule>()?;
        rack.try_install::<TuringMachineModule>()?;
        rack.try_install::<MidiOutModule>()?;
        rack.load_patch(&args.patch)?;
        rack.set_driver_mode(SimulationDriver::Manual).start();
        advance_and_drain(&mut rack, 200)?;

        let source = ("lfo_main", "bipolar_out");
        let target = ("tm_main", "write_modulation");
        rack.unpatch(source, target, 0)?;
        #[cfg(feature = "graph")]
        let after_remove = rack.graph().cables().len();
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
        let patch = rack.snapshot_patch()?;
        let cable = patch
            .cables
            .iter()
            .find(|cable| {
                cable.source == (source.0.into(), source.1.into())
                    && cable.target == (target.0.into(), target.1.into())
            })
            .ok_or("saved cable was not present in the active patch")?;
        if cable.transform
            != Some(CableTransform::Affine {
                factor: 0.0,
                offset: 0.25,
            })
        {
            return Err("saved cable transform did not match requested constant".into());
        }
        advance_and_drain(&mut rack, 100)?;
        let completed_samples = rack.sample_time().completed_samples();
        #[cfg(feature = "graph")]
        let after_add = rack.graph().cables().len();
        let graph_counts: Option<(usize, usize)> = {
            #[cfg(feature = "graph")]
            {
                Some((after_remove, after_add))
            }
            #[cfg(not(feature = "graph"))]
            {
                None
            }
        };
        Ok::<_, Box<dyn std::error::Error>>((
            occurrence,
            cable.description.clone(),
            completed_samples,
            graph_counts,
        ))
    })();
    rack.stop();
    let (occurrence, description, completed_samples, _graph_counts) = result?;
    println!(
        "completed_samples={completed_samples}\noccurrence={occurrence}\ndescription={description}\ntransform=Affine {{ factor: 0.0, offset: 0.25 }}\nsaved={}",
        args.save.display()
    );
    #[cfg(feature = "graph")]
    if let Some((after_remove, after_add)) = _graph_counts {
        println!("graph_cable_counts={after_remove}->{after_add}");
    }
    Ok(())
}

fn advance_and_drain(rack: &mut Oxurack, count: u64) -> Result<(), oxurack::CoreError> {
    for _ in 0..count {
        rack.advance_samples(1)?;
        for _event in rack.drain_midi() {}
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("rewire-live-patch: {error}");
            ExitCode::FAILURE
        }
    }
}
