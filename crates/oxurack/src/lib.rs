//! Umbrella crate for the oxurack modular MIDI rack.
//!
//! Provides [`OxurackApp`], the top-level entry point that wires
//! `oxurack-core`'s ECS infrastructure into a runnable Bevy
//! application with fixed-timestep simulation, module registration,
//! and patch loading.

use std::path::Path;
use std::time::Duration;

use bevy_app::prelude::App;
use bevy_ecs::prelude::Resource;
use bevy_time::prelude::{Fixed, Time};

pub use oxurack_core;

/// Whether the simulation tick loop is running.
///
/// Module tick systems (clock, LFO, etc.) should early-return when
/// this is `false`. Defaults to `false` on construction.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationRunning(pub bool);

/// Top-level oxurack application.
///
/// Wraps a Bevy [`App`] configured with:
///
/// - [`CorePlugin::fixed()`](oxurack_core::CorePluginFixed) — tick
///   systems in [`FixedUpdate`](bevy_app::FixedUpdate) at 1000 Hz.
/// - [`Time<Fixed>`] period set to 1 ms.
/// - [`SimulationRunning`] resource (initially `false`).
///
/// Module crates register via [`register_module`](Self::register_module).
/// Patches are loaded via [`load_patch`](Self::load_patch), which stores
/// the resulting [`PatchHandle`](oxurack_core::PatchHandle) as a
/// resource for downstream entity lookups.
pub struct OxurackApp {
    app: App,
}

impl OxurackApp {
    /// Creates a new oxurack app with fixed-timestep simulation at 1000 Hz.
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

    /// Registers a module type in the [`ModuleRegistry`](oxurack_core::ModuleRegistry).
    ///
    /// After registration, patches referencing `M::KIND` can be loaded
    /// and their module instances will be spawned by `M::spawn`.
    ///
    /// # Note on parameter setters
    ///
    /// This method registers the module's metadata (kind, ports,
    /// parameter schemas, spawner) but does **not** register per-parameter
    /// setter functions in the [`ParameterRegistry`](oxurack_core::ParameterRegistry).
    /// The `OxurackModule` trait currently has no way to expose setter
    /// functions. Bucket 3 will address this gap when it closes the
    /// `SetParameter` command.
    pub fn register_module<M: oxurack_core::OxurackModule>(&mut self) -> &mut Self {
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

        // apply_patch_to_world validates internally and needs &mut World,
        // but also reads &ModuleRegistry from the same world. It handles
        // the borrow split internally by taking the registry reference
        // before mutating. We pass the registry as a separate parameter.
        //
        // Clone the registry to break the borrow overlap between the
        // immutable registry read and the mutable world access needed
        // for spawning.
        let registry = self
            .app
            .world()
            .get_resource::<oxurack_core::ModuleRegistry>()
            .expect("ModuleRegistry should be initialized by CorePlugin")
            .clone();

        let handle = oxurack_core::apply_patch_to_world(&patch, &registry, self.app.world_mut())?;

        self.app.world_mut().insert_resource(handle);
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

    /// Returns a mutable reference to the underlying Bevy [`App`].
    pub fn bevy_app_mut(&mut self) -> &mut App {
        &mut self.app
    }

    /// Returns a shared reference to the underlying Bevy [`App`].
    pub fn bevy_app(&self) -> &App {
        &self.app
    }
}

impl Default for OxurackApp {
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
        let oxa = OxurackApp::new();
        let sr = oxa
            .bevy_app()
            .world()
            .get_resource::<SimulationRunning>()
            .expect("SimulationRunning should be present");
        assert!(!sr.0, "SimulationRunning should default to false");
    }

    #[test]
    fn test_new_has_time_fixed_at_1ms() {
        let oxa = OxurackApp::new();
        let time = oxa
            .bevy_app()
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
        let oxa = OxurackApp::new();
        assert!(
            oxa.bevy_app()
                .world()
                .get_resource::<ModuleRegistry>()
                .is_some(),
            "ModuleRegistry should be present"
        );
    }

    #[test]
    fn test_register_module_populates_registry() {
        let mut oxa = OxurackApp::new();
        oxa.register_module::<TestGen>();

        let registry = oxa
            .bevy_app()
            .world()
            .get_resource::<ModuleRegistry>()
            .unwrap();
        assert!(
            registry.contains(&oxurack_core::ModuleKind::from("test_gen")),
            "test_gen should be registered"
        );
    }

    #[test]
    fn test_load_patch_stores_handle() {
        let mut oxa = OxurackApp::new();
        oxa.register_module::<TestGen>();
        oxa.register_module::<TestSink>();

        let path = save_patch_to_temp(&two_module_patch(), "oxurack_test_umbrella_load.ron");
        oxa.load_patch(&path).expect("load_patch should succeed");
        let _ = std::fs::remove_file(&path);

        let handle = oxa
            .bevy_app()
            .world()
            .get_resource::<PatchHandle>()
            .expect("PatchHandle should be stored after load_patch");
        assert_eq!(handle.modules.len(), 2);
        assert!(handle.modules.contains_key("gen_1"));
        assert!(handle.modules.contains_key("sink_1"));
    }

