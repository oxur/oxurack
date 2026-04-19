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
/// Obtained from [`create_queues`] and passed to the ECS world. The RT
/// thread retains the complementary producer/consumer pair.
pub struct RtHandles {
    /// Receives events produced by the RT thread (clock ticks, MIDI
    /// input, transport changes, errors).
    pub events: Consumer<RtEvent>,
    /// Sends commands to the RT thread (MIDI output, tempo changes,
    /// transport, shutdown).
    pub commands: Producer<EcsCommand>,
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
    _event_capacity: usize,
    _command_capacity: usize,
) -> (RtSideQueues, RtHandles) {
    unimplemented!()
}
