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

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;

/// Configuration for a MIDI output port connection.
///
/// The `name` field is matched case-insensitively as a substring
/// against the system's available MIDI output port names.
#[derive(Debug, Clone)]
pub struct MidiOutputConfig {
    /// Human-readable name (or substring) used to match an available port.
    pub name: String,
}

/// Configuration for a MIDI input port connection.
///
/// The `name` field is matched case-insensitively as a substring
/// against the system's available MIDI input port names.
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
///
/// Specifies the clock mode (master or slave), which MIDI ports to
/// open, and the capacity of the lock-free communication queues.
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
/// signals shutdown and joins the thread. Use [`Runtime::start`] to
/// create one.
pub struct Runtime {
    /// Join handle for the RT thread (`None` after `stop` or `drop`).
    pub(crate) thread: Option<JoinHandle<()>>,
    /// Atomic flag shared with the RT thread to signal shutdown.
    shutdown: Arc<AtomicBool>,
}

impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Runtime")
            .field("running", &self.thread.is_some())
            .finish()
    }
}

impl Runtime {
    /// Spawns the RT thread with the given configuration.
    ///
    /// Blocks until the thread has elevated its priority and opened all
    /// MIDI ports. Returns a `Runtime` handle (for lifecycle management)
    /// and [`RtHandles`] (for queue communication with the ECS world).
    ///
    /// # Errors
    ///
    /// Returns [`Error::MidiInit`] if MIDI ports cannot be opened,
    /// [`Error::PortNotFound`] if a configured port name has no match,
    /// or [`Error::PriorityElevation`] if RT priority cannot be obtained.
    pub fn start(config: RuntimeConfig) -> Result<(Self, RtHandles), Error> {
        let (rt_queues, ecs_handles) = crate::queues::create_queues(
            config.event_queue_capacity,
            config.command_queue_capacity,
        );

        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = Arc::clone(&shutdown);

        let (ready_tx, ready_rx) = std::sync::mpsc::sync_channel(1);

        let thread = std::thread::Builder::new()
            .name("oxurack-rt".into())
            .spawn(move || {
                crate::thread::rt_thread_main(rt_queues, config, ready_tx, shutdown_clone);
            })
            .map_err(|e| Error::MidiInit(format!("failed to spawn RT thread: {e}")))?;

        // Wait for the thread to signal readiness (or error).
        let ready_result = ready_rx.recv().map_err(|_| Error::ThreadPanicked)?;
        ready_result?;

        Ok((
            Self {
                thread: Some(thread),
                shutdown,
            },
            ecs_handles,
        ))
    }

    /// Gracefully shuts down the RT thread.
    ///
    /// Sets the shutdown flag and joins the thread. This is also called
    /// automatically on [`Drop`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::AlreadyStopped`] if the runtime was already
    /// shut down, or [`Error::ThreadPanicked`] if the thread panicked.
    pub fn stop(&mut self) -> Result<(), Error> {
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            thread.join().map_err(|_| Error::ThreadPanicked)?;
        } else {
            return Err(Error::AlreadyStopped);
        }
        Ok(())
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_debug() {
        let config = RuntimeConfig {
            clock_mode: ClockMode::Master {
                tempo_bpm: 120.0,
                send_transport: false,
            },
            outputs: Vec::new(),
            inputs: Vec::new(),
            event_queue_capacity: 1024,
            command_queue_capacity: 1024,
        };
        let (mut runtime, _handles) = Runtime::start(config).unwrap();

        let debug_running = format!("{runtime:?}");
        assert!(
            debug_running.contains("running: true"),
            "expected 'running: true' in debug output, got: {debug_running}"
        );

        runtime.stop().unwrap();

        let debug_stopped = format!("{runtime:?}");
        assert!(
            debug_stopped.contains("running: false"),
            "expected 'running: false' in debug output, got: {debug_stopped}"
        );
    }

    #[test]
    fn test_rt_handles_debug() {
        let config = RuntimeConfig {
            clock_mode: ClockMode::Master {
                tempo_bpm: 120.0,
                send_transport: false,
            },
            outputs: Vec::new(),
            inputs: Vec::new(),
            event_queue_capacity: 1024,
            command_queue_capacity: 1024,
        };
        let (mut runtime, handles) = Runtime::start(config).unwrap();

        let debug = format!("{handles:?}");
        assert!(
            debug.contains("RtHandles"),
            "expected 'RtHandles' in debug output, got: {debug}"
        );

        runtime.stop().unwrap();
    }

    #[test]
    fn test_runtime_config_debug() {
        let config = RuntimeConfig {
            clock_mode: ClockMode::Master {
                tempo_bpm: 120.0,
                send_transport: false,
            },
            outputs: Vec::new(),
            inputs: Vec::new(),
            event_queue_capacity: 1024,
            command_queue_capacity: 1024,
        };
        let debug = format!("{config:?}");
        assert!(
            debug.contains("RuntimeConfig"),
            "expected 'RuntimeConfig' in debug output, got: {debug}"
        );
    }

    #[test]
    fn test_clock_mode_debug() {
        let master = ClockMode::Master {
            tempo_bpm: 120.0,
            send_transport: true,
        };
        let debug = format!("{master:?}");
        assert!(
            debug.contains("Master"),
            "expected 'Master' in debug output, got: {debug}"
        );

        let slave = ClockMode::Slave {
            clock_input_port: "test".to_string(),
            timeout_ns: 1_000_000_000,
        };
        let debug = format!("{slave:?}");
        assert!(
            debug.contains("Slave"),
            "expected 'Slave' in debug output, got: {debug}"
        );
    }

    #[test]
    fn test_midi_output_config_debug() {
        let config = MidiOutputConfig {
            name: "test-port".to_string(),
        };
        let debug = format!("{config:?}");
        assert!(
            debug.contains("test-port"),
            "expected port name in debug output, got: {debug}"
        );
    }

    #[test]
    fn test_midi_input_config_debug() {
        let config = MidiInputConfig {
            name: "test-input".to_string(),
        };
        let debug = format!("{config:?}");
        assert!(
            debug.contains("test-input"),
            "expected port name in debug output, got: {debug}"
        );
    }
}
