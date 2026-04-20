//! Message types exchanged between the RT thread and the ECS world.
//!
//! These types form the ABI boundary of the lock-free queues. They are pure
//! value types — small, `Copy`, and allocation-free — designed for zero-cost
//! transfer across the queue.

/// An event produced by the RT thread for consumption by the ECS world.
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
/// Re-exported from [`oxurack_midi::MidiWire`]. Stores up to 3 bytes
/// of a MIDI message plus a length indicator. `Copy` and fits in 4
/// bytes, making it ideal for lock-free queues.
pub type MidiMessage = oxurack_midi::MidiWire;

/// A command sent from the ECS world to the RT thread.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EcsCommand {
    /// Send a MIDI message out on an output port.
    SendMidi {
        /// Index of the output port to send on.
        output_port_index: u8,
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

// MidiMessage constructors and methods (note_on, note_off, cc,
// program_change, pitch_bend, from_bytes, to_bytes) are now provided
// by oxurack_midi::MidiWire, which is re-exported above as MidiMessage.

/// Classification of a raw MIDI byte sequence.
///
/// Separates system real-time messages (clock, transport) and system
/// common messages (song position) from channel voice/mode messages.
/// This is used internally to route incoming MIDI bytes to the
/// appropriate handler in the RT thread.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum MidiClassification {
    /// MIDI Clock byte (0xF8).
    Clock,
    /// Transport Start (0xFA).
    Start,
    /// Transport Stop (0xFC).
    Stop,
    /// Transport Continue (0xFB).
    Continue,
    /// Song Position Pointer (0xF2) with 14-bit position.
    SongPosition {
        /// 14-bit song position in MIDI beats (6 clocks per beat).
        position: u16,
    },
    /// Active Sensing (0xFE) — ignored by the system.
    ActiveSensing,
    /// System Reset (0xFF) — ignored by the system.
    SystemReset,
    /// A channel voice or mode message.
    Channel(MidiMessage),
}

/// Classifies a raw MIDI byte sequence.
///
/// Returns `None` if the input is empty, the first byte is not a valid
/// status byte (< 0x80), or the message type is not handled (e.g. SysEx).
///
/// # System Real-Time Messages
///
/// Single-byte messages that can appear at any time:
/// - `0xF8` → [`MidiClassification::Clock`]
/// - `0xFA` → [`MidiClassification::Start`]
/// - `0xFB` → [`MidiClassification::Continue`]
/// - `0xFC` → [`MidiClassification::Stop`]
/// - `0xFE` → [`MidiClassification::ActiveSensing`]
/// - `0xFF` → [`MidiClassification::SystemReset`]
///
/// # System Common Messages
///
/// - `0xF2` → [`MidiClassification::SongPosition`] (2 data bytes, 7 bits
///   each, LSB first)
///
/// # Channel Messages
///
/// Status bytes `0x80..=0xEF` are delegated to [`MidiMessage::from_bytes`].
pub(crate) fn classify_midi(bytes: &[u8]) -> Option<MidiClassification> {
    let &status = bytes.first()?;

    if status < 0x80 {
        return None; // Data byte without status — running status not handled here
    }

    match status {
        0xF8 => Some(MidiClassification::Clock),
        0xFA => Some(MidiClassification::Start),
        0xFB => Some(MidiClassification::Continue),
        0xFC => Some(MidiClassification::Stop),
        0xFE => Some(MidiClassification::ActiveSensing),
        0xFF => Some(MidiClassification::SystemReset),
        0xF2 => {
            // Song Position Pointer: 2 data bytes, 7 bits each, LSB first.
            let lsb = *bytes.get(1).unwrap_or(&0);
            let msb = *bytes.get(2).unwrap_or(&0);
            let position = (lsb as u16) | ((msb as u16) << 7);
            Some(MidiClassification::SongPosition { position })
        }
        0x80..=0xEF => {
            // Channel messages: delegate to MidiMessage::from_bytes.
            let msg = MidiMessage::from_bytes(bytes)?;
            Some(MidiClassification::Channel(msg))
        }
        _ => None, // Other system messages (0xF0 SysEx, 0xF1 MTC, 0xF3 Song Select, etc.)
    }
}

