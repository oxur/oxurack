use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use clap::{ArgGroup, Parser};
use midir::{MidiOutput, MidiOutputConnection};
use oxurack::{MidiOutModule, Oxurack, ScopeSelector, SimulationDriver};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
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
    /// Export bounded signal/MIDI capture as CSV (overwrites PATH).
    #[arg(long, conflicts_with = "list_outputs")]
    scope_csv: Option<PathBuf>,
    /// Capture an exact MODULE.PORT scalar port; repeatable. Requires --scope-csv.
    #[arg(long, value_parser = scope_selector, requires = "scope_csv", conflicts_with = "list_outputs")]
    scope_port: Vec<ScopeSelector>,
    /// Maximum mixed signal/MIDI records; defaults to 100000. Requires --scope-csv.
    #[arg(long, value_parser = positive_usize, requires = "scope_csv", conflicts_with = "list_outputs")]
    scope_capacity: Option<usize>,
}

fn nonempty(value: &str) -> Result<String, String> {
    if value.is_empty() {
        Err("output name must not be empty".to_owned())
    } else {
        Ok(value.to_owned())
    }
}

fn positive_usize(value: &str) -> Result<usize, String> {
    let capacity = value
        .parse::<usize>()
        .map_err(|_| "scope capacity must be a positive integer".to_owned())?;
    if capacity == 0 {
        Err("scope capacity must be positive".to_owned())
    } else {
        Ok(capacity)
    }
}

fn scope_selector(value: &str) -> Result<ScopeSelector, String> {
    let Some((module, port)) = value.rsplit_once('.') else {
        return Err("scope port must be MODULE.PORT".to_owned());
    };
    if module.is_empty() || port.is_empty() {
        return Err("scope module and port must not be empty".to_owned());
    }
    Ok(ScopeSelector::new(module, port))
}

fn prepare_scope(cli: &Cli, rack: &mut Oxurack) -> Result<Option<BufWriter<File>>, Box<dyn Error>> {
    let Some(path) = &cli.scope_csv else {
        return Ok(None);
    };
    let capacity = cli.scope_capacity.unwrap_or(100_000);
    rack.configure_scope(&cli.scope_port, capacity)?;
    let file = File::create(path)?;
    Ok(Some(BufWriter::new(file)))
}

fn finish_scope(
    rack: &Oxurack,
    output: Option<BufWriter<File>>,
    primary: Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let Some(mut writer) = output else {
        return primary;
    };
    let mut export_errors = Vec::new();
    if let Some(capture) = rack.scope() {
        if let Err(error) = capture.write_csv(&mut writer) {
            export_errors.push(format!("CSV write failed: {error}"));
        }
        if let Err(error) = writer.flush() {
            export_errors.push(format!("CSV flush failed: {error}"));
        }
        eprintln!(
            "turing-machine-midi: Scope capacity={} retained={} overwritten={}",
            capture.capacity(),
            capture.records().count(),
            capture.overwritten()
        );
    }
    match (primary, export_errors.is_empty()) {
        (Ok(()), true) => Ok(()),
        (Err(primary), true) => Err(primary),
        (Ok(()), false) => Err(io::Error::other(export_errors.join("; ")).into()),
        (Err(primary), false) => {
            Err(io::Error::other(format!("{primary}; {}", export_errors.join("; "))).into())
        }
    }
}

fn prepare_rack(patch: &std::path::Path) -> Result<Oxurack, Box<dyn Error>> {
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>()?;
    rack.try_install::<LfoModule>()?;
    rack.try_install::<TuringMachineModule>()?;
    rack.try_install::<MidiOutModule>()?;
    rack.load_patch(patch)?;
    rack.set_driver_mode(SimulationDriver::Manual);
    Ok(rack)
}

fn run_trace(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let patch = cli.patch.as_deref().ok_or("patch is required")?;
    let mut rack = prepare_rack(patch)?;
    let capture_output = prepare_scope(cli, &mut rack)?;
    rack.start();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let result = (|| {
        for _ in 0..cli.samples {
            if rack.advance_samples(1)? == 0 {
                break;
            }
            for event in playback::drain_generated(&mut rack) {
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
    finish_scope(&rack, capture_output, result)
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
    let capture_output = prepare_scope(cli, &mut rack)?;

    let result = (|| {
        let cancellation = Arc::new(AtomicBool::new(false));
        let handler_flag = Arc::clone(&cancellation);
        ctrlc::try_set_handler(move || {
            handler_flag.store(true, std::sync::atomic::Ordering::SeqCst)
        })?;

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
        Ok::<(), Box<dyn Error>>(())
    })();
    rack.stop();
    finish_scope(&rack, capture_output, result)
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
        assert!(
            Cli::try_parse_from(["example", "--patch", "x", "--scope-csv", "capture.csv"]).is_ok()
        );
        assert!(
            Cli::try_parse_from(["example", "--patch", "x", "--scope-port", "lfo_main.out"])
                .is_err()
        );
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-csv",
                "capture.csv",
                "--scope-port",
                ".out"
            ])
            .is_err()
        );
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-csv",
                "capture.csv",
                "--scope-port",
                "lfo_main."
            ])
            .is_err()
        );
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-csv",
                "capture.csv",
                "--scope-capacity",
                "0"
            ])
            .is_err()
        );
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-csv",
                "capture.csv",
                "--scope-capacity",
                "1"
            ])
            .is_ok()
        );
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-csv",
                "capture.csv",
                "--list-outputs"
            ])
            .is_err()
        );
        let selector = scope_selector("module.with.dots.port").unwrap();
        assert_eq!(selector.module, "module.with.dots");
        assert_eq!(selector.port, "port");
        Cli::command().debug_assert();
    }

    #[test]
    fn csv_export_runs_after_primary_failure_and_keeps_that_failure() {
        let patch = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        let mut rack = prepare_rack(&patch).unwrap();
        rack.configure_scope(&[], 4).unwrap();
        let path =
            std::env::temp_dir().join(format!("oxurack-scope-export-{}.csv", std::process::id()));
        let writer = BufWriter::new(File::create(&path).unwrap());
        let primary: Box<dyn Error> = io::Error::other("primary playback failure").into();
        let error = finish_scope(&rack, Some(writer), Err(primary)).unwrap_err();
        assert!(error.to_string().contains("primary playback failure"));
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "sample,record,module,port,value_kind,value,status,data1,data2\n"
        );
        let _ = std::fs::remove_file(path);
    }
}
