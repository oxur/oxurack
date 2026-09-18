use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};

use bevy_app::{App, FixedUpdate, Update};
use bevy_ecs::prelude::{Component, Entity, Query, ResMut, Resource, With, World};
use bevy_ecs::schedule::InternedScheduleLabel;
use oxurack::{CoreError, Oxurack, ParameterValue, PatchHandle};
use oxurack_core::{
    ModuleConfig, ModuleKind, ModuleRegistry, OxurackModule, ParameterGetter, ParameterRegistry,
    ParameterSchema, ParameterSetter, Patch, PortSchema,
};

const KIND: &str = "behavior_counter";
const PARAMETER: &str = "increment";

#[derive(Component, Clone, Debug, PartialEq, Eq)]
struct StepState {
    increment: i64,
    total: i64,
}

#[derive(Component)]
struct OriginalCounter;

#[derive(Component)]
struct ConflictingMarker;

#[derive(Resource, Default)]
struct HookCalls(usize);

#[derive(Resource, Default)]
struct ConflictingHookCalls(usize);

struct BehaviorCounter;

fn invalid_parameter(reason: &str) -> CoreError {
    CoreError::InvalidParameterValue {
        module: KIND.to_owned(),
        param: PARAMETER.to_owned(),
        reason: reason.to_owned(),
    }
}

fn set_increment(
    world: &mut World,
    entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError> {
    let ParameterValue::Int(increment) = value else {
        return Err(invalid_parameter("expected an integer"));
    };
    world
        .entity_mut(entity)
        .get_mut::<StepState>()
        .ok_or_else(|| invalid_parameter("module has no step state"))?
        .increment = increment;
    Ok(())
}

fn get_increment(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    world
        .entity(entity)
        .get::<StepState>()
        .map(|state| ParameterValue::Int(state.increment))
        .ok_or_else(|| invalid_parameter("module has no step state"))
}

fn behavior_counter_system(
    mut hook_calls: ResMut<HookCalls>,
    mut query: Query<&mut StepState, With<OriginalCounter>>,
) {
    hook_calls.0 += 1;
    for mut state in &mut query {
        state.total += state.increment;
    }
}

impl OxurackModule for BehaviorCounter {
    const KIND: &'static str = KIND;
    const DISPLAY_NAME: &'static str = "Behavior Counter";
    const DESCRIPTION: &'static str = "A fixture module with installed behavior.";

    fn port_schema() -> &'static [PortSchema] {
        &[]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        &[ParameterSchema {
            name: PARAMETER,
            description: "Amount added to the total on each fixed tick.",
            default: ParameterValue::Int(1),
        }]
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let increment = match parameters.get(PARAMETER) {
            Some(ParameterValue::Int(increment)) => *increment,
            None => 1,
            Some(_) => return Err(invalid_parameter("expected an integer")),
        };
        let entity = oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);
        world.entity_mut(entity).insert((
            OriginalCounter,
            StepState {
                increment,
                total: 0,
            },
        ));
        Ok(entity)
    }

    fn register_systems(app: &mut App, schedule: InternedScheduleLabel) {
        app.init_resource::<HookCalls>()
            .add_systems(schedule, behavior_counter_system);
    }

    fn parameter_setters() -> &'static [(&'static str, ParameterSetter)] {
        &[(PARAMETER, set_increment)]
    }

    fn parameter_getters() -> &'static [(&'static str, ParameterGetter)] {
        &[(PARAMETER, get_increment)]
    }
}

struct ConflictingCounter;

fn conflicting_setter(
    _world: &mut World,
    _entity: Entity,
    _value: ParameterValue,
) -> Result<(), CoreError> {
    Err(invalid_parameter("conflicting setter"))
}

fn conflicting_getter(_world: &World, _entity: Entity) -> Result<ParameterValue, CoreError> {
    Err(invalid_parameter("conflicting getter"))
}

fn conflicting_counter_system(mut hook_calls: ResMut<ConflictingHookCalls>) {
    hook_calls.0 += 1;
}

impl OxurackModule for ConflictingCounter {
    const KIND: &'static str = KIND;
    const DISPLAY_NAME: &'static str = "Conflicting Counter";
    const DESCRIPTION: &'static str = "A deliberately conflicting fixture module.";

