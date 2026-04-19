# Project Plan: `oxurack-rt` Crate Implementation

## Context

The `oxurack-rt` crate is Tier 1 of the oxurack architecture — the real-time MIDI thread that handles clock generation/tracking, MIDI I/O, and the lock-free queue bridge to the ECS world. It is the first piece of new infrastructure needed before any module can produce audible output through the rack. In doc 0009's build order, the RT crate corresponds to Phase 0 deliverables P0-7 through P0-9 (skeleton + master clock + queues), with the slave clock deferred to Phase 4.

Design spec: `docs/design/01-draft/0006-oxurack-rt-the-real-time-thread.md`
Architecture: `docs/design/01-draft/0005-oxurack-system-architecture.md`
Build order: `docs/design/01-draft/0009-module-catalog-and-build-order-v2.md`

The crate has NO dependency on Bevy or any other oxurack crate. It is fully standalone and reusable.

---

## Phase 1: Scaffold and Foundation

**Goal**: Compilable crate with full public API types, working RT priority elevation, and microsecond-precision timing primitives.

**Aligns with**: Doc 0006 build steps 1-2.

### Milestone 1.1 — Crate scaffold compiles

Create the full file tree from doc 0006 section 1:

```
crates/oxurack-rt/
├── Cargo.toml          # edition 2024, deps from doc 0006 section 2
├── README.md
└── src/
    ├── lib.rs           # Runtime, RuntimeConfig, ClockMode, RtHandles, re-exports
    ├── error.rs         # Error enum via thiserror (PriorityElevation, MidiInit, PortNotFound, QueueFull, ThreadPanicked, AlreadyStopped)
    ├── priority.rs      # elevate_rt_priority() placeholder
    ├── timing.rs        # MonotonicClock placeholder
    ├── clock/
    │   ├── mod.rs       # Shared types (TickSchedule), interval_ns_from_bpm()
    │   ├── master.rs    # MasterClock placeholder
    │   ├── slave.rs     # SlaveClock placeholder
    │   └── passthrough.rs  # Stub (v1.1+)
    ├── midi_io.rs       # MidiPorts placeholder
    ├── messages.rs      # RtEvent, EcsCommand, MidiMessage, TransportEvent, RtErrorCode — all fully defined, Copy + Clone + Debug + Send
    ├── queues.rs        # Queue constructor placeholder
    └── thread.rs        # RT loop placeholder
```

