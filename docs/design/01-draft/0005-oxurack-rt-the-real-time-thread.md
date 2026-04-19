---
number: 6
title: "Oxurack-rt: The Real-Time Thread"
author: "the ECS"
component: All
tags: [change-me]
created: 2026-04-18
updated: 2026-04-18
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# Oxurack-rt: The Real-Time Thread

## Purpose

The `oxurack-rt` crate owns the real-time concerns of oxurack: MIDI clock
generation and tracking, MIDI message I/O with microsecond-level timing, and
the lock-free queue bridge to the rest of the system. It runs on one or more
dedicated OS threads elevated to real-time priority.

This crate has no dependency on Bevy, on any module crate, or on anything
else in the oxurack workspace. The only shared code is a tiny set of message
type definitions (the SPSC queue payloads) that live either in this crate
and are re-exported by `oxurack-core`, or in a separate `oxurack-rt-abi`
crate that both import. The current recommendation is the latter: a
minimal ABI crate keeps `oxurack-rt` fully standalone and reusable.

The discipline driving this design: the RT thread's only job is to be on
time. Everything else — pattern generation, cable routing, patch state,
user interaction — lives above in the ECS world. This separation is what
gives the RT thread a chance of meeting its timing budget when oxurack is
running alongside a DAW with a heavy session.

---

## 1. Crate Layout

```
crates/oxurack-rt/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs              # Public API surface, Runtime struct
    ├── priority.rs         # RT priority elevation, platform abstraction
    ├── timing.rs           # Monotonic clock, spin-sleep integration
    ├── clock/
    │   ├── mod.rs          # Clock trait, shared types
    │   ├── master.rs       # Master mode: tempo-driven generator
    │   ├── slave.rs        # Slave mode: PLL and transport handling
    │   └── passthrough.rs  # Slave+master chained mode (v1.1+)
    ├── midi_io.rs          # midir wrapper, input/output port management
    ├── messages.rs         # SPSC queue message types (the ABI)
    ├── queues.rs           # rtrb wrapper, queue constructors
    └── thread.rs           # The RT thread loop itself
```

---

## 2. Dependencies

```toml
[dependencies]
midir                  = "0.10"      # MIDI I/O
rtrb                   = "0.3"       # SPSC lock-free ring buffers
spin_sleep             = "1"         # hybrid sleep-then-spin timing
quanta                 = "0.12"      # fast, drift-corrected monotonic clock
audio_thread_priority  = "0.34"      # platform RT priority elevation
thiserror              = "2"

[dev-dependencies]
pretty_assertions      = "1"
# A MIDI loopback or virtual port tool for integration tests is platform-
# specific; developer documentation covers local setup.
```

Intentionally minimal. No async runtime. No logging framework in the RT
code path (logging is allowed only from non-RT setup/teardown code; the hot
path uses `UnsafeCell<Option<RtError>>` or a similar shared error slot that
non-RT code polls).

---

## 3. Public API (lib.rs)

The crate's public surface is deliberately small. Consumers (the `oxurack`
binary and `oxurack-core`) construct a `Runtime`, receive a pair of queue
handles, and then ignore the RT thread entirely.

```rust
use std::time::Duration;

/// Handle to the RT thread. Drop to stop the thread cleanly.
pub struct Runtime {
    join_handle: Option<std::thread::JoinHandle<()>>,
    shutdown_signal: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

/// Configuration for starting the runtime.
pub struct RuntimeConfig {
    pub clock_mode: ClockMode,
    pub midi_outputs: Vec<MidiOutputConfig>,
    pub midi_inputs: Vec<MidiInputConfig>,
    pub queue_capacity: usize,        // per-queue capacity, default 4096
    pub tick_period: Duration,         // internal scheduling granularity
}

pub enum ClockMode {
    Master { tempo_bpm: f64, send_transport: bool },
    Slave { clock_input_port: String, timeout: Duration },
    // Passthrough { ... } — v1.1+
}

pub struct MidiOutputConfig {
    pub port_name: String,             // matched against midir enumeration
    pub name_for_logging: String,
}

pub struct MidiInputConfig {
    pub port_name: String,
    pub name_for_logging: String,
    pub listens_for_clock: bool,       // true for the slave-mode input
}

/// Handles used by the ECS world to communicate with the RT thread.
pub struct RtHandles {
    pub rt_to_ecs: rtrb::Consumer<RtEvent>,
    pub ecs_to_rt: rtrb::Producer<EcsCommand>,
}

impl Runtime {
    /// Start the RT thread. Blocks until the thread has elevated priority
    /// and opened all configured MIDI ports. Returns on success with the
    /// runtime handle and the queue handles for the ECS side.
    pub fn start(config: RuntimeConfig) -> Result<(Self, RtHandles), Error> { ... }

    /// Signal the RT thread to exit and join it. Also called on drop.
    pub fn stop(mut self) -> Result<(), Error> { ... }
}

impl Drop for Runtime {
    fn drop(&mut self) { ... }
}
```

