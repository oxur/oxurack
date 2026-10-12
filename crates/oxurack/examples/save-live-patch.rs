use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use oxurack::{MidiOutModule, Oxurack, ParameterValue, SimulationDriver};
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
