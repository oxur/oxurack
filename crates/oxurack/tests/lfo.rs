use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use bevy_app::{App, FixedUpdate, Update};
use bevy_ecs::prelude::{Entity, Query, ResMut, Resource, World};
use bevy_ecs::schedule::{InternedScheduleLabel, IntoScheduleConfigs, ScheduleLabel};
use oxurack::{
    CableConfig, CoreError, CorePlugin, Module, ModuleConfig, Oxurack, ParameterValue, Patch,
    SampleTime, SimulationDriver, SimulationRunning, TickPhase, Value,
};
use oxurack_core::{
    CurrentValue, Module as ModuleComponent, ModuleKind, OxurackModule, ParameterRegistry, Port,
    deserialize_patch, save_patch_to_file,
};
use oxurack_mod_lfo::LfoModule;

#[path = "../examples/turing_machine_midi/oracle.rs"]
mod oracle;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/patches/turing-machine-lfo.ron"
);
const DEFAULT_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/patches/turing-machine-midi.ron"
);
static TEMP_PATCH_COUNTER: AtomicU64 = AtomicU64::new(0);

fn read_patch(path: &str) -> Patch {
    deserialize_patch(&std::fs::read_to_string(path).unwrap()).unwrap()
}

fn save_temp(patch: &Patch) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "oxurack-lfo-{}-{}.ron",
        std::process::id(),
        TEMP_PATCH_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    save_patch_to_file(patch, &path).unwrap();
    path
}

fn accelerate(patch: &mut Patch) {
    patch
        .modules
        .iter_mut()
        .for_each(|module| match module.instance_name.as_str() {
            "clock_main" => {
                module
                    .parameters
                    .insert("bpm".into(), ParameterValue::Float(1000.0));
                module
                    .parameters
                    .insert("ppqn".into(), ParameterValue::Int(30));
                module
                    .parameters
                    .insert("pulse_width_ms".into(), ParameterValue::Int(1));
            }
            "lfo_main" => {
                module
                    .parameters
                    .insert("waveform".into(), ParameterValue::String("square".into()));
                module
                    .parameters
                    .insert("frequency_hz".into(), ParameterValue::Float(62.5));
                module
                    .parameters
                    .insert("phase_offset".into(), ParameterValue::Float(0.0));
                module
                    .parameters
                    .insert("pulse_width".into(), ParameterValue::Float(0.5));
            }
            _ => {}
        });
}

fn install_rack(patch: &Patch) -> (Oxurack, std::path::PathBuf) {
    let path = save_temp(patch);
    let mut rack = Oxurack::new();
    rack.try_install::<oxurack_mod_clock::ClockModule>()
        .unwrap();
    rack.try_install::<LfoModule>().unwrap();
    rack.try_install::<oxurack_mod_turingmachine::TuringMachineModule>()
        .unwrap();
    rack.try_install::<oxurack::MidiOutModule>().unwrap();
    rack.load_patch(&path).unwrap();
    rack.set_driver_mode(SimulationDriver::Manual).start();
    (rack, path)
}

fn trace(rack: &mut Oxurack, samples: u64) -> Vec<(u64, [u8; 3])> {
    let mut actual = Vec::new();
    for _ in 0..samples {
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        actual.extend(
            rack.drain_midi()
                .map(|event| (event.sample, event.message.to_bytes())),
        );
    }
    actual
}

#[test]
fn loaded_accelerated_patch_matches_independent_probability_and_midi_oracles() {
    let mut patch = read_patch(FIXTURE);
    accelerate(&mut patch);
    let (mut subject, path) = install_rack(&patch);
    let actual = trace(&mut subject, 24);
    let expected = oracle::expected_modulated_trace(24);
    assert_eq!(actual, expected);
    assert!(!actual.is_empty());

    let mut control_patch = patch.clone();
    control_patch
        .cables
        .retain(|cable| cable.source != ("lfo_main".to_owned(), "bipolar_out".to_owned()));
    let (mut control, control_path) = install_rack(&control_patch);
    let unmodulated = trace(&mut control, 24);
    assert_ne!(
        actual, unmodulated,
        "control proves the modulation cable matters"
    );
    assert_ne!(unmodulated, oracle::expected_trace(24));
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(control_path);
}