Priority elevation happens inside `Runtime::start`. If elevation fails
(e.g., insufficient permissions on Linux, or the API call fails on macOS),
`start` returns an error with a descriptive message. We do not silently
fall back to normal priority.

---

## 4. Queue Message Schemas

These types form the ABI between the RT thread and the ECS world. They
must be `Copy` wherever possible, `Send`, and use only fixed-size data
(no `String`, no `Vec`, no heap-allocated types) to keep queue operations
wait-free.

### 4.1 RT → ECS events

```rust
#[derive(Debug, Clone, Copy)]
pub enum RtEvent {
    /// A clock tick arrived. `subdivision` is the tick's position within
    /// the current beat (0..23 for 24 PPQN). `beat` is the current beat
    /// number since the last Start/Continue. `tempo_bpm` is the current
    /// estimated tempo (equal to configured tempo in master mode, PLL
    /// output in slave mode).
    ClockTick {
        subdivision: u8,
        beat: u64,
        tempo_bpm: f64,
        timestamp_ns: u64,    // monotonic clock timestamp of the tick
    },

    /// External transport control (slave mode) or confirmation (master mode).
    Transport(TransportEvent),

    /// An incoming MIDI message from a configured input port.
    /// Clock and transport messages are filtered out and become
    /// ClockTick / Transport events above; only non-clock, non-transport
    /// messages reach here.
    MidiInput {
        input_port_index: u8,
        timestamp_ns: u64,
        message: MidiMessage,   // fixed-size representation; see below
    },

    /// Song Position Pointer event (0xF2). Fires on both master and
    /// slave modes. Value is in MIDI beats (sixteenth notes, 0..16383).
    SongPosition { position: u16 },

    /// The RT thread has encountered an error it wants to report but not
    /// fail on. Non-fatal.
    NonFatalError(RtErrorCode),
}

#[derive(Debug, Clone, Copy)]
pub enum TransportEvent {
    Start,
    Stop,
    Continue,
}

/// A fixed-size representation of a MIDI message.
/// SysEx is handled separately and does not pass through this path.
#[derive(Debug, Clone, Copy)]
pub struct MidiMessage {
    pub status: u8,
    pub data1: u8,
    pub data2: u8,
    pub length: u8,   // 1, 2, or 3 (excluding padding)
}
```

### 4.2 ECS → RT commands

```rust
#[derive(Debug, Clone, Copy)]
pub enum EcsCommand {
    /// Schedule a MIDI message to be emitted at a future moment.
    /// `timestamp_ns` is an absolute monotonic-clock timestamp.
    /// The RT thread emits the message at that moment (within jitter budget).
    /// If the timestamp is in the past when dequeued, emit immediately.
    SendMidi {
        output_port_index: u8,
        timestamp_ns: u64,
        message: MidiMessage,
    },

    /// Request a tempo change (master mode only).
    SetTempo { bpm: f64 },

    /// Request a transport action (master mode only).
    SendTransport(TransportEvent),

    /// Send Song Position Pointer (master mode only).
    SendSongPosition { position: u16 },

    /// Request a clean shutdown of the RT thread.
    Shutdown,
}
```

SysEx support is deferred. When we add it, it will be a separate
non-wait-free path (since SysEx messages are variable-length, often large,
and used only out of the real-time critical window — patch dumps, device
configuration, etc.).

---

## 5. Clock: Master Mode

Master mode is straightforward.

The RT thread maintains a monotonic tick schedule. At tempo T (BPM), the
inter-tick interval is `60.0 / (T * 24.0)` seconds — for 120 BPM, that is
approximately 20.833 milliseconds. The thread loop uses `spin_sleep` to
sleep until shortly before the next tick, then spins until the `quanta`
monotonic clock crosses the scheduled tick timestamp, then:

