//! Bridge between the RT thread and the ECS world.
//!
//! Converts [`oxurack_rt::RtEvent`] messages into core ECS messages
//! and flushes outbound MIDI commands to the RT thread.
//!
//! This module is only available when the `rt-bridge` feature is
//! enabled. It provides:
//!
//! - [`RtBridge`] -- a Bevy resource wrapping the RT thread's queue
//!   handles.
//! - [`MidiOutputQueue`] -- a buffer for outbound MIDI commands.
//! - [`drain_rt_events_system`] -- a system that converts RT events
//!   into core ECS messages (runs in `PreUpdate`).
//! - [`flush_midi_output_system`] -- a system that flushes buffered
//!   MIDI output commands to the RT thread (runs in `PostUpdate`).
//! - [`convert_rt_midi`] / [`convert_core_midi`] -- conversion
//!   functions between the compact RT MIDI format and core's
//!   structured [`MidiMessage`](crate::MidiMessage).

use bevy_ecs::change_detection::NonSendMut;
use bevy_ecs::prelude::{MessageWriter, ResMut, Resource};

// ── Resources ───────────────────────────────────────────────────────

/// Non-send resource wrapping the RT thread's queue handles.
///
/// Insert this as a non-send resource into the Bevy app after calling
/// [`Runtime::start`](oxurack_rt::Runtime::start) via
/// `world.insert_non_send_resource(bridge)`. The bridge systems use
/// [`NonSendMut`] to access it, ensuring they run on the main thread.
pub struct RtBridge {
    /// Consumer end of the RT-to-ECS event queue.
    pub events: rtrb::Consumer<oxurack_rt::RtEvent>,
    /// Producer end of the ECS-to-RT command queue.
    pub commands: rtrb::Producer<oxurack_rt::EcsCommand>,
}

// RtBridge wraps rtrb Consumer/Producer which are Send but not Sync.
// We use NonSend/NonSendMut system parameters instead of Resource to
// avoid an unsafe Sync impl. Bevy's scheduler will ensure these
// systems run on the main thread.

/// Buffer for MIDI commands to be sent to the RT thread.
///
/// Modules push commands here during the `Produce` phase; the
/// [`flush_midi_output_system`] drains them to the RT queue in
/// `PostUpdate`.
#[derive(Resource, Default)]
pub struct MidiOutputQueue {
    /// Buffered commands awaiting flush to the RT thread.
    pub commands: Vec<oxurack_rt::EcsCommand>,
}

// ── MIDI conversion ─────────────────────────────────────────────────

/// Converts `oxurack_rt`'s compact [`MidiMessage`](oxurack_rt::MidiMessage)
/// to core's structured [`MidiMessage`](crate::MidiMessage).
///
/// Returns `None` for unrecognised status bytes (system messages, etc.).
///
/// Delegates to [`oxurack_midi::MidiWire::to_message`].
#[must_use]
pub fn convert_rt_midi(rt_msg: &oxurack_rt::MidiMessage) -> Option<crate::MidiMessage> {
    rt_msg.to_message()
}

/// Converts core's structured [`MidiMessage`](crate::MidiMessage) back
/// to `oxurack_rt`'s compact format.
///
/// Returns `None` for system messages (`Clock`, `Start`, `Stop`,
/// `Continue`, `SongPosition`, `SystemExclusive`) which have no
/// channel-message representation in the RT format.
///
/// Delegates to [`oxurack_midi::MidiMessage::to_wire`].
#[must_use]
pub fn convert_core_midi(msg: &crate::MidiMessage) -> Option<oxurack_rt::MidiMessage> {
    msg.to_wire()
}

// ── RtErrorCode conversion ──────────────────────────────────────────

impl From<oxurack_rt::RtErrorCode> for crate::RtWarningCode {
    fn from(code: oxurack_rt::RtErrorCode) -> Self {
        match code {
            oxurack_rt::RtErrorCode::PriorityElevationFailed => Self::PriorityElevationFailed,
            oxurack_rt::RtErrorCode::ClockDropout => Self::ClockDropout,
            oxurack_rt::RtErrorCode::ClockNotLocked => Self::ClockNotLocked,
            oxurack_rt::RtErrorCode::QueueOverflow => Self::QueueOverflow,
            oxurack_rt::RtErrorCode::OutputPortLost => Self::OutputPortLost,
            oxurack_rt::RtErrorCode::InputPortLost => Self::InputPortLost,
            // RtErrorCode is #[non_exhaustive], so handle unknown future variants.
            _ => Self::QueueOverflow, // fallback for unknown codes
        }
    }
}

