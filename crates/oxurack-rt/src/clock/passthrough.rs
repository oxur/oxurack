//! Passthrough clock mode: receives external clock and re-emits it
//! on output ports, optionally multiplied or divided.
//!
//! Unlike the [`super::slave::SlaveClock`], passthrough mode does not
//! apply PLL smoothing or oscillator interpolation. It forwards clock
//! ticks directly and is suitable for deterministic clock distribution
//! chains where jitter smoothing is undesirable.

/// Passthrough clock state.
///
/// Tracks incoming MIDI clock ticks and computes how many output ticks
/// to emit based on the configured multiply/divide ratio.
///
/// # Multiply / Divide
///
/// The ratio `multiply / divide` determines the output-to-input tick
/// ratio. For example:
///
/// - `multiply=1, divide=1`: 1:1 forwarding (pass through unchanged).
/// - `multiply=2, divide=1`: emit 2 output ticks per input tick.
/// - `multiply=1, divide=2`: emit 1 output tick per 2 input ticks.
/// - `multiply=3, divide=2`: emit 3 output ticks per 2 input ticks.
///
/// Integer math is used to avoid floating-point accumulation errors:
/// each input tick increments `input_count`, and the difference
/// `(count * multiply) / divide - ((count - 1) * multiply) / divide`
/// determines how many output ticks to fire.
#[derive(Debug)]
pub(crate) struct PassthroughClock {
    /// Clock multiplication factor (clamped to >= 1).
    multiply: u8,
    /// Clock division factor (clamped to >= 1).
    divide: u8,
    /// Timeout threshold in nanoseconds for dropout detection.
    timeout_ns: u64,

    /// Number of incoming clock ticks received since start/reset.
    input_count: u64,
    /// Current subdivision within the output beat (0..23 for 24 PPQN).
    output_subdivision: u8,
    /// Cumulative output beat count since transport start/reset.
    output_beat: u64,

    /// Timestamp of the most recently received tick, for dropout
    /// detection.
    last_tick_ns: Option<u64>,

    /// Transport running state.
    is_running: bool,
}

impl PassthroughClock {
    /// Creates a new passthrough clock.
    ///
    /// Both `multiply` and `divide` are clamped to a minimum of 1.
    ///
    /// # Arguments
    ///
    /// * `multiply` - Clock multiplication factor.
    /// * `divide` - Clock division factor.
    /// * `timeout_ns` - Nanoseconds of silence before a dropout is
    ///   reported.
    pub(crate) fn new(multiply: u8, divide: u8, timeout_ns: u64) -> Self {
        Self {
            multiply: multiply.max(1),
            divide: divide.max(1),
            timeout_ns,
            input_count: 0,
            output_subdivision: 0,
            output_beat: 0,
            last_tick_ns: None,
            is_running: true,
        }
    }

    /// Processes an incoming clock byte (0xF8).
    ///
    /// Returns the number of output ticks to emit for this input tick,
    /// which may be 0 (when division absorbs the tick), 1 (1:1 or
    /// exact division point), or more (when multiplying).
    ///
    /// # Arguments
    ///
    /// * `timestamp_ns` - Monotonic timestamp of the received tick.
    pub(crate) fn feed_clock(&mut self, timestamp_ns: u64) -> u32 {
        self.last_tick_ns = Some(timestamp_ns);
        self.input_count += 1;

        // Integer math avoids floating-point accumulation errors.
        // The total number of output ticks after `n` input ticks is
        // `(n * multiply) / divide`. The delta is this tick's
        // contribution.
        let prev_total =
            ((self.input_count - 1) * self.multiply as u64) / self.divide as u64;
        let curr_total =
            (self.input_count * self.multiply as u64) / self.divide as u64;

        (curr_total - prev_total) as u32
    }

    /// Advances the output position by one tick.
    ///
    /// Increments the subdivision counter, wrapping at 24 (one full
    /// beat at 24 PPQN) and incrementing the beat count.
    pub(crate) fn advance_output(&mut self) {
        self.output_subdivision += 1;
        if self.output_subdivision >= 24 {
            self.output_subdivision = 0;
            self.output_beat += 1;
        }
    }

