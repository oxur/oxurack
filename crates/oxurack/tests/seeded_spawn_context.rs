//! Public umbrella load proof for pre-construction spawn context delivery.

use std::collections::HashMap;
use std::path::Path;

use bevy_ecs::prelude::{Component, Entity, World};
use oxurack::{CoreError, ModuleSpawnContext, Oxurack, ParameterValue};
use oxurack_core::{ModuleId, OxurackModule, ParameterSchema, PortSchema};

const KIND: &str = "umbrella_seeded_probe";

#[derive(Component, Debug, PartialEq, Eq)]
struct ContextState {
    master_seed: u64,
    module_id: ModuleId,
    instance_seed: u64,
    offset: i64,
}

struct UmbrellaSeededProbe;

impl OxurackModule for UmbrellaSeededProbe {
    const KIND: &'static str = KIND;
    const DISPLAY_NAME: &'static str = "Umbrella Seeded Probe";

    fn port_schema() -> &'static [PortSchema] {
        &[]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        &[ParameterSchema {
            name: "offset",
            description: "Initial context-probe offset.",
            default: ParameterValue::Int(0),
        }]
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

    fn spawn_with_context(
        world: &mut World,
        context: &ModuleSpawnContext<'_>,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let offset = match parameters.get("offset") {
            Some(ParameterValue::Int(value)) => *value,
            None => 0,
            Some(_) => {
                return Err(CoreError::InvalidParameterValue {
                    module: Self::KIND.to_owned(),
                    param: "offset".to_owned(),
                    reason: "expected an integer".to_owned(),
                });
            }
        };
        let entity = oxurack_core::spawn_module_entity(world, Self::KIND, context.instance_name());
        world.entity_mut(entity).insert(ContextState {
            master_seed: context.master_seed(),
            module_id: context.module_id(),
            instance_seed: context.instance_seed(),
            offset,
        });
        Ok(entity)
    }
}

#[test]
fn public_umbrella_load_delivers_context_before_state_construction() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/seeded-spawn-context.ron");
    let mut rack = Oxurack::new();
    rack.install::<UmbrellaSeededProbe>();
    rack.load_patch(&fixture)
        .expect("public umbrella patch load should succeed");

    let app = rack.into_app();
    let handle = app.world().resource::<oxurack_core::PatchHandle>().clone();
    let state = app
        .world()
        .entity(handle.modules["tm_a"])
        .get::<ContextState>()
        .expect("context-aware state should be present");
    assert_eq!(state.master_seed, 42);
    assert_eq!(state.module_id, ModuleId::from_instance_name("tm_a"));
    assert_eq!(state.instance_seed, 0x2181_9f1b_fbd4_fcea);
    assert_eq!(state.offset, 2);
}
