# CDC execution contract — TM prerequisites and successors

Decision version 1.0, 2026-09-17. Answers all four design requests in
[CRC escalation03](../../../crc-escalation03.md). Operator authority: TM inside
the Bevy framework producing MIDI, then Clock/LFO/Scope; retain other features.
CDC authorizes the designs below within that scope. **Only arc09/slice01 is an
issued implementation assignment.** Later API detail is filled into its own
slice before execution; do not implement all this document in slice01.

## 1. Module behavior installation — binding for slice01

Add this default method to `oxurack_core::OxurackModule`:

```rust
fn register_systems(
    app: &mut bevy_app::App,
    schedule: bevy_ecs::schedule::InternedScheduleLabel,
) {}
```

The trait already depends on Bevy's World; exposing Bevy's schedule identifier
adds no new framework dependency. The locked Bevy 0.18.1 source defines this alias
in schedule/set.rs. Existing implementations compile unchanged via the no-op
default. The method installs systems/resources, never per-patch module instances
or physical MIDI handles. Systems target the supplied schedule and appropriate
TickPhase; they must not hardcode Update or register per instance.

Add `Oxurack::try_install<M: OxurackModule>(&mut self) -> Result<&mut Self, CoreError>`.
Store successful installations privately per Oxurack, keyed by ModuleKind with
Rust TypeId as the identity witness. TypeId is process-local bookkeeping only;
never persist/hash it into seeds or sort musical events by it. Cases:

- New kind: register metadata/spawner and all setter/getter functions, then call
  register_systems with `FixedUpdate.intern()` exactly once, then record success.
- Same Rust type already installed under that kind: return the same rack without
  re-registering metadata/accessors, resetting resources or running the hook again.
- Different Rust type claiming an installed kind: return a new
  `CoreError::ModuleKindConflict { kind: String }` before changing any resource,
  registry, getter/setter, spawner or schedule. Error text names the kind.

Retain `install<M>() -> &mut Self` as the existing chaining convenience. Delegate
to try_install and document its panic on programmer-supplied kind collision;
callers selecting modules dynamically should use try_install. Patch-file input
must not trigger this panic path. Do not silently return success for a conflict.
Registration-hook panic is a programmer failure, not a recoverable installation
transaction; do not catch unwind or promise rollback of arbitrary user systems.

Installation metadata is per rack, not a global singleton; two racks install
independently. Multiple patch instances share the registered systems, which query
all matching entities. Loading a patch creates instances through the existing
spawner and does not reinstall behavior. Raw ModuleRegistry registration remains
metadata-only and unchanged; app owners using core directly may call the hook
with their schedule explicitly and own that lifecycle. No generic core installer,
plugin registry redesign or factory abstraction is required in slice01.

Keep `Oxurack::step`, timing, SimulationRunning, patch seeding, TickNow and phase
ordering unchanged in slice01. A Produce-phase fixture is sufficient to prove
installation. Consuming-input timing is deliberately implemented in slice02.

## 2. Simulation, musical clocks and visibility — arc09/slice02 + arc10

Keep one logical simulation sample at 1 ms for this milestone. A sample index and
fixed duration form the authoritative module-time input; wall/render time merely
drives how many samples the live driver requests. Existing step() remains an app
update that can run zero or more fixed samples. Provide a separate explicit manual
sample API for deterministic fixtures, sharing the same pipeline as live driving.
Manual and live modes are mutually exclusive so a manual step cannot also consume
Bevy's accumulated elapsed time; switching mode must discard/settle accumulated
live time with documented semantics. Do not sleep in exact-sample tests.

Choose sample-delayed module processing, not implicit topological execution:
Produce publishes autonomous/current sample outputs and state computed on the
previous sample; Propagate gathers cables; a named MergeInputs phase completes
all input merges; module Consume systems then read those current merged inputs
and update state for the next Produce; Observe runs last. Move core's internal
merge out of the unordered Consume set into MergeInputs. Keep existing Produce,
Propagate and Consume labels, adding MergeInputs/Observe explicitly. Stateful
Clock→TM→sink paths therefore have documented one-sample computational latency
per stateful stage. No claim that cable sorting executes a whole chain in one
sample. Tests must assert the full sample-indexed trace, not just final values.

Move the shared running-state contract to core (re-export the existing umbrella
name) when adding sample execution; raw core compatibility and default behavior
must be stated in slice02. Rack construction remains stopped. Start/stop gates
all module sample phases consistently; stopping does not restart RNG. Full MIDI
note cleanup belongs to arc10. Stable instance identity controls any observable
cross-instance ordering; unrelated entity allocation/order must not alter traces.

M1 uses an internal musical Clock driven by these sample times. Pull the minimal
Clock capability forward from M2: BPM plus pulses-per-quarter and pulse width;
Gate output with explicit rising edge at the documented initial sample and at
subsequent periods. TM advances once per rising edge, not every high sample.
Reject non-finite/invalid rates and configurations unable to preserve a low sample
between triggers. Default is one pulse per quarter and 10 ms width; detailed
boundary rounding is specified with golden sample traces before Clock coding.
M2 enriches this same Clock, rather than introducing another authority.

RT is initially an **output-only device service**; no independent RT-generated
clock drives TM and no spontaneous MIDI Clock bytes go out. Existing ClockMode
has no assumed output-only contract: arc10 must implement/test that mode or an
equivalent explicit command service, including prompt command draining while
stopped. Do not simply reuse a sleeping master loop and claim tight timing.
External 24-PPQN slave/master synchronization remains a retained arc08/arc10 gate:
future mapping preserves each clock event, uses integer division/phase state,
resets on Start and continues on Continue. Sample-quantized M1 makes no claim of
meeting the inherited P99 <500 microsecond hardware target.

