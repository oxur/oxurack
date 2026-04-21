//! Umbrella crate for the oxurack modular MIDI rack.
//!
//! [`Oxurack`] is the rack: install modules, load patches, set knobs,
//! power on, play. The Bevy ECS engine runs underneath but stays out
//! of your way unless you ask for it via [`Oxurack::into_app`].

use std::path::Path;
use std::time::Duration;

use bevy_app::prelude::App;
use bevy_ecs::prelude::Resource;
use bevy_time::prelude::{Fixed, Time};

// ── Re-exports from oxurack-core ────────────────────────────────────
//
// Curated public surface. Users import from `oxurack::`, not
// `oxurack::oxurack_core::`.

pub use oxurack_core::OxurackModule as Module;

pub use oxurack_core::{CoreError, CorePlugin, CorePluginFixed, PatchHandle, Scale};

pub use oxurack_core::{
    CableConfig, CableTransform, ModuleConfig, ParameterValue, Patch, Value, ValueKind,
};

pub use oxurack_core::{
    MergePolicy, ModuleId, ModuleKind, ParameterName, ParameterSchema, Port, PortDirection,
    PortName, PortSchema, TickPhase,
};

pub use oxurack_core::{CurrentValue, spawn_module_entity, spawn_port_on_module};

/// Whether the simulation tick loop is running.
///
/// Module tick systems (clock, LFO, etc.) should early-return when
/// this is `false`. Defaults to `false` on construction.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationRunning(pub bool);

/// The rack. Install modules, load patches, power on, play.
///
/// Internally wraps a Bevy [`App`] with fixed-timestep simulation
/// at 1000 Hz. Modules are installed via [`install`](Self::install),
/// patches loaded via [`load_patch`](Self::load_patch). The Bevy
/// engine is accessible via [`into_app`](Self::into_app) when needed.
pub struct Oxurack {
    app: App,
}

impl Oxurack {
    /// Build a new rack with fixed-timestep simulation at 1000 Hz.
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_plugins(bevy_time::TimePlugin);
        app.add_plugins(oxurack_core::CorePlugin::fixed());

        app.world_mut()
            .resource_mut::<Time<Fixed>>()
            .set_timestep(Duration::from_millis(1));

        app.init_resource::<SimulationRunning>();

        Self { app }
    }

    /// Install a module type into the rack.
    ///
    /// After installation, patches referencing `M::KIND` can be loaded
    /// and their module instances will be spawned by `M::spawn`.
    pub fn install<M: oxurack_core::OxurackModule>(&mut self) -> &mut Self {
        self.app
            .world_mut()
            .resource_mut::<oxurack_core::ModuleRegistry>()
            .register::<M>();
        self
    }

    /// Loads a patch from a RON file and applies it to the world.
    ///
    /// On success, the resulting [`PatchHandle`](oxurack_core::PatchHandle)
    /// is stored as a resource. If a handle from a previous load exists,
    /// it is replaced.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError`](oxurack_core::CoreError) on I/O, parse,
    /// validation, or spawn failure. On error, no `PatchHandle` is
    /// inserted (or the previous one is left intact).
    pub fn load_patch(&mut self, path: &Path) -> Result<(), oxurack_core::CoreError> {
        let patch = oxurack_core::load_patch_from_file(path)?;

        // resource_scope temporarily removes ModuleRegistry from the
        // world, giving us an &-ref to pass to apply_patch_to_world
        // alongside the &mut World it needs for spawning.
        self.app
            .world_mut()
            .resource_scope::<oxurack_core::ModuleRegistry, _>(|world, registry| {
                let handle = oxurack_core::apply_patch_to_world(&patch, &registry, world)?;
                world.insert_resource(handle);
                Ok::<(), oxurack_core::CoreError>(())
            })?;
        Ok(())
    }

    /// Runs a single app update cycle.
    ///
    /// In fixed-timestep mode, this advances `Time<Fixed>` and may
    /// trigger zero or more fixed-update steps depending on elapsed
    /// time.
    pub fn step(&mut self) {
        self.app.update();
    }

    /// Runs the app to completion (blocking).
    pub fn run(mut self) {
        self.app.run();
    }

    /// Consume the rack and return the underlying Bevy [`App`].
    pub fn into_app(self) -> App {
        self.app
    }

    #[cfg(test)]
    fn world(&self) -> &bevy_ecs::world::World {
        self.app.world()
    }

    #[cfg(test)]
    fn world_mut(&mut self) -> &mut bevy_ecs::world::World {
        self.app.world_mut()
    }
}