#[test]
fn unchanged_lfo_fixture_has_repeatable_ten_thousand_sample_trace() {
    let patch = read_patch(FIXTURE);
    let (mut first, first_path) = install_rack(&patch);
    let first_trace = trace(&mut first, 10_000);
    let (mut second, second_path) = install_rack(&patch);
    let second_trace = trace(&mut second, 10_000);
    assert!(!first_trace.is_empty());
    assert_eq!(first_trace, second_trace);
    let _ = std::fs::remove_file(first_path);
    let _ = std::fs::remove_file(second_path);
}

#[derive(Resource, Default, Debug)]
struct LfoObservation(Vec<Vec<f32>>);

struct LfoObserver;

impl OxurackModule for LfoObserver {
    const KIND: &'static str = "lfo_test_observer";
    const DISPLAY_NAME: &'static str = "LFO Test Observer";

    fn port_schema() -> &'static [oxurack::PortSchema] {
        &[]
    }

    fn parameter_schema() -> &'static [oxurack::ParameterSchema] {
        &[]
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        Ok(oxurack_core::spawn_module_entity(
            world,
            Self::KIND,
            instance_name,
        ))
    }

    fn register_systems(app: &mut App, schedule: bevy_ecs::schedule::InternedScheduleLabel) {
        app.init_resource::<LfoObservation>();
        app.add_systems(schedule, observe_lfos.in_set(TickPhase::Observe));
    }
}

fn observe_lfos(ports: Query<(&Port, &CurrentValue)>, mut observations: ResMut<LfoObservation>) {
    let mut values: Vec<_> = ports
        .iter()
        .filter_map(|(port, current)| {
            (port.name.as_ref() == "bipolar_out").then_some(match current.0 {
                Value::Bipolar(value) => value,
                _ => f32::NAN,
            })
        })
        .collect();
    values.sort_by(f32::total_cmp);
    observations.0.push(values);
}

fn add_lfo_config(patch: &mut Patch, name: &str, offset: f32) {
    let mut parameters = HashMap::new();
    parameters.insert("waveform".into(), ParameterValue::String("saw_up".into()));
    parameters.insert("frequency_hz".into(), ParameterValue::Float(62.5));
    parameters.insert("phase_offset".into(), ParameterValue::Float(offset));
    parameters.insert("pulse_width".into(), ParameterValue::Float(0.5));
    patch.modules.push(ModuleConfig {
        kind: "lfo".into(),
        instance_name: name.into(),
        parameters,
        ..ModuleConfig::default()
    });
}

#[test]
fn clock_sync_offsets_stop_resume_and_idempotent_reinstall_are_observed() {
    let mut patch = read_patch(FIXTURE);
    patch
        .modules
        .retain(|module| module.instance_name != "midi_main" && module.instance_name != "tm_main");
    patch.cables.clear();
    patch
        .modules
        .iter_mut()
        .find(|module| module.instance_name == "lfo_main")
        .unwrap()
        .parameters
        .insert("waveform".into(), ParameterValue::String("saw_up".into()));
    patch
        .modules
        .iter_mut()
        .find(|module| module.instance_name == "lfo_main")
        .unwrap()
        .parameters
        .insert("frequency_hz".into(), ParameterValue::Float(62.5));
    add_lfo_config(&mut patch, "lfo_offset", 0.25);
    patch.cables.push(CableConfig {
        source: ("clock_main".into(), "pulse".into()),
        target: ("lfo_main".into(), "sync_in".into()),
        ..CableConfig::default()
    });
    patch.cables.push(CableConfig {
        source: ("clock_main".into(), "pulse".into()),
        target: ("lfo_offset".into(), "sync_in".into()),
        ..CableConfig::default()
    });
    accelerate(&mut patch);
    patch
        .modules
        .iter_mut()
        .find(|module| module.instance_name == "lfo_main")
        .unwrap()
        .parameters
        .insert("waveform".into(), ParameterValue::String("saw_up".into()));

    let (mut rack, path) = install_rack(&patch);
    rack.try_install::<LfoObserver>().unwrap();
    rack.advance_samples(6).unwrap();
    rack.stop();
    assert_eq!(rack.advance_samples(1).unwrap(), 0);
    rack.try_install::<LfoModule>().unwrap();
    rack.start();
    rack.advance_samples(2).unwrap();
    let app = rack.into_app();
    let observed = &app.world().resource::<LfoObservation>().0;
    assert_eq!(observed.len(), 8);
    let actual = observed.to_vec();
    let expected = [
        vec![-1.0, -0.5],
        vec![-1.0, -0.5],
        vec![-0.875, -0.375],
        vec![-1.0, -0.5],
        vec![-0.875, -0.375],
        vec![-1.0, -0.5],
        vec![-0.875, -0.375],
        vec![-1.0, -0.5],
    ];
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.len(), 2);
        assert!(
            actual
                .iter()
                .zip(expected.iter())
                .all(|(a, e)| (a - e).abs() < 1.0e-6),
            "actual={actual:?}, expected={expected:?}"
        );
    }
    let _ = std::fs::remove_file(path);
}

