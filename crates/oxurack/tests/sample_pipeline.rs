use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::{Component, Entity, Query, Res, ResMut, Resource, World};
use bevy_ecs::schedule::{InternedScheduleLabel, IntoScheduleConfigs};
use bevy_time::{Time, TimeUpdateStrategy};
use oxurack::{
    CoreError, MergePolicy, Oxurack, ParameterValue, PortDirection, PortSchema, SampleTime,
    SimulationDriver, TickPhase, Value, ValueKind,
};
use oxurack_core::{CurrentValue, OxurackModule, spawn_module_entity, spawn_port_on_module};

const PRODUCER_KIND: &str = "sample_producer";
const RELAY_KIND: &str = "sample_relay";
const SINK_KIND: &str = "sample_sink";

#[derive(Component)]
struct ProducerPorts {
    out: Entity,
}

#[derive(Component)]
struct RelayPorts {
    input: Entity,
    out: Entity,
}

#[derive(Component)]
struct SinkPorts {
    input: Entity,
    hold: Entity,
    out: Entity,
}

#[derive(Component, Default)]
struct RelayState {
    output: f32,
}

#[derive(Component, Default)]
struct SinkState {
    output: f32,
}

#[derive(Resource, Default, Debug, PartialEq, Eq)]
struct PhaseTrace(Vec<&'static str>);

#[derive(Resource, Default, Debug, PartialEq, Eq)]
struct PhaseClockTrace(Vec<(u64, u64)>);

#[derive(Clone, Debug, PartialEq)]
struct ObservedSample {
    index: u64,
    generic_delta: std::time::Duration,
    producer_out: f32,
    relay_in: f32,
    relay_out: f32,
    sink_in: f32,
    sink_out: f32,
    disconnected_hold: f32,
}

type SampleRun = (
    Vec<ObservedSample>,
    Vec<&'static str>,
    Vec<(u64, u64)>,
    SampleTime,
);

#[derive(Resource, Default)]
struct ObservedSamples(Vec<ObservedSample>);

fn float_value(values: &Query<&CurrentValue>, entity: Entity) -> f32 {
    match values.get(entity).expect("fixture port should exist").0 {
        Value::Float(value) => value,
        value => panic!("expected float fixture value, got {value:?}"),
    }
}

fn record_phase(
    name: &'static str,
    sample_time: &SampleTime,
    trace: &mut PhaseTrace,
    clocks: &mut PhaseClockTrace,
) {
    trace.0.push(name);
    clocks.0.push((
        sample_time
            .last_index()
            .expect("running sample has an index"),
        sample_time.completed_samples(),
    ));
}

fn produce_marker(
    sample_time: Res<SampleTime>,
    mut trace: ResMut<PhaseTrace>,
    mut clocks: ResMut<PhaseClockTrace>,
) {
    record_phase("Produce", &sample_time, &mut trace, &mut clocks);
}

fn propagate_marker(
    sample_time: Res<SampleTime>,
    mut trace: ResMut<PhaseTrace>,
    mut clocks: ResMut<PhaseClockTrace>,
) {
    record_phase("Propagate", &sample_time, &mut trace, &mut clocks);
}

fn merge_marker(
    sample_time: Res<SampleTime>,
    mut trace: ResMut<PhaseTrace>,
    mut clocks: ResMut<PhaseClockTrace>,
) {
    record_phase("MergeInputs", &sample_time, &mut trace, &mut clocks);
}

fn consume_marker(
    sample_time: Res<SampleTime>,
    mut trace: ResMut<PhaseTrace>,
    mut clocks: ResMut<PhaseClockTrace>,
) {
    record_phase("Consume", &sample_time, &mut trace, &mut clocks);
}

fn observe_marker(
    sample_time: Res<SampleTime>,
    mut trace: ResMut<PhaseTrace>,
    mut clocks: ResMut<PhaseClockTrace>,
) {
    record_phase("Observe", &sample_time, &mut trace, &mut clocks);
}

fn producer_system(
    sample_time: Res<SampleTime>,
    producers: Query<&ProducerPorts>,
    mut values: Query<&mut CurrentValue>,
) {
    let Some(index) = sample_time.last_index() else {
        return;
    };
    for ports in &producers {
        values
            .get_mut(ports.out)
            .expect("producer output should exist")
            .0 = Value::Float(index as f32 + 1.0);
    }
}

fn relay_produce(relays: Query<(&RelayPorts, &RelayState)>, mut values: Query<&mut CurrentValue>) {
    for (ports, state) in &relays {
        values
            .get_mut(ports.out)
            .expect("relay output should exist")
            .0 = Value::Float(state.output);
    }
}

fn sink_produce(sinks: Query<(&SinkPorts, &SinkState)>, mut values: Query<&mut CurrentValue>) {
    for (ports, state) in &sinks {
        values
            .get_mut(ports.out)
            .expect("sink output should exist")
            .0 = Value::Float(state.output);
    }
}

fn relay_consume(relays: Query<(&RelayPorts, &mut RelayState)>, values: Query<&CurrentValue>) {
    for (ports, mut state) in relays {
        state.output = float_value(&values, ports.input);
    }
}

fn sink_consume(sinks: Query<(&SinkPorts, &mut SinkState)>, values: Query<&CurrentValue>) {
    for (ports, mut state) in sinks {
        state.output = float_value(&values, ports.input);
    }
}

fn observe_system(
    sample_time: Res<SampleTime>,
    producers: Query<&ProducerPorts>,
    relays: Query<&RelayPorts>,
    sinks: Query<&SinkPorts>,
    values: Query<&CurrentValue>,
    generic_time: Res<Time>,
    mut observed: ResMut<ObservedSamples>,
) {
    let producer = producers.iter().next().expect("one producer expected");
    let relay = relays.iter().next().expect("one relay expected");
    let sink = sinks.iter().next().expect("one sink expected");
    observed.0.push(ObservedSample {
        index: sample_time
            .last_index()
            .expect("running sample has an index"),
        generic_delta: generic_time.delta(),
        producer_out: float_value(&values, producer.out),
        relay_in: float_value(&values, relay.input),
        relay_out: float_value(&values, relay.out),
        sink_in: float_value(&values, sink.input),
        sink_out: float_value(&values, sink.out),
        disconnected_hold: float_value(&values, sink.hold),
    });
}

struct SampleProducer;

impl OxurackModule for SampleProducer {
    const KIND: &'static str = PRODUCER_KIND;
    const DISPLAY_NAME: &'static str = "Sample Producer";

    fn port_schema() -> &'static [PortSchema] {
        &[PortSchema {
            name: "out",
            direction: PortDirection::Output,
            value_kind: ValueKind::Float,
            merge_policy: MergePolicy::Reject,
            description: "Sample-indexed output",
        }]
    }

    fn parameter_schema() -> &'static [oxurack::ParameterSchema] {
        &[]
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let entity = spawn_module_entity(world, Self::KIND, instance_name);
        let out = spawn_port_on_module(
            world,
            entity,
            "out",
            PortDirection::Output,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        world.entity_mut(entity).insert(ProducerPorts { out });
        Ok(entity)
    }

    fn register_systems(app: &mut App, schedule: InternedScheduleLabel) {
        app.init_resource::<PhaseTrace>()
            .init_resource::<PhaseClockTrace>()
            .init_resource::<ObservedSamples>()
            .add_systems(
                schedule,
                (
                    produce_marker.in_set(TickPhase::Produce),
                    producer_system.in_set(TickPhase::Produce),
                    propagate_marker.in_set(TickPhase::Propagate),
                    merge_marker.in_set(TickPhase::MergeInputs),
                    consume_marker.in_set(TickPhase::Consume),
                    observe_marker.in_set(TickPhase::Observe),
                    observe_system
                        .in_set(TickPhase::Observe)
                        .after(observe_marker),
                    relay_produce.in_set(TickPhase::Produce),
                    sink_produce.in_set(TickPhase::Produce),
                    relay_consume.in_set(TickPhase::Consume),
                    sink_consume.in_set(TickPhase::Consume),
                ),
            );
    }
}