// ── Systems ─────────────────────────────────────────────────────────

/// Drains RT events from the queue and emits core ECS messages.
///
/// Runs in `PreUpdate`. Converts:
///
/// - `ClockTick` into [`TickNow`](crate::TickNow) (every tick, not just subdivision 0)
/// - `Transport` into [`TransportChanged`](crate::TransportChanged)
/// - `MidiInput` into [`MidiInReceived`](crate::MidiInReceived)
///
/// If the [`RtBridge`] resource is not present, this system is a no-op.
pub fn drain_rt_events_system(
    bridge: Option<NonSendMut<RtBridge>>,
    mut tick_writer: MessageWriter<crate::TickNow>,
    mut transport_writer: MessageWriter<crate::TransportChanged>,
    mut midi_writer: MessageWriter<crate::MidiInReceived>,
    mut warning_writer: MessageWriter<crate::RtWarning>,
    mut spp_writer: MessageWriter<crate::SongPositionChanged>,
) {
    let Some(mut bridge) = bridge else { return };
    while let Ok(event) = bridge.events.pop() {
        match event {
            oxurack_rt::RtEvent::ClockTick {
                beat, subdivision, ..
            } => {
                let frame = beat.wrapping_mul(24).wrapping_add(subdivision as u64);
                tick_writer.write(crate::TickNow { frame });
            }
            oxurack_rt::RtEvent::Transport(t) => {
                let state = match t {
                    oxurack_rt::TransportEvent::Start => crate::TransportState::Started,
                    oxurack_rt::TransportEvent::Stop => crate::TransportState::Stopped,
                    oxurack_rt::TransportEvent::Continue => crate::TransportState::Continued,
                    // TransportEvent is #[non_exhaustive]
                    _ => continue,
                };
                transport_writer.write(crate::TransportChanged(state));
            }
            oxurack_rt::RtEvent::MidiInput {
                input_port_index,
                timestamp_ns,
                message,
            } => {
                if let Some(core_msg) = convert_rt_midi(&message) {
                    midi_writer.write(crate::MidiInReceived {
                        port_index: input_port_index,
                        timestamp_ns,
                        message: core_msg,
                    });
                }
            }
            oxurack_rt::RtEvent::NonFatalError(code) => {
                warning_writer.write(crate::RtWarning {
                    code: crate::RtWarningCode::from(code),
                });
            }
            oxurack_rt::RtEvent::SongPosition { position } => {
                spp_writer.write(crate::SongPositionChanged { position });
            }
            // RtEvent is #[non_exhaustive]; ignore unknown future variants.
            _ => {}
        }
    }
}

