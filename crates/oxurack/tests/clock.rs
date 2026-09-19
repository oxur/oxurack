use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy_ecs::prelude::{Commands, Component, Entity, Query, Res, ResMut, Resource, World};
use bevy_time::{Time, TimeUpdateStrategy, Virtual};
use oxurack::{
    CoreError, MergePolicy, Oxurack, ParameterSchema, ParameterValue, PortDirection, PortSchema,
    SampleTime, SimulationDriver, TickPhase, Value, ValueKind,
};
use oxurack_core::{
    CurrentValue, Module, OxurackModule, save_patch_to_file, spawn_module_entity,
    spawn_port_on_module,
};
use oxurack_mod_clock::ClockModule;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/clock.ron");
const MILLIS_PER_MINUTE: u128 = 60_000_000;

#[derive(Component)]
struct ConsumerPort {
    input: Entity,
}

#[derive(Component, Default)]
struct ConsumerState {
    previous: bool,
    current: bool,
    rises: Vec<u64>,
}

#[derive(Resource, Default, Clone, Debug, PartialEq, Eq)]
struct GateTrace {
    samples: BTreeMap<String, Vec<bool>>,
    rises: BTreeMap<String, Vec<u64>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ClockRun {
    trace: GateTrace,
    sample_time: SampleTime,
}

struct GateConsumer;

impl OxurackModule for GateConsumer {
    const KIND: &'static str = "clock_test_consumer";
    const DISPLAY_NAME: &'static str = "Clock Test Consumer";

    fn port_schema() -> &'static [PortSchema] {
        &[PortSchema {
            name: "pulse_in",
            direction: PortDirection::Input,
            value_kind: ValueKind::Gate,
            merge_policy: MergePolicy::Reject,
            description: "Clock pulse input",
        }]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        &[]
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let entity = spawn_module_entity(world, Self::KIND, instance_name);
        let input = spawn_port_on_module(
            world,
            entity,
            "pulse_in",
            PortDirection::Input,
            ValueKind::Gate,
            MergePolicy::Reject,
        );
        world
            .entity_mut(entity)
            .insert((ConsumerPort { input }, ConsumerState::default()));
        Ok(entity)
    }

    fn register_systems(
        app: &mut bevy_app::App,
        schedule: bevy_ecs::schedule::InternedScheduleLabel,
    ) {
        use bevy_ecs::schedule::IntoScheduleConfigs;

        app.add_systems(schedule, consume_gate.in_set(TickPhase::Consume));
        app.add_systems(schedule, observe_gate.in_set(TickPhase::Observe));
    }
}

fn consume_gate(
    sample_time: Res<SampleTime>,
    consumers: Query<(&ConsumerPort, &mut ConsumerState)>,
    values: Query<&CurrentValue>,
) {
    let index = sample_time
        .last_index()
        .expect("consumer runs only inside a sample");
    for (ports, mut state) in consumers {
        let current = match values.get(ports.input).expect("consumer input exists").0 {
            Value::Gate(value) => value,
            value => panic!("expected Gate input, got {value:?}"),
        };
        if current && !state.previous {
            state.rises.push(index);
        }
        state.previous = current;
        state.current = current;
    }
}

fn observe_gate(
    consumers: Query<(&Module, &ConsumerState)>,
    trace: Option<ResMut<GateTrace>>,
    mut commands: Commands,
) {
    if let Some(mut trace) = trace {
        for (module, state) in consumers {
            trace
                .samples
                .entry(module.instance_name.clone())
                .or_default()
                .push(state.current);
            trace
                .rises
                .insert(module.instance_name.clone(), state.rises.clone());
        }
    } else {
        let mut trace = GateTrace::default();
        for (module, state) in consumers {
            trace
                .samples
                .entry(module.instance_name.clone())
                .or_default()
                .push(state.current);
            trace
                .rises
                .insert(module.instance_name.clone(), state.rises.clone());
        }
        commands.insert_resource(trace);
    }
}

fn installed_fixture() -> Oxurack {
    let mut rack = Oxurack::new();
    rack.install::<ClockModule>();
    rack.install::<ClockModule>();
    rack.install::<GateConsumer>();
    rack.install::<GateConsumer>();
    rack.load_patch(Path::new(FIXTURE))
        .expect("clock fixture loads");
    rack
}

fn run_manual(groups: &[u64]) -> ClockRun {
    let mut rack = installed_fixture();
    rack.set_driver_mode(SimulationDriver::Manual).start();
    for &group in groups {
        rack.advance_samples(group)
            .expect("manual clock samples should run");
    }
    let sample_time = rack.sample_time();
    let app = rack.into_app();
    ClockRun {
        trace: app.world().resource::<GateTrace>().clone(),
        sample_time,
    }
}

fn run_manual_path(path: &Path, groups: &[u64]) -> ClockRun {
    let mut rack = Oxurack::new();
    rack.install::<ClockModule>();
    rack.install::<ClockModule>();
    rack.install::<GateConsumer>();
    rack.install::<GateConsumer>();
    rack.load_patch(path).expect("clock fixture loads");
    rack.set_driver_mode(SimulationDriver::Manual).start();
    for &group in groups {
        rack.advance_samples(group)
            .expect("manual clock samples should run");
    }
    let sample_time = rack.sample_time();
    let app = rack.into_app();
    ClockRun {
        trace: app.world().resource::<GateTrace>().clone(),
        sample_time,
    }
}

