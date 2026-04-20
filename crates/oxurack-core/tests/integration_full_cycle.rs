//! Full-cycle integration test for the oxurack-core ECS pipeline.
//!
//! Defines two test modules (CounterModule and AccumulatorModule),
//! wires them with a cable via `apply_patch_to_world`, ticks for 100
//! frames, and asserts determinism and value propagation.

use std::collections::HashMap;

use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::hierarchy::ChildOf;
use bevy_ecs::prelude::{Component, Entity, Query};
use bevy_ecs::schedule::IntoScheduleConfigs;

use oxurack_core::{
    apply_patch_to_world, CableConfig, CorePlugin, CurrentValue, MergePolicy, Module, ModuleConfig,
    ModuleRegistry, OxurackModule, ParameterSchema, ParameterValue, Patch, Port, PortDirection,
    PortSchema, TickPhase, Value, ValueKind,
};

// ── CounterModule ───────────────────────────────────────────────────

/// ECS component holding the counter module's mutable state.
#[derive(Component, Debug, Default)]
struct CounterState {
    count: u32,
}

/// A test module that increments a counter each tick and writes
/// `Float(counter / 100.0)` to its `"out"` port.
struct CounterModule;

impl OxurackModule for CounterModule {
    const KIND: &'static str = "test_counter";
    const DISPLAY_NAME: &'static str = "Test Counter";
    const DESCRIPTION: &'static str = "Increments each tick for testing.";

    fn port_schema() -> &'static [PortSchema] {
        &[PortSchema {
            name: "out",
            direction: PortDirection::Output,
            value_kind: ValueKind::Float,
            merge_policy: MergePolicy::Reject,
            description: "Counter output",
        }]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        &[]
    }

    fn spawn(
        world: &mut bevy_ecs::world::World,
        instance_name: &str,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, oxurack_core::CoreError> {
        let module_entity =
            oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);

        for schema in Self::port_schema() {
            oxurack_core::spawn_port_on_module(
                world,
                module_entity,
                schema.name,
                schema.direction,
                schema.value_kind,
                schema.merge_policy,
            );
        }

        // Insert the mutable state component on the module entity.
        world
            .entity_mut(module_entity)
            .insert(CounterState::default());

        Ok(module_entity)
    }
}

/// Bevy plugin that registers `CounterModule` and its tick system.
struct CounterModulePlugin;

impl Plugin for CounterModulePlugin {
    fn build(&self, app: &mut App) {
        app.world_mut()
            .resource_mut::<ModuleRegistry>()
            .register::<CounterModule>();
        app.add_systems(
            Update,
            counter_tick_system.in_set(TickPhase::Produce),
        );
    }
}

/// Tick system for `CounterModule`.
///
/// Increments the counter and writes the scaled value to the `"out"`
/// port child entity.
fn counter_tick_system(
    mut module_q: Query<(Entity, &Module, &mut CounterState)>,
    mut port_q: Query<(&Port, &mut CurrentValue, &ChildOf)>,
) {
    for (module_entity, module, mut state) in module_q.iter_mut() {
        if module.kind.as_ref() != CounterModule::KIND {
            continue;
        }
        state.count += 1;
        let value = Value::Float(state.count as f32 / 100.0);

        for (port, mut cv, child_of) in port_q.iter_mut() {
            if child_of.0 == module_entity && port.name.as_ref() == "out" {
                cv.0 = value;
            }
        }
    }
}

// ── AccumulatorModule ───────────────────────────────────────────────

/// ECS component holding the accumulator module's mutable state.
#[derive(Component, Debug, Default)]
struct AccumulatorState {
    sum: f32,
}

/// A test module that reads its `"in"` port and accumulates the value
/// into a running sum, writing the sum to its `"out"` port.
struct AccumulatorModule;

impl OxurackModule for AccumulatorModule {
    const KIND: &'static str = "test_accumulator";
    const DISPLAY_NAME: &'static str = "Test Accumulator";
    const DESCRIPTION: &'static str = "Sums input values for testing.";