/// Flushes buffered MIDI output commands to the RT thread.
///
/// Runs in `PostUpdate`. Drains the [`MidiOutputQueue`] and pushes
/// each command to the RT thread's command queue via [`RtBridge`].
///
/// Commands that cannot be pushed (queue full) are silently dropped
/// to avoid blocking the game loop. If the [`RtBridge`] resource is
/// not present, this system is a no-op.
pub fn flush_midi_output_system(
    bridge: Option<NonSendMut<RtBridge>>,
    mut queue: ResMut<MidiOutputQueue>,
) {
    let Some(mut bridge) = bridge else { return };
    for cmd in queue.commands.drain(..) {
        let _ = bridge.commands.push(cmd); // drop on full, don't panic
    }
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // ── convert_rt_midi ─────────────────────────────────────────

    #[test]
    fn test_convert_rt_midi_note_on() {
        let rt_msg = oxurack_rt::MidiMessage::note_on(1, 60, 100);
        let core = convert_rt_midi(&rt_msg).expect("should convert NoteOn");

        assert_eq!(
            core,
            crate::MidiMessage::NoteOn {
                channel: 1,
                note: 60,
                velocity: 100,
            }
        );
    }

    #[test]
    fn test_convert_rt_midi_note_on_velocity_zero_becomes_note_off() {
        let rt_msg = oxurack_rt::MidiMessage::note_on(0, 64, 0);
        let core = convert_rt_midi(&rt_msg).expect("should convert to NoteOff");

        assert_eq!(
            core,
            crate::MidiMessage::NoteOff {
                channel: 0,
                note: 64,
                velocity: 0,
            }
        );
    }

    #[test]
    fn test_convert_rt_midi_note_off() {
        let rt_msg = oxurack_rt::MidiMessage::note_off(2, 72, 64);
        let core = convert_rt_midi(&rt_msg).expect("should convert NoteOff");

        assert_eq!(
            core,
            crate::MidiMessage::NoteOff {
                channel: 2,
                note: 72,
                velocity: 64,
            }
        );
    }

    #[test]
    fn test_convert_rt_midi_cc() {
        let rt_msg = oxurack_rt::MidiMessage::cc(3, 74, 127);
        let core = convert_rt_midi(&rt_msg).expect("should convert CC");

        assert_eq!(
            core,
            crate::MidiMessage::ControlChange {
                channel: 3,
                controller: 74,
                value: 127,
            }
        );
    }

    #[test]
    fn test_convert_rt_midi_program_change() {
        let rt_msg = oxurack_rt::MidiMessage::program_change(5, 42);
        let core = convert_rt_midi(&rt_msg).expect("should convert ProgramChange");

        assert_eq!(
            core,
            crate::MidiMessage::ProgramChange {
                channel: 5,
                program: 42,
            }
        );
    }

    #[test]
    fn test_convert_rt_midi_pitch_bend() {
        // Centre position: lsb=0, msb=64 => raw = 64*128 + 0 = 8192 => value = 0
        let rt_msg = oxurack_rt::MidiMessage::pitch_bend(0, 0, 64);
        let core = convert_rt_midi(&rt_msg).expect("should convert PitchBend");

        assert_eq!(
            core,
            crate::MidiMessage::PitchBend {
                channel: 0,
                value: 0,
            }
        );
    }

    #[test]
    fn test_convert_rt_midi_channel_pressure() {
        let rt_msg = oxurack_rt::MidiMessage {
            status: 0xD0,
            data1: 100,
            data2: 0,
            length: 2,
        };
        let core = convert_rt_midi(&rt_msg).expect("should convert ChannelPressure");

        assert_eq!(
            core,
            crate::MidiMessage::ChannelPressure {
                channel: 0,
                pressure: 100,
            }
        );
    }

    #[test]
    fn test_convert_rt_midi_poly_key_pressure() {
        let rt_msg = oxurack_rt::MidiMessage {
            status: 0xA1,
            data1: 60,
            data2: 80,
            length: 3,
        };
        let core = convert_rt_midi(&rt_msg).expect("should convert PolyKeyPressure");

        assert_eq!(
            core,
            crate::MidiMessage::PolyKeyPressure {
                channel: 1,
                note: 60,
                pressure: 80,
            }
        );
    }

    #[test]
    fn test_convert_rt_midi_unknown_returns_none() {
        // System realtime byte in status position (0xF8 = Clock).
        let rt_msg = oxurack_rt::MidiMessage {
            status: 0xF8,
            data1: 0,
            data2: 0,
            length: 1,
        };
        assert!(convert_rt_midi(&rt_msg).is_none());
    }

    // ── convert_core_midi ───────────────────────────────────────

    #[test]
    fn test_convert_core_midi_note_on() {
        let core = crate::MidiMessage::NoteOn {
            channel: 1,
            note: 60,
            velocity: 100,
        };
        let rt = convert_core_midi(&core).expect("should convert NoteOn");

        assert_eq!(rt, oxurack_rt::MidiMessage::note_on(1, 60, 100));
    }

    #[test]
    fn test_convert_core_midi_note_off() {
        let core = crate::MidiMessage::NoteOff {
            channel: 2,
            note: 72,
            velocity: 64,
        };
        let rt = convert_core_midi(&core).expect("should convert NoteOff");

        assert_eq!(rt, oxurack_rt::MidiMessage::note_off(2, 72, 64));
    }

    #[test]
    fn test_convert_core_midi_cc() {
        let core = crate::MidiMessage::ControlChange {
            channel: 3,
            controller: 74,
            value: 127,
        };
        let rt = convert_core_midi(&core).expect("should convert CC");

        assert_eq!(rt, oxurack_rt::MidiMessage::cc(3, 74, 127));
    }

    #[test]
    fn test_convert_core_midi_program_change() {
        let core = crate::MidiMessage::ProgramChange {
            channel: 5,
            program: 42,
        };
        let rt = convert_core_midi(&core).expect("should convert ProgramChange");

        assert_eq!(rt, oxurack_rt::MidiMessage::program_change(5, 42));
    }

    #[test]
    fn test_convert_core_midi_pitch_bend() {
        let core = crate::MidiMessage::PitchBend {
            channel: 0,
            value: 0,
        };
        let rt = convert_core_midi(&core).expect("should convert PitchBend");

        // Centre: value 0 => biased = 8192 => lsb = 0, msb = 64
        assert_eq!(rt, oxurack_rt::MidiMessage::pitch_bend(0, 0, 64));
    }

    #[test]
    fn test_convert_core_midi_channel_pressure() {
        let core = crate::MidiMessage::ChannelPressure {
            channel: 0,
            pressure: 100,
        };
        let rt = convert_core_midi(&core).expect("should convert ChannelPressure");

        assert_eq!(rt.status, 0xD0);
        assert_eq!(rt.data1, 100);
    }

    #[test]
    fn test_convert_core_midi_poly_key_pressure() {
        let core = crate::MidiMessage::PolyKeyPressure {
            channel: 1,
            note: 60,
            pressure: 80,
        };
        let rt = convert_core_midi(&core).expect("should convert PolyKeyPressure");

        assert_eq!(rt.status, 0xA1);
        assert_eq!(rt.data1, 60);
        assert_eq!(rt.data2, 80);
    }

    #[test]
    fn test_convert_core_midi_system_message_returns_none() {
        assert!(convert_core_midi(&crate::MidiMessage::Clock).is_none());
        assert!(convert_core_midi(&crate::MidiMessage::Start).is_none());
        assert!(convert_core_midi(&crate::MidiMessage::Stop).is_none());
        assert!(convert_core_midi(&crate::MidiMessage::Continue).is_none());
        assert!(convert_core_midi(&crate::MidiMessage::SongPosition { position: 0 }).is_none());
        assert!(convert_core_midi(&crate::MidiMessage::SystemExclusive).is_none());
    }

    // ── Round-trip ──────────────────────────────────────────────

    #[test]
    fn test_convert_rt_to_core_to_rt_note_on_roundtrip() {
        let original = oxurack_rt::MidiMessage::note_on(3, 72, 110);
        let core = convert_rt_midi(&original).expect("RT -> core should succeed");
        let back = convert_core_midi(&core).expect("core -> RT should succeed");
        assert_eq!(original, back);
    }

    #[test]
    fn test_convert_rt_to_core_to_rt_cc_roundtrip() {
        let original = oxurack_rt::MidiMessage::cc(0, 1, 64);
        let core = convert_rt_midi(&original).expect("RT -> core should succeed");
        let back = convert_core_midi(&core).expect("core -> RT should succeed");
        assert_eq!(original, back);
    }

    #[test]
    fn test_convert_rt_to_core_to_rt_pitch_bend_roundtrip() {
        let original = oxurack_rt::MidiMessage::pitch_bend(0, 0, 64);
        let core = convert_rt_midi(&original).expect("RT -> core should succeed");
        let back = convert_core_midi(&core).expect("core -> RT should succeed");
        assert_eq!(original, back);
    }

    #[test]
    fn test_convert_rt_to_core_to_rt_program_change_roundtrip() {
        let original = oxurack_rt::MidiMessage::program_change(7, 99);
        let core = convert_rt_midi(&original).expect("RT -> core should succeed");
        let back = convert_core_midi(&core).expect("core -> RT should succeed");
        assert_eq!(original, back);
    }

    // ── MidiOutputQueue ─────────────────────────────────────────

    #[test]
    fn test_midi_output_queue_default_is_empty() {
        let queue = MidiOutputQueue::default();
        assert!(queue.commands.is_empty());
    }

    #[test]
    fn test_midi_output_queue_push_and_drain() {
        let mut queue = MidiOutputQueue::default();

        queue
            .commands
            .push(oxurack_rt::EcsCommand::SetTempo { bpm: 120.0 });
        queue.commands.push(oxurack_rt::EcsCommand::Shutdown);
        assert_eq!(queue.commands.len(), 2);

        let drained: Vec<_> = queue.commands.drain(..).collect();
        assert_eq!(drained.len(), 2);
        assert!(queue.commands.is_empty());
    }
}
