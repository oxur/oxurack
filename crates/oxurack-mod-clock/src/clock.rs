//! The standalone sample-driven clock state machine.

const MILLIS_PER_MINUTE: u64 = 60_000_000;
const DEFAULT_PPQN: u16 = 1;
const DEFAULT_PULSE_WIDTH_MS: u16 = 10;

/// Errors returned when a clock configuration is invalid.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum ClockError {
    /// BPM was NaN or infinite.
    #[error("BPM must be finite")]
    NonFiniteBpm,
    /// BPM was outside the supported range of 1 through 1000.
    #[error("BPM must be in the range 1..=1000")]
    BpmOutOfRange,
    /// PPQN was outside the supported range of 1 through 96.
    #[error("PPQN must be in the range 1..=96")]
    InvalidPpqn,
    /// Pulse width was outside the supported range of 1 through 1000 ms.
    #[error("pulse width must be in the range 1..=1000 ms")]
    InvalidPulseWidth,
    /// The configured pulse width leaves no low sample between pulses.
    #[error("pulse width overlaps the next pulse")]
    PulseOverlap,
}

/// A validated, canonical musical-clock configuration.
///
/// BPM is stored at 0.001 BPM resolution. A valid configuration also
/// guarantees at least one low sample between consecutive pulses at the
/// one-millisecond sample rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockConfig {
    milli_bpm: u32,
    ppqn: u16,
    pulse_width_ms: u16,
}

impl Default for ClockConfig {
    fn default() -> Self {
        Self {
            milli_bpm: 120_000,
            ppqn: DEFAULT_PPQN,
            pulse_width_ms: DEFAULT_PULSE_WIDTH_MS,
        }
    }
}

impl ClockConfig {
    /// Creates a validated configuration.
    ///
    /// BPM is canonicalized to the nearest 0.001 BPM using ties rounded
    /// upward. The cross-field check uses integer arithmetic after
    /// canonicalization.
    ///
    /// # Errors
    ///
    /// Returns a [`ClockError`] identifying the first invalid condition in
    /// validation order: finite BPM, BPM range, PPQN range, pulse-width
    /// range, then pulse overlap.
    pub fn try_new(bpm: f32, ppqn: u16, pulse_width_ms: u16) -> Result<Self, ClockError> {
        if !bpm.is_finite() {
            return Err(ClockError::NonFiniteBpm);
        }
        if !(1.0..=1000.0).contains(&bpm) {
            return Err(ClockError::BpmOutOfRange);
        }
        if !(1..=96).contains(&ppqn) {
            return Err(ClockError::InvalidPpqn);
        }
        if !(1..=1000).contains(&pulse_width_ms) {
            return Err(ClockError::InvalidPulseWidth);
        }

        let milli_bpm = (f64::from(bpm) * 1000.0).round() as u32;
        let rate = u64::from(milli_bpm) * u64::from(ppqn);
        if MILLIS_PER_MINUTE / rate < u64::from(pulse_width_ms) + 1 {
            return Err(ClockError::PulseOverlap);
        }

        Ok(Self {
            milli_bpm,
            ppqn,
            pulse_width_ms,
        })
    }

    /// Returns the canonical BPM value at 0.001 BPM resolution.
    #[must_use]
    pub fn bpm(&self) -> f32 {
        self.milli_bpm as f32 / 1000.0
    }

    /// Returns pulses per quarter note.
    #[must_use]
    pub const fn ppqn(&self) -> u16 {
        self.ppqn
    }

    /// Returns pulse width in milliseconds.
    #[must_use]
    pub const fn pulse_width_ms(&self) -> u16 {
        self.pulse_width_ms
    }
}

/// A deterministic one-millisecond musical clock.
///
/// Each call to [`Clock::next_sample`] advances exactly one sample. The
/// first sample of a fresh epoch is high. A successful configuration change
/// inserts one low sample before the next fresh epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    config: ClockConfig,
    phase: u64,
    high_remaining: u16,
    fresh: bool,
    restart_pending: bool,
}

impl Clock {
    /// Creates a clock at the beginning of a fresh epoch.
    #[must_use]
    pub const fn new(config: ClockConfig) -> Self {
        Self {
            config,
            phase: 0,
            high_remaining: 0,
            fresh: true,
            restart_pending: false,
        }
    }

    /// Returns the current validated configuration.
    #[must_use]
    pub const fn config(&self) -> ClockConfig {
        self.config
    }

    /// Replaces the configuration and schedules one low separator sample.
    ///
    /// Setting the same canonical configuration is a true no-op. A changed
    /// configuration takes effect at the next executed sample after the
    /// separator; repeated changes coalesce to the last configuration while
    /// retaining the pending separator.
    pub fn set_config(&mut self, config: ClockConfig) {
        if self.config == config {
            return;
        }
        self.config = config;
        self.restart_pending = true;
    }

