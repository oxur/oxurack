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
pub use cable::{Cable, CableIndex, CableTransform};
pub use error::{CoreError, PatchError, TickError};
pub use module::{Module, ModuleId, ModuleKind, spawn_module_entity};
pub use port::{CurrentValue, MergePolicy, Port, PortDirection, PortName, spawn_port_on_module};
pub use value::{MidiMessage, Value, ValueKind};

// Phase 2
pub use rng::{derive_module_rng, derive_seed};
pub use tick::{
    MergeBuffers, PropagationOrder, PropagationOrderDirty, TickNow, TickOrder, TickPhase,
    compute_tick_order, mark_propagation_order_dirty,
};

// Phase 4
pub use event::dispatch_core_command;
pub use event::{
    CoreCommand, MidiInReceived, PatchLoaded, RtWarning, RtWarningCode, SongPositionChanged,
    TransportChanged, TransportState,
};
pub use module::{ModuleRegistration, ModuleRegistry, ModuleSpawner, OxurackModule, PortSchema};
pub use parameter::{ParameterName, ParameterRegistry, ParameterSchema, ParameterValue};
pub use patch::{
    CableConfig, ModuleConfig, Patch, PatchHandle, apply_patch_to_world, deserialize_patch,
    load_patch_from_file, load_patch_into_world, save_patch_to_file, serialize_patch,
    validate_patch,
};
pub use scale::Scale;

// Phase 6 (rt-bridge feature)
#[cfg(feature = "rt-bridge")]
pub use bridge::{
    MidiOutputQueue, RtBridge, convert_core_midi, convert_rt_midi, drain_rt_events_system,
    flush_midi_output_system,
};

// ── CorePlugin ──────────────────────────────────────────────────────

use bevy_app::prelude::{App, Plugin, Update};
#[cfg(feature = "rt-bridge")]
use bevy_app::prelude::{PostUpdate, PreUpdate};
use bevy_ecs::schedule::IntoScheduleConfigs;

/// Bevy plugin that registers core resources and system-set ordering.
///
/// # What it does
///
/// - Initialises the [`CableIndex`], [`MergeBuffers`], [`TickOrder`],
///   [`ParameterRegistry`], and [`ModuleRegistry`] resources.
/// - Registers the [`TickNow`], [`TransportChanged`], [`MidiInReceived`],
///   [`CoreCommand`], and [`PatchLoaded`] messages.
/// - Configures the [`TickPhase`] system sets in the [`Update`] schedule
///   as a strict chain: `Produce -> Propagate -> Consume`.
/// - Adds the propagate and consume systems to [`TickPhase::Propagate`]
///   and [`TickPhase::Consume`] respectively.
///
/// # `rt-bridge` feature
///
/// When the `rt-bridge` feature is enabled, the plugin also:
///
/// - Initialises the `MidiOutputQueue` resource.
/// - Adds `drain_rt_events_system` to `PreUpdate`.
/// - Adds `flush_midi_output_system` to `PostUpdate`.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CableIndex>()
            .init_resource::<tick::MergeBuffers>()
            .init_resource::<tick::TickOrder>()
            .init_resource::<tick::PropagationOrder>()
            .init_resource::<tick::PropagationOrderDirty>()
            .init_resource::<ParameterRegistry>()
            .init_resource::<ModuleRegistry>()
            .add_message::<tick::TickNow>()
            .add_message::<TransportChanged>()
            .add_message::<MidiInReceived>()
            .add_message::<CoreCommand>()
            .add_message::<PatchLoaded>()
            .add_message::<RtWarning>()
            .add_message::<SongPositionChanged>()
            .configure_sets(
                Update,
                (TickPhase::Produce, TickPhase::Propagate, TickPhase::Consume).chain(),
            )
            .add_systems(
                Update,
                (
                    tick::rebuild_propagation_order_system
                        .in_set(TickPhase::Propagate)
                        .before(tick::propagate_cables_system),
                    tick::propagate_cables_system.in_set(TickPhase::Propagate),
                    tick::consume_ports_system.in_set(TickPhase::Consume),
                ),
            );

        #[cfg(feature = "rt-bridge")]
        {
            app.init_resource::<bridge::MidiOutputQueue>()
                .add_systems(PreUpdate, bridge::drain_rt_events_system)
                .add_systems(PostUpdate, bridge::flush_midi_output_system);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::App;

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
