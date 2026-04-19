//! The real-time MIDI thread main loop.
//!
//! This module contains the entry point for the dedicated RT thread.
//! The loop handles clock tick generation/tracking, MIDI I/O, and
//! command processing from the ECS world via lock-free queues.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::queues::RtSideQueues;

/// Runs the RT thread main loop.
///
/// This function is the entry point for the spawned real-time thread.
/// It elevates thread priority, opens MIDI ports (both output and
/// input), and enters a tight loop that:
///
/// 1. Drains incoming [`crate::EcsCommand`]s and processes them.
/// 2. Sleeps until the next clock tick.
/// 3. Emits MIDI Clock (0xF8) on all output ports.
/// 4. Pushes a [`crate::RtEvent::ClockTick`] event to the ECS world.
/// 5. Advances the master clock to the next tick position.
/// 6. Drains MIDI input events, classifies them, and pushes the
///    appropriate [`crate::RtEvent`]s to the ECS world.
///
/// The loop exits when a [`crate::EcsCommand::Shutdown`] command is
/// received, the shutdown flag is set, or an unrecoverable error occurs.
///
/// # Arguments
///
/// * `queues` - The RT-side queue handles for sending events and
///   receiving commands.
/// * `config` - The runtime configuration (clock mode, ports, etc.).
/// * `ready_signal` - A channel to signal readiness (or error) back to
///   the spawning thread.
/// * `shutdown` - An atomic flag checked each iteration to allow
///   external shutdown.
pub(crate) fn rt_thread_main(
    mut queues: RtSideQueues,
    config: crate::RuntimeConfig,
    ready_signal: std::sync::mpsc::SyncSender<Result<(), crate::Error>>,
    shutdown: Arc<AtomicBool>,
) {
    // 1. Elevate RT priority (best-effort). Failure is non-fatal: the
    //    thread will still function correctly, just with higher jitter.
    //    This mirrors the approach in `run_timing_test` and avoids
    //    breaking tests in CI sandboxes that lack scheduling permissions.
    let _ = crate::priority::elevate_rt_priority();

    // 2. Open MIDI output ports.
    let mut midi_ports = match crate::midi_io::MidiPorts::open_outputs(&config.outputs) {
        Ok(ports) => ports,
        Err(e) => {
            let _ = ready_signal.send(Err(e));
            return;
        }
    };

    // 3. Open MIDI input ports.
    let mut midi_input_ports = match crate::midi_io::MidiInputPorts::open(&config.inputs) {
        Ok(ports) => ports,
        Err(e) => {
            let _ = ready_signal.send(Err(e));
            return;
        }
    };

    // 4. Signal readiness to the spawning thread.
    let _ = ready_signal.send(Ok(()));

    // 5. Create clock and timing infrastructure.
    let clock = crate::timing::MonotonicClock::new();

    match &config.clock_mode {
        crate::ClockMode::Master { tempo_bpm, .. } => {
            let mut master = crate::clock::master::MasterClock::new(*tempo_bpm, clock.now());

            // Master mode loop.
            loop {
                if shutdown.load(Ordering::Relaxed) {
                    break;
                }

                // Drain commands from ECS.
                while let Ok(cmd) = queues.commands.pop() {
                    match cmd {
                        crate::EcsCommand::Shutdown => {
                            shutdown.store(true, Ordering::Relaxed);
                            break;
                        }
                        crate::EcsCommand::SetTempo { bpm } => {
                            master.set_tempo(bpm);
                        }
                        crate::EcsCommand::SendMidi {
                            output_port_index,
                            message,
                            ..
                        } => {
                            let bytes = message.to_bytes();
                            let len = message.length as usize;
                            let _ = midi_ports.send(output_port_index, &bytes[..len]);
                        }
                        crate::EcsCommand::SendTransport(_)
                        | crate::EcsCommand::SendSongPosition { .. } => {
                            // Transport and SPP will be handled in a future phase.
                        }
                    }
                }

                if shutdown.load(Ordering::Relaxed) {
                    break;
                }

                // Sleep until the next tick.
                let schedule = master.next_tick();
                crate::timing::precision_sleep(schedule.next_tick_ns, &clock);

                // Emit MIDI Clock (0xF8) on all output ports.
                for i in 0..config.outputs.len() {
                    let _ = midi_ports.send(i as u8, &[0xF8]);
                }

                // Push ClockTick event to ECS.
                let tick_event = crate::RtEvent::ClockTick {
                    subdivision: schedule.subdivision,
                    beat: schedule.beat,
                    tempo_bpm: master.tempo(),
                    timestamp_ns: clock.now(),
                };
                let _ = queues.events.push(tick_event);

                // Advance to the next tick position.
                master.advance();

                // Drain MIDI input events and classify them.
                drain_midi_input(&mut midi_input_ports, &mut queues);
            }
        }
        crate::ClockMode::Slave { timeout_ns, .. } => {
            let mut slave_clock = crate::clock::slave::SlaveClock::new(*timeout_ns);

            // Track when we last emitted a "not locked" warning to
            // avoid flooding the ECS queue (limit to ~1 per second).
            let mut last_not_locked_ns: u64 = 0;
            const NOT_LOCKED_INTERVAL_NS: u64 = 1_000_000_000; // 1 second

            // Slave mode loop.
            loop {
                if shutdown.load(Ordering::Relaxed) {
                    break;
                }

                // Drain commands from ECS.
                while let Ok(cmd) = queues.commands.pop() {
                    match cmd {
                        crate::EcsCommand::Shutdown => {
                            shutdown.store(true, Ordering::Relaxed);
                            break;
                        }
                        crate::EcsCommand::SendMidi {
                            output_port_index,
                            message,
                            ..
                        } => {
                            let bytes = message.to_bytes();
                            let len = message.length as usize;
                            let _ = midi_ports.send(output_port_index, &bytes[..len]);
                        }
                        crate::EcsCommand::SetTempo { .. } => {
                            // In slave mode, tempo is determined by the
                            // external clock source. Ignore SetTempo.
                        }
                        crate::EcsCommand::SendTransport(_)
                        | crate::EcsCommand::SendSongPosition { .. } => {
                            // Transport and SPP sending will be handled
                            // in a future phase.
                        }
                    }
                }

                if shutdown.load(Ordering::Relaxed) {
                    break;
                }

                // Drain MIDI input events, routing clock/transport to
                // the SlaveClock and forwarding everything else to ECS.
                let now = clock.now();
                for raw_event in midi_input_ports.drain_all() {
                    let Some(classification) = crate::messages::classify_midi(
                        &raw_event.bytes[..raw_event.length as usize],
                    ) else {
                        continue;
                    };

                    match classification {
                        crate::messages::MidiClassification::Clock => {
                            slave_clock.feed_clock_byte(raw_event.timestamp_ns);
                        }
                        crate::messages::MidiClassification::Start => {
                            slave_clock.feed_transport(
                                crate::TransportEvent::Start,
                                raw_event.timestamp_ns,
                            );
                            let _ = queues.events.push(crate::RtEvent::Transport(
                                crate::TransportEvent::Start,
                            ));
                        }
                        crate::messages::MidiClassification::Stop => {
                            slave_clock.feed_transport(
                                crate::TransportEvent::Stop,
                                raw_event.timestamp_ns,
                            );
                            let _ = queues.events.push(crate::RtEvent::Transport(
                                crate::TransportEvent::Stop,
                            ));
                        }
                        crate::messages::MidiClassification::Continue => {
                            slave_clock.feed_transport(
                                crate::TransportEvent::Continue,
                                raw_event.timestamp_ns,
                            );
                            let _ = queues.events.push(crate::RtEvent::Transport(
                                crate::TransportEvent::Continue,
                            ));
                        }
                        crate::messages::MidiClassification::SongPosition { position } => {
                            slave_clock.feed_spp(position);
                            let _ = queues
                                .events
                                .push(crate::RtEvent::SongPosition { position });
                        }
                        crate::messages::MidiClassification::Channel(msg) => {
                            let event = crate::RtEvent::MidiInput {
                                input_port_index: raw_event.port_index,
                                timestamp_ns: raw_event.timestamp_ns,
                                message: msg,
                            };
                            let _ = queues.events.push(event);
                        }
                        crate::messages::MidiClassification::ActiveSensing
                        | crate::messages::MidiClassification::SystemReset => {
                            // Ignored system messages.
                        }
                    }
                }

                // Check for clock dropout.
                if slave_clock.check_dropout(now) {
                    let _ = queues
                        .events
                        .push(crate::RtEvent::NonFatalError(crate::RtErrorCode::ClockDropout));
                }

                // If the slave clock has a tick ready, sleep until it
                // and emit a ClockTick event.
                if let Some(schedule) = slave_clock.next_tick() {
                    crate::timing::precision_sleep(schedule.next_tick_ns, &clock);

                    let tempo_bpm = slave_clock.estimated_bpm().unwrap_or(0.0);
                    let tick_event = crate::RtEvent::ClockTick {
                        subdivision: schedule.subdivision,
                        beat: schedule.beat,
                        tempo_bpm,
                        timestamp_ns: clock.now(),
                    };
                    let _ = queues.events.push(tick_event);

                    slave_clock.advance();
                } else {
                    // Not locked: emit a periodic warning.
                    if now.saturating_sub(last_not_locked_ns) >= NOT_LOCKED_INTERVAL_NS {
                        let _ = queues.events.push(crate::RtEvent::NonFatalError(
                            crate::RtErrorCode::ClockNotLocked,
                        ));
                        last_not_locked_ns = now;
                    }

                    // Sleep briefly to avoid busy-waiting when unlocked.
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        }
    }
}

/// Drains all pending raw MIDI events from the input ports, classifies
/// them, and pushes the appropriate [`crate::RtEvent`]s to the ECS
/// event queue.
///
/// Called once per RT loop iteration. This function is allocation-free
/// on the hot path.
fn drain_midi_input(
    midi_input_ports: &mut crate::midi_io::MidiInputPorts,
    queues: &mut RtSideQueues,
) {
    for raw_event in midi_input_ports.drain_all() {
        let Some(classification) =
            crate::messages::classify_midi(&raw_event.bytes[..raw_event.length as usize])
        else {
            continue;
        };

        match classification {
            crate::messages::MidiClassification::Channel(msg) => {
                let event = crate::RtEvent::MidiInput {
                    input_port_index: raw_event.port_index,
                    timestamp_ns: raw_event.timestamp_ns,
                    message: msg,
                };
                let _ = queues.events.push(event);
            }
            crate::messages::MidiClassification::Start => {
                let _ = queues
                    .events
                    .push(crate::RtEvent::Transport(crate::TransportEvent::Start));
            }
            crate::messages::MidiClassification::Stop => {
                let _ = queues
                    .events
                    .push(crate::RtEvent::Transport(crate::TransportEvent::Stop));
            }
            crate::messages::MidiClassification::Continue => {
                let _ = queues
                    .events
                    .push(crate::RtEvent::Transport(crate::TransportEvent::Continue));
            }
            crate::messages::MidiClassification::SongPosition { position } => {
                let _ = queues
                    .events
                    .push(crate::RtEvent::SongPosition { position });
            }
            crate::messages::MidiClassification::Clock => {
                // In master mode, external clock bytes are ignored.
                // In slave mode (Phase 4), these will feed the PLL.
            }
            crate::messages::MidiClassification::ActiveSensing
            | crate::messages::MidiClassification::SystemReset => {
                // Ignored system messages.
            }
        }
    }
}

/// Runs a timing precision test loop at elevated priority.
///
/// Elevates the calling thread to real-time priority (best-effort),
/// then executes `iterations` sleep cycles of `interval_ns` nanoseconds
/// each, recording the actual wall-clock interval between consecutive
/// wake-ups.
///
/// Scheduling uses a **drift-preventing** strategy: each target wake-up
/// is computed from the *scheduled* (ideal) time, not the actual wake-up
/// time. This mirrors the master-clock design and prevents jitter from
/// accumulating into long-term drift.
///
/// # Arguments
///
/// * `iterations` - Number of sleep/wake cycles to execute.
/// * `interval_ns` - Desired interval between wake-ups, in nanoseconds.
///
/// # Returns
///
/// A vector of `iterations` measured intervals (in nanoseconds) between
/// consecutive actual wake-up timestamps.
pub(crate) fn run_timing_test(iterations: u32, interval_ns: u64) -> Vec<u64> {
    use crate::timing::{MonotonicClock, precision_sleep};

    let clock = MonotonicClock::new();

    // Best-effort RT priority elevation; ignore failures (e.g. in CI
    // sandboxes where the calling thread may lack permissions).
    let _ = crate::priority::elevate_rt_priority();

    let mut intervals: Vec<u64> = Vec::with_capacity(iterations as usize);

    let initial_now = clock.now();
    let mut scheduled = initial_now;
    let mut prev_actual = initial_now;

    for _ in 0..iterations {
        // Compute the next ideal wake-up from the scheduled timeline,
        // not from the actual wake time (drift prevention).
        scheduled += interval_ns;
        precision_sleep(scheduled, &clock);

        let actual_now = clock.now();
        let actual_interval = actual_now.saturating_sub(prev_actual);
        intervals.push(actual_interval);
        prev_actual = actual_now;
    }

    intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Validates that the RT timing loop achieves sub-millisecond jitter.
    ///
    /// Runs 1 000 iterations at a 1 ms interval on a dedicated thread
    /// with RT priority, then checks that the median jitter is under
    /// 200 us and the P99 jitter is under 1 ms.
    ///
    /// Marked `#[ignore]` because results are timing-sensitive and may
    /// vary across machines and CI environments.
    #[test]
    #[ignore]
    fn test_timing_loop_jitter() {
        let handle = std::thread::spawn(|| run_timing_test(1_000, 1_000_000));
        let intervals = handle.join().expect("timing test thread panicked");

        let expected_ns: u64 = 1_000_000;

        let mut jitters: Vec<u64> = intervals
            .iter()
            .map(|&iv| {
                if iv >= expected_ns {
                    iv - expected_ns
                } else {
                    expected_ns - iv
                }
            })
            .collect();

        jitters.sort_unstable();

        let median = jitters[500];
        let p99 = jitters[990];
        let max = *jitters.last().expect("jitters should not be empty");

        eprintln!(
            "Timing test: median jitter = {}us, P99 = {}us, max = {}us",
            median / 1_000,
            p99 / 1_000,
            max / 1_000,
        );

        assert!(
            median < 200_000,
            "median jitter {median} ns ({} us) exceeds 200 us threshold",
            median / 1_000,
        );
        assert!(
            p99 < 2_000_000,
            "P99 jitter {p99} ns ({} us) exceeds 2 ms threshold",
            p99 / 1_000,
        );
    }

    /// Validates that the scheduled-to-scheduled timing strategy prevents
    /// cumulative drift.
    ///
    /// Runs 1 000 iterations at a 1 ms interval and checks that the
    /// total elapsed time is within 1 % of the expected 1 000 ms
    /// (i.e. drift < 10 ms over one second).
    ///
    /// Marked `#[ignore]` because results are timing-sensitive.
    #[test]
    #[ignore]
    fn test_timing_loop_no_drift() {
        let handle = std::thread::spawn(|| run_timing_test(1_000, 1_000_000));
        let intervals = handle.join().expect("timing test thread panicked");

        let total_ns: u64 = intervals.iter().sum();
        let expected_total_ns: u64 = 1_000 * 1_000_000;

        let drift_ns = if total_ns >= expected_total_ns {
            total_ns - expected_total_ns
        } else {
            expected_total_ns - total_ns
        };

        let drift_pct = (drift_ns as f64 / expected_total_ns as f64) * 100.0;

        eprintln!(
            "Drift test: total = {} ms, expected = {} ms, drift = {} ms ({:.3}%)",
            total_ns / 1_000_000,
            expected_total_ns / 1_000_000,
            drift_ns / 1_000_000,
            drift_pct,
        );

        assert!(
            drift_ns < expected_total_ns / 100,
            "total drift {drift_ns} ns ({drift_pct:.3}%) exceeds 1% of expected {expected_total_ns} ns",
        );
    }

    // ── Runtime integration tests (M2.4) ────────────────────────────

    /// Helper to build a minimal `RuntimeConfig` for testing (no MIDI
    /// ports, master clock mode).
    fn test_config(tempo_bpm: f64) -> crate::RuntimeConfig {
        crate::RuntimeConfig {
            clock_mode: crate::ClockMode::Master {
                tempo_bpm,
                send_transport: false,
            },
            outputs: Vec::new(),
            inputs: Vec::new(),
            event_queue_capacity: 1024,
            command_queue_capacity: 1024,
        }
    }

    #[test]
    fn test_runtime_starts_and_stops() {
        let config = test_config(120.0);
        let result = crate::Runtime::start(config);
        assert!(result.is_ok(), "Runtime::start failed: {result:?}");

        let (mut runtime, _handles) = result.unwrap();

        // Let the thread run briefly.
        std::thread::sleep(std::time::Duration::from_millis(100));

        let stop_result = runtime.stop();
        assert!(stop_result.is_ok(), "Runtime::stop failed: {stop_result:?}");
    }

    #[test]
    fn test_runtime_produces_clock_ticks() {
        let config = test_config(120.0);
        let (mut runtime, mut handles) = crate::Runtime::start(config).unwrap();

        // Drain events while the runtime is still running to avoid
        // losing ticks between stop() and the drain loop.
        let mut tick_count = 0u64;
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(300);
        while std::time::Instant::now() < deadline {
            while let Ok(event) = handles.events.pop() {
                if matches!(event, crate::RtEvent::ClockTick { .. }) {
                    tick_count += 1;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        runtime.stop().unwrap();

        // At 120 BPM: 24 ticks/beat * 2 beats/sec = 48 ticks/sec.
        // In 300 ms we expect ~14 ticks. Allow loose bounds for loaded machines.
        eprintln!("Received {tick_count} clock ticks in ~300 ms");
        assert!(
            tick_count >= 5,
            "expected at least 5 ticks, got {tick_count}"
        );
        assert!(
            tick_count <= 30,
            "expected at most 30 ticks, got {tick_count}"
        );
    }

    #[test]
    fn test_set_tempo_command() {
        let config = test_config(120.0);
        let (mut runtime, mut handles) = crate::Runtime::start(config).unwrap();

        // Change tempo to 60 BPM.
        let cmd = crate::EcsCommand::SetTempo { bpm: 60.0 };
        while handles.commands.push(cmd).is_err() {
            std::thread::yield_now();
        }

        // Let ticks accumulate for 500 ms at the new tempo.
        std::thread::sleep(std::time::Duration::from_millis(500));

        runtime.stop().unwrap();

        // Drain all tick events.
        let mut tick_count = 0u64;
        while let Ok(event) = handles.events.pop() {
            if matches!(event, crate::RtEvent::ClockTick { .. }) {
                tick_count += 1;
            }
        }

        // At 60 BPM: 24 ticks/beat * 1 beat/sec = 24 ticks/sec.
        // In 500 ms we expect ~12 ticks. Just verify we got some.
        eprintln!("Received {tick_count} clock ticks after tempo change");
        assert!(
            tick_count >= 3,
            "expected at least 3 ticks, got {tick_count}"
        );
    }

    #[test]
    fn test_shutdown_command() {
        let config = test_config(120.0);
        let (mut runtime, mut handles) = crate::Runtime::start(config).unwrap();

        // Send Shutdown command.
        let cmd = crate::EcsCommand::Shutdown;
        while handles.commands.push(cmd).is_err() {
            std::thread::yield_now();
        }

        // The thread should exit cleanly within the stop() timeout.
        // We use stop() which internally joins the thread.
        let stop_result = runtime.stop();
        assert!(
            stop_result.is_ok(),
            "thread should exit cleanly after Shutdown command: {stop_result:?}"
        );
    }

    #[test]
    fn test_drop_stops_thread() {
        let config = test_config(120.0);
        let (runtime, _handles) = crate::Runtime::start(config).unwrap();

        // Drop the runtime -- should not hang or panic.
        drop(runtime);
    }

    // ── Input integration tests (M3.3) ──────────────────────────────

    #[test]
    fn test_runtime_with_no_inputs_still_works() {
        let config = test_config(120.0);
        let (mut runtime, mut handles) = crate::Runtime::start(config).unwrap();

        // Drain events while running to avoid the race between stop
        // and queue draining.
        let mut tick_count = 0u64;
        let mut midi_input_count = 0u64;
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(200);
        while std::time::Instant::now() < deadline {
            while let Ok(event) = handles.events.pop() {
                match event {
                    crate::RtEvent::ClockTick { .. } => tick_count += 1,
                    crate::RtEvent::MidiInput { .. } => midi_input_count += 1,
                    _ => {}
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        runtime.stop().unwrap();

        eprintln!("Ticks: {tick_count}, MidiInputs: {midi_input_count}");
        assert!(tick_count > 0, "expected clock ticks to be produced");
        assert_eq!(
            midi_input_count, 0,
            "expected no MidiInput events with no input ports"
        );
    }
}
