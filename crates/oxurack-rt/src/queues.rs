//! Lock-free SPSC queues for RT-thread / ECS-world communication.
//!
//! Uses `rtrb` (real-time ring buffer) to provide bounded, lock-free,
//! single-producer / single-consumer queues. The RT thread produces
//! [`RtEvent`]s and consumes [`EcsCommand`]s; the ECS world does the
//! reverse.

use rtrb::{Consumer, Producer};

use crate::messages::{EcsCommand, RtEvent};

/// The ECS-side handles for communicating with the RT thread.
///
/// Returned by [`Runtime::start()`](crate::Runtime::start).
/// The ECS world uses these to receive events from the RT thread and
/// send commands to it.
pub struct RtHandles {
    /// Receives events produced by the RT thread (clock ticks, MIDI
    /// input, transport changes, errors).
    pub events: Consumer<RtEvent>,
    /// Sends commands to the RT thread (MIDI output, tempo changes,
    /// transport, shutdown).
    pub commands: Producer<EcsCommand>,
}

impl std::fmt::Debug for RtHandles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RtHandles").finish_non_exhaustive()
    }
}

/// The RT-thread-side handles (not exposed publicly).
pub(crate) struct RtSideQueues {
    /// Produces events for the ECS world to consume.
    pub(crate) events: Producer<RtEvent>,
    /// Consumes commands sent by the ECS world.
    pub(crate) commands: Consumer<EcsCommand>,
}

/// Creates a matched pair of lock-free queue handles.
///
/// Returns the RT-side handles (for the spawned thread) and the
/// ECS-side handles (for the caller).
///
/// # Arguments
///
/// * `event_capacity` - Maximum number of buffered RT-to-ECS events.
/// * `command_capacity` - Maximum number of buffered ECS-to-RT commands.
pub(crate) fn create_queues(
    event_capacity: usize,
    command_capacity: usize,
) -> (RtSideQueues, RtHandles) {
    let (event_producer, event_consumer) = rtrb::RingBuffer::new(event_capacity);
    let (command_producer, command_consumer) = rtrb::RingBuffer::new(command_capacity);

    let rt_side = RtSideQueues {
        events: event_producer,
        commands: command_consumer,
    };

    let ecs_side = RtHandles {
        events: event_consumer,
        commands: command_producer,
    };

    (rt_side, ecs_side)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::{MidiMessage, RtErrorCode, TransportEvent};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_queue_roundtrip_rt_event() {
        let (mut rt_side, mut ecs_side) = create_queues(16, 16);

        let tick = RtEvent::ClockTick {
            subdivision: 0,
            beat: 1,
            tempo_bpm: 120.0,
            timestamp_ns: 500_000,
        };

        rt_side.events.push(tick).expect("push should succeed");
        let received = ecs_side.events.pop().expect("pop should succeed");
        assert_eq!(received, tick);
    }

    #[test]
    fn test_queue_roundtrip_ecs_command() {
        let (mut rt_side, mut ecs_side) = create_queues(16, 16);

        let cmd = EcsCommand::SetTempo { bpm: 140.0 };

        ecs_side.commands.push(cmd).expect("push should succeed");
        let received = rt_side.commands.pop().expect("pop should succeed");
        assert_eq!(received, cmd);
    }

    #[test]
    fn test_queue_full_returns_error() {
        let (mut rt_side, _ecs_side) = create_queues(4, 4);

        let tick = RtEvent::ClockTick {
            subdivision: 0,
            beat: 0,
            tempo_bpm: 120.0,
            timestamp_ns: 0,
        };

        // Fill the queue to capacity.
        for _ in 0..4 {
            rt_side.events.push(tick).expect("push should succeed");
        }

        // The 5th push must fail (queue is full).
        let result = rt_side.events.push(tick);
        assert!(result.is_err(), "push to a full queue should return Err");
    }

    #[test]
    fn test_queue_empty_returns_none() {
        let (mut rt_side, mut ecs_side) = create_queues(16, 16);

        let event_result = ecs_side.events.pop();
        assert!(
            event_result.is_err(),
            "pop from empty event queue should return Err"
        );

        let command_result = rt_side.commands.pop();
        assert!(
            command_result.is_err(),
            "pop from empty command queue should return Err"
        );
    }

    #[test]
    fn test_queue_cross_thread() {
        let (mut rt_side, mut ecs_side) = create_queues(128, 16);

        let handle = std::thread::spawn(move || {
            for i in 0..100u64 {
                let tick = RtEvent::ClockTick {
                    subdivision: 0,
                    beat: i,
                    tempo_bpm: 120.0,
                    timestamp_ns: i * 1_000,
                };
                // Spin until the push succeeds (queue should never be
                // full with capacity 128, but this is defensive).
                while rt_side.events.push(tick).is_err() {
                    std::thread::yield_now();
                }
            }
        });

        let mut received = Vec::with_capacity(100);
        while received.len() < 100 {
            match ecs_side.events.pop() {
                Ok(event) => received.push(event),
                Err(_) => std::thread::sleep(std::time::Duration::from_micros(10)),
            }
        }

        handle.join().expect("producer thread should not panic");

        assert_eq!(received.len(), 100);
        for (i, event) in received.iter().enumerate() {
            let expected = RtEvent::ClockTick {
                subdivision: 0,
                beat: i as u64,
                tempo_bpm: 120.0,
                timestamp_ns: i as u64 * 1_000,
            };
            assert_eq!(*event, expected);
        }
    }

    #[test]
    fn test_queue_multiple_event_types() {
        let (mut rt_side, mut ecs_side) = create_queues(16, 16);

        let events = [
            RtEvent::ClockTick {
                subdivision: 12,
                beat: 42,
                tempo_bpm: 128.0,
                timestamp_ns: 999,
            },
            RtEvent::Transport(TransportEvent::Start),
            RtEvent::MidiInput {
                input_port_index: 0,
                timestamp_ns: 1_000_000,
                message: MidiMessage::note_on(0, 60, 100),
            },
            RtEvent::SongPosition { position: 384 },
            RtEvent::NonFatalError(RtErrorCode::QueueOverflow),
        ];

        for event in &events {
            rt_side.events.push(*event).expect("push should succeed");
        }

        for expected in &events {
            let received = ecs_side.events.pop().expect("pop should succeed");
            assert_eq!(received, *expected);
        }
    }
}
