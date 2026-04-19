//! Master clock: generates MIDI clock ticks at a configurable tempo.
//!
//! The master clock produces a deterministic, drift-free sequence of
//! tick schedules at 24 PPQN. It performs no sleeping or I/O -- it is
//! pure state tracking, with the RT loop responsible for sleeping until
//! the scheduled time and then calling [`MasterClock::advance`].

use super::{TickSchedule, interval_ns_from_bpm};

/// A master MIDI clock that generates ticks at a configurable tempo.
///
/// The master clock owns the tempo and produces a steady stream of
/// 24-PPQN clock pulses. It is used when this system is the clock
/// source for the MIDI network.
///
/// # Drift Prevention
///
/// [`MasterClock::advance`] computes the next tick timestamp as
/// `self.next_tick_ns + self.interval_ns`, anchoring each tick to the
/// previous *ideal* time rather than any actual emission time. Over
/// thousands of ticks this keeps accumulated error at zero -- the only
/// rounding comes from the single `interval_ns_from_bpm` conversion.
#[derive(Debug)]
pub(crate) struct MasterClock {
    /// Current tempo in beats per minute.
    tempo_bpm: f64,
    /// Precomputed interval between ticks in nanoseconds.
    interval_ns: u64,
    /// Scheduled monotonic timestamp of the next tick.
    next_tick_ns: u64,
    /// Current subdivision within the beat (0..23).
    subdivision: u8,
    /// Cumulative beat count since start (or last reset).
    beat: u64,
}

impl MasterClock {
    /// Creates a new master clock at the given tempo.
    ///
    /// # Arguments
    ///
    /// * `tempo_bpm` - Initial tempo in beats per minute.
    /// * `start_ns` - Monotonic timestamp in nanoseconds to anchor the
    ///   first tick.
    pub(crate) fn new(tempo_bpm: f64, start_ns: u64) -> Self {
        let interval_ns = interval_ns_from_bpm(tempo_bpm);
        Self {
            tempo_bpm,
            interval_ns,
            next_tick_ns: start_ns,
            subdivision: 0,
            beat: 0,
        }
    }

    /// Returns the schedule for the next tick without advancing state.
    ///
    /// This is a pure peek -- calling it multiple times without an
    /// intervening [`advance`](Self::advance) returns identical values.
    pub(crate) fn next_tick(&self) -> TickSchedule {
        TickSchedule {
            next_tick_ns: self.next_tick_ns,
            interval_ns: self.interval_ns,
            subdivision: self.subdivision,
            beat: self.beat,
        }
    }

    /// Advances to the next tick position.
    ///
    /// Updates:
    /// - `next_tick_ns`: previous **scheduled** time + `interval_ns`
    ///   (not actual emission time -- this is the drift-prevention
    ///   invariant).
    /// - `subdivision`: wraps from 23 back to 0.
    /// - `beat`: increments when subdivision wraps.
    pub(crate) fn advance(&mut self) {
        self.next_tick_ns += self.interval_ns;
        self.subdivision += 1;
        if self.subdivision >= 24 {
            self.subdivision = 0;
            self.beat += 1;
        }
    }

    /// Changes the tempo, taking effect from the next tick.
    ///
    /// Recomputes `interval_ns` from the new BPM. The tick that was
    /// already scheduled completes on its old schedule; only subsequent
    /// calls to [`advance`](Self::advance) use the new interval.
    ///
    /// # Arguments
    ///
    /// * `bpm` - New tempo in beats per minute.
    pub(crate) fn set_tempo(&mut self, bpm: f64) {
        self.tempo_bpm = bpm;
        self.interval_ns = interval_ns_from_bpm(bpm);
    }

    /// Resets the musical position to beat 0, subdivision 0.
    ///
    /// The tempo and scheduled `next_tick_ns` are **not** changed -- only
    /// the beat/subdivision counters are cleared.
    pub(crate) fn reset(&mut self) {
        self.beat = 0;
        self.subdivision = 0;
    }

    /// Returns the current tempo in BPM.
    pub(crate) fn tempo(&self) -> f64 {
        self.tempo_bpm
    }

    /// Returns the current beat count.
    #[cfg(test)]
    pub(crate) fn beat(&self) -> u64 {
        self.beat
    }

    /// Returns the current subdivision within the beat (0..23).
    #[cfg(test)]
    pub(crate) fn subdivision(&self) -> u8 {
        self.subdivision
    }

