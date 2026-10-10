//! Public rack replay and lifecycle proof for the Turing Machine adapter.

use std::collections::BTreeMap;
use std::env;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use bevy_app::App;
use bevy_ecs::hierarchy::Children;
use bevy_ecs::prelude::{Entity, Query, Res, ResMut, Resource, World};
use bevy_ecs::schedule::{InternedScheduleLabel, IntoScheduleConfigs};
use bevy_time::TimeUpdateStrategy;
use oxurack::{
    CoreError, Oxurack, ParameterSchema, ParameterValue, Patch, Port, PortSchema, SampleTime,
    SimulationDriver, TickPhase, Value,
};
use oxurack_core::{
    CurrentValue, Module, OxurackModule, deserialize_patch, save_patch_to_file, spawn_module_entity,
};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_turingmachine::{Scale, StepOutputs, TuringMachine, TuringMachineModule};

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/turing-machine.ron"
);
const CHILD_ENV: &str = "OXURACK_TURING_MACHINE_CHILD";
static TEMP_PATCH_COUNTER: AtomicUsize = AtomicUsize::new(0);

const OUTPUT_NAMES: [&str; 28] = [
    "step",
    "note",
    "note_valid",
    "velocity",
    "velocity_valid",
    "gate",
    "scale_note",
    "scale_note_valid",
    "pulse_1",
    "pulse_2",
    "pulse_3",
    "pulse_4",
    "pulse_5",
    "pulse_6",
    "gate_1",
    "gate_2",
    "gate_3",
    "gate_4",
    "gate_5",
    "gate_6",
    "gate_7",
    "gate_8",
    "div2",
    "div4",
    "noise_cc",
    "register_bits",
    "length",
    "write_probability",
];

#[derive(Resource, Default, Clone, Debug, PartialEq)]
struct Trace {
    rows: BTreeMap<String, Vec<TraceRow>>,
}

#[derive(Clone, Debug, PartialEq)]
struct TraceRow {
    sample: u64,
    values: Vec<(String, Value)>,
}

struct TraceObserver;

impl OxurackModule for TraceObserver {
    const KIND: &'static str = "turing_machine_trace_consumer";
    const DISPLAY_NAME: &'static str = "Turing Machine Trace Consumer";

    fn port_schema() -> &'static [PortSchema] {
        &[]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        &[]
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        _parameters: &std::collections::HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        Ok(spawn_module_entity(world, Self::KIND, instance_name))
    }

    fn register_systems(app: &mut App, schedule: InternedScheduleLabel) {
        app.init_resource::<Trace>().add_systems(
            schedule,
            observe_turing_machine_outputs.in_set(TickPhase::Observe),
        );
    }
}