struct SampleRelay;

impl OxurackModule for SampleRelay {
    const KIND: &'static str = RELAY_KIND;
    const DISPLAY_NAME: &'static str = "Sample Relay";

    fn port_schema() -> &'static [PortSchema] {
        &[
            PortSchema {
                name: "in",
                direction: PortDirection::Input,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::LastWins,
                description: "Merged current-sample input",
            },
            PortSchema {
                name: "out",
                direction: PortDirection::Output,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::Reject,
                description: "One-sample delayed output",
            },
        ]
    }

    fn parameter_schema() -> &'static [oxurack::ParameterSchema] {
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
            "in",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::LastWins,
        );
        let out = spawn_port_on_module(
            world,
            entity,
            "out",
            PortDirection::Output,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        world
            .entity_mut(entity)
            .insert((RelayPorts { input, out }, RelayState::default()));
        Ok(entity)
    }
}

struct SampleSink;

impl OxurackModule for SampleSink {
    const KIND: &'static str = SINK_KIND;
    const DISPLAY_NAME: &'static str = "Sample Sink";

    fn port_schema() -> &'static [PortSchema] {
        &[
            PortSchema {
                name: "in",
                direction: PortDirection::Input,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::LastWins,
                description: "Merged relay input",
            },
            PortSchema {
                name: "hold",
                direction: PortDirection::Input,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::LastWins,
                description: "Disconnected sample-and-hold input",
            },
            PortSchema {
                name: "out",
                direction: PortDirection::Output,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::Reject,
                description: "One-sample delayed output",
            },
        ]
    }

    fn parameter_schema() -> &'static [oxurack::ParameterSchema] {
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
            "in",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::LastWins,
        );
        let hold = spawn_port_on_module(
            world,
            entity,
            "hold",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::LastWins,
        );
        let out = spawn_port_on_module(
            world,
            entity,
            "out",
            PortDirection::Output,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        world
            .entity_mut(hold)
            .get_mut::<CurrentValue>()
            .expect("hold input should have a current value")
            .0 = Value::Float(0.25);
        world
            .entity_mut(entity)
            .insert((SinkPorts { input, hold, out }, SinkState::default()));
        Ok(entity)
    }
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sample-pipeline.ron")
}