    /// Sets position from a MIDI Song Position Pointer value.
    ///
    /// SPP is expressed in "MIDI beats" (sixteenth notes), ranging from
    /// 0 to 16383. The conversions are:
    ///
    /// - One quarter-note beat = 4 MIDI beats (sixteenth notes).
    /// - One MIDI beat = 6 MIDI clocks (24 PPQN / 4 = 6).
    ///
    /// # Arguments
    ///
    /// * `midi_beats` - Song Position Pointer value in sixteenth notes.
    pub(crate) fn set_position_from_spp(&mut self, midi_beats: u16) {
        self.beat = (midi_beats / 4) as u64;
        self.subdivision = ((midi_beats % 4) * 6) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new_initial_state() {
        let clock = MasterClock::new(120.0, 1000);
        assert_eq!(clock.subdivision(), 0);
        assert_eq!(clock.beat(), 0);
        assert_eq!(clock.tempo(), 120.0);
    }

    #[test]
    fn test_interval_at_120_bpm() {
        let clock = MasterClock::new(120.0, 0);
        let schedule = clock.next_tick();
        // 60 / (120 * 24) * 1_000_000_000 = 20_833_333.333... -> 20_833_333
        assert_eq!(schedule.interval_ns, 20_833_333);
    }

    #[test]
    fn test_subdivision_wraps_at_24() {
        let mut clock = MasterClock::new(120.0, 0);
        for _ in 0..24 {
            clock.advance();
        }
        assert_eq!(clock.subdivision(), 0);
        assert_eq!(clock.beat(), 1);
    }

    #[test]
    fn test_beat_increments_correctly() {
        let mut clock = MasterClock::new(120.0, 0);
        for _ in 0..48 {
            clock.advance();
        }
        assert_eq!(clock.beat(), 2);
        assert_eq!(clock.subdivision(), 0);
    }

    #[test]
    fn test_advance_schedules_are_drift_free() {
        let start_ns: u64 = 0;
        let mut clock = MasterClock::new(120.0, start_ns);
        let interval = clock.next_tick().interval_ns;
        assert_eq!(interval, 20_833_333);

        for _ in 0..24_000 {
            clock.advance();
        }

        let expected = start_ns + 24_000 * 20_833_333_u64;
        assert_eq!(clock.next_tick().next_tick_ns, expected);
    }

    #[test]
    fn test_set_tempo_changes_interval() {
        let mut clock = MasterClock::new(120.0, 0);

        // Advance a few ticks at 120 BPM.
        for _ in 0..5 {
            clock.advance();
        }

        // Record the scheduled time before tempo change.
        let schedule_before = clock.next_tick();
        assert_eq!(schedule_before.interval_ns, 20_833_333);

        // Change tempo to 140 BPM.
        clock.set_tempo(140.0);

        // The current next_tick_ns is unchanged (old schedule completes).
        assert_eq!(clock.next_tick().next_tick_ns, schedule_before.next_tick_ns);

        // But the interval now reflects 140 BPM.
        let expected_interval = interval_ns_from_bpm(140.0);
        assert_eq!(clock.next_tick().interval_ns, expected_interval);
        assert_eq!(clock.tempo(), 140.0);

        // After advancing, the new interval is used.
        clock.advance();
        let expected_next = schedule_before.next_tick_ns + expected_interval;
        assert_eq!(clock.next_tick().next_tick_ns, expected_next);
    }

    #[test]
    fn test_reset_clears_position() {
        let mut clock = MasterClock::new(120.0, 1000);

        // Advance 30 ticks: beat=1, subdivision=6.
        for _ in 0..30 {
            clock.advance();
        }
        assert_eq!(clock.beat(), 1);
        assert_eq!(clock.subdivision(), 6);

        let next_before = clock.next_tick().next_tick_ns;
        let tempo_before = clock.tempo();

        clock.reset();

        assert_eq!(clock.beat(), 0);
        assert_eq!(clock.subdivision(), 0);
        assert_eq!(clock.tempo(), tempo_before);
        assert_eq!(clock.next_tick().next_tick_ns, next_before);
    }

    #[test]
    fn test_set_position_from_spp() {
        let mut clock = MasterClock::new(120.0, 0);

        // SPP 96: 96/4 = 24 beats, 96%4 = 0 -> subdivision = 0.
        clock.set_position_from_spp(96);
        assert_eq!(clock.beat(), 24);
        assert_eq!(clock.subdivision(), 0);

        // SPP 7: 7/4 = 1 beat, 7%4 = 3 -> subdivision = 3*6 = 18.
        clock.set_position_from_spp(7);
        assert_eq!(clock.beat(), 1);
        assert_eq!(clock.subdivision(), 18);
    }

    #[test]
    fn test_next_tick_returns_current_schedule() {
        let clock = MasterClock::new(120.0, 5000);

        let first = clock.next_tick();
        let second = clock.next_tick();

        // Peek is idempotent -- same result without advance.
        assert_eq!(first, second);
    }
}
