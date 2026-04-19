//! Port types for module inputs and outputs.
//!
//! Every module exposes a set of named ports, each carrying signals of a
//! particular [`ValueKind`](crate::ValueKind). Input ports additionally
//! declare a [`MergePolicy`] that governs how multiple incoming cables
//! are combined.

use std::fmt;

use bevy_ecs::prelude::{Component, Entity};
use bevy_ecs::world::World;
use bevy_reflect::Reflect;

use crate::value::Value;
use crate::ValueKind;

/// Port name -- a lightweight string newtype.
///
/// Port names are case-sensitive, non-empty identifiers like `"pitch"`,
/// `"gate_in"`, or `"audio_out"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct PortName(String);

impl From<&str> for PortName {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for PortName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for PortName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for PortName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Whether a port is an input or an output.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum PortDirection {
    /// The port receives signals.
    Input,
    /// The port emits signals.
    Output,
}

/// Policy for merging multiple incoming cables on a single input port.
///
/// Not every policy is valid for every [`ValueKind`]. Use
/// [`MergePolicy::is_valid_for`] to check compatibility.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum MergePolicy {
    /// Reject a second connection -- only one cable allowed.
    Reject,
    /// Arithmetic mean of all incoming values.
    Average,
    /// Sum of all incoming values.
    Sum,
    /// Maximum of all incoming values.
    Max,
    /// Round-robin interleave (for MIDI streams).
    Interleave,
    /// Last write wins -- the most recently written value is kept.
    LastWins,
}

impl MergePolicy {
    /// Returns `true` if this merge policy is valid for the given
    /// [`ValueKind`].
    ///
    /// # Validity table
    ///
    /// | Policy       | Float | Gate | Bipolar | Midi  | Raw   |
    /// |--------------|-------|------|---------|-------|-------|
    /// | Reject       | yes   | yes  | yes     | yes   | yes   |
    /// | Average      | yes   | no   | yes     | no    | no    |
    /// | Sum          | yes   | yes  | yes     | no    | no    |
    /// | Max          | yes   | yes  | yes     | no    | no    |
    /// | Interleave   | no    | no   | no      | yes   | no    |
    /// | LastWins     | yes   | yes  | yes     | yes   | yes   |
    pub fn is_valid_for(&self, kind: ValueKind) -> bool {
        match self {
            Self::Reject | Self::LastWins => true,
            Self::Average => matches!(kind, ValueKind::Float | ValueKind::Bipolar),
            Self::Sum | Self::Max => {
                matches!(kind, ValueKind::Float | ValueKind::Gate | ValueKind::Bipolar)
            }
            Self::Interleave => matches!(kind, ValueKind::Midi),
        }
    }
}

// ── ECS component types ────────────────────────────────────────────

/// A port on a module entity (child entity of the module).
#[derive(Component, Debug, Clone, Reflect)]
pub struct Port {
    /// Human-readable port name.
    pub name: PortName,
    /// Whether this port is an input or output.
    pub direction: PortDirection,
    /// The kind of signal this port carries.
    pub value_kind: ValueKind,
    /// How multiple incoming cables are merged (only relevant for inputs).
    pub merge_policy: MergePolicy,
}

/// The current value on a port, updated each tick.
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct CurrentValue(pub Value);

// ── Spawn helpers ──────────────────────────────────────────────────

