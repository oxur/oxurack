//! Structured MIDI message with named fields per message type.

use crate::MidiWire;

/// Structured MIDI message with named fields per message type.
///
/// This is the "rich" representation used in the ECS world. For the
/// compact wire format used by the RT thread, see [`MidiWire`].
///
/// # Conversions
///
/// Use [`MidiMessage::to_wire`] to convert to the compact wire format.
/// Only channel voice messages have a wire representation; system
/// messages (`Clock`, `Start`, `Stop`, `Continue`, `SongPosition`,
/// `SystemExclusive`) return `None`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MidiMessage {
    /// Note-on event.
    NoteOn {
        /// MIDI channel (0--15).
        channel: u8,
        /// Note number (0--127).
        note: u8,
        /// Velocity (0--127).
        velocity: u8,
    },
    /// Note-off event.
    NoteOff {
        /// MIDI channel (0--15).
        channel: u8,
        /// Note number (0--127).
        note: u8,
        /// Velocity (0--127).
        velocity: u8,
    },
    /// Control change (CC) event.
    ControlChange {
        /// MIDI channel (0--15).
        channel: u8,
        /// Controller number (0--127).
        controller: u8,
        /// Controller value (0--127).
        value: u8,
    },
    /// Pitch bend event.
    PitchBend {
        /// MIDI channel (0--15).
        channel: u8,
        /// 14-bit signed pitch-bend value (-8192..=8191).
        value: i16,
    },
    /// Program change event.
    ProgramChange {
        /// MIDI channel (0--15).
        channel: u8,
        /// Program number (0--127).
        program: u8,
    },
    /// Channel (aftertouch) pressure event.
    ChannelPressure {
        /// MIDI channel (0--15).
        channel: u8,
        /// Pressure value (0--127).
        pressure: u8,
    },
    /// Polyphonic key pressure event.
    PolyKeyPressure {
        /// MIDI channel (0--15).
        channel: u8,
        /// Note number (0--127).
        note: u8,
        /// Pressure value (0--127).
        pressure: u8,
    },
    /// MIDI timing clock (24 PPQN).
    Clock,
    /// MIDI start.
    Start,
    /// MIDI stop.
    Stop,
    /// MIDI continue.
    Continue,
    /// Song position pointer.
    SongPosition {
        /// Position in MIDI beats (1 beat = 6 clock ticks).
        position: u16,
    },
    /// System exclusive (data is not carried inline).
    SystemExclusive,
}

impl MidiMessage {
    /// Converts this structured message to the compact wire format.
    ///
    /// Returns `None` for system messages (`Clock`, `Start`, `Stop`,
    /// `Continue`, `SongPosition`, `SystemExclusive`) which have no
    /// channel-message representation in the wire format.
    #[must_use]
    pub fn to_wire(&self) -> Option<MidiWire> {
        match self {
            Self::NoteOn {
                channel,
                note,
                velocity,
            } => Some(MidiWire::note_on(*channel, *note, *velocity)),

            Self::NoteOff {
                channel,
                note,
                velocity,
            } => Some(MidiWire::note_off(*channel, *note, *velocity)),

            Self::ControlChange {
                channel,
                controller,
                value,
            } => Some(MidiWire::cc(*channel, *controller, *value)),

            Self::ProgramChange { channel, program } => {
                Some(MidiWire::program_change(*channel, *program))
            }

            Self::PitchBend { channel, value } => {
                let biased = (*value + 8192) as u16;
                let lsb = (biased & 0x7F) as u8;
                let msb = ((biased >> 7) & 0x7F) as u8;
                Some(MidiWire::pitch_bend(*channel, lsb, msb))
            }

            Self::ChannelPressure { channel, pressure } => Some(MidiWire {
                status: 0xD0 | channel,
                data1: *pressure,
                data2: 0,
                length: 2,
            }),

            Self::PolyKeyPressure {
                channel,
                note,
                pressure,
            } => Some(MidiWire {
                status: 0xA0 | channel,
                data1: *note,
                data2: *pressure,
                length: 3,
            }),

            // System messages have no compact channel-message representation.
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

    #[test]
    fn test_to_wire_note_on() {
        let msg = MidiMessage::NoteOn {
            channel: 1,
            note: 60,
            velocity: 100,
        };
        let wire = msg.to_wire().expect("NoteOn should convert");
        assert_eq!(wire, MidiWire::note_on(1, 60, 100));
    }

    #[test]
    fn test_to_wire_note_off() {
        let msg = MidiMessage::NoteOff {
            channel: 2,
            note: 72,
            velocity: 64,
        };
        let wire = msg.to_wire().expect("NoteOff should convert");
        assert_eq!(wire, MidiWire::note_off(2, 72, 64));
    }

    #[test]
    fn test_to_wire_cc() {
        let msg = MidiMessage::ControlChange {
            channel: 3,
            controller: 74,
            value: 127,
        };
        let wire = msg.to_wire().expect("CC should convert");
        assert_eq!(wire, MidiWire::cc(3, 74, 127));
    }

    #[test]
    fn test_to_wire_program_change() {
        let msg = MidiMessage::ProgramChange {
            channel: 5,
            program: 42,
        };
        let wire = msg.to_wire().expect("ProgramChange should convert");
        assert_eq!(wire, MidiWire::program_change(5, 42));
    }

    #[test]
    fn test_to_wire_pitch_bend_centre() {
        let msg = MidiMessage::PitchBend {
            channel: 0,
            value: 0,
        };
        let wire = msg.to_wire().expect("PitchBend should convert");
        // Centre: value 0 => biased = 8192 => lsb = 0, msb = 64
        assert_eq!(wire, MidiWire::pitch_bend(0, 0, 64));
    }

    #[test]
    fn test_to_wire_channel_pressure() {
        let msg = MidiMessage::ChannelPressure {
            channel: 0,
            pressure: 100,
        };
        let wire = msg.to_wire().expect("ChannelPressure should convert");
        assert_eq!(wire.status, 0xD0);
        assert_eq!(wire.data1, 100);
    }

    #[test]
    fn test_to_wire_poly_key_pressure() {
        let msg = MidiMessage::PolyKeyPressure {
            channel: 1,
            note: 60,
            pressure: 80,
        };
        let wire = msg.to_wire().expect("PolyKeyPressure should convert");
        assert_eq!(wire.status, 0xA1);
        assert_eq!(wire.data1, 60);
        assert_eq!(wire.data2, 80);
    }

    #[test]
    fn test_to_wire_system_messages_return_none() {
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
}
