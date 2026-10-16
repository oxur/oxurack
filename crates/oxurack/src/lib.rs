//! Umbrella crate for the oxurack modular MIDI rack.
//!
//! [`Oxurack`] is the rack: install modules, load patches, set knobs,
//! power on, play. The Bevy ECS engine runs underneath but stays out
//! of your way unless you ask for it via [`Oxurack::into_app`].

use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use bevy_app::prelude::{App, FixedUpdate};
use bevy_ecs::schedule::ScheduleLabel;
#[cfg(test)]
use bevy_time::TimeUpdateStrategy;
use bevy_time::prelude::{Fixed, Real, Time, Virtual};

mod cables;
mod commands;
mod handles;
mod labels;
mod lifecycle;
mod observation;
mod persistence;
mod queries;
mod scope;
pub use commands::{CommandId, CommandOutcome, CommandResult};
pub use handles::{Handle, KnobRef, ModuleRef, PortRef};
pub use observation::{
    CableInfo, LifetimeId, Observation, ObservationBatchId, ObservationChange, ObservationCounter,
    ObservationCursor, ObservationError, ObservationGap, ObservationPage, ObservationSequence,
    ObservationStart, ObservationTarget, PatchInfo, PatchOrigin, TagMatch, TagQueryError,
    UnloadReason,
};
pub use scope::{MidiDelivery, ScopeCapture, ScopeError, ScopeRecord, ScopeSelector};

// ── Re-exports from oxurack-graph (feature-gated) ──────────────────
#[cfg(feature = "graph")]
pub use oxurack_graph;

// ── Re-exports from oxurack-core ────────────────────────────────────
//
// Curated public surface. Users import from `oxurack::`, not
// `oxurack::oxurack_core::`.

pub use oxurack_core::OxurackModule as Module;

pub use oxurack_core::{
    ChangeSource, CoreCommand, CoreError, CorePlugin, CorePluginFixed, LabelKind, LabelTarget,
    LabelValue, ModuleSpawnContext, PatchHandle, SampleStart, SampleTime, Scale, SimulationRunning,
    Tag, TagError, TickError,
};

pub use oxurack_mod_midiout::{MidiOutModule, MidiOutputBuffer, MidiOutputEvent};

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
    installed_modules: HashMap<oxurack_core::ModuleKind, usize>,
    install_order: Vec<InstalledType>,
    driver: SimulationDriver,
    loaded_identity: Option<persistence::LoadedIdentity>,
    lifetime_token: Arc<()>,
    instance_tokens: HashMap<String, Arc<()>>,
    command_queue: commands::CommandQueue,
    observation: observation::Journal,
}

struct InstalledType {
    kind: oxurack_core::ModuleKind,
    type_id: TypeId,
    type_name: &'static str,
    install_into: fn(&mut App),
}

fn install_type<M: oxurack_core::OxurackModule>(app: &mut App) {
    let kind = oxurack_core::ModuleKind::from(M::KIND);
    {
        let world = app.world_mut();
        world
            .resource_mut::<oxurack_core::ModuleRegistry>()
            .register::<M>();
        let mut parameters = world.resource_mut::<oxurack_core::ParameterRegistry>();
        for &(name, setter) in M::parameter_setters() {
            parameters.register(kind.clone(), name, setter);
        }
        for &(name, getter) in M::parameter_getters() {
            parameters.register_getter(kind.clone(), name, getter);
        }
    }
    M::register_systems(app, FixedUpdate.intern());
}

fn build_base_app() -> Result<App, CoreError> {
    let mut app = App::new();
    app.add_plugins(bevy_time::TimePlugin);
    app.add_plugins(oxurack_core::CorePlugin::fixed());
    app.init_resource::<oxurack_core::ObservationStaging>();
    app.insert_resource(MidiOutputBuffer::try_new()?);
    scope::install_observer(&mut app);
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
    Ok(app)
}

impl Oxurack {
    /// Build a new rack with fixed-timestep simulation at 1000 Hz.
    ///
    /// # Panics
    ///
    /// Panics if the fixed MIDI event storage reservation fails.
    pub fn new() -> Self {
        let app = build_base_app().expect("cannot reserve fixed MIDI event storage for rack");

        Self {
            app,
            installed_modules: HashMap::new(),
            install_order: Vec::new(),
            driver: SimulationDriver::Live,
            loaded_identity: None,
            lifetime_token: Arc::new(()),
            instance_tokens: HashMap::new(),
            command_queue: commands::CommandQueue::new(),
            observation: observation::Journal::default(),
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
    /// A module kind is registered once per App lifetime. Reinstalling the same Rust
    /// type is idempotent, while a different type claiming an installed kind
    /// is rejected before the rack's registries, schedules, or replay recipe change.
    /// Successful installation invokes
    /// [`oxurack_core::OxurackModule::register_systems`]
    /// once with [`FixedUpdate`]. The hook runs against this supplied App and is
    /// replayed once into each new App lifetime. A panic, global side effect, or
    /// change to a non-owned handle is not caught and has no rollback guarantee.
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
            Some(index) if self.install_order[*index].type_id == type_id => return Ok(self),
            Some(_) => {
                return Err(oxurack_core::CoreError::ModuleKindConflict {
                    kind: M::KIND.to_owned(),
                });
            }
            None => {}
        }

        install_type::<M>(&mut self.app);
        let index = self.install_order.len();
        self.install_order.push(InstalledType {
            kind: kind.clone(),
            type_id,
            type_name: std::any::type_name::<M>(),
            install_into: install_type::<M>,
        });
        self.installed_modules.insert(kind, index);
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

    /// Loads a patch from a RON file into a fresh rack lifetime.
    ///
    /// When the `graph` feature is enabled, the `GraphMaintenance`
    /// schedule runs synchronously after the patch is applied, so
    /// `rack.graph().modules()` reflects the just-loaded patch immediately.
    ///
    /// The rack must be stopped and its MIDI output drained before file I/O.
    /// A failed candidate leaves the current lifetime unchanged. Hooks and
    /// spawners may have external side effects or panic; those are not rolled back.
    ///
    /// # Errors
    ///
    /// Returns lifecycle admission errors before I/O, then I/O, validation,
    /// candidate output, or spawn errors. Candidate MIDI output is rejected.
    pub fn load_patch(&mut self, path: &Path) -> Result<(), oxurack_core::CoreError> {
        lifecycle::load_patch(self, path)
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
        let previous = self.app.world().resource::<SimulationRunning>().0;
        self.app.world_mut().resource_mut::<SimulationRunning>().0 = true;
        if !previous {
            self.observation.publish(
                self.observation.lifetime,
                self.sample_time().completed_samples(),
                vec![observation::ObservationChange::RunningChanged {
                    previous,
                    running: true,
                }],
            );
        }

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
        let previous = self.app.world().resource::<SimulationRunning>().0;
        self.app.world_mut().resource_mut::<SimulationRunning>().0 = false;
        if previous {
            self.observation.publish(
                self.observation.lifetime,
                self.sample_time().completed_samples(),
                vec![observation::ObservationChange::RunningChanged {
                    previous,
                    running: false,
                }],
            );
        }
        self.app.world_mut().resource_mut::<Time<Virtual>>().pause();
        self.discard_fixed_overstep();
        self.rebase_live_time();
        self
    }

    /// Returns a copy of the rack's authoritative sample clock.
    pub fn sample_time(&self) -> SampleTime {
        *self.app.world().resource::<SampleTime>()
    }

    /// Validates and enables bounded capture while the rack is stopped.
    ///
    /// Capacity counts mixed signal and MIDI records. The selector slice may
    /// be empty for MIDI-only capture. A failed configuration leaves any
    /// existing capture, records, and overwrite count unchanged.
    ///
    /// # Errors
    ///
    /// Returns [`ScopeError`] for running state, invalid capacity, missing
    /// patch/selector, unsupported port kind/state, or allocation failure.
    pub fn configure_scope(
        &mut self,
        selectors: &[ScopeSelector],
        capacity: usize,
    ) -> Result<&mut Self, ScopeError> {
        scope::configure(self.app.world_mut(), selectors, capacity)?;
        Ok(self)
    }

    /// Returns the currently enabled Scope capture, if any.
    #[must_use]
    pub fn scope(&self) -> Option<&ScopeCapture> {
        self.app.world().get_resource::<ScopeCapture>()
    }

    /// Clears retained records and the overwrite count while keeping capture enabled.
    pub fn clear_scope(&mut self) -> &mut Self {
        scope::clear(self.app.world_mut());
        self
    }

    /// Removes Scope capture. Calling this when capture is disabled is a no-op.
    pub fn disable_scope(&mut self) -> &mut Self {
        scope::disable(self.app.world_mut());
        self
    }

    /// Copies an event explicitly drained by the caller into Scope, if enabled.
    ///
    /// Call only after the borrow returned by [`Self::drain_midi`] has ended.
    pub fn record_scope_midi(&mut self, event: MidiOutputEvent, delivery: MidiDelivery) {
        scope::record_midi(self.app.world_mut(), event, delivery);
    }

    /// Drains MIDI events produced by completed samples, in sink/message order.
    ///
    /// The returned borrow must be consumed or dropped before advancing again.
    /// Dropping it early discards remaining events. Draining does not clear a
    /// terminal output fault; a successful whole-App replacement clears it.
    /// Carried cleanup may retain a sample timestamp from the old lifetime.
    pub fn drain_midi(&mut self) -> std::vec::Drain<'_, MidiOutputEvent> {
        self.app
            .world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .into_inner()
            .drain()
    }