    fn port_schema() -> &'static [PortSchema] {
        &[]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        &[ParameterSchema {
            name: PARAMETER,
            description: "A replacement parameter that must not be installed.",
            default: ParameterValue::Int(99),
        }]
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let entity = oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);
        world.entity_mut(entity).insert(ConflictingMarker);
        Ok(entity)
    }

    fn register_systems(app: &mut App, schedule: InternedScheduleLabel) {
        app.init_resource::<ConflictingHookCalls>()
            .add_systems(schedule, conflicting_counter_system);
    }

    fn parameter_setters() -> &'static [(&'static str, ParameterSetter)] {
        &[(PARAMETER, conflicting_setter)]
    }

    fn parameter_getters() -> &'static [(&'static str, ParameterGetter)] {
        &[(PARAMETER, conflicting_getter)]
    }
}

struct LegacyCounter;

impl OxurackModule for LegacyCounter {
    const KIND: &'static str = "legacy_counter";
    const DISPLAY_NAME: &'static str = "Legacy Counter";

    fn port_schema() -> &'static [PortSchema] {
        &[]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
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
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/behavior-installation.ron")
}

fn add_update_schedule(app: &mut App) {
    app.add_systems(Update, || {});
}

fn state(app: &App, handle: &PatchHandle, name: &str) -> StepState {
    app.world()
        .entity(handle.modules[name])
        .get::<StepState>()
        .unwrap_or_else(|| panic!("{name} should have StepState"))
        .clone()
}

fn set_parameter(app: &mut App, entity: Entity, value: i64) {
    let kind = ModuleKind::from(KIND);
    app.world_mut()
        .resource_scope::<ParameterRegistry, _>(|world, registry| {
            registry
                .set_parameter(world, entity, &kind, PARAMETER, ParameterValue::Int(value))
                .expect("fixture setter should remain registered");
        });
}

fn get_parameter(app: &mut App, entity: Entity) -> ParameterValue {
    let kind = ModuleKind::from(KIND);
    app.world_mut()
        .resource_scope::<ParameterRegistry, _>(|world, registry| {
            registry
                .get_parameter(world, entity, &kind, PARAMETER)
                .expect("fixture getter should remain registered")
        })
}

fn save_legacy_patch() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "oxurack-module-installation-legacy-{}.ron",
        std::process::id()
    ));
    let patch = Patch {
        version: "1.0".to_owned(),
        master_seed: 42,
        bpm: 120.0,
        modules: vec![ModuleConfig {
            kind: LegacyCounter::KIND.to_owned(),
            instance_name: "legacy".to_owned(),
            parameters: HashMap::new(),
            display_name: String::new(),
            description: String::new(),
            tags: vec![],
        }],
        cables: vec![],
        rack: None,
    };
    oxurack_core::save_patch_to_file(&patch, &path).expect("legacy fixture should save");
    path
}

#[test]
fn installed_systems_execute_each_loaded_instance_on_fixed_update() {
    let mut rack = Oxurack::new();
    rack.install::<BehaviorCounter>();
    rack.load_patch(&fixture_path())
        .expect("behavior fixture should load");
    let mut app = rack.into_app();
    add_update_schedule(&mut app);
    let handle = app.world().resource::<PatchHandle>().clone();

    assert_eq!(state(&app, &handle, "alpha").total, 0);
    assert_eq!(state(&app, &handle, "beta").total, 0);
    assert_eq!(state(&app, &handle, "alpha").increment, 2);
    assert_eq!(state(&app, &handle, "beta").increment, 3);
    assert_eq!(app.world().resource::<HookCalls>().0, 0);

    app.world_mut().run_schedule(Update);
    assert_eq!(state(&app, &handle, "alpha").total, 0);
    assert_eq!(state(&app, &handle, "beta").total, 0);
    assert_eq!(app.world().resource::<HookCalls>().0, 0);

    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(state(&app, &handle, "alpha").total, 2);
    assert_eq!(state(&app, &handle, "beta").total, 3);
    assert_eq!(app.world().resource::<HookCalls>().0, 1);

    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(state(&app, &handle, "alpha").total, 4);
    assert_eq!(state(&app, &handle, "beta").total, 6);
    assert_eq!(app.world().resource::<HookCalls>().0, 2);

    #[cfg(feature = "graph")]
    assert_eq!(
        app.world()
            .resource::<oxurack_graph::RackGraph>()
            .module_count(),
        2
    );
}

