//! ECS world, values, ports, and cables for oxurack.
//!
//! `oxurack-core` defines the foundational types for the oxurack modular
//! synthesiser: signal values (audio CV, gates, MIDI), port descriptors,
//! cable transforms, module identifiers, and error types.
//!
//! # Phase 1 modules
//!
//! - [`value`] -- signal values and coercion
//! - [`port`] -- port names, directions, merge policies, and the `Port` component
//! - [`cable`] -- cable transforms, the `Cable` component, and `CableIndex`
//! - [`module`] -- module kind, ID, and the `Module` component
//! - [`error`] -- error types
//!
//! # Phase 2 modules
//!
//! - [`tick`] -- frame-tick scheduling phases ([`TickPhase`]), merge
//!   buffers, topological ordering, and tick systems
//! - [`rng`] -- deterministic seed derivation and module-local RNG
//!
//! # Phase 4 modules
//!
//! - [`parameter`] -- module parameter descriptors, values, and registry
//! - [`scale`] -- musical scales and quantisation helpers
//! - [`event`] -- ECS messages for transport, MIDI input, and commands
//!
//! # Phase 5 modules
//!
//! - [`patch`] -- patch persistence: data structures, validation, RON
//!   serialisation, and file I/O
//!
//! # Phase 6 modules (feature-gated)
//!
//! - `bridge` -- RT bridge for converting real-time MIDI messages to
//!   ECS messages and flushing outbound commands (requires the
//!   `rt-bridge` feature)

pub mod cable;
pub mod error;
pub mod event;
pub mod label;
pub mod module;
pub mod parameter;
pub mod port;
pub mod rng;
pub mod scale;
pub mod tick;
pub mod value;

pub mod patch;

#[cfg(feature = "rt-bridge")]
pub mod bridge;

// ── Re-exports ──────────────────────────────────────────────────────

// Phase 1
pub use cable::{Cable, CableIndex, CableOrder, CableTransform};
pub use error::{CoreError, PatchError, TickError};
pub use module::{Module, ModuleId, ModuleKind, spawn_module_entity};
pub use port::{CurrentValue, MergePolicy, Port, PortDirection, PortName, spawn_port_on_module};
pub use value::{MidiMessage, Value, ValueKind};

// Phase 2
pub use rng::{derive_module_rng, derive_seed};
pub use tick::{
    MergeBuffers, PropagationOrder, PropagationOrderDirty, SampleStart, SampleTime,
    SimulationRunning, TickNow, TickOrder, TickPhase, compute_tick_order,
    mark_propagation_order_dirty,
};

// Phase 4
pub use event::{
    CableConnected, CableDisconnected, ChangeSource, CoreCommand, LabelChanged, LabelKind,
    LabelValue, MidiInReceived, ModuleAdded, ModuleRemoved, ParameterChanged, PatchLoaded,
    PatchSource, RtWarning, RtWarningCode, SongPositionChanged, TransportChanged, TransportState,
};
pub use event::{dispatch_core_command, set_module_parameter};
pub use label::{
    Description, DisplayName, DisplayNameFormatter, LabelTarget, RackMeta, Tag, TagError, Tags,
    display_name_of, label_value, set_label_value,
};
pub use module::{
    ModuleRegistration, ModuleRegistry, ModuleSpawnContext, ModuleSpawner, OxurackModule,
    PortSchema,
};
pub use parameter::{
    ParameterGetter, ParameterName, ParameterRegistry, ParameterSchema, ParameterSetter,
    ParameterValue,
};
pub use patch::{
    CableConfig, ModuleConfig, Patch, PatchHandle, RackConfig, apply_patch_to_world,
    deserialize_patch, load_patch_from_file, load_patch_into_world, save_patch_to_file,
    serialize_patch, validate_patch,
};
pub use scale::Scale;

// Phase 6 (rt-bridge feature)
#[cfg(feature = "rt-bridge")]
pub use bridge::{
    MidiOutputQueue, RtBridge, convert_core_midi, convert_rt_midi, drain_rt_events_system,
    flush_midi_output_system,
};

// ── CorePlugin ──────────────────────────────────────────────────────

use bevy_app::prelude::{App, FixedUpdate, Plugin, Update};
#[cfg(feature = "rt-bridge")]
use bevy_app::prelude::{PostUpdate, PreUpdate};
use bevy_ecs::schedule::{IntoScheduleConfigs, ScheduleLabel};