1. Emits a MIDI Clock byte (0xF8) on every configured output port whose
   config requests clock-out.
2. Emits any MIDI messages whose scheduled timestamp is at or before now
   (drained from `ecs_to_rt`).
3. Pushes a `ClockTick` event into `rt_to_ecs`.
4. Computes the next tick's scheduled timestamp: `last + interval`. Does
   not accumulate drift — each tick's schedule is derived from the previous
   *scheduled* time, not the previous *actual* emission time.

Start / Stop / Continue handling: when an `EcsCommand::SendTransport`
arrives, the RT thread emits the corresponding byte (0xFA / 0xFC / 0xFB)
on configured output ports and, for Start, resets the internal beat
counter to 0.

Tempo changes: when an `EcsCommand::SetTempo` arrives, the next tick's
schedule is recomputed based on the new tempo. The current tick (if the
command arrived mid-interval) completes on the old schedule.

Song Position Pointer: on Start, position is reset to 0 automatically.
Explicit SPP via `EcsCommand::SendSongPosition` emits the appropriate
two-byte message and resets the internal beat counter to the specified
position (in sixteenth notes, per the MIDI spec).

---

## 6. Clock: Slave Mode

Slave mode is where the real engineering lives.

The RT thread opens the configured clock-input MIDI port, registers a
callback with `midir`, and receives incoming bytes. The callback runs on
whatever thread `midir` uses internally (platform-specific) and must do
minimal work — it writes the arrival timestamp (via `quanta`) and the byte
into a small internal SPSC queue feeding the RT thread's main loop. The
main loop polls this queue at high frequency.

Clock messages (0xF8) arrive nominally every `60.0 / (T * 24.0)` seconds.
In practice:

- USB-MIDI has millisecond-scale jitter driven by USB transaction timing.
- macOS IAC bus has scheduler-driven jitter, typically sub-millisecond
  but occasionally worse under load.
- Hardware MIDI (via a USB-MIDI interface) has 31.25 kbaud serialisation
  jitter plus the interface's processing delay.
- DAWs sometimes emit clock in bursts rather than at exact intervals.

If we naively used each incoming tick's arrival time to drive internal
ticks, downstream timing would jitter audibly — especially Euclidean
patterns and fast subdivisions.

### 6.1 PLL / tempo estimator design

The slave clock uses a phase-locked loop structure:

1. **Phase detector.** Compute the error between the expected next-tick
   time (based on the current estimated tempo and the phase we think we
   are at) and the actual arrival time of the incoming tick.

2. **Loop filter.** A one-pole lowpass (exponential moving average) on the
   instantaneous tempo inferred from the most-recent few ticks, plus a
   fractional adjustment applied to the phase. Time constant on the
   tempo filter is tunable; a starting value of about 8 incoming ticks
   (1/3 of a beat) feels right — fast enough to follow real tempo
   changes, slow enough to reject jitter.

3. **Controlled oscillator.** The RT thread's internal tick generator,
   running at the filtered tempo estimate. Internal ticks fire at the
   filtered rate, not at the raw incoming times. This is what downstream
   modules actually see.

The estimator reports its tempo to the ECS world with each tick event so
modules can display and respond to the current tempo.

Edge cases to handle:

- **Startup.** Before we have enough ticks to estimate tempo, we either
  wait silently (no ticks emitted) or use a configured fallback tempo.
  Preferred: wait, and emit a `NonFatalError(RtErrorCode::ClockNotLocked)`
  until we have at least 4 incoming ticks to establish an initial estimate.
- **Tempo jumps.** DAWs sometimes change tempo abruptly (user clicks a
  new BPM, or a tempo automation curve crosses a segment boundary). The
  estimator should detect large phase errors and reset to the new tempo
  quickly rather than filtering slowly.
- **Clock dropout.** If no ticks arrive for longer than about 4 expected
  intervals, consider the clock lost. Emit a `NonFatalError`, stop
  producing internal ticks, and wait for re-establishment.
- **Transport messages.** Start (0xFA) resets the internal beat counter
  to 0. Stop (0xFC) halts internal tick emission (modules see no more
  clock events until Continue or Start). Continue (0xFB) resumes from
  the current position.
- **SPP.** Song Position Pointer (0xF2 followed by two data bytes)
  updates the internal beat counter. Must be handled before the next
  Start or Continue to take effect correctly.

### 6.2 Reference implementations

We are not the first people to need a MIDI clock PLL. References:

