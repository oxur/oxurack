//! Umbrella crate for the oxurack modular MIDI rack.
//!
//! [`Oxurack`] is the rack: install modules, load patches, set knobs,
//! power on, play. The Bevy ECS engine runs underneath but stays out
//! of your way unless you ask for it via [`Oxurack::into_app`].

use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

use bevy_app::prelude::{App, FixedUpdate};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_time::prelude::{Fixed, Real, Time, Virtual};

// ── Re-exports from oxurack-graph (feature-gated) ──────────────────
#[cfg(feature = "graph")]
pub use oxurack_graph;

// ── Re-exports from oxurack-core ────────────────────────────────────
//
// Curated public surface. Users import from `oxurack::`, not
// `oxurack::oxurack_core::`.

pub use oxurack_core::OxurackModule as Module;

pub use oxurack_core::{
    CoreError, CorePlugin, CorePluginFixed, PatchHandle, SampleTime, Scale, SimulationRunning,
};

pub use oxurack_core::{
    CableConfig, CableTransform, ModuleConfig, ParameterValue, Patch, Value, ValueKind,
};

pub use oxurack_core::{
    MergePolicy, ModuleId, ModuleKind, ParameterName, ParameterSchema, Port, PortDirection,
    PortName, PortSchema, TickPhase,
};

// spawn_module_entity, spawn_port_on_module, and CurrentValue are
// module-author internals. Module crates import them directly from
// oxurack_core, not from the user-facing oxurack surface.

/// The driver that owns sample execution for a rack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationDriver {
    /// Bevy's live fixed-time loop owns sample execution.
    Live,
    /// Explicit calls to [`Oxurack::advance_samples`] own sample execution.
    Manual,
}

/// The rack. Install modules, load patches, power on, play.
///
/// Internally wraps a Bevy [`App`] with fixed-timestep simulation
/// at 1000 Hz. Modules are installed via [`install`](Self::install),
/// patches loaded via [`load_patch`](Self::load_patch). The Bevy
/// engine is accessible via [`into_app`](Self::into_app) when needed.
pub struct Oxurack {
    app: App,
    installed_modules: HashMap<oxurack_core::ModuleKind, TypeId>,
    driver: SimulationDriver,
}

impl Oxurack {
    /// Build a new rack with fixed-timestep simulation at 1000 Hz.
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_plugins(bevy_time::TimePlugin);
        app.add_plugins(oxurack_core::CorePlugin::fixed());

        #[cfg(feature = "graph")]
        app.add_plugins(oxurack_graph::RackGraphPlugin);

        app.world_mut()
            .resource_mut::<Time<Fixed>>()
            .set_timestep(Duration::from_millis(1));

        app.world_mut().insert_resource(SimulationRunning(false));
        app.world_mut().resource_mut::<Time<Virtual>>().pause();
        app.world_mut()
            .resource_mut::<Time<Fixed>>()
            .discard_overstep(Duration::MAX);