All method bodies `unimplemented!()`. Message types fully defined (they're pure data).

**Accept**: `cargo build -p oxurack-rt && cargo clippy -p oxurack-rt -- -D warnings`

### Milestone 1.2 — Message types complete and tested

Implement in `messages.rs`:
- `MidiMessage::note_on()`, `note_off()`, `cc()`, `from_bytes()`, `to_bytes()`
- Size assertions: `MidiMessage` = 4 bytes, `RtEvent` < 64 bytes, `EcsCommand` < 64 bytes
- `Copy + Send + 'static` compile-time checks for both queue types
- Round-trip tests for all constructors

### Milestone 1.3 — Timing primitives work

Implement in `timing.rs`:
- `MonotonicClock` wrapping `quanta::Clock` — `now() -> u64` (ns), `elapsed_since()`
- `precision_sleep(target_ns, clock)` using `spin_sleep` + spin-wait on quanta
- Native accuracy threshold: 1ms on macOS

**Tests**: monotonic advance, sleep accuracy within +/- 2ms, no undershoot

### Milestone 1.4 — RT priority elevation works

Implement in `priority.rs`:
- `elevate_rt_priority()` via `audio_thread_priority::promote_current_thread_to_real_time()`
- Called from within the RT thread itself
- Failure returns `Error::PriorityElevation` with descriptive message

**Tests**: spawn thread, elevate, assert Ok (`#[ignore]` for CI)

### Milestone 1.5 — Foundation integration: 1ms wake-up loop

Implement in `thread.rs`:
- Minimal loop: elevate priority, sleep 1ms N times, record actual wake times
- Expose as test helper

**Tests** (`#[ignore]`): 1000 iterations, assert median jitter < 200us, P99 < 1ms

**Phase 1 exit**: `make check` passes with oxurack-rt in the workspace. Timing test demonstrates sub-millisecond accuracy.

---

## Phase 2: Master Clock and Queue Integration

**Goal**: Working master clock generating 24-PPQN ticks, bidirectional lock-free queues, MIDI Clock bytes on a real output port.

**Aligns with**: Doc 0006 build steps 3-4. Doc 0009 deliverables P0-7 through P0-9.

### Milestone 2.1 — Queue infrastructure

Implement in `queues.rs`:
- `create_queues(capacity) -> (RtSide, EcsSide)` using `rtrb`
- `RtSide`: `Producer<RtEvent>` + `Consumer<EcsCommand>`
- `EcsSide` = `RtHandles`: `Consumer<RtEvent>` + `Producer<EcsCommand>`

**Tests**: roundtrip both directions, full-queue returns error (no block), empty returns None, cross-thread correctness

### Milestone 2.2 — MIDI output port management

Implement in `midi_io.rs`:
- `MidiPorts::open_outputs(configs) -> Result<Self, Error>` — enumerate via midir, match by name, open
- `MidiPorts::send(port_index, bytes)` — raw byte send
- `list_midi_output_ports() -> Result<Vec<String>, Error>` — enumeration helper

**Tests**: `PortNotFound` on nonexistent port, enumeration doesn't error (`#[ignore]`)

### Milestone 2.3 — Master clock generates 24-PPQN ticks

Implement in `clock/mod.rs` and `clock/master.rs`:
- `TickSchedule { next_tick_ns, interval_ns, subdivision, beat }`
- `interval_ns_from_bpm(bpm) -> u64`
- `MasterClock::new(bpm)`, `next_tick()`, `advance()`, `set_tempo()`, `reset()`
- Drift prevention: next tick = previous *scheduled* + interval, not previous *actual*

**Tests**: interval at 120 BPM = 20,833,333 ns, subdivision wraps at 24, beat increments, tempo change, reset, no drift after 24000 ticks

### Milestone 2.4 — RT thread loop with master clock + queues

Implement in `thread.rs` and `lib.rs`:
- Full master-mode loop: elevate priority, open ports, signal readiness, loop (sleep-to-tick, emit 0xF8, drain ecs_to_rt, push ClockTick to rt_to_ecs)
- `Runtime::start(config) -> Result<(Self, RtHandles), Error>`
- `Runtime::stop()`, `Drop` impl
- Handle `EcsCommand::SetTempo`, `EcsCommand::Shutdown`

**Tests**: start/stop lifecycle, ClockTick rate at 120 BPM (~expected count in 200ms), SetTempo changes rate, Shutdown exits within 100ms, Drop stops thread

### Milestone 2.5 — MIDI Clock on a real port (integration)

Integration test (`#[ignore]`): start runtime with IAC bus output, run 2s at 120 BPM, externally verify 0xF8 bytes arrive at ~24 PPQN.

**Phase 2 exit**: `Runtime::start()` works. ECS side reads ClockTick events at correct rate. 0xF8 emitted on configured output. SetTempo and Shutdown handled. This is the point where `oxurack-core` can begin integration.

---

## Phase 3: MIDI Input and Message Parsing

**Goal**: Receive MIDI on input ports, parse into structured types, surface as `RtEvent::MidiInput`. Clock/transport bytes classified but not yet processed by PLL.

**Aligns with**: Doc 0006 build step 5.

### Milestone 3.1 — MIDI input port management

Extend `midi_io.rs`:
- `MidiInputPorts::open_inputs(configs, internal_sender)` — open via midir, callback writes `RawMidiEvent { port_index, timestamp_ns, bytes, length }` into internal SPSC queue
- Callback does minimal work: timestamp via quanta + write to queue
- `list_midi_input_ports()` enumeration helper

### Milestone 3.2 — MIDI message classification

Implement in `messages.rs`:
- `classify_midi(bytes) -> MidiClassification` — separates Clock (0xF8), Start (0xFA), Stop (0xFC), Continue (0xFB), SPP (0xF2), Active Sensing (0xFE, ignored), channel messages
- Running status handling: if first byte < 0x80, apply previous status

**Tests**: classify each message type, running status sequences, SPP LSB/MSB encoding

### Milestone 3.3 — RT thread input integration

Extend `thread.rs`:
- Poll internal raw-MIDI queue each iteration
- Classify each message: channel messages -> `RtEvent::MidiInput`, transport -> `RtEvent::Transport`, clock bytes stored for future slave mode

**Tests**: inject raw bytes into internal queue, assert correct `RtEvent` variants; integration test with IAC bus (`#[ignore]`)

**Phase 3 exit**: MIDI input appears as `RtEvent::MidiInput` on the queue. Clock/transport bytes parsed and classified.

---

## Phase 4: Slave Clock and PLL

**Goal**: Phase-locked loop tracking an external MIDI clock master with jitter within budget (median <100us, P99 <500us).

**Aligns with**: Doc 0006 build step 6. Doc 0009 deliverables P4-1 through P4-3.

**This is the deepest engineering** — tested first with synthetic data, then against real DAW output.

### Milestone 4.1 — Tempo estimator (offline, no thread)

Implement in `clock/slave.rs`:
- `TempoEstimator::new()`, `feed_tick(timestamp_ns)`, `estimated_tempo_bpm() -> Option<f64>`, `is_locked() -> bool`
- Circular buffer of ~48 recent tick timestamps
- Exponential moving average with ~8-tick time constant
- Detect large phase errors (>3x expected interval) -> reset filter
- Detect clock dropout (>4 expected intervals) -> unlock

**Tests**: locks after 4 ticks, smooths jitter (Gaussian sigma 500us -> estimate within 0.5%), follows tempo change within ~8 ticks, detects dropout, handles tempo jump by resetting

### Milestone 4.2 — Phase-locked oscillator

Implement in `clock/slave.rs`:
- `SlaveOscillator` takes estimator output, produces internal tick schedules
- Phase correction: proportional adjustment of next tick time by fraction of phase error
- Pauses when estimator is unlocked

**Tests**: paused when unlocked, produces ticks when locked, output jitter on synthetic 120 BPM + 1ms Gaussian noise meets budget (median <100us, P99 <500us), phase correction converges within ~4 ticks

### Milestone 4.3 — SlaveClock facade

Implement in `clock/slave.rs`:
- `SlaveClock` wraps `TempoEstimator` + `SlaveOscillator`
- `feed_clock_byte(timestamp_ns)`, `next_tick() -> Option<TickSchedule>`, `is_locked()`, `estimated_tempo()`
- Timeout: no clock bytes for configured duration -> unlock

### Milestone 4.4 — Thread loop integration for slave mode

Extend `thread.rs`:
- Branch on `ClockMode::Slave` — poll raw-MIDI queue for clock bytes, feed to SlaveClock, use `next_tick()` to drive emission schedule
- Push `ClockTick` events with PLL-estimated tempo
- Do NOT re-emit 0xF8 (we're a slave)
- Emit `NonFatalError(ClockNotLocked)` when not locked

**Tests** (`#[ignore]`): lock to external 120 BPM source via IAC bus, assert ClockTick events with reasonable tempo; report not-locked when no external clock

**Phase 4 exit**: Slave clock locks within 4 ticks (~80ms). Jitter meets budget against Logic Pro on macOS. Tempo changes followed within 1/3 beat. Dropout detected within 4 expected intervals.

---

## Phase 5: Transport, SPP, and Polish

**Goal**: Complete Start/Stop/Continue/SPP in both modes. Harden all failure modes. API documentation.

**Aligns with**: Doc 0006 build steps 7-8.

### Milestone 5.1 — Transport in master mode

Extend `clock/master.rs` and `thread.rs`:
- Transport state: `is_running`. Start resets position + sets running. Stop clears running (thread loops but no ticks). Continue resumes without reset.
- `EcsCommand::SendTransport` -> emit 0xFA/0xFC/0xFB on outputs + push `RtEvent::Transport`

### Milestone 5.2 — SPP in master mode

Extend `thread.rs` and `clock/master.rs`:
- `EcsCommand::SendSongPosition` -> emit 0xF2 + LSB + MSB, update beat counter
- `MasterClock::set_position_from_spp(midi_beats: u16)`

### Milestone 5.3 — Transport and SPP in slave mode

Extend `clock/slave.rs`:
- Transport state machine: 0xFA resets + starts, 0xFC stops (no ticks even if clock arrives), 0xFB continues
- SPP (0xF2) updates position before next Start/Continue

### Milestone 5.4 — Error handling for failure modes

Implement across `midi_io.rs`, `thread.rs`, `messages.rs`:
- `RtErrorCode` enum: `OutputPortLost`, `InputPortLost`, `QueueOverflow`, `ClockNotLocked`, `ClockDropout`
- Output port disappearance: mark lost, push NonFatalError, stop sending, attempt re-enumeration every ~5s
- Input port disappearance: push NonFatalError, stop producing clock ticks
- Queue overflow: drop event, continue (never block RT thread)

### Milestone 5.5 — API polish and documentation

- `#[non_exhaustive]` on public enums that may grow
- `#[must_use]` where appropriate
- Module-level doc comments on every file
- Public item doc comments
- `crates/oxurack-rt/README.md` with usage example and platform requirements
- `cargo doc -p oxurack-rt --no-deps` clean with no warnings

**Phase 5 exit**: Transport works in both modes. All doc 0006 section 12 failure modes handled. No panics on port loss, queue overflow, or clock dropout. `make check` passes. Crate is documented.

---

## Dependency Graph

```
Phase 1 (Foundation)
    |
    v
Phase 2 (Master Clock + Queues)
    |                \
    v                 v
Phase 3 (Input)     Phase 4.1-4.3 (PLL offline)
    |                 |
    +--------+--------+
             |
             v
        Phase 4.4 (Slave thread integration)
             |
             v
        Phase 5 (Transport + Polish)
```

Phase 4 milestones 4.1-4.3 (PLL algorithm, tested with synthetic data) can be developed in parallel with Phase 3 (MIDI input). Phase 4.4 (integrating slave mode into the thread loop) requires both.

---

## Testing Strategy

| Level | Location | CI? |
|-------|----------|-----|
| Unit tests (message types, clock math, PLL convergence) | `#[cfg(test)]` in each file | Yes |
| PLL tests with synthetic jitter data | `clock/slave.rs` tests | Yes |
| Integration tests (real MIDI ports) | `tests/` directory | `#[ignore]` |
| Timing/jitter benchmarks | `tests/timing.rs` | `#[ignore]` |
| Manual DAW validation (Logic Pro) | Developer workflow | No |

Coverage target: 95%+ on non-`#[ignore]` paths per workspace convention.

PLL test data: synthetic tick streams with known tempo + controlled Gaussian noise, avoiding need for real MIDI hardware in CI.

---

## Key Patterns to Follow

- **Error handling**: use `thiserror` derive (the existing turingmachine crate has thiserror as a dep but implements Error manually; oxurack-rt should use the derive macro idiomatically)
- **Edition**: 2024 (matching turingmachine)
- **License**: MIT OR Apache-2.0 (matching turingmachine Cargo.toml)
- **Dev deps**: `pretty_assertions = "1"` (matching turingmachine)
- **No `#![deny(warnings)]`** in library code (per anti-pattern AP-01)
- **`#[non_exhaustive]`** on public enums (per idiom ID-01)

---

## Verification

After all phases complete:
1. `make check` passes (build + lint + test for full workspace)
2. `make coverage` shows 95%+ on oxurack-rt (excluding `#[ignore]` tests)
3. `cargo doc -p oxurack-rt --no-deps` produces clean docs
4. Manual: start runtime in master mode, verify 0xF8 on IAC bus via MIDI Monitor
5. Manual: start runtime in slave mode with Logic Pro as master, verify ClockTick events track tempo
6. Manual: jitter measurement against known source meets P99 < 500us budget
