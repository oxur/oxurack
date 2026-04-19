//! ECS world, values, ports, and cables for oxurack.
//!
//! `oxurack-core` defines the foundational types for the oxurack modular
//! synthesizer: signal values (audio CV, gates, MIDI), port descriptors,
//! cable transforms, module identifiers, and error types.
//!
//! # Phase 1 modules (implemented)
//!
//! - [`value`] -- signal values and coercion
//! - [`port`] -- port names, directions, and merge policies
//! - [`cable`] -- cable transforms
//! - [`module`] -- module kind and ID types
//! - [`error`] -- error types
//!
//! # Phase 2+ modules (stubs)
//!
//! - `tick` -- frame-tick scheduling
//! - `parameter` -- module parameter descriptors
//! - `patch` -- patch graph
//! - `scale` -- musical scales
//! - `rng` -- deterministic RNG
//! - `event` -- ECS events

pub mod cable;
pub mod error;
pub mod module;
pub mod port;
pub mod value;

// Phase 2+ stubs.
mod event;
mod parameter;
mod patch;
mod rng;
mod scale;
mod tick;

// ── Re-exports ──────────────────────────────────────────────────────

pub use cable::CableTransform;
pub use error::{CoreError, PatchError, TickError};
pub use module::{ModuleId, ModuleKind};
pub use port::{MergePolicy, PortDirection, PortName};
pub use value::{MidiMessage, Value, ValueKind};
