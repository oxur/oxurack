//! Standalone oscillator state and validation.

use std::str::FromStr;

/// One of the supported periodic oscillator shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waveform {
    /// A sinusoidal cycle beginning at its midpoint.
    Sine,
    /// A triangle beginning at its minimum.
    Triangle,
    /// A rising ramp from -1 to +1.
    SawUp,
    /// A falling ramp from +1 to -1.
    SawDown,
    /// A pulse with a configurable positive portion of each cycle.
    Square,
}

impl Waveform {
    /// Returns the canonical patch and display name for this waveform.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sine => "sine",
            Self::Triangle => "tri",
            Self::SawUp => "saw_up",
            Self::SawDown => "saw_down",
            Self::Square => "square",
        }
    }
}

impl std::fmt::Display for Waveform {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Waveform {
    type Err = LfoError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "sine" => Ok(Self::Sine),
            "tri" => Ok(Self::Triangle),
            "saw_up" => Ok(Self::SawUp),
            "saw_down" => Ok(Self::SawDown),
            "square" => Ok(Self::Square),
            _ => Err(LfoError::UnknownWaveform(value.to_owned())),
        }
    }
}

/// Errors returned when an LFO configuration or waveform is invalid.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum LfoError {
    /// Frequency was not finite.
    #[error("frequency must be finite")]
    NonFiniteFrequency,
    /// Frequency was outside the supported range 0 through 100 Hz.
    #[error("frequency must be in the range 0..=100 Hz")]
    FrequencyOutOfRange,
    /// Phase offset was not finite.
    #[error("phase offset must be finite")]
    NonFinitePhaseOffset,
    /// Phase offset was outside the supported range 0 through less than 1.
    #[error("phase offset must be in the range 0..1")]
    PhaseOffsetOutOfRange,
    /// Pulse width was not finite.
    #[error("pulse width must be finite")]
    NonFinitePulseWidth,
    /// Pulse width was outside the supported range greater than 0 through less than 1.
    #[error("pulse width must be in the range (0, 1)")]
    PulseWidthOutOfRange,
    /// Waveform name was not one of the supported canonical names.
    #[error("unknown waveform {0:?}; expected sine, tri, saw_up, saw_down, or square")]
    UnknownWaveform(String),
}

/// Validated LFO configuration. Frequency is measured in hertz and phase in cycles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LfoConfig {
    waveform: Waveform,
    frequency_hz: f32,
    phase_offset: f32,
    pulse_width: f32,
}

impl Default for LfoConfig {
    fn default() -> Self {
        Self {
            waveform: Waveform::Sine,
            frequency_hz: 1.0,
            phase_offset: 0.0,
            pulse_width: 0.5,
        }
    }
}

impl LfoConfig {
    /// Creates a configuration after checking all numeric ranges.
    ///
    /// Frequency must be finite and in `[0, 100]` Hz, phase offset finite and
    /// in `[0, 1)` cycles, and pulse width finite and in `(0, 1)` cycles.
    ///
    /// # Errors
    ///
    /// Returns the first invalid numeric field in frequency, phase-offset,
    /// pulse-width order.
    pub fn try_new(
        waveform: Waveform,
        frequency_hz: f32,
        phase_offset: f32,
        pulse_width: f32,
    ) -> Result<Self, LfoError> {
        if !frequency_hz.is_finite() {
            return Err(LfoError::NonFiniteFrequency);
        }
        if !(0.0..=100.0).contains(&frequency_hz) {
            return Err(LfoError::FrequencyOutOfRange);
        }
        if !phase_offset.is_finite() {
            return Err(LfoError::NonFinitePhaseOffset);
        }
        if !(0.0..1.0).contains(&phase_offset) {
            return Err(LfoError::PhaseOffsetOutOfRange);
        }
        if !pulse_width.is_finite() {
            return Err(LfoError::NonFinitePulseWidth);
        }
        if pulse_width <= 0.0 || pulse_width >= 1.0 {
            return Err(LfoError::PulseWidthOutOfRange);
        }
        Ok(Self {
            waveform,
            frequency_hz,
            phase_offset,
            pulse_width,
        })
    }

    /// Returns the selected waveform.
    pub const fn waveform(self) -> Waveform {
        self.waveform
    }

