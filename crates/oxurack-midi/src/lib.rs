//! Shared MIDI message types for the oxurack ecosystem.
//!
//! This crate defines two complementary MIDI representations:
//!
//! - [`MidiMessage`] -- a structured enum with named fields per message
//!   type, intended for ergonomic ECS-side processing.
//! - [`MidiWire`] -- a compact 4-byte struct for lock-free queues,
//!   designed for zero-copy, allocation-free RT I/O.
//!
//! Both types are `Copy` and suitable for transfer across thread
//! boundaries. Conversion methods are provided directly on the types:
//!
//! - [`MidiMessage::to_wire`] -- structured to compact.
//! - [`MidiWire::to_message`] -- compact to structured.
//!
//! # Feature flags
//!
//! - **`reflect`** -- derives [`bevy_reflect::Reflect`] on [`MidiMessage`].
//! - **`serde`** -- derives [`serde::Serialize`] and [`serde::Deserialize`]
//!   on [`MidiMessage`].

mod message;
mod wire;

pub use message::MidiMessage;
pub use wire::MidiWire;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // ── Size assertions ────────────────────────────────────────────

    #[test]
    fn test_midi_wire_is_4_bytes() {
        assert_eq!(
            std::mem::size_of::<MidiWire>(),
            4,
            "MidiWire must be exactly 4 bytes"
        );
    }

    // ── Roundtrip: MidiMessage -> MidiWire -> MidiMessage ──────────

    #[test]
    fn test_roundtrip_note_on() {
        let msg = MidiMessage::NoteOn {
            channel: 3,
            note: 72,
            velocity: 110,
        };
        let wire = msg.to_wire().expect("NoteOn should convert to wire");
        let back = wire.to_message().expect("wire should convert back");
        assert_eq!(msg, back);
    }

    #[test]
    fn test_roundtrip_note_off() {
        let msg = MidiMessage::NoteOff {
            channel: 2,
            note: 64,
            velocity: 80,
        };
        let wire = msg.to_wire().expect("NoteOff should convert to wire");
        let back = wire.to_message().expect("wire should convert back");
        assert_eq!(msg, back);
    }

    #[test]
    fn test_roundtrip_cc() {
        let msg = MidiMessage::ControlChange {
            channel: 0,
            controller: 1,
            value: 64,
        };
        let wire = msg.to_wire().expect("CC should convert to wire");
        let back = wire.to_message().expect("wire should convert back");
        assert_eq!(msg, back);
    }

    #[test]
    fn test_roundtrip_program_change() {
        let msg = MidiMessage::ProgramChange {
            channel: 7,
            program: 99,
        };
        let wire = msg.to_wire().expect("ProgramChange should convert to wire");
        let back = wire.to_message().expect("wire should convert back");
        assert_eq!(msg, back);
    }

    #[test]
    fn test_roundtrip_pitch_bend_centre() {
        let msg = MidiMessage::PitchBend {
            channel: 0,
            value: 0,
        };
        let wire = msg.to_wire().expect("PitchBend should convert to wire");
        let back = wire.to_message().expect("wire should convert back");
        assert_eq!(msg, back);
    }

    #[test]
    fn test_roundtrip_pitch_bend_max() {
        let msg = MidiMessage::PitchBend {
            channel: 0,
            value: 8191,
        };
        let wire = msg.to_wire().expect("PitchBend should convert to wire");
        let back = wire.to_message().expect("wire should convert back");
        assert_eq!(msg, back);
    }

    #[test]
    fn test_roundtrip_pitch_bend_min() {
        let msg = MidiMessage::PitchBend {
            channel: 0,
            value: -8192,
        };
        let wire = msg.to_wire().expect("PitchBend should convert to wire");
        let back = wire.to_message().expect("wire should convert back");
        assert_eq!(msg, back);
    }

    #[test]
    fn test_roundtrip_channel_pressure() {
        let msg = MidiMessage::ChannelPressure {
            channel: 0,
            pressure: 100,
        };
        let wire = msg
            .to_wire()
            .expect("ChannelPressure should convert to wire");
        let back = wire.to_message().expect("wire should convert back");
        assert_eq!(msg, back);
    }

    #[test]
    fn test_roundtrip_poly_key_pressure() {
        let msg = MidiMessage::PolyKeyPressure {
            channel: 1,
            note: 60,
            pressure: 80,
        };
        let wire = msg
            .to_wire()
            .expect("PolyKeyPressure should convert to wire");
        let back = wire.to_message().expect("wire should convert back");
        assert_eq!(msg, back);
    }

    // ── System messages have no wire representation ────────────────

    #[test]
    fn test_system_messages_to_wire_returns_none() {
        assert!(MidiMessage::Clock.to_wire().is_none());
        assert!(MidiMessage::Start.to_wire().is_none());
        assert!(MidiMessage::Stop.to_wire().is_none());
        assert!(MidiMessage::Continue.to_wire().is_none());
        assert!(
            MidiMessage::SongPosition { position: 0 }
                .to_wire()
                .is_none()
        );
        assert!(MidiMessage::SystemExclusive.to_wire().is_none());
    }

    // ── Wire roundtrip: MidiWire -> bytes -> MidiWire ──────────────

    #[test]
    fn test_wire_note_on_roundtrip() {
        let original = MidiWire::note_on(3, 72, 110);
        let bytes = original.to_bytes();
        let reconstructed = MidiWire::from_bytes(&bytes);
        assert_eq!(Some(original), reconstructed);
    }

    #[test]
    fn test_wire_program_change_roundtrip() {
        let original = MidiWire::program_change(7, 99);
        let bytes = original.to_bytes();
        let reconstructed = MidiWire::from_bytes(&bytes);
        assert_eq!(Some(original), reconstructed);
    }

    // ── Note-on with velocity 0 becomes NoteOff ────────────────────

    #[test]
    fn test_note_on_velocity_zero_becomes_note_off() {
        let wire = MidiWire::note_on(0, 64, 0);
        let msg = wire.to_message().expect("should convert");
        assert_eq!(
            msg,
            MidiMessage::NoteOff {
                channel: 0,
                note: 64,
                velocity: 0,
            }
        );
    }

    // ── Unknown status in wire returns None ────────────────────────

    #[test]
    fn test_wire_unknown_status_returns_none() {
        let wire = MidiWire {
            status: 0xF8,
            data1: 0,
            data2: 0,
            length: 1,
        };
        assert!(wire.to_message().is_none());
    }
}
