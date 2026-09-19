# Arc10 M1 design — version 1.1

2026-09-18, CDC. Binding elaboration within Operator-selected TM-first scope.
Baseline `37deeace`; predecessor: execution-contract.md sections 2–5 under
Arc09/Slice01/artifacts. Clock is the only current implementation assignment.
The other sections constrain later CRC-authored slices, not permission for CC
to implement them now. API-level detail must be checked against current source
when each future slice opens. Incompatible changes return to CDC through Duncan.

## 1. Clock and sample ownership

One sample is 1 ms. The standalone Clock advances exactly one sample per call;
its rack adapter executes once in the supplied schedule's Produce phase.
The complete numeric, reset/edit and Gate contract is in Slice01's plan. Core
latched running controls execution: modules must not add per-phase running
checks that truncate a sample. Clock Gate is current-sample visible to TM
Consume, whose new snapshot is published on the following sample.

Clock's `bpm` parameter owns its rate (default 120). Patch.bpm is existing
metadata, validated by core but not passed to ModuleSpawnContext or a tempo
resource. M1 fixtures set Clock bpm explicitly equal to their patch metadata;
no automatic inference, global SetBpm implementation or live tempo bridge is
claimed. Multiple Clock instances may deliberately use different rates.

## 2. TM inventory and integration boundary

Use existing `oxurack-mod-turingmachine` with an opt-in `rack` feature; default
build remains standalone. Expose a separate adapter wrapping the existing
TuringMachine, initialized with context.instance_seed() through with_seed.
Do not reuse midi-io, OS randomness or duplicate the algorithm. Consume detects
false→true on a Gate clock input, treating initial previous value as false.
At most one engine tick per sample; never tick every high sample. Publish all
fields of that completed step as a coherent next-Produce snapshot plus a
one-sample step trigger; clear trigger on the next Produce without clearing
held snapshot fields. A later sink sees the snapshot and its trigger together.

Before issuing Slice02, CRC writes an explicit mapping table for every actual
StepOutputs field and every engine setting/action. Baseline outputs: note,
velocity, gate, scale_note, pulses[6], gates[8], div2/div4, noise_cc,
register_bits, length, write_probability. Settings/actions: write and modulation,
length/position, root/scale/range, separate scale-output scale/root, reset,
move_step and tick, with getters/current-state introspection. Notes/velocity/raw
bits use Raw and validity Gates; booleans Gate; write probability Float. The
arbitrary TM/core Scale conversion must preserve supported intervals and roots.
Getters read actual engine state. Each mapping must be delivered or assigned an
explicit successor/re-entry; no silent first-audible-subset scope reduction.
Stop preserves RNG/register/edge history; seed restart reconstructs the engine,
while reset is the existing register/divider reset and is not seed restart.
The required actual-source inventory and complete mapping are now resolved in
[Slice02's plan](slice02-turing-machine-adapter/slice-plan.md); that packet is
the binding detail for adapter parameters, 32 ports, Scale conversion, action
priority, modulation and one-sample publication.

## 3. MIDI batches, bounded storage and failure

Use an owned, fixed-capacity batch as the unit of admission and RT transfer,
never multiple independent EcsCommand::SendMidi pushes advertised as atomic.
M1 batch capacity is 32 MidiWire messages (bounded note off/on plus explicit
CC inventory); reject a mapping exceeding this bound before play, never truncate.
A batch belongs to one endpoint and one source step and carries stable module
identity plus local order. Sink processes sources by lexical instance name, then
message sequence: prior note-off, permitted new note-on, enabled CCs in defined
mapping order. Same-pitch retrigger is off/on; silent step releases prior note.
No held Value::Midi is treated as an event stream.

Normal output ring capacity is 256 whole batches per session (one endpoint for
M1). Offline capture uses the same admission interface and capacity/refusal
semantics, with an explicitly drained bounded consumer; tests may choose smaller
capacities to force pressure. No bridge means a visible unavailable fault, not
an unbounded backlog. RT consumers may pop bounded batches sequentially; physical
send failure can occur within a batch, so atomic admission never claims atomic
wire delivery. Report it and enter cleanup/recovery.

Use a distinct coalescing control lane independent of normal-ring capacity:
shared atomic cleanup-request and shutdown-request flags, plus acknowledgement
state. One outstanding cleanup is sufficient; repeated stop/panic requests are
idempotent. Cleanup is processed before another normal batch and discards queued
stale performance batches while play is quiesced. The consumer owns a fixed
16×128 active-note bitset for the single endpoint; mark a note only after a
successful send. Cleanup sends explicit offs for tracked notes then CC123 and
CC120 on channels used by this session. The offline consumer models the same
state transitions. A normal-ring refusal latches fault and requests cleanup;
producer emits no further note-ons until explicit recovery, and clears its local
pending/active bookkeeping. Do not spin or block the sample thread for capacity.

Lifecycle protocol uses play-quiesced handshake: producer stops admission,
requests cleanup, consumer drains/discards and acknowledges, then explicit
recovery may reset faults and resume. No concurrent re-enable while cleanup is
in flight; no need for speculative epoch protocols in M1. Runtime failure or
acknowledgement timeout cannot be relabeled successful silence. CRC must give
exact Rust types, atomic memory-order/ownership rationale and discriminating
mock-consumer tests in Slice03/04; escalate if current APIs cannot support this
contract without a scope change. Preserve existing RT modes and tests.

## 4. Output-only runtime and session lifetime

Existing RT master loop generates 0xF8; it is not an output-only service. Slice04
adds an explicit output-only mode/path that drains commands while rack samples
are stopped and emits no unsolicited clock, transport or notes. It must not wait
for a master musical tick to process cleanup. Recommended dedicated loop checks
control, handles batches, then uses a wakeable bounded idle wait (at most 1 ms),
without claiming OS scheduling latency. Keep shutdown's existing out-of-band
flag; provide completion acknowledgement and a bounded join attempt. Timeout is
an explicit shutdown failure with retained ownership/recovery, not a hidden
blocking Drop. Never assert a hard wall-clock guarantee from this loop design.

A single session owner owns runtime/endpoint/bridge. Validate static configuration
and mapping, open endpoint, establish queues, load a stopped rack, then explicit
play. Failed setup emits no notes and cleans acquired resources. Stop/panic
quiesces admission and attempts cleanup; disconnect reports cleanup unconfirmed,
clears local bookkeeping and requires explicit reconnect/recovery. Replacement
load/live save/full transactional rollback remain Arc12, not implied by static
loading. Endpoint routing in M1 is one explicitly selected physical output;
multi-endpoint configuration remains visible future work, not guessed indices.

## 5. Acceptance and retained gates

Offline deterministic Clock→TM→sink fixture must reproduce exact ordered bytes
in two fresh processes, with changed seed and spawn-order controls. Include a
synthetic sink fixture forcing same-pitch retrigger, silence, CC ordering, full
queue, absent bridge, disconnect and cleanup-before-resume. Real device playback
requires a named endpoint, host/toolchain/seed/patch, witness observations and
explicit Operator acceptance; mock success cannot close that row. No endpoint
is selected or opened by this planning assignment. Final MIDI sync P99 timing,
95% coverage, resource/soak and broader musical acceptance remain project gates.

## Version history

- v1.1 — 2026-09-19: linked the current-source Slice02 inventory that fulfills
  the pre-issue mapping gate; no MIDI/runtime or acceptance contract changed.
- v1.0 — 2026-09-18: CDC established the five-slice M1 architecture and bounds.
