//! Seeded module construction, patch handoff, and fresh-process replay tests.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::{Component, Entity, Query, Res, ResMut, Resource, World};
use bevy_ecs::schedule::IntoScheduleConfigs;
use oxurack_core::{
    CoreError, CorePlugin, Module, ModuleConfig, ModuleId, ModuleKind, ModuleRegistry,
    ModuleSpawnContext, OxurackModule, ParameterSchema, ParameterValue, Patch, PatchSource,
    PortSchema, SampleTime, SimulationRunning, TickPhase, apply_patch_to_world, deserialize_patch,
};
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};

const CHILD_ENV: &str = "OXURACK_SEEDED_PROBE_CHILD";
const PROBE_KIND: &str = "seeded_probe";
const OFFSET_PARAMETER: &str = "offset";
const EXPECTED_TRACE: &str = concat!(
    "0|tm_a|4fffb24cd3888b92|2\n",
    "0|tm_b|98a8f97c490c3bfe|3\n",
    "1|tm_a|2cc8263b24bccaa8|2\n",
    "1|tm_b|ebb0290dbb044775|3\n",
    "2|tm_a|a5198b340227b2dc|2\n",
    "2|tm_b|4bbaa57382d07c6f|3",
);

#[derive(Component, Debug, PartialEq, Eq)]
struct ProbeMetadata {
    master_seed: u64,
    module_id: ModuleId,
    instance_seed: u64,
    offset: i64,
}

#[derive(Component)]
struct ProbeState {
    rng: SmallRng,
    offset: i64,
}

#[derive(Resource, Default)]
struct ProbeTrace {
    rows: Vec<String>,
}

struct SeededProbe;

fn invalid_parameter(reason: &str) -> CoreError {
    CoreError::InvalidParameterValue {
        module: PROBE_KIND.to_owned(),
        param: OFFSET_PARAMETER.to_owned(),
        reason: reason.to_owned(),
    }
}

fn offset(parameters: &HashMap<String, ParameterValue>) -> Result<i64, CoreError> {
    match parameters.get(OFFSET_PARAMETER) {
        Some(ParameterValue::Int(value)) => Ok(*value),
        None => Ok(0),
        Some(_) => Err(invalid_parameter("expected an integer")),
    }
}

impl OxurackModule for SeededProbe {
    const KIND: &'static str = PROBE_KIND;
    const DISPLAY_NAME: &'static str = "Seeded Probe";

    fn port_schema() -> &'static [PortSchema] {
        &[]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        &[ParameterSchema {
            name: OFFSET_PARAMETER,
            description: "Initial trace offset.",
            default: ParameterValue::Int(0),
        }]
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let offset = offset(parameters)?;
        let entity = oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);
        world.entity_mut(entity).insert((
            ProbeMetadata {
                master_seed: 0,
                module_id: ModuleId::from_instance_name(instance_name),
                instance_seed: 0,
                offset,
            },
            ProbeState {
                rng: SmallRng::seed_from_u64(0),
                offset,
            },
        ));
        Ok(entity)
    }

    fn spawn_with_context(
        world: &mut World,
        context: &ModuleSpawnContext<'_>,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let offset = offset(parameters)?;
        let entity = oxurack_core::spawn_module_entity(world, Self::KIND, context.instance_name());
        world.entity_mut(entity).insert((
            ProbeMetadata {
                master_seed: context.master_seed(),
                module_id: context.module_id(),
                instance_seed: context.instance_seed(),
                offset,
            },
            ProbeState {
                rng: SmallRng::seed_from_u64(context.instance_seed()),
                offset,
            },
        ));
        Ok(entity)
    }
}

struct FailingProbe;

impl OxurackModule for FailingProbe {
    const KIND: &'static str = "failing_probe";
    const DISPLAY_NAME: &'static str = "Failing Probe";

    fn port_schema() -> &'static [PortSchema] {
        &[]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        &[]
    }

    fn spawn(
        _world: &mut World,
        _instance_name: &str,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        Err(CoreError::InvalidParameterValue {
            module: Self::KIND.to_owned(),
            param: "spawn".to_owned(),
            reason: "deliberate spawn failure".to_owned(),
        })
    }
}

#[derive(Component)]
struct LegacyMarker;

struct LegacyOnly;

impl OxurackModule for LegacyOnly {
    const KIND: &'static str = "legacy_only";
    const DISPLAY_NAME: &'static str = "Legacy Only";

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
        let entity = oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);
        world.entity_mut(entity).insert(LegacyMarker);
        Ok(entity)
    }
}

