//! Clock subsystem: master clock generation, slave clock tracking, and
//! shared scheduling types.

pub(crate) mod master;
pub(crate) mod passthrough;
pub(crate) mod slave;

/// Scheduling state for the next clock tick.
///
/// Used by both master and slave clocks to communicate when the next
/// MIDI clock pulse should fire and what musical position it represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickSchedule {
    /// Monotonic timestamp (in nanoseconds) of the next scheduled tick.
    pub next_tick_ns: u64,
    /// Interval between ticks in nanoseconds (derived from tempo).
    pub interval_ns: u64,
    /// Current subdivision within the beat (0..23 for 24 PPQN).
    pub subdivision: u8,
    /// Cumulative beat count since transport start.
    pub beat: u64,
}

/// Computes the interval in nanoseconds between MIDI clock ticks for a
/// given tempo.
///
/// MIDI clock runs at 24 pulses per quarter note (PPQN). Given a tempo
/// in BPM, the interval between pulses is:
///
/// ```text
/// interval = 60 / (bpm * 24) seconds
/// ```
///
/// # Examples
///
/// ```
/// use oxurack_rt::clock::interval_ns_from_bpm;
///
/// // At 120 BPM, each tick is ~20.83 ms
/// let interval = interval_ns_from_bpm(120.0);
/// assert_eq!(interval, 20_833_333);
/// ```
#[must_use]
pub fn interval_ns_from_bpm(bpm: f64) -> u64 {
    (60.0 / (bpm * 24.0) * 1_000_000_000.0) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_ns_from_bpm_120() {
        // 60 / (120 * 24) = 0.020833... seconds = 20_833_333 ns
        let interval = interval_ns_from_bpm(120.0);
        assert_eq!(interval, 20_833_333);
    }

    #[test]
    fn test_interval_ns_from_bpm_60() {
        // 60 / (60 * 24) = 0.041666... seconds = 41_666_666 ns
        let interval = interval_ns_from_bpm(60.0);
        assert_eq!(interval, 41_666_666);
    }

    #[test]
    fn test_interval_ns_from_bpm_240() {
        // 60 / (240 * 24) = 0.010416... seconds = 10_416_666 ns
        let interval = interval_ns_from_bpm(240.0);
        assert_eq!(interval, 10_416_666);
    }
}
