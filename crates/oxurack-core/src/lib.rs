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
//! - [`tick`] -- frame-tick scheduling phases ([`TickPhase`])
//! - [`rng`] -- deterministic seed derivation
//!
//! # Phase 2+ stubs
//!
//! - `parameter` -- module parameter descriptors
//! - `patch` -- patch graph
//! - `scale` -- musical scales
//! - `event` -- ECS events

pub mod cable;
pub mod error;
pub mod module;
pub mod port;
pub mod rng;
pub mod tick;
pub mod value;

// Phase 2+ stubs.
mod event;
mod parameter;
mod patch;
mod scale;

// ── Re-exports ──────────────────────────────────────────────────────

pub use cable::{Cable, CableIndex, CableTransform};
pub use error::{CoreError, PatchError, TickError};
pub use module::{spawn_module_entity, Module, ModuleId, ModuleKind};
pub use port::{spawn_port_on_module, CurrentValue, MergePolicy, Port, PortDirection, PortName};
pub use rng::derive_seed;
pub use tick::TickPhase;
pub use value::{MidiMessage, Value, ValueKind};

// ── CorePlugin ──────────────────────────────────────────────────────

use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::schedule::IntoScheduleConfigs;

/// Bevy plugin that registers core resources and system-set ordering.
///
/// # What it does
///
/// - Initialises the [`CableIndex`] resource.
/// - Configures the [`TickPhase`] system sets in the [`Update`] schedule
///   as a strict chain: `Produce -> Propagate -> Consume`.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CableIndex>().configure_sets(
            Update,
            (
                TickPhase::Produce,
                TickPhase::Propagate,
                TickPhase::Consume,
            )
                .chain(),
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
}
