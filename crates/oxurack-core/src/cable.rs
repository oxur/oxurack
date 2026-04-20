//! Cable transforms applied to signals in transit between ports.
//!
//! A [`CableTransform`] sits on a cable and modifies the signal as it
//! flows from an output port to an input port. Transforms are
//! type-aware: each variant only works on specific [`Value`]
//! kinds and returns `None` for incompatible inputs.

use std::collections::HashMap;

use bevy_ecs::prelude::{Component, Entity, Resource};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::Value;

/// A signal transform applied inline on a cable.
///
/// Each variant operates on a specific subset of [`Value`]
/// kinds. [`CableTransform::apply`] returns `None` when the input kind
/// is not supported by the transform.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CableTransform {
    /// Linear transform: `out = in * factor + offset`.
    ///
    /// Works on [`Value::Float`] and
    /// [`Value::Bipolar`].
    Affine {
        /// Multiplicative scale factor.
        factor: f32,
        /// Additive offset.
        offset: f32,
    },

    /// Invert the signal.
    ///
    /// - Float: `out = 1.0 - in`
    /// - Bipolar: `out = -in`
    Invert,

    /// Clamp the signal to `[min, max]`.
    ///
    /// Works on [`Value::Float`] and
    /// [`Value::Bipolar`].
    Clamp {
        /// Lower bound.
        min: f32,
        /// Upper bound.
        max: f32,
    },

    /// Convert a float to a gate by thresholding.
    ///
    /// Works on [`Value::Float`].
    /// `out = Gate(in >= threshold)`
    Threshold {
        /// Threshold value.
        threshold: f32,
    },

    /// Convert a gate to a float.
    ///
    /// Works on [`Value::Gate`].
    /// `out = Float(if gate { 1.0 } else { 0.0 })`
    GateToFloat,

    /// Convert unipolar float (0..1) to bipolar (-1..1).
    ///
    /// Works on [`Value::Float`].
    /// `out = Bipolar(in * 2.0 - 1.0)`
    Unipolar,

    /// Convert bipolar (-1..1) to unipolar float (0..1).
    ///
    /// Works on [`Value::Bipolar`].
    /// `out = Float((in + 1.0) / 2.0)`
    Bipolarize,
}

impl CableTransform {
    /// Apply this transform to the given input value.
    ///
    /// Returns `Some(output)` if the transform is applicable to the
    /// input's kind, or `None` if the combination is unsupported.
    #[must_use]
    pub fn apply(&self, input: Value) -> Option<Value> {
        match (self, input) {
            // ── Affine ──────────────────────────────────────────
            (Self::Affine { factor, offset }, Value::Float(v)) => {
                Some(Value::Float(v * factor + offset))
            }
            (Self::Affine { factor, offset }, Value::Bipolar(v)) => {
                Some(Value::Bipolar(v * factor + offset))
            }

            // ── Invert ─────────────────────────────────────────
            (Self::Invert, Value::Float(v)) => Some(Value::Float(1.0 - v)),
            (Self::Invert, Value::Bipolar(v)) => Some(Value::Bipolar(-v)),

            // ── Clamp ──────────────────────────────────────────
            (Self::Clamp { min, max }, Value::Float(v)) => Some(Value::Float(v.clamp(*min, *max))),
            (Self::Clamp { min, max }, Value::Bipolar(v)) => {
                Some(Value::Bipolar(v.clamp(*min, *max)))
            }

            // ── Threshold ──────────────────────────────────────
            (Self::Threshold { threshold }, Value::Float(v)) => Some(Value::Gate(v >= *threshold)),

            // ── GateToFloat ────────────────────────────────────
            (Self::GateToFloat, Value::Gate(b)) => Some(Value::Float(if b { 1.0 } else { 0.0 })),

            // ── Unipolar (Float -> Bipolar) ────────────────────
            (Self::Unipolar, Value::Float(v)) => Some(Value::Bipolar(v * 2.0 - 1.0)),

            // ── Bipolarize (Bipolar -> Float) ──────────────────
            (Self::Bipolarize, Value::Bipolar(v)) => Some(Value::Float((v + 1.0) / 2.0)),

            // ── Everything else is unsupported ─────────────────
            _ => None,
        }
    }
}

// ── ECS component and resource types ───────────────────────────────

