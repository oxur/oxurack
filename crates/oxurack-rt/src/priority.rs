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
pub(crate) fn elevate_rt_priority() -> Result<(), crate::Error> {
    let _handle = audio_thread_priority::promote_current_thread_to_real_time(
        RT_BUFFER_FRAMES,
        RT_SAMPLE_RATE_HZ,
    )
    .map_err(|e| crate::Error::PriorityElevation(e.to_string()))?;

    Ok(())
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
}