        Self {
            app,
            installed_modules: HashMap::new(),
            driver: SimulationDriver::Live,
        }
    }

    /// Install a module type into the rack.
    ///
    /// After installation, patches referencing `M::KIND` can be loaded
    /// and their module instances will be spawned by `M::spawn`.
    ///
    /// # Panics
    ///
    /// Panics if a different Rust module type has already claimed `M::KIND`.
    /// Use [`try_install`](Self::try_install) when the module type is selected
    /// from fallible or user-provided input.
    pub fn install<M: oxurack_core::OxurackModule>(&mut self) -> &mut Self {
        if let Err(error) = self.try_install::<M>() {
            panic!("cannot install module kind '{}': {error}", M::KIND);
        }

        self
    }

    /// Try to install a module type into the rack.
    ///
    /// A module kind is registered once per rack. Reinstalling the same Rust
    /// type is idempotent, while a different type claiming an installed kind
    /// is rejected before the rack's registries or schedules are changed.
    /// Successful installation invokes
    /// [`oxurack_core::OxurackModule::register_systems`]
    /// once with [`FixedUpdate`]. A panic from that user-provided hook is not
    /// caught and does not promise rollback of earlier registration changes.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::ModuleKindConflict`]
    /// when a different Rust type already owns `M::KIND` in this rack.
    pub fn try_install<M: oxurack_core::OxurackModule>(
        &mut self,
    ) -> Result<&mut Self, oxurack_core::CoreError> {
        let kind = oxurack_core::ModuleKind::from(M::KIND);
        let type_id = TypeId::of::<M>();

        match self.installed_modules.get(&kind) {
            Some(installed) if *installed == type_id => return Ok(self),
            Some(_) => {
                return Err(oxurack_core::CoreError::ModuleKindConflict {
                    kind: M::KIND.to_owned(),
                });
            }
            None => {}
        }

        {
            let world = self.app.world_mut();
            world
                .resource_mut::<oxurack_core::ModuleRegistry>()
                .register::<M>();

            let mut parameter_registry = world.resource_mut::<oxurack_core::ParameterRegistry>();
            for &(name, setter) in M::parameter_setters() {
                parameter_registry.register(kind.clone(), name, setter);
            }
            for &(name, getter) in M::parameter_getters() {
                parameter_registry.register_getter(kind.clone(), name, getter);
            }
        }

        M::register_systems(&mut self.app, FixedUpdate.intern());
        self.installed_modules.insert(kind, type_id);
        Ok(self)
    }

    /// Access the rack's graph view for navigation queries.
    ///
    /// Returns a [`GraphView`](oxurack_graph::GraphView) over the rack's
    /// ECS world. All query methods resolve labels fresh at call time.
    ///
    /// Available only with the `graph` Cargo feature (enabled by default).
    #[cfg(feature = "graph")]
    pub fn graph(&self) -> oxurack_graph::GraphView<'_> {
        oxurack_graph::GraphView::new(self.app.world())
    }

    /// Loads a patch from a RON file and applies it to the world.
    ///
    /// On success, the resulting [`PatchHandle`]
    /// is stored as a resource. If a handle from a previous load exists,
    /// it is replaced.
    ///
    /// When the `graph` feature is enabled, the `GraphMaintenance`
    /// schedule runs synchronously after the patch is applied, so
    /// `rack.graph().modules()` reflects the just-loaded patch immediately.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError`] on I/O, parse,
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
                let handle = oxurack_core::apply_patch_to_world(
                    &patch,
                    &registry,
                    world,
                    oxurack_core::PatchSource::File(path.to_path_buf()),
                )?;
                world.insert_resource(handle);
                Ok::<(), oxurack_core::CoreError>(())
            })?;

        // Freshness contract: run graph maintenance synchronously so
        // rack.graph().modules() reflects the just-loaded patch.
        #[cfg(feature = "graph")]
        self.app
            .world_mut()
            .run_schedule(oxurack_graph::GraphMaintenance);

        Ok(())
    }

    /// Returns the driver currently responsible for sample execution.
    pub fn driver_mode(&self) -> SimulationDriver {
        self.driver
    }

    /// Selects the driver responsible for future samples.
    ///
    /// Switching to manual mode pauses virtual time and discards pending
    /// fixed overstep. Switching back to live mode rebases real time while
    /// virtual time is paused, then resumes it only when the rack is running.
    pub fn set_driver_mode(&mut self, driver: SimulationDriver) -> &mut Self {
        if self.driver == driver {
            if driver == SimulationDriver::Live {
                self.rebase_live_time();
            }
            return self;
        }

        self.discard_fixed_overstep();
        self.app.world_mut().resource_mut::<Time<Virtual>>().pause();

        if driver == SimulationDriver::Live {
            self.rebase_live_time();
            if self.is_running() {
                self.app
                    .world_mut()
                    .resource_mut::<Time<Virtual>>()
                    .unpause();
            }
        }

        self.driver = driver;
        self
    }

    /// Starts the rack without resetting module state or sample time.
    pub fn start(&mut self) -> &mut Self {
        self.app.world_mut().resource_mut::<SimulationRunning>().0 = true;

        self.rebase_live_time();
        if self.driver == SimulationDriver::Live {
            self.app
                .world_mut()
                .resource_mut::<Time<Virtual>>()
                .unpause();
        }
        self
    }

    /// Stops the rack without resetting module state or sample time.
    pub fn stop(&mut self) -> &mut Self {
        self.app.world_mut().resource_mut::<SimulationRunning>().0 = false;
        self.app.world_mut().resource_mut::<Time<Virtual>>().pause();
        self.discard_fixed_overstep();
        self.rebase_live_time();
        self
    }

    /// Returns a copy of the rack's authoritative sample clock.
    pub fn sample_time(&self) -> SampleTime {
        *self.app.world().resource::<SampleTime>()
    }

    /// Advances exactly `count` samples under the manual driver.
    ///
    /// Returns zero without changing state when the rack is stopped or the
    /// requested count is zero. A live rack returns
    /// [`CoreError::ManualSampleRequiresManualDriver`] without mutation.
    pub fn advance_samples(&mut self, count: u64) -> Result<u64, CoreError> {
        if self.driver != SimulationDriver::Manual {
            return Err(CoreError::ManualSampleRequiresManualDriver);
        }
        if count == 0 || !self.is_running() {
            return Ok(0);
        }

        for _ in 0..count {
            let world = self.app.world_mut();
            world
                .resource_mut::<Time<Virtual>>()
                .advance_by(SampleTime::period());
            world
                .resource_mut::<Time<Fixed>>()
                .advance_by(SampleTime::period());

            let fixed_time = world.resource::<Time<Fixed>>().as_generic();
            *world.resource_mut::<Time>() = fixed_time;
            world.run_schedule(FixedUpdate);

            let virtual_time = world.resource::<Time<Virtual>>().as_generic();
            *world.resource_mut::<Time>() = virtual_time;
        }

        Ok(count)
    }

    fn is_running(&self) -> bool {
        self.app.world().resource::<SimulationRunning>().0
    }

    fn discard_fixed_overstep(&mut self) {
        self.app
            .world_mut()
            .resource_mut::<Time<Fixed>>()
            .discard_overstep(Duration::MAX);
    }

    fn rebase_live_time(&mut self) {
        self.app
            .world_mut()
            .resource_mut::<Time<Real>>()
            .update_with_instant(Instant::now());
    }

    /// Runs a single app update cycle.
    ///
    /// In fixed-timestep mode, this advances `Time<Fixed>` and may
    /// trigger zero or more fixed-update steps depending on elapsed
    /// time.
    pub fn step(&mut self) {
        self.app.update();
    }

    /// Consume the rack and return the underlying Bevy [`App`].
    ///
    /// The caller takes responsibility for preserving the shared
    /// [`SimulationRunning`] state and named [`TickPhase`] membership after
    /// ownership transfers to Bevy.
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
    use bevy_ecs::prelude::{Component, Entity, Query, ResMut, Resource, With};
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use oxurack_core::{
        CableConfig, CoreError, MergePolicy, ModuleConfig, ModuleRegistry, OxurackModule,
        ParameterGetter, ParameterRegistry, ParameterSchema, ParameterSetter, ParameterValue,
        PatchHandle, PortDirection, PortSchema, ValueKind,
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

    fn save_patch_to_temp(patch: &oxurack_core::Patch, name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(name);
        oxurack_core::save_patch_to_file(patch, &path).expect("save should succeed");
        path
    }

    #[derive(Component, Clone, Debug, PartialEq, Eq)]
    struct LiveStepState {
        increment: i64,
        total: i64,
    }

    #[derive(Component)]
    struct LiveCounter;

    #[derive(Resource, Default)]
    struct LiveHookCalls(usize);

    #[derive(Resource, Default)]
    struct LiveSystemRuns(usize);

    struct LiveBehaviorCounter;

    fn live_invalid_parameter(reason: &str) -> CoreError {
        CoreError::InvalidParameterValue {
            module: "behavior_counter".to_owned(),
            param: "increment".to_owned(),
            reason: reason.to_owned(),
        }
    }

    fn live_set_increment(
        world: &mut bevy_ecs::world::World,
        entity: Entity,
        value: ParameterValue,
    ) -> Result<(), CoreError> {
        let ParameterValue::Int(increment) = value else {
            return Err(live_invalid_parameter("expected an integer"));
        };
        world
            .entity_mut(entity)
            .get_mut::<LiveStepState>()
            .ok_or_else(|| live_invalid_parameter("module has no live step state"))?
            .increment = increment;
        Ok(())
    }

    fn live_get_increment(
        world: &bevy_ecs::world::World,
        entity: Entity,
    ) -> Result<ParameterValue, CoreError> {
        world
            .entity(entity)
            .get::<LiveStepState>()
            .map(|state| ParameterValue::Int(state.increment))
            .ok_or_else(|| live_invalid_parameter("module has no live step state"))
    }

    fn live_behavior_system(
        mut system_runs: ResMut<LiveSystemRuns>,
        mut query: Query<&mut LiveStepState, With<LiveCounter>>,
    ) {
        system_runs.0 += 1;
        for mut state in &mut query {
            state.total += state.increment;
        }
    }

    impl OxurackModule for LiveBehaviorCounter {
        const KIND: &'static str = "behavior_counter";
        const DISPLAY_NAME: &'static str = "Live Behavior Counter";
        const DESCRIPTION: &'static str = "A unit-test module for live reinstall behavior.";

        fn port_schema() -> &'static [PortSchema] {
            &[]
        }

        fn parameter_schema() -> &'static [ParameterSchema] {
            &[ParameterSchema {
                name: "increment",
                description: "Amount added on every fixed tick.",
                default: ParameterValue::Int(1),
            }]
        }

        fn spawn(
            world: &mut bevy_ecs::world::World,
            instance_name: &str,
            parameters: &HashMap<String, ParameterValue>,
        ) -> Result<Entity, CoreError> {
            let increment = match parameters.get("increment") {
                Some(ParameterValue::Int(increment)) => *increment,
                None => 1,
                Some(_) => return Err(live_invalid_parameter("expected an integer")),
            };
            let entity = oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);
            world.entity_mut(entity).insert((
                LiveCounter,
                LiveStepState {
                    increment,
                    total: 0,
                },
            ));
            Ok(entity)
        }

        fn register_systems(app: &mut App, schedule: bevy_ecs::schedule::InternedScheduleLabel) {
            app.init_resource::<LiveHookCalls>()
                .init_resource::<LiveSystemRuns>()
                .add_systems(schedule, live_behavior_system.in_set(TickPhase::Produce));
            app.world_mut().resource_mut::<LiveHookCalls>().0 += 1;
        }

        fn parameter_setters() -> &'static [(&'static str, ParameterSetter)] {
            &[("increment", live_set_increment)]
        }

        fn parameter_getters() -> &'static [(&'static str, ParameterGetter)] {
            &[("increment", live_get_increment)]
        }
    }

    fn live_state(rack: &Oxurack, handle: &PatchHandle, name: &str) -> LiveStepState {
        rack.world()
            .entity(handle.modules[name])
            .get::<LiveStepState>()
            .unwrap_or_else(|| panic!("{name} should have LiveStepState"))
            .clone()
    }

    fn live_parameter(rack: &mut Oxurack, entity: Entity) -> ParameterValue {
        let kind = ModuleKind::from(LiveBehaviorCounter::KIND);
        rack.world_mut()
            .resource_scope::<ParameterRegistry, _>(|world, registry| {
                registry
                    .get_parameter(world, entity, &kind, "increment")
                    .expect("live getter should remain registered")
            })
    }

    #[test]
    fn test_reinstall_after_parameter_edit_preserves_loaded_state() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/behavior-installation.ron");
        let mut rack = Oxurack::new();
        rack.install::<LiveBehaviorCounter>();
        rack.load_patch(&fixture)
            .expect("behavior fixture should load through the public path");
        rack.start();

        let handle = rack.world().resource::<PatchHandle>().clone();
        let alpha = handle.modules["alpha"];
        let beta = handle.modules["beta"];
        assert_eq!(live_state(&rack, &handle, "alpha").total, 0);
        assert_eq!(live_state(&rack, &handle, "beta").total, 0);
        assert_eq!(live_state(&rack, &handle, "alpha").increment, 2);
        assert_eq!(live_state(&rack, &handle, "beta").increment, 3);
        assert_eq!(rack.world().resource::<LiveHookCalls>().0, 1);

        rack.world_mut().run_schedule(FixedUpdate);
        rack.world_mut().run_schedule(FixedUpdate);
        assert_eq!(live_state(&rack, &handle, "alpha").total, 4);
        assert_eq!(live_state(&rack, &handle, "beta").total, 6);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 2);

        let kind = ModuleKind::from(LiveBehaviorCounter::KIND);
        {
            let world = rack.world_mut();
            world.resource_scope::<ParameterRegistry, _>(|world, registry| {
                registry
                    .set_parameter(world, alpha, &kind, "increment", ParameterValue::Int(4))
                    .expect("live setter should remain registered");
                assert_eq!(
                    registry
                        .get_parameter(world, alpha, &kind, "increment")
                        .expect("alpha getter should remain registered"),
                    ParameterValue::Int(4)
                );
                assert_eq!(
                    registry
                        .get_parameter(world, beta, &kind, "increment")
                        .expect("beta getter should remain registered"),
                    ParameterValue::Int(3)
                );
            });
        }

        rack.try_install::<LiveBehaviorCounter>()
            .expect("same-type reinstall should be idempotent after live edits");
        assert_eq!(live_state(&rack, &handle, "alpha").total, 4);
        assert_eq!(live_state(&rack, &handle, "beta").total, 6);
        assert_eq!(live_parameter(&mut rack, alpha), ParameterValue::Int(4));
        assert_eq!(live_parameter(&mut rack, beta), ParameterValue::Int(3));
        assert_eq!(rack.world().resource::<LiveHookCalls>().0, 1);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 2);

        rack.world_mut().run_schedule(FixedUpdate);
        assert_eq!(live_state(&rack, &handle, "alpha").total, 8);
        assert_eq!(live_state(&rack, &handle, "beta").total, 9);
        assert_eq!(rack.world().resource::<LiveHookCalls>().0, 1);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 3);
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
    fn test_driver_transitions_discard_overstep_without_catch_up() {
        let mut rack = Oxurack::new();
        rack.start();
        rack.world_mut()
            .resource_mut::<Time<Fixed>>()
            .accumulate_overstep(Duration::from_millis(5));

        rack.set_driver_mode(SimulationDriver::Manual);
        assert_eq!(rack.driver_mode(), SimulationDriver::Manual);
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().overstep(),
            Duration::ZERO
        );
        assert!(rack.world().resource::<Time<Virtual>>().is_paused());

        assert_eq!(
            rack.advance_samples(1).expect("manual sample should run"),
            1
        );
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert_eq!(
            rack.world().resource::<Time<Virtual>>().elapsed(),
            Duration::from_millis(1)
        );
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().elapsed(),
            Duration::from_millis(1)
        );

        rack.set_driver_mode(SimulationDriver::Live);
        assert_eq!(rack.driver_mode(), SimulationDriver::Live);
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().overstep(),
            Duration::ZERO
        );
        assert!(!rack.world().resource::<Time<Virtual>>().is_paused());

        rack.world_mut()
            .resource_mut::<Time<Fixed>>()
            .accumulate_overstep(Duration::from_millis(5));
        rack.stop();
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().overstep(),
            Duration::ZERO
        );
        assert!(rack.world().resource::<Time<Virtual>>().is_paused());
        rack.start();
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().overstep(),
            Duration::ZERO
        );
        assert!(!rack.world().resource::<Time<Virtual>>().is_paused());
        assert_eq!(rack.sample_time().completed_samples(), 1);
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
        oxa.start();

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

    // ── Graph tests (feature-gated) ───────────────────────────────

    #[cfg(feature = "graph")]
    #[test]
    fn test_graph_accessor_returns_view() {
        let mut oxa = Oxurack::new();
        oxa.install::<TestGen>();
        oxa.install::<TestSink>();

        let path = save_patch_to_temp(&two_module_patch(), "oxurack_test_graph_view.ron");
        oxa.load_patch(&path).expect("load should succeed");
        let _ = std::fs::remove_file(&path);

        let view = oxa.graph();
        assert_eq!(view.module_count(), 2);
        assert!(view.module_by_name("gen_1").is_some());
        assert!(view.module_by_name("sink_1").is_some());
    }

    #[cfg(feature = "graph")]
    #[test]
    fn test_load_patch_makes_graph_current_synchronously() {
        let mut oxa = Oxurack::new();
        oxa.install::<TestGen>();
        oxa.install::<TestSink>();

        let path = save_patch_to_temp(&two_module_patch(), "oxurack_test_graph_freshness.ron");
        oxa.load_patch(&path).expect("load should succeed");
        let _ = std::fs::remove_file(&path);

        // Freshness contract: the graph is current immediately after
        // load_patch returns — no app.update() needed. This works
        // because load_patch runs GraphMaintenance synchronously.
        let graph = oxa.world().resource::<oxurack_graph::RackGraph>();
        assert_eq!(
            graph.module_count(),
            2,
            "graph should have 2 modules immediately after load_patch (no update needed)"
        );
        assert_eq!(
            graph.cable_count(),
            1,
            "graph should have 1 cable immediately after load_patch (no update needed)"
        );

        // Also verify via the GraphView query surface.
        let modules = oxa.graph().modules();
        assert_eq!(modules.len(), 2);
    }
}
