//! Error types for the oxurack-rt crate.

/// Errors that can occur during real-time MIDI operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to elevate the RT thread to real-time priority.
    #[error("RT priority elevation failed: {0}")]
    PriorityElevation(String),

    /// MIDI subsystem initialization failed.
    #[error("MIDI initialization failed: {0}")]
    MidiInit(String),

    /// A requested MIDI port could not be found by name.
    #[error("MIDI port not found: {name}")]
    PortNotFound {
        /// The name that was searched for.
        name: String,
    },

    /// The RT-to-ECS lock-free queue is full; events are being dropped.
    #[error("RT-to-ECS queue full")]
    QueueFull,

    /// The RT thread panicked and is no longer running.
    #[error("RT thread panicked")]
    ThreadPanicked,

    /// Attempted to stop or interact with a runtime that has already stopped.
    #[error("runtime already stopped")]
    AlreadyStopped,
}
