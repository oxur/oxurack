use std::num::NonZeroU32;

/// Clock divider for a MIDI Turing Machine.
///
/// Divides an incoming clock signal by a configurable integer factor.
/// On every call to [`tick`](ClockDivider::tick), an internal counter
/// advances; the method returns `true` exactly once per `division` ticks,
/// then resets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClockDivider {
    division: NonZeroU32,
    counter: u32,
}

impl ClockDivider {
    /// Create a new `ClockDivider` that fires every `division` ticks.
    ///
    /// The internal counter starts at zero.
    pub fn new(division: NonZeroU32) -> Self {
        Self {
            division,
            counter: 0,
        }
    }

    /// Advance the clock by one tick.
    ///
    /// Returns `true` when the divider fires (i.e. when the internal
    /// counter reaches `division`), at which point the counter resets to
    /// zero.
    pub fn tick(&mut self) -> bool {
        self.counter += 1;
        if self.counter >= self.division.get() {
            self.counter = 0;
            true
        } else {
            false
        }
    }

    /// Reset the internal counter to zero without changing the division.
    pub fn reset(&mut self) {
        self.counter = 0;
    }

    /// Return the configured clock division factor.
    pub fn division(&self) -> NonZeroU32 {
        self.division
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nz(n: u32) -> NonZeroU32 {
        NonZeroU32::new(n).unwrap()
    }

    #[test]
    fn div2_fires_every_other_tick() {
        let mut clk = ClockDivider::new(nz(2));
        let results: Vec<bool> = (0..6).map(|_| clk.tick()).collect();
        assert_eq!(results, vec![false, true, false, true, false, true]);
    }

    #[test]
    fn div4_fires_every_fourth_tick() {
        let mut clk = ClockDivider::new(nz(4));
        let results: Vec<bool> = (0..8).map(|_| clk.tick()).collect();

        // Should fire on ticks 4 and 8 (indices 3 and 7).
        for (i, &fired) in results.iter().enumerate() {
            if i == 3 || i == 7 {
                assert!(fired, "expected fire on tick {}", i + 1);
            } else {
                assert!(!fired, "unexpected fire on tick {}", i + 1);
            }
        }
    }

    #[test]
    fn reset_restarts_counters() {
        let mut clk = ClockDivider::new(nz(3));

        // Tick twice (counter becomes 2).
        assert!(!clk.tick());
        assert!(!clk.tick());

        clk.reset();

        // After reset the pattern should restart from scratch.
        let results: Vec<bool> = (0..3).map(|_| clk.tick()).collect();
        assert_eq!(results, vec![false, false, true]);
    }

    #[test]
    fn div1_always_fires() {
        let mut clk = ClockDivider::new(nz(1));
        for _ in 0..5 {
            assert!(clk.tick());
        }
    }

    #[test]
    fn test_division_getter() {
        let clk2 = ClockDivider::new(nz(2));
        assert_eq!(clk2.division(), nz(2));

        let clk4 = ClockDivider::new(nz(4));
        assert_eq!(clk4.division(), nz(4));

        let clk1 = ClockDivider::new(nz(1));
        assert_eq!(clk1.division(), nz(1));
    }

    #[test]
    fn test_div2_many_ticks() {
        let mut clk = ClockDivider::new(nz(2));
        let fire_count: usize = (0..100).filter(|_| clk.tick()).count();
        assert_eq!(
            fire_count, 50,
            "div2 should fire exactly 50 times in 100 ticks"
        );
    }
}
