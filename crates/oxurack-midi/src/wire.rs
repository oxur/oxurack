//! Compact 4-byte MIDI message for lock-free queues.

use crate::MidiMessage;

/// Compact 4-byte MIDI message for lock-free queues.
///
/// Used by the RT thread for zero-copy, allocation-free MIDI I/O.
/// For the structured representation with named fields, see
/// [`MidiMessage`].
///
/// # Conversions
///
/// Use [`MidiWire::to_message`] to convert to the structured format.
/// Use [`MidiMessage::to_wire`] for the reverse direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiWire {
    /// MIDI status byte (channel message, system message, etc.).
    pub status: u8,
    /// First data byte (0 if unused).
    pub data1: u8,
    /// Second data byte (0 if unused).
    pub data2: u8,
    /// Number of valid bytes (1, 2, or 3).
    pub length: u8,
}

impl MidiWire {
    /// Creates a Note On message on the given channel.
    #[must_use]
    pub fn note_on(channel: u8, note: u8, velocity: u8) -> Self {
        Self {
            status: 0x90 | channel,
            data1: note,
            data2: velocity,
            length: 3,
        }
    }

    /// Creates a Note Off message on the given channel.
    #[must_use]
    pub fn note_off(channel: u8, note: u8, velocity: u8) -> Self {
        Self {
            status: 0x80 | channel,
            data1: note,
            data2: velocity,
            length: 3,
        }
    }

    /// Creates a Control Change message on the given channel.
    #[must_use]
    pub fn cc(channel: u8, controller: u8, value: u8) -> Self {
        Self {
            status: 0xB0 | channel,
            data1: controller,
            data2: value,
            length: 3,
        }
    }

    /// Creates a Program Change message on the given channel.
    #[must_use]
    pub fn program_change(channel: u8, program: u8) -> Self {
        Self {
            status: 0xC0 | channel,
            data1: program,
            data2: 0,
            length: 2,
        }
    }

    /// Creates a Pitch Bend message on the given channel.
    #[must_use]
    pub fn pitch_bend(channel: u8, lsb: u8, msb: u8) -> Self {
        Self {
            status: 0xE0 | channel,
            data1: lsb,
            data2: msb,
            length: 3,
        }
    }

    /// Parses a MIDI message from a byte slice.
    ///
    /// Returns `None` if the slice is empty, the first byte is not a
    /// valid status byte (< 0x80), or the status byte indicates a
    /// system message (>= 0xF0), which are handled separately.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let &status = bytes.first()?;

        if status < 0x80 {
            return None;
        }

        let length: u8 = match status {
            0x80..=0xBF | 0xE0..=0xEF => 3,
            0xC0..=0xDF => 2,
            // System messages (0xF0..=0xFF): not handled yet.
            _ => return None,
        };

        let data1 = if length >= 2 {
            *bytes.get(1).unwrap_or(&0)
        } else {
            0
        };

        let data2 = if length >= 3 {
            *bytes.get(2).unwrap_or(&0)
        } else {
            0
        };

