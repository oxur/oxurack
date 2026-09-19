use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bevy_ecs::prelude::{Commands, Component, Entity, Query, Res, ResMut, Resource, World};
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

#[derive(Resource, Default)]
struct GateTrace {
    samples: BTreeMap<String, Vec<bool>>,
    rises: BTreeMap<String, Vec<u64>>,
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

fn rack_with_fixture() -> Oxurack {
    let mut rack = Oxurack::new();
    rack.install::<ClockModule>()
        .install::<GateConsumer>()
        .set_driver_mode(SimulationDriver::Manual);
    rack.load_patch(Path::new(FIXTURE))
        .expect("clock fixture loads");
    rack.start();
    rack
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
fn installed_clocks_drive_connected_consumers_with_named_edges() {
    let mut rack = rack_with_fixture();
    assert_eq!(rack.advance_samples(1_105).unwrap(), 1_105);
    let app = rack.into_app();
    let trace = app.world().resource::<GateTrace>();

    assert_eq!(trace.samples["consumer_slow"].len(), 1_105);
    assert_eq!(trace.samples["consumer_fast"].len(), 1_105);
    assert_eq!(trace.rises["consumer_slow"], vec![0, 500, 1_000]);
    assert_eq!(trace.rises["consumer_fast"], vec![0, 250, 500, 750, 1_000]);
    assert_eq!(trace.rises["consumer_default"], vec![0, 500, 1_000]);
    assert_eq!(trace.rises["consumer_patch_bpm"], vec![0, 667]);
    assert!(
        trace.samples["consumer_slow"][3..500]
            .iter()
            .all(|value| !*value)
    );
    assert!(
        trace.samples["consumer_fast"][2..250]
            .iter()
            .all(|value| !*value)
    );
}

#[test]
fn grouping_reinstall_stop_resume_and_spawn_permutation_preserve_behavior() {
    let mut rack = rack_with_fixture();
    rack.advance_samples(1).unwrap();
    rack.install::<ClockModule>();
    rack.install::<GateConsumer>();
    rack.advance_samples(2).unwrap();
    rack.stop();
    assert_eq!(rack.advance_samples(0).unwrap(), 0);
    assert_eq!(rack.advance_samples(10).unwrap(), 0);
    rack.start();
    rack.advance_samples(247).unwrap();
    let app = rack.into_app();
    let trace = app.world().resource::<GateTrace>();
    assert_eq!(trace.rises["consumer_fast"], vec![0]);

    let path = write_reversed_fixture();
    let mut reversed = Oxurack::new();
    reversed
        .install::<ClockModule>()
        .install::<GateConsumer>()
        .set_driver_mode(SimulationDriver::Manual);
    reversed.load_patch(&path).unwrap();
    reversed.start();
    reversed.advance_samples(501).unwrap();
    let reversed_app = reversed.into_app();
    let trace = reversed_app.world().resource::<GateTrace>();
    assert_eq!(trace.rises["consumer_slow"], vec![0, 500]);
    assert_eq!(trace.rises["consumer_fast"], vec![0, 250, 500]);
    assert!(trace.rises["consumer_unrelated"].is_empty());
    let _ = std::fs::remove_file(path);
}