/// Error codes for non-fatal conditions reported by the RT thread.
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
    /// RT priority elevation failed; the thread is running at normal
    /// OS priority. Timing jitter may be higher than expected.
    PriorityElevationFailed,
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

    // ── Classification tests ────────────────────────────────────────

    #[test]
    fn test_classify_clock() {
        assert_eq!(classify_midi(&[0xF8]), Some(MidiClassification::Clock));
    }

    #[test]
    fn test_classify_start() {
        assert_eq!(classify_midi(&[0xFA]), Some(MidiClassification::Start));
    }

    #[test]
    fn test_classify_stop() {
        assert_eq!(classify_midi(&[0xFC]), Some(MidiClassification::Stop));
    }

    #[test]
    fn test_classify_continue() {
        assert_eq!(classify_midi(&[0xFB]), Some(MidiClassification::Continue));
    }

    #[test]
    fn test_classify_active_sensing() {
        assert_eq!(
            classify_midi(&[0xFE]),
            Some(MidiClassification::ActiveSensing)
        );
    }

    #[test]
    fn test_classify_system_reset() {
        assert_eq!(
            classify_midi(&[0xFF]),
            Some(MidiClassification::SystemReset)
        );
    }

    #[test]
    fn test_classify_note_on() {
        assert_eq!(
            classify_midi(&[0x90, 60, 100]),
            Some(MidiClassification::Channel(MidiMessage {
                status: 0x90,
                data1: 60,
                data2: 100,
                length: 3,
            }))
        );
    }

    #[test]
    fn test_classify_program_change() {
        assert_eq!(
            classify_midi(&[0xC0, 42]),
            Some(MidiClassification::Channel(MidiMessage {
                status: 0xC0,
                data1: 42,
                data2: 0,
                length: 2,
            }))
        );
    }

    #[test]
    fn test_classify_song_position() {
        // LSB = 0x10, MSB = 0x02 → position = 0x10 | (0x02 << 7) = 16 + 256 = 272
        assert_eq!(
            classify_midi(&[0xF2, 0x10, 0x02]),
            Some(MidiClassification::SongPosition { position: 272 })
        );
    }

    #[test]
    fn test_classify_empty_returns_none() {
        assert_eq!(classify_midi(&[]), None);
    }

    #[test]
    fn test_classify_data_byte_returns_none() {
        assert_eq!(classify_midi(&[0x60]), None);
    }

    #[test]
    fn test_classify_sysex_returns_none() {
        assert_eq!(classify_midi(&[0xF0, 0x7E, 0xF7]), None);
    }

    // ── Additional system message classification tests ──────────────

    #[test]
    fn test_classify_mtc_quarter_frame() {
        // MTC Quarter Frame (0xF1) is not handled; returns None.
        assert_eq!(classify_midi(&[0xF1, 0x00]), None);
    }

    #[test]
    fn test_classify_song_select() {
        // Song Select (0xF3) is not handled; returns None.
        assert_eq!(classify_midi(&[0xF3, 0x00]), None);
    }

    #[test]
    fn test_classify_tune_request() {
        // Tune Request (0xF6) is not handled; returns None.
        assert_eq!(classify_midi(&[0xF6]), None);
    }

    // ── from_bytes edge cases ──────────────────────────────────────

    #[test]
    fn test_from_bytes_note_on_missing_data2() {
        // Note On with only status + data1 (no velocity byte).
        // Should still return Some, padding data2 with 0.
        let msg = MidiMessage::from_bytes(&[0x90, 60]);
        assert!(msg.is_some(), "should parse partial Note On");
        let msg = msg.unwrap();
        assert_eq!(msg.status, 0x90);
        assert_eq!(msg.data1, 60);
        assert_eq!(msg.data2, 0);
        assert_eq!(msg.length, 3);
    }

    #[test]
    fn test_from_bytes_single_status_byte() {
        // Note On with only the status byte (no data bytes at all).
        // Should still return Some, padding both data bytes with 0.
        let msg = MidiMessage::from_bytes(&[0x90]);
        assert!(msg.is_some(), "should parse status-only Note On");
        let msg = msg.unwrap();
        assert_eq!(msg.status, 0x90);
        assert_eq!(msg.data1, 0);
        assert_eq!(msg.data2, 0);
        assert_eq!(msg.length, 3);
    }

    #[test]
    fn test_from_bytes_program_change_single_byte() {
        // Program Change with only the status byte.
        let msg = MidiMessage::from_bytes(&[0xC0]);
        assert!(msg.is_some(), "should parse status-only Program Change");
        let msg = msg.unwrap();
        assert_eq!(msg.status, 0xC0);
        assert_eq!(msg.data1, 0);
        assert_eq!(msg.data2, 0);
        assert_eq!(msg.length, 2);
    }

    // ── to_bytes for all message types ─────────────────────────────

    #[test]
    fn test_to_bytes_note_on() {
        let msg = MidiMessage::note_on(0, 60, 100);
        assert_eq!(msg.to_bytes(), [0x90, 60, 100]);
    }

    #[test]
    fn test_to_bytes_note_off() {
        let msg = MidiMessage::note_off(1, 64, 0);
        assert_eq!(msg.to_bytes(), [0x81, 64, 0]);
    }

    #[test]
    fn test_to_bytes_cc() {
        let msg = MidiMessage::cc(2, 74, 127);
        assert_eq!(msg.to_bytes(), [0xB2, 74, 127]);
    }

    #[test]
    fn test_to_bytes_program_change() {
        let msg = MidiMessage::program_change(5, 42);
        assert_eq!(msg.to_bytes(), [0xC5, 42, 0]);
    }

    #[test]
    fn test_to_bytes_pitch_bend() {
        let msg = MidiMessage::pitch_bend(0, 0, 64);
        assert_eq!(msg.to_bytes(), [0xE0, 0, 64]);
    }

    // ── Song Position edge cases ───────────────────────────────────

    #[test]
    fn test_classify_song_position_zero() {
        assert_eq!(
            classify_midi(&[0xF2, 0x00, 0x00]),
            Some(MidiClassification::SongPosition { position: 0 })
        );
    }

    #[test]
    fn test_classify_song_position_max() {
        // Maximum 14-bit value: LSB=0x7F, MSB=0x7F → 0x3FFF = 16383
        assert_eq!(
            classify_midi(&[0xF2, 0x7F, 0x7F]),
            Some(MidiClassification::SongPosition { position: 16383 })
        );
    }

    #[test]
    fn test_classify_song_position_missing_bytes() {
        // SPP with no data bytes: should default to position 0.
        assert_eq!(
            classify_midi(&[0xF2]),
            Some(MidiClassification::SongPosition { position: 0 })
        );
    }

    // ── Channel Pressure (0xD0) ────────────────────────────────────

    #[test]
    fn test_classify_channel_pressure() {
        // Channel Pressure is a 2-byte channel message (0xD0..0xDF).
        let result = classify_midi(&[0xD0, 100]);
        assert_eq!(
            result,
            Some(MidiClassification::Channel(MidiMessage {
                status: 0xD0,
                data1: 100,
                data2: 0,
                length: 2,
            }))
        );
    }
}