fn module_entity(world: &mut World, name: &str) -> Entity {
    let mut query = world.query::<(Entity, &ModuleComponent)>();
    query
        .iter(world)
        .find_map(|(entity, module)| (module.instance_name == name).then_some(entity))
        .unwrap()
}

fn parameter_port(world: &mut World, module: Entity, name: &str) -> Entity {
    let mut query = world.query::<(Entity, &Port, &bevy_ecs::hierarchy::ChildOf)>();
    query
        .iter(world)
        .find_map(|(entity, port, parent)| {
            (parent.parent() == module && port.name.as_ref() == name).then_some(entity)
        })
        .unwrap()
}

fn set_registered(
    app: &mut App,
    entity: Entity,
    name: &str,
    value: ParameterValue,
) -> Result<(), CoreError> {
    app.world_mut()
        .resource_scope::<ParameterRegistry, _>(|world, registry| {
            registry.set_parameter(world, entity, &ModuleKind::from("lfo"), name, value)
        })
}

fn get_registered(app: &App, entity: Entity, name: &str) -> ParameterValue {
    app.world()
        .resource::<ParameterRegistry>()
        .get_parameter(app.world(), entity, &ModuleKind::from("lfo"), name)
        .unwrap()
}

#[test]
fn setters_getters_edits_validation_and_failed_spawn_are_atomic() {
    let mut patch = read_patch(FIXTURE);
    patch
        .modules
        .retain(|module| module.instance_name == "lfo_main");
    patch.cables.clear();
    let (rack, path) = install_rack(&patch);
    let mut app = rack.into_app();
    let lfo = module_entity(app.world_mut(), "lfo_main");
    let out = parameter_port(app.world_mut(), lfo, "bipolar_out");

    for (name, value, expected) in [
        (
            "waveform",
            ParameterValue::String("saw_up".into()),
            ParameterValue::String("saw_up".into()),
        ),
        (
            "frequency_hz",
            ParameterValue::Float(62.5),
            ParameterValue::Float(62.5),
        ),
        (
            "phase_offset",
            ParameterValue::Float(0.25),
            ParameterValue::Float(0.25),
        ),
        (
            "pulse_width",
            ParameterValue::Float(0.25),
            ParameterValue::Float(0.25),
        ),
    ] {
        set_registered(&mut app, lfo, name, value).unwrap();
        assert_eq!(get_registered(&app, lfo, name), expected);
    }

    app.world_mut()
        .resource_mut::<oxurack::SimulationRunning>()
        .0 = true;
    for _ in 0..3 {
        app.world_mut().run_schedule(FixedUpdate);
    }
    let before_edit = match app.world().get::<CurrentValue>(out).unwrap().0 {
        Value::Bipolar(value) => value,
        value => panic!("expected bipolar output, got {value:?}"),
    };
    set_registered(&mut app, lfo, "phase_offset", ParameterValue::Float(0.0)).unwrap();
    let after_edit_expected = -0.625;
    assert!(
        (before_edit - (-0.25)).abs() < 1.0e-6,
        "before edit={before_edit}"
    );
    app.world_mut().run_schedule(FixedUpdate);
    let after_edit = match app.world().get::<CurrentValue>(out).unwrap().0 {
        Value::Bipolar(value) => value,
        value => panic!("expected bipolar output, got {value:?}"),
    };
    assert!((after_edit - after_edit_expected).abs() < 1.0e-6);

    for (name, bad) in [
        ("frequency_hz", ParameterValue::Int(2)),
        ("frequency_hz", ParameterValue::Float(f32::NAN)),
        ("frequency_hz", ParameterValue::Float(101.0)),
        ("phase_offset", ParameterValue::Float(1.0)),
        ("pulse_width", ParameterValue::Float(0.0)),
        ("waveform", ParameterValue::Float(1.0)),
        ("waveform", ParameterValue::String("triangle".into())),
    ] {
        assert!(set_registered(&mut app, lfo, name, bad).is_err());
    }
    assert_eq!(
        get_registered(&app, lfo, "frequency_hz"),
        ParameterValue::Float(62.5)
    );
    assert_eq!(
        get_registered(&app, lfo, "phase_offset"),
        ParameterValue::Float(0.0)
    );
    assert_eq!(
        get_registered(&app, lfo, "waveform"),
        ParameterValue::String("saw_up".into())
    );
    app.world_mut().run_schedule(FixedUpdate);
    let after_failed_edits = match app.world().get::<CurrentValue>(out).unwrap().0 {
        Value::Bipolar(value) => value,
        value => panic!("expected bipolar output, got {value:?}"),
    };
    assert!((after_failed_edits - (-0.5)).abs() < 1.0e-6);

    let mut bad_world = World::new();
    let before_entities = bad_world.entities().len();
    let mut invalid = HashMap::new();
    invalid.insert("z_unknown".into(), ParameterValue::Int(0));
    invalid.insert("a_unknown".into(), ParameterValue::Int(0));
    invalid.insert("frequency_hz".into(), ParameterValue::Float(f32::NAN));
    assert!(matches!(
        LfoModule::spawn(&mut bad_world, "bad", &invalid),
        Err(CoreError::UnknownParameter { param, .. }) if param == "a_unknown"
    ));
    assert_eq!(bad_world.entities().len(), before_entities);
    let _ = std::fs::remove_file(path);
}