    /// Returns the first terminal MIDI output fault, if one has been latched.
    #[must_use]
    pub fn midi_output_error(&self) -> Option<&TickError> {
        self.app.world().resource::<MidiOutputBuffer>().fault()
    }

    /// Advances up to `count` complete samples under the manual driver.
    ///
    /// The return value is the number of samples that completed. A shared stop
    /// after a sample begins lets that latched sample finish, then prevents the
    /// next sample from starting. Drain all MIDI events before requesting
    /// another sample: this facade applies a conservative single-slot admission
    /// rule even when the next sample might be idle. A stopped rack or
    /// zero-count request returns zero without changing state. A live rack returns
    /// [`CoreError::ManualSampleRequiresManualDriver`] before mutation.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::ManualSampleRequiresManualDriver`] in live mode, or
    /// [`CoreError::Tick`] for output backpressure or a latched MIDI output fault.
    /// A tick error reports the number of samples completed by this call;
    /// a fault found during Consume includes that finalized sample.
    pub fn advance_samples(&mut self, count: u64) -> Result<u64, CoreError> {
        if self.driver != SimulationDriver::Manual {
            return Err(CoreError::ManualSampleRequiresManualDriver);
        }
        if count == 0 {
            return Ok(0);
        }

        let request_start = self.sample_time().completed_samples();
        for _ in 0..count {
            if !self.is_running() {
                break;
            }

            if let Err(error) = self
                .app
                .world()
                .resource::<MidiOutputBuffer>()
                .check_admission()
            {
                {
                    let mut buffer = self.app.world_mut().resource_mut::<MidiOutputBuffer>();
                    buffer.latch_fault(error.clone());
                }
                self.app.world_mut().resource_mut::<SimulationRunning>().0 = false;
                return Err(CoreError::Tick(with_completed_samples(
                    error,
                    self.sample_time()
                        .completed_samples()
                        .saturating_sub(request_start),
                )));
            }

            let before = self.sample_time().completed_samples();

            {
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

            let after = self.sample_time().completed_samples();
            if let Some(error) = self.midi_output_error().cloned() {
                return Err(CoreError::Tick(with_completed_samples(
                    error,
                    after.saturating_sub(request_start),
                )));
            }
            debug_assert!(after >= before);
        }

        Ok(self
            .sample_time()
            .completed_samples()
            .saturating_sub(request_start))
    }

    /// Returns whether this rack is currently advancing samples.
    ///
    /// The result reads the rack's authoritative running state; it is not a
    /// cached playback flag and this call does not mutate the rack.
    #[must_use]
    pub fn is_running(&self) -> bool {
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
    /// ownership transfers to Bevy. Facade command slots and their results are
    /// not stored in the App; process them before consuming the rack or they
    /// are abandoned with the facade.
    pub fn into_app(self) -> App {
        // Queued facade commands and their results are intentionally not a
        // Bevy resource. Collect results before transferring App ownership.
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

fn with_completed_samples(error: TickError, completed_samples: u64) -> TickError {
    match error {
        TickError::OutputBackpressure {
            pending_messages, ..
        } => TickError::OutputBackpressure {
            pending_messages,
            completed_samples,
        },
        TickError::InvalidMidiOutput {
            module,
            port,
            reason,
            ..
        } => TickError::InvalidMidiOutput {
            module,
            port,
            reason,
            completed_samples,
        },
        TickError::OutputCapacity { reason, .. } => TickError::OutputCapacity {
            reason,
            completed_samples,
        },
        other => other,
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
    use bevy_ecs::hierarchy::Children;
    use bevy_ecs::prelude::{Component, Entity, Query, Res, ResMut, Resource, With};
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use oxurack_core::{
        CableConfig, CoreError, CurrentValue, MergePolicy, ModuleConfig, ModuleRegistry,
        ModuleSpawnContext, OxurackModule, ParameterGetter, ParameterRegistry, ParameterSchema,
        ParameterSetter, ParameterValue, PatchHandle, PortDirection, PortSchema, ValueKind,
    };
    use oxurack_mod_clock::ClockModule;
    use oxurack_mod_lfo::LfoModule;
    use oxurack_mod_turingmachine::TuringMachineModule;
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

    struct FaultLateFailure;

    #[derive(Resource)]
    struct FaultCandidateMarker;

    impl OxurackModule for FaultLateFailure {
        const KIND: &'static str = "fault_late_failure";
        const DISPLAY_NAME: &'static str = "Fault late failure";

        fn port_schema() -> &'static [PortSchema] {
            &[]
        }

        fn parameter_schema() -> &'static [ParameterSchema] {
            &[]
        }

        fn spawn(
            world: &mut bevy_ecs::world::World,
            instance_name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<Entity, CoreError> {
            world.insert_resource(FaultCandidateMarker);
            let entity = world.spawn_empty().id();
            world.write_message(oxurack_core::ModuleAdded {
                entity,
                instance_name: instance_name.to_owned(),
                kind: Self::KIND.into(),
            });
            Err(CoreError::NotImplemented("fault candidate late failure"))
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
    struct FirstRecipeHook;
    struct SecondRecipeHook;
    struct CollidingRecipeHook;

    #[derive(Resource, Default)]
    struct RecipeHookOrder(Vec<u8>);

    #[derive(Resource, Default)]
    struct RecipeHookRuns(usize);

    fn recipe_hook_probe(mut runs: ResMut<RecipeHookRuns>) {
        runs.0 += 1;
    }

    macro_rules! recipe_hook_module {
        ($module:ty, $kind:literal, $order:literal) => {
            impl OxurackModule for $module {
                const KIND: &'static str = $kind;
                const DISPLAY_NAME: &'static str = $kind;

                fn port_schema() -> &'static [PortSchema] {
                    &[]
                }

                fn parameter_schema() -> &'static [ParameterSchema] {
                    &[]
                }

                fn spawn(
                    world: &mut bevy_ecs::world::World,
                    _instance_name: &str,
                    _parameters: &HashMap<String, ParameterValue>,
                ) -> Result<Entity, CoreError> {
                    Ok(world.spawn_empty().id())
                }

                fn register_systems(
                    app: &mut App,
                    schedule: bevy_ecs::schedule::InternedScheduleLabel,
                ) {
                    app.init_resource::<RecipeHookOrder>()
                        .init_resource::<RecipeHookRuns>();
                    app.world_mut()
                        .resource_mut::<RecipeHookOrder>()
                        .0
                        .push($order);
                    app.add_systems(schedule, recipe_hook_probe.in_set(TickPhase::Observe));
                }
            }
        };
    }

    recipe_hook_module!(FirstRecipeHook, "recipe_first", 1);
    recipe_hook_module!(SecondRecipeHook, "recipe_second", 2);

    impl OxurackModule for CollidingRecipeHook {
        const KIND: &'static str = "recipe_first";
        const DISPLAY_NAME: &'static str = "Colliding recipe hook";

        fn port_schema() -> &'static [PortSchema] {
            &[]
        }

        fn parameter_schema() -> &'static [ParameterSchema] {
            &[]
        }

        fn spawn(
            _world: &mut bevy_ecs::world::World,
            _instance_name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<Entity, CoreError> {
            Err(CoreError::NotImplemented("collision spawner must not run"))
        }
    }

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

    fn module_identities(world: &bevy_ecs::world::World) -> Vec<(String, u64)> {
        let mut identities = world
            .resource::<PatchHandle>()
            .modules
            .iter()
            .map(|(name, entity)| (name.clone(), entity.to_bits()))
            .collect::<Vec<_>>();
        identities.sort();
        identities
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

    #[test]
    fn installation_recipe_replays_once_across_restart_unload_and_reload() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/behavior-installation.ron");
        let mut rack = Oxurack::new();
        rack.try_install::<FirstRecipeHook>().unwrap();
        rack.try_install::<SecondRecipeHook>().unwrap();
        rack.try_install::<LiveBehaviorCounter>().unwrap();
        rack.set_driver_mode(SimulationDriver::Manual);
        rack.load_patch(&fixture).unwrap();
        let original_handle = rack.world().resource::<PatchHandle>().clone();
        let alpha = original_handle.modules["alpha"];
        let beta = original_handle.modules["beta"];
        assert_eq!(rack.world().resource::<RecipeHookOrder>().0, [1, 2]);
        assert_eq!(rack.world().resource::<LiveHookCalls>().0, 1);
        assert_eq!(live_parameter(&mut rack, alpha), ParameterValue::Int(2));
        assert_eq!(live_parameter(&mut rack, beta), ParameterValue::Int(3));
        rack.start();
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        assert_eq!(live_state(&rack, &original_handle, "alpha").total, 2);
        assert_eq!(live_state(&rack, &original_handle, "beta").total, 3);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 1);
        assert_eq!(rack.world().resource::<RecipeHookRuns>().0, 2);
        rack.stop();
        for _event in rack.drain_midi() {}

        rack.set_parameter("alpha", "increment", ParameterValue::Int(4))
            .unwrap();
        assert_eq!(live_parameter(&mut rack, alpha), ParameterValue::Int(4));
        assert!(matches!(
            rack.try_install::<CollidingRecipeHook>(),
            Err(CoreError::ModuleKindConflict { .. })
        ));
        rack.try_install::<LiveBehaviorCounter>().unwrap();
        assert_eq!(rack.world().resource::<RecipeHookOrder>().0, [1, 2]);
        assert_eq!(live_state(&rack, &original_handle, "alpha").total, 2);
        assert_eq!(live_state(&rack, &original_handle, "beta").total, 3);
        assert_eq!(rack.world().resource::<LiveHookCalls>().0, 1);
        rack.start();
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        assert_eq!(live_state(&rack, &original_handle, "alpha").total, 6);
        assert_eq!(live_state(&rack, &original_handle, "beta").total, 6);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 2);
        assert_eq!(rack.world().resource::<RecipeHookRuns>().0, 4);
        rack.stop();
        for _event in rack.drain_midi() {}

        rack.restart().unwrap();
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert_eq!(rack.world().resource::<RecipeHookOrder>().0, [1, 2]);
        assert_eq!(rack.world().resource::<LiveHookCalls>().0, 1);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 0);
        assert_eq!(rack.world().resource::<RecipeHookRuns>().0, 0);
        assert!(!rack.world().resource::<SimulationRunning>().0);
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().elapsed(),
            Duration::ZERO
        );
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().overstep(),
            Duration::ZERO
        );
        assert_eq!(
            rack.world().resource::<Time<Virtual>>().elapsed(),
            Duration::ZERO
        );
        assert_eq!(
            rack.world().resource::<Time<Virtual>>().delta(),
            Duration::ZERO
        );
        let restarted = rack.world().resource::<PatchHandle>().clone();
        let alpha = restarted.modules["alpha"];
        let beta = restarted.modules["beta"];
        assert_eq!(live_parameter(&mut rack, alpha), ParameterValue::Int(4));
        assert_eq!(live_parameter(&mut rack, beta), ParameterValue::Int(3));
        assert_eq!(live_state(&rack, &restarted, "alpha").total, 0);
        assert_eq!(live_state(&rack, &restarted, "beta").total, 0);
        rack.start();
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        assert_eq!(live_state(&rack, &restarted, "alpha").total, 4);
        assert_eq!(live_state(&rack, &restarted, "beta").total, 3);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 1);
        assert_eq!(rack.world().resource::<RecipeHookRuns>().0, 2);

        rack.stop();
        for _event in rack.drain_midi() {}
        rack.unload_patch().unwrap();
        assert!(rack.world().get_resource::<PatchHandle>().is_none());
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert_eq!(rack.world().resource::<RecipeHookOrder>().0, [1, 2]);
        assert!(!rack.world().resource::<SimulationRunning>().0);
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().elapsed(),
            Duration::ZERO
        );
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().overstep(),
            Duration::ZERO
        );
        assert_eq!(
            rack.world().resource::<Time<Virtual>>().elapsed(),
            Duration::ZERO
        );
        assert_eq!(
            rack.world().resource::<Time<Virtual>>().delta(),
            Duration::ZERO
        );
        assert_eq!(rack.world().resource::<LiveHookCalls>().0, 1);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 0);
        assert_eq!(rack.world().resource::<RecipeHookRuns>().0, 0);
        rack.world_mut().insert_resource(RecipeHookRuns(17));
        rack.unload_patch().unwrap();
        assert_eq!(rack.world().resource::<RecipeHookOrder>().0, [1, 2]);
        assert_eq!(rack.world().resource::<RecipeHookRuns>().0, 17);

        rack.load_patch(&fixture).unwrap();
        let reloaded = rack.world().resource::<PatchHandle>().clone();
        let alpha = reloaded.modules["alpha"];
        let beta = reloaded.modules["beta"];
        assert_eq!(rack.world().resource::<RecipeHookOrder>().0, [1, 2]);
        assert!(!rack.world().resource::<SimulationRunning>().0);
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().elapsed(),
            Duration::ZERO
        );
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().overstep(),
            Duration::ZERO
        );
        assert_eq!(
            rack.world().resource::<Time<Virtual>>().elapsed(),
            Duration::ZERO
        );
        assert_eq!(
            rack.world().resource::<Time<Virtual>>().delta(),
            Duration::ZERO
        );
        assert_eq!(rack.world().resource::<LiveHookCalls>().0, 1);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 0);
        assert_eq!(rack.world().resource::<RecipeHookRuns>().0, 0);
        assert_eq!(live_parameter(&mut rack, alpha), ParameterValue::Int(2));
        assert_eq!(live_parameter(&mut rack, beta), ParameterValue::Int(3));
        rack.start();
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        assert_eq!(live_state(&rack, &reloaded, "alpha").total, 2);
        assert_eq!(live_state(&rack, &reloaded, "beta").total, 3);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 1);
        assert_eq!(rack.world().resource::<RecipeHookRuns>().0, 2);
    }

    #[derive(Resource)]
    struct LoadedOwnerMarker;

    #[derive(Resource)]
    struct EmptyOwnerMarker;

    #[test]
    fn failed_restart_snapshot_and_unloaded_restart_preserve_admission_boundaries() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<LfoModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<MidiOutModule>().unwrap();
        rack.set_driver_mode(SimulationDriver::Manual);
        rack.load_patch(&fixture).unwrap();
        rack.configure_scope(&[ScopeSelector::new("lfo_main", "bipolar_out")], 64)
            .unwrap();
        rack.start();
        assert_eq!(rack.advance_samples(7).unwrap(), 7);
        rack.stop();
        let snapshot = rack.snapshot_patch().unwrap();
        let membership = module_identities(rack.world());
        let sample_time = rack.sample_time();
        let driver = rack.driver_mode();
        let cable = {
            let world = rack.world_mut();
            let mut query = world.query::<(Entity, &oxurack_core::Cable)>();
            query.iter(world).next().map(|(entity, _)| entity).unwrap()
        };
        rack.world_mut()
            .get_mut::<oxurack_core::Cable>(cable)
            .unwrap()
            .enabled = false;
        let capture_disabled_owner = |rack: &Oxurack| {
            let fixed = rack.world().resource::<Time<Fixed>>();
            let virtual_time = rack.world().resource::<Time<Virtual>>();
            let scope = rack.scope().unwrap();
            let output = rack.world().resource::<MidiOutputBuffer>();
            let cable_state = rack.world().get::<oxurack_core::Cable>(cable).unwrap();
            let mut handle_modules = rack
                .world()
                .resource::<PatchHandle>()
                .modules
                .iter()
                .map(|(name, entity)| (name.clone(), entity.to_bits()))
                .collect::<Vec<_>>();
            handle_modules.sort();
            let module_added = rack
                .world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleAdded>>()
                .iter_current_update_messages()
                .map(|event| {
                    (
                        event.entity.to_bits(),
                        event.instance_name.clone(),
                        event.kind.to_string(),
                    )
                })
                .collect::<Vec<_>>();
            let patch_loaded = rack
                .world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::PatchLoaded>>()
                .iter_current_update_messages()
                .map(|event| {
                    (
                        event.patch_name.clone(),
                        event.source.clone(),
                        event.module_count,
                        event.cable_count,
                        event.label_count,
                    )
                })
                .collect::<Vec<_>>();
            let installations = rack
                .install_order
                .iter()
                .map(|installed| installed.kind.to_string())
                .collect::<Vec<_>>();
            (
                (
                    rack.sample_time(),
                    rack.driver_mode(),
                    rack.world().resource::<SimulationRunning>().0,
                    fixed.elapsed(),
                    fixed.overstep(),
                    virtual_time.elapsed(),
                    virtual_time.delta(),
                    virtual_time.is_paused(),
                    output.fault().map(|fault| format!("{fault:?}")),
                ),
                (
                    scope.selectors().to_vec(),
                    scope.capacity(),
                    scope.overwritten(),
                    scope.records().copied().collect::<Vec<_>>(),
                ),
                (
                    module_identities(rack.world()),
                    handle_modules,
                    (
                        cable.to_bits(),
                        cable_state.enabled,
                        cable_state.source_port.to_bits(),
                        cable_state.target_port.to_bits(),
                        rack.world()
                            .get::<oxurack_core::CableOrder>(cable)
                            .map(|order| order.0),
                    ),
                    installations,
                ),
                (module_added, patch_loaded),
            )
        };
        let before_failed_restart = capture_disabled_owner(&rack);
        assert!(matches!(
            rack.restart(),
            Err(CoreError::Patch(oxurack_core::PatchError::UnsupportedSnapshot { reason }))
                if reason.contains("disabled") && reason.contains("cable")
        ));
        assert_eq!(capture_disabled_owner(&rack), before_failed_restart);
        assert_eq!(module_identities(rack.world()), membership);
        assert_eq!(rack.sample_time(), sample_time);
        assert_eq!(rack.driver_mode(), driver);
        assert!(rack.world().get_resource::<PatchHandle>().is_some());
        rack.world_mut()
            .get_mut::<oxurack_core::Cable>(cable)
            .unwrap()
            .enabled = true;
        assert_eq!(rack.snapshot_patch().unwrap(), snapshot);

        rack.unload_patch().unwrap();
        assert!(matches!(rack.restart(), Err(CoreError::NoPatchLoaded)));
        assert_eq!(rack.sample_time().completed_samples(), 0);
        let mut unloaded_running = Oxurack::new();
        unloaded_running
            .set_driver_mode(SimulationDriver::Manual)
            .start();
        assert!(matches!(
            unloaded_running.restart(),
            Err(CoreError::LifecycleRequiresStopped)
        ));
        unloaded_running.stop();
        assert!(matches!(
            unloaded_running.restart(),
            Err(CoreError::NoPatchLoaded)
        ));
    }

    #[test]
    fn empty_unload_retires_owner_then_repeated_noops_preserve_fault_and_hooks() {
        let empty_patch = Patch {
            version: "empty-test-version".into(),
            master_seed: 91,
            bpm: 97.5,
            modules: Vec::new(),
            cables: Vec::new(),
            rack: None,
        };
        let path = save_patch_to_temp(&empty_patch, "oxurack-empty-lifecycle.ron");
        let mut rack = Oxurack::new();
        rack.try_install::<FirstRecipeHook>().unwrap();
        rack.try_install::<SecondRecipeHook>().unwrap();
        rack.set_driver_mode(SimulationDriver::Manual);
        rack.load_patch(&path).unwrap();
        assert_eq!(rack.snapshot_patch().unwrap(), empty_patch);
        assert_eq!(rack.world().resource::<RecipeHookOrder>().0, [1, 2]);
        rack.world_mut().insert_resource(LoadedOwnerMarker);
        rack.unload_patch().unwrap();
        assert!(rack.world().get_resource::<LoadedOwnerMarker>().is_none());
        assert!(rack.world().get_resource::<PatchHandle>().is_none());
        assert!(matches!(
            rack.snapshot_patch(),
            Err(CoreError::NoPatchLoaded)
        ));
        assert_eq!(rack.world().resource::<RecipeHookOrder>().0, [1, 2]);
        assert!(rack.drain_midi().next().is_none());

        rack.world_mut().insert_resource(EmptyOwnerMarker);
        let known_fault = TickError::OutputCapacity {
            reason: "empty rack fault remains first".into(),
            completed_samples: 9,
        };
        rack.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .latch_fault(known_fault.clone());
        let sample_time = rack.sample_time();
        let hook_order = rack.world().resource::<RecipeHookOrder>().0.clone();
        rack.reset().unwrap();
        assert!(rack.drain_midi().next().is_none());
        assert!(rack.world().get_resource::<EmptyOwnerMarker>().is_some());
        assert_eq!(rack.world().resource::<RecipeHookOrder>().0, hook_order);
        assert_eq!(rack.sample_time(), sample_time);
        assert_eq!(
            rack.midi_output_error().map(|error| format!("{error:?}")),
            Some(format!("{known_fault:?}"))
        );
        rack.unload_patch().unwrap();
        rack.unload_patch().unwrap();
        assert!(rack.world().get_resource::<EmptyOwnerMarker>().is_some());
        assert_eq!(rack.world().resource::<RecipeHookOrder>().0, hook_order);
        assert_eq!(rack.sample_time(), sample_time);
        assert_eq!(
            rack.midi_output_error().map(|error| format!("{error:?}")),
            Some(format!("{known_fault:?}"))
        );
        assert!(rack.drain_midi().next().is_none());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn nonempty_unload_cleanup_blocks_repeat_until_the_pending_batch_is_drained() {
        let patch = Patch {
            version: "1.0".into(),
            master_seed: 42,
            bpm: 120.0,
            modules: vec![ModuleConfig {
                kind: MidiOutModule::KIND.into(),
                instance_name: "midi_main".into(),
                parameters: HashMap::new(),
                ..Default::default()
            }],
            cables: Vec::new(),
            rack: None,
        };
        let path = save_patch_to_temp(&patch, "oxurack-nonempty-unload-cleanup.ron");
        let mut rack = Oxurack::new();
        rack.try_install::<MidiOutModule>().unwrap();
        rack.set_driver_mode(SimulationDriver::Manual);
        rack.load_patch(&path).unwrap();
        let sink = rack.world().resource::<PatchHandle>().modules["midi_main"];
        for (port, value) in [
            ("step", Value::Gate(true)),
            ("note_valid", Value::Gate(true)),
            ("velocity_valid", Value::Gate(true)),
            ("gate", Value::Gate(true)),
            ("note", Value::Raw(60)),
            ("velocity", Value::Raw(100)),
        ] {
            set_port_value(&mut rack, sink, port, value);
        }
        rack.start();
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        assert_eq!(
            rack.drain_midi()
                .map(|event| (event.sample, event.message.to_bytes()))
                .collect::<Vec<_>>(),
            vec![(0, [0x90, 60, 100])]
        );
        rack.stop();
        rack.unload_patch().unwrap();
        assert!(matches!(
            rack.snapshot_patch(),
            Err(CoreError::NoPatchLoaded)
        ));
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert!(matches!(
            rack.unload_patch(),
            Err(CoreError::PendingMidi { count: 3 })
        ));
        assert_eq!(
            rack.world().resource::<MidiOutputBuffer>().pending_count(),
            3
        );
        assert_eq!(
            rack.drain_midi()
                .map(|event| (event.sample, event.message.to_bytes()))
                .collect::<Vec<_>>(),
            vec![(1, [0x80, 60, 0]), (1, [0xB0, 123, 0]), (1, [0xB0, 120, 0]),]
        );
        rack.unload_patch().unwrap();
        assert_eq!(
            rack.world().resource::<MidiOutputBuffer>().pending_count(),
            0
        );
        assert!(rack.drain_midi().next().is_none());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn live_driver_replacement_rebases_time_without_catch_up() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/behavior-installation.ron");
        let mut rack = Oxurack::new();
        rack.try_install::<LiveBehaviorCounter>().unwrap();
        rack.set_driver_mode(SimulationDriver::Live);
        rack.load_patch(&fixture).unwrap();
        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO));
        rack.start();
        rack.step();
        assert_eq!(rack.sample_time().completed_samples(), 0);
        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(1)));
        rack.step();
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 1);

        rack.stop();
        rack.restart().unwrap();
        assert_eq!(rack.driver_mode(), SimulationDriver::Live);
        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
                500,
            )));
        rack.step();
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 0);
        rack.start();
        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO));
        rack.step();
        assert_eq!(rack.sample_time().completed_samples(), 0);
        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(1)));
        rack.step();
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 1);

        rack.stop();
        rack.unload_patch().unwrap();
        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
                500,
            )));
        rack.step();
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 0);
        rack.load_patch(&fixture).unwrap();
        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO));
        rack.start();
        rack.step();
        assert_eq!(rack.sample_time().completed_samples(), 0);
        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(1)));
        rack.step();
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 1);
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum StopPhase {
        Produce,
        Propagate,
        MergeInputs,
        Consume,
        Observe,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct PhaseMarker {
        phase: StopPhase,
        index: u64,
    }

    #[derive(Resource, Clone, Debug, PartialEq, Eq)]
    struct StopControl {
        stop: Option<(StopPhase, u64)>,
        resume: Option<(StopPhase, u64)>,
    }

    impl StopControl {
        fn none() -> Self {
            Self {
                stop: None,
                resume: None,
            }
        }
    }

    #[derive(Resource, Default, Clone, Debug, PartialEq, Eq)]
    struct StopTrace {
        markers: Vec<PhaseMarker>,
    }

    #[derive(Component, Clone, Debug, PartialEq, Eq)]
    struct StopProbeState {
        instance_seed: u64,
        state: u64,
        steps: u64,
    }

    struct StopProbe;

    impl OxurackModule for StopProbe {
        const KIND: &'static str = "stop_probe";
        const DISPLAY_NAME: &'static str = "Stop Accounting Probe";

        fn port_schema() -> &'static [PortSchema] {
            &[PortSchema {
                name: "out",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Deterministic stop-accounting state.",
            }]
        }

        fn parameter_schema() -> &'static [ParameterSchema] {
            &[]
        }

        fn spawn(
            world: &mut bevy_ecs::world::World,
            instance_name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<Entity, CoreError> {
            spawn_stop_probe(world, instance_name, 0)
        }

        fn spawn_with_context(
            world: &mut bevy_ecs::world::World,
            context: &ModuleSpawnContext<'_>,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<Entity, CoreError> {
            spawn_stop_probe(world, context.instance_name(), context.instance_seed())
        }

        fn register_systems(app: &mut App, schedule: bevy_ecs::schedule::InternedScheduleLabel) {
            app.add_systems(schedule, stop_probe_produce.in_set(TickPhase::Produce));
        }
    }

    fn spawn_stop_probe(
        world: &mut bevy_ecs::world::World,
        instance_name: &str,
        instance_seed: u64,
    ) -> Result<Entity, CoreError> {
        let entity = oxurack_core::spawn_module_entity(world, StopProbe::KIND, instance_name);
        oxurack_core::spawn_port_on_module(
            world,
            entity,
            "out",
            PortDirection::Output,
            ValueKind::Gate,
            MergePolicy::Reject,
        );
        world.entity_mut(entity).insert(StopProbeState {
            instance_seed,
            state: instance_seed,
            steps: 0,
        });
        Ok(entity)
    }

    fn record_phase(
        phase: StopPhase,
        index: u64,
        trace: &mut StopTrace,
        control: &StopControl,
        running: &mut SimulationRunning,
    ) {
        trace.markers.push(PhaseMarker { phase, index });
        if control.stop == Some((phase, index)) {
            running.0 = false;
        }
        if control.resume == Some((phase, index)) {
            running.0 = true;
        }
    }

    fn stop_probe_produce(
        sample_time: Res<SampleTime>,
        mut trace: ResMut<StopTrace>,
        control: Res<StopControl>,
        mut running: ResMut<SimulationRunning>,
        mut states: Query<(&mut StopProbeState, &Children)>,
        mut current_values: Query<&mut CurrentValue>,
    ) {
        let Some(index) = sample_time.last_index() else {
            return;
        };

        for (mut state, children) in &mut states {
            state.state = state
                .state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            state.steps += 1;
            let value = oxurack_core::Value::Gate(state.state & 1 == 1);
            for child in children.iter() {
                if let Ok(mut current) = current_values.get_mut(*child) {
                    current.0 = value;
                }
            }
        }

        record_phase(
            StopPhase::Produce,
            index,
            &mut trace,
            &control,
            &mut running,
        );
    }

    fn stop_probe_propagate(
        sample_time: Res<SampleTime>,
        mut trace: ResMut<StopTrace>,
        control: Res<StopControl>,
        mut running: ResMut<SimulationRunning>,
    ) {
        let Some(index) = sample_time.last_index() else {
            return;
        };
        record_phase(
            StopPhase::Propagate,
            index,
            &mut trace,
            &control,
            &mut running,
        );
    }

    fn stop_probe_merge_inputs(
        sample_time: Res<SampleTime>,
        mut trace: ResMut<StopTrace>,
        control: Res<StopControl>,
        mut running: ResMut<SimulationRunning>,
    ) {
        let Some(index) = sample_time.last_index() else {
            return;
        };
        record_phase(
            StopPhase::MergeInputs,
            index,
            &mut trace,
            &control,
            &mut running,
        );
    }

    fn stop_probe_consume(
        sample_time: Res<SampleTime>,
        mut trace: ResMut<StopTrace>,
        control: Res<StopControl>,
        mut running: ResMut<SimulationRunning>,
    ) {
        let Some(index) = sample_time.last_index() else {
            return;
        };
        record_phase(
            StopPhase::Consume,
            index,
            &mut trace,
            &control,
            &mut running,
        );
    }

    fn stop_probe_observe(
        sample_time: Res<SampleTime>,
        mut trace: ResMut<StopTrace>,
        control: Res<StopControl>,
        mut running: ResMut<SimulationRunning>,
    ) {
        let Some(index) = sample_time.last_index() else {
            return;
        };
        record_phase(
            StopPhase::Observe,
            index,
            &mut trace,
            &control,
            &mut running,
        );
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct StopObservation {
        sample_time: SampleTime,
        fixed_elapsed: Duration,
        virtual_elapsed: Duration,
        generic_elapsed: Duration,
        fixed_delta: Duration,
        virtual_delta: Duration,
        generic_delta: Duration,
        fixed_overstep: Duration,
        state: StopProbeState,
        port_gate: bool,
        markers: Vec<PhaseMarker>,
    }

    fn stop_probe_patch() -> oxurack_core::Patch {
        oxurack_core::Patch {
            version: "1.0".to_owned(),
            master_seed: 42,
            bpm: 120.0,
            modules: vec![ModuleConfig {
                kind: StopProbe::KIND.to_owned(),
                instance_name: "probe".to_owned(),
                ..Default::default()
            }],
            cables: vec![],
            rack: None,
        }
    }

    fn stop_probe_rack(name: &str, control: StopControl) -> (Oxurack, PatchHandle) {
        let mut rack = Oxurack::new();
        rack.install::<StopProbe>();

        let path = save_patch_to_temp(
            &stop_probe_patch(),
            &format!("oxurack_slice04_{name}_{}.ron", std::process::id()),
        );
        rack.load_patch(&path)
            .expect("stop-accounting patch should load through the public path");
        let _ = std::fs::remove_file(&path);
        rack.world_mut().insert_resource(control);
        rack.world_mut().insert_resource(StopTrace::default());
        rack.app.add_systems(
            FixedUpdate,
            (
                stop_probe_propagate.in_set(TickPhase::Propagate),
                stop_probe_merge_inputs.in_set(TickPhase::MergeInputs),
                stop_probe_consume.in_set(TickPhase::Consume),
                stop_probe_observe.in_set(TickPhase::Observe),
            ),
        );
        rack.set_driver_mode(SimulationDriver::Manual).start();

        let handle = rack.world().resource::<PatchHandle>().clone();
        (rack, handle)
    }

    fn stop_probe_observation(rack: &Oxurack, handle: &PatchHandle) -> StopObservation {
        let world = rack.world();
        let module_entity = handle.modules["probe"];
        let state = world
            .entity(module_entity)
            .get::<StopProbeState>()
            .expect("probe state should exist")
            .clone();
        let children = world
            .entity(module_entity)
            .get::<Children>()
            .expect("probe should have a port child");
        let port_gate = children
            .iter()
            .find_map(|child| {
                let port = world.entity(*child).get::<oxurack_core::Port>()?;
                if port.name.as_ref() != "out" {
                    return None;
                }
                match world.entity(*child).get::<CurrentValue>()?.0 {
                    oxurack_core::Value::Gate(value) => Some(value),
                    _ => None,
                }
            })
            .expect("probe output gate should exist");
        let fixed = world.resource::<Time<Fixed>>();
        let virtual_time = world.resource::<Time<Virtual>>();
        let generic = world.resource::<Time>();

        StopObservation {
            sample_time: *world.resource::<SampleTime>(),
            fixed_elapsed: fixed.elapsed(),
            virtual_elapsed: virtual_time.elapsed(),
            generic_elapsed: generic.elapsed(),
            fixed_delta: fixed.delta(),
            virtual_delta: virtual_time.delta(),
            generic_delta: generic.delta(),
            fixed_overstep: fixed.overstep(),
            state,
            port_gate,
            markers: world.resource::<StopTrace>().markers.clone(),
        }
    }

    fn expected_markers(samples: u64) -> Vec<PhaseMarker> {
        let phases = [
            StopPhase::Produce,
            StopPhase::Propagate,
            StopPhase::MergeInputs,
            StopPhase::Consume,
            StopPhase::Observe,
        ];
        (0..samples)
            .flat_map(|index| {
                phases
                    .iter()
                    .copied()
                    .map(move |phase| PhaseMarker { phase, index })
            })
            .collect()
    }

    fn assert_partial_sample(
        rack: &Oxurack,
        handle: &PatchHandle,
        returned: u64,
        expected_samples: u64,
        message: &str,
    ) -> StopObservation {
        let observation = stop_probe_observation(rack, handle);
        assert_eq!(returned, expected_samples, "{message}: returned count");
        assert_eq!(
            observation.sample_time.completed_samples(),
            expected_samples,
            "{message}: completed count"
        );
        assert_eq!(
            observation.sample_time.last_index(),
            expected_samples.checked_sub(1),
            "{message}: last sample index"
        );
        assert_eq!(
            observation.fixed_elapsed,
            Duration::from_millis(expected_samples),
            "{message}: fixed elapsed"
        );
        assert_eq!(
            observation.virtual_elapsed,
            Duration::from_millis(expected_samples),
            "{message}: virtual elapsed"
        );
        assert_eq!(
            observation.generic_elapsed, observation.virtual_elapsed,
            "{message}: generic Time must restore Virtual time"
        );
        assert_eq!(
            observation.fixed_delta,
            Duration::from_millis(1),
            "{message}: fixed delta"
        );
        assert_eq!(
            observation.virtual_delta,
            Duration::from_millis(1),
            "{message}: virtual delta"
        );
        assert_eq!(
            observation.generic_delta, observation.virtual_delta,
            "{message}: generic delta must restore Virtual time"
        );
        assert_eq!(
            observation.fixed_overstep,
            Duration::ZERO,
            "{message}: fixed overstep"
        );
        assert_eq!(
            observation.markers,
            expected_markers(expected_samples),
            "{message}: complete phase trace"
        );
        assert_eq!(
            observation.state.steps, expected_samples,
            "{message}: module state advances once per completed sample"
        );
        assert_eq!(
            observation.state.instance_seed,
            ModuleSpawnContext::new(42, "probe").instance_seed(),
            "{message}: spawn context seed is owned by module state"
        );
        assert_eq!(
            observation.port_gate,
            observation.state.state & 1 == 1,
            "{message}: output port follows owned module state"
        );
        observation
    }

    // ── Tests ──────────────────────────────────────────────────────

    #[test]
    fn test_stop_accounting_stops_in_produce_after_one_complete_sample() {
        let (mut rack, handle) = stop_probe_rack(
            "produce",
            StopControl {
                stop: Some((StopPhase::Produce, 0)),
                resume: None,
            },
        );
        let returned = rack
            .advance_samples(3)
            .expect("Produce stop should be partial success");
        assert_partial_sample(&rack, &handle, returned, 1, "Produce stop at sample zero");
    }

    #[test]
    fn test_stop_accounting_stops_in_consume_after_one_complete_sample() {
        let (mut rack, handle) = stop_probe_rack(
            "consume",
            StopControl {
                stop: Some((StopPhase::Consume, 0)),
                resume: None,
            },
        );
        let returned = rack
            .advance_samples(3)
            .expect("Consume stop should be partial success");
        assert_partial_sample(&rack, &handle, returned, 1, "Consume stop at sample zero");
    }

    #[test]
    fn test_stop_accounting_stops_in_observe_after_one_complete_sample() {
        let (mut rack, handle) = stop_probe_rack(
            "observe",
            StopControl {
                stop: Some((StopPhase::Observe, 0)),
                resume: None,
            },
        );
        let returned = rack
            .advance_samples(3)
            .expect("Observe stop should be partial success");
        assert_partial_sample(&rack, &handle, returned, 1, "Observe stop at sample zero");
    }

    #[test]
    fn test_stop_accounting_stops_at_sample_one_with_two_complete_samples() {
        let (mut rack, handle) = stop_probe_rack(
            "sample-one",
            StopControl {
                stop: Some((StopPhase::Consume, 1)),
                resume: None,
            },
        );
        let returned = rack
            .advance_samples(5)
            .expect("sample-one stop should be partial success");
        let observation = assert_partial_sample(&rack, &handle, returned, 2, "stop at sample one");
        assert_eq!(observation.markers.len(), 10);
    }

    #[test]
    fn test_stop_accounting_repeated_stopped_calls_are_full_noops() {
        let (mut rack, handle) = stop_probe_rack(
            "stopped-noop",
            StopControl {
                stop: Some((StopPhase::Observe, 0)),
                resume: None,
            },
        );
        let returned = rack
            .advance_samples(3)
            .expect("initial stop should be partial success");
        assert_partial_sample(&rack, &handle, returned, 1, "initial stopped observation");
        let before = stop_probe_observation(&rack, &handle);

        for requested in [1, 3, 0, 100] {
            assert_eq!(
                rack.advance_samples(requested)
                    .expect("stopped manual call should be a no-op"),
                0,
                "stopped request {requested} should complete zero samples"
            );
            assert_eq!(
                stop_probe_observation(&rack, &handle),
                before,
                "stopped request {requested} changed clock, state, port or trace"
            );
        }
    }

    #[test]
    fn test_stop_accounting_start_resumes_against_uninterrupted_reference() {
        let (mut rack, handle) = stop_probe_rack(
            "resume",
            StopControl {
                stop: Some((StopPhase::Produce, 0)),
                resume: None,
            },
        );
        let returned = rack
            .advance_samples(3)
            .expect("initial stop should be partial success");
        assert_partial_sample(&rack, &handle, returned, 1, "resume starting point");

        rack.world_mut().insert_resource(StopControl::none());
        rack.start();
        assert_eq!(
            rack.advance_samples(1)
                .expect("resumed manual sample should run"),
            1
        );
        let resumed = stop_probe_observation(&rack, &handle);

        let (mut reference, reference_handle) = stop_probe_rack("reference", StopControl::none());
        assert_eq!(
            reference
                .advance_samples(2)
                .expect("uninterrupted reference should run"),
            2
        );
        let expected = stop_probe_observation(&reference, &reference_handle);

        assert_eq!(
            resumed, expected,
            "resume must match fresh two-sample state"
        );
        assert_eq!(resumed.sample_time.last_index(), Some(1));
        assert_eq!(resumed.sample_time.completed_samples(), 2);
    }

    #[test]
    fn test_stop_accounting_false_then_true_allows_next_sample() {
        let (mut rack, handle) = stop_probe_rack(
            "false-true",
            StopControl {
                stop: Some((StopPhase::Produce, 0)),
                resume: Some((StopPhase::Observe, 0)),
            },
        );
        let returned = rack
            .advance_samples(3)
            .expect("later resume should preserve batch continuation");
        let observation =
            assert_partial_sample(&rack, &handle, returned, 3, "false then true in one sample");
        assert!(
            rack.world().resource::<SimulationRunning>().0,
            "final shared running value should permit the next sample"
        );
        assert_eq!(observation.markers.len(), 15);
    }

    #[test]
    fn test_stop_accounting_uninterrupted_manual_and_zero_count() {
        let (mut rack, handle) = stop_probe_rack("uninterrupted", StopControl::none());
        let before = stop_probe_observation(&rack, &handle);
        assert_eq!(
            rack.advance_samples(0)
                .expect("manual zero-count request should be a no-op"),
            0
        );
        assert_eq!(
            stop_probe_observation(&rack, &handle),
            before,
            "manual zero-count request mutated the rack"
        );

        let returned = rack
            .advance_samples(3)
            .expect("uninterrupted manual request should complete all samples");
        assert_partial_sample(&rack, &handle, returned, 3, "uninterrupted manual request");
    }

    #[test]
    fn test_stop_accounting_live_error_precedes_mutation_for_all_counts() {
        let (mut rack, handle) = stop_probe_rack("live-error", StopControl::none());
        rack.set_driver_mode(SimulationDriver::Live);

        for requested in [3, 0] {
            let before = stop_probe_observation(&rack, &handle);
            assert!(matches!(
                rack.advance_samples(requested),
                Err(CoreError::ManualSampleRequiresManualDriver)
            ));
            assert_eq!(
                stop_probe_observation(&rack, &handle),
                before,
                "Live request {requested} mutated before returning its typed error"
            );
        }

        rack.stop();
        let before = stop_probe_observation(&rack, &handle);
        assert!(matches!(
            rack.advance_samples(2),
            Err(CoreError::ManualSampleRequiresManualDriver)
        ));
        assert_eq!(
            stop_probe_observation(&rack, &handle),
            before,
            "stopped Live request mutated before returning its typed error"
        );
    }

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
    fn public_running_observation_tracks_start_and_stop_without_advancing() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/behavior-installation.ron");
        let mut rack = Oxurack::new();
        rack.install::<LiveBehaviorCounter>();
        rack.load_patch(&fixture)
            .expect("behavior fixture should load through the public path");
        let before = rack.sample_time();
        assert!(!rack.is_running());
        rack.start();
        assert!(rack.is_running());
        assert_eq!(rack.sample_time(), before);
        rack.stop();
        assert!(!rack.is_running());
        assert_eq!(rack.sample_time(), before);
    }

    #[test]
    fn test_live_transitions_and_stop_start_discard_catch_up() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/behavior-installation.ron");
        let mut rack = Oxurack::new();
        rack.install::<LiveBehaviorCounter>();
        rack.load_patch(&fixture)
            .expect("behavior fixture should load through the public path");
        rack.start();

        let handle = rack.world().resource::<PatchHandle>().clone();
        rack.set_driver_mode(SimulationDriver::Manual);
        assert_eq!(
            rack.advance_samples(1).expect("manual sample should run"),
            1
        );
        assert_eq!(rack.sample_time().last_index(), Some(0));
        assert_eq!(live_state(&rack, &handle, "alpha").total, 2);

        rack.set_driver_mode(SimulationDriver::Live);
        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO));
        rack.step();
        assert_eq!(rack.sample_time().last_index(), Some(0));
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert_eq!(live_state(&rack, &handle, "alpha").total, 2);

        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(1)));
        rack.step();
        assert_eq!(rack.sample_time().last_index(), Some(1));
        assert_eq!(rack.sample_time().completed_samples(), 2);
        assert_eq!(live_state(&rack, &handle, "alpha").total, 4);

        rack.world_mut()
            .resource_mut::<Time<Fixed>>()
            .accumulate_overstep(Duration::from_millis(5));
        rack.stop();
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().overstep(),
            Duration::ZERO
        );
        let stopped_time = rack.sample_time();
        let stopped_state = live_state(&rack, &handle, "alpha");

        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(5)));
        rack.step();
        assert_eq!(rack.sample_time(), stopped_time);
        assert_eq!(live_state(&rack, &handle, "alpha"), stopped_state);

        rack.start();
        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO));
        rack.step();
        assert_eq!(rack.sample_time(), stopped_time);
        assert_eq!(live_state(&rack, &handle, "alpha"), stopped_state);

        rack.world_mut()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(1)));
        rack.step();
        assert_eq!(rack.sample_time().last_index(), Some(2));
        assert_eq!(rack.sample_time().completed_samples(), 3);
        assert_eq!(live_state(&rack, &handle, "alpha").total, 6);
        assert_eq!(rack.world().resource::<LiveSystemRuns>().0, 3);
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

    #[test]
    fn midi_facade_denies_unread_next_sample_before_either_clock_advances() {
        let mut rack = Oxurack::new();
        rack.try_install::<MidiOutModule>().unwrap();
        let sink = MidiOutModule::spawn(rack.world_mut(), "sink", &HashMap::new()).unwrap();
        for (port, value) in [
            ("step", Value::Gate(true)),
            ("note_valid", Value::Gate(true)),
            ("velocity_valid", Value::Gate(true)),
            ("gate", Value::Gate(true)),
            ("note", Value::Raw(60)),
            ("velocity", Value::Raw(100)),
            ("noise_cc", Value::Raw(0)),
        ] {
            set_port_value(&mut rack, sink, port, value);
        }
        rack.set_driver_mode(SimulationDriver::Manual).start();
        let error = rack.advance_samples(5).unwrap_err();
        assert!(matches!(
            error,
            CoreError::Tick(TickError::OutputBackpressure {
                pending_messages: 1,
                completed_samples: 1
            })
        ));
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert_eq!(
            rack.world().resource::<Time<Fixed>>().elapsed(),
            Duration::from_millis(1)
        );
        assert_eq!(
            rack.world().resource::<Time<Virtual>>().elapsed(),
            Duration::from_millis(1)
        );
        assert!(matches!(
            rack.midi_output_error(),
            Some(TickError::OutputBackpressure {
                pending_messages: 1,
                completed_samples: 0
            })
        ));
        let first_fault = rack.midi_output_error().map(|error| format!("{error:?}"));
        assert!(first_fault.is_some());
        rack.reset().unwrap();
        assert_eq!(
            rack.midi_output_error().map(|error| format!("{error:?}")),
            first_fault,
            "the first reset preserves the actual stored backpressure fault"
        );
        let reset_events: Vec<_> = rack.drain_midi().collect();
        assert_eq!(
            reset_events
                .iter()
                .map(|event| (event.sample, event.message.to_bytes()))
                .collect::<Vec<_>>(),
            vec![
                (0, [0x90, 60, 100]),
                (1, [0x80, 60, 0]),
                (1, [0xB0, 123, 0]),
                (1, [0xB0, 120, 0]),
            ]
        );
        let retained_fault = rack.midi_output_error().map(|error| format!("{error:?}"));
        rack.reset().unwrap();
        assert!(rack.drain_midi().next().is_none());
        assert_eq!(
            rack.midi_output_error().map(|error| format!("{error:?}")),
            retained_fault
        );
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert_eq!(
            rack.advance_samples(1).unwrap(),
            0,
            "drain does not clear the terminal fault or restart the rack"
        );
    }

    #[test]
    fn pending_output_rejects_lifecycle_and_reset_preserves_then_cleans_exact_state() {
        #[derive(Debug, PartialEq)]
        struct RetainedState {
            patch: Patch,
            driver: SimulationDriver,
            sample_time: SampleTime,
            running: bool,
            fixed_elapsed: Duration,
            fixed_overstep: Duration,
            virtual_elapsed: Duration,
            virtual_delta: Duration,
            virtual_paused: bool,
            selectors: Vec<ScopeSelector>,
            scope_capacity: usize,
            scope_overwritten: u64,
            scope_records: Vec<ScopeRecord>,
            pending_events: usize,
            fault: Option<String>,
            modules: Vec<(u64, String, String)>,
            loaded: Vec<(String, oxurack_core::PatchSource, usize, usize, usize)>,
            installations: Vec<String>,
        }

        fn retained_state(rack: &mut Oxurack) -> RetainedState {
            let patch = rack.snapshot_patch().unwrap();
            let fixed = rack.world().resource::<Time<Fixed>>();
            let virtual_time = rack.world().resource::<Time<Virtual>>();
            let scope = rack.scope().unwrap();
            let output = rack.world().resource::<MidiOutputBuffer>();
            let modules = rack
                .world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleAdded>>()
                .iter_current_update_messages()
                .map(|event| {
                    (
                        event.entity.to_bits(),
                        event.instance_name.clone(),
                        event.kind.to_string(),
                    )
                })
                .collect::<Vec<_>>();
            let loaded = rack
                .world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::PatchLoaded>>()
                .iter_current_update_messages()
                .map(|event| {
                    (
                        event.patch_name.clone(),
                        event.source.clone(),
                        event.module_count,
                        event.cable_count,
                        event.label_count,
                    )
                })
                .collect::<Vec<_>>();
            let mut loaded = loaded;
            loaded.sort_by(|left, right| left.0.cmp(&right.0));
            let mut modules = modules;
            modules.sort();
            RetainedState {
                patch,
                driver: rack.driver_mode(),
                sample_time: rack.sample_time(),
                running: rack.world().resource::<SimulationRunning>().0,
                fixed_elapsed: fixed.elapsed(),
                fixed_overstep: fixed.overstep(),
                virtual_elapsed: virtual_time.elapsed(),
                virtual_delta: virtual_time.delta(),
                virtual_paused: virtual_time.is_paused(),
                selectors: scope.selectors().to_vec(),
                scope_capacity: scope.capacity(),
                scope_overwritten: scope.overwritten(),
                scope_records: scope.records().copied().collect(),
                pending_events: output.pending_count(),
                fault: output.fault().map(|fault| format!("{fault:?}")),
                modules,
                loaded,
                installations: rack
                    .install_order
                    .iter()
                    .map(|installed| installed.kind.to_string())
                    .collect(),
            }
        }

        let mut rack = Oxurack::new();
        rack.try_install::<MidiOutModule>().unwrap();
        let patch = Patch {
            version: "1.0".into(),
            master_seed: 42,
            bpm: 120.0,
            modules: vec![ModuleConfig {
                kind: MidiOutModule::KIND.into(),
                instance_name: "midi_main".into(),
                parameters: HashMap::new(),
                ..Default::default()
            }],
            cables: Vec::new(),
            rack: None,
        };
        let path = save_patch_to_temp(&patch, "oxurack-pending-lifecycle.ron");
        rack.set_driver_mode(SimulationDriver::Manual);
        rack.load_patch(&path).unwrap();
        let sink = rack.world().resource::<PatchHandle>().modules["midi_main"];
        for (port, value) in [
            ("step", Value::Gate(true)),
            ("note_valid", Value::Gate(true)),
            ("velocity_valid", Value::Gate(true)),
            ("gate", Value::Gate(true)),
            ("note", Value::Raw(60)),
            ("velocity", Value::Raw(100)),
        ] {
            set_port_value(&mut rack, sink, port, value);
        }
        rack.configure_scope(&[ScopeSelector::new("midi_main", "note")], 32)
            .unwrap();
        rack.start();
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        assert_eq!(
            rack.world().resource::<MidiOutputBuffer>().pending_count(),
            1
        );

        for operation in ["load", "restart", "unload"] {
            let before = retained_state(&mut rack);
            let result = match operation {
                "load" => rack.load_patch(Path::new("/missing-running-pending")),
                "restart" => rack.restart(),
                "unload" => rack.unload_patch(),
                _ => unreachable!(),
            };
            assert!(matches!(result, Err(CoreError::LifecycleRequiresStopped)));
            assert_eq!(rack.sample_time().completed_samples(), 1);
            assert_eq!(
                rack.world().resource::<MidiOutputBuffer>().pending_count(),
                1
            );
            assert_eq!(retained_state(&mut rack), before);
        }
        rack.stop();
        assert_eq!(
            rack.world().resource::<MidiOutputBuffer>().pending_count(),
            1
        );
        for operation in ["load", "restart", "unload"] {
            let before = retained_state(&mut rack);
            let result = match operation {
                "load" => rack.load_patch(Path::new("/missing-stopped-pending")),
                "restart" => rack.restart(),
                "unload" => rack.unload_patch(),
                _ => unreachable!(),
            };
            assert!(matches!(result, Err(CoreError::PendingMidi { count: 1 })));
            assert_eq!(retained_state(&mut rack), before);
        }
        let mut expected_first_reset = retained_state(&mut rack);
        assert_eq!(expected_first_reset.pending_events, 1);
        expected_first_reset.pending_events = 4;
        rack.reset().unwrap();
        assert_eq!(
            retained_state(&mut rack),
            expected_first_reset,
            "first reset preserves all retained owner state except pending output 1→4"
        );
        assert_eq!(
            rack.world().resource::<MidiOutputBuffer>().pending_count(),
            4
        );
        let after_first_reset = retained_state(&mut rack);
        rack.reset().unwrap();
        assert_eq!(
            rack.world().resource::<MidiOutputBuffer>().pending_count(),
            4
        );
        assert_eq!(retained_state(&mut rack), after_first_reset);
        let drained: Vec<_> = rack
            .drain_midi()
            .map(|event| (event.sample, event.message.to_bytes()))
            .collect();
        assert_eq!(
            drained,
            vec![
                (0, [0x90, 60, 100]),
                (1, [0x80, 60, 0]),
                (1, [0xB0, 123, 0]),
                (1, [0xB0, 120, 0]),
            ]
        );

        rack.start();
        rack.stop();
        assert!(rack.drain_midi().next().is_none());
        rack.start();
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        assert_eq!(
            rack.drain_midi()
                .map(|event| (event.sample, event.message.to_bytes()))
                .collect::<Vec<_>>(),
            vec![(1, [0x90, 60, 100])]
        );
        let mut expected_running_reset = retained_state(&mut rack);
        assert!(expected_running_reset.running);
        assert_eq!(expected_running_reset.sample_time.completed_samples(), 2);
        assert_eq!(expected_running_reset.pending_events, 0);
        expected_running_reset.pending_events = 3;
        rack.reset().unwrap();
        assert_eq!(
            retained_state(&mut rack),
            expected_running_reset,
            "running reset preserves state and appends exactly three cleanup events"
        );
        assert_eq!(
            rack.drain_midi()
                .map(|event| (event.sample, event.message.to_bytes()))
                .collect::<Vec<_>>(),
            vec![(2, [0x80, 60, 0]), (2, [0xB0, 123, 0]), (2, [0xB0, 120, 0]),]
        );
        rack.stop();
        rack.reset().unwrap();
        assert!(rack.drain_midi().next().is_none());

        let mut unloaded = Oxurack::new();
        unloaded.try_install::<MidiOutModule>().unwrap();
        let sink = MidiOutModule::spawn(unloaded.world_mut(), "raw_sink", &HashMap::new()).unwrap();
        for (port, value) in [
            ("step", Value::Gate(true)),
            ("note_valid", Value::Gate(true)),
            ("velocity_valid", Value::Gate(true)),
            ("gate", Value::Gate(true)),
            ("note", Value::Raw(62)),
            ("velocity", Value::Raw(99)),
        ] {
            set_port_value(&mut unloaded, sink, port, value);
        }
        unloaded.set_driver_mode(SimulationDriver::Manual).start();
        assert_eq!(unloaded.advance_samples(1).unwrap(), 1);
        unloaded.stop();
        assert!(matches!(
            unloaded.load_patch(Path::new("/missing-unloaded-pending")),
            Err(CoreError::PendingMidi { count: 1 })
        ));
        assert_eq!(unloaded.sample_time().completed_samples(), 1);
        assert_eq!(
            unloaded
                .world()
                .resource::<MidiOutputBuffer>()
                .pending_count(),
            1
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn installed_sink_without_sink_instances_keeps_grouped_advancement() {
        let mut rack = Oxurack::new();
        rack.try_install::<MidiOutModule>().unwrap();
        rack.set_driver_mode(SimulationDriver::Manual).start();
        assert_eq!(rack.advance_samples(3).unwrap(), 3);
        assert!(rack.drain_midi().next().is_none());
    }

    #[test]
    fn ordinary_stop_preserves_pending_output_and_resumes_after_drain() {
        let mut rack = Oxurack::new();
        rack.try_install::<MidiOutModule>().unwrap();
        let sink = MidiOutModule::spawn(rack.world_mut(), "sink", &HashMap::new()).unwrap();
        for (port, value) in [
            ("step", Value::Gate(true)),
            ("note_valid", Value::Gate(true)),
            ("velocity_valid", Value::Gate(true)),
            ("gate", Value::Gate(true)),
            ("note", Value::Raw(60)),
            ("velocity", Value::Raw(100)),
        ] {
            set_port_value(&mut rack, sink, port, value);
        }
        rack.set_driver_mode(SimulationDriver::Manual).start();
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        rack.try_install::<MidiOutModule>().unwrap();
        assert_eq!(rack.sample_time().completed_samples(), 1);
        rack.stop();
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert!(rack.midi_output_error().is_none());
        assert_eq!(
            rack.drain_midi()
                .map(|event| event.message.to_bytes())
                .collect::<Vec<_>>(),
            vec![[0x90, 60, 100]]
        );
        rack.start();
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        assert_eq!(rack.sample_time().completed_samples(), 2);
    }

    #[test]
    fn facade_reports_consume_fault_after_failing_sample_finalizes() {
        let mut rack = Oxurack::new();
        rack.try_install::<MidiOutModule>().unwrap();
        let sink = MidiOutModule::spawn(rack.world_mut(), "sink", &HashMap::new()).unwrap();
        for (port, value) in [
            ("step", Value::Gate(true)),
            ("note_valid", Value::Gate(true)),
            ("velocity_valid", Value::Gate(true)),
            ("gate", Value::Gate(true)),
            ("note", Value::Raw(128)),
            ("velocity", Value::Raw(100)),
        ] {
            set_port_value(&mut rack, sink, port, value);
        }
        rack.set_driver_mode(SimulationDriver::Manual).start();
        assert!(matches!(
            rack.advance_samples(3),
            Err(CoreError::Tick(TickError::InvalidMidiOutput {
                module,
                port,
                completed_samples: 1,
                ..
            })) if module == "sink" && port == "note"
        ));
        assert_eq!(rack.sample_time().completed_samples(), 1);
        assert!(!rack.world().resource::<SimulationRunning>().0);
        assert!(rack.drain_midi().next().is_none());
    }

    #[test]
    fn real_backpressure_fault_recovers_through_load_restart_and_unload() {
        for operation in ["load", "restart", "unload"] {
            let patch = Patch {
                version: "1.0".into(),
                master_seed: 42,
                bpm: 120.0,
                modules: vec![ModuleConfig {
                    kind: MidiOutModule::KIND.into(),
                    instance_name: "midi_main".into(),
                    parameters: HashMap::new(),
                    ..Default::default()
                }],
                cables: Vec::new(),
                rack: None,
            };
            let path = std::env::temp_dir().join(format!(
                "oxurack-real-fault-{operation}-{}.ron",
                std::process::id()
            ));
            oxurack_core::save_patch_to_file(&patch, &path).unwrap();
            let mut rack = Oxurack::new();
            rack.try_install::<MidiOutModule>().unwrap();
            rack.try_install::<FaultLateFailure>().unwrap();
            rack.set_driver_mode(SimulationDriver::Manual);
            rack.load_patch(&path).unwrap();
            rack.configure_scope(&[ScopeSelector::new("midi_main", "note")], 512)
                .unwrap();
            let sink = rack.world().resource::<PatchHandle>().modules["midi_main"];
            for (port, value) in [
                ("step", Value::Gate(true)),
                ("note_valid", Value::Gate(true)),
                ("velocity_valid", Value::Gate(true)),
                ("gate", Value::Gate(true)),
                ("note", Value::Raw(60)),
                ("velocity", Value::Raw(100)),
            ] {
                set_port_value(&mut rack, sink, port, value);
            }
            rack.start();
            for _ in 0..251 {
                assert_eq!(rack.advance_samples(1).unwrap(), 1);
                for _event in rack.drain_midi() {}
            }
            assert_eq!(rack.sample_time().completed_samples(), 251);
            let error = rack.advance_samples(2).unwrap_err();
            assert!(matches!(
                error,
                CoreError::Tick(TickError::OutputBackpressure {
                    pending_messages: 2,
                    completed_samples: 1
                })
            ));
            assert_eq!(rack.sample_time().completed_samples(), 252);
            assert!(!rack.world().resource::<SimulationRunning>().0);
            let original_fault = format!("{:?}", rack.midi_output_error().unwrap());
            let before_snapshot = rack.snapshot_patch().unwrap();
            let before_scope: Vec<_> = rack.scope().unwrap().records().copied().collect();
            let rejected = match operation {
                "load" => rack.load_patch(Path::new("/missing-real-fault-candidate")),
                "restart" => rack.restart(),
                "unload" => rack.unload_patch(),
                _ => unreachable!(),
            };
            assert!(matches!(rejected, Err(CoreError::PendingMidi { count: 2 })));
            assert_eq!(rack.snapshot_patch().unwrap(), before_snapshot);
            assert_eq!(rack.sample_time().completed_samples(), 252);
            assert_eq!(
                rack.scope().unwrap().records().copied().collect::<Vec<_>>(),
                before_scope
            );
            assert_eq!(
                format!("{:?}", rack.midi_output_error().unwrap()),
                original_fault
            );
            let ordinary = rack.drain_midi().collect::<Vec<_>>();
            assert_eq!(
                ordinary
                    .iter()
                    .map(|event| (event.sample, event.message.to_bytes()))
                    .collect::<Vec<_>>(),
                vec![(251, [0x80, 60, 0]), (251, [0x90, 60, 100])]
            );
            assert_eq!(
                format!("{:?}", rack.midi_output_error().unwrap()),
                original_fault
            );
            let retained_snapshot = rack.snapshot_patch().unwrap();
            let retained_membership = module_identities(rack.world());
            let retained_scope = rack.scope().unwrap().records().copied().collect::<Vec<_>>();
            let retained_sample = rack.sample_time();
            let retained_driver = rack.driver_mode();
            let retained_module_messages = rack
                .world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleAdded>>()
                .iter_current_update_messages()
                .map(|message| {
                    (
                        message.entity.to_bits(),
                        message.instance_name.clone(),
                        message.kind.to_string(),
                    )
                })
                .collect::<Vec<_>>();
            let retained_loaded_messages = rack
                .world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::PatchLoaded>>()
                .iter_current_update_messages()
                .map(|message| {
                    (
                        message.patch_name.clone(),
                        message.source.clone(),
                        message.module_count,
                        message.cable_count,
                        message.label_count,
                    )
                })
                .collect::<Vec<_>>();
            let mut failed_candidate = patch.clone();
            failed_candidate.modules.push(ModuleConfig {
                kind: FaultLateFailure::KIND.into(),
                instance_name: "fault_late_failure".into(),
                ..Default::default()
            });
            let failed_candidate_path = std::env::temp_dir().join(format!(
                "oxurack-real-fault-late-candidate-{operation}-{}.ron",
                std::process::id()
            ));
            oxurack_core::save_patch_to_file(&failed_candidate, &failed_candidate_path).unwrap();
            assert!(matches!(
                rack.load_patch(&failed_candidate_path),
                Err(CoreError::NotImplemented("fault candidate late failure"))
            ));
            assert!(
                rack.world()
                    .get_resource::<FaultCandidateMarker>()
                    .is_none()
            );
            assert_eq!(rack.snapshot_patch().unwrap(), retained_snapshot);
            assert_eq!(module_identities(rack.world()), retained_membership);
            assert_eq!(
                rack.scope().unwrap().records().copied().collect::<Vec<_>>(),
                retained_scope
            );
            assert_eq!(rack.sample_time(), retained_sample);
            assert_eq!(rack.driver_mode(), retained_driver);
            assert_eq!(
                format!("{:?}", rack.midi_output_error().unwrap()),
                original_fault
            );
            assert_eq!(
                rack.world()
                    .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleAdded>>()
                    .iter_current_update_messages()
                    .map(|message| (
                        message.entity.to_bits(),
                        message.instance_name.clone(),
                        message.kind.to_string()
                    ))
                    .collect::<Vec<_>>(),
                retained_module_messages
            );
            assert_eq!(
                rack.world()
                    .resource::<bevy_ecs::message::Messages<oxurack_core::PatchLoaded>>()
                    .iter_current_update_messages()
                    .map(|message| (
                        message.patch_name.clone(),
                        message.source.clone(),
                        message.module_count,
                        message.cable_count,
                        message.label_count
                    ))
                    .collect::<Vec<_>>(),
                retained_loaded_messages
            );
            let _ = std::fs::remove_file(failed_candidate_path);
            match operation {
                "load" => rack.load_patch(&path).unwrap(),
                "restart" => rack.restart().unwrap(),
                "unload" => rack.unload_patch().unwrap(),
                _ => unreachable!(),
            }
            assert_eq!(rack.sample_time().completed_samples(), 0);
            assert_eq!(rack.driver_mode(), SimulationDriver::Manual);
            assert!(rack.scope().is_none());
            assert!(rack.midi_output_error().is_none());
            assert_eq!(
                rack.drain_midi()
                    .map(|event| (event.sample, event.message.to_bytes()))
                    .collect::<Vec<_>>(),
                vec![
                    (252, [0x80, 60, 0]),
                    (252, [0xB0, 123, 0]),
                    (252, [0xB0, 120, 0]),
                ]
            );
            if operation == "unload" {
                rack.load_patch(&path).unwrap();
                let _cleanup_after_reload = rack.drain_midi().collect::<Vec<_>>();
            }
            let fresh_sink = rack.world().resource::<PatchHandle>().modules["midi_main"];
            for (port, value) in [
                ("step", Value::Gate(true)),
                ("note_valid", Value::Gate(true)),
                ("velocity_valid", Value::Gate(true)),
                ("gate", Value::Gate(true)),
                ("note", Value::Raw(60)),
                ("velocity", Value::Raw(100)),
            ] {
                set_port_value(&mut rack, fresh_sink, port, value);
            }
            rack.start();
            assert_eq!(rack.advance_samples(1).unwrap(), 1);
            assert_eq!(
                rack.drain_midi()
                    .map(|event| (event.sample, event.message.to_bytes()))
                    .collect::<Vec<_>>(),
                vec![(0, [0x90, 60, 100])]
            );
            assert!(rack.midi_output_error().is_none());
            rack.stop();
            let _ = std::fs::remove_file(path);
        }
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