    /// Returns base frequency in hertz.
    pub const fn frequency_hz(self) -> f32 {
        self.frequency_hz
    }

    /// Returns phase offset in cycles.
    pub const fn phase_offset(self) -> f32 {
        self.phase_offset
    }

    /// Returns the square-wave positive width as a fraction of one cycle.
    pub const fn pulse_width(self) -> f32 {
        self.pulse_width
    }
}

/// The unipolar and bipolar readings from one oscillator sample.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LfoSample {
    /// Output scaled to `[0, 1]`.
    pub out: f32,
    /// Output scaled to `[-1, 1]`.
    pub bipolar_out: f32,
}

/// A deterministic oscillator advanced explicitly by one-millisecond samples.
#[derive(Debug, Clone, PartialEq)]
pub struct Lfo {
    config: LfoConfig,
    phase: f64,
    previous_sync: bool,
}

impl Lfo {
    /// Starts an oscillator at phase zero with sync low.
    #[must_use]
    pub const fn new(config: LfoConfig) -> Self {
        Self {
            config,
            phase: 0.0,
            previous_sync: false,
        }
    }

    /// Returns the current validated configuration.
    #[must_use]
    pub const fn config(&self) -> LfoConfig {
        self.config
    }

    /// Replaces the configuration without changing phase or sync history.
    pub fn set_config(&mut self, config: LfoConfig) {
        self.config = config;
    }

