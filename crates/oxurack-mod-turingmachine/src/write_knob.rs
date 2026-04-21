use rand::RngExt;

/// Models the WRITE knob on a MIDI Turing Machine.
///
/// The knob controls the probability of keeping the existing feedback bit
/// versus substituting a fresh random bit. At probability 1.0 the feedback
/// bit is always preserved (fully locked loop); at 0.0 every bit is
/// replaced with random noise (fully random sequence).
#[derive(Debug, Clone, PartialEq)]
pub struct WriteKnob {
    probability: f32,
}

impl WriteKnob {
    /// Creates a new `WriteKnob` with the given probability, clamped to
    /// the valid range \[0.0, 1.0\].
    pub fn new(probability: f32) -> Self {
        Self {
            probability: probability.clamp(0.0, 1.0),
        }
    }

    /// Sets the probability, clamping to \[0.0, 1.0\].
    pub fn set_probability(&mut self, value: f32) {
        self.probability = value.clamp(0.0, 1.0);
    }

    /// Adds `offset` to the current probability and clamps the result to
    /// \[0.0, 1.0\].
    pub fn modulate(&mut self, offset: f32) {
        self.probability = (self.probability + offset).clamp(0.0, 1.0);
    }

    /// Resolves the write decision for a single bit.
    ///
    /// With probability [`self.probability`], the `feedback_bit` is returned
    /// unchanged. Otherwise a fresh random bit is generated.
    pub fn resolve(&self, feedback_bit: bool, rng: &mut impl rand::Rng) -> bool {
        let roll: f32 = rng.random::<f32>();
        if roll < self.probability {
            feedback_bit
        } else {
            rng.random::<bool>()
        }
    }

    /// Returns the current probability value.
    pub fn probability(&self) -> f32 {
        self.probability
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    #[test]
    fn probability_1_always_keeps() {
        let knob = WriteKnob::new(1.0);
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            assert!(knob.resolve(true, &mut rng));
        }
        for _ in 0..100 {
            assert!(!knob.resolve(false, &mut rng));
        }
    }

    #[test]
    fn probability_0_ignores_feedback() {
        let knob = WriteKnob::new(0.0);
        let mut rng = SmallRng::seed_from_u64(99);

        let mut saw_different = false;
        for _ in 0..1000 {
            if !knob.resolve(true, &mut rng) {
                saw_different = true;
                break;
            }
        }
        assert!(
            saw_different,
            "at probability 0.0, at least some results should differ from feedback_bit"
        );
    }

    #[test]
    fn modulate_clamps() {
        let mut knob = WriteKnob::new(0.5);

        knob.modulate(999.0);
        assert_eq!(knob.probability(), 1.0);

        knob.modulate(-999.0);
        assert_eq!(knob.probability(), 0.0);
    }

    #[test]
    fn new_clamps() {
        let high = WriteKnob::new(2.0);
        assert_eq!(high.probability(), 1.0);

        let low = WriteKnob::new(-1.0);
        assert_eq!(low.probability(), 0.0);
    }
}
