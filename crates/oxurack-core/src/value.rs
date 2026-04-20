//! Signal values that flow through the modular rack.
//!
//! [`Value`] is the universal signal carrier: every cable, port, and parameter
//! speaks in terms of `Value`. The [`ValueKind`] discriminant enables
//! type-aware routing and merge policies without matching every variant.

/// Structured MIDI message for the ECS world.
///
/// Re-exported from [`oxurack_midi::MidiMessage`]. For the compact
/// wire format used by the RT thread, see [`oxurack_midi::MidiWire`].
pub use oxurack_midi::MidiMessage;

/// Universal signal value carried by cables and stored in ports.
///
/// # Size guarantee
///
/// `Value` is kept at 16 bytes or fewer so it can be cheaply copied
/// through the ECS without heap allocation.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    /// Unipolar float in the range 0.0..=1.0 (by convention).
    Float(f32),
    /// Boolean gate: high (`true`) or low (`false`).
    Gate(bool),
    /// Bipolar float in the range -1.0..=1.0 (by convention).
    Bipolar(f32),
    /// A MIDI message.
    Midi(MidiMessage),
    /// Raw 16-bit value (uninterpreted).
    Raw(u16),
}

/// Discriminant for [`Value`] without payload.
///
/// Useful for port declarations, merge-policy checks, and coercion
/// tables where only the *kind* of signal matters, not its data.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueKind {
    /// Corresponds to [`Value::Float`].
    Float,
    /// Corresponds to [`Value::Gate`].
    Gate,
    /// Corresponds to [`Value::Bipolar`].
    Bipolar,
    /// Corresponds to [`Value::Midi`].
    Midi,
    /// Corresponds to [`Value::Raw`].
    Raw,
}