#[test]
fn new_and_original_patch_files_are_parseable() {
    assert_eq!(read_patch(FIXTURE).modules.len(), 4);
    assert_eq!(read_patch(DEFAULT_FIXTURE).modules.len(), 3);
}

fn actual_schedule_app(fixed: bool) -> (App, InternedScheduleLabel, Entity) {
    actual_schedule_app_with_frequency(fixed, 62.5)
}

fn actual_schedule_app_with_frequency(
    fixed: bool,
    frequency_hz: f32,
) -> (App, InternedScheduleLabel, Entity) {
    let mut app = App::new();
    let schedule = if fixed {
        app.add_plugins(CorePlugin::fixed());
        FixedUpdate.intern()
    } else {
        app.add_plugins(CorePlugin);
        Update.intern()
    };
    LfoModule::register_systems(&mut app, schedule);

    let parameters = HashMap::from([
        ("waveform".into(), ParameterValue::String("saw_up".into())),
        ("frequency_hz".into(), ParameterValue::Float(frequency_hz)),
        ("phase_offset".into(), ParameterValue::Float(0.0)),
        ("pulse_width".into(), ParameterValue::Float(0.5)),
    ]);
    let lfo = LfoModule::spawn(app.world_mut(), "direct_lfo", &parameters).unwrap();
    (app, schedule, lfo)
}

fn corrupt_input_and_run(fixed: bool, input_name: &str, wrong_value: Value) {
    let (mut app, schedule, lfo) = actual_schedule_app(fixed);
    let input = parameter_port(app.world_mut(), lfo, input_name);
    app.world_mut().get_mut::<CurrentValue>(input).unwrap().0 = wrong_value;
    app.world_mut().resource_mut::<SimulationRunning>().0 = true;
    app.world_mut().run_schedule(schedule);
}

fn output_value(app: &App, port: Entity) -> Value {
    app.world().get::<CurrentValue>(port).unwrap().0
}

