//! Slave clock: tracks an external MIDI clock source using a PLL-based
//! tempo estimator and a free-running oscillator for inter-tick
//! interpolation.

use super::TickSchedule;
use crate::TransportEvent;

// ── Milestone 4.1: TempoEstimator ──────────────────────────────────

/// Estimates tempo from incoming MIDI clock tick timestamps using
/// an exponential moving average filter.
///
/// The estimator records raw tick timestamps, computes instantaneous
/// intervals between consecutive ticks, and smooths them with an EMA
/// filter. It requires at least 4 ticks before reporting a locked
/// estimate, providing resilience against startup transients.
///
/// # Tempo Jump Detection
///
/// If an instantaneous interval exceeds 3x the current filtered
/// estimate, the filter state is reset to handle abrupt tempo changes
/// without a long convergence tail.
///
/// # Dropout Detection
///
/// [`TempoEstimator::check_dropout`] detects when the external clock
/// has gone silent for too long (>4x the estimated interval), resetting
/// the estimator to an unlocked state.
#[derive(Debug)]
pub(crate) struct TempoEstimator {
    /// Circular buffer of recent tick timestamps.
    timestamps: Vec<u64>,
    /// Write position in the circular buffer.
    head: usize,
    /// Number of valid entries recorded so far.
    count: usize,
    /// Size of the circular buffer (smoothing window).
    window_size: usize,

    /// Filtered tempo estimate (EMA-smoothed interval in nanoseconds).
    filtered_interval_ns: Option<f64>,

    /// EMA smoothing factor: `alpha = 2.0 / (window + 1)`.
    alpha: f64,

    /// Timestamp of the most recently received tick, for dropout detection.
    last_tick_ns: Option<u64>,
}

impl TempoEstimator {
    /// Creates a new tempo estimator with the given smoothing window size.
    ///
    /// The window size controls the EMA time constant. Typical values are
    /// 8-12 ticks (roughly 1/3 to 1/2 of a beat at 24 PPQN).
    ///
    /// # Arguments
    ///
    /// * `window_size` - Number of ticks for the smoothing window.
    pub(crate) fn new(window_size: usize) -> Self {
        let window_size = window_size.max(2);
        Self {
            timestamps: vec![0; window_size],
            head: 0,
            count: 0,
            window_size,
            filtered_interval_ns: None,
            alpha: 2.0 / (window_size as f64 + 1.0),
            last_tick_ns: None,
        }
    }

    /// Records an incoming clock tick timestamp and updates the estimate.
    ///
    /// On the first tick, only the timestamp is stored. On ticks 2-3,
    /// intervals are accumulated but the estimator does not report locked.
    /// From tick 4 onward, the EMA filter produces a stable estimate.
    ///
    /// If an instantaneous interval exceeds 3x the current filtered
    /// estimate, the filter is reset (tempo jump detection).
    ///
    /// # Arguments
    ///
    /// * `timestamp_ns` - Monotonic timestamp of the received tick.
    pub(crate) fn feed_tick(&mut self, timestamp_ns: u64) {
        let prev_tick = self.last_tick_ns;
        self.last_tick_ns = Some(timestamp_ns);

        // Store timestamp in circular buffer.
        self.timestamps[self.head] = timestamp_ns;
        self.head = (self.head + 1) % self.window_size;
        self.count += 1;

        // On the first tick there is no previous timestamp to compute
        // an interval from.
        let Some(prev) = prev_tick else {
            return;
        };

        let instantaneous = timestamp_ns.saturating_sub(prev) as f64;

        // Tempo jump detection: if the instantaneous interval is more
        // than 3x the current filtered estimate, reset the filter.
        if let Some(filtered) = self.filtered_interval_ns
            && (instantaneous > 3.0 * filtered || instantaneous < filtered / 3.0)
        {
            // Reset filter state but keep last_tick_ns so the next
            // tick can compute a fresh interval.
            self.filtered_interval_ns = Some(instantaneous);
            self.count = 2; // We have exactly one interval now.
            return;
        }

        // Update EMA filter.
        match self.filtered_interval_ns {
            Some(filtered) => {
                self.filtered_interval_ns =
                    Some(self.alpha * instantaneous + (1.0 - self.alpha) * filtered);
            }
            None => {
                // First interval: seed the filter.
                self.filtered_interval_ns = Some(instantaneous);
            }
        }
    }

    /// Returns the filtered interval in nanoseconds, or `None` if fewer
    /// than 4 ticks have been received.
    pub(crate) fn estimated_interval_ns(&self) -> Option<u64> {
        if self.count < 4 {
            return None;
        }
        self.filtered_interval_ns.map(|f| f as u64)
    }

    /// Returns the current estimated tempo in BPM, or `None` if not
    /// enough ticks have been received to form an estimate.
    ///
    /// Converts from the filtered interval using the standard MIDI clock
    /// formula: `BPM = 60 / (interval_ns * 24 / 1_000_000_000)`.
    pub(crate) fn estimated_bpm(&self) -> Option<f64> {
        let interval_ns = self.estimated_interval_ns()?;
        if interval_ns == 0 {
            return None;
        }
        Some(60.0 / (interval_ns as f64 * 24.0 / 1_000_000_000.0))
    }