/// Spawn a port entity as a child of the given module entity.
///
/// The port is created with a [`Port`] component describing its
/// characteristics and a [`CurrentValue`] initialised to the default
/// for the given [`ValueKind`].
pub fn spawn_port_on_module(
    world: &mut World,
    module_entity: Entity,
    name: &str,
    direction: PortDirection,
    value_kind: ValueKind,
    merge_policy: MergePolicy,
) -> Entity {
    let port_entity = world
        .spawn((
            Port {
                name: PortName::from(name),
                direction,
                value_kind,
                merge_policy,
            },
            CurrentValue(Value::default_for_kind(value_kind)),
        ))
        .id();

    // Make port a child of the module via the ChildOf relationship.
    world.entity_mut(module_entity).add_child(port_entity);

    port_entity
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    // ── PortName ────────────────────────────────────────────────────

    #[test]
    fn test_port_name_from_str() {
        let name = PortName::from("pitch");
        assert_eq!(name.as_ref(), "pitch");
    }

    #[test]
    fn test_port_name_from_string() {
        let name = PortName::from(String::from("gate_in"));
        assert_eq!(name.as_ref(), "gate_in");
    }

    #[test]
    fn test_port_name_display() {
        let name = PortName::from("audio_out");
        assert_eq!(format!("{name}"), "audio_out");
    }

    #[test]
    fn test_port_name_equality() {
        let a = PortName::from("cv");
        let b = PortName::from("cv");
        let c = PortName::from("gate");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_port_name_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(PortName::from("a"));
        set.insert(PortName::from("b"));
        set.insert(PortName::from("a")); // duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_port_name_debug() {
        let name = PortName::from("test");
        let debug = format!("{name:?}");
        assert!(
            debug.contains("test"),
            "expected 'test' in debug output, got: {debug}"
        );
    }

    #[test]
    fn test_port_name_clone() {
        let name = PortName::from("original");
        let cloned = name.clone();
        assert_eq!(name, cloned);
    }

    // ── PortDirection ───────────────────────────────────────────────

    #[test]
    fn test_port_direction_debug() {
        assert_eq!(format!("{:?}", PortDirection::Input), "Input");
        assert_eq!(format!("{:?}", PortDirection::Output), "Output");
    }

    #[test]
    fn test_port_direction_equality() {
        assert_eq!(PortDirection::Input, PortDirection::Input);
        assert_eq!(PortDirection::Output, PortDirection::Output);
        assert_ne!(PortDirection::Input, PortDirection::Output);
    }

    #[test]
    fn test_port_direction_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(PortDirection::Input);
        set.insert(PortDirection::Output);
        set.insert(PortDirection::Input); // duplicate
        assert_eq!(set.len(), 2);
    }

    // ── MergePolicy × ValueKind — exhaustive 30-case table ─────────

    // Reject: always valid
    #[test]
    fn test_merge_reject_float() {
        assert!(MergePolicy::Reject.is_valid_for(ValueKind::Float));
    }
    #[test]
    fn test_merge_reject_gate() {
        assert!(MergePolicy::Reject.is_valid_for(ValueKind::Gate));
    }
    #[test]
    fn test_merge_reject_bipolar() {
        assert!(MergePolicy::Reject.is_valid_for(ValueKind::Bipolar));
    }
    #[test]
    fn test_merge_reject_midi() {
        assert!(MergePolicy::Reject.is_valid_for(ValueKind::Midi));
    }
    #[test]
    fn test_merge_reject_raw() {
        assert!(MergePolicy::Reject.is_valid_for(ValueKind::Raw));
    }

    // Average: Float, Bipolar only
    #[test]
    fn test_merge_average_float() {
        assert!(MergePolicy::Average.is_valid_for(ValueKind::Float));
    }
    #[test]
    fn test_merge_average_gate() {
        assert!(!MergePolicy::Average.is_valid_for(ValueKind::Gate));
    }
    #[test]
    fn test_merge_average_bipolar() {
        assert!(MergePolicy::Average.is_valid_for(ValueKind::Bipolar));
    }
    #[test]
    fn test_merge_average_midi() {
        assert!(!MergePolicy::Average.is_valid_for(ValueKind::Midi));
    }
    #[test]
    fn test_merge_average_raw() {
        assert!(!MergePolicy::Average.is_valid_for(ValueKind::Raw));
    }

    // Sum: Float, Gate (OR), Bipolar
    #[test]
    fn test_merge_sum_float() {
        assert!(MergePolicy::Sum.is_valid_for(ValueKind::Float));
    }
    #[test]
    fn test_merge_sum_gate() {
        assert!(MergePolicy::Sum.is_valid_for(ValueKind::Gate));
    }
    #[test]
    fn test_merge_sum_bipolar() {
        assert!(MergePolicy::Sum.is_valid_for(ValueKind::Bipolar));
    }
    #[test]
    fn test_merge_sum_midi() {
        assert!(!MergePolicy::Sum.is_valid_for(ValueKind::Midi));
    }
    #[test]
    fn test_merge_sum_raw() {
        assert!(!MergePolicy::Sum.is_valid_for(ValueKind::Raw));
    }

    // Max: Float, Gate (OR), Bipolar
    #[test]
    fn test_merge_max_float() {
        assert!(MergePolicy::Max.is_valid_for(ValueKind::Float));
    }
    #[test]
    fn test_merge_max_gate() {
        assert!(MergePolicy::Max.is_valid_for(ValueKind::Gate));
    }
    #[test]
    fn test_merge_max_bipolar() {
        assert!(MergePolicy::Max.is_valid_for(ValueKind::Bipolar));
    }
    #[test]
    fn test_merge_max_midi() {
        assert!(!MergePolicy::Max.is_valid_for(ValueKind::Midi));
    }
    #[test]
    fn test_merge_max_raw() {
        assert!(!MergePolicy::Max.is_valid_for(ValueKind::Raw));
    }

    // Interleave: Midi only
    #[test]
    fn test_merge_interleave_float() {
        assert!(!MergePolicy::Interleave.is_valid_for(ValueKind::Float));
    }
    #[test]
    fn test_merge_interleave_gate() {
        assert!(!MergePolicy::Interleave.is_valid_for(ValueKind::Gate));
    }
    #[test]
    fn test_merge_interleave_bipolar() {
        assert!(!MergePolicy::Interleave.is_valid_for(ValueKind::Bipolar));
    }
    #[test]
    fn test_merge_interleave_midi() {
        assert!(MergePolicy::Interleave.is_valid_for(ValueKind::Midi));
    }
    #[test]
    fn test_merge_interleave_raw() {
        assert!(!MergePolicy::Interleave.is_valid_for(ValueKind::Raw));
    }

    // LastWins: always valid
    #[test]
    fn test_merge_last_wins_float() {
        assert!(MergePolicy::LastWins.is_valid_for(ValueKind::Float));
    }
    #[test]
    fn test_merge_last_wins_gate() {
        assert!(MergePolicy::LastWins.is_valid_for(ValueKind::Gate));
    }
    #[test]
    fn test_merge_last_wins_bipolar() {
        assert!(MergePolicy::LastWins.is_valid_for(ValueKind::Bipolar));
    }
    #[test]
    fn test_merge_last_wins_midi() {
        assert!(MergePolicy::LastWins.is_valid_for(ValueKind::Midi));
    }
    #[test]
    fn test_merge_last_wins_raw() {
        assert!(MergePolicy::LastWins.is_valid_for(ValueKind::Raw));
    }

    // ── MergePolicy misc ────────────────────────────────────────────

    #[test]
    fn test_merge_policy_debug() {
        assert_eq!(format!("{:?}", MergePolicy::Reject), "Reject");
        assert_eq!(format!("{:?}", MergePolicy::Average), "Average");
        assert_eq!(format!("{:?}", MergePolicy::Sum), "Sum");
        assert_eq!(format!("{:?}", MergePolicy::Max), "Max");
        assert_eq!(format!("{:?}", MergePolicy::Interleave), "Interleave");
        assert_eq!(format!("{:?}", MergePolicy::LastWins), "LastWins");
    }

    #[test]
    fn test_merge_policy_clone_and_eq() {
        let policy = MergePolicy::Average;
        let cloned = policy;
        assert_eq!(policy, cloned);
    }

    #[test]
    fn test_merge_policy_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(MergePolicy::Reject);
        set.insert(MergePolicy::Average);
        set.insert(MergePolicy::Sum);
        set.insert(MergePolicy::Max);
        set.insert(MergePolicy::Interleave);
        set.insert(MergePolicy::LastWins);
        assert_eq!(set.len(), 6);
    }

    // ── Port + CurrentValue component tests ────────────────────────

    #[test]
    fn test_spawn_port_component_roundtrip() {
        let mut world = World::new();
        let parent = world.spawn_empty().id();

        let port_entity = world
            .spawn((
                Port {
                    name: PortName::from("pitch"),
                    direction: PortDirection::Input,
                    value_kind: ValueKind::Float,
                    merge_policy: MergePolicy::LastWins,
                },
                CurrentValue(Value::Float(0.5)),
            ))
            .id();

        world.entity_mut(parent).add_child(port_entity);

        let port = world.entity(port_entity).get::<Port>().unwrap();
        assert_eq!(port.name, PortName::from("pitch"));
        assert_eq!(port.direction, PortDirection::Input);
        assert_eq!(port.value_kind, ValueKind::Float);
        assert_eq!(port.merge_policy, MergePolicy::LastWins);

        let cv = world.entity(port_entity).get::<CurrentValue>().unwrap();
        assert_eq!(*cv, CurrentValue(Value::Float(0.5)));
    }

    // ── spawn_port_on_module tests ─────────────────────────────────

    #[test]
    fn test_spawn_port_on_module_creates_child_with_components() {
        let mut world = World::new();
        let module_entity = world.spawn_empty().id();

        let port_entity = spawn_port_on_module(
            &mut world,
            module_entity,
            "gate_in",
            PortDirection::Input,
            ValueKind::Gate,
            MergePolicy::Reject,
        );

        // Port component is present and correct.
        let port = world.entity(port_entity).get::<Port>().unwrap();
        assert_eq!(port.name, PortName::from("gate_in"));
        assert_eq!(port.direction, PortDirection::Input);
        assert_eq!(port.value_kind, ValueKind::Gate);
        assert_eq!(port.merge_policy, MergePolicy::Reject);

        // CurrentValue is initialised to the default for Gate.
        let cv = world.entity(port_entity).get::<CurrentValue>().unwrap();
        assert_eq!(*cv, CurrentValue(Value::Gate(false)));
    }

    #[test]
    fn test_spawn_port_on_module_output() {
        let mut world = World::new();
        let module_entity = world.spawn_empty().id();

        let port_entity = spawn_port_on_module(
            &mut world,
            module_entity,
            "audio_out",
            PortDirection::Output,
            ValueKind::Bipolar,
            MergePolicy::Reject,
        );

        let port = world.entity(port_entity).get::<Port>().unwrap();
        assert_eq!(port.name, PortName::from("audio_out"));
        assert_eq!(port.direction, PortDirection::Output);
        assert_eq!(port.value_kind, ValueKind::Bipolar);

        let cv = world.entity(port_entity).get::<CurrentValue>().unwrap();
        assert_eq!(*cv, CurrentValue(Value::Bipolar(0.0)));
    }
}
