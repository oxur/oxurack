use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use oxurack::{MidiOutModule, Oxurack, SimulationDriver};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_turingmachine::TuringMachineModule;

#[derive(Debug, Parser)]
#[command(about = "Stream a seeded oxurack Turing Machine MIDI byte trace")]
struct Cli {
    /// RON patch to load.
    #[arg(long)]
    patch: PathBuf,
    /// Number of 1 ms rack samples to advance.
    #[arg(long, default_value_t = 2000)]
    samples: u64,
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let mut rack = Oxurack::new();
    let result = (|| {
        rack.try_install::<ClockModule>()?;
        rack.try_install::<TuringMachineModule>()?;
        rack.try_install::<MidiOutModule>()?;
        rack.load_patch(&cli.patch)?;
        rack.set_driver_mode(SimulationDriver::Manual).start();
        for _ in 0..cli.samples {
            if rack.advance_samples(1)? == 0 {
                break;
            }
            for event in rack.drain_midi() {
                let [status, data1, data2] = event.message.to_bytes();
                writeln!(
                    output,
                    "{}|{status:02X} {data1:02X} {data2:02X}",
                    event.sample
                )?;
            }
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    })();
    rack.stop();
    result?;
    output.flush()?;
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("turing-machine-midi: {error}");
            ExitCode::FAILURE
        }
    }
}