fn exercise_supplied_schedule(fixed: bool) {
    let (mut app, schedule, lfo) = actual_schedule_app(fixed);
    let unipolar_out = parameter_port(app.world_mut(), lfo, "out");
    let bipolar_out = parameter_port(app.world_mut(), lfo, "bipolar_out");
    let initial_unipolar = output_value(&app, unipolar_out);
    let initial_bipolar = output_value(&app, bipolar_out);
    assert_eq!(initial_unipolar, Value::Float(0.0));
    assert_eq!(initial_bipolar, Value::Bipolar(0.0));

    app.world_mut().resource_mut::<SimulationRunning>().0 = false;
    app.world_mut().run_schedule(schedule);
    assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 0);
    assert_eq!(output_value(&app, unipolar_out), initial_unipolar);
    assert_eq!(output_value(&app, bipolar_out), initial_bipolar);

    app.world_mut().resource_mut::<SimulationRunning>().0 = true;
    let mut bipolar = Vec::new();
    let mut unipolar = Vec::new();
    for _ in 0..4 {
        app.world_mut().run_schedule(schedule);
        let Value::Bipolar(bipolar_value) = output_value(&app, bipolar_out) else {
            panic!("LFO bipolar_out must publish Value::Bipolar");
        };
        let Value::Float(unipolar_value) = output_value(&app, unipolar_out) else {
            panic!("LFO out must publish Value::Float");
        };
        bipolar.push(bipolar_value);
        unipolar.push(unipolar_value);
    }
    assert_eq!(bipolar, [-1.0, -0.875, -0.75, -0.625]);
    assert_eq!(unipolar, [0.0, 0.0625, 0.125, 0.1875]);
    assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 4);

    app.world_mut().resource_mut::<SimulationRunning>().0 = false;
    app.world_mut().run_schedule(schedule);
    assert_eq!(output_value(&app, unipolar_out), Value::Float(0.1875));
    assert_eq!(output_value(&app, bipolar_out), Value::Bipolar(-0.625));
    assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 4);

    app.world_mut().resource_mut::<SimulationRunning>().0 = true;
    app.world_mut().run_schedule(schedule);
    assert_eq!(output_value(&app, unipolar_out), Value::Float(0.25));
    assert_eq!(output_value(&app, bipolar_out), Value::Bipolar(-0.5));
    assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 5);
}

#[test]
fn installed_lfo_runs_once_per_sample_and_honors_core_gating_in_update() {
    exercise_supplied_schedule(false);
}

#[test]
fn installed_lfo_runs_once_per_sample_and_honors_core_gating_in_fixed_update() {
    exercise_supplied_schedule(true);
}

#[test]
fn installed_lfo_keeps_typed_nonfinite_cv_neutral_in_both_schedules() {
    for fixed in [false, true] {
        let (mut app, schedule, lfo) = actual_schedule_app_with_frequency(fixed, 0.0);
        let frequency_in = parameter_port(app.world_mut(), lfo, "freq_in");
        let unipolar_out = parameter_port(app.world_mut(), lfo, "out");
        let bipolar_out = parameter_port(app.world_mut(), lfo, "bipolar_out");
        app.world_mut()
            .get_mut::<CurrentValue>(frequency_in)
            .unwrap()
            .0 = Value::Float(f32::NAN);
        app.world_mut().resource_mut::<SimulationRunning>().0 = true;

        for _ in 0..3 {
            app.world_mut().run_schedule(schedule);
            assert_eq!(output_value(&app, bipolar_out), Value::Bipolar(-1.0));
            assert_eq!(output_value(&app, unipolar_out), Value::Float(0.0));
            assert!(match output_value(&app, unipolar_out) {
                Value::Float(value) => value.is_finite(),
                _ => false,
            });
        }
        assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 3);
    }
}

#[test]
#[should_panic(expected = "LFO freq_in must carry Value::Float")]
fn update_schedule_rejects_wrong_frequency_input_type() {
    corrupt_input_and_run(false, "freq_in", Value::Gate(true));
}

#[test]
#[should_panic(expected = "LFO sync_in must carry Value::Gate")]
fn update_schedule_rejects_wrong_sync_input_type() {
    corrupt_input_and_run(false, "sync_in", Value::Float(0.5));
}

#[test]
#[should_panic(expected = "LFO freq_in must carry Value::Float")]
fn fixed_update_schedule_rejects_wrong_frequency_input_type() {
    corrupt_input_and_run(true, "freq_in", Value::Gate(true));
}

#[test]
#[should_panic(expected = "LFO sync_in must carry Value::Gate")]
fn fixed_update_schedule_rejects_wrong_sync_input_type() {
    corrupt_input_and_run(true, "sync_in", Value::Float(0.5));
}