fn probe_observe_system(
    sample_time: Res<SampleTime>,
    mut trace: ResMut<ProbeTrace>,
    mut probes: Query<(&Module, &mut ProbeState)>,
) {
    let Some(index) = sample_time.last_index() else {
        return;
    };

    let mut rows: Vec<(String, u64, i64)> = probes
        .iter_mut()
        .map(|(module, mut state)| {
            (
                module.instance_name.clone(),
                state.rng.random(),
                state.offset,
            )
        })
        .collect();
    rows.sort_by(|left, right| left.0.cmp(&right.0));

    trace.rows.extend(
        rows.into_iter()
            .map(|(name, value, offset)| format!("{index}|{name}|{value:016x}|{offset}")),
    );
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/seeded-spawn-context.ron")
}

fn fixture() -> Patch {
    deserialize_patch(
        &std::fs::read_to_string(fixture_path()).expect("seeded fixture should be readable"),
    )
    .expect("seeded fixture should parse")
}

fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins(CorePlugin::fixed());
    app.init_resource::<ProbeTrace>()
        .add_systems(FixedUpdate, probe_observe_system.in_set(TickPhase::Observe));
    app.world_mut()
        .resource_mut::<ModuleRegistry>()
        .register::<SeededProbe>();
    app
}

fn apply(app: &mut App, patch: &Patch) -> oxurack_core::PatchHandle {
    app.world_mut()
        .resource_scope::<ModuleRegistry, _>(|world, registry| {
            apply_patch_to_world(patch, &registry, world, PatchSource::InMemory)
        })
        .expect("patch should apply")
}

fn trace(app: &App) -> String {
    app.world().resource::<ProbeTrace>().rows.join("\n")
}

fn run_trace(patch: &Patch, samples: usize) -> String {
    let mut app = build_app();
    apply(&mut app, patch);
    for _ in 0..samples {
        app.world_mut().run_schedule(FixedUpdate);
    }
    trace(&app)
}

fn reference_trace() -> String {
    let mut probes = [
        ("tm_a", 0x2181_9f1b_fbd4_fcea, 2_i64),
        ("tm_b", 0x2181_9e1b_fbd4_fb37, 3_i64),
    ];
    let mut rows = Vec::new();
    for index in 0..3 {
        for (name, seed, offset) in &mut probes {
            let mut rng = SmallRng::seed_from_u64(*seed);
            for _ in 0..index {
                let _: u64 = rng.random();
            }
            rows.push(format!(
                "{index}|{name}|{:016x}|{offset}",
                rng.random::<u64>()
            ));
        }
    }
    rows.join("\n")
}

fn reordered_patch() -> Patch {
    let mut patch = fixture();
    patch.modules.reverse();
    patch
}

#[test]
fn context_fields_and_parameters_are_visible_before_state_construction() {
    let mut app = build_app();
    let handle = apply(&mut app, &fixture());

    let world = app.world();
    let alpha = world.entity(handle.modules["tm_a"]);
    let metadata = alpha
        .get::<ProbeMetadata>()
        .expect("context metadata should be constructed with the module");
    assert_eq!(metadata.master_seed, 42);
    assert_eq!(metadata.module_id, ModuleId::from_instance_name("tm_a"));
    assert_eq!(metadata.instance_seed, 0x2181_9f1b_fbd4_fcea);
    assert_eq!(metadata.offset, 2);
    assert_eq!(
        ModuleId::from_instance_name("vco_1"),
        ModuleId(0xfda4_2c6e_50ff_0d2a)
    );
}

#[test]
fn default_context_hook_forwards_to_legacy_spawn() {
    let mut registry = ModuleRegistry::default();
    registry.register::<LegacyOnly>();
    let patch = Patch {
        version: "1.0".to_owned(),
        master_seed: 42,
        bpm: 120.0,
        modules: vec![ModuleConfig {
            kind: LegacyOnly::KIND.to_owned(),
            instance_name: "legacy".to_owned(),
            ..Default::default()
        }],
        cables: vec![],
        rack: None,
    };
    let mut app = App::new();
    app.add_plugins(CorePlugin::fixed());
    let handle = apply_patch_to_world(&patch, &registry, app.world_mut(), PatchSource::InMemory)
        .expect("legacy default hook should load");
    assert!(
        app.world()
            .entity(handle.modules["legacy"])
            .contains::<LegacyMarker>()
    );
}

