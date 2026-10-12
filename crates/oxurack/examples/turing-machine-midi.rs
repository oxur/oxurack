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
#[path = "turing_machine_midi/workbench.rs"]
mod workbench;

#[derive(Debug, Parser)]
#[command(
    about = "Stream a seeded oxurack Turing Machine MIDI byte trace or play it to a MIDI output"
)]
#[command(group(ArgGroup::new("output-selection").args(["output_name", "output_index"]).multiple(false)))]
#[command(group(ArgGroup::new("scope-output").args(["scope_csv", "scope_view"]).multiple(true)))]
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
    /// Capture an exact MODULE.PORT scalar port; repeatable. Requires CSV or view output.
    #[arg(long, value_parser = scope_selector, requires = "scope-output", conflicts_with = "list_outputs")]
    scope_port: Vec<ScopeSelector>,
    /// Maximum mixed signal/MIDI records; defaults to 100000. Requires CSV or view output.
    #[arg(long, value_parser = positive_usize, requires = "scope-output", conflicts_with = "list_outputs")]
    scope_capacity: Option<usize>,
    /// Print the retained Scope workbench view to stderr after the run.
    #[arg(long, conflicts_with = "list_outputs")]
    scope_view: bool,
    /// Number of retained signal and MIDI rows in each view tail (1..=1000; default: 12).
    #[arg(long, value_parser = scope_rows, requires = "scope_view", conflicts_with = "list_outputs")]
    scope_rows: Option<usize>,
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