- The `mseq` crate on crates.io has a working master+slave implementation.
  Its architectural shape (Conductor / Context runtime) does not fit
  oxurack, but its clock-tracking logic is studied as a reference.
- The JUCE MidiMessageCollector and equivalent C++ sources.
- Ableton Link's clock synchronisation model (more complex than we need
  but worth understanding as an upper bound).

Our own implementation starts simple (one-pole filter on the tempo, no
proportional-integral term) and iterates based on measured jitter against
a DAW test harness.

### 6.3 Jitter budget

The target jitter budget on the internal tick stream, measured against a
known-good external master:

- **Median jitter**: < 100 µs.
- **P99 jitter**: < 500 µs.
- **Worst case**: < 2 ms (anything worse is a bug).

These are ambitious but achievable with a proper PLL on a modern Mac with
reasonable background load. If measurement shows we cannot meet these
budgets with `midir` alone, we revisit and consider `coremidi` on macOS
with kernel-scheduled message delivery. The architecture allows for this
substitution without touching any other crate.

---

## 7. MIDI I/O

MIDI input and output is handled via `midir`. On construction, the RT
thread enumerates available ports, matches the configured names, opens
connections, and stores the handles in stack arrays indexed by the
config's port indices.

Output: the RT thread drains the `ecs_to_rt` queue each tick cycle and
emits any messages whose scheduled timestamp has arrived. Messages with
future timestamps remain in the queue until their moment. A small internal
priority queue (binary heap or sorted ring buffer) orders messages by
scheduled time; this is the one place heap allocation is permitted in
the hot path, with pre-allocated capacity set from `RuntimeConfig`.

Input: `midir` delivers messages via callback on a platform thread. The
callback writes arrival timestamp and bytes into the internal SPSC queue
feeding the main RT loop. The main loop parses and dispatches (clock
bytes go to the clock subsystem, transport bytes go to the transport
subsystem, everything else becomes `RtEvent::MidiInput`).

SysEx and running-status handling: SysEx is deferred (as noted). Running
status is handled transparently during input parsing — when a status byte
is omitted, the previous status is applied.

---

## 8. Timing Primitives

Monotonic time: `quanta::Instant` and `quanta::Clock`. `quanta` wraps
platform-specific fast clock APIs (CLOCK_MONOTONIC_RAW on Linux,
mach_continuous_time on macOS, QueryPerformanceCounter on Windows) with
drift correction and calibration. Measured nanosecond accuracy on
recent hardware.

Sleeping: `spin_sleep::sleep`. `spin_sleep` sleeps via the OS up to a
configurable "native accuracy" threshold before the target time, then
spins for the remainder. The default threshold is conservative; for
oxurack's needs we configure it to 1 ms on macOS (which typically
delivers ~1 ms OS sleep accuracy).

The combination of `quanta` timestamps and `spin_sleep` sleeping gives
us sub-millisecond scheduling accuracy when RT priority is elevated and
the machine is not completely pegged.

---

## 9. Priority Elevation

`audio_thread_priority` handles the platform-specific policy calls. On
macOS the relevant API is `thread_policy_set` with
`THREAD_TIME_CONSTRAINT_POLICY`, which requires specifying period,
computation, and constraint times in mach absolute time units. The crate
abstracts this.

Elevation happens once, inside the RT thread itself, immediately after
it starts. If it fails, the thread reports the error via the startup
channel used by `Runtime::start`, which then returns an error to the
caller.

On macOS, elevation typically succeeds without special privileges. On
Linux, elevation requires either CAP_SYS_NICE, PAM rtprio settings, or
running as root. We document the setup requirements in the crate README
and fail loudly if elevation fails.

---

## 10. Testing Strategy

Unit tests cover the pure logic: PLL behaviour on synthetic tick streams,
tempo estimator convergence, transport state machine, message schema
round-tripping.

Integration tests use a virtual MIDI port pair (on macOS, the IAC bus; on
Linux, snd-virmidi) with a test harness that either generates a known
clock stream into the input or captures our output for analysis.

Timing tests: a dedicated test binary that drives the RT thread for a
fixed duration, measures actual vs. scheduled emission times, and
reports jitter statistics. This is run manually during development
(timing tests are inherently flaky in CI).

Determinism tests: given a seeded RNG-free workload (a specific sequence
of `EcsCommand::SendMidi` calls), the thread's MIDI output must match
byte-for-byte across runs.