/// Bevy plugin that registers core resources and system-set ordering.
///
/// # What it does
///
/// - Initialises the [`CableIndex`], [`MergeBuffers`], [`TickOrder`],
///   [`ParameterRegistry`], and [`ModuleRegistry`] resources.
/// - Registers the [`TickNow`], [`TransportChanged`], [`MidiInReceived`],
///   [`CoreCommand`], and [`PatchLoaded`] messages.
/// - Configures the [`TickPhase`] system sets in the selected schedule as a
///   strict `Produce -> Propagate -> MergeInputs -> Consume -> Observe` chain.
/// - Gates all named sample phases on [`SimulationRunning`], latched once at
///   sample begin, and publishes [`SampleTime`] around the chain.
/// - Adds propagation to [`TickPhase::Propagate`] and port merging to
///   [`TickPhase::MergeInputs`].
///
/// For fixed-timestep simulation (e.g. 1000 Hz), use
/// [`CorePlugin::fixed()`] which targets [`FixedUpdate`] instead.
///
/// # `rt-bridge` feature
///
/// When the `rt-bridge` feature is enabled, the plugin also:
///
/// - Initialises the `MidiOutputQueue` resource.
/// - Adds `drain_rt_events_system` to `PreUpdate`.
/// - Adds `flush_midi_output_system` to `PostUpdate`.
pub struct CorePlugin;

impl CorePlugin {
    /// Returns a plugin variant that places tick systems in
    /// [`FixedUpdate`] instead of [`Update`].
    ///
    /// Use this when the simulation should run at a fixed rate
    /// (configured via `Time<Fixed>`) independent of the render
    /// frame rate.
    pub fn fixed() -> CorePluginFixed {
        CorePluginFixed
    }
}

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        build_core_plugin(app, Update);
    }
}

/// Variant of [`CorePlugin`] that places tick systems in [`FixedUpdate`].
///
/// Created via [`CorePlugin::fixed()`]. Resources and messages are
/// identical to [`CorePlugin`]; only the schedule for tick-phase
/// systems differs.
pub struct CorePluginFixed;

impl Plugin for CorePluginFixed {
    fn build(&self, app: &mut App) {
        build_core_plugin(app, FixedUpdate);
    }
}

