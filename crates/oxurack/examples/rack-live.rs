use std::error::Error;
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

#[path = "rack_live/control.rs"]
mod control;
#[path = "rack_live/input.rs"]
mod input;
#[cfg(test)]
#[path = "turing_machine_midi/oracle.rs"]
mod oracle;
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "rack-live reuses the playback session; finite-runner helpers stay available to the sibling example"
    )
)]
#[path = "turing_machine_midi/playback.rs"]
mod playback;
#[path = "turing_machine_midi/workbench.rs"]
mod workbench;

#[derive(Debug, Parser)]
#[command(about = "Run an editable oxurack MIDI rack in the terminal")]
#[command(group(ArgGroup::new("output-selection").args(["output_name", "output_index"]).multiple(false)))]
struct Cli {
    /// RON patch to load.
    #[arg(
        long,
        required_unless_present = "list_outputs",
        conflicts_with = "list_outputs"
    )]
    patch: Option<PathBuf>,
    /// List current MIDI output names and exit.
    #[arg(long, conflicts_with_all = ["output_name", "output_index"])]
    list_outputs: bool,
    /// Select one exact, unambiguous MIDI output name.
    #[arg(long, value_parser = nonempty)]
    output_name: Option<String>,
    /// Select a zero-based MIDI output position from the current list.
    #[arg(long)]
    output_index: Option<usize>,
    /// Enable bounded Scope capture for the selected exact scalar ports.
    #[arg(
        long,
        value_parser = scope_selector,
        requires = "scope_view",
        conflicts_with = "list_outputs"
    )]
    scope_port: Vec<ScopeSelector>,
    /// Print a Scope workbench view; also enables MIDI-only capture with no ports.
    #[arg(long, conflicts_with = "list_outputs")]
    scope_view: bool,
    /// Maximum mixed Scope records (default 100000).
    #[arg(
        long,
        value_parser = positive_usize,
        requires = "scope_view",
        conflicts_with = "list_outputs"
    )]
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
    let value = value
        .parse::<usize>()
        .map_err(|_| "Scope capacity must be a positive integer".to_owned())?;
    if value == 0 {
        Err("Scope capacity must be positive".to_owned())
    } else {
        Ok(value)
    }
}

fn scope_selector(value: &str) -> Result<ScopeSelector, String> {
    let Some((module, port)) = value.rsplit_once('.') else {
        return Err("Scope port must be MODULE.PORT".to_owned());
    };
    if module.is_empty() || port.is_empty() {
        return Err("Scope module and port must not be empty".to_owned());
    }
    Ok(ScopeSelector::new(module, port))
}

fn prepare_rack(cli: &Cli) -> Result<Oxurack, Box<dyn Error>> {
    let patch = cli.patch.as_deref().ok_or("--patch is required")?;
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>()?;
    rack.try_install::<LfoModule>()?;
    rack.try_install::<TuringMachineModule>()?;
    rack.try_install::<MidiOutModule>()?;
    rack.load_patch(patch)?;
    rack.set_driver_mode(SimulationDriver::Manual);
    if cli.scope_view {
        rack.configure_scope(&cli.scope_port, cli.scope_capacity.unwrap_or(100_000))?;
    }
    Ok(rack)
}

fn output_names(
    output: &MidiOutput,
) -> Result<(Vec<midir::MidiOutputPort>, Vec<String>), Box<dyn Error>> {
    let ports = output.ports();
    let names = ports
        .iter()
        .map(|port| output.port_name(port))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((ports, names))
}

fn list_outputs() -> Result<(), Box<dyn Error>> {
    let output = MidiOutput::new("oxurack rack-live")?;
    let (ports, names) = output_names(&output)?;
    let mut stdout = io::stdout().lock();
    for (index, name) in names.iter().enumerate() {
        writeln!(stdout, "{index}|{name}")?;
    }
    stdout.flush()?;
    drop(ports);
    Ok(())
}

struct NativeSender(MidiOutputConnection);

impl playback::MidiSender for NativeSender {
    fn send(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        self.0.send(bytes)?;
        Ok(())
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    if cli.list_outputs {
        return list_outputs();
    }
    let patch = cli.patch.as_deref().ok_or("--patch is required")?;
    let mut rack = prepare_rack(&cli)?;
    let cancellation = Arc::new(AtomicBool::new(false));
    let handler_flag = Arc::clone(&cancellation);
    ctrlc::try_set_handler(move || handler_flag.store(true, std::sync::atomic::Ordering::SeqCst))?;
    let native_sender = if cli.output_name.is_some() || cli.output_index.is_some() {
        let output = MidiOutput::new("oxurack rack-live")?;
        let (ports, names) = output_names(&output)?;
        let selected = if let Some(name) = &cli.output_name {
            playback::select_name(&names, name).map_err(io::Error::other)?
        } else {
            playback::select_index(&names, cli.output_index.unwrap_or_default())
                .map_err(io::Error::other)?
        };
        let connection = output.connect(&ports[selected], "oxurack rack-live")?;
        Some((NativeSender(connection), selected, names[selected].clone()))
    } else {
        None
    };
    let stdout = io::stdout();
    let mut trace = BufWriter::new(stdout.lock());
    let stderr = io::stderr();
    let mut diagnostics = BufWriter::new(stderr.lock());
    writeln!(
        diagnostics,
        "rack-live: paused at sample 0; enter help for commands"
    )?;
    if let Some((_, selected, name)) = &native_sender {
        writeln!(
            diagnostics,
            "rack-live: connected to MIDI output {selected}|{name}"
        )?;
    }
    diagnostics.flush()?;
    let reader = input::Reader::spawn(io::stdin());

    let result = if let Some((sender, _, _)) = native_sender {
        let mut session = playback::Session::new(sender);
        control::run(
            &mut rack,
            patch,
            Some(&mut session),
            &mut trace,
            &mut diagnostics,
            reader.receiver(),
            &cancellation,
        )
    } else {
        control::run::<NativeSender, _, _>(
            &mut rack,
            patch,
            None,
            &mut trace,
            &mut diagnostics,
            reader.receiver(),
            &cancellation,
        )
    };
    reader.shutdown();
    result?;
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("rack-live: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_requires_exact_selection_and_scopes_only_with_capture() {
        assert!(Cli::try_parse_from(["rack-live", "--patch", "x"]).is_ok());
        assert!(Cli::try_parse_from(["rack-live", "--list-outputs"]).is_ok());
        assert!(Cli::try_parse_from(["rack-live"]).is_err());
        assert!(Cli::try_parse_from(["rack-live", "--patch", "x", "--output-name", ""]).is_err());
        assert!(
            Cli::try_parse_from([
                "rack-live",
                "--patch",
                "x",
                "--output-name",
                "A",
                "--output-index",
                "0"
            ])
            .is_err()
        );
        assert!(
            Cli::try_parse_from(["rack-live", "--patch", "x", "--scope-port", "lfo_main.out"])
                .is_err()
        );
        assert!(
            Cli::try_parse_from([
                "rack-live",
                "--patch",
                "x",
                "--scope-view",
                "--scope-port",
                "lfo_main.out",
                "--scope-capacity",
                "1"
            ])
            .is_ok()
        );
        assert!(
            Cli::try_parse_from([
                "rack-live",
                "--patch",
                "x",
                "--scope-view",
                "--scope-capacity",
                "0"
            ])
            .is_err()
        );
        assert!(Cli::try_parse_from(["rack-live", "--list-outputs", "--scope-view"]).is_err());
        Cli::command().debug_assert();
    }
}
