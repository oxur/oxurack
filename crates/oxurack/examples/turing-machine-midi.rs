use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use clap::{ArgGroup, Parser};
use midir::{MidiOutput, MidiOutputConnection};
use oxurack::{MidiOutModule, Oxurack, SimulationDriver};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_turingmachine::TuringMachineModule;

#[cfg(test)]
#[path = "turing_machine_midi/oracle.rs"]
mod oracle;
#[path = "turing_machine_midi/playback.rs"]
mod playback;

#[derive(Debug, Parser)]
#[command(
    about = "Stream a seeded oxurack Turing Machine MIDI byte trace or play it to a MIDI output"
)]
#[command(group(ArgGroup::new("output-selection").args(["output_name", "output_index"]).multiple(false)))]
struct Cli {
    /// RON patch to load.
    #[arg(
        long,
        required_unless_present = "list_outputs",
        conflicts_with = "list_outputs"
    )]
    patch: Option<PathBuf>,
    /// Number of 1 ms rack samples to advance (default: 2000).
    #[arg(long, default_value_t = 2000)]
    samples: u64,
    /// List available MIDI outputs and exit.
    #[arg(long, conflicts_with_all = ["output_name", "output_index"])]
    list_outputs: bool,
    /// Select one exact, unambiguous MIDI output name for live playback.
    #[arg(long, conflicts_with = "list_outputs", value_parser = nonempty)]
    output_name: Option<String>,
    /// Select a MIDI output by its zero-based position in the current output list.
    #[arg(long, conflicts_with = "list_outputs")]
    output_index: Option<usize>,
}

fn nonempty(value: &str) -> Result<String, String> {
    if value.is_empty() {
        Err("output name must not be empty".to_owned())
    } else {
        Ok(value.to_owned())
    }
}

fn prepare_rack(patch: &std::path::Path) -> Result<Oxurack, Box<dyn Error>> {
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>()?;
    rack.try_install::<TuringMachineModule>()?;
    rack.try_install::<MidiOutModule>()?;
    rack.load_patch(patch)?;
    rack.set_driver_mode(SimulationDriver::Manual);
    Ok(rack)
}

fn run_trace(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let patch = cli.patch.as_deref().ok_or("patch is required")?;
    let mut rack = prepare_rack(patch)?;
    rack.start();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let result = (|| {
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
        output.flush()?;
        Ok::<(), Box<dyn Error>>(())
    })();
    rack.stop();
    result
}

fn list_outputs() -> Result<(), Box<dyn Error>> {
    let output = MidiOutput::new("oxurack turing-machine-midi")?;
    let ports = output.ports();
    let names = ports
        .iter()
        .map(|port| output.port_name(port))
        .collect::<Result<Vec<_>, _>>()?;
    let mut stdout = io::stdout().lock();
    for (index, name) in names.iter().enumerate() {
        writeln!(stdout, "{index}|{name}")?;
    }
    stdout.flush()?;
    Ok(())
}

struct NativeSender(MidiOutputConnection);

impl playback::MidiSender for NativeSender {
    fn send(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        self.0.send(bytes)?;
        Ok(())
    }
}

fn run_live(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let patch = cli.patch.as_deref().ok_or("patch is required")?;
    let mut rack = prepare_rack(patch)?;

    let cancellation = Arc::new(AtomicBool::new(false));
    let handler_flag = Arc::clone(&cancellation);
    ctrlc::try_set_handler(move || handler_flag.store(true, std::sync::atomic::Ordering::SeqCst))?;

    let output = MidiOutput::new("oxurack turing-machine-midi")?;
    let ports = output.ports();
    let mut names = Vec::with_capacity(ports.len());
    for port in &ports {
        names.push(output.port_name(port)?);
    }
    let selected = if let Some(name) = &cli.output_name {
        playback::select_name(&names, name).map_err(io::Error::other)?
    } else if let Some(index) = cli.output_index {
        playback::select_index(&names, index).map_err(io::Error::other)?
    } else {
        return Err("live playback requires --output-name or --output-index".into());
    };
    let connection = output.connect(&ports[selected], "oxurack turing-machine-midi")?;
    eprintln!(
        "turing-machine-midi: connected to MIDI output {selected}|{}",
        names[selected]
    );

    rack.start();
    let stdout = io::stdout();
    let output = stdout.lock();
    let mut session = playback::Session::new(NativeSender(connection));
    playback::play(&mut rack, &mut session, cli.samples, &cancellation, output)?;
    Ok(())
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    if cli.list_outputs {
        list_outputs()
    } else if cli.output_name.is_some() || cli.output_index.is_some() {
        run_live(&cli)
    } else {
        run_trace(&cli)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_modes_and_conflicts_are_validated() {
        assert!(Cli::try_parse_from(["example", "--list-outputs"]).is_ok());
        assert!(Cli::try_parse_from(["example"]).is_err());
        assert!(Cli::try_parse_from(["example", "--patch", "x", "--output-name", "A"]).is_ok());
        assert!(Cli::try_parse_from(["example", "--patch", "x", "--output-index", "0"]).is_ok());
        assert!(Cli::try_parse_from(["example", "--patch", "x", "--list-outputs"]).is_err());
        assert!(Cli::try_parse_from(["example", "--list-outputs", "--output-index", "0"]).is_err());
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--output-name",
                "A",
                "--output-index",
                "0"
            ])
            .is_err()
        );
        assert!(Cli::try_parse_from(["example", "--patch", "x", "--output-index", "-1"]).is_err());
        assert!(Cli::try_parse_from(["example", "--patch", "x", "--output-name", ""]).is_err());
        assert!(Cli::try_parse_from(["example", "--patch", "x", "--samples", "0"]).is_ok());
        Cli::command().debug_assert();
    }
}