impl Default for Oxurack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxurack_core::{
        CableConfig, CoreError, MergePolicy, ModuleConfig, ModuleRegistry, OxurackModule,
        ParameterSchema, ParameterValue, PatchHandle, PortDirection, PortSchema, ValueKind,
    };
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    // ── Test module ────────────────────────────────────────────────

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
            world: &mut bevy_ecs::world::World,
            instance_name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<bevy_ecs::prelude::Entity, CoreError> {
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
            world: &mut bevy_ecs::world::World,
            instance_name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<bevy_ecs::prelude::Entity, CoreError> {
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

    fn two_module_patch() -> oxurack_core::Patch {
        oxurack_core::Patch {
            version: "1.0".to_string(),
            master_seed: 42,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "test_gen".to_string(),
                    instance_name: "gen_1".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "test_sink".to_string(),
                    instance_name: "sink_1".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![CableConfig {
                source: ("gen_1".to_string(), "out".to_string()),
                target: ("sink_1".to_string(), "in".to_string()),
                transform: None,
            }],
        }
    }

    fn save_patch_to_temp(patch: &oxurack_core::Patch, name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(name);
        oxurack_core::save_patch_to_file(patch, &path).expect("save should succeed");
        path
    }

    // ── Tests ──────────────────────────────────────────────────────

    #[test]
    fn test_new_has_simulation_running_false() {
        let oxa = Oxurack::new();
        let sr = oxa
            .world()
            .get_resource::<SimulationRunning>()
            .expect("SimulationRunning should be present");
        assert!(!sr.0, "SimulationRunning should default to false");
    }

    #[test]
    fn test_new_has_time_fixed_at_1ms() {
        let oxa = Oxurack::new();
        let time = oxa
            .world()
            .get_resource::<Time<Fixed>>()
            .expect("Time<Fixed> should be present");
        assert_eq!(
            time.timestep(),
            Duration::from_millis(1),
            "Time<Fixed> should be exactly 1 ms"
        );
    }

    #[test]
    fn test_new_has_module_registry() {
        let oxa = Oxurack::new();
        assert!(
            oxa.world().get_resource::<ModuleRegistry>().is_some(),
            "ModuleRegistry should be present"
        );
    }

    #[test]
    fn test_install_populates_registry() {
        let mut oxa = Oxurack::new();
        oxa.install::<TestGen>();

        let registry = oxa.world().get_resource::<ModuleRegistry>().unwrap();
        assert!(
            registry.contains(&oxurack_core::ModuleKind::from("test_gen")),
            "test_gen should be registered"
        );
    }

    #[test]
    fn test_load_patch_stores_handle() {
        let mut oxa = Oxurack::new();
        oxa.install::<TestGen>();
        oxa.install::<TestSink>();

        let path = save_patch_to_temp(&two_module_patch(), "oxurack_test_umbrella_load.ron");
        oxa.load_patch(&path).expect("load_patch should succeed");
        let _ = std::fs::remove_file(&path);

        let handle = oxa
            .world()
            .get_resource::<PatchHandle>()
            .expect("PatchHandle should be stored after load_patch");
        assert_eq!(handle.modules.len(), 2);
        assert!(handle.modules.contains_key("gen_1"));
        assert!(handle.modules.contains_key("sink_1"));
    }

    #[test]
    fn test_load_patch_unregistered_kind_errors() {
        let mut oxa = Oxurack::new();
        // Do NOT register any modules.

        let path = save_patch_to_temp(&two_module_patch(), "oxurack_test_umbrella_unreg.ron");
        let result = oxa.load_patch(&path);
        let _ = std::fs::remove_file(&path);

        assert!(result.is_err(), "should fail for unregistered module kind");

        assert!(
            oxa.world().get_resource::<PatchHandle>().is_none(),
            "PatchHandle should not be inserted on error"
        );
    }

    #[test]
    fn test_load_patch_replaces_previous_handle() {
        let mut oxa = Oxurack::new();
        oxa.install::<TestGen>();
        oxa.install::<TestSink>();

        // Load patch A.
        let patch_a = two_module_patch();
        let path_a = save_patch_to_temp(&patch_a, "oxurack_test_umbrella_a.ron");
        oxa.load_patch(&path_a).expect("load A should succeed");
        let _ = std::fs::remove_file(&path_a);

        // Load patch B with different instance names.
        let mut patch_b = two_module_patch();
        patch_b.modules[0].instance_name = "gen_2".to_string();
        patch_b.modules[1].instance_name = "sink_2".to_string();
        patch_b.cables[0].source.0 = "gen_2".to_string();
        patch_b.cables[0].target.0 = "sink_2".to_string();
        let path_b = save_patch_to_temp(&patch_b, "oxurack_test_umbrella_b.ron");
        oxa.load_patch(&path_b).expect("load B should succeed");
        let _ = std::fs::remove_file(&path_b);

        let handle = oxa
            .world()
            .get_resource::<PatchHandle>()
            .expect("PatchHandle should exist");
        assert!(
            handle.modules.contains_key("gen_2"),
            "should have patch B's gen_2"
        );
        assert!(
            handle.modules.contains_key("sink_2"),
            "should have patch B's sink_2"
        );
        assert!(
            !handle.modules.contains_key("gen_1"),
            "should NOT have patch A's gen_1"
        );
    }

    #[test]
    fn test_install_then_apply_patch_spawns_entities() {
        let mut oxa = Oxurack::new();
        oxa.install::<TestGen>();
        oxa.install::<TestSink>();

        let path = save_patch_to_temp(&two_module_patch(), "oxurack_test_umbrella_spawn.ron");
        oxa.load_patch(&path).expect("load should succeed");
        let _ = std::fs::remove_file(&path);

        let handle = oxa.world().get_resource::<PatchHandle>().unwrap();
        let gen_entity = handle.modules["gen_1"];

        let module = oxa
            .world()
            .entity(gen_entity)
            .get::<oxurack_core::Module>()
            .expect("gen_1 entity should have Module component");
        assert_eq!(module.kind, oxurack_core::ModuleKind::from("test_gen"));
        assert_eq!(module.instance_name, "gen_1");
    }

    fn find_port_value(
        oxa: &mut Oxurack,
        module_entity: bevy_ecs::prelude::Entity,
        port_name: &str,
    ) -> oxurack_core::Value {
        let world = oxa.world_mut();
        let mut query = world.query::<(
            &oxurack_core::Port,
            &oxurack_core::CurrentValue,
            &bevy_ecs::hierarchy::ChildOf,
        )>();
        query
            .iter(world)
            .find_map(|(port, cv, child_of)| {
                if child_of.0 == module_entity && port.name.as_ref() == port_name {
                    Some(cv.0)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("port '{port_name}' not found on entity {module_entity:?}"))
    }

    fn set_port_value(
        oxa: &mut Oxurack,
        module_entity: bevy_ecs::prelude::Entity,
        port_name: &str,
        value: oxurack_core::Value,
    ) {
        let world = oxa.world_mut();
        let mut query = world.query::<(
            bevy_ecs::prelude::Entity,
            &oxurack_core::Port,
            &bevy_ecs::hierarchy::ChildOf,
        )>();
        let port_entity = query
            .iter(world)
            .find_map(|(e, port, child_of)| {
                if child_of.0 == module_entity && port.name.as_ref() == port_name {
                    Some(e)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("port '{port_name}' not found on entity {module_entity:?}"));
        world
            .entity_mut(port_entity)
            .get_mut::<oxurack_core::CurrentValue>()
            .unwrap()
            .0 = value;
    }

    #[test]
    fn test_propagation_runs_in_fixed_update_not_update() {
        let mut oxa = Oxurack::new();
        oxa.install::<TestGen>();
        oxa.install::<TestSink>();

        let path = save_patch_to_temp(&two_module_patch(), "oxurack_test_umbrella_tick.ron");
        oxa.load_patch(&path).expect("load should succeed");
        let _ = std::fs::remove_file(&path);

        let handle = oxa.world().get_resource::<PatchHandle>().unwrap().clone();
        let gen_entity = handle.modules["gen_1"];
        let sink_entity = handle.modules["sink_1"];

        set_port_value(
            &mut oxa,
            gen_entity,
            "out",
            oxurack_core::Value::Float(0.42),
        );

        // Running Update should NOT propagate — tick systems are in FixedUpdate.
        // Ensure the Update schedule exists (it is only auto-created by
        // app.update(), not by CorePlugin::fixed() which targets FixedUpdate).
        let world = oxa.world_mut();
        world.add_schedule(bevy_ecs::schedule::Schedule::new(bevy_app::prelude::Update));
        world.run_schedule(bevy_app::prelude::Update);
        let after_update = find_port_value(&mut oxa, sink_entity, "in");
        assert_eq!(
            after_update,
            oxurack_core::Value::Float(0.0),
            "Update should NOT propagate values — tick systems are in FixedUpdate"
        );

        // Running FixedUpdate SHOULD propagate.
        oxa.world_mut().run_schedule(bevy_app::prelude::FixedUpdate);
        let after_fixed = find_port_value(&mut oxa, sink_entity, "in");
        assert_eq!(
            after_fixed,
            oxurack_core::Value::Float(0.42),
            "FixedUpdate should propagate values through the cable"
        );
    }

    #[test]
    fn test_step_does_not_panic() {
        let mut oxa = Oxurack::new();
        oxa.step();
        oxa.step();
    }
}
