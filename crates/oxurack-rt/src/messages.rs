//! Message types exchanged between the RT thread and the ECS world.
//!
//! These types form the ABI boundary of the lock-free queues. They are pure
//! value types — small, `Copy`, and allocation-free — designed for zero-cost
//! transfer across the queue.

/// An event produced by the RT thread for consumption by the ECS world.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RtEvent {
    /// A clock tick occurred at the given subdivision of a beat.
    ClockTick {
        /// MIDI clock subdivision within the beat (0..23 for 24 PPQN).
        subdivision: u8,
        /// Cumulative beat count since transport start.
        beat: u64,
        /// Current tempo in beats per minute.
        tempo_bpm: f64,
        /// Monotonic timestamp in nanoseconds when the tick occurred.
        timestamp_ns: u64,
    },

    /// A transport state change.
    Transport(TransportEvent),

    /// A MIDI message received on an input port.
    MidiInput {
        /// Index of the input port that received the message.
        input_port_index: u8,
        /// Monotonic timestamp in nanoseconds when the message arrived.
        timestamp_ns: u64,
        /// The MIDI message payload.
        message: MidiMessage,
    },

    /// A MIDI Song Position Pointer update.
    SongPosition {
        /// 14-bit song position in MIDI beats (6 clocks per beat).
        position: u16,
    },

    /// A non-fatal error that the ECS world should be aware of.
    NonFatalError(RtErrorCode),
}

/// Transport state change events.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportEvent {
    /// Playback started from the beginning.
    Start,
    /// Playback stopped.
    Stop,
    /// Playback resumed from the current position.
    Continue,
}

/// A compact MIDI message representation.
///
/// Stores up to 3 bytes of a MIDI message plus a length indicator.
/// This is `Copy` and fits in 4 bytes, making it ideal for lock-free queues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiMessage {
    /// MIDI status byte (channel message, system message, etc.).
    pub status: u8,
    /// First data byte (0 if unused).
    pub data1: u8,
    /// Second data byte (0 if unused).
    pub data2: u8,
    /// Number of valid bytes (1, 2, or 3).
    pub length: u8,
}

/// A command sent from the ECS world to the RT thread.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EcsCommand {
    /// Send a MIDI message out on an output port.
    SendMidi {
        /// Index of the output port to send on.
        output_port_index: u8,
        /// Desired send timestamp in nanoseconds (0 = immediate).
        timestamp_ns: u64,
        /// The MIDI message payload.
        message: MidiMessage,
    },

    /// Change the master clock tempo.
    SetTempo {
        /// New tempo in beats per minute.
        bpm: f64,
    },

    /// Send a MIDI transport message (Start/Stop/Continue).
    SendTransport(TransportEvent),

    /// Send a MIDI Song Position Pointer message.
    SendSongPosition {
        /// 14-bit song position in MIDI beats.
        position: u16,
    },

    /// Gracefully shut down the RT thread.
    Shutdown,
}

impl MidiMessage {
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
    /// Returns `None` if the slice is empty, the first byte is not a valid
    /// status byte (< 0x80), or the status byte indicates a system message
    /// (>= 0xF0), which are handled separately.
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
}

/// Error codes for non-fatal conditions reported by the RT thread.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtErrorCode {
    /// A MIDI output port was disconnected or became unavailable.
    OutputPortLost,
    /// A MIDI input port was disconnected or became unavailable.
    InputPortLost,
    /// The RT-to-ECS queue overflowed; some events were dropped.
    QueueOverflow,
    /// The slave clock has not yet locked to an external clock source.
    ClockNotLocked,
    /// The slave clock detected a dropout in the external clock signal.
    ClockDropout,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // ── Size assertions ──────────────────────────────────────────────

    #[test]
    fn test_midi_message_size() {
        assert_eq!(std::mem::size_of::<MidiMessage>(), 4);
    }

    #[test]
    fn test_rt_event_fits_cache_line() {
        assert!(std::mem::size_of::<RtEvent>() <= 64);
    }

    #[test]
    fn test_ecs_command_fits_cache_line() {
        assert!(std::mem::size_of::<EcsCommand>() <= 64);
    }

    // ── Compile-time trait checks ────────────────────────────────────

    fn _assert_rt_event_is_copy_send()
    where
        RtEvent: Copy + Send + 'static,
    {
    }

    fn _assert_ecs_command_is_copy_send()
    where
        EcsCommand: Copy + Send + 'static,
    {
    }

    // ── Constructor tests ────────────────────────────────────────────

    #[test]
    fn test_note_on() {
        let msg = MidiMessage::note_on(0, 60, 100);
        assert_eq!(msg.status, 0x90);
        assert_eq!(msg.data1, 60);
        assert_eq!(msg.data2, 100);
        assert_eq!(msg.length, 3);
    }

    #[test]
    fn test_note_off() {
        let msg = MidiMessage::note_off(1, 64, 0);
        assert_eq!(msg.status, 0x81);
        assert_eq!(msg.data1, 64);
        assert_eq!(msg.data2, 0);
        assert_eq!(msg.length, 3);
    }

    #[test]
    fn test_cc() {
        let msg = MidiMessage::cc(2, 74, 127);
        assert_eq!(msg.status, 0xB2);
        assert_eq!(msg.data1, 74);
        assert_eq!(msg.data2, 127);
        assert_eq!(msg.length, 3);
    }

    #[test]
    fn test_program_change() {
        let msg = MidiMessage::program_change(5, 42);
        assert_eq!(msg.status, 0xC5);
        assert_eq!(msg.data1, 42);
        assert_eq!(msg.data2, 0);
        assert_eq!(msg.length, 2);
    }

    #[test]
    fn test_pitch_bend() {
        let msg = MidiMessage::pitch_bend(0, 0, 64);
        assert_eq!(msg.status, 0xE0);
        assert_eq!(msg.data1, 0);
        assert_eq!(msg.data2, 64);
        assert_eq!(msg.length, 3);
    }

    // ── Round-trip tests ─────────────────────────────────────────────

    #[test]
    fn test_note_on_roundtrip() {
        let original = MidiMessage::note_on(3, 72, 110);
        let bytes = original.to_bytes();
        let reconstructed = MidiMessage::from_bytes(&bytes);
        assert_eq!(Some(original), reconstructed);
    }

    #[test]
    fn test_program_change_roundtrip() {
        let original = MidiMessage::program_change(7, 99);
        let bytes = original.to_bytes();
        let reconstructed = MidiMessage::from_bytes(&bytes);
        assert_eq!(Some(original), reconstructed);
    }

    #[test]
    fn test_from_bytes_empty_returns_none() {
        assert_eq!(MidiMessage::from_bytes(&[]), None);
    }

    #[test]
    fn test_from_bytes_data_byte_returns_none() {
        assert_eq!(MidiMessage::from_bytes(&[0x7F, 0x60, 0x40]), None);
    }

    #[test]
    fn test_from_bytes_system_returns_none() {
        assert_eq!(MidiMessage::from_bytes(&[0xF0, 0x7E, 0x7F]), None);
    }

    // ── to_bytes test ────────────────────────────────────────────────

    #[test]
    fn test_to_bytes_pads_short_messages() {
        let msg = MidiMessage::program_change(0, 5);
        let bytes = msg.to_bytes();
        assert_eq!(bytes, [0xC0, 5, 0]);
    }
}