## 3. Seed handoff and reset — arc09/slice03

Use an explicit immutable spawn context containing patch master seed, stable
instance name/ID and derived instance seed, delivered before constructing engine
state. Add a context-aware spawner path with a compatibility default for existing
metadata-only modules; avoid a mutable global seed resource that can leak across
patches or partially failed loads. Plan the exact public signature before issuing
slice03, preserving legacy spawn behavior and tests. TM uses with_seed, never
new()'s OS seed, on the patch-driven path.

Choose a documented fixed byte-level seed derivation (no DefaultHasher/AHasher
random defaults, TypeId, Entity or map iteration). Use FNV-1a 64-bit with specified
wrapping arithmetic over domain `oxurack.module-seed.v1` plus a zero byte, master
seed as 8 little-endian bytes, UTF-8 instance-name length as 8 little-endian bytes,
then name bytes; offset basis 14695981039346656037, prime 1099511628211. This is
non-cryptographic reproducibility, not collision resistance or authentication.
Golden vectors and a fresh-process executable fixture are required. The current
AHasher default uses runtime/compile-time generated keys per its locked crate
documentation, so same-process assertions cannot establish the required promise.
ModuleId::from_instance_name also uses derive_seed(0, name); include module-ID
and ordering compatibility in the slice03 audit, not just the RNG helper.
Changing derivation changes the pre-existing helper's sequence; disclose the
compatibility amendment and pin seed algorithm version for fixtures. No prior
cross-version stream guarantee is inferred. SmallRng stream compatibility is
bounded to the recorded implementation/dependency state, not all future rand versions.

Transport stop preserves state. Seed restart reconstructs engine/RNG from context
and reapplies parameters; it is not engine.reset(), which clears register/dividers
without reconstructing RNG. Mid-performance exact-state snapshots are retained
arc12 work, not required to reproduce a patch from its beginning.

## 4. TM adapter, MIDI events and lifetime — arc10

Packaging decision: add opt-in `rack` feature to the existing TM crate; expose a
separate rack adapter type wrapping the unchanged standalone engine. Default
standalone builds stay free of Bevy/core; `midi-io` remains the historical direct
wrapper and is not used by the rack adapter. No musical algorithm duplication.
Map all StepOutputs fields and all engine settings/actions in the adapter's design
inventory: notes/validity, velocity, gate, scale-note validity, pulse/gate arrays,
dividers, noise CC, register bits, length and write probability. Getters reflect
actual engine values. Arbitrary supported scale data requires explicit conversion,
not collapsing both Scale types without compatibility evidence. A first audible
subset may be released only with every remaining mapping explicitly assigned;
the existing engine's capabilities cannot disappear silently.

For M1, use typed scalar signal ports for TM output snapshots plus an explicit
one-sample step trigger. Note/velocity values use Raw with separate validity gates;
boolean outputs use Gate, write probability Float. Scope can inspect these.
Clock rate/pulse validation must leave at least one low sample between step
triggers. MIDI-out consumes a coherent snapshot on each step-trigger edge and
maintains active-note/channel state. A held Value::Midi is **not** an event stream.
General arbitrary MIDI event cables remain later design work, not a prerequisite
for this bounded TM path. Do not claim support for multiple TM steps in one sample.

MIDI-out builds an ordered batch per source step: previous note-off first, new
note-on only when gate+validity allow, then enabled CC mappings in stable order.
Retriggering the same pitch is still off/on; silence emits the pending off.
The bounded output queue accepts a whole batch or reports refusal—no partial
musical step. Order across modules is stable instance identity then local sequence.
An offline capture sink uses the same batch/sink interface as the RT adapter;
tests cover two sources, multiple messages per step and identical-pitch retriggers.
Queue overflow must latch a visible output fault, stop new note-ons and reserve a
control path for cleanup; silently dropping a note-off is not an acceptable policy.
Detailed capacity/control-lane mechanics require the arc10 first-slice design,
not CC invention inside arc09. Bounds cover absent bridge, stalled consumer and
backlog; bounded memory is required even without a device.

A single session owner holds Runtime, bridge and endpoint mapping. Startup order:
validate static patch/config, open selected endpoint, install bridge/queues, load
stopped rack, then explicit play. Failure before play must not produce notes.
Stop/panic attempts explicit note-offs plus appropriate channel cleanup before
shutdown; disconnect makes physical cleanup impossible and must report that fact,
clear local bookkeeping and require explicit recovery, not pretend successful
silence. Bounded teardown joins the runtime; do not rely only on Drop. Repeated load
and rollback promises wait for an explicit transaction design. Static RON load is
sufficient for M1; live save/cable editing/full REPL remain arc12/07 work.

## 5. Workbench and quality owners

Arc11 provides LFO phase continuity, bounded post-merge sample capture in Observe,
MIDI event capture, overflow counters, pause/reset and minimal inspection/export.
Arc07 owns the eventual interaction/display surface; rendering never drives samples.
Scope presentation can wait until capture works; no mandatory four-pane TUI.

Q-01: first arc09 slice repairs only the observed rustdoc links (TM length/write
knob and graph lib), plus docs for its new API; no behavior edits in those three
legacy doc-only files. CDC rerun found 11 diagnostics, not just the original two.
Q-02: arc08/slice02-coverage-provenance diagnoses the nine profile mismatches with
pinned toolchain/commands and isolated generated outputs; no warning suppression or
exclusion to manufacture acceptance. Q-03: arc08/slice03-coverage-hardening follows
trusted measurement, with behavior-based tests to meet retained 95% or a separately
approved disposition. It is not an excuse to write tests that mirror the code.
New implementation must have focused positive/negative tests immediately; current
93.47% is a known failing global gate, not a newly lowered target.
