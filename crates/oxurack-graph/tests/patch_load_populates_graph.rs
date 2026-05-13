//! Integration test: loading a patch populates the RackGraph.

use std::collections::HashMap;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use oxurack_core::{
    CableConfig, CoreError, CorePlugin, MergePolicy, ModuleConfig, ModuleRegistry, OxurackModule,
    ParameterSchema, ParameterValue, Patch, PortDirection, PortSchema, ValueKind,
};
use oxurack_graph::{RackGraph, RackGraphPlugin};
use pretty_assertions::assert_eq;

// ── Test modules ───────────────────────────────────────────────────────

struct TestGen;

impl OxurackModule for TestGen {
    const KIND: &'static str = "test_gen";
    const DISPLAY_NAME: &'static str = "Test Generator";

    fn port_schema() -> &'static [PortSchema] {
        &[PortSchema {
            name: "out",
            direction: PortDirection::Output,
            value_kind: ValueKind::Float,
            merge_policy: MergePolicy::Reject,
            description: "Output",
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
        let entity = oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);
        for schema in Self::port_schema() {
            oxurack_core::spawn_port_on_module(
                world,
                entity,
                schema.name,
                schema.direction,
                schema.value_kind,
                schema.merge_policy,
            );
        }
        Ok(entity)
    }
}

struct TestSink;

impl OxurackModule for TestSink {
    const KIND: &'static str = "test_sink";
    const DISPLAY_NAME: &'static str = "Test Sink";

    fn port_schema() -> &'static [PortSchema] {
        &[PortSchema {
            name: "in",
            direction: PortDirection::Input,
            value_kind: ValueKind::Float,
            merge_policy: MergePolicy::LastWins,
            description: "Input",
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
        let entity = oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);
        for schema in Self::port_schema() {
            oxurack_core::spawn_port_on_module(
                world,
                entity,
                schema.name,
                schema.direction,
                schema.value_kind,
                schema.merge_policy,
            );
        }
        Ok(entity)
    }
}

fn two_module_patch() -> Patch {
    Patch {
        version: "1.0".to_string(),
        master_seed: 42,
        bpm: 120.0,
        modules: vec![
            ModuleConfig {
                kind: "test_gen".to_string(),
                instance_name: "gen_1".to_string(),
                parameters: HashMap::new(),
                display_name: String::new(),
                description: String::new(),
                tags: vec![],
            },
            ModuleConfig {
                kind: "test_sink".to_string(),
                instance_name: "sink_1".to_string(),
                parameters: HashMap::new(),
                display_name: String::new(),
                description: String::new(),
                tags: vec![],
            },
        ],
        cables: vec![CableConfig {
            source: ("gen_1".to_string(), "out".to_string()),
            target: ("sink_1".to_string(), "in".to_string()),
            transform: None,
            display_name: String::new(),
            description: String::new(),
            tags: vec![],
        }],
        rack: None,
    }
}

/// Test #10: loading a patch via `apply_patch_to_world` populates the
/// `RackGraph` with the correct module and cable counts.
#[test]
fn test_patch_load_populates_graph() {
    let mut app = App::new();
    app.add_plugins(CorePlugin);
    app.add_plugins(RackGraphPlugin);

    // Initial update to set up schedules.
    app.update();

    // Register module types.
    app.world_mut()
        .resource_mut::<ModuleRegistry>()
        .register::<TestGen>();
    app.world_mut()
        .resource_mut::<ModuleRegistry>()
        .register::<TestSink>();

    let patch = two_module_patch();

    // Apply the patch. This emits ModuleAdded and CableConnected.
    app.world_mut()
        .resource_scope::<ModuleRegistry, _>(|world, registry| {
            let _handle = oxurack_core::apply_patch_to_world(
                &patch,
                &registry,
                world,
                oxurack_core::PatchSource::InMemory,
            )
            .expect("patch should apply");
        });

    // Run one update cycle to let maintenance systems consume messages.
    app.update();

    let graph = app.world().resource::<RackGraph>();

    assert_eq!(
        graph.module_count(),
        patch.modules.len(),
        "graph module count should match patch"
    );
    assert_eq!(
        graph.cable_count(),
        patch.cables.len(),
        "graph cable count should match patch"
    );

    // Cross-check: both modules resolvable by instance name.
    assert!(graph.entity_by_instance_name("gen_1").is_some());
    assert!(graph.entity_by_instance_name("sink_1").is_some());

    // Cross-check: the entities are registered as modules.
    let gen_entity = graph.entity_by_instance_name("gen_1").unwrap();
    let sink_entity = graph.entity_by_instance_name("sink_1").unwrap();
    assert!(graph.contains_module(gen_entity));
    assert!(graph.contains_module(sink_entity));
}