fn scope_rows(value: &str) -> Result<usize, String> {
    let rows = value
        .parse::<usize>()
        .map_err(|_| "scope rows must be an integer from 1 to 1000".to_owned())?;
    if (1..=1000).contains(&rows) {
        Ok(rows)
    } else {
        Err("scope rows must be from 1 to 1000".to_owned())
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
    if cli.scope_csv.is_none() && !cli.scope_view {
        return Ok(None);
    }
    let capacity = cli.scope_capacity.unwrap_or(100_000);
    rack.configure_scope(&cli.scope_port, capacity)?;
    cli.scope_csv
        .as_ref()
        .map(|path| File::create(path).map(BufWriter::new).map_err(Into::into))
        .transpose()
}

fn finalize_scope<C: Write, V: Write>(
    cli: &Cli,
    rack: &Oxurack,
    mut csv: Option<C>,
    view: &mut V,
    primary: Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let mut report_errors = Vec::new();
    if let Some(capture) = rack.scope() {
        if let Some(writer) = &mut csv {
            if let Err(error) = capture.write_csv(writer) {
                report_errors.push(format!("CSV write failed: {error}"));
            }
            if let Err(error) = writer.flush() {
                report_errors.push(format!("CSV flush failed: {error}"));
            }
        }
        if cli.scope_view {
            if let Some(patch) = cli.patch.as_deref() {
                if let Err(error) = workbench::write_view(
                    view,
                    patch,
                    rack.sample_time(),
                    capture,
                    cli.scope_rows.unwrap_or(12),
                ) {
                    report_errors.push(format!("Scope view write failed: {error}"));
                }
            } else {
                report_errors.push("Scope view write failed: patch is unavailable".to_owned());
            }
            if let Err(error) = view.flush() {
                report_errors.push(format!("Scope view flush failed: {error}"));
            }
        }
        if csv.is_some() {
            eprintln!(
                "turing-machine-midi: Scope capacity={} retained={} overwritten={}",
                capture.capacity(),
                capture.records().count(),
                capture.overwritten()
            );
        }
    } else if csv.is_some() || cli.scope_view {
        report_errors.push("Scope report failed: capture is unavailable".to_owned());
    }
    match (primary, report_errors.is_empty()) {
        (Ok(()), true) => Ok(()),
        (Err(primary), true) => Err(primary),
        (Ok(()), false) => Err(io::Error::other(report_errors.join("; ")).into()),
        (Err(primary), false) => {
            Err(io::Error::other(format!("{primary}; {}", report_errors.join("; "))).into())
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
    finalize_scope(cli, &rack, capture_output, &mut io::stderr().lock(), result)
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
    finalize_scope(cli, &rack, capture_output, &mut io::stderr().lock(), result)
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
        assert!(Cli::try_parse_from(["example", "--patch", "x", "--scope-view"]).is_ok());
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-view",
                "--scope-rows",
                "1"
            ])
            .is_ok()
        );
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-view",
                "--scope-rows",
                "1000"
            ])
            .is_ok()
        );
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-view",
                "--scope-rows",
                "0"
            ])
            .is_err()
        );
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-view",
                "--scope-rows",
                "1001"
            ])
            .is_err()
        );
        assert!(Cli::try_parse_from(["example", "--patch", "x", "--scope-rows", "2"]).is_err());
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-view",
                "--scope-csv",
                "capture.csv"
            ])
            .is_ok()
        );
        let combined = Cli::try_parse_from([
            "example",
            "--patch",
            "x",
            "--scope-view",
            "--scope-port",
            "module.with.dots.port",
            "--scope-port",
            "lfo_main.bipolar_out",
            "--scope-capacity",
            "1",
        ])
        .unwrap();
        assert_eq!(combined.scope_rows.unwrap_or(12), 12);
        assert_eq!(combined.scope_port[0].module, "module.with.dots");
        assert_eq!(combined.scope_port[1].module, "lfo_main");
        assert!(Cli::try_parse_from(["example", "--patch", "x", "--scope-capacity", "4"]).is_err());
        assert!(
            Cli::try_parse_from(["example", "--patch", "x", "--scope-view", "--list-outputs"])
                .is_err()
        );
        assert!(
            Cli::try_parse_from([
                "example",
                "--patch",
                "x",
                "--scope-view",
                "--scope-rows",
                "2",
                "--list-outputs"
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
        let cli = Cli::try_parse_from([
            "example",
            "--patch",
            patch.to_str().unwrap(),
            "--scope-csv",
            path.to_str().unwrap(),
        ])
        .unwrap();
        let error =
            finalize_scope(&cli, &rack, Some(writer), &mut Vec::new(), Err(primary)).unwrap_err();
        assert!(error.to_string().contains("primary playback failure"));
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "sample,record,module,port,value_kind,value,status,data1,data2\n"
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn scope_preflight_rejects_selector_and_file_before_start() {
        let patch = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        let mut rack = prepare_rack(&patch).unwrap();
        let selector_cli = Cli::try_parse_from([
            "example",
            "--patch",
            patch.to_str().unwrap(),
            "--scope-view",
            "--scope-port",
            "unknown.port",
        ])
        .unwrap();
        assert!(prepare_scope(&selector_cli, &mut rack).is_err());
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert!(rack.scope().is_none());

        let bad_path = std::env::temp_dir().join(format!(
            "oxurack-missing-dir-{}/capture.csv",
            std::process::id()
        ));
        let file_cli = Cli::try_parse_from([
            "example",
            "--patch",
            patch.to_str().unwrap(),
            "--scope-csv",
            bad_path.to_str().unwrap(),
        ])
        .unwrap();
        assert!(prepare_scope(&file_cli, &mut rack).is_err());
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert_eq!(rack.advance_samples(1).unwrap(), 0);
    }

    struct FailingWriter {
        label: &'static str,
        calls: Arc<std::sync::Mutex<Vec<String>>>,
    }

    impl Write for FailingWriter {
        fn write(&mut self, _bytes: &[u8]) -> io::Result<usize> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("{} write", self.label));
            Err(io::Error::other(format!("{} write injected", self.label)))
        }

        fn flush(&mut self) -> io::Result<()> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("{} flush", self.label));
            Err(io::Error::other(format!("{} flush injected", self.label)))
        }
    }

    #[test]
    fn finalizer_attempts_each_report_in_order_and_preserves_primary_failure() {
        let patch = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        let cli = Cli::try_parse_from([
            "example",
            "--patch",
            patch.to_str().unwrap(),
            "--scope-csv",
            "capture.csv",
            "--scope-view",
        ])
        .unwrap();
        let mut rack = prepare_rack(&patch).unwrap();
        rack.configure_scope(&[], 4).unwrap();
        let calls = Arc::new(std::sync::Mutex::new(Vec::new()));
        let csv = FailingWriter {
            label: "csv",
            calls: Arc::clone(&calls),
        };
        let mut view = FailingWriter {
            label: "view",
            calls: Arc::clone(&calls),
        };
        let primary: Box<dyn Error> = io::Error::other("primary playback failure").into();
        let error = finalize_scope(&cli, &rack, Some(csv), &mut view, Err(primary)).unwrap_err();
        let message = error.to_string();
        assert!(
            message.find("primary playback failure").unwrap()
                < message.find("CSV write failed").unwrap()
        );
        assert!(
            message.find("CSV write failed").unwrap() < message.find("CSV flush failed").unwrap()
        );
        assert!(
            message.find("CSV flush failed").unwrap()
                < message.find("Scope view write failed").unwrap()
        );
        assert!(
            message.find("Scope view write failed").unwrap()
                < message.find("Scope view flush failed").unwrap()
        );
        assert_eq!(
            *calls.lock().unwrap(),
            ["csv write", "csv flush", "view write", "view flush"]
        );
    }

    #[test]
    fn finalizer_keeps_one_successful_report_when_the_other_output_fails() {
        let patch = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        let cli = Cli::try_parse_from([
            "example",
            "--patch",
            patch.to_str().unwrap(),
            "--scope-csv",
            "capture.csv",
            "--scope-view",
        ])
        .unwrap();
        let mut rack = prepare_rack(&patch).unwrap();
        rack.configure_scope(&[], 4).unwrap();

        let mut view = FailingWriter {
            label: "view",
            calls: Arc::default(),
        };
        let mut csv = Vec::new();
        assert!(finalize_scope(&cli, &rack, Some(&mut csv), &mut view, Ok(())).is_err());
        assert!(
            csv.starts_with(b"sample,record,module,port,value_kind,value,status,data1,data2\n")
        );

        let failed_csv = FailingWriter {
            label: "csv",
            calls: Arc::default(),
        };
        let mut view = Vec::new();
        assert!(finalize_scope(&cli, &rack, Some(failed_csv), &mut view, Ok(())).is_err());
        assert!(
            String::from_utf8(view)
                .unwrap()
                .starts_with("Scope workbench\n")
        );
    }

    struct TestSender {
        calls: Arc<std::sync::Mutex<Vec<[u8; 3]>>>,
        fail_at: Option<usize>,
        cancel_after: Option<(Arc<AtomicBool>, usize)>,
    }

    impl playback::MidiSender for TestSender {
        fn send(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
            let mut calls = self.calls.lock().unwrap();
            let index = calls.len();
            let event = [bytes[0], bytes[1], bytes[2]];
            calls.push(event);
            if self.fail_at == Some(index) {
                Err(format!("injected send failure at {index}").into())
            } else {
                if let Some((flag, after)) = &self.cancel_after
                    && index == *after
                {
                    flag.store(true, std::sync::atomic::Ordering::SeqCst);
                }
                Ok(())
            }
        }
    }

    #[test]
    fn actual_play_failure_is_finalized_after_cleanup_and_keeps_generated_sent_prefix() {
        let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-midi.ron");
        let mut parsed =
            oxurack_core::deserialize_patch(&std::fs::read_to_string(&source).unwrap()).unwrap();
        parsed.modules[0]
            .parameters
            .insert("bpm".into(), oxurack::ParameterValue::Float(1000.0));
        parsed.modules[0]
            .parameters
            .insert("ppqn".into(), oxurack::ParameterValue::Int(30));
        let fixture =
            std::env::temp_dir().join(format!("oxurack-finalizer-{}.ron", std::process::id()));
        oxurack_core::save_patch_to_file(&parsed, &fixture).unwrap();
        let mut rack = prepare_rack(&fixture).unwrap();
        rack.configure_scope(&[], 128).unwrap().start();
        let sender_calls = Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut session = playback::Session::new(TestSender {
            calls: Arc::clone(&sender_calls),
            fail_at: Some(1),
            cancel_after: None,
        });
        let error = playback::play(
            &mut rack,
            &mut session,
            200,
            &AtomicBool::new(false),
            Vec::new(),
        )
        .unwrap_err();
        rack.stop();
        assert!(error.to_string().contains("injected send failure at 1"));
        let records: Vec<_> = rack.scope().unwrap().records().copied().collect();
        let generated = records
            .iter()
            .filter(|record| {
                matches!(
                    record,
                    oxurack::ScopeRecord::Midi {
                        delivery: oxurack::MidiDelivery::Generated,
                        ..
                    }
                )
            })
            .count();
        let sent = records
            .iter()
            .filter(|record| {
                matches!(
                    record,
                    oxurack::ScopeRecord::Midi {
                        delivery: oxurack::MidiDelivery::Sent,
                        ..
                    }
                )
            })
            .count();
        assert!(generated >= 2);
        assert_eq!(sent, 1);
        assert!(
            sender_calls.lock().unwrap().len() > sent,
            "cleanup sends precede finalization but stay outside Scope"
        );
        let cli = Cli::try_parse_from([
            "example",
            "--patch",
            fixture.to_str().unwrap(),
            "--scope-view",
        ])
        .unwrap();
        let mut view = Vec::new();
        let final_error =
            finalize_scope(&cli, &rack, None::<Vec<u8>>, &mut view, Err(error.into())).unwrap_err();
        assert!(
            final_error
                .to_string()
                .contains("injected send failure at 1")
        );
        let view = String::from_utf8(view).unwrap();
        assert!(view.contains(&format!("generated={generated} sent={sent}")));
        assert!(view.contains("recent MIDI"));
        let _ = std::fs::remove_file(fixture);
    }

    #[test]
    fn actual_cancelled_play_renders_stopped_partial_capture_without_more_drain() {
        let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-midi.ron");
        let mut parsed =
            oxurack_core::deserialize_patch(&std::fs::read_to_string(&source).unwrap()).unwrap();
        parsed.modules[0]
            .parameters
            .insert("bpm".into(), oxurack::ParameterValue::Float(1000.0));
        parsed.modules[0]
            .parameters
            .insert("ppqn".into(), oxurack::ParameterValue::Int(30));
        let fixture = std::env::temp_dir().join(format!(
            "oxurack-cancelled-workbench-{}.ron",
            std::process::id()
        ));
        oxurack_core::save_patch_to_file(&parsed, &fixture).unwrap();
        let mut rack = prepare_rack(&fixture).unwrap();
        rack.configure_scope(&[], 128).unwrap().start();

        let cancellation = Arc::new(AtomicBool::new(false));
        let sender_calls = Arc::new(std::sync::Mutex::new(Vec::new()));
        let sender = TestSender {
            calls: Arc::clone(&sender_calls),
            fail_at: None,
            cancel_after: Some((Arc::clone(&cancellation), 0)),
        };
        let mut session = playback::Session::new(sender);
        playback::play(&mut rack, &mut session, 200, &cancellation, Vec::new()).unwrap();
        assert_eq!(rack.advance_samples(1).unwrap(), 0);
        assert_eq!(rack.drain_midi().count(), 0);

        let records: Vec<_> = rack.scope().unwrap().records().copied().collect();
        let generated = records
            .iter()
            .filter(|record| {
                matches!(
                    record,
                    oxurack::ScopeRecord::Midi {
                        delivery: oxurack::MidiDelivery::Generated,
                        ..
                    }
                )
            })
            .count();
        let sent = records
            .iter()
            .filter(|record| {
                matches!(
                    record,
                    oxurack::ScopeRecord::Midi {
                        delivery: oxurack::MidiDelivery::Sent,
                        ..
                    }
                )
            })
            .count();
        assert!(generated >= sent);
        assert_eq!(sent, 1);
        assert!(sender_calls.lock().unwrap().len() > sent);

        let cli = Cli::try_parse_from([
            "example",
            "--patch",
            fixture.to_str().unwrap(),
            "--scope-view",
        ])
        .unwrap();
        let mut view = Vec::new();
        finalize_scope(&cli, &rack, None::<Vec<u8>>, &mut view, Ok(())).unwrap();
        let view = String::from_utf8(view).unwrap();
        assert!(view.contains(&format!("generated={generated} sent={sent}")));
        assert!(view.contains("recent MIDI"));
        let _ = std::fs::remove_file(fixture);
    }

    #[test]
    fn clear_and_continue_recipe_drains_each_sample_and_preserves_phase()
    -> Result<(), Box<dyn Error>> {
        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        let mut rack = prepare_rack(&fixture)?;
        rack.configure_scope(&[ScopeSelector::new("lfo_main", "bipolar_out")], 1000)?;
        rack.start();
        for _ in 0..200 {
            assert_eq!(rack.advance_samples(1)?, 1);
            let events: Vec<_> = rack.drain_midi().collect();
            for event in events {
                rack.record_scope_midi(event, oxurack::MidiDelivery::Generated);
            }
        }
        rack.stop();
        assert_eq!(rack.sample_time().completed_samples(), 200);

        rack.clear_scope();
        assert!(rack.scope().is_some());
        assert_eq!(rack.scope().unwrap().records().count(), 0);
        assert_eq!(rack.scope().unwrap().overwritten(), 0);

        rack.start();
        for _ in 0..100 {
            if rack.advance_samples(1)? == 0 {
                break;
            }
            let events: Vec<_> = rack.drain_midi().collect();
            for event in events {
                rack.record_scope_midi(event, oxurack::MidiDelivery::Generated);
            }
        }
        rack.stop();

        assert_eq!(rack.sample_time().completed_samples(), 300);
        assert!(rack.midi_output_error().is_none());
        assert_eq!(rack.drain_midi().count(), 0);
        let capture = rack.scope().unwrap();
        assert_eq!(capture.overwritten(), 0);
        let records: Vec<_> = capture.records().copied().collect();
        assert_eq!(records.len(), 101);

        let mut signal_samples = Vec::new();
        let mut generated_events = Vec::new();
        let mut sent_events = 0;
        for record in records {
            match record {
                oxurack::ScopeRecord::Signal { sample, .. } => signal_samples.push(sample),
                oxurack::ScopeRecord::Midi {
                    event,
                    delivery: oxurack::MidiDelivery::Generated,
                } => generated_events.push((event.sample, event.message.to_bytes())),
                oxurack::ScopeRecord::Midi {
                    delivery: oxurack::MidiDelivery::Sent,
                    ..
                } => sent_events += 1,
                _ => panic!("unexpected future Scope record or delivery variant"),
            }
        }
        assert_eq!(signal_samples, (200..300).collect::<Vec<_>>());
        assert_eq!(generated_events, [(251, [0x90, 0x3e, 0x15])]);
        assert_eq!(sent_events, 0);
        Ok(())
    }
}