#[test]
fn same_type_reinstall_preserves_loaded_state_and_accessors() {
    let mut rack = Oxurack::new();
    rack.install::<BehaviorCounter>();
    rack.load_patch(&fixture_path())
        .expect("behavior fixture should load");
    rack.try_install::<BehaviorCounter>()
        .expect("same type reinstall should be idempotent");

    let mut app = rack.into_app();
    add_update_schedule(&mut app);
    let handle = app.world().resource::<PatchHandle>().clone();
    app.world_mut().run_schedule(FixedUpdate);
    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(state(&app, &handle, "alpha").total, 4);
    assert_eq!(state(&app, &handle, "beta").total, 6);

    let alpha = handle.modules["alpha"];
    set_parameter(&mut app, alpha, 4);
    assert_eq!(get_parameter(&mut app, alpha), ParameterValue::Int(4));
    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(state(&app, &handle, "alpha").total, 8);
    assert_eq!(state(&app, &handle, "beta").total, 9);
    assert_eq!(app.world().resource::<HookCalls>().0, 3);
}

#[test]
fn conflicting_type_returns_before_mutation_and_original_behavior_survives() {
    let mut rack = Oxurack::new();
    rack.install::<BehaviorCounter>();
    let error = match rack.try_install::<ConflictingCounter>() {
        Ok(_) => panic!("same-kind different type should conflict"),
        Err(error) => error,
    };
    match error {
        CoreError::ModuleKindConflict { kind } => assert_eq!(kind, KIND),
        other => panic!("unexpected installation error: {other}"),
    }

    rack.load_patch(&fixture_path())
        .expect("original registration should still load the fixture");
    let mut app = rack.into_app();
    add_update_schedule(&mut app);
    let handle = app.world().resource::<PatchHandle>().clone();
    let registration = app
        .world()
        .resource::<ModuleRegistry>()
        .get(&ModuleKind::from(KIND))
        .expect("original registration should remain");
    assert_eq!(registration.display_name, "Behavior Counter");
    let original_spawner = registration.spawner;
    let mut probe_world = World::new();
    let probe = original_spawner(&mut probe_world, "probe", &HashMap::new())
        .expect("original spawner should remain");
    assert!(probe_world.entity(probe).contains::<OriginalCounter>());

    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(state(&app, &handle, "alpha").total, 2);
    assert_eq!(state(&app, &handle, "beta").total, 3);
    assert!(app.world().get_resource::<ConflictingHookCalls>().is_none());

    set_parameter(&mut app, handle.modules["alpha"], 4);
    assert_eq!(
        get_parameter(&mut app, handle.modules["alpha"]),
        ParameterValue::Int(4)
    );
    assert!(app.world().get_resource::<ConflictingHookCalls>().is_none());
}

#[test]
fn installations_are_rack_local_and_legacy_modules_still_load() {
    let mut first = Oxurack::new();
    let mut second = Oxurack::new();
    first.install::<BehaviorCounter>();
    second.install::<BehaviorCounter>();
    first
        .load_patch(&fixture_path())
        .expect("first fixture should load");
    second
        .load_patch(&fixture_path())
        .expect("second fixture should load");

    let mut first_app = first.into_app();
    let mut second_app = second.into_app();
    let first_handle = first_app.world().resource::<PatchHandle>().clone();
    let second_handle = second_app.world().resource::<PatchHandle>().clone();
    first_app.world_mut().run_schedule(FixedUpdate);
    second_app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(state(&first_app, &first_handle, "alpha").total, 2);
    assert_eq!(state(&second_app, &second_handle, "alpha").total, 2);
    assert_eq!(first_app.world().resource::<HookCalls>().0, 1);
    assert_eq!(second_app.world().resource::<HookCalls>().0, 1);

    let path = save_legacy_patch();
    let mut legacy_rack = Oxurack::new();
    legacy_rack
        .install::<LegacyCounter>()
        .install::<LegacyCounter>();
    legacy_rack
        .load_patch(&path)
        .expect("legacy module should load with the default hook");
    let _ = std::fs::remove_file(path);
    let legacy_app = legacy_rack.into_app();
    assert_eq!(
        legacy_app.world().resource::<PatchHandle>().modules.len(),
        1
    );
}

#[test]
fn old_install_panics_clearly_but_try_install_is_typed() {
    let panic = catch_unwind(AssertUnwindSafe(|| {
        let mut rack = Oxurack::new();
        rack.install::<BehaviorCounter>();
        rack.install::<ConflictingCounter>();
    }))
    .expect_err("the chaining API should panic on a programmer collision");
    let message = if let Some(message) = panic.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = panic.downcast_ref::<&str>() {
        (*message).to_owned()
    } else {
        String::new()
    };
    assert!(
        message.contains(KIND),
        "panic should name the kind: {message}"
    );

    let mut rack = Oxurack::new();
    rack.install::<BehaviorCounter>();
    assert!(matches!(
        rack.try_install::<ConflictingCounter>(),
        Err(CoreError::ModuleKindConflict { kind }) if kind == KIND
    ));
}