        Some(Self {
            status,
            data1,
            data2,
            length,
        })
    }

    /// Serialises the message to a 3-byte array, zero-padded if shorter.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; 3] {
        [self.status, self.data1, self.data2]
    }

    /// Converts this compact wire message to the structured format.
    ///
    /// Returns `None` for unrecognised status bytes (system messages,
    /// etc.).
    #[must_use]
    pub fn to_message(&self) -> Option<MidiMessage> {
        let bytes = self.to_bytes();
        let status = bytes[0];
        let data1 = bytes[1];
        let data2 = bytes[2];
        let channel = status & 0x0F;

        match status & 0xF0 {
            0x90 if data2 > 0 => Some(MidiMessage::NoteOn {
                channel,
                note: data1,
                velocity: data2,
            }),
            0x90 => Some(MidiMessage::NoteOff {
                channel,
                note: data1,
                velocity: 0,
            }),
            0x80 => Some(MidiMessage::NoteOff {
                channel,
                note: data1,
                velocity: data2,
            }),
            0xB0 => Some(MidiMessage::ControlChange {
                channel,
                controller: data1,
                value: data2,
            }),
            0xE0 => {
                let value = ((data2 as i16) << 7 | data1 as i16) - 8192;
                Some(MidiMessage::PitchBend { channel, value })
            }
            0xC0 => Some(MidiMessage::ProgramChange {
                channel,
                program: data1,
            }),
            0xD0 => Some(MidiMessage::ChannelPressure {
                channel,
                pressure: data1,
            }),
            0xA0 => Some(MidiMessage::PolyKeyPressure {
                channel,
                note: data1,
                pressure: data2,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // ── Size assertion ─────────────────────────────────────────────

    #[test]
    fn test_midi_wire_size() {
        assert_eq!(std::mem::size_of::<MidiWire>(), 4);
    }

    // ── Constructor tests ──────────────────────────────────────────

    #[test]
    fn test_note_on() {
        let msg = MidiWire::note_on(0, 60, 100);
        assert_eq!(msg.status, 0x90);
        assert_eq!(msg.data1, 60);
        assert_eq!(msg.data2, 100);
        assert_eq!(msg.length, 3);
    }

    #[test]
    fn test_note_off() {
        let msg = MidiWire::note_off(1, 64, 0);
        assert_eq!(msg.status, 0x81);
        assert_eq!(msg.data1, 64);
        assert_eq!(msg.data2, 0);
        assert_eq!(msg.length, 3);
    }

    #[test]
    fn test_cc() {
        let msg = MidiWire::cc(2, 74, 127);
        assert_eq!(msg.status, 0xB2);
        assert_eq!(msg.data1, 74);
        assert_eq!(msg.data2, 127);
        assert_eq!(msg.length, 3);
    }

    #[test]
    fn test_program_change() {
        let msg = MidiWire::program_change(5, 42);
        assert_eq!(msg.status, 0xC5);
        assert_eq!(msg.data1, 42);
        assert_eq!(msg.data2, 0);
        assert_eq!(msg.length, 2);
    }

    #[test]
    fn test_pitch_bend() {
        let msg = MidiWire::pitch_bend(0, 0, 64);
        assert_eq!(msg.status, 0xE0);
        assert_eq!(msg.data1, 0);
        assert_eq!(msg.data2, 64);
        assert_eq!(msg.length, 3);
    }

    // ── from_bytes / to_bytes roundtrips ───────────────────────────

    #[test]
    fn test_note_on_roundtrip() {
        let original = MidiWire::note_on(3, 72, 110);
        let bytes = original.to_bytes();
        let reconstructed = MidiWire::from_bytes(&bytes);
        assert_eq!(Some(original), reconstructed);
    }

    #[test]
    fn test_program_change_roundtrip() {
        let original = MidiWire::program_change(7, 99);
        let bytes = original.to_bytes();
        let reconstructed = MidiWire::from_bytes(&bytes);
        assert_eq!(Some(original), reconstructed);
    }

    #[test]
    fn test_from_bytes_empty_returns_none() {
        assert_eq!(MidiWire::from_bytes(&[]), None);
    }

    #[test]
    fn test_from_bytes_data_byte_returns_none() {
        assert_eq!(MidiWire::from_bytes(&[0x7F, 0x60, 0x40]), None);
    }

    #[test]
    fn test_from_bytes_system_returns_none() {
        assert_eq!(MidiWire::from_bytes(&[0xF0, 0x7E, 0x7F]), None);
    }

    // ── to_bytes padding ───────────────────────────────────────────

    #[test]
    fn test_to_bytes_pads_short_messages() {
        let msg = MidiWire::program_change(0, 5);
        let bytes = msg.to_bytes();
        assert_eq!(bytes, [0xC0, 5, 0]);
    }

    #[test]
    fn test_to_bytes_note_on() {
        let msg = MidiWire::note_on(0, 60, 100);
        assert_eq!(msg.to_bytes(), [0x90, 60, 100]);
    }

    #[test]
    fn test_to_bytes_note_off() {
        let msg = MidiWire::note_off(1, 64, 0);
        assert_eq!(msg.to_bytes(), [0x81, 64, 0]);
    }

    #[test]
    fn test_to_bytes_cc() {
        let msg = MidiWire::cc(2, 74, 127);
        assert_eq!(msg.to_bytes(), [0xB2, 74, 127]);
    }

    #[test]
    fn test_to_bytes_program_change() {
        let msg = MidiWire::program_change(5, 42);
        assert_eq!(msg.to_bytes(), [0xC5, 42, 0]);
    }

    #[test]
    fn test_to_bytes_pitch_bend() {
        let msg = MidiWire::pitch_bend(0, 0, 64);
        assert_eq!(msg.to_bytes(), [0xE0, 0, 64]);
    }

    // ── from_bytes edge cases ──────────────────────────────────────

    #[test]
    fn test_from_bytes_note_on_missing_data2() {
        let msg = MidiWire::from_bytes(&[0x90, 60]);
        assert!(msg.is_some(), "should parse partial Note On");
        let msg = msg.unwrap();
        assert_eq!(msg.status, 0x90);
        assert_eq!(msg.data1, 60);
        assert_eq!(msg.data2, 0);
        assert_eq!(msg.length, 3);
    }

    #[test]
    fn test_from_bytes_single_status_byte() {
        let msg = MidiWire::from_bytes(&[0x90]);
        assert!(msg.is_some(), "should parse status-only Note On");
        let msg = msg.unwrap();
        assert_eq!(msg.status, 0x90);
        assert_eq!(msg.data1, 0);
        assert_eq!(msg.data2, 0);
        assert_eq!(msg.length, 3);
    }

    #[test]
    fn test_from_bytes_program_change_single_byte() {
        let msg = MidiWire::from_bytes(&[0xC0]);
        assert!(msg.is_some(), "should parse status-only Program Change");
        let msg = msg.unwrap();
        assert_eq!(msg.status, 0xC0);
        assert_eq!(msg.data1, 0);
        assert_eq!(msg.data2, 0);
        assert_eq!(msg.length, 2);
    }

    // ── to_message tests ───────────────────────────────────────────

    #[test]
    fn test_to_message_note_on() {
        let wire = MidiWire::note_on(1, 60, 100);
        let msg = wire.to_message().expect("should convert NoteOn");
        assert_eq!(
            msg,
            MidiMessage::NoteOn {
                channel: 1,
                note: 60,
                velocity: 100,
            }
        );
    }

    #[test]
    fn test_to_message_note_on_velocity_zero() {
        let wire = MidiWire::note_on(0, 64, 0);
        let msg = wire.to_message().expect("should convert to NoteOff");
        assert_eq!(
            msg,
            MidiMessage::NoteOff {
                channel: 0,
                note: 64,
                velocity: 0,
            }
        );
    }

    #[test]
    fn test_to_message_note_off() {
        let wire = MidiWire::note_off(2, 72, 64);
        let msg = wire.to_message().expect("should convert NoteOff");
        assert_eq!(
            msg,
            MidiMessage::NoteOff {
                channel: 2,
                note: 72,
                velocity: 64,
            }
        );
    }

    #[test]
    fn test_to_message_cc() {
        let wire = MidiWire::cc(3, 74, 127);
        let msg = wire.to_message().expect("should convert CC");
        assert_eq!(
            msg,
            MidiMessage::ControlChange {
                channel: 3,
                controller: 74,
                value: 127,
            }
        );
    }

    #[test]
    fn test_to_message_program_change() {
        let wire = MidiWire::program_change(5, 42);
        let msg = wire.to_message().expect("should convert ProgramChange");
        assert_eq!(
            msg,
            MidiMessage::ProgramChange {
                channel: 5,
                program: 42,
            }
        );
    }

    #[test]
    fn test_to_message_pitch_bend_centre() {
        // Centre position: lsb=0, msb=64 => raw = 64*128 + 0 = 8192 => value = 0
        let wire = MidiWire::pitch_bend(0, 0, 64);
        let msg = wire.to_message().expect("should convert PitchBend");
        assert_eq!(
            msg,
            MidiMessage::PitchBend {
                channel: 0,
                value: 0,
            }
        );
    }

    #[test]
    fn test_to_message_channel_pressure() {
        let wire = MidiWire {
            status: 0xD0,
            data1: 100,
            data2: 0,
            length: 2,
        };
        let msg = wire
            .to_message()
            .expect("should convert ChannelPressure");
        assert_eq!(
            msg,
            MidiMessage::ChannelPressure {
                channel: 0,
                pressure: 100,
            }
        );
    }

    #[test]
    fn test_to_message_poly_key_pressure() {
        let wire = MidiWire {
            status: 0xA1,
            data1: 60,
            data2: 80,
            length: 3,
        };
        let msg = wire
            .to_message()
            .expect("should convert PolyKeyPressure");
        assert_eq!(
            msg,
            MidiMessage::PolyKeyPressure {
                channel: 1,
                note: 60,
                pressure: 80,
            }
        );
    }

    #[test]
    fn test_to_message_unknown_returns_none() {
        let wire = MidiWire {
            status: 0xF8,
            data1: 0,
            data2: 0,
            length: 1,
        };
        assert!(wire.to_message().is_none());
    }
}