    /// Returns `true` if the estimator has a valid tempo estimate
    /// (at least 4 ticks received).
    pub(crate) fn is_locked(&self) -> bool {
        self.count >= 4 && self.filtered_interval_ns.is_some()
    }

    /// Checks whether the external clock has dropped out.
    ///
    /// Returns `true` if the time since the last tick exceeds 4x the
    /// estimated interval. When a dropout is detected, the estimator
    /// resets to an unlocked state.
    ///
    /// # Arguments
    ///
    /// * `now_ns` - Current monotonic timestamp in nanoseconds.
    pub(crate) fn check_dropout(&mut self, now_ns: u64) -> bool {
        let (Some(last), Some(interval_ns)) = (self.last_tick_ns, self.filtered_interval_ns) else {
            return false;
        };

        let elapsed = now_ns.saturating_sub(last) as f64;
        if elapsed > 4.0 * interval_ns {
            self.reset();
            return true;
        }

        false
    }

    /// Resets the estimator to an unlocked state.
    fn reset(&mut self) {
        self.count = 0;
        self.head = 0;
        self.filtered_interval_ns = None;
        self.last_tick_ns = None;
    }

    /// Returns the raw filtered interval (for internal use by
    /// `SlaveClock` which needs the value even below 4 ticks for
    /// oscillator seeding).
    fn raw_filtered_interval_ns(&self) -> Option<f64> {
        self.filtered_interval_ns
    }
}

// ── Milestone 4.2: SlaveOscillator ─────────────────────────────────

/// A free-running oscillator that generates smoothed internal tick
/// schedules based on the tempo estimator's output.
///
/// Rather than forwarding raw external clock timestamps (which carry
/// USB and OS jitter), the oscillator maintains its own internal tick
/// timeline and applies proportional phase corrections when external
/// ticks arrive, producing a much smoother output stream.
#[derive(Debug)]
pub(crate) struct SlaveOscillator {
    /// Next scheduled internal tick timestamp (in nanoseconds).
    /// `None` when the oscillator is stopped or not yet started.
    next_tick_ns: Option<u64>,
    /// Current subdivision within the beat (0..23 for 24 PPQN).
    subdivision: u8,
    /// Cumulative beat count since transport start.
    beat: u64,
    /// Phase correction gain (0.0..1.0). Controls how aggressively
    /// the oscillator tracks the external clock phase.
    phase_gain: f64,
}

impl SlaveOscillator {
    /// Creates a new slave oscillator in the stopped state.
    pub(crate) fn new() -> Self {
        Self {
            next_tick_ns: None,
            subdivision: 0,
            beat: 0,
            phase_gain: 0.2,
        }
    }

    /// Synchronizes the oscillator to an incoming external clock tick.
    ///
    /// If the oscillator has not started, it begins producing ticks
    /// starting one interval after the external tick. If already running,
    /// a proportional phase correction is applied to gently steer the
    /// internal tick schedule toward the external source.
    ///
    /// # Arguments
    ///
    /// * `external_tick_ns` - Monotonic timestamp of the external tick.
    /// * `estimated_interval_ns` - Current best estimate of the tick
    ///   interval in nanoseconds.
    pub(crate) fn sync_to_external(&mut self, external_tick_ns: u64, estimated_interval_ns: u64) {
        match self.next_tick_ns {
            None => {
                // First sync: start the oscillator one interval from now.
                self.next_tick_ns = Some(external_tick_ns + estimated_interval_ns);
            }
            Some(expected) => {
                // Compute phase error: positive means external tick
                // arrived later than expected (we're running fast).
                let phase_error = external_tick_ns as f64 - expected as f64;
                let correction = (self.phase_gain * phase_error) as i64;

                // Apply correction to the next scheduled tick.
                if correction >= 0 {
                    self.next_tick_ns = Some(expected + correction as u64);
                } else {
                    self.next_tick_ns = Some(expected.saturating_sub(correction.unsigned_abs()));
                }
            }
        }
    }

    /// Returns the schedule for the next internal tick, or `None` if
    /// the oscillator is not running.
    ///
    /// # Arguments
    ///
    /// * `estimated_interval_ns` - Current best estimate of the tick
    ///   interval, used to populate the schedule's interval field.
    pub(crate) fn next_tick(&self, estimated_interval_ns: u64) -> Option<TickSchedule> {
        self.next_tick_ns.map(|next| TickSchedule {
            next_tick_ns: next,
            interval_ns: estimated_interval_ns,
            subdivision: self.subdivision,
            beat: self.beat,
        })
    }

    /// Advances to the next tick position.
    ///
    /// Updates `next_tick_ns` by adding the estimated interval, and
    /// increments the subdivision counter (wrapping at 24 and bumping
    /// the beat count).
    ///
    /// # Arguments
    ///
    /// * `estimated_interval_ns` - Current best estimate of the tick
    ///   interval in nanoseconds.
    pub(crate) fn advance(&mut self, estimated_interval_ns: u64) {
        if let Some(ref mut next) = self.next_tick_ns {
            *next += estimated_interval_ns;
        }
        self.subdivision += 1;
        if self.subdivision >= 24 {
            self.subdivision = 0;
            self.beat += 1;
        }
    }