impl Value {
    /// Returns the [`ValueKind`] discriminant for this value.
    #[must_use]
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Float(_) => ValueKind::Float,
            Self::Gate(_) => ValueKind::Gate,
            Self::Bipolar(_) => ValueKind::Bipolar,
            Self::Midi(_) => ValueKind::Midi,
            Self::Raw(_) => ValueKind::Raw,
        }
    }

    /// Attempt to coerce this value into the given `target` kind.
    ///
    /// Returns `Some(coerced)` when a well-defined conversion exists,
    /// or `None` when the conversion is undefined (e.g. anything
    /// involving [`ValueKind::Midi`] or [`ValueKind::Raw`] except
    /// identity).
    ///
    /// # Coercion table
    ///
    /// | Source   | Target   | Rule                                |
    /// |----------|----------|-------------------------------------|
    /// | *any*    | *same*   | identity -- returns `*self`         |
    /// | Float    | Gate     | `v >= 0.5`                          |
    /// | Float    | Bipolar  | `v * 2.0 - 1.0`                    |
    /// | Gate     | Float    | `true => 1.0`, `false => 0.0`      |
    /// | Gate     | Bipolar  | `true => 1.0`, `false => -1.0`     |
    /// | Bipolar  | Float    | `(v + 1.0) / 2.0`                  |
    /// | Bipolar  | Gate     | `v > 0.0`                          |
    /// | Midi/Raw | *other*  | `None`                              |
    /// | *other*  | Midi/Raw | `None`                              |
    #[must_use]
    pub fn try_coerce(&self, target: ValueKind) -> Option<Value> {
        // Identity: same kind always succeeds.
        if self.kind() == target {
            return Some(*self);
        }

        match (self, target) {
            // Float -> Gate
            (Self::Float(v), ValueKind::Gate) => Some(Self::Gate(*v >= 0.5)),
            // Float -> Bipolar
            (Self::Float(v), ValueKind::Bipolar) => Some(Self::Bipolar(*v * 2.0 - 1.0)),

            // Gate -> Float
            (Self::Gate(b), ValueKind::Float) => Some(Self::Float(if *b { 1.0 } else { 0.0 })),
            // Gate -> Bipolar
            (Self::Gate(b), ValueKind::Bipolar) => Some(Self::Bipolar(if *b { 1.0 } else { -1.0 })),

            // Bipolar -> Float
            (Self::Bipolar(v), ValueKind::Float) => Some(Self::Float((*v + 1.0) / 2.0)),
            // Bipolar -> Gate
            (Self::Bipolar(v), ValueKind::Gate) => Some(Self::Gate(*v > 0.0)),

            // Everything else (involving Midi or Raw) is undefined.
            _ => None,
        }
    }

    /// Returns a sensible default value for the given kind.
    #[must_use]
    pub fn default_for_kind(kind: ValueKind) -> Self {
        match kind {
            ValueKind::Float => Self::Float(0.0),
            ValueKind::Gate => Self::Gate(false),
            ValueKind::Bipolar => Self::Bipolar(0.0),
            ValueKind::Midi => Self::Midi(MidiMessage::Clock),
            ValueKind::Raw => Self::Raw(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    // ── kind() ──────────────────────────────────────────────────────

    #[test]
    fn test_kind_float() {
        assert_eq!(Value::Float(0.5).kind(), ValueKind::Float);
    }

    #[test]
    fn test_kind_gate() {
        assert_eq!(Value::Gate(true).kind(), ValueKind::Gate);
    }

    #[test]
    fn test_kind_bipolar() {
        assert_eq!(Value::Bipolar(-0.5).kind(), ValueKind::Bipolar);
    }

    #[test]
    fn test_kind_midi() {
        assert_eq!(Value::Midi(MidiMessage::Clock).kind(), ValueKind::Midi);
    }

    #[test]
    fn test_kind_raw() {
        assert_eq!(Value::Raw(42).kind(), ValueKind::Raw);
    }

    // ── default_for_kind() ──────────────────────────────────────────

    #[test]
    fn test_default_for_kind_float() {
        assert_eq!(Value::default_for_kind(ValueKind::Float), Value::Float(0.0));
    }

    #[test]
    fn test_default_for_kind_gate() {
        assert_eq!(Value::default_for_kind(ValueKind::Gate), Value::Gate(false));
    }

    #[test]
    fn test_default_for_kind_bipolar() {
        assert_eq!(
            Value::default_for_kind(ValueKind::Bipolar),
            Value::Bipolar(0.0)
        );
    }

    #[test]
    fn test_default_for_kind_midi() {
        assert_eq!(
            Value::default_for_kind(ValueKind::Midi),
            Value::Midi(MidiMessage::Clock)
        );
    }

    #[test]
    fn test_default_for_kind_raw() {
        assert_eq!(Value::default_for_kind(ValueKind::Raw), Value::Raw(0));
    }

    // ── try_coerce() — identity ─────────────────────────────────────

    #[test]
    fn test_coerce_float_to_float() {
        let v = Value::Float(0.7);
        assert_eq!(v.try_coerce(ValueKind::Float), Some(v));
    }

    #[test]
    fn test_coerce_gate_to_gate() {
        let v = Value::Gate(true);
        assert_eq!(v.try_coerce(ValueKind::Gate), Some(v));
    }

    #[test]
    fn test_coerce_bipolar_to_bipolar() {
        let v = Value::Bipolar(-0.3);
        assert_eq!(v.try_coerce(ValueKind::Bipolar), Some(v));
    }

    #[test]
    fn test_coerce_midi_to_midi() {
        let v = Value::Midi(MidiMessage::Start);
        assert_eq!(v.try_coerce(ValueKind::Midi), Some(v));
    }

    #[test]
    fn test_coerce_raw_to_raw() {
        let v = Value::Raw(1000);
        assert_eq!(v.try_coerce(ValueKind::Raw), Some(v));
    }

    // ── try_coerce() — Float -> others ──────────────────────────────

    #[test]
    fn test_coerce_float_to_gate_high() {
        assert_eq!(
            Value::Float(0.5).try_coerce(ValueKind::Gate),
            Some(Value::Gate(true))
        );
    }

    #[test]
    fn test_coerce_float_to_gate_low() {
        assert_eq!(
            Value::Float(0.49).try_coerce(ValueKind::Gate),
            Some(Value::Gate(false))
        );
    }

    #[test]
    fn test_coerce_float_to_bipolar() {
        // 0.75 * 2 - 1 = 0.5
        assert_eq!(
            Value::Float(0.75).try_coerce(ValueKind::Bipolar),
            Some(Value::Bipolar(0.5))
        );
    }

    #[test]
    fn test_coerce_float_to_bipolar_zero() {
        // 0.5 * 2 - 1 = 0.0
        assert_eq!(
            Value::Float(0.5).try_coerce(ValueKind::Bipolar),
            Some(Value::Bipolar(0.0))
        );
    }

    #[test]
    fn test_coerce_float_to_midi_none() {
        assert_eq!(Value::Float(0.5).try_coerce(ValueKind::Midi), None);
    }

    #[test]
    fn test_coerce_float_to_raw_none() {
        assert_eq!(Value::Float(0.5).try_coerce(ValueKind::Raw), None);
    }

    // ── try_coerce() — Gate -> others ───────────────────────────────

    #[test]
    fn test_coerce_gate_true_to_float() {
        assert_eq!(
            Value::Gate(true).try_coerce(ValueKind::Float),
            Some(Value::Float(1.0))
        );
    }

    #[test]
    fn test_coerce_gate_false_to_float() {
        assert_eq!(
            Value::Gate(false).try_coerce(ValueKind::Float),
            Some(Value::Float(0.0))
        );
    }

    #[test]
    fn test_coerce_gate_true_to_bipolar() {
        assert_eq!(
            Value::Gate(true).try_coerce(ValueKind::Bipolar),
            Some(Value::Bipolar(1.0))
        );
    }

    #[test]
    fn test_coerce_gate_false_to_bipolar() {
        assert_eq!(
            Value::Gate(false).try_coerce(ValueKind::Bipolar),
            Some(Value::Bipolar(-1.0))
        );
    }

    #[test]
    fn test_coerce_gate_to_midi_none() {
        assert_eq!(Value::Gate(true).try_coerce(ValueKind::Midi), None);
    }

    #[test]
    fn test_coerce_gate_to_raw_none() {
        assert_eq!(Value::Gate(false).try_coerce(ValueKind::Raw), None);
    }

    // ── try_coerce() — Bipolar -> others ────────────────────────────

    #[test]
    fn test_coerce_bipolar_to_float() {
        // (0.5 + 1.0) / 2.0 = 0.75
        assert_eq!(
            Value::Bipolar(0.5).try_coerce(ValueKind::Float),
            Some(Value::Float(0.75))
        );
    }

    #[test]
    fn test_coerce_bipolar_neg_to_float() {
        // (-1.0 + 1.0) / 2.0 = 0.0
        assert_eq!(
            Value::Bipolar(-1.0).try_coerce(ValueKind::Float),
            Some(Value::Float(0.0))
        );
    }

    #[test]
    fn test_coerce_bipolar_to_gate_positive() {
        assert_eq!(
            Value::Bipolar(0.1).try_coerce(ValueKind::Gate),
            Some(Value::Gate(true))
        );
    }

    #[test]
    fn test_coerce_bipolar_to_gate_zero() {
        assert_eq!(
            Value::Bipolar(0.0).try_coerce(ValueKind::Gate),
            Some(Value::Gate(false))
        );
    }

    #[test]
    fn test_coerce_bipolar_to_gate_negative() {
        assert_eq!(
            Value::Bipolar(-0.5).try_coerce(ValueKind::Gate),
            Some(Value::Gate(false))
        );
    }

    #[test]
    fn test_coerce_bipolar_to_midi_none() {
        assert_eq!(Value::Bipolar(0.5).try_coerce(ValueKind::Midi), None);
    }

    #[test]
    fn test_coerce_bipolar_to_raw_none() {
        assert_eq!(Value::Bipolar(0.5).try_coerce(ValueKind::Raw), None);
    }

    // ── try_coerce() — Midi -> others ───────────────────────────────

    #[test]
    fn test_coerce_midi_to_float_none() {
        assert_eq!(
            Value::Midi(MidiMessage::Clock).try_coerce(ValueKind::Float),
            None
        );
    }

    #[test]
    fn test_coerce_midi_to_gate_none() {
        assert_eq!(
            Value::Midi(MidiMessage::Clock).try_coerce(ValueKind::Gate),
            None
        );
    }

    #[test]
    fn test_coerce_midi_to_bipolar_none() {
        assert_eq!(
            Value::Midi(MidiMessage::Clock).try_coerce(ValueKind::Bipolar),
            None
        );
    }

    #[test]
    fn test_coerce_midi_to_raw_none() {
        assert_eq!(
            Value::Midi(MidiMessage::Clock).try_coerce(ValueKind::Raw),
            None
        );
    }

    // ── try_coerce() — Raw -> others ────────────────────────────────

    #[test]
    fn test_coerce_raw_to_float_none() {
        assert_eq!(Value::Raw(42).try_coerce(ValueKind::Float), None);
    }

    #[test]
    fn test_coerce_raw_to_gate_none() {
        assert_eq!(Value::Raw(42).try_coerce(ValueKind::Gate), None);
    }

    #[test]
    fn test_coerce_raw_to_bipolar_none() {
        assert_eq!(Value::Raw(42).try_coerce(ValueKind::Bipolar), None);
    }

    #[test]
    fn test_coerce_raw_to_midi_none() {
        assert_eq!(Value::Raw(42).try_coerce(ValueKind::Midi), None);
    }

    // ── size assertion ──────────────────────────────────────────────

    #[test]
    fn test_value_size_at_most_16_bytes() {
        assert!(
            std::mem::size_of::<Value>() <= 16,
            "Value is {} bytes, expected <= 16",
            std::mem::size_of::<Value>()
        );
    }

    // ── MidiMessage coverage ────────────────────────────────────────

    #[test]
    fn test_midi_message_debug_and_clone() {
        let msgs = [
            MidiMessage::NoteOn {
                channel: 0,
                note: 60,
                velocity: 100,
            },
            MidiMessage::NoteOff {
                channel: 0,
                note: 60,
                velocity: 0,
            },
            MidiMessage::ControlChange {
                channel: 1,
                controller: 74,
                value: 64,
            },
            MidiMessage::PitchBend {
                channel: 0,
                value: -100,
            },
            MidiMessage::ProgramChange {
                channel: 0,
                program: 5,
            },
            MidiMessage::ChannelPressure {
                channel: 0,
                pressure: 80,
            },
            MidiMessage::PolyKeyPressure {
                channel: 0,
                note: 60,
                pressure: 50,
            },
            MidiMessage::Clock,
            MidiMessage::Start,
            MidiMessage::Stop,
            MidiMessage::Continue,
            MidiMessage::SongPosition { position: 42 },
            MidiMessage::SystemExclusive,
        ];

        for msg in &msgs {
            // Verify Debug works.
            let debug = format!("{msg:?}");
            assert!(!debug.is_empty());

            // Verify Clone + PartialEq.
            let cloned = *msg;
            assert_eq!(*msg, cloned);
        }
    }

    // ── ValueKind coverage ──────────────────────────────────────────

    #[test]
    fn test_value_kind_eq_and_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ValueKind::Float);
        set.insert(ValueKind::Gate);
        set.insert(ValueKind::Bipolar);
        set.insert(ValueKind::Midi);
        set.insert(ValueKind::Raw);
        assert_eq!(set.len(), 5);
    }

    #[test]
    fn test_value_kind_debug() {
        assert_eq!(format!("{:?}", ValueKind::Float), "Float");
        assert_eq!(format!("{:?}", ValueKind::Gate), "Gate");
        assert_eq!(format!("{:?}", ValueKind::Bipolar), "Bipolar");
        assert_eq!(format!("{:?}", ValueKind::Midi), "Midi");
        assert_eq!(format!("{:?}", ValueKind::Raw), "Raw");
    }
}