fn observe_turing_machine_outputs(
    sample_time: Res<SampleTime>,
    modules: Query<(&Module, &Children)>,
    ports: Query<(&Port, &CurrentValue)>,
    mut trace: ResMut<Trace>,
) {
    let Some(sample) = sample_time.last_index() else {
        return;
    };
    let mut found = Vec::new();
    for (module, children) in &modules {
        if module.kind.as_ref() != TuringMachineModule::KIND {
            continue;
        }
        let mut by_name = BTreeMap::new();
        for child in children.iter() {
            if let Ok((port, value)) = ports.get(*child) {
                by_name.insert(port.name.as_ref().to_owned(), value.0);
            }
        }
        let values = OUTPUT_NAMES
            .iter()
            .map(|name| {
                (
                    (*name).to_owned(),
                    *by_name
                        .get(*name)
                        .expect("Turing Machine output must be present"),
                )
            })
            .collect();
        found.push((module.instance_name.clone(), TraceRow { sample, values }));
    }
    found.sort_by(|left, right| left.0.cmp(&right.0));
    for (name, row) in found {
        trace.rows.entry(name).or_default().push(row);
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Run {
    trace: Trace,
    sample_time: SampleTime,
}

fn fixture() -> Patch {
    deserialize_patch(&std::fs::read_to_string(FIXTURE).expect("fixture should be readable"))
        .expect("Turing Machine fixture should parse")
}

fn new_rack() -> Oxurack {
    let mut rack = Oxurack::new();
    rack.install::<ClockModule>();
    rack.install::<ClockModule>();
    rack.install::<TuringMachineModule>();
    rack.install::<TuringMachineModule>();
    rack.install::<TraceObserver>();
    rack.install::<TraceObserver>();
    rack
}

fn run_patch(patch: &Patch, groups: &[u32], live: bool) -> Run {
    let path = env::temp_dir().join(format!(
        "oxurack_turing_machine_{}_{}_{}.ron",
        std::process::id(),
        TEMP_PATCH_COUNTER.fetch_add(1, Ordering::Relaxed),
        if live { "live" } else { "manual" }
    ));
    save_patch_to_file(patch, &path).expect("temporary patch should save");
    let mut rack = new_rack();
    rack.load_patch(&path)
        .expect("public Turing Machine patch load");
    let _ = std::fs::remove_file(&path);

    if live {
        rack.start();
        let mut app = rack.into_app();
        for group in groups {
            app.insert_resource(TimeUpdateStrategy::FixedTimesteps(*group));
            app.update();
        }
        let sample_time = *app.world().resource::<SampleTime>();
        let trace = app.world().resource::<Trace>().clone();
        Run { trace, sample_time }
    } else {
        rack.set_driver_mode(SimulationDriver::Manual).start();
        for group in groups {
            assert_eq!(
                rack.advance_samples(u64::from(*group))
                    .expect("manual samples should run"),
                u64::from(*group)
            );
        }
        let sample_time = rack.sample_time();
        let app = rack.into_app();
        let trace = app.world().resource::<Trace>().clone();
        Run { trace, sample_time }
    }
}

fn expected_step_values(outputs: &StepOutputs) -> Vec<(String, Value)> {
    let mut values = Vec::with_capacity(28);
    values.push(("step".to_owned(), Value::Gate(true)));
    optional_values(&mut values, "note", "note_valid", outputs.note);
    optional_values(&mut values, "velocity", "velocity_valid", outputs.velocity);
    values.push(("gate".to_owned(), Value::Gate(outputs.gate)));
    optional_values(
        &mut values,
        "scale_note",
        "scale_note_valid",
        outputs.scale_note,
    );
    for (index, value) in outputs.pulses.iter().copied().enumerate() {
        values.push((format!("pulse_{}", index + 1), Value::Gate(value)));
    }
    for (index, value) in outputs.gates.iter().copied().enumerate() {
        values.push((format!("gate_{}", index + 1), Value::Gate(value)));
    }
    values.push(("div2".to_owned(), Value::Gate(outputs.div2)));
    values.push(("div4".to_owned(), Value::Gate(outputs.div4)));
    values.push((
        "noise_cc".to_owned(),
        Value::Raw(u16::from(outputs.noise_cc)),
    ));
    values.push((
        "register_bits".to_owned(),
        Value::Raw(outputs.register_bits),
    ));
    values.push((
        "length".to_owned(),
        Value::Raw(u16::try_from(outputs.length).expect("engine length fits")),
    ));
    values.push((
        "write_probability".to_owned(),
        Value::Float(outputs.write_probability),
    ));
    values
}

fn optional_values(
    values: &mut Vec<(String, Value)>,
    name: &str,
    valid_name: &str,
    value: Option<u8>,
) {
    values.push((
        name.to_owned(),
        Value::Raw(u16::from(value.unwrap_or_default())),
    ));
    values.push((valid_name.to_owned(), Value::Gate(value.is_some())));
}

fn configured_engine(master_seed: u64, instance_name: &str, primary: bool) -> TuringMachine {
    let mut engine = TuringMachine::with_seed(
        oxurack_core::ModuleSpawnContext::new(master_seed, instance_name).instance_seed(),
    );
    if primary {
        engine.set_write(0.75);
        engine.set_length(12);
        engine.set_length_position(3);
        engine.set_scale(Scale::new(vec![0, 2, 4, 7, 9], "Primary Five"));
        engine.set_root(5);
        engine.set_note_range(40..=76);
        engine.set_scale_output_scale(Scale::new(vec![0, 3, 5, 7, 10], "Primary Blues"));
        engine.set_scale_output_root(8);
    } else {
        engine.set_write(0.25);
        engine.set_length(8);
        engine.set_scale(Scale::new(vec![0, 2, 3, 5, 7, 10], "Secondary Minor"));
        engine.set_root(1);
        engine.set_note_range(48..=72);
        engine.set_scale_output_scale(Scale::new(vec![0, 2, 4, 6, 8, 10], "Secondary Whole"));
        engine.set_scale_output_root(6);
    }
    engine
}

fn assert_delayed_standalone_match(run: &Run) {
    assert_delayed_standalone_match_for(run, 42, "tm_primary");
}

fn assert_delayed_standalone_match_for(run: &Run, master_seed: u64, primary_name: &str) {
    for (name, primary) in [(primary_name, true), ("tm_secondary", false)] {
        let rows = run.trace.rows.get(name).expect("TM trace should exist");
        assert_eq!(rows.len(), 24, "complete Observe trace for {name}");
        for (sample, row) in rows.iter().enumerate() {
            assert_eq!(row.sample, sample as u64);
            assert_eq!(row.values.len(), 28);
            assert_eq!(
                row.values
                    .iter()
                    .map(|(name, _)| name.as_str())
                    .collect::<Vec<_>>(),
                OUTPUT_NAMES
            );
        }
        let mut engine = configured_engine(master_seed, name, primary);
        let mut pending = None;
        let mut last_published = expected_step_values(&StepOutputs::default());
        last_published[0].1 = Value::Gate(false);
        for sample in 0..24_u64 {
            let expected = if sample == 0 {
                last_published.clone()
            } else if sample % 2 == 1 {
                last_published = expected_step_values(
                    pending
                        .take()
                        .as_ref()
                        .expect("delayed odd sample has pending step"),
                );
                last_published.clone()
            } else {
                let mut held = last_published.clone();
                held[0].1 = Value::Gate(false);
                held
            };
            assert_eq!(
                rows[sample as usize].values, expected,
                "standalone full-sample oracle differs for {name} at sample {sample}",
            );
            if sample % 2 == 0 {
                pending = Some(engine.tick());
            }
        }
    }
}

#[test]
fn public_fixture_has_complete_delayed_trace_and_standalone_oracle() {
    let patch = fixture();
    assert_eq!(patch.master_seed, 42);
    assert_eq!(
        patch.modules.iter().filter(|m| m.kind == "clock").count(),
        2
    );
    assert_eq!(
        patch
            .modules
            .iter()
            .filter(|m| m.kind == "turing_machine")
            .count(),
        2
    );
    assert!(patch.cables.iter().all(|cable| cable.source.1 == "pulse"));
    let run = run_patch(&patch, &[24], false);
    assert_eq!(run.sample_time.completed_samples(), 24);
    for rows in run.trace.rows.values() {
        assert_eq!(rows.len(), 24);
    }
    assert_eq!(
        run.trace.rows["tm_primary"]
            .iter()
            .filter_map(|row| match row.values[0].1 {
                Value::Gate(true) => Some(row.sample),
                _ => None,
            })
            .collect::<Vec<_>>(),
        vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23]
    );
    assert_delayed_standalone_match(&run);
}

#[test]
fn manual_live_groupings_and_reversed_patch_order_match() {
    let patch = fixture();
    let manual_one = run_patch(&patch, &[24], false);
    let manual_groups = run_patch(&patch, &[1, 2, 3, 5, 13], false);
    let live_one = run_patch(
        &patch,
        &[
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
        true,
    );
    let live_groups = run_patch(&patch, &[1, 2, 3, 5, 13], true);
    assert_eq!(manual_one, manual_groups);
    assert_eq!(manual_one, live_one);
    assert_eq!(manual_one, live_groups);

    let mut reversed = patch.clone();
    reversed.modules.reverse();
    reversed.cables.reverse();
    assert_eq!(run_patch(&reversed, &[24], false), manual_one);

    let mut inserted = patch.clone();
    let mut extra_observer = inserted
        .modules
        .iter()
        .find(|module| module.kind == TraceObserver::KIND)
        .expect("fixture observer")
        .clone();
    extra_observer.instance_name = "trace_extra".to_owned();
    let first_tm = inserted
        .modules
        .iter()
        .position(|module| module.kind == TuringMachineModule::KIND)
        .expect("fixture TM");
    inserted.modules.insert(first_tm, extra_observer);
    assert_eq!(run_patch(&inserted, &[24], false), manual_one);

    let mut removed = patch.clone();
    removed
        .modules
        .retain(|module| module.instance_name != "trace_unrelated");
    assert_eq!(run_patch(&removed, &[24], false), manual_one);
}

#[test]
fn changed_seed_and_instance_name_change_trace() {
    let patch = fixture();
    let baseline = run_patch(&patch, &[24], false);
    assert_delayed_standalone_match(&baseline);

    let mut seed_changed = patch.clone();
    seed_changed.master_seed = 43;
    let changed_seed_run = run_patch(&seed_changed, &[24], false);
    assert_delayed_standalone_match_for(&changed_seed_run, 43, "tm_primary");
    assert_eq!(
        changed_seed_run.trace.rows["tm_primary"].len(),
        baseline.trace.rows["tm_primary"].len()
    );
    assert_eq!(
        changed_seed_run.trace.rows["tm_secondary"].len(),
        baseline.trace.rows["tm_secondary"].len()
    );
    assert!(
        baseline.trace.rows["tm_primary"]
            .iter()
            .zip(&changed_seed_run.trace.rows["tm_primary"])
            .any(|(before, after)| before.values != after.values),
        "changed master seed must change primary output contents"
    );

    let mut name_changed = patch.clone();
    for module in &mut name_changed.modules {
        if module.instance_name == "tm_primary" {
            module.instance_name = "tm_renamed".to_owned();
        }
    }
    for cable in &mut name_changed.cables {
        if cable.target.0 == "tm_primary" {
            cable.target.0 = "tm_renamed".to_owned();
        }
    }
    let renamed_run = run_patch(&name_changed, &[24], false);
    assert_delayed_standalone_match_for(&renamed_run, 42, "tm_renamed");
    let baseline_rows = &baseline.trace.rows["tm_primary"];
    let renamed_rows = &renamed_run.trace.rows["tm_renamed"];
    assert_eq!(baseline_rows.len(), renamed_rows.len());
    assert!(
        baseline_rows
            .iter()
            .zip(renamed_rows)
            .any(|(a, b)| a.values != b.values)
    );
    assert_eq!(
        baseline.trace.rows["tm_secondary"],
        renamed_run.trace.rows["tm_secondary"]
    );
}

#[test]
fn stop_pending_resume_and_reinstall_preserve_progress() {
    fn collect(rack: Oxurack) -> Run {
        let app = rack.into_app();
        Run {
            trace: app.world().resource::<Trace>().clone(),
            sample_time: *app.world().resource::<SampleTime>(),
        }
    }
    fn assert_stopped(rack: &mut Oxurack) {
        let time = rack.sample_time();
        assert_eq!(rack.advance_samples(0).unwrap(), 0);
        assert_eq!(rack.advance_samples(3).unwrap(), 0);
        assert_eq!(
            rack.sample_time(),
            time,
            "stopped calls leave SampleTime unchanged"
        );
    }

    let uninterrupted = run_patch(&fixture(), &[24], false);
    assert_delayed_standalone_match(&uninterrupted);

    // Stop after Consume created the sample-0 snapshot but before Produce 1.
    // Clock is high at this boundary and its pending TM snapshot must survive.
    let mut pending = new_rack();
    pending.load_patch(Path::new(FIXTURE)).unwrap();
    pending.set_driver_mode(SimulationDriver::Manual).start();
    assert_eq!(pending.advance_samples(1).unwrap(), 1);
    pending.stop();
    assert_stopped(&mut pending);
    pending.install::<ClockModule>();
    pending.install::<TuringMachineModule>();
    pending.start();
    assert_eq!(pending.advance_samples(23).unwrap(), 23);
    pending.stop();
    let pending_run = collect(pending);
    assert_eq!(
        pending_run, uninterrupted,
        "pending snapshot and idempotent reinstall preserve full trace"
    );
    assert_delayed_standalone_match(&pending_run);

    // Pause after sample 1 published a step; resume through the same remaining
    // 22 executed samples and compare every named value and time index.
    let mut published = new_rack();
    published.load_patch(Path::new(FIXTURE)).unwrap();
    published.set_driver_mode(SimulationDriver::Manual).start();
    assert_eq!(published.advance_samples(2).unwrap(), 2);
    published.stop();
    assert_stopped(&mut published);
    published.start();
    assert_eq!(published.advance_samples(22).unwrap(), 22);
    published.stop();
    let published_run = collect(published);
    assert_eq!(
        published_run, uninterrupted,
        "pause after publication preserves continued lifecycle"
    );
    assert_delayed_standalone_match(&published_run);

    // Exercise the high Clock boundary independently from the pending/reinstall
    // case: a held high across stop/resume cannot create an additional edge.
    let mut high_boundary = new_rack();
    high_boundary.load_patch(Path::new(FIXTURE)).unwrap();
    high_boundary
        .set_driver_mode(SimulationDriver::Manual)
        .start();
    assert_eq!(high_boundary.advance_samples(1).unwrap(), 1);
    high_boundary.stop();
    assert_stopped(&mut high_boundary);
    high_boundary.start();
    assert_eq!(high_boundary.advance_samples(23).unwrap(), 23);
    high_boundary.stop();
    let high_run = collect(high_boundary);
    assert_eq!(
        high_run, uninterrupted,
        "held Clock input does not retrigger on resume"
    );
    assert_delayed_standalone_match(&high_run);
}

#[test]
fn fresh_process_replays_canonical_trace_twice() {
    if let Some(mode) = env::var_os(CHILD_ENV) {
        let mut patch = fixture();
        match mode.to_string_lossy().as_ref() {
            "seed43" => patch.master_seed = 43,
            "renamed" => {
                for module in &mut patch.modules {
                    if module.instance_name == "tm_primary" {
                        module.instance_name = "tm_renamed".to_owned();
                    }
                }
                for cable in &mut patch.cables {
                    if cable.target.0 == "tm_primary" {
                        cable.target.0 = "tm_renamed".to_owned();
                    }
                }
            }
            "canonical" => {}
            other => panic!("unknown child mode {other}"),
        }
        let run = run_patch(&patch, &[24], false);
        println!("TRACE_BEGIN");
        for (name, rows) in run.trace.rows {
            for row in rows {
                println!("{name}|{}|{:?}", row.sample, row.values);
            }
        }
        println!("TRACE_END");
        return;
    }

    fn child(mode: &str) -> String {
        let output = Command::new(env::current_exe().expect("test executable path"))
            .args([
                "--exact",
                "fresh_process_replays_canonical_trace_twice",
                "--nocapture",
            ])
            .env(CHILD_ENV, mode)
            .output()
            .expect("child process should start");
        assert!(
            output.status.success(),
            "child failed: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).expect("child stdout should be UTF-8");
        let begin = stdout.find("TRACE_BEGIN\n").expect("trace begin marker");
        let end = stdout[begin..]
            .find("TRACE_END\n")
            .expect("trace end marker");
        stdout[begin + "TRACE_BEGIN\n".len()..begin + end].to_owned()
    }

    fn rows(trace: &str) -> BTreeMap<(String, u64), String> {
        trace
            .lines()
            .map(|line| {
                let mut parts = line.splitn(3, '|');
                let name = parts.next().unwrap().to_owned();
                let sample = parts.next().unwrap().parse().unwrap();
                let values = parts.next().unwrap().to_owned();
                ((name, sample), values)
            })
            .collect()
    }

    let canonical = child("canonical");
    assert_eq!(canonical, child("canonical"));
    let canonical_rows = rows(&canonical);
    let changed_seed_rows = rows(&child("seed43"));
    assert!(
        canonical_rows.iter().any(|((name, sample), values)| {
            name == "tm_primary" && changed_seed_rows.get(&(name.clone(), *sample)) != Some(values)
        }),
        "fresh child with master seed 43 must change corresponding primary content"
    );
    let renamed_rows = rows(&child("renamed"));
    let primary_changed = (0..24_u64).any(|sample| {
        canonical_rows.get(&("tm_primary".to_owned(), sample))
            != renamed_rows.get(&("tm_renamed".to_owned(), sample))
    });
    assert!(
        primary_changed,
        "fresh child rename must change corresponding output content"
    );
    for sample in 0..24_u64 {
        assert_eq!(
            canonical_rows.get(&("tm_secondary".to_owned(), sample)),
            renamed_rows.get(&("tm_secondary".to_owned(), sample)),
            "renaming primary must preserve secondary sample {sample}",
        );
    }
}
