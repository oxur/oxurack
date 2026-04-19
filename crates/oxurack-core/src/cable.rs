//! Cable transforms applied to signals in transit between ports.
//!
//! A [`CableTransform`] sits on a cable and modifies the signal as it
//! flows from an output port to an input port. Transforms are
//! type-aware: each variant only works on specific [`Value`](crate::Value)
//! kinds and returns `None` for incompatible inputs.

use crate::Value;

/// A signal transform applied inline on a cable.
///
/// Each variant operates on a specific subset of [`Value`](crate::Value)
/// kinds. [`CableTransform::apply`] returns `None` when the input kind
/// is not supported by the transform.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CableTransform {
    /// Linear transform: `out = in * factor + offset`.
    ///
    /// Works on [`Value::Float`](crate::Value::Float) and
    /// [`Value::Bipolar`](crate::Value::Bipolar).
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
    /// Works on [`Value::Float`](crate::Value::Float) and
    /// [`Value::Bipolar`](crate::Value::Bipolar).
    Clamp {
        /// Lower bound.
        min: f32,
        /// Upper bound.
        max: f32,
    },

    /// Convert a float to a gate by thresholding.
    ///
    /// Works on [`Value::Float`](crate::Value::Float).
    /// `out = Gate(in >= threshold)`
    Threshold {
        /// Threshold value.
        threshold: f32,
    },

    /// Convert a gate to a float.
    ///
    /// Works on [`Value::Gate`](crate::Value::Gate).
    /// `out = Float(if gate { 1.0 } else { 0.0 })`
    GateToFloat,

    /// Convert unipolar float (0..1) to bipolar (-1..1).
    ///
    /// Works on [`Value::Float`](crate::Value::Float).
    /// `out = Bipolar(in * 2.0 - 1.0)`
    Unipolar,

    /// Convert bipolar (-1..1) to unipolar float (0..1).
    ///
    /// Works on [`Value::Bipolar`](crate::Value::Bipolar).
    /// `out = Float((in + 1.0) / 2.0)`
    Bipolarize,
}

impl CableTransform {
    /// Apply this transform to the given input value.
    ///
    /// Returns `Some(output)` if the transform is applicable to the
    /// input's kind, or `None` if the combination is unsupported.
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
            (Self::Clamp { min, max }, Value::Float(v)) => {
                Some(Value::Float(v.clamp(*min, *max)))
            }
            (Self::Clamp { min, max }, Value::Bipolar(v)) => {
                Some(Value::Bipolar(v.clamp(*min, *max)))
            }

            // ── Threshold ──────────────────────────────────────
            (Self::Threshold { threshold }, Value::Float(v)) => {
                Some(Value::Gate(v >= *threshold))
            }

            // ── GateToFloat ────────────────────────────────────
            (Self::GateToFloat, Value::Gate(b)) => {
                Some(Value::Float(if b { 1.0 } else { 0.0 }))
            }

            // ── Unipolar (Float -> Bipolar) ────────────────────
            (Self::Unipolar, Value::Float(v)) => Some(Value::Bipolar(v * 2.0 - 1.0)),

            // ── Bipolarize (Bipolar -> Float) ──────────────────
            (Self::Bipolarize, Value::Bipolar(v)) => Some(Value::Float((v + 1.0) / 2.0)),

            // ── Everything else is unsupported ─────────────────
            _ => None,
        }
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
        assert_eq!(
            t.apply(Value::Midi(crate::value::MidiMessage::Clock)),
            None
        );
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
        let t = CableTransform::Clamp {
            min: 0.2,
            max: 0.8,
        };
        assert_eq!(t.apply(Value::Float(0.5)), Some(Value::Float(0.5)));
    }

    #[test]
    fn test_clamp_float_below() {
        let t = CableTransform::Clamp {
            min: 0.2,
            max: 0.8,
        };
        assert_eq!(t.apply(Value::Float(0.1)), Some(Value::Float(0.2)));
    }

    #[test]
    fn test_clamp_float_above() {
        let t = CableTransform::Clamp {
            min: 0.2,
            max: 0.8,
        };
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
        let t = CableTransform::Clamp {
            min: 0.0,
            max: 1.0,
        };
        assert_eq!(t.apply(Value::Gate(true)), None);
    }

    #[test]
    fn test_clamp_midi_none() {
        let t = CableTransform::Clamp {
            min: 0.0,
            max: 1.0,
        };
        assert_eq!(
            t.apply(Value::Midi(crate::value::MidiMessage::Clock)),
            None
        );
    }

    #[test]
    fn test_clamp_raw_none() {
        let t = CableTransform::Clamp {
            min: 0.0,
            max: 1.0,
        };
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
        assert_eq!(
            t.apply(Value::Midi(crate::value::MidiMessage::Clock)),
            None
        );
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
        assert_eq!(
            CableTransform::GateToFloat.apply(Value::Bipolar(0.5)),
            None
        );
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
}
