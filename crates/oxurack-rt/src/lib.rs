// TODO: remove once scaffold is wired up (Milestone 2.4)
#![allow(dead_code)]

//! Real-time MIDI clock and I/O thread for oxurack.
//!
//! `oxurack-rt` runs on a dedicated OS thread elevated to real-time
//! priority. It handles:
//!
//! - **Clock generation** (master mode) at a configurable tempo, producing
//!   24-PPQN MIDI clock ticks.
//! - **Clock tracking** (slave mode) using a PLL-based tempo estimator
//!   locked to an external MIDI clock source.
//! - **MIDI I/O** via `midir`, forwarding input messages to the ECS world
//!   and sending output messages on command.
//! - **Lock-free communication** with the ECS world through bounded SPSC
//!   queues (`rtrb`).
//!
//! # Architecture
//!
//! ```text
//! ┌────────────────────┐     rtrb queues     ┌──────────────────┐
//! │   ECS world        │◄═══ RtEvent ═══════►│   RT thread      │
//! │ (RtHandles)        │════ EcsCommand ════►│ (rt_thread_main) │
//! └────────────────────┘                     └──────────────────┘
//! ```
//!
//! The caller creates a [`Runtime`] via [`Runtime::start`], receiving
//! [`RtHandles`] for queue access. Dropping the `Runtime` (or calling
//! [`Runtime::stop`]) shuts down the thread gracefully.

pub mod clock;
mod error;
mod messages;
mod midi_io;
mod priority;
mod queues;
mod thread;
mod timing;

// Re-export the public API.
pub use error::Error;
pub use messages::{EcsCommand, MidiMessage, RtErrorCode, RtEvent, TransportEvent};
pub use midi_io::{list_midi_input_ports, list_midi_output_ports};
pub use queues::RtHandles;

use std::thread::JoinHandle;

/// Configuration for a MIDI output port connection.
#[derive(Debug, Clone)]
pub struct MidiOutputConfig {
    /// Human-readable name (or substring) used to match an available port.
    pub name: String,
}

/// Configuration for a MIDI input port connection.
#[derive(Debug, Clone)]
pub struct MidiInputConfig {
    /// Human-readable name (or substring) used to match an available port.
    pub name: String,
}

/// Selects how the RT thread generates or tracks MIDI clock.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ClockMode {
    /// This system is the clock master: it generates clock ticks at the
    /// specified tempo and optionally sends MIDI transport messages.
    Master {
        /// Initial tempo in beats per minute.
        tempo_bpm: f64,
        /// Whether to send MIDI Start/Stop/Continue messages on the
        /// output ports.
        send_transport: bool,
    },

    /// This system tracks an external MIDI clock source on the given
    /// input port, using a PLL to smooth jitter.
    Slave {
        /// Name (or substring) of the MIDI input port carrying the
        /// external clock.
        clock_input_port: String,
        /// Timeout in nanoseconds: if no clock tick is received within
        /// this window, the slave reports [`RtErrorCode::ClockDropout`].
        timeout_ns: u64,
    },
}

/// Full configuration for starting the RT runtime.
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Clock mode selection (master or slave).
    pub clock_mode: ClockMode,
    /// MIDI output port configurations.
    pub outputs: Vec<MidiOutputConfig>,
    /// MIDI input port configurations.
    pub inputs: Vec<MidiInputConfig>,
    /// Capacity of the RT-to-ECS event queue.
    pub event_queue_capacity: usize,
    /// Capacity of the ECS-to-RT command queue.
    pub command_queue_capacity: usize,
}

/// Handle to the running RT thread.
///
/// Owns the join handle for the spawned thread. Dropping the `Runtime`
/// sends a shutdown command and joins the thread. Use [`Runtime::start`]
/// to create one.
pub struct Runtime {
    /// Join handle for the RT thread (None after stop/drop).
    _thread: Option<JoinHandle<()>>,
}

impl Runtime {
    /// Spawns the RT thread with the given configuration.
    ///
    /// Returns a `Runtime` handle (for lifecycle management) and
    /// [`RtHandles`] (for queue communication with the ECS world).
    ///
    /// # Errors
    ///
    /// Returns [`Error::MidiInit`] if MIDI ports cannot be opened, or
    /// [`Error::PortNotFound`] if a configured port name has no match.
    pub fn start(_config: RuntimeConfig) -> Result<(Self, RtHandles), Error> {
        unimplemented!()
    }

    /// Gracefully shuts down the RT thread.
    ///
    /// Sends a [`EcsCommand::Shutdown`] command via the queue and joins
    /// the thread. This is also called automatically on drop.
    ///
    /// # Errors
    ///
    /// Returns [`Error::AlreadyStopped`] if the runtime was already
    /// shut down, or [`Error::ThreadPanicked`] if the thread panicked.
    pub fn stop(&mut self) -> Result<(), Error> {
        unimplemented!()
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        // Placeholder: will send Shutdown and join in a future milestone.
    }
}