    fn port_schema() -> &'static [PortSchema] {
        &[
            PortSchema {
                name: "in",
                direction: PortDirection::Input,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::LastWins,
                description: "Value input",
            },
            PortSchema {
                name: "out",
                direction: PortDirection::Output,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::Reject,
                description: "Accumulated sum output",
            },
        ]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        &[]
    }

    fn spawn(
        world: &mut bevy_ecs::world::World,
        instance_name: &str,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, oxurack_core::CoreError> {
        let module_entity =
            oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);

        for schema in Self::port_schema() {
            oxurack_core::spawn_port_on_module(
                world,
                module_entity,
                schema.name,
                schema.direction,
                schema.value_kind,
                schema.merge_policy,
            );
        }

        // Insert the mutable state component on the module entity.
        world
            .entity_mut(module_entity)
            .insert(AccumulatorState::default());

        Ok(module_entity)
    }
}

/// Bevy plugin that registers `AccumulatorModule` and its tick system.
struct AccumulatorModulePlugin;

impl Plugin for AccumulatorModulePlugin {
    fn build(&self, app: &mut App) {
        app.world_mut()
            .resource_mut::<ModuleRegistry>()
            .register::<AccumulatorModule>();
        app.add_systems(
            Update,
            accumulator_tick_system.in_set(TickPhase::Produce),
        );
    }
}