---

## 11. Platform Matrix

v1 targets macOS primary. Linux and Windows are expected to work but are
not the primary test targets.

**macOS.** Primary. Uses CoreMIDI via `midir`, mach time via `quanta`,
time-constraint policy via `audio_thread_priority`. Tested on Apple Silicon
(M1/M2/M4 class). Should work on Intel Macs as well but not verified.

**Linux.** Uses ALSA via `midir`, CLOCK_MONOTONIC_RAW via `quanta`,
SCHED_FIFO via `audio_thread_priority`. Requires appropriate rtprio limits
or CAP_SYS_NICE. Tested informally; no CI coverage in v1.

**Windows.** Uses WinMM via `midir`, QPC via `quanta`, priority-boosted
threads via `audio_thread_priority`. Expected to work with worse jitter
than the Unix platforms. Not a v1 target.

---

## 12. Failure Modes and Recovery

Failure modes the RT thread must handle:

- **MIDI output port disappears** (device unplugged). Stop emitting to
  that port, emit `NonFatalError(RtErrorCode::OutputPortLost)`, continue
  operating on remaining ports. Periodically attempt re-enumeration.
- **MIDI input port disappears** (in slave mode). Emit
  `NonFatalError(RtErrorCode::InputPortLost)`, stop producing internal
  clock ticks, wait for re-establishment.
- **Queue full** on `rt_to_ecs` (ECS side falling behind). This should
  never happen at normal workloads but may under pathological load. Drop
  the event, emit `NonFatalError(RtErrorCode::QueueOverflow)`. Dropping
  is preferable to blocking because blocking the RT thread is worse than
  losing one event.
- **Queue full** on `ecs_to_rt` (ECS side pushing faster than RT drains).
  The ECS side sees this as a failed push; its system should handle the
  failure appropriately (batch up messages, drop oldest, etc.). The RT
  thread itself is not involved.
- **Priority elevation failure**. Fail at startup; do not silently
  continue at normal priority.

---

## 13. Build Order

1. **Scaffold the crate** with minimal deps and the public API surface
   compilable but `unimplemented!()`. Confirms the Cargo tree shape.
2. **Priority + timing**. Get a thread running at RT priority that wakes
   up once a millisecond and reports accurate timing statistics.
3. **MIDI output + master clock**. Get a clean 24-PPQN clock emitting on
   a configured output port. Measure jitter against an external reference
   (another DAW or a hardware MIDI monitor).
4. **Queue integration**. Wire up `rtrb` queues both directions. Write
   test harnesses for each.
5. **MIDI input + message parsing**. Receive messages on a configured
   input port, parse them, surface as `RtEvent::MidiInput`.
6. **Slave clock + PLL**. The deepest engineering. Start with a simple
   moving-average tempo estimator; measure against real DAW output;
   iterate.
7. **Transport and SPP**. Handle Start / Stop / Continue / SPP in both
   master and slave modes. Test with Logic Pro as the reference master.
8. **Error handling polish**. Port disappearance, queue overflow,
   elevation failure. Document recovery behaviour.

---

## 14. Open Questions

**Q1. Coremidi direct vs. midir.** `midir` is the obvious choice for v1:
cross-platform, well-maintained, shared with the existing `turingmachine`
crate's MIDI I/O feature. If jitter measurement reveals we cannot meet
the timing budget through `midir` on macOS, the fallback is direct
`coremidi` use with kernel-scheduled message delivery. Architecture
permits the substitution; do not pre-emptively complicate.

**Q2. Running status emission.** Some MIDI devices expect or prefer
running-status-compressed output. Our v1 emission always includes the
full status byte (simpler, more robust). If any target device misbehaves,
we revisit.

**Q3. MIDI 2.0 readiness.** The `midir` crate does not yet support MIDI
2.0's UMP (Universal MIDI Packet) format. When it does, we extend
`MidiMessage` or add a parallel `MidiMessage2` type. Not v1 scope.

**Q4. Multiple input ports for clock.** Currently one slave-clock input
is supported. If a user wants multiple input sources (e.g., two DAWs in
sync), we would need priority / arbitration logic. Defer to v1.1+ unless
a concrete use case emerges.

**Q5. Queue capacity tuning.** The default of 4096 per queue is a guess.
During integration testing with realistic workloads we measure actual
queue depth and adjust the defaults (or expose a per-workload tuning
knob if no single value works).