fn installed_fixture() -> Oxurack {
    let mut rack = Oxurack::new();
    rack.install::<SampleProducer>()
        .install::<SampleRelay>()
        .install::<SampleSink>();
    rack.load_patch(&fixture_path())
        .expect("sample pipeline fixture should load through the public path");
    rack
}

fn expected_samples() -> Vec<ObservedSample> {
    vec![
        ObservedSample {
            index: 0,
            generic_delta: std::time::Duration::from_millis(1),
            producer_out: 1.0,
            relay_in: 1.0,
            relay_out: 0.0,
            sink_in: 0.0,
            sink_out: 0.0,
            disconnected_hold: 0.25,
        },
        ObservedSample {
            index: 1,
            generic_delta: std::time::Duration::from_millis(1),
            producer_out: 2.0,
            relay_in: 2.0,
            relay_out: 1.0,
            sink_in: 1.0,
            sink_out: 0.0,
            disconnected_hold: 0.25,
        },
        ObservedSample {
            index: 2,
            generic_delta: std::time::Duration::from_millis(1),
            producer_out: 3.0,
            relay_in: 3.0,
            relay_out: 2.0,
            sink_in: 2.0,
            sink_out: 1.0,
            disconnected_hold: 0.25,
        },
    ]
}

fn observed(app: &App) -> Vec<ObservedSample> {
    app.world().resource::<ObservedSamples>().0.clone()
}

fn phase_trace(app: &App) -> Vec<&'static str> {
    app.world().resource::<PhaseTrace>().0.clone()
}

fn phase_clock_trace(app: &App) -> Vec<(u64, u64)> {
    app.world().resource::<PhaseClockTrace>().0.clone()
}

fn run_manual() -> SampleRun {
    let mut rack = installed_fixture();
    assert_eq!(rack.sample_time(), SampleTime::default());
    rack.set_driver_mode(SimulationDriver::Manual).start();
    assert_eq!(
        rack.advance_samples(3).expect("manual samples should run"),
        3
    );
    let sample_time = rack.sample_time();
    let app = rack.into_app();
    (
        observed(&app),
        phase_trace(&app),
        phase_clock_trace(&app),
        sample_time,
    )
}

fn run_live(groups: &[u32]) -> (Vec<ObservedSample>, SampleTime) {
    let mut rack = installed_fixture();
    rack.start();
    let mut app = rack.into_app();
    app.insert_resource(TimeUpdateStrategy::FixedTimesteps(1));
    for &group in groups {
        *app.world_mut().resource_mut::<TimeUpdateStrategy>() =
            TimeUpdateStrategy::FixedTimesteps(group);
        app.update();
    }
    let sample_time = *app.world().resource::<SampleTime>();
    (observed(&app), sample_time)
}

