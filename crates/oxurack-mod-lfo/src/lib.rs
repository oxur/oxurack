//! A deterministic sample-driven low-frequency oscillator for oxurack.
//!
//! The standalone engine advances once per caller-provided one-millisecond
//! sample. Enable the optional `rack` feature for the [`LfoModule`] adapter.
//!
//! # Standalone example
//!
//! ```
//! use oxurack_mod_lfo::{Lfo, LfoConfig, Waveform};
//!
//! let config = LfoConfig::try_new(Waveform::Sine, 1.0, 0.0, 0.5).unwrap();
//! let mut lfo = Lfo::new(config);
//! let sample = lfo.next_sample(0.0, false);
//! assert_eq!(sample.out, 0.5);
//! ```

mod lfo;

pub use lfo::{Lfo, LfoConfig, LfoError, LfoSample, Waveform};

#[cfg(feature = "rack")]
mod rack;

#[cfg(feature = "rack")]
pub use rack::LfoModule;