    #[test]
    fn test_load_patch_unregistered_kind_errors() {
        let mut oxa = OxurackApp::new();
        // Do NOT register any modules.

        let path = save_patch_to_temp(&two_module_patch(), "oxurack_test_umbrella_unreg.ron");
        let result = oxa.load_patch(&path);
        let _ = std::fs::remove_file(&path);

        assert!(result.is_err(), "should fail for unregistered module kind");

        assert!(
            oxa.bevy_app()
                .world()
                .get_resource::<PatchHandle>()
                .is_none(),
            "PatchHandle should not be inserted on error"
        );
    }

    #[test]
    fn test_load_patch_replaces_previous_handle() {
        let mut oxa = OxurackApp::new();
        oxa.register_module::<TestGen>();
        oxa.register_module::<TestSink>();

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
            .bevy_app()
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
    fn test_register_module_then_apply_patch_spawns_entities() {
        let mut oxa = OxurackApp::new();
        oxa.register_module::<TestGen>();
        oxa.register_module::<TestSink>();

        let path = save_patch_to_temp(&two_module_patch(), "oxurack_test_umbrella_spawn.ron");
        oxa.load_patch(&path).expect("load should succeed");
        let _ = std::fs::remove_file(&path);

        let handle = oxa
            .bevy_app()
            .world()
            .get_resource::<PatchHandle>()
            .unwrap();
        let gen_entity = handle.modules["gen_1"];

        let module = oxa
            .bevy_app()
            .world()
            .entity(gen_entity)
            .get::<oxurack_core::Module>()
            .expect("gen_1 entity should have Module component");
        assert_eq!(module.kind, oxurack_core::ModuleKind::from("test_gen"));
        assert_eq!(module.instance_name, "gen_1");
    }

    #[test]
    fn test_tick_systems_run_in_fixed_update() {
        let mut oxa = OxurackApp::new();
        oxa.register_module::<TestGen>();
        oxa.register_module::<TestSink>();

        let path = save_patch_to_temp(&two_module_patch(), "oxurack_test_umbrella_tick.ron");
        oxa.load_patch(&path).expect("load should succeed");
        let _ = std::fs::remove_file(&path);

        // Set gen_1's output port to a known value.
        let handle = oxa
            .bevy_app()
            .world()
            .get_resource::<PatchHandle>()
            .unwrap()
            .clone();
        let gen_entity = handle.modules["gen_1"];

        {
            let world = oxa.bevy_app_mut().world_mut();
            let mut query = world.query::<(
                bevy_ecs::prelude::Entity,
                &oxurack_core::Port,
                &bevy_ecs::hierarchy::ChildOf,
            )>();
            let out_port = query
                .iter(world)
                .find_map(|(e, port, child_of)| {
                    if child_of.0 == gen_entity && port.name.as_ref() == "out" {
                        Some(e)
                    } else {
                        None
                    }
                })
                .expect("gen_1 should have 'out' port");
            world
                .entity_mut(out_port)
                .get_mut::<oxurack_core::CurrentValue>()
                .unwrap()
                .0 = oxurack_core::Value::Float(0.42);
        }

        // The first update initialises the time system's epoch.
        // Subsequent updates advance Time<Real> from the wall clock,
        // which propagates to Time<Virtual> and triggers FixedUpdate
        // when enough time has accumulated past the 1 ms timestep.
        oxa.step();
        std::thread::sleep(Duration::from_millis(5));
        oxa.step();

        // Check that propagation happened: sink_1's input should have
        // the value from gen_1's output.
        let sink_entity = handle.modules["sink_1"];
        let world = oxa.bevy_app_mut().world_mut();
        let mut query = world.query::<(
            bevy_ecs::prelude::Entity,
            &oxurack_core::Port,
            &oxurack_core::CurrentValue,
            &bevy_ecs::hierarchy::ChildOf,
        )>();
        let sink_cv = query
            .iter(world)
            .find_map(|(_, port, cv, child_of)| {
                if child_of.0 == sink_entity && port.name.as_ref() == "in" {
                    Some(cv.0)
                } else {
                    None
                }
            })
            .expect("sink_1 should have 'in' port");
        assert_eq!(
            sink_cv,
            oxurack_core::Value::Float(0.42),
            "propagation should have carried the value through the cable"
        );
    }

    #[test]
    fn test_step_does_not_panic() {
        let mut oxa = OxurackApp::new();
        oxa.step();
        oxa.step();
    }
}