/// A cable connecting two port entities.
///
/// Cables live as their own entities in the ECS world, referencing the
/// source and target port entities. An optional [`CableTransform`]
/// modifies the signal in transit.
#[derive(Component, Debug, Clone)]
pub struct Cable {
    /// The output port entity this cable reads from.
    pub source_port: Entity,
    /// The input port entity this cable writes to.
    pub target_port: Entity,
    /// Optional inline signal transform.
    pub transform: Option<CableTransform>,
    /// Whether this cable is currently active.
    pub enabled: bool,
}

/// Index for fast cable lookup by source or target port.
///
/// This resource maintains two hash maps so that cable queries by port
/// entity are O(1) amortised instead of requiring a full scan.
#[derive(Resource, Default, Debug)]
pub struct CableIndex {
    by_target: HashMap<Entity, SmallVec<[Entity; 4]>>,
    by_source: HashMap<Entity, SmallVec<[Entity; 4]>>,
}

impl CableIndex {
    /// Returns the cable entities whose target is `port`.
    pub fn cables_targeting(&self, port: Entity) -> &[Entity] {
        self.by_target.get(&port).map_or(&[], |v| v.as_slice())
    }

    /// Returns the cable entities whose source is `port`.
    pub fn cables_from(&self, port: Entity) -> &[Entity] {
        self.by_source.get(&port).map_or(&[], |v| v.as_slice())
    }

    /// Register a cable entity in the index.
    pub fn add_cable(&mut self, cable_entity: Entity, cable: &Cable) {
        self.by_target
            .entry(cable.target_port)
            .or_default()
            .push(cable_entity);
        self.by_source
            .entry(cable.source_port)
            .or_default()
            .push(cable_entity);
    }

    /// Remove a cable entity from the index.
    pub fn remove_cable(&mut self, cable_entity: Entity, cable: &Cable) {
        if let Some(v) = self.by_target.get_mut(&cable.target_port) {
            v.retain(|e| *e != cable_entity);
        }
        if let Some(v) = self.by_source.get_mut(&cable.source_port) {
            v.retain(|e| *e != cable_entity);
        }
    }