    /// Returns the current output subdivision within the beat (0..23).
    pub(crate) fn output_subdivision(&self) -> u8 {
        self.output_subdivision
    }

    /// Returns the current cumulative output beat count.
    pub(crate) fn output_beat(&self) -> u64 {
        self.output_beat
    }

    /// Checks whether the external clock has dropped out.
    ///
    /// Returns `true` if the time since the last received tick exceeds
    /// the configured `timeout_ns`. Returns `false` if no tick has
    /// been received yet (nothing to time out on).
    ///
    /// # Arguments
    ///
    /// * `now_ns` - Current monotonic timestamp in nanoseconds.
    pub(crate) fn check_dropout(&self, now_ns: u64) -> bool {
        if let Some(last) = self.last_tick_ns {
            now_ns.saturating_sub(last) > self.timeout_ns
        } else {
            false
        }
    }

    /// Resets the musical position and input counter.
    ///
    /// Used when a MIDI Transport Start message is received.
    pub(crate) fn reset(&mut self) {
        self.output_subdivision = 0;
        self.output_beat = 0;
        self.input_count = 0;
    }

    /// Sets the transport running state.
    pub(crate) fn set_running(&mut self, running: bool) {
        self.is_running = running;
    }

    /// Returns `true` if the transport is running.
    pub(crate) fn is_running(&self) -> bool {
        self.is_running
    }

    /// Sets the musical position from a MIDI Song Position Pointer.
    ///
    /// SPP is expressed in "MIDI beats" (sixteenth notes). The
    /// conversions are:
    /// - One quarter-note beat = 4 MIDI beats (sixteenth notes).
    /// - One MIDI beat = 6 MIDI clocks (24 PPQN / 4 = 6).
    ///
    /// # Arguments
    ///
    /// * `midi_beats` - Song Position Pointer value in sixteenth notes.
    pub(crate) fn set_position_from_spp(&mut self, midi_beats: u16) {
        self.output_beat = (midi_beats / 4) as u64;
        self.output_subdivision = ((midi_beats % 4) * 6) as u8;
    }
}

// ── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_passthrough_1_to_1() {
        let mut clock = PassthroughClock::new(1, 1, 1_000_000_000);

        let mut total_output = 0u32;
        for i in 0..24 {
            let ticks = clock.feed_clock(i * 20_833_333);
            total_output += ticks;
        }

        assert_eq!(total_output, 24, "1:1 should produce 24 output ticks from 24 inputs");
    }

    #[test]
    fn test_passthrough_multiply_2() {
        let mut clock = PassthroughClock::new(2, 1, 1_000_000_000);

        let mut total_output = 0u32;
        for i in 0..24 {
            let ticks = clock.feed_clock(i * 20_833_333);
            total_output += ticks;
        }

        assert_eq!(total_output, 48, "multiply=2 should produce 48 output ticks from 24 inputs");
    }

    #[test]
    fn test_passthrough_divide_2() {
        let mut clock = PassthroughClock::new(1, 2, 1_000_000_000);

        let mut total_output = 0u32;
        for i in 0..24 {
            let ticks = clock.feed_clock(i * 20_833_333);
            total_output += ticks;
        }

        assert_eq!(total_output, 12, "divide=2 should produce 12 output ticks from 24 inputs");
    }

    #[test]
    fn test_passthrough_multiply_3_divide_2() {
        let mut clock = PassthroughClock::new(3, 2, 1_000_000_000);

        let mut total_output = 0u32;
        for i in 0..24 {
            let ticks = clock.feed_clock(i * 20_833_333);
            total_output += ticks;
        }

        assert_eq!(total_output, 36, "multiply=3, divide=2 should produce 36 output ticks from 24 inputs");
    }

    #[test]
    fn test_passthrough_dropout() {
        let timeout_ns = 500_000_000; // 500 ms
        let mut clock = PassthroughClock::new(1, 1, timeout_ns);

        // Feed one tick.
        clock.feed_clock(1_000_000);

        // Not enough time has passed: no dropout.
        assert!(
            !clock.check_dropout(100_000_000),
            "should not be a dropout within the timeout window"
        );

        // Enough time has passed: dropout.
        assert!(
            clock.check_dropout(1_000_000 + timeout_ns + 1),
            "should detect dropout after timeout"
        );
    }

    #[test]
    fn test_passthrough_no_dropout_before_ticks() {
        let clock = PassthroughClock::new(1, 1, 500_000_000);

        // No ticks received: check_dropout should return false.
        assert!(
            !clock.check_dropout(10_000_000_000),
            "should not report dropout when no tick has been received"
        );
    }

    #[test]
    fn test_passthrough_reset() {
        let mut clock = PassthroughClock::new(1, 1, 1_000_000_000);

        // Feed some ticks and advance output.
        for i in 0..30 {
            let out = clock.feed_clock(i * 20_833_333);
            for _ in 0..out {
                clock.advance_output();
            }
        }

        assert!(clock.output_beat() > 0 || clock.output_subdivision() > 0,
            "position should have advanced");

        clock.reset();

        assert_eq!(clock.output_subdivision(), 0, "subdivision should be 0 after reset");
        assert_eq!(clock.output_beat(), 0, "beat should be 0 after reset");
    }

    #[test]
    fn test_passthrough_advance_output_wraps_at_24() {
        let mut clock = PassthroughClock::new(1, 1, 1_000_000_000);

        // Advance 24 times: should wrap to beat 1, subdivision 0.
        for _ in 0..24 {
            clock.advance_output();
        }

        assert_eq!(clock.output_subdivision(), 0);
        assert_eq!(clock.output_beat(), 1);

        // Advance 6 more.
        for _ in 0..6 {
            clock.advance_output();
        }

        assert_eq!(clock.output_subdivision(), 6);
        assert_eq!(clock.output_beat(), 1);
    }

    #[test]
    fn test_passthrough_running_state() {
        let mut clock = PassthroughClock::new(1, 1, 1_000_000_000);

        assert!(clock.is_running(), "should be running initially");

        clock.set_running(false);
        assert!(!clock.is_running(), "should be stopped after set_running(false)");

        clock.set_running(true);
        assert!(clock.is_running(), "should be running after set_running(true)");
    }

    #[test]
    fn test_passthrough_multiply_and_divide_clamped_to_1() {
        // multiply=0 and divide=0 should be clamped to 1.
        let mut clock = PassthroughClock::new(0, 0, 1_000_000_000);

        let mut total_output = 0u32;
        for i in 0..24 {
            total_output += clock.feed_clock(i * 20_833_333);
        }

        assert_eq!(total_output, 24, "clamped (0,0) should behave as (1,1)");
    }

    #[test]
    fn test_passthrough_set_position_from_spp() {
        let mut clock = PassthroughClock::new(1, 1, 1_000_000_000);

        // SPP 7: 7/4 = 1 beat, 7%4 = 3 -> subdivision = 3*6 = 18.
        clock.set_position_from_spp(7);
        assert_eq!(clock.output_beat(), 1);
        assert_eq!(clock.output_subdivision(), 18);

        // SPP 0: position 0.
        clock.set_position_from_spp(0);
        assert_eq!(clock.output_beat(), 0);
        assert_eq!(clock.output_subdivision(), 0);
    }

    #[test]
    fn test_passthrough_divide_produces_even_distribution() {
        // With divide=3, every 3rd input tick should produce 1 output tick.
        let mut clock = PassthroughClock::new(1, 3, 1_000_000_000);

        let mut outputs = Vec::new();
        for i in 0..24 {
            outputs.push(clock.feed_clock(i * 20_833_333));
        }

        let total: u32 = outputs.iter().sum();
        assert_eq!(total, 8, "divide=3 from 24 inputs should produce 8 outputs");

        // Every 3rd tick should produce 1, the others 0.
        for (i, &out) in outputs.iter().enumerate() {
            if (i + 1) % 3 == 0 {
                assert_eq!(out, 1, "tick {} should produce 1 output", i);
            } else {
                assert_eq!(out, 0, "tick {} should produce 0 outputs", i);
            }
        }
    }
}
