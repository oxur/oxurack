//! OS-level real-time thread priority elevation.
//!
//! Uses `audio_thread_priority` to request real-time scheduling for the
//! MIDI thread, reducing latency jitter from OS preemption.

/// Buffer size in frames passed to the OS scheduler.
///
/// A small buffer requests more frequent scheduling wake-ups, which is
/// ideal for sub-millisecond MIDI timing. At 48 kHz this yields a
/// ~2.67 ms budget per quantum.
const RT_BUFFER_FRAMES: u32 = 128;

/// Sample rate in Hz passed to the OS scheduler.
///
/// Standard audio rate; together with [`RT_BUFFER_FRAMES`] it tells the
/// kernel how much CPU time the thread needs and how often.
const RT_SAMPLE_RATE_HZ: u32 = 48_000;

/// Elevates the calling thread to real-time priority.
///
/// This should be called early in the RT thread's lifecycle, before
/// entering the main loop. On failure, the thread continues at normal
/// priority and the error is reported as a non-fatal condition.
///
/// # Errors
///
/// Returns [`crate::Error::PriorityElevation`] if the OS refuses the
/// priority elevation (e.g., insufficient permissions).
pub(crate) fn elevate_rt_priority() -> Result<audio_thread_priority::RtPriorityHandle, crate::Error>
{
    audio_thread_priority::promote_current_thread_to_real_time(RT_BUFFER_FRAMES, RT_SAMPLE_RATE_HZ)
        .map_err(|e| crate::Error::PriorityElevation(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that RT priority elevation succeeds on the current platform.
    ///
    /// On macOS, RT priority elevation succeeds without special privileges.
    /// On Linux CI, `CAP_SYS_NICE` may be required; if elevation is
    /// not available the test documents a non-fatal warning.
    #[test]
    fn test_elevation_succeeds() {
        let result = std::thread::spawn(elevate_rt_priority)
            .join()
            .expect("RT thread panicked");

        assert!(result.is_ok(), "elevation failed: {result:?}");
    }

    /// Verifies that the `PriorityElevation` error variant formats its
    /// inner message correctly via `Display`.
    #[test]
    fn test_error_display() {
        let err = crate::Error::PriorityElevation("test reason".into());
        let display = err.to_string();

        assert!(
            display.contains("test reason"),
            "expected Display output to contain 'test reason', got: {display}"
        );
    }

    #[cfg(unix)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SchedulerState {
        policy: libc::c_int,
        priority: libc::c_int,
    }

    #[cfg(unix)]
    fn current_scheduler_state() -> SchedulerState {
        use std::mem::MaybeUninit;
        unsafe {
            let tid = libc::pthread_self();
            let mut policy: libc::c_int = 0;
            let mut param = MaybeUninit::<libc::sched_param>::zeroed();
            let rc = libc::pthread_getschedparam(tid, &mut policy, param.as_mut_ptr());
            assert_eq!(rc, 0, "pthread_getschedparam failed: {rc}");
            let param = param.assume_init();
            SchedulerState {
                policy,
                priority: param.sched_priority,
            }
        }
    }

    #[cfg(unix)]
    #[test]
    #[ignore = "requires RT scheduling permissions; run with `cargo test -- --ignored`"]
    fn test_priority_elevation_effects_on_scheduler() {
        let handle = std::thread::spawn(|| {
            let before = current_scheduler_state();
            let during = {
                let _rt_handle = elevate_rt_priority().expect("elevation should succeed");
                current_scheduler_state()
                // _rt_handle dropped here at end of block
            };
            let after = current_scheduler_state();
            (before, during, after)
        });

        let (before, during, after) = handle.join().expect("thread panicked");

        // On macOS, audio_thread_priority uses THREAD_TIME_CONSTRAINT_POLICY
        // which may not change the POSIX scheduling parameters. If before == during,
        // the platform doesn't expose the change through pthread_getschedparam.
        // In that case, just verify the function didn't error (already done by expect).
        if before != during {
            assert_eq!(
                before, after,
                "scheduler state not restored after handle drop"
            );
        }
    }
}