#[test]
fn direct_registration_falls_back_to_legacy_spawner_and_replacement_clears_hook() {
    let mut registry = ModuleRegistry::default();
    registry.register::<SeededProbe>();
    let kind = ModuleKind::from(PROBE_KIND);
    let registration = registry
        .get(&kind)
        .expect("registered probe should exist")
        .clone();
    registry.insert(kind, registration);

    let mut app = App::new();
    app.add_plugins(CorePlugin::fixed());
    let handle = apply_patch_to_world(
        &fixture(),
        &registry,
        app.world_mut(),
        PatchSource::InMemory,
    )
    .expect("manual replacement registration should load");
    let metadata = app
        .world()
        .entity(handle.modules["tm_a"])
        .get::<ProbeMetadata>();
    assert_eq!(metadata.expect("legacy state should exist").master_seed, 0);
}

#[test]
fn failed_spawn_does_not_leak_context_to_an_independent_load() {
    let mut app = build_app();
    app.world_mut()
        .resource_mut::<ModuleRegistry>()
        .register::<FailingProbe>();
    let bad = Patch {
        version: "1.0".to_owned(),
        master_seed: 99,
        bpm: 120.0,
        modules: vec![ModuleConfig {
            kind: FailingProbe::KIND.to_owned(),
            instance_name: "failed".to_owned(),
            ..Default::default()
        }],
        cables: vec![],
        rack: None,
    };
    let failure = app
        .world_mut()
        .resource_scope::<ModuleRegistry, _>(|world, registry| {
            apply_patch_to_world(&bad, &registry, world, PatchSource::InMemory)
        });
    assert!(
        failure.is_err(),
        "the deliberate spawner failure must surface"
    );

    let handle = apply(&mut app, &fixture());
    let metadata = app.world().entity(handle.modules["tm_a"]);
    assert_eq!(
        metadata
            .get::<ProbeMetadata>()
            .expect("independent load should construct probe state")
            .instance_seed,
        0x2181_9f1b_fbd4_fcea
    );
}

#[test]
fn reordered_and_changed_inputs_have_the_required_trace_relationships() {
    let original = run_trace(&fixture(), 3);
    let reordered = run_trace(&reordered_patch(), 3);
    assert_eq!(
        original, reordered,
        "per-name traces must ignore patch order"
    );
    assert_eq!(original, EXPECTED_TRACE);

    let mut changed_seed = fixture();
    changed_seed.master_seed = 43;
    assert_ne!(original, run_trace(&changed_seed, 3));

    let mut changed_name = fixture();
    changed_name.modules[0].instance_name = "tm_c".to_owned();
    assert_ne!(original, run_trace(&changed_name, 3));
}

#[test]
fn stop_resume_continues_and_fresh_load_restarts_trace() {
    let mut app = build_app();
    apply(&mut app, &fixture());
    app.world_mut().run_schedule(FixedUpdate);
    let first = trace(&app);

    app.world_mut().resource_mut::<SimulationRunning>().0 = false;
    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(
        trace(&app),
        first,
        "stopped schedule must not advance probes"
    );

    app.world_mut().resource_mut::<SimulationRunning>().0 = true;
    app.world_mut().run_schedule(FixedUpdate);
    assert_ne!(
        trace(&app),
        first,
        "resume must continue the owned RNG state"
    );

    assert_eq!(
        run_trace(&fixture(), 1),
        first,
        "fresh patch construction must restart from the fixture seed"
    );
}

fn child_trace() -> String {
    let output = Command::new(std::env::current_exe().expect("test executable path"))
        .args(["--exact", "seeded_probe_child", "--nocapture"])
        .env(CHILD_ENV, "1")
        .output()
        .expect("child process should start");
    assert!(
        output.status.success(),
        "child failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("child stdout should be UTF-8");
    let begin = stdout.find("TRACE_BEGIN\n").expect("child trace marker");
    let end = stdout[begin..]
        .find("\nTRACE_END")
        .map(|offset| begin + offset)
        .expect("child trace end marker");
    stdout[begin + "TRACE_BEGIN\n".len()..end].to_owned()
}

#[test]
fn seeded_probe_child() {
    if std::env::var_os(CHILD_ENV).is_none() {
        return;
    }
    println!("TRACE_BEGIN");
    println!("{}", run_trace(&fixture(), 3));
    println!("TRACE_END");
}

#[test]
fn fresh_process_replay_matches_committed_fixture_twice() {
    if std::env::var_os(CHILD_ENV).is_some() {
        return;
    }
    let first = child_trace();
    let second = child_trace();
    assert_eq!(first, EXPECTED_TRACE);
    assert_eq!(second, EXPECTED_TRACE);
    assert_eq!(
        first, second,
        "separate child processes must replay exactly"
    );
}

#[test]
fn committed_trace_fixture_comes_from_hard_coded_plan_golden_seeds() {
    assert_eq!(reference_trace(), EXPECTED_TRACE);
}