    /// Evaluates the current phase, then advances one millisecond.
    ///
    /// Finite frequency CV is clamped to `[0, 1]` and adds up to 100 Hz.
    /// Non-finite CV is neutralized to zero. A rising sync resets phase before
    /// this sample is evaluated; held-high sync does not reset repeatedly.
    ///
    /// # Panics
    ///
    /// Does not panic for any input value.
    pub fn next_sample(&mut self, frequency_cv: f32, sync: bool) -> LfoSample {
        if sync && !self.previous_sync {
            self.phase = 0.0;
        }
        self.previous_sync = sync;

        let phase = (self.phase + f64::from(self.config.phase_offset)).rem_euclid(1.0);
        let bipolar = match self.config.waveform {
            Waveform::Sine => (std::f64::consts::TAU * phase).sin(),
            Waveform::Triangle => 1.0 - 4.0 * (phase - 0.5).abs(),
            Waveform::SawUp => 2.0 * phase - 1.0,
            Waveform::SawDown => 1.0 - 2.0 * phase,
            Waveform::Square => {
                if phase < f64::from(self.config.pulse_width) {
                    1.0
                } else {
                    -1.0
                }
            }
        }
        .clamp(-1.0, 1.0);

        let cv = if frequency_cv.is_finite() {
            frequency_cv.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let frequency = (f64::from(self.config.frequency_hz) + 100.0 * f64::from(cv)).min(100.0);
        self.phase = (self.phase + frequency / 1000.0).rem_euclid(1.0);

        LfoSample {
            out: ((bipolar + 1.0) * 0.5) as f32,
            bipolar_out: bipolar as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn configured(waveform: Waveform, frequency: f32, offset: f32, width: f32) -> Lfo {
        Lfo::new(LfoConfig::try_new(waveform, frequency, offset, width).unwrap())
    }

    #[test]
    fn waveform_phase_orientation_and_cycle_are_exact() {
        let cases = [
            (Waveform::Sine, [0.0, 1.0, 0.0, -1.0, 0.0]),
            (Waveform::Triangle, [-1.0, 0.0, 1.0, 0.0, -1.0]),
            (Waveform::SawUp, [-1.0, -0.5, 0.0, 0.5, -1.0]),
            (Waveform::SawDown, [1.0, 0.5, 0.0, -0.5, 1.0]),
            (Waveform::Square, [1.0, 1.0, -1.0, -1.0, 1.0]),
        ];
        for (waveform, expected) in cases {
            let mut lfo = configured(waveform, 62.5, 0.0, 0.5);
            for index in 0..=16 {
                let sample = lfo.next_sample(0.0, false);
                if [0, 4, 8, 12, 16].contains(&index) {
                    let want = expected[(index / 4) as usize];
                    assert!(
                        (sample.bipolar_out - want).abs() < 1.0e-6,
                        "{waveform:?} sample {index}"
                    );
                }
            }
        }
    }

    #[test]
    fn square_width_and_cv_bounds_follow_the_sample_rate_contract() {
        let mut square = configured(Waveform::Square, 62.5, 0.0, 0.25);
        let samples: Vec<_> = (0..8)
            .map(|_| square.next_sample(0.0, false).bipolar_out)
            .collect();
        assert_eq!(samples, [1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0]);

        let mut cv = configured(Waveform::SawUp, 0.0, 0.0, 0.5);
        let expected = [-1.0, -0.875, -0.75, -0.625, -0.5];
        for (signal, want) in [0.625, 0.625, 0.625, 0.625, 0.625]
            .into_iter()
            .zip(expected)
        {
            let sample = cv.next_sample(signal, false);
            assert!((sample.bipolar_out - want).abs() < 1.0e-6);
            assert!(sample.out.is_finite() && (0.0..=1.0).contains(&sample.out));
        }
        let mut clamped = configured(Waveform::SawUp, 0.0, 0.0, 0.5);
        assert_eq!(clamped.next_sample(5.0, false).bipolar_out, -1.0);
        assert!((clamped.next_sample(-1.0, false).bipolar_out + 0.8).abs() < 1.0e-6);
        let mut neutral = configured(Waveform::SawUp, 0.0, 0.0, 0.5);
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let sample = neutral.next_sample(bad, false);
            assert_eq!(sample.bipolar_out, -1.0);
            assert_eq!(neutral.phase, 0.0);
        }
    }

    #[test]
    fn rising_sync_resets_once_and_zero_frequency_holds() {
        let mut lfo = configured(Waveform::SawUp, 0.0, 0.0, 0.5);
        let outputs = [
            lfo.next_sample(0.625, false).bipolar_out,
            lfo.next_sample(0.625, false).bipolar_out,
            lfo.next_sample(0.625, true).bipolar_out,
            lfo.next_sample(0.625, true).bipolar_out,
            lfo.next_sample(0.625, false).bipolar_out,
            lfo.next_sample(0.625, true).bipolar_out,
        ];
        assert_eq!(outputs, [-1.0, -0.875, -1.0, -0.875, -0.75, -1.0]);

        let mut stopped = configured(Waveform::Sine, 0.0, 0.0, 0.5);
        for _ in 0..4 {
            assert_eq!(stopped.next_sample(0.0, false).out, 0.5);
        }
        assert_eq!(stopped.phase, 0.0);
    }

    #[test]
    fn validation_and_waveform_names_are_exact() {
        for (wave, expected) in [
            (Waveform::Sine, "sine"),
            (Waveform::Triangle, "tri"),
            (Waveform::SawUp, "saw_up"),
            (Waveform::SawDown, "saw_down"),
            (Waveform::Square, "square"),
        ] {
            assert_eq!(wave.to_string(), expected);
            assert_eq!(expected.parse::<Waveform>(), Ok(wave));
        }
        assert!("triangle".parse::<Waveform>().is_err());
        assert_eq!(LfoConfig::default().frequency_hz(), 1.0);
        assert_eq!(
            LfoConfig::try_new(Waveform::Sine, f32::NAN, 0.0, 0.5),
            Err(LfoError::NonFiniteFrequency)
        );
        assert_eq!(
            LfoConfig::try_new(Waveform::Sine, 101.0, 0.0, 0.5),
            Err(LfoError::FrequencyOutOfRange)
        );
        assert_eq!(
            LfoConfig::try_new(Waveform::Sine, 1.0, 1.0, 0.5),
            Err(LfoError::PhaseOffsetOutOfRange)
        );
        assert_eq!(
            LfoConfig::try_new(Waveform::Sine, 1.0, f32::NAN, 0.5),
            Err(LfoError::NonFinitePhaseOffset)
        );
        assert_eq!(
            LfoConfig::try_new(Waveform::Sine, 1.0, 0.0, 0.0),
            Err(LfoError::PulseWidthOutOfRange)
        );
        assert_eq!(
            LfoConfig::try_new(Waveform::Sine, 1.0, 0.0, f32::INFINITY),
            Err(LfoError::NonFinitePulseWidth)
        );
    }
}