fn run_live(groups: &[u32]) -> ClockRun {
    let mut rack = installed_fixture();
    rack.start();
    let mut app = rack.into_app();
    let maximum_group = groups.iter().copied().max().unwrap_or(1);
    app.world_mut()
        .resource_mut::<Time<Virtual>>()
        .set_max_delta(Duration::from_millis(u64::from(maximum_group)));
    for &group in groups {
        app.insert_resource(TimeUpdateStrategy::FixedTimesteps(group));
        app.update();
    }
    ClockRun {
        trace: app.world().resource::<GateTrace>().clone(),
        sample_time: *app.world().resource::<SampleTime>(),
    }
}

fn expected_samples(rate_milli_bpm: u64, pulse_width_ms: usize, total: usize) -> Vec<bool> {
    let rate = u128::from(rate_milli_bpm);
    let mut samples = vec![false; total];
    let maximum_pulse = (total as u128 * rate / MILLIS_PER_MINUTE) + 1;
    for pulse in 0..=maximum_pulse {
        let rising = (pulse * MILLIS_PER_MINUTE).div_ceil(rate);
        if rising >= total as u128 {
            break;
        }
        let start = rising as usize;
        let end = (start + pulse_width_ms).min(total);
        samples[start..end].fill(true);
    }
    samples
}

fn expected_trace(total: usize) -> GateTrace {
    let clocks = [
        ("consumer_slow", 120_000, 3),
        ("consumer_fast", 240_000, 2),
        ("consumer_default", 120_000, 10),
        ("consumer_patch_bpm", 90_000, 10),
        ("consumer_unrelated", 0, 0),
    ];
    let mut trace = GateTrace::default();
    for (name, rate, width) in clocks {
        let samples = if rate == 0 {
            vec![false; total]
        } else {
            expected_samples(rate, width, total)
        };
        let mut previous = false;
        let rises = samples
            .iter()
            .enumerate()
            .filter_map(|(index, &current)| {
                let rising = current && !previous;
                previous = current;
                rising.then_some(index as u64)
            })
            .collect();
        trace.samples.insert(name.to_owned(), samples);
        trace.rises.insert(name.to_owned(), rises);
    }
    trace
}

fn assert_complete_run(run: &ClockRun, total: usize) {
    assert_eq!(run.trace, expected_trace(total));
    assert_eq!(run.sample_time.completed_samples(), total as u64);
    assert_eq!(run.sample_time.last_index(), Some(total as u64 - 1));
}

fn write_reversed_fixture() -> PathBuf {
    let mut patch = oxurack_core::load_patch_from_file(Path::new(FIXTURE)).unwrap();
    patch.modules.reverse();
    let path =
        std::env::temp_dir().join(format!("oxurack-clock-reversed-{}.ron", std::process::id()));
    save_patch_to_file(&patch, &path).unwrap();
    path
}

#[test]
fn installed_clocks_drive_the_complete_expected_trace() {
    let run = run_manual(&[1_105]);
    assert_complete_run(&run, 1_105);
}

#[test]
fn manual_and_live_groupings_match_full_trace_and_sample_time() {
    let manual_single = run_manual(&[1_105]);
    let manual_grouped = run_manual(&[1, 2, 7, 95, 1_000]);
    let live_single = run_live(&[1; 1_105]);
    let live_grouped = run_live(&[1, 2, 7, 95, 1_000]);

    assert_complete_run(&manual_single, 1_105);
    assert_eq!(manual_grouped, manual_single);
    assert_eq!(live_single, manual_single);
    assert_eq!(live_grouped, manual_single);
}

#[test]
fn pause_resume_stop_while_high_and_stopped_requests_match_reference() {
    let reference = run_manual(&[1_105]);
    let mut rack = installed_fixture();
    rack.set_driver_mode(SimulationDriver::Manual).start();
    rack.advance_samples(2).unwrap();
    rack.stop();
    assert_eq!(rack.advance_samples(0).unwrap(), 0);
    assert_eq!(rack.advance_samples(10).unwrap(), 0);
    rack.start();
    rack.advance_samples(1_103).unwrap();
    let subject = {
        let sample_time = rack.sample_time();
        let app = rack.into_app();
        ClockRun {
            trace: app.world().resource::<GateTrace>().clone(),
            sample_time,
        }
    };

    assert_eq!(subject, reference);
}

#[test]
fn reinstall_after_progress_does_not_duplicate_or_reset_behavior() {
    let reference = run_manual(&[1_105]);
    let mut rack = installed_fixture();
    rack.set_driver_mode(SimulationDriver::Manual).start();
    rack.advance_samples(3).unwrap();
    rack.install::<ClockModule>();
    rack.install::<ClockModule>();
    rack.install::<GateConsumer>();
    rack.install::<GateConsumer>();
    rack.advance_samples(1_102).unwrap();
    let subject = {
        let sample_time = rack.sample_time();
        let app = rack.into_app();
        ClockRun {
            trace: app.world().resource::<GateTrace>().clone(),
            sample_time,
        }
    };

    assert_eq!(subject, reference);
}

#[test]
fn reversed_fixture_has_the_same_full_trace() {
    let path = write_reversed_fixture();
    let normal = run_manual(&[1_105]);
    let reversed = run_manual_path(&path, &[1, 2, 7, 95, 1_000]);
    let _ = std::fs::remove_file(path);

    assert_eq!(reversed, normal);
}