#[test]
fn manual_pipeline_has_indexed_latency_and_phase_order() {
    let (samples, phases, clocks, sample_time) = run_manual();
    assert_eq!(samples, expected_samples());
    assert_eq!(sample_time.last_index(), Some(2));
    assert_eq!(sample_time.completed_samples(), 3);
    assert_eq!(
        phases,
        vec![
            "Produce",
            "Propagate",
            "MergeInputs",
            "Consume",
            "Observe",
            "Produce",
            "Propagate",
            "MergeInputs",
            "Consume",
            "Observe",
            "Produce",
            "Propagate",
            "MergeInputs",
            "Consume",
            "Observe",
        ]
    );
    assert_eq!(
        clocks,
        vec![
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (2, 2),
            (2, 2),
            (2, 2),
            (2, 2),
            (2, 2),
        ]
    );
}

#[test]
fn manual_and_controlled_live_drives_match_grouping() {
    let (manual, _, _, manual_time) = run_manual();
    let (live_single, live_single_time) = run_live(&[1, 1, 1]);
    let (live_grouped, live_grouped_time) = run_live(&[1, 2]);
    assert_eq!(live_single, manual);
    assert_eq!(live_grouped, manual);
    assert_eq!(live_single_time, manual_time);
    assert_eq!(live_grouped_time, manual_time);
}

#[test]
fn stopped_manual_and_live_requests_do_not_change_sample_state() {
    let mut rack = installed_fixture();
    assert_eq!(rack.driver_mode(), SimulationDriver::Live);
    let before = rack.sample_time();
    assert!(matches!(
        rack.advance_samples(1),
        Err(CoreError::ManualSampleRequiresManualDriver)
    ));
    assert_eq!(rack.driver_mode(), SimulationDriver::Live);
    assert_eq!(rack.sample_time(), before);

    rack.set_driver_mode(SimulationDriver::Manual);
    assert_eq!(rack.advance_samples(0).expect("zero is a no-op"), 0);
    assert_eq!(rack.advance_samples(2).expect("stopped is a no-op"), 0);
    assert_eq!(rack.sample_time(), before);
    rack.step();
    assert_eq!(rack.sample_time(), before);

    rack.start();
    assert_eq!(rack.advance_samples(1).expect("started manual rack"), 1);
    let after_one = rack.sample_time();
    rack.stop();
    assert_eq!(rack.advance_samples(2).expect("stopped manual rack"), 0);
    assert_eq!(rack.sample_time(), after_one);
    rack.start();
    assert_eq!(rack.advance_samples(1).expect("restarted manual rack"), 1);
    assert_eq!(rack.sample_time().last_index(), Some(1));
}

#[test]
fn last_wins_is_name_stable_with_spawn_permutation_and_unrelated_entity() {
    fn run(order: &[&str]) -> Value {
        let mut app = App::new();
        app.add_plugins(oxurack::CorePlugin::fixed());
        let world = app.world_mut();
        let mut modules = HashMap::new();
        for name in order {
            let module = spawn_module_entity(world, "source", name);
            let output = spawn_port_on_module(
                world,
                module,
                "out",
                PortDirection::Output,
                ValueKind::Float,
                MergePolicy::Reject,
            );
            world
                .entity_mut(output)
                .get_mut::<CurrentValue>()
                .expect("source output should exist")
                .0 = Value::Float(if *name == "a" { 1.0 } else { 2.0 });
            modules.insert(*name, output);
        }
        let unrelated = world.spawn_empty().id();
        let sink = spawn_module_entity(world, "sink", "sink");
        let input = spawn_port_on_module(
            world,
            sink,
            "in",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::LastWins,
        );
        for source in ["a", "b"] {
            world.spawn(oxurack_core::Cable {
                source_port: modules[source],
                target_port: input,
                transform: None,
                enabled: true,
            });
        }
        assert!(world.get_entity(unrelated).is_ok());
        world.run_schedule(FixedUpdate);
        world.entity(input).get::<CurrentValue>().unwrap().0
    }

    assert_eq!(run(&["a", "b"]), Value::Float(2.0));
    assert_eq!(run(&["b", "a"]), Value::Float(2.0));
}