    /// Resets the musical position to beat 0, subdivision 0.
    ///
    /// Used when a MIDI Transport Start message is received.
    pub(crate) fn reset_position(&mut self) {
        self.beat = 0;
        self.subdivision = 0;
    }

    /// Stops the oscillator, ceasing tick production.
    ///
    /// Used when a MIDI Transport Stop message is received.
    pub(crate) fn stop(&mut self) {
        self.next_tick_ns = None;
    }

    /// Resumes the oscillator from a stopped state.
    ///
    /// Sets the next tick to occur one interval from `now_ns`.
    ///
    /// # Arguments
    ///
    /// * `now_ns` - Current monotonic timestamp in nanoseconds.
    /// * `estimated_interval_ns` - Current best estimate of the tick
    ///   interval in nanoseconds.
    pub(crate) fn resume(&mut self, now_ns: u64, estimated_interval_ns: u64) {
        if self.next_tick_ns.is_none() {
            self.next_tick_ns = Some(now_ns + estimated_interval_ns);
        }
    }

    /// Sets position from a MIDI Song Position Pointer value.
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
        self.beat = (midi_beats / 4) as u64;
        self.subdivision = ((midi_beats % 4) * 6) as u8;
    }
}

// ── Milestone 4.3: SlaveClock facade ───────────────────────────────

/// A slave MIDI clock that tracks an external clock source.
///
/// Combines a [`TempoEstimator`] for measuring incoming tempo with a
/// [`SlaveOscillator`] for smooth inter-tick interpolation. Produces
/// the same [`TickSchedule`] interface as [`super::master::MasterClock`].
///
/// # PLL Architecture
///
/// The slave clock implements a phase-locked loop:
///
/// 1. **Phase detector**: Compares expected internal tick time against
///    actual external tick arrival.
/// 2. **Loop filter**: EMA filter in `TempoEstimator` smooths
///    instantaneous tempo measurements.
/// 3. **Controlled oscillator**: `SlaveOscillator` generates internal
///    ticks at the filtered tempo, with proportional phase correction.
#[derive(Debug)]
pub(crate) struct SlaveClock {
    /// Tempo estimator (EMA-filtered interval from external ticks).
    estimator: TempoEstimator,
    /// Smooth internal tick generator.
    oscillator: SlaveOscillator,
    /// Timeout threshold in nanoseconds for dropout detection.
    timeout_ns: u64,
    /// Transport state: `true` when playback is running (Start or
    /// Continue received), `false` after Stop.
    is_running: bool,
}

impl SlaveClock {
    /// Creates a new slave clock with a default estimator window of 8.
    ///
    /// # Arguments
    ///
    /// * `timeout_ns` - If no external tick arrives within this many
    ///   nanoseconds, the clock reports a dropout.
    pub(crate) fn new(timeout_ns: u64) -> Self {
        Self {
            estimator: TempoEstimator::new(8),
            oscillator: SlaveOscillator::new(),
            timeout_ns,
            is_running: true,
        }
    }

    /// Processes an incoming MIDI Clock byte (0xF8).
    ///
    /// Feeds the timestamp to the tempo estimator and, if locked,
    /// synchronizes the oscillator to the external tick phase.
    ///
    /// # Arguments
    ///
    /// * `timestamp_ns` - Monotonic timestamp of the received clock byte.
    pub(crate) fn feed_clock_byte(&mut self, timestamp_ns: u64) {
        self.estimator.feed_tick(timestamp_ns);

        if let Some(interval) = self.estimator.estimated_interval_ns() {
            self.oscillator.sync_to_external(timestamp_ns, interval);
        } else if let Some(raw_interval) = self.estimator.raw_filtered_interval_ns() {
            // Even before fully locked, start the oscillator with the
            // best estimate we have so it is ready when lock is acquired.
            let interval = raw_interval as u64;
            if interval > 0 {
                self.oscillator.sync_to_external(timestamp_ns, interval);
            }
        }
    }

    /// Processes a MIDI transport event (Start, Stop, Continue).
    ///
    /// - **Start**: Resets the musical position and sets running state.
    /// - **Stop**: Stops the oscillator and clears running state.
    /// - **Continue**: Resumes the oscillator if an interval estimate
    ///   is available.
    ///
    /// # Arguments
    ///
    /// * `event` - The transport event to process.
    /// * `now_ns` - Current monotonic timestamp (used for Continue
    ///   to schedule the next tick).
    pub(crate) fn feed_transport(&mut self, event: TransportEvent, now_ns: u64) {
        match event {
            TransportEvent::Start => {
                self.oscillator.reset_position();
                self.is_running = true;
            }
            TransportEvent::Stop => {
                self.oscillator.stop();
                self.is_running = false;
            }
            TransportEvent::Continue => {
                self.is_running = true;
                if let Some(interval) = self.estimator.estimated_interval_ns() {
                    self.oscillator.resume(now_ns, interval);
                } else if let Some(raw) = self.estimator.raw_filtered_interval_ns() {
                    let interval = raw as u64;
                    if interval > 0 {
                        self.oscillator.resume(now_ns, interval);
                    }
                }
            }
        }
    }

