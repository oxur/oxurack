//! High-resolution monotonic timing and precision sleep utilities.
//!
//! Built on [`quanta`] for low-overhead timestamps and [`spin_sleep`] for
//! hybrid busy-wait/OS-sleep that achieves sub-millisecond accuracy.
//!
//! # Design
//!
//! [`MonotonicClock`] captures a raw tick epoch at construction time and
//! uses [`quanta::Clock::delta_as_nanos`] to convert elapsed ticks into
//! nanoseconds.  This avoids the overhead of constructing
//! [`std::time::Duration`] values on every call while still returning
//! calibrated, monotonically increasing timestamps.
//!
//! [`precision_sleep`] delegates to [`spin_sleep::sleep`], which sleeps
//! via the OS for most of the requested duration and then spin-waits for
//! the final sub-millisecond portion, guaranteeing no undershoot.

use std::time::Duration;

/// A high-resolution monotonic clock backed by [`quanta`].
///
/// Provides nanosecond timestamps suitable for scheduling MIDI clock
/// ticks and measuring inter-tick intervals.
///
/// Timestamps are relative to the moment the clock was created: the
/// very first call to [`Self::now`] returns a small positive value
/// representing the nanoseconds elapsed since construction.
#[derive(Debug)]
pub(crate) struct MonotonicClock {
    /// The underlying quanta clock used for reading raw ticks.
    clock: quanta::Clock,
    /// Raw tick value captured at construction time (the epoch).
    epoch: u64,
}

impl MonotonicClock {
    /// Creates a new monotonic clock, calibrating the underlying timer.
    ///
    /// The current raw tick count is recorded as the epoch; all
    /// subsequent [`Self::now`] values are relative to this point.
    pub(crate) fn new() -> Self {
        let clock = quanta::Clock::new();
        let epoch = clock.raw();
        Self { clock, epoch }
    }

    /// Returns the current monotonic time in nanoseconds since this
    /// clock was created.
    ///
    /// The value is derived from [`quanta::Clock::delta_as_nanos`],
    /// which converts raw hardware ticks into calibrated nanoseconds
    /// with minimal overhead.
    pub(crate) fn now(&self) -> u64 {
        self.clock.delta_as_nanos(self.epoch, self.clock.raw())
    }

    /// Returns the elapsed time in nanoseconds since the given start
    /// timestamp.
    ///
    /// Uses saturating subtraction so the result is always non-negative,
    /// even if `start` was obtained from a different clock instance (in
    /// which case the value is meaningless but at least won't wrap).
    ///
    /// # Arguments
    ///
    /// * `start` - A timestamp previously obtained from [`Self::now`].
    #[cfg(test)]
    pub(crate) fn elapsed_since(&self, start: u64) -> u64 {
        self.now().saturating_sub(start)
    }
}

/// Sleeps until the target timestamp with high precision.
///
/// Uses [`spin_sleep::sleep`] to combine an OS sleep (for most of the
/// duration) with a busy-wait spin (for the final microseconds),
/// achieving sub-millisecond wake-up accuracy.
///
/// If `target_ns` is already in the past (or equal to the current
/// time), this function returns immediately without sleeping.
///
/// # Arguments
///
/// * `target_ns` - The monotonic timestamp (in nanoseconds) to sleep
///   until.  Must be on the same timescale as values returned by
///   [`MonotonicClock::now`].
/// * `clock` - The monotonic clock used to read the current time.
pub(crate) fn precision_sleep(target_ns: u64, clock: &MonotonicClock) {
    let now = clock.now();
    if target_ns <= now {
        return;
    }

    let remaining_ns = target_ns - now;
    spin_sleep::sleep(Duration::from_nanos(remaining_ns));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotonic_clock_advances() {
        let clock = MonotonicClock::new();
        let t1 = clock.now();
        // Spin briefly to ensure the clock advances.
        std::hint::spin_loop();
        let t2 = clock.now();
        assert!(t2 > t1, "expected clock to advance: t1={t1}, t2={t2}");
    }

    #[test]
    fn test_elapsed_since() {
        let clock = MonotonicClock::new();
        let start = clock.now();
        // Burn some time so elapsed is measurably positive.
        std::thread::sleep(Duration::from_millis(1));
        let elapsed = clock.elapsed_since(start);
        assert!(elapsed > 0, "expected positive elapsed time, got {elapsed}");
    }

    #[test]
    fn test_precision_sleep_accuracy() {
        let clock = MonotonicClock::new();
        let before = clock.now();
        let target = before + 10_000_000; // 10 ms from now
        precision_sleep(target, &clock);
        let after = clock.now();
        let elapsed_ms = (after - before) / 1_000_000;
        assert!(
            (8..=30).contains(&elapsed_ms),
            "expected ~10 ms sleep, got {elapsed_ms} ms"
        );
    }

    #[test]
    fn test_precision_sleep_no_undershoot() {
        let clock = MonotonicClock::new();
        let before = clock.now();
        let target = before + 5_000_000; // 5 ms from now
        precision_sleep(target, &clock);
        let after = clock.now();
        let elapsed_ns = after - before;
        assert!(
            elapsed_ns >= 5_000_000,
            "expected >= 5 ms, got {elapsed_ns} ns ({} ms)",
            elapsed_ns / 1_000_000
        );
    }

    #[test]
    fn test_precision_sleep_past_target_returns_immediately() {
        let clock = MonotonicClock::new();
        // Ensure we have a timestamp in the past.
        std::thread::sleep(Duration::from_millis(1));
        let past_target = 0; // Well before now.
        let before = clock.now();
        precision_sleep(past_target, &clock);
        let after = clock.now();
        let elapsed_ns = after - before;
        assert!(
            elapsed_ns < 1_000_000,
            "expected < 1 ms for past target, got {elapsed_ns} ns"
        );
    }
}
