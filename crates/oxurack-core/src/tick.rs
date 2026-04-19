//! Frame-tick scheduling phases.
//!
//! [`TickPhase`] defines the three-phase ordering for each tick of
//! the modular rack:
//!
//! 1. **Produce** -- modules generate output values.
//! 2. **Propagate** -- cables carry values from outputs to inputs.
//! 3. **Consume** -- modules read their input ports.

use bevy_ecs::schedule::SystemSet;

/// System set labels for the three phases of a rack tick.
///
/// Configured as a strict chain (`Produce -> Propagate -> Consume`)
/// inside the [`Update`](bevy_app::Update) schedule by [`CorePlugin`](crate::CorePlugin).
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TickPhase {
    /// Modules write to their output ports.
    Produce,
    /// Cables propagate values from output ports to input ports.
    Propagate,
    /// Modules read from their input ports.
    Consume,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_phase_debug() {
        assert_eq!(format!("{:?}", TickPhase::Produce), "Produce");
        assert_eq!(format!("{:?}", TickPhase::Propagate), "Propagate");
        assert_eq!(format!("{:?}", TickPhase::Consume), "Consume");
    }

    #[test]
    fn test_tick_phase_eq() {
        assert_eq!(TickPhase::Produce, TickPhase::Produce);
        assert_ne!(TickPhase::Produce, TickPhase::Propagate);
        assert_ne!(TickPhase::Propagate, TickPhase::Consume);
    }

    #[test]
    fn test_tick_phase_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(TickPhase::Produce);
        set.insert(TickPhase::Propagate);
        set.insert(TickPhase::Consume);
        set.insert(TickPhase::Produce); // duplicate
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_tick_phase_clone() {
        let phase = TickPhase::Propagate;
        let cloned = phase.clone();
        assert_eq!(phase, cloned);
    }
}