    /// Processes a MIDI Song Position Pointer message.
    ///
    /// # Arguments
    ///
    /// * `position` - 14-bit song position in MIDI beats (sixteenth notes).
    pub(crate) fn feed_spp(&mut self, position: u16) {
        self.oscillator.set_position_from_spp(position);
    }

    /// Returns the schedule for the next internal tick, or `None` if
    /// the clock is not running or not locked.
    pub(crate) fn next_tick(&self) -> Option<TickSchedule> {
        if !self.is_running || !self.estimator.is_locked() {
            return None;
        }
        let interval = self.estimator.estimated_interval_ns()?;
        self.oscillator.next_tick(interval)
    }

    /// Advances the oscillator to the next tick position.
    pub(crate) fn advance(&mut self) {
        if let Some(interval) = self.estimator.estimated_interval_ns() {
            self.oscillator.advance(interval);
        }
    }

    /// Returns `true` if the estimator has locked onto the external clock.
    #[cfg(test)]
    pub(crate) fn is_locked(&self) -> bool {
        self.estimator.is_locked()
    }

    /// Returns the current estimated tempo in BPM, if available.
    pub(crate) fn estimated_bpm(&self) -> Option<f64> {
        self.estimator.estimated_bpm()
    }

    /// Checks for external clock dropout.
    ///
    /// If no tick has arrived within the timeout window (or within 4x
    /// the estimated interval), the estimator is reset and the oscillator
    /// is stopped.
    ///
    /// # Arguments
    ///
    /// * `now_ns` - Current monotonic timestamp in nanoseconds.
    ///
    /// # Returns
    ///
    /// `true` if a dropout was detected.
    pub(crate) fn check_dropout(&mut self, now_ns: u64) -> bool {
        if self.estimator.check_dropout(now_ns) {
            self.oscillator.stop();
            return true;
        }

        // Also check against the configured timeout if the estimator
        // has a last tick but isn't locked yet.
        if let Some(last_tick) = self.estimator.last_tick_ns
            && now_ns.saturating_sub(last_tick) > self.timeout_ns
        {
            self.oscillator.stop();
            return true;
        }

        false
    }
}

// ── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Interval between MIDI clock ticks at 120 BPM (24 PPQN).
    /// 60 / (120 * 24) * 1e9 = 20_833_333.33... ns
    const INTERVAL_120_BPM_NS: u64 = 20_833_333;

    /// Interval between MIDI clock ticks at 140 BPM (24 PPQN).
    const INTERVAL_140_BPM_NS: u64 = 17_857_142;

    // ── TempoEstimator tests (Milestone 4.1) ────────────────────────

    #[test]
    fn test_estimator_not_locked_initially() {
        let estimator = TempoEstimator::new(8);
        assert!(!estimator.is_locked());
        assert_eq!(estimator.estimated_interval_ns(), None);
        assert_eq!(estimator.estimated_bpm(), None);
    }

    #[test]
    fn test_estimator_locks_after_4_ticks() {
        let mut estimator = TempoEstimator::new(8);

        // Feed 4 ticks at 120 BPM intervals.
        for i in 0..4 {
            estimator.feed_tick(i * INTERVAL_120_BPM_NS);
        }

        assert!(estimator.is_locked());

        let bpm = estimator.estimated_bpm().expect("should have BPM estimate");
        let error_pct = ((bpm - 120.0) / 120.0).abs() * 100.0;
        assert!(
            error_pct < 1.0,
            "expected BPM within 1% of 120, got {bpm} (error: {error_pct:.2}%)"
        );
    }

    #[test]
    fn test_estimator_smooths_jitter() {
        let mut estimator = TempoEstimator::new(8);

        // Feed 48 ticks at 120 BPM with alternating +/- 500us jitter.
        let jitter_ns: i64 = 500_000; // 500 us
        for i in 0..48 {
            let ideal_time = i as u64 * INTERVAL_120_BPM_NS;
            let jitter = if i % 2 == 0 { jitter_ns } else { -jitter_ns };
            let timestamp = (ideal_time as i64 + jitter) as u64;
            estimator.feed_tick(timestamp);
        }

        assert!(estimator.is_locked());

        let bpm = estimator.estimated_bpm().expect("should have BPM estimate");
        let error_pct = ((bpm - 120.0) / 120.0).abs() * 100.0;
        assert!(
            error_pct < 1.0,
            "expected BPM within 1% of 120, got {bpm} (error: {error_pct:.2}%)"
        );
    }

    #[test]
    fn test_estimator_follows_tempo_change() {
        let mut estimator = TempoEstimator::new(8);

        // Feed 24 ticks at 120 BPM.
        for i in 0..24 {
            estimator.feed_tick(i * INTERVAL_120_BPM_NS);
        }

        // Then 24 ticks at 140 BPM, continuing from where we left off.
        let base_time = 23 * INTERVAL_120_BPM_NS;
        for i in 1..=24 {
            estimator.feed_tick(base_time + i * INTERVAL_140_BPM_NS);
        }

        let bpm = estimator.estimated_bpm().expect("should have BPM estimate");
        let error_pct = ((bpm - 140.0) / 140.0).abs() * 100.0;
        assert!(
            error_pct < 2.0,
            "expected BPM within 2% of 140, got {bpm} (error: {error_pct:.2}%)"
        );
    }

    #[test]
    fn test_estimator_detects_dropout() {
        let mut estimator = TempoEstimator::new(8);

        // Feed 10 ticks at 120 BPM.
        for i in 0..10 {
            estimator.feed_tick(i * INTERVAL_120_BPM_NS);
        }

        assert!(estimator.is_locked());

        // Check dropout with a timestamp 200ms later (about 10x the interval).
        let last_tick = 9 * INTERVAL_120_BPM_NS;
        let dropout_time = last_tick + 200_000_000; // 200 ms
        assert!(estimator.check_dropout(dropout_time));
        assert!(!estimator.is_locked());
    }

    #[test]
    fn test_estimator_handles_tempo_jump() {
        let mut estimator = TempoEstimator::new(8);

        // Feed 12 ticks at 120 BPM.
        for i in 0..12 {
            estimator.feed_tick(i * INTERVAL_120_BPM_NS);
        }

        assert!(estimator.is_locked());
        let bpm_before = estimator.estimated_bpm().expect("should be locked");
        assert!(
            (bpm_before - 120.0).abs() < 2.0,
            "should be near 120 BPM, got {bpm_before}"
        );

        // Immediately switch to 60 BPM (2x interval — this is >3x the
        // current filtered interval, triggering a tempo jump reset).
        // Actually, 60 BPM = 2x interval. The check is >3x, so 2x won't
        // trigger the jump. Let's use 30 BPM (4x interval) which will
        // definitely trigger the reset.
        let base_time = 11 * INTERVAL_120_BPM_NS;
        let interval_30_bpm = INTERVAL_120_BPM_NS * 4; // 30 BPM

        // Feed ticks at the new tempo. The first one triggers the jump
        // detector, then we need enough to re-lock.
        for i in 1..=12 {
            estimator.feed_tick(base_time + i as u64 * interval_30_bpm);
        }

        assert!(
            estimator.is_locked(),
            "should re-lock after tempo jump within ~8 ticks"
        );

        let bpm = estimator.estimated_bpm().expect("should have BPM estimate");
        let error_pct = ((bpm - 30.0) / 30.0).abs() * 100.0;
        assert!(
            error_pct < 5.0,
            "expected BPM within 5% of 30, got {bpm} (error: {error_pct:.2}%)"
        );
    }

    // ── SlaveOscillator tests (Milestone 4.2) ───────────────────────

    #[test]
    fn test_oscillator_not_running_initially() {
        let osc = SlaveOscillator::new();
        assert_eq!(osc.next_tick(INTERVAL_120_BPM_NS), None);
    }

    #[test]
    fn test_oscillator_starts_on_first_sync() {
        let mut osc = SlaveOscillator::new();
        osc.sync_to_external(1000, INTERVAL_120_BPM_NS);
        let schedule = osc.next_tick(INTERVAL_120_BPM_NS);
        assert!(
            schedule.is_some(),
            "oscillator should produce a tick after sync"
        );
        let s = schedule.expect("already checked");
        assert_eq!(s.next_tick_ns, 1000 + INTERVAL_120_BPM_NS);
        assert_eq!(s.subdivision, 0);
        assert_eq!(s.beat, 0);
    }

    #[test]
    fn test_oscillator_subdivision_wraps() {
        let mut osc = SlaveOscillator::new();
        osc.sync_to_external(0, INTERVAL_120_BPM_NS);

        // Advance 24 times (one full beat).
        for _ in 0..24 {
            osc.advance(INTERVAL_120_BPM_NS);
        }

        let schedule = osc
            .next_tick(INTERVAL_120_BPM_NS)
            .expect("should be running");
        assert_eq!(schedule.subdivision, 0);
        assert_eq!(schedule.beat, 1);
    }

    #[test]
    fn test_oscillator_stop_and_resume() {
        let mut osc = SlaveOscillator::new();
        osc.sync_to_external(0, INTERVAL_120_BPM_NS);
        assert!(osc.next_tick(INTERVAL_120_BPM_NS).is_some());

        osc.stop();
        assert_eq!(osc.next_tick(INTERVAL_120_BPM_NS), None);

        osc.resume(100_000_000, INTERVAL_120_BPM_NS);
        let schedule = osc
            .next_tick(INTERVAL_120_BPM_NS)
            .expect("should be running after resume");
        assert_eq!(schedule.next_tick_ns, 100_000_000 + INTERVAL_120_BPM_NS);
    }

    #[test]
    fn test_oscillator_reset_position() {
        let mut osc = SlaveOscillator::new();
        osc.sync_to_external(0, INTERVAL_120_BPM_NS);

        // Advance a few ticks.
        for _ in 0..30 {
            osc.advance(INTERVAL_120_BPM_NS);
        }
        let before = osc.next_tick(INTERVAL_120_BPM_NS).expect("running");
        assert_eq!(before.beat, 1);
        assert_eq!(before.subdivision, 6);

        osc.reset_position();
        let after = osc.next_tick(INTERVAL_120_BPM_NS).expect("running");
        assert_eq!(after.beat, 0);
        assert_eq!(after.subdivision, 0);
    }

    #[test]
    fn test_oscillator_spp() {
        let mut osc = SlaveOscillator::new();
        osc.sync_to_external(0, INTERVAL_120_BPM_NS);

        // SPP 7: 7/4 = 1 beat, 7%4 = 3 -> subdivision = 3*6 = 18.
        osc.set_position_from_spp(7);
        let schedule = osc.next_tick(INTERVAL_120_BPM_NS).expect("running");
        assert_eq!(schedule.beat, 1);
        assert_eq!(schedule.subdivision, 18);
    }

    // ── SlaveClock tests (Milestone 4.3) ────────────────────────────

    #[test]
    fn test_slave_clock_not_locked_initially() {
        let clock = SlaveClock::new(1_000_000_000);
        assert!(!clock.is_locked());
        assert_eq!(clock.estimated_bpm(), None);
        assert_eq!(clock.next_tick(), None);
    }

    #[test]
    fn test_slave_clock_locks_after_ticks() {
        let mut clock = SlaveClock::new(1_000_000_000);

        // Feed 8 ticks at 120 BPM.
        for i in 0..8 {
            clock.feed_clock_byte(i * INTERVAL_120_BPM_NS);
        }

        assert!(clock.is_locked());

        let bpm = clock.estimated_bpm().expect("should have BPM estimate");
        let error_pct = ((bpm - 120.0) / 120.0).abs() * 100.0;
        assert!(
            error_pct < 1.0,
            "expected BPM within 1% of 120, got {bpm} (error: {error_pct:.2}%)"
        );

        // Should produce a next tick when locked.
        assert!(clock.next_tick().is_some());
    }

    #[test]
    fn test_slave_clock_timeout() {
        let timeout_ns = 500_000_000; // 500 ms
        let mut clock = SlaveClock::new(timeout_ns);

        // Feed ticks to lock.
        for i in 0..8 {
            clock.feed_clock_byte(i * INTERVAL_120_BPM_NS);
        }
        assert!(clock.is_locked());

        // Check dropout after the timeout window.
        let last_tick = 7 * INTERVAL_120_BPM_NS;
        let dropout_time = last_tick + timeout_ns + 1;
        assert!(clock.check_dropout(dropout_time));
        assert!(!clock.is_locked());
    }

    #[test]
    fn test_slave_clock_transport_start_resets() {
        let mut clock = SlaveClock::new(1_000_000_000);

        // Feed ticks to lock.
        for i in 0..8 {
            clock.feed_clock_byte(i * INTERVAL_120_BPM_NS);
        }

        // Advance the oscillator a few ticks.
        for _ in 0..10 {
            clock.advance();
        }

        // Start should reset position.
        let now = 8 * INTERVAL_120_BPM_NS;
        clock.feed_transport(TransportEvent::Start, now);

        let schedule = clock.next_tick().expect("should be running");
        assert_eq!(schedule.beat, 0);
        assert_eq!(schedule.subdivision, 0);
    }

    #[test]
    fn test_slave_clock_transport_stop_halts() {
        let mut clock = SlaveClock::new(1_000_000_000);

        // Feed ticks to lock.
        for i in 0..8 {
            clock.feed_clock_byte(i * INTERVAL_120_BPM_NS);
        }
        assert!(clock.next_tick().is_some());

        // Stop should halt tick production.
        let now = 8 * INTERVAL_120_BPM_NS;
        clock.feed_transport(TransportEvent::Stop, now);
        assert_eq!(clock.next_tick(), None);
    }

    #[test]
    fn test_slave_clock_transport_continue_resumes() {
        let mut clock = SlaveClock::new(1_000_000_000);

        // Feed ticks to lock.
        for i in 0..8 {
            clock.feed_clock_byte(i * INTERVAL_120_BPM_NS);
        }

        let now = 8 * INTERVAL_120_BPM_NS;

        // Stop, then Continue.
        clock.feed_transport(TransportEvent::Stop, now);
        assert_eq!(clock.next_tick(), None);

        clock.feed_transport(TransportEvent::Continue, now + INTERVAL_120_BPM_NS);
        assert!(
            clock.next_tick().is_some(),
            "should produce ticks after Continue"
        );
    }

    // ── Synthetic jitter test ───────────────────────────────────────

    #[test]
    fn test_slave_output_jitter_within_budget() {
        // Simple deterministic LCG for pseudo-random jitter.
        // Parameters from Numerical Recipes.
        let mut rng_state: u64 = 42;
        let lcg_next = |state: &mut u64| -> i64 {
            *state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            // Map to range [-1_500_000, +1_500_000] (1.5 ms).
            let raw = (*state >> 33) as i64; // 0..2^31
            (raw % 3_000_001) - 1_500_000
        };

        // Collect internal tick timestamps by feeding external ticks
        // and reading the oscillator schedule after each one.
        let mut slave = SlaveClock::new(1_000_000_000);
        let mut internal_ticks: Vec<u64> = Vec::new();
        let mut last_external: u64 = 0;

        for i in 0..240u64 {
            let ideal = i * INTERVAL_120_BPM_NS;
            let jitter = lcg_next(&mut rng_state);
            let timestamp = (ideal as i64 + jitter).max(0) as u64;
            // Ensure monotonicity of external timestamps.
            let timestamp = timestamp.max(last_external + 1);
            last_external = timestamp;

            slave.feed_clock_byte(timestamp);

            // If locked, collect the next internal tick.
            if let Some(schedule) = slave.next_tick() {
                internal_ticks.push(schedule.next_tick_ns);
                slave.advance();
            }
        }

        // We need enough internal ticks to measure jitter.
        assert!(
            internal_ticks.len() >= 100,
            "expected at least 100 internal ticks, got {}",
            internal_ticks.len()
        );

        // Compute jitter: deviation of internal tick intervals from
        // the ideal 120 BPM interval.
        let mut jitters_ns: Vec<u64> = Vec::new();
        for window in internal_ticks.windows(2) {
            let actual_interval = window[1].saturating_sub(window[0]);
            let jitter = if actual_interval >= INTERVAL_120_BPM_NS {
                actual_interval - INTERVAL_120_BPM_NS
            } else {
                INTERVAL_120_BPM_NS - actual_interval
            };
            jitters_ns.push(jitter);
        }

        jitters_ns.sort_unstable();

        let median_idx = jitters_ns.len() / 2;
        let p99_idx = (jitters_ns.len() as f64 * 0.99) as usize;
        let p99_idx = p99_idx.min(jitters_ns.len() - 1);

        let median_us = jitters_ns[median_idx] / 1_000;
        let p99_us = jitters_ns[p99_idx] / 1_000;

        eprintln!(
            "Jitter test: {} ticks, median = {}us, P99 = {}us",
            jitters_ns.len(),
            median_us,
            p99_us,
        );

        assert!(
            median_us < 500,
            "median jitter {median_us}us exceeds 500us threshold"
        );
        assert!(
            p99_us < 2000,
            "P99 jitter {p99_us}us exceeds 2000us threshold"
        );
    }

    // ── Additional TempoEstimator edge cases ───────────────────────

    #[test]
    fn test_estimator_not_locked_with_3_ticks() {
        let mut estimator = TempoEstimator::new(8);

        // Feed exactly 3 ticks: should NOT be locked (needs 4).
        for i in 0..3 {
            estimator.feed_tick(i * INTERVAL_120_BPM_NS);
        }

        assert!(!estimator.is_locked(), "3 ticks should not lock the estimator");
        assert_eq!(estimator.estimated_bpm(), None);
        assert_eq!(estimator.estimated_interval_ns(), None);
    }

    #[test]
    fn test_estimator_zero_interval_returns_none_bpm() {
        let mut estimator = TempoEstimator::new(8);

        // Feed 4 ticks all at the same timestamp (degenerate case).
        // This produces intervals of 0, which after filtering gives 0.
        for _ in 0..4 {
            estimator.feed_tick(1_000_000);
        }

        // The estimator considers itself locked (count >= 4), but
        // estimated_bpm should return None since the interval is 0.
        if estimator.is_locked() {
            let bpm = estimator.estimated_bpm();
            assert_eq!(bpm, None, "zero interval should yield None BPM");
        }
    }

    #[test]
    fn test_estimator_raw_filtered_interval() {
        let mut estimator = TempoEstimator::new(8);

        // Before any ticks: no raw interval.
        assert_eq!(estimator.raw_filtered_interval_ns(), None);

        // After 2 ticks: raw interval should be available even though
        // the estimator is not yet locked.
        estimator.feed_tick(0);
        estimator.feed_tick(INTERVAL_120_BPM_NS);
        assert!(!estimator.is_locked());
        let raw = estimator.raw_filtered_interval_ns();
        assert!(
            raw.is_some(),
            "should have raw interval after 2 ticks"
        );
        let raw = raw.unwrap();
        let error = (raw - INTERVAL_120_BPM_NS as f64).abs();
        assert!(
            error < 1.0,
            "raw interval should be close to the tick interval"
        );
    }

    #[test]
    fn test_estimator_no_dropout_before_ticks() {
        let mut estimator = TempoEstimator::new(8);
        // No ticks fed: check_dropout should return false.
        assert!(!estimator.check_dropout(1_000_000_000));
    }

    #[test]
    fn test_estimator_no_dropout_within_window() {
        let mut estimator = TempoEstimator::new(8);

        for i in 0..8 {
            estimator.feed_tick(i * INTERVAL_120_BPM_NS);
        }
        assert!(estimator.is_locked());

        // Check at a time within the 4x window: should NOT be a dropout.
        let last_tick = 7 * INTERVAL_120_BPM_NS;
        let within_window = last_tick + 2 * INTERVAL_120_BPM_NS;
        assert!(!estimator.check_dropout(within_window));
    }

    // ── Additional SlaveClock edge cases ────────────────────────────

    #[test]
    fn test_slave_clock_advance_when_not_locked() {
        let mut clock = SlaveClock::new(1_000_000_000);
        // Advancing before locking should be a no-op (no panic).
        clock.advance();
        assert!(!clock.is_locked());
    }

    #[test]
    fn test_slave_clock_feed_spp() {
        let mut clock = SlaveClock::new(1_000_000_000);

        // Feed ticks to lock.
        for i in 0..8 {
            clock.feed_clock_byte(i * INTERVAL_120_BPM_NS);
        }
        assert!(clock.is_locked());

        // Set SPP and verify via next_tick schedule.
        clock.feed_spp(8); // 8 MIDI beats = 2 quarter-note beats, sub 0
        let schedule = clock.next_tick().expect("should be locked and running");
        assert_eq!(schedule.beat, 2);
        assert_eq!(schedule.subdivision, 0);
    }

    #[test]
    fn test_slave_clock_continue_before_locked() {
        let mut clock = SlaveClock::new(1_000_000_000);

        // Feed only 2 ticks (not locked).
        clock.feed_clock_byte(0);
        clock.feed_clock_byte(INTERVAL_120_BPM_NS);
        assert!(!clock.is_locked());

        // Stop and Continue before the estimator is locked.
        // Continue should use the raw filtered interval to resume.
        let now = 2 * INTERVAL_120_BPM_NS;
        clock.feed_transport(TransportEvent::Stop, now);
        clock.feed_transport(TransportEvent::Continue, now + INTERVAL_120_BPM_NS);

        // Even though not locked, the oscillator should have been
        // resumed via the raw interval path.
        // (next_tick still returns None because is_locked() is false,
        // but the oscillator state has been set.)
        assert!(!clock.is_locked());
    }

    #[test]
    fn test_slave_clock_dropout_resets_oscillator() {
        let mut clock = SlaveClock::new(500_000_000);

        // Feed ticks to lock.
        for i in 0..8 {
            clock.feed_clock_byte(i * INTERVAL_120_BPM_NS);
        }
        assert!(clock.is_locked());
        assert!(clock.next_tick().is_some());

        // Simulate dropout.
        let last_tick = 7 * INTERVAL_120_BPM_NS;
        let dropout_time = last_tick + 500_000_001; // Just past the timeout.
        assert!(clock.check_dropout(dropout_time));

        // After dropout, the oscillator should be stopped.
        assert!(!clock.is_locked());
        assert_eq!(clock.next_tick(), None);
    }

    // ── SlaveOscillator phase correction ───────────────────────────

    #[test]
    fn test_oscillator_phase_correction() {
        let mut osc = SlaveOscillator::new();
        // First sync starts the oscillator.
        osc.sync_to_external(1_000_000, INTERVAL_120_BPM_NS);

        let schedule_before = osc.next_tick(INTERVAL_120_BPM_NS).unwrap();
        let expected_first = 1_000_000 + INTERVAL_120_BPM_NS;
        assert_eq!(schedule_before.next_tick_ns, expected_first);

        // Second sync: external tick arrives a bit late (positive phase error).
        let late_tick = expected_first + 500_000; // 500us late
        osc.sync_to_external(late_tick, INTERVAL_120_BPM_NS);

        let schedule_after = osc.next_tick(INTERVAL_120_BPM_NS).unwrap();
        // With phase_gain = 0.2, the correction should push the next
        // tick slightly later than expected_first.
        assert!(
            schedule_after.next_tick_ns > expected_first,
            "phase correction should shift next tick later for a late external tick"
        );
    }

    #[test]
    fn test_oscillator_resume_does_nothing_if_already_running() {
        let mut osc = SlaveOscillator::new();
        osc.sync_to_external(0, INTERVAL_120_BPM_NS);

        let before = osc.next_tick(INTERVAL_120_BPM_NS).unwrap().next_tick_ns;

        // Resume when already running should be a no-op.
        osc.resume(1_000_000_000, INTERVAL_120_BPM_NS);

        let after = osc.next_tick(INTERVAL_120_BPM_NS).unwrap().next_tick_ns;
        assert_eq!(before, after, "resume should not change next_tick_ns if already running");
    }

    // ── Estimator window_size minimum clamp ────────────────────────

    #[test]
    fn test_estimator_minimum_window_size() {
        // Window size 1 should be clamped to 2.
        let mut estimator = TempoEstimator::new(1);

        for i in 0..4 {
            estimator.feed_tick(i * INTERVAL_120_BPM_NS);
        }

        assert!(
            estimator.is_locked(),
            "estimator with window_size=1 (clamped to 2) should still lock"
        );
    }
}