/// Tick system for `AccumulatorModule`.
///
/// Reads the `"in"` port, adds its value to the running sum, and
/// writes the sum to the `"out"` port.
fn accumulator_tick_system(
    mut module_q: Query<(Entity, &Module, &mut AccumulatorState)>,
    mut port_q: Query<(&Port, &mut CurrentValue, &ChildOf)>,
) {
    for (module_entity, module, mut state) in module_q.iter_mut() {
        if module.kind.as_ref() != AccumulatorModule::KIND {
            continue;
        }

        // Single pass: read "in" and write "out" in one iteration.
        let mut input_value = 0.0_f32;
        for (port, cv, child_of) in port_q.iter() {
            if child_of.0 == module_entity
                && port.name.as_ref() == "in"
                && port.direction == PortDirection::Input
            {
                if let Value::Float(v) = cv.0 {
                    input_value = v;
                }
                break;
            }
        }

        state.sum += input_value;
        let output = Value::Float(state.sum);

        for (port, mut cv, child_of) in port_q.iter_mut() {
            if child_of.0 == module_entity
                && port.name.as_ref() == "out"
                && port.direction == PortDirection::Output
            {
                cv.0 = output;
                break;
            }
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────

/// Builds the test patch: counter_1 -> acc_1.
fn test_patch() -> Patch {
    Patch {
        version: "1.0".to_string(),
        master_seed: 42,
        bpm: 120.0,
        modules: vec![
            ModuleConfig {
                kind: "test_counter".to_string(),
                instance_name: "counter_1".to_string(),
                parameters: HashMap::new(),
            },
            ModuleConfig {
                kind: "test_accumulator".to_string(),
                instance_name: "acc_1".to_string(),
                parameters: HashMap::new(),
            },
        ],
        cables: vec![CableConfig {
            source: ("counter_1".to_string(), "out".to_string()),
            target: ("acc_1".to_string(), "in".to_string()),
            transform: None,
        }],
    }
}

/// Builds a standalone `ModuleRegistry` with both test module types
/// registered. This avoids needing to clone the registry out of the
/// Bevy world (which would require `Clone` on `ModuleRegistry`).
fn test_registry() -> ModuleRegistry {
    let mut registry = ModuleRegistry::default();
    registry.register::<CounterModule>();
    registry.register::<AccumulatorModule>();
    registry
}

/// Creates a test App with `CorePlugin` and both module plugins, then
/// applies the patch and returns `(app, patch_handle)`.
fn build_test_app(
    patch: &Patch,
) -> (App, oxurack_core::PatchHandle) {
    let mut app = App::new();
    app.add_plugins((CorePlugin, CounterModulePlugin, AccumulatorModulePlugin));

    // Build a standalone registry for patch application. The plugins
    // have already registered the modules in the world's registry,
    // but we need a separate reference for `apply_patch_to_world`.
    let registry = test_registry();

    let handle =
        apply_patch_to_world(patch, &registry, app.world_mut()).expect("patch should apply");

    (app, handle)
}

/// Finds a port entity by name among the children of the given module.
fn find_port(
    app: &mut App,
    module_entity: Entity,
    port_name: &str,
) -> Entity {
    let world = app.world_mut();
    let mut query = world.query::<(Entity, &Port, &ChildOf)>();
    query
        .iter(world)
        .find_map(|(entity, port, child_of)| {
            if child_of.0 == module_entity && port.name.as_ref() == port_name {
                Some(entity)
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            panic!(
                "port '{port_name}' not found on module entity {module_entity:?}"
            )
        })
}

/// Reads the current `Float` value of a port entity.
fn read_float(app: &App, port_entity: Entity) -> f32 {
    let cv = app
        .world()
        .entity(port_entity)
        .get::<CurrentValue>()
        .expect("port should have CurrentValue");
    match cv.0 {
        Value::Float(v) => v,
        other => panic!("expected Float, got {other:?}"),
    }
}

// ── Tests ───────────────────────────────────────────────────────────

/// Verifies that running 100 ticks propagates values through the
/// counter -> accumulator pipeline and that the accumulator's output
/// has changed from its initial value.
#[test]
fn test_full_cycle_propagation() {
    let patch = test_patch();
    let (mut app, handle) = build_test_app(&patch);

    let acc_entity = handle.modules["acc_1"];
    let acc_out = find_port(&mut app, acc_entity, "out");

    // Initial value should be zero.
    let initial = read_float(&app, acc_out);
    assert!(
        (initial - 0.0).abs() < f32::EPSILON,
        "accumulator output should start at 0.0, got {initial}"
    );

    // Run 100 ticks.
    for _ in 0..100 {
        app.update();
    }

    let final_value = read_float(&app, acc_out);
    assert!(
        final_value > 0.0,
        "accumulator output should be > 0 after 100 ticks, got {final_value}"
    );

    // The counter writes 1/100 on tick 1, 2/100 on tick 2, etc.
    // Due to the Produce -> Propagate -> Consume phase ordering, the
    // accumulator reads the previous tick's propagated value. On tick N,
    // it reads (N-1)/100. Over 100 ticks, it sums:
    //   0/100 + 1/100 + ... + 99/100 = 4950/100 = 49.5
    let expected = 49.5_f32;
    assert!(
        (final_value - expected).abs() < 0.01,
        "accumulator output should be ~{expected}, got {final_value}"
    );
}

/// Verifies that the same patch, run identically, produces the same
/// final output (determinism).
#[test]
fn test_full_cycle_determinism() {
    let patch = test_patch();

    let run = |p: &Patch| -> f32 {
        let (mut app, handle) = build_test_app(p);
        let acc_entity = handle.modules["acc_1"];
        let acc_out = find_port(&mut app, acc_entity, "out");

        for _ in 0..100 {
            app.update();
        }

        read_float(&app, acc_out)
    };

    let result_a = run(&patch);
    let result_b = run(&patch);

    assert!(
        (result_a - result_b).abs() < f32::EPSILON,
        "two runs with the same patch should produce identical output: {result_a} vs {result_b}"
    );
}

/// Verifies that the full test completes quickly (under 1 second).
#[test]
fn test_full_cycle_performance() {
    let start = std::time::Instant::now();

    let patch = test_patch();
    let (mut app, _handle) = build_test_app(&patch);

    for _ in 0..100 {
        app.update();
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed < std::time::Duration::from_secs(1),
        "100-tick test should complete in under 1 second, took {elapsed:?}"
    );
}

/// Verifies that the counter module's output increases monotonically.
#[test]
fn test_counter_output_monotonic() {
    let patch = test_patch();
    let (mut app, handle) = build_test_app(&patch);

    let counter_entity = handle.modules["counter_1"];
    let counter_out = find_port(&mut app, counter_entity, "out");

    let mut prev = read_float(&app, counter_out);

    for _ in 0..50 {
        app.update();
        let current = read_float(&app, counter_out);
        assert!(
            current >= prev,
            "counter output should be monotonically increasing: {prev} -> {current}"
        );
        prev = current;
    }
}

/// Verifies that the accumulator's sum grows monotonically (since
/// the counter always produces positive values).
#[test]
fn test_accumulator_sum_monotonic() {
    let patch = test_patch();
    let (mut app, handle) = build_test_app(&patch);

    let acc_entity = handle.modules["acc_1"];
    let acc_out = find_port(&mut app, acc_entity, "out");

    let mut prev = read_float(&app, acc_out);

    for _ in 0..50 {
        app.update();
        let current = read_float(&app, acc_out);
        assert!(
            current >= prev,
            "accumulator output should be monotonically increasing: {prev} -> {current}"
        );
        prev = current;
    }
}
