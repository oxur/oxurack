# Baseline findings

Captured against implementation source
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162` on 2026-09-17. These are bounded
source observations for R-03 and dependency observations for the selected
TM-first, then workbench path. They do not implement or approve any contract.

The distinction used here is:

* **Exists** means a source symbol or local test is present.
* **Observed** means the current command or source inspection demonstrated the
  behavior described.
* **Missing acceptance** means the source does not establish the end-to-end
  contract required by the milestone or retained project gate.

The source searches are preserved in `artifacts/logs/41-source-tm-adapter-search.log`,
`42-source-workbench-inventory.log`, `43-source-persistence-command-search.log`,
`45-source-ignored-tests.log` and the corrected `46-source-midi-graph-search-corrected.log`.
The single failed source searches and their corrected line views are also
preserved in logs 44, 47, 48 and 49–53.

## TM-01 — rack registration and lifecycle

`oxurack-core/src/module.rs:127-187` defines `OxurackModule` with metadata,
port and parameter schemas, a spawn function, and parameter accessors.
`module.rs:252-280` shows `ModuleRegistry::register` storing the module
metadata and spawner. It does not register a module behavior or tick callback.
`oxurack/src/lib.rs:77-95` shows `Oxurack::install` registering a module and
its setters/getters. `oxurack-mod-turingmachine/Cargo.toml:1-21` depends on
rand, midir, serde and thiserror, without `oxurack-core` or Bevy; its
`src/lib.rs:1-22` reexports the standalone engine. No TM implementation of
`OxurackModule` was found.

The generic umbrella install tests pass, but they do not install or tick a TM.
The rack adapter, registration feature wiring, entity lifecycle and repeated or
multiple-instance behavior are therefore missing acceptance. This blocks M1.
Re-entry requires an explicitly owned adapter design and a source/test probe
that installs the TM through the rack API and exercises its lifecycle.

## TM-02 — clock input and tick semantics

`oxurack/src/lib.rs:57-74` configures Bevy fixed time at 1 ms, while
`lib.rs:155-162` lets `Oxurack::step` produce zero or more fixed updates based
on elapsed time. With the `rt-bridge` feature,
`oxurack-core/src/bridge.rs:97-122` converts an RT `ClockTick` into `TickNow`
with a PPQN-derived frame. No source path connects `TickNow` to the standalone
TM engine's `tick()` method. The RT clock files found by the inventory are
timing infrastructure, not a rack Clock module.

The current tests establish fixed scheduling and bridge conversion separately;
they do not establish musical pulse division, reset/start behavior, exact
one-pulse-per-TM-step behavior, or the relationship between the 1 kHz ECS fixed
step and a musical clock. That missing handoff blocks M1. Re-entry requires a
named clock-to-module contract and a deterministic test at the chosen timing
boundary.

## TM-03 — signal propagation and module execution order

`oxurack-core/src/lib.rs:163-199` installs the strict `Produce -> Propagate ->
Consume` schedules. `oxurack-core/src/tick.rs:207-248` performs cable
propagation and `tick.rs:251-275` consumes values using sample-and-hold when a
tick has no contribution. The TM engine's `tick()` is implemented at
`oxurack-mod-turingmachine/src/engine.rs:85-120`, but no rack system invokes
it or converts its `StepOutputs` into ECS port values.

The core schedule and its unit tests pass as local substrate observations. They
do not demonstrate TM execution in the schedule, edge latency, gate retrigger
behavior, or output visibility to a downstream module. M1 re-entry requires a
test fixture that drives the selected clock, runs the rack schedule, and checks
the complete TM-to-port-to-MIDI path with the documented ordering.

## TM-04 — seed handoff and reproducibility

`oxurack-core/src/patch.rs:21-43` carries `Patch::master_seed`.
`patch.rs:505-550` validates and applies a patch, then calls a module spawner
with the world, instance name and parameters; the call site does not pass the
patch seed. Core provides `derive_seed` and `derive_module_rng` in
`oxurack-core/src/rng.rs:14-48`, based on master seed plus instance name.
The standalone TM provides `TuringMachine::with_seed` at
`oxurack-mod-turingmachine/src/engine.rs:55-63` and has a same-process
determinism test at `engine.rs:273-281`.

The available tests show deterministic local constructors, not patch-seed
handoff, fresh-process equivalence, instance ordering, or replay after the
specified reset/patch lifecycle. `TuringMachine::reset` clears register and
divider state while preserving settings and does not reconstruct the RNG. M1
re-entry requires a decision about which state is persisted and a separate
process or equivalent process-boundary replay test.

## TM-05 — port, parameter and output mapping

`oxurack-mod-turingmachine/src/outputs.rs:1-44` exposes note, velocity, gate,
scale note, six pulse outputs, eight gate outputs, div2/div4, noise CC, raw
register bits, length and write probability. The TM `Scale` has private fields
in `quantizer.rs:14-23`, and is a type local to the TM crate. The rack module
trait's schema boundary is in `oxurack-core/src/module.rs:127-187`.

This establishes a broad standalone output value, but no rack port schema,
parameter schema, getter/setter mapping, absent-note policy, range conversion,
or feature-off behavior. The separate `oxurack-core::Scale` and TM quantizer
types are not evidence of an adapter mapping. M1 re-entry requires the mapping
to be written as an owned contract and checked through the public rack API.

## TM-06 — MIDI event path and MIDI-out module

Core supports a scalar MIDI value in `oxurack-core/src/value.rs:21-50` and
reexports the shared structured type from `oxurack-midi/src/lib.rs:1-26`.
The RT command type has `SendMidi` in `oxurack-rt/src/messages.rs:56-91`, and
the RT package contains physical port code in `oxurack-rt/src/midi_io.rs`.
The TM's optional direct MIDI wrapper is in
`oxurack-mod-turingmachine/src/midi_io.rs`; it is not a rack module. The file
inventory in log 42 found no rack `midi-out` module.

The core and RT representations and conversion tests pass, but no TM adapter
emits ordered note-on/note-off, CC, gate or auxiliary events into the rack
output queue. The direct standalone MIDI feature cannot prove rack delivery,
backpressure or cleanup. M1 re-entry requires the output port/event contract
and a no-hardware queue test before a physical-device test.

## TM-07 — runtime ownership, queue and shutdown

The optional core bridge defines `MidiOutputQueue` and queue conversion in
`oxurack-core/src/bridge.rs:43-52` and `bridge.rs:156-181`; a full output queue
is drained into the RT ring buffer and a warning is emitted for dropped
commands. RT lifecycle and priority code live under `oxurack-rt`, but the
umbrella crate metadata does not include an RT runtime dependency. The public
command dispatch in `oxurack-core/src/event.rs:301-350` leaves `Panic` and
`SetBpm` as successful no-ops.

The queue and conversion unit tests pass. No current source evidence shows the
umbrella owning RT start/stop, queue insertion, panic cleanup, or a loss policy
for musical events. This blocks M1's MIDI milestone and remains a release risk.
Re-entry requires a named runtime ownership boundary, queue-full semantics,
shutdown/panic tests, and a hardware acceptance run with the required setup.

## TM-08 — patch load/save and live state capture

`oxurack-core/src/patch.rs:455-480` serializes a supplied `Patch` and saves it
to a file. `patch.rs:826-846` emits `PatchLoaded` after applying a loaded
patch, while `patch.rs:895-907` loads the file and applies it to the world.
There is no live-world-to-`Patch` capture path in these symbols. The queued
command dispatch in `event.rs:301-350` returns `NotImplemented` for
`LoadPatch`, `SavePatch`, `AddCable` and `RemoveCable`.

Static patch validation/application is present and tested. Saving a supplied
patch is not the same as capturing current module state, and the queued
operations remain explicit stubs. A static seeded patch can be a bounded M1
fixture if CDC/CRC select that scope; live editing and save/reload remain M2 or
later re-entry work and must retain their original requirements.

## WB-01 — Clock workbench module

The file inventory found RT clock implementations under
`crates/oxurack-rt/src/clock/` and the core `TickNow` bridge, but no rack Clock
module. These existing components do not establish a Bevy module with ports,
BPM/division controls, reset/start semantics or a source for the workbench.
The workbench cannot be accepted from the current source. Re-entry requires
the selected clock module boundary and a test that connects it to a rack
consumer; the retained RT master/slave jitter target remains a separate
hardware gate.

## WB-02 — LFO workbench module

No LFO module file or registered rack implementation was found in the source
inventory. Core value kinds include Float and Bipolar, but those value types do
not implement an LFO's phase, rate, amplitude, reset or tick behavior. WB-02
is therefore a missing dependency for the second workbench milestone. Re-entry
requires a module contract and deterministic phase/reset tests.

## WB-03 — Scope workbench module

No Scope module or bounded signal-history implementation was found. The graph
view's `parameters_of` query is at `oxurack-graph/src/lib.rs:780-824` and its
tests at `lib.rs:2751-2840`; `GraphMaintenance` is topology/parameter
introspection, not signal recording. The umbrella synchronously runs graph
maintenance after patch load at `oxurack/src/lib.rs:145-150`.

Graph freshness and parameter query tests pass for the exercised load path, but
they do not provide a scope buffer, sample timing, overflow policy or display
boundary. WB-03 re-entry requires a bounded recorder and tests showing that
observation does not drive or alter simulation.

## Cross-cutting source gaps

The bounded source checks also confirm the following retained gaps:

* `CoreCommand` dispatch in `oxurack-core/src/event.rs:301-350` returns
  `NotImplemented` for cable mutation and queued patch load/save, while
  `Panic` and `SetBpm` return `Ok(())` without changing state.
* `event.rs:353-420` contains a TODO at the setter dispatch boundary for
  central schema validation. Existing setter errors and parameter tests show
  local behavior, not centralized validation of every module schema.
* `GraphView::parameters_of` and the current getter follow-up are exercised by
  tests in `oxurack-graph/src/lib.rs:2751-2840`. Patch-load freshness is tested,
  but future topology and parameter mutations are not an end-to-end acceptance
  proof.
* The shared MIDI type and RT queue are present, but the inventory found no
  rack MIDI-out module. The four coverage failures are environmental MIDI
  initialization failures, so they do not establish physical MIDI behavior.

## Validation and retained quality gates

The locked build, clippy, format, workspace tests and feature probes passed.
Documentation failed because `RUSTDOCFLAGS=-Dwarnings` promoted unresolved
links to `NonZeroUsize` in `crates/oxurack-mod-turingmachine/src/length.rs:67`
and `self.probability` in `write_knob.rs:36`. The direct dependency command
completed and listed available updates; no dependency update was made.

Both coverage commands failed in `oxurack-rt` because MIDI support could not be
initialized. The failed tests were `test_open_input_nonexistent_returns_error`,
`test_list_output_ports`, `test_open_nonexistent_port_returns_error` and
`test_list_input_ports`; each expected a port-level result but reached the MIDI
initialization error. The run reached 146 passing and 3 ignored tests and
produced no overall coverage percentage or denominator.

The three ignored tests are in `oxurack-rt/src/thread.rs:750` and
`thread.rs:796` (timing/jitter and drift checks) and
`oxurack-rt/src/priority.rs:92` (scheduler-priority effects). Their source
comments name RT scheduling permission or quality-run prerequisites. The
project's 95 percent coverage target, RT P99 target, 10/50/100 instance
benchmarks, 24-hour soak, hardware MIDI, DAW session and Operator musical
acceptance remain retained obligations and were not run by this slice.