fn build_core_plugin(app: &mut App, schedule: impl ScheduleLabel + Clone) {
    if app.world().get_resource::<SimulationRunning>().is_none() {
        app.insert_resource(SimulationRunning(true));
    }

    app.init_resource::<CableIndex>()
        .init_resource::<tick::MergeBuffers>()
        .init_resource::<tick::TickOrder>()
        .init_resource::<tick::PropagationOrder>()
        .init_resource::<tick::PropagationOrderDirty>()
        .init_resource::<tick::SampleTime>()
        .init_resource::<tick::SampleGate>()
        .init_resource::<ParameterRegistry>()
        .init_resource::<ModuleRegistry>()
        .add_message::<tick::TickNow>()
        .add_message::<TransportChanged>()
        .add_message::<MidiInReceived>()
        .add_message::<CoreCommand>()
        .add_message::<PatchLoaded>()
        .add_message::<RtWarning>()
        .add_message::<SongPositionChanged>()
        .add_message::<ModuleAdded>()
        .add_message::<ModuleRemoved>()
        .add_message::<CableConnected>()
        .add_message::<CableDisconnected>()
        .add_message::<ParameterChanged>()
        .add_message::<LabelChanged>()
        .init_resource::<RackMeta>()
        .init_resource::<DisplayNameFormatter>()
        .configure_sets(
            schedule.clone(),
            (
                TickPhase::Produce,
                TickPhase::Propagate,
                TickPhase::MergeInputs,
                TickPhase::Consume,
                TickPhase::Observe,
            )
                .chain()
                .run_if(tick::sample_is_running),
        )
        .add_systems(
            schedule.clone(),
            (
                tick::begin_sample_system
                    .in_set(tick::SampleStart)
                    .before(TickPhase::Produce),
                tick::rebuild_propagation_order_system
                    .in_set(TickPhase::Propagate)
                    .before(tick::propagate_cables_system),
                tick::propagate_cables_system.in_set(TickPhase::Propagate),
                tick::consume_ports_system.in_set(TickPhase::MergeInputs),
                tick::finalize_sample_system.after(TickPhase::Observe),
            ),
        );

    #[cfg(feature = "rt-bridge")]
    {
        app.init_resource::<bridge::MidiOutputQueue>()
            .add_systems(PreUpdate, bridge::drain_rt_events_system)
            .add_systems(PostUpdate, bridge::flush_midi_output_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::App;
    use bevy_ecs::prelude::{ResMut, Resource};
    use bevy_ecs::schedule::IntoScheduleConfigs;

    #[derive(Resource, Default, Debug, PartialEq, Eq)]
    struct PhaseCounts([u32; 5]);

    fn count_produce(mut counts: ResMut<PhaseCounts>) {
        counts.0[0] += 1;
    }

    fn count_propagate(mut counts: ResMut<PhaseCounts>) {
        counts.0[1] += 1;
    }

    fn count_merge(mut counts: ResMut<PhaseCounts>) {
        counts.0[2] += 1;
    }

    fn count_consume(mut counts: ResMut<PhaseCounts>) {
        counts.0[3] += 1;
    }

    fn count_observe(mut counts: ResMut<PhaseCounts>) {
        counts.0[4] += 1;
    }

    fn add_phase_counters(app: &mut App, schedule: impl ScheduleLabel + Clone) {
        app.init_resource::<PhaseCounts>().add_systems(
            schedule,
            (
                count_produce.in_set(TickPhase::Produce),
                count_propagate.in_set(TickPhase::Propagate),
                count_merge.in_set(TickPhase::MergeInputs),
                count_consume.in_set(TickPhase::Consume),
                count_observe.in_set(TickPhase::Observe),
            ),
        );
    }

    #[test]
    fn test_core_plugin_does_not_panic() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        // Run one update cycle to verify nothing panics.
        app.update();
    }

    #[test]
    fn test_core_plugin_registers_cable_index() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        app.update();

        // CableIndex should exist as a resource.
        let world = app.world();
        assert!(
            world.get_resource::<CableIndex>().is_some(),
            "CableIndex resource should be present after adding CorePlugin"
        );
    }

    #[test]
    fn test_core_plugin_registers_merge_buffers() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        app.update();

        let world = app.world();
        assert!(
            world.get_resource::<MergeBuffers>().is_some(),
            "MergeBuffers resource should be present after adding CorePlugin"
        );
    }

    #[test]
    fn test_core_plugin_registers_tick_order() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        app.update();

        let world = app.world();
        assert!(
            world.get_resource::<TickOrder>().is_some(),
            "TickOrder resource should be present after adding CorePlugin"
        );
    }

    #[test]
    fn test_core_plugin_registers_propagation_order() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        app.update();

        let world = app.world();
        assert!(
            world.get_resource::<PropagationOrder>().is_some(),
            "PropagationOrder resource should be present after adding CorePlugin"
        );
    }

    #[test]
    fn test_core_plugin_registers_parameter_registry() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        app.update();

        let world = app.world();
        assert!(
            world.get_resource::<ParameterRegistry>().is_some(),
            "ParameterRegistry resource should be present after adding CorePlugin"
        );
    }

    #[test]
    fn test_core_plugin_registers_module_registry() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        app.update();

        let world = app.world();
        assert!(
            world.get_resource::<ModuleRegistry>().is_some(),
            "ModuleRegistry resource should be present after adding CorePlugin"
        );
    }

    #[test]
    fn test_core_plugin_runs_five_phases_by_default_and_gates_explicit_false() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        add_phase_counters(&mut app, Update);

        app.update();
        assert_eq!(app.world().resource::<PhaseCounts>().0, [1, 1, 1, 1, 1]);
        assert_eq!(app.world().resource::<SampleTime>().last_index(), Some(0));
        assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 1);

        let mut stopped = App::new();
        stopped.insert_resource(SimulationRunning(false));
        stopped.add_plugins(CorePlugin);
        add_phase_counters(&mut stopped, Update);
        stopped.update();
        assert_eq!(stopped.world().resource::<PhaseCounts>().0, [0, 0, 0, 0, 0]);
        assert_eq!(
            *stopped.world().resource::<SampleTime>(),
            SampleTime::default()
        );

        stopped.world_mut().resource_mut::<SimulationRunning>().0 = true;
        stopped.update();
        assert_eq!(stopped.world().resource::<PhaseCounts>().0, [1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_core_plugin_fixed_runs_named_phases_and_latches_stop() {
        fn stop_in_produce(mut running: ResMut<SimulationRunning>) {
            running.0 = false;
        }

        let mut app = App::new();
        app.add_plugins(CorePlugin::fixed());
        add_phase_counters(&mut app, FixedUpdate);
        app.add_systems(FixedUpdate, stop_in_produce.in_set(TickPhase::Produce));

        app.world_mut().run_schedule(FixedUpdate);
        assert_eq!(app.world().resource::<PhaseCounts>().0, [1, 1, 1, 1, 1]);
        assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 1);

        app.world_mut().run_schedule(FixedUpdate);
        assert_eq!(app.world().resource::<PhaseCounts>().0, [1, 1, 1, 1, 1]);
        assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 1);
    }

    #[test]
    fn test_core_plugin_fixed_preserves_preinserted_false_until_enabled() {
        let mut app = App::new();
        app.insert_resource(SimulationRunning(false));
        app.add_plugins(CorePlugin::fixed());
        add_phase_counters(&mut app, FixedUpdate);

        app.world_mut().run_schedule(FixedUpdate);
        assert_eq!(app.world().resource::<PhaseCounts>().0, [0, 0, 0, 0, 0]);
        assert_eq!(*app.world().resource::<SampleTime>(), SampleTime::default());

        app.world_mut().resource_mut::<SimulationRunning>().0 = true;
        app.world_mut().run_schedule(FixedUpdate);
        assert_eq!(app.world().resource::<PhaseCounts>().0, [1, 1, 1, 1, 1]);
        assert_eq!(app.world().resource::<SampleTime>().last_index(), Some(0));
        assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 1);
    }

    #[cfg(feature = "rt-bridge")]
    #[test]
    fn test_core_plugin_registers_midi_output_queue() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        app.update();

        let world = app.world();
        assert!(
            world.get_resource::<MidiOutputQueue>().is_some(),
            "MidiOutputQueue resource should be present after adding CorePlugin with rt-bridge"
        );
    }
}
