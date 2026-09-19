//! Deterministic sample-driven musical clock for oxurack.
//!
//! The standalone clock advances once for each caller-provided sample. Its
//! default configuration is 120 BPM, one pulse per quarter note, and a
//! ten-millisecond pulse width. The optional `rack` feature adds the
//! [`ClockModule`] adapter for the oxurack ECS.
//!
//! # Standalone example
//!
//! ```
//! # fn main() -> Result<(), oxurack_mod_clock::ClockError> {
//! use oxurack_mod_clock::{Clock, ClockConfig};
//!
//! let config = ClockConfig::try_new(120.0, 1, 10)?;
//! let mut clock = Clock::new(config);
//! assert!(clock.next_sample());
//! # Ok(())
//! # }
//! ```

mod clock;

pub use clock::{Clock, ClockConfig, ClockError};

#[cfg(feature = "rack")]
mod rack;

#[cfg(feature = "rack")]
pub use rack::ClockModule;
