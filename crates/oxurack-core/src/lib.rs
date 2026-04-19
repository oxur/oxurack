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

// ── Re-exports ──────────────────────────────────────────────────────

// Phase 1
pub use cable::{Cable, CableIndex, CableTransform};
pub use error::{CoreError, PatchError, TickError};
pub use module::{spawn_module_entity, Module, ModuleId, ModuleKind};
pub use port::{spawn_port_on_module, CurrentValue, MergePolicy, Port, PortDirection, PortName};
pub use value::{MidiMessage, Value, ValueKind};

// Phase 2
pub use rng::{derive_module_rng, derive_seed};
pub use tick::{compute_tick_order, MergeBuffers, TickNow, TickOrder, TickPhase};

// Phase 4
pub use event::{CoreCommand, MidiInReceived, PatchLoaded, TransportChanged, TransportState};
pub use module::{ModuleRegistration, ModuleRegistry, OxurackModule, PortSchema};
pub use parameter::{ParameterName, ParameterRegistry, ParameterSchema, ParameterValue};
pub use patch::{
    deserialize_patch, load_patch_from_file, save_patch_to_file, serialize_patch, validate_patch,
    CableConfig, ModuleConfig, Patch,
};
pub use scale::Scale;

// ── CorePlugin ──────────────────────────────────────────────────────

use bevy_app::prelude::{App, Plugin, Update};
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
/// - Adds the [`propagate_cables_system`](tick::propagate_cables_system)
///   to [`TickPhase::Propagate`] and
///   [`consume_ports_system`](tick::consume_ports_system) to
///   [`TickPhase::Consume`].
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CableIndex>()
            .init_resource::<tick::MergeBuffers>()
            .init_resource::<tick::TickOrder>()
            .init_resource::<ParameterRegistry>()
            .init_resource::<ModuleRegistry>()
            .add_message::<tick::TickNow>()
            .add_message::<TransportChanged>()
            .add_message::<MidiInReceived>()
            .add_message::<CoreCommand>()
            .add_message::<PatchLoaded>()
            .configure_sets(
                Update,
                (
                    TickPhase::Produce,
                    TickPhase::Propagate,
                    TickPhase::Consume,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    tick::propagate_cables_system.in_set(TickPhase::Propagate),
                    tick::consume_ports_system.in_set(TickPhase::Consume),
                ),
            );
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
}