    /// Returns an iterator over all target port entities that have at
    /// least one cable connected.
    pub fn target_ports(&self) -> impl Iterator<Item = Entity> + '_ {
        self.by_target.keys().copied()
    }

    /// Clear the entire index.
    pub fn clear(&mut self) {
        self.by_target.clear();
        self.by_source.clear();
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    // ── Affine ──────────────────────────────────────────────────────

    #[test]
    fn test_affine_float() {
        // 0.5 * 2.0 + 0.25 = 1.25 (exact in IEEE 754)
        let t = CableTransform::Affine {
            factor: 2.0,
            offset: 0.25,
        };
        assert_eq!(t.apply(Value::Float(0.5)), Some(Value::Float(1.25)));
    }

    #[test]
    fn test_affine_bipolar() {
        let t = CableTransform::Affine {
            factor: 0.5,
            offset: 0.0,
        };
        assert_eq!(t.apply(Value::Bipolar(1.0)), Some(Value::Bipolar(0.5)));
    }

    #[test]
    fn test_affine_gate_none() {
        let t = CableTransform::Affine {
            factor: 1.0,
            offset: 0.0,
        };
        assert_eq!(t.apply(Value::Gate(true)), None);
    }

    #[test]
    fn test_affine_midi_none() {
        let t = CableTransform::Affine {
            factor: 1.0,
            offset: 0.0,
        };
        assert_eq!(t.apply(Value::Midi(crate::value::MidiMessage::Clock)), None);
    }

    #[test]
    fn test_affine_raw_none() {
        let t = CableTransform::Affine {
            factor: 1.0,
            offset: 0.0,
        };
        assert_eq!(t.apply(Value::Raw(42)), None);
    }

    // ── Invert ─────────────────────────────────────────────────────

    #[test]
    fn test_invert_float() {
        // 1.0 - 0.25 = 0.75 (exact in IEEE 754)
        assert_eq!(
            CableTransform::Invert.apply(Value::Float(0.25)),
            Some(Value::Float(0.75))
        );
    }

    #[test]
    fn test_invert_bipolar() {
        assert_eq!(
            CableTransform::Invert.apply(Value::Bipolar(0.5)),
            Some(Value::Bipolar(-0.5))
        );
    }

    #[test]
    fn test_invert_gate_none() {
        assert_eq!(CableTransform::Invert.apply(Value::Gate(true)), None);
    }

    #[test]
    fn test_invert_midi_none() {
        assert_eq!(
            CableTransform::Invert.apply(Value::Midi(crate::value::MidiMessage::Start)),
            None
        );
    }

    #[test]
    fn test_invert_raw_none() {
        assert_eq!(CableTransform::Invert.apply(Value::Raw(1)), None);
    }

    // ── Clamp ──────────────────────────────────────────────────────

    #[test]
    fn test_clamp_float_within() {
        let t = CableTransform::Clamp { min: 0.2, max: 0.8 };
        assert_eq!(t.apply(Value::Float(0.5)), Some(Value::Float(0.5)));
    }

    #[test]
    fn test_clamp_float_below() {
        let t = CableTransform::Clamp { min: 0.2, max: 0.8 };
        assert_eq!(t.apply(Value::Float(0.1)), Some(Value::Float(0.2)));
    }

    #[test]
    fn test_clamp_float_above() {
        let t = CableTransform::Clamp { min: 0.2, max: 0.8 };
        assert_eq!(t.apply(Value::Float(0.9)), Some(Value::Float(0.8)));
    }

    #[test]
    fn test_clamp_bipolar() {
        let t = CableTransform::Clamp {
            min: -0.5,
            max: 0.5,
        };
        assert_eq!(t.apply(Value::Bipolar(-1.0)), Some(Value::Bipolar(-0.5)));
    }

    #[test]
    fn test_clamp_gate_none() {
        let t = CableTransform::Clamp { min: 0.0, max: 1.0 };
        assert_eq!(t.apply(Value::Gate(true)), None);
    }

    #[test]
    fn test_clamp_midi_none() {
        let t = CableTransform::Clamp { min: 0.0, max: 1.0 };
        assert_eq!(t.apply(Value::Midi(crate::value::MidiMessage::Clock)), None);
    }

    #[test]
    fn test_clamp_raw_none() {
        let t = CableTransform::Clamp { min: 0.0, max: 1.0 };
        assert_eq!(t.apply(Value::Raw(5)), None);
    }

    // ── Threshold ──────────────────────────────────────────────────

    #[test]
    fn test_threshold_above() {
        let t = CableTransform::Threshold { threshold: 0.5 };
        assert_eq!(t.apply(Value::Float(0.7)), Some(Value::Gate(true)));
    }

    #[test]
    fn test_threshold_at_boundary() {
        let t = CableTransform::Threshold { threshold: 0.5 };
        assert_eq!(t.apply(Value::Float(0.5)), Some(Value::Gate(true)));
    }

    #[test]
    fn test_threshold_below() {
        let t = CableTransform::Threshold { threshold: 0.5 };
        assert_eq!(t.apply(Value::Float(0.3)), Some(Value::Gate(false)));
    }

    #[test]
    fn test_threshold_gate_none() {
        let t = CableTransform::Threshold { threshold: 0.5 };
        assert_eq!(t.apply(Value::Gate(true)), None);
    }

    #[test]
    fn test_threshold_bipolar_none() {
        let t = CableTransform::Threshold { threshold: 0.5 };
        assert_eq!(t.apply(Value::Bipolar(0.5)), None);
    }

    #[test]
    fn test_threshold_midi_none() {
        let t = CableTransform::Threshold { threshold: 0.5 };
        assert_eq!(t.apply(Value::Midi(crate::value::MidiMessage::Clock)), None);
    }

    #[test]
    fn test_threshold_raw_none() {
        let t = CableTransform::Threshold { threshold: 0.5 };
        assert_eq!(t.apply(Value::Raw(100)), None);
    }

    // ── GateToFloat ────────────────────────────────────────────────

    #[test]
    fn test_gate_to_float_true() {
        assert_eq!(
            CableTransform::GateToFloat.apply(Value::Gate(true)),
            Some(Value::Float(1.0))
        );
    }

    #[test]
    fn test_gate_to_float_false() {
        assert_eq!(
            CableTransform::GateToFloat.apply(Value::Gate(false)),
            Some(Value::Float(0.0))
        );
    }

    #[test]
    fn test_gate_to_float_float_none() {
        assert_eq!(CableTransform::GateToFloat.apply(Value::Float(0.5)), None);
    }

    #[test]
    fn test_gate_to_float_bipolar_none() {
        assert_eq!(CableTransform::GateToFloat.apply(Value::Bipolar(0.5)), None);
    }

    #[test]
    fn test_gate_to_float_midi_none() {
        assert_eq!(
            CableTransform::GateToFloat.apply(Value::Midi(crate::value::MidiMessage::Stop)),
            None
        );
    }

    #[test]
    fn test_gate_to_float_raw_none() {
        assert_eq!(CableTransform::GateToFloat.apply(Value::Raw(0)), None);
    }

    // ── Unipolar (Float -> Bipolar) ────────────────────────────────

    #[test]
    fn test_unipolar_zero() {
        assert_eq!(
            CableTransform::Unipolar.apply(Value::Float(0.0)),
            Some(Value::Bipolar(-1.0))
        );
    }

    #[test]
    fn test_unipolar_half() {
        assert_eq!(
            CableTransform::Unipolar.apply(Value::Float(0.5)),
            Some(Value::Bipolar(0.0))
        );
    }

    #[test]
    fn test_unipolar_one() {
        assert_eq!(
            CableTransform::Unipolar.apply(Value::Float(1.0)),
            Some(Value::Bipolar(1.0))
        );
    }

    #[test]
    fn test_unipolar_gate_none() {
        assert_eq!(CableTransform::Unipolar.apply(Value::Gate(true)), None);
    }

    #[test]
    fn test_unipolar_bipolar_none() {
        assert_eq!(CableTransform::Unipolar.apply(Value::Bipolar(0.5)), None);
    }

    #[test]
    fn test_unipolar_midi_none() {
        assert_eq!(
            CableTransform::Unipolar.apply(Value::Midi(crate::value::MidiMessage::Clock)),
            None
        );
    }

    #[test]
    fn test_unipolar_raw_none() {
        assert_eq!(CableTransform::Unipolar.apply(Value::Raw(0)), None);
    }

    // ── Bipolarize (Bipolar -> Float) ──────────────────────────────

    #[test]
    fn test_bipolarize_neg_one() {
        assert_eq!(
            CableTransform::Bipolarize.apply(Value::Bipolar(-1.0)),
            Some(Value::Float(0.0))
        );
    }

    #[test]
    fn test_bipolarize_zero() {
        assert_eq!(
            CableTransform::Bipolarize.apply(Value::Bipolar(0.0)),
            Some(Value::Float(0.5))
        );
    }

    #[test]
    fn test_bipolarize_one() {
        assert_eq!(
            CableTransform::Bipolarize.apply(Value::Bipolar(1.0)),
            Some(Value::Float(1.0))
        );
    }

    #[test]
    fn test_bipolarize_float_none() {
        assert_eq!(CableTransform::Bipolarize.apply(Value::Float(0.5)), None);
    }

    #[test]
    fn test_bipolarize_gate_none() {
        assert_eq!(CableTransform::Bipolarize.apply(Value::Gate(true)), None);
    }

    #[test]
    fn test_bipolarize_midi_none() {
        assert_eq!(
            CableTransform::Bipolarize.apply(Value::Midi(crate::value::MidiMessage::Clock)),
            None
        );
    }

    #[test]
    fn test_bipolarize_raw_none() {
        assert_eq!(CableTransform::Bipolarize.apply(Value::Raw(0)), None);
    }

    // ── CableTransform misc ────────────────────────────────────────

    #[test]
    fn test_cable_transform_debug() {
        let t = CableTransform::Affine {
            factor: 1.0,
            offset: 0.0,
        };
        let debug = format!("{t:?}");
        assert!(
            debug.contains("Affine"),
            "expected 'Affine' in debug output, got: {debug}"
        );
    }

    #[test]
    fn test_cable_transform_clone_and_eq() {
        let a = CableTransform::Invert;
        let b = a;
        assert_eq!(a, b);
    }

    // ── Cable component tests ─────────────────────────────────────

    #[test]
    fn test_cable_component_roundtrip() {
        use bevy_ecs::world::World;

        let mut world = World::new();

        let src = world.spawn_empty().id();
        let tgt = world.spawn_empty().id();

        let cable_entity = world
            .spawn(Cable {
                source_port: src,
                target_port: tgt,
                transform: Some(CableTransform::Invert),
                enabled: true,
            })
            .id();

        let cable = world.entity(cable_entity).get::<Cable>().unwrap();
        assert_eq!(cable.source_port, src);
        assert_eq!(cable.target_port, tgt);
        assert_eq!(cable.transform, Some(CableTransform::Invert));
        assert!(cable.enabled);
    }

    #[test]
    fn test_cable_component_no_transform() {
        use bevy_ecs::world::World;

        let mut world = World::new();

        let src = world.spawn_empty().id();
        let tgt = world.spawn_empty().id();

        let cable_entity = world
            .spawn(Cable {
                source_port: src,
                target_port: tgt,
                transform: None,
                enabled: false,
            })
            .id();

        let cable = world.entity(cable_entity).get::<Cable>().unwrap();
        assert!(cable.transform.is_none());
        assert!(!cable.enabled);
    }

    // ── CableIndex tests ──────────────────────────────────────────

    #[test]
    fn test_cable_index_add_and_lookup() {
        use bevy_ecs::world::World;

        let mut world = World::new();
        let src_a = world.spawn_empty().id();
        let tgt_a = world.spawn_empty().id();
        let src_b = world.spawn_empty().id();

        let cable_1 = world.spawn_empty().id();
        let cable_2 = world.spawn_empty().id();
        let cable_3 = world.spawn_empty().id();

        let c1 = Cable {
            source_port: src_a,
            target_port: tgt_a,
            transform: None,
            enabled: true,
        };
        let c2 = Cable {
            source_port: src_b,
            target_port: tgt_a,
            transform: None,
            enabled: true,
        };
        let c3 = Cable {
            source_port: src_a,
            target_port: world.spawn_empty().id(),
            transform: None,
            enabled: true,
        };

        let mut index = CableIndex::default();
        index.add_cable(cable_1, &c1);
        index.add_cable(cable_2, &c2);
        index.add_cable(cable_3, &c3);

        // Two cables target tgt_a.
        let targeting = index.cables_targeting(tgt_a);
        assert_eq!(targeting.len(), 2);
        assert!(targeting.contains(&cable_1));
        assert!(targeting.contains(&cable_2));

        // Two cables originate from src_a.
        let from_a = index.cables_from(src_a);
        assert_eq!(from_a.len(), 2);
        assert!(from_a.contains(&cable_1));
        assert!(from_a.contains(&cable_3));

        // One cable from src_b.
        let from_b = index.cables_from(src_b);
        assert_eq!(from_b.len(), 1);
        assert!(from_b.contains(&cable_2));
    }

    #[test]
    fn test_cable_index_remove() {
        use bevy_ecs::world::World;

        let mut world = World::new();
        let src = world.spawn_empty().id();
        let tgt = world.spawn_empty().id();
        let cable_entity = world.spawn_empty().id();

        let cable = Cable {
            source_port: src,
            target_port: tgt,
            transform: None,
            enabled: true,
        };

        let mut index = CableIndex::default();
        index.add_cable(cable_entity, &cable);
        assert_eq!(index.cables_targeting(tgt).len(), 1);
        assert_eq!(index.cables_from(src).len(), 1);

        index.remove_cable(cable_entity, &cable);
        assert!(index.cables_targeting(tgt).is_empty());
        assert!(index.cables_from(src).is_empty());
    }

    #[test]
    fn test_cable_index_unknown_port_returns_empty() {
        use bevy_ecs::world::World;

        let mut world = World::new();
        let unknown = world.spawn_empty().id();

        let index = CableIndex::default();
        assert!(index.cables_targeting(unknown).is_empty());
        assert!(index.cables_from(unknown).is_empty());
    }

    #[test]
    fn test_cable_index_clear() {
        use bevy_ecs::world::World;

        let mut world = World::new();
        let src = world.spawn_empty().id();
        let tgt = world.spawn_empty().id();
        let cable_entity = world.spawn_empty().id();

        let cable = Cable {
            source_port: src,
            target_port: tgt,
            transform: None,
            enabled: true,
        };

        let mut index = CableIndex::default();
        index.add_cable(cable_entity, &cable);
        assert!(!index.cables_targeting(tgt).is_empty());

        index.clear();
        assert!(index.cables_targeting(tgt).is_empty());
        assert!(index.cables_from(src).is_empty());
    }

    #[test]
    fn test_cable_index_target_ports() {
        use bevy_ecs::world::World;

        let mut world = World::new();
        let src_a = world.spawn_empty().id();
        let tgt_a = world.spawn_empty().id();
        let tgt_b = world.spawn_empty().id();

        let cable_1 = world.spawn_empty().id();
        let cable_2 = world.spawn_empty().id();

        let c1 = Cable {
            source_port: src_a,
            target_port: tgt_a,
            transform: None,
            enabled: true,
        };
        let c2 = Cable {
            source_port: src_a,
            target_port: tgt_b,
            transform: None,
            enabled: true,
        };

        let mut index = CableIndex::default();
        index.add_cable(cable_1, &c1);
        index.add_cable(cable_2, &c2);

        let mut targets: Vec<Entity> = index.target_ports().collect();
        targets.sort();

        assert_eq!(targets.len(), 2);
        assert!(targets.contains(&tgt_a));
        assert!(targets.contains(&tgt_b));
    }

    #[test]
    fn test_cable_index_target_ports_empty() {
        let index = CableIndex::default();
        assert_eq!(index.target_ports().count(), 0);
    }
}