    /// Advances one sample and returns the Gate level for that sample.
    pub fn next_sample(&mut self) -> bool {
        if self.restart_pending {
            self.restart_pending = false;
            self.phase = 0;
            self.high_remaining = 0;
            self.fresh = true;
            return false;
        }

        let rising = if self.fresh {
            self.fresh = false;
            true
        } else {
            let rate = u64::from(self.config.milli_bpm) * u64::from(self.config.ppqn);
            self.phase += rate;
            if self.phase >= MILLIS_PER_MINUTE {
                self.phase -= MILLIS_PER_MINUTE;
                true
            } else {
                false
            }
        };

        if rising {
            self.high_remaining = self.config.pulse_width_ms;
        }
        if self.high_remaining > 0 {
            self.high_remaining -= 1;
            true
        } else {
            false
        }
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new(ClockConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rising_indices(clock: &mut Clock, samples: usize) -> Vec<usize> {
        let mut previous = false;
        let mut rises = Vec::new();
        for index in 0..samples {
            let current = clock.next_sample();
            if current && !previous {
                rises.push(index);
            }
            previous = current;
        }
        rises
    }

    #[test]
    fn default_pulse_width_and_interval_are_exact() {
        let mut clock = Clock::default();
        let output: Vec<_> = (0..1_010).map(|_| clock.next_sample()).collect();
        assert!(output[0..10].iter().all(|value| *value));
        assert!(output[10..500].iter().all(|value| !*value));
        assert!(output[500..510].iter().all(|value| *value));
        assert!(output[510..1_000].iter().all(|value| !*value));
        assert!(output[1_000..1_010].iter().all(|value| *value));
    }

    #[test]
    fn fractional_rate_uses_bounded_integer_accumulator() {
        let config = ClockConfig::try_new(137.0, 4, 3).unwrap();
        let mut clock = Clock::new(config);
        assert_eq!(
            rising_indices(&mut clock, 800),
            vec![0, 110, 219, 329, 438, 548, 657, 767]
        );
    }

    #[test]
    fn boundary_rate_has_a_low_sample() {
        let config = ClockConfig::try_new(1000.0, 30, 1).unwrap();
        let mut clock = Clock::new(config);
        assert_eq!(rising_indices(&mut clock, 8), vec![0, 2, 4, 6]);
        assert_eq!(
            ClockConfig::try_new(1000.0, 30, 2),
            Err(ClockError::PulseOverlap)
        );
    }

    #[test]
    fn validation_order_and_bpm_canonicalization_are_stable() {
        assert_eq!(
            ClockConfig::try_new(f32::NAN, 0, 0),
            Err(ClockError::NonFiniteBpm)
        );
        assert_eq!(
            ClockConfig::try_new(0.0, 0, 0),
            Err(ClockError::BpmOutOfRange)
        );
        assert_eq!(
            ClockConfig::try_new(120.0, 0, 1),
            Err(ClockError::InvalidPpqn)
        );
        assert_eq!(
            ClockConfig::try_new(120.0, 1, 0),
            Err(ClockError::InvalidPulseWidth)
        );
        assert_eq!(ClockConfig::try_new(120.0004, 1, 10).unwrap().bpm(), 120.0);
        assert_eq!(
            ClockConfig::try_new(120.0006, 1, 10).unwrap().bpm(),
            120.001
        );
    }

    #[test]
    fn configuration_change_forces_one_low_sample_and_equal_is_noop() {
        let original = ClockConfig::try_new(120.0, 1, 3).unwrap();
        let faster = ClockConfig::try_new(240.0, 1, 10).unwrap();
        let mut clock = Clock::new(original);
        assert_eq!(
            (0..3).map(|_| clock.next_sample()).collect::<Vec<_>>(),
            vec![true; 3]
        );
        clock.set_config(faster);
        assert!(!clock.next_sample());
        assert_eq!(
            (0..10).map(|_| clock.next_sample()).collect::<Vec<_>>(),
            vec![true; 10]
        );

        let mut control = Clock::new(original);
        let mut edited = Clock::new(original);
        for _ in 0..3 {
            assert_eq!(control.next_sample(), edited.next_sample());
        }
        edited.set_config(original);
        assert_eq!(control.next_sample(), edited.next_sample());
    }

    #[test]
    fn multiple_edits_coalesce_and_invalid_configuration_is_not_constructible() {
        let first = ClockConfig::try_new(120.0, 1, 3).unwrap();
        let second = ClockConfig::try_new(240.0, 1, 10).unwrap();
        let third = ClockConfig::try_new(300.0, 1, 5).unwrap();
        let mut clock = Clock::new(first);
        clock.next_sample();
        clock.set_config(second);
        clock.set_config(third);
        clock.set_config(first);
        clock.set_config(first);
        assert!(!clock.next_sample());
        assert!(clock.next_sample());
        assert_eq!(clock.config(), first);
        assert_eq!(
            ClockConfig::try_new(1000.0, 97, 0),
            Err(ClockError::InvalidPpqn)
        );
        assert_eq!(
            ClockConfig::try_new(1000.0, 96, 1001),
            Err(ClockError::InvalidPulseWidth)
        );
    }
}
