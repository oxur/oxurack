# Arc10 M1 design — version 1.2

2026-10-10, Sofie/CDC. Sections 1–2 retain the accepted Clock and current
Slice02 adapter contracts. Sections 3–5 replace the unissued future design under
[directive08](../cdc-directive08.md), authorized by Duncan's request to recalibrate
this unreleased project. Inspected source: `9ba55dd`; CRC repins after Slice02.
This is arc design, not an issued Slice03 assignment. CRC supplies the concrete
Rust API/recipe against the accepted predecessor before handing work to CC.

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

## 3. MIDI output and the first executable example

**Binding behavior:** install a MIDI-out module that consumes the adapter's
published step and coherent scalar inputs in Consume. A true step is one event
in that sample; held note values without a step emit nothing. Use lexical sink
instance order, then prior note-off, new note-on, enabled CCs. Same-pitch steps
retrigger off/on; a silent or invalid-note step releases the old note. Invalid
velocity uses 64; zero velocity produces no new sounding note. Validate MIDI
channel/data/config ranges rather than relying on MidiWire's masking constructors.

M1 supports one monophonic voice per sink: note/validity, velocity/validity, gate,
step, and optional noise/note/velocity CC routes (in that order). Each CC is
emitted only when its corresponding value is valid; noise is always valid.
Defaults: channel 0, all CC routes disabled. Reject CC numbers above 119 and
ambiguous duplicate enabled routes. The adapter's remaining outputs remain
available for patching and Scope; exposing a signal does not require inventing
an automatic MIDI translation for every TM field. Scale-note can use the same
note/validity inputs through ordinary cabling. Two sinks may share an endpoint
but must use distinct channels for M1, avoiding ambiguous note ownership.

**Recommended implementation:** reuse `oxurack_midi::MidiWire`, the module hook
and port conventions, and `Oxurack::advance_samples(1)`. Keep a small owned
per-sample output buffer and expose a narrow drain through the rack facade.
Preallocate for the configured static sinks (at most five messages per sink per
step); use checked capacity arithmetic. A caller must drain before advancing
another output-producing sample: detect unread output before overwriting it,
return a visible error and stop playback. No silent loss or growing history.
Do not accumulate the entire performance in memory: the example streams traces.
The exact buffer/drain API and its interaction with multi-sample advancement
are CRC's source-grounded prompt detail; unread-output detection must precede
advancing a sample that cannot be retained. Preserve the existing completed-
sample accounting contract on partial advancement.

Slice03 delivers the installed sink, actual seeded RON fixture, a runnable
headless example printing `(sample, bytes)`, and focused tests. The example
loads Clock → TM → MIDI-out and uses the real rack schedule, not a standalone
TM loop or test-only replacement. Reuse its trace path in Slice04 and Arc11.

**Was:** fixed 32-message batches, a 256-batch ring, independent atomic control
lane and acknowledgement protocol, active-note bitset and simulated consumer.
These implementation prescriptions are retired for M1. Ordered messages and
finite storage remain; atomic queue admission is no longer an M1 promise.

## 4. Direct playback and session lifetime

Slice04 extends that same example with explicit endpoint listing/selection and
live output through `midir`, using one owner on the runner thread. Synchronously
drain after each sample, send messages in order, and pace samples using monotonic
deadlines. Late work must not generate an unbounded catch-up burst: after sending
one overdue sample, schedule the next at least 1 ms later. This simple demo may
slow under load and makes no low-jitter or hard-real-time claim. Do not start an
independent master clock or emit unsolicited MIDI clock/transport messages.

Validate the patch/mapping and open the explicitly selected endpoint before
play. Selection must resolve unambiguously; no silent first-substring match.
Provide a finite duration and an interrupt/stop path. Stop sample advancement,
discard unsent performance output, then attempt explicit offs for successfully
sent active notes and CC123/CC120 on session-used channels. Repeated stop is
harmless. Track sent notes in ordinary owned state; a set or small table is
adequate, with no atomic bitset requirement. On partial send failure, stop new
notes, attempt cleanup, report the send error and any cleanup failure, and exit
unsuccessfully. A disconnected endpoint cannot be declared silent. Recovery is
an explicit new session; reconnect-in-place is outside M1.

Do not promise cleanup after forced process termination or a wall-clock timeout
on an OS MIDI call. Expose errors through Result and the runner's exit status;
Drop may release ownership but is not evidence of successful MIDI cleanup.
Use one narrow send seam to exercise send failure and cleanup ordering in tests.
No asynchronous worker, new runtime mode, control lane, join protocol or generic
backend framework is required. Existing RT modes remain available unchanged by
this design; integration with them is future work, not a prerequisite to play.

**Was:** dedicated output-only RT service, wakeable <=1 ms loop, bounded join,
quiesce/ack/recovery protocol. Deferred to Arc12 runtime API when live concurrent
operation needs it, with performance evidence from Arc08; exact old mechanics
are design candidates, not mandatory future solutions.

## 5. Evidence, musical use and limits

Slice03 proves the real installed path and exact byte/sample order. Use a small
synthetic sequence for retrigger, silence/invalid note, idle sample and CC order;
one integrated seeded patch for composition; and two fresh example processes
for equal traces. Retain Slice02's independently reviewed seed/mapping evidence
where unchanged instead of rebuilding its whole control matrix in the sink.
One fixture with reversed spawn versus lexical order proves ordering.

Slice04 proves validation/open failure, send failure after a successful note,
stop cleanup and no stale replay using the actual runner path and its send seam.
Demonstrate the example with Duncan on a named software OR hardware MIDI endpoint
as soon as the live path exists. Record command, fixture/seed, endpoint and brief
observations in the existing closing report. No separate Slice05 acceptance
packet or fresh acceptance tool is needed. An offline/mock trace cannot be called
a device witness. Physical timing, long runs and broader musical acceptance
remain Arc08/project work.

If an endpoint is unavailable, CRC records the specific witness gap in T-04 and
hands the software composition to CDC. CDC may close with that explicit deferral
to Arc08 hardware/musical acceptance and open Arc11; do not call M1 witnessed
playback complete until Duncan supplies the observation. Device availability
must not block developing the LFO/Scope workbench on verified offline output.

Validation and prompt-authoring policy are in directive08; they replace inherited
future-slice ritual gates, not Slice02's issued assignment. No ledger closes from
this design change alone.

## Version history

- v1.2 — 2026-10-10: Operator's Arc10/Slice02 progress review exposed premature
  asynchronous infrastructure. Replaced future sections 3–5 with per-sample
  output and a direct runner, merged endpoint acceptance into Slice04, and
  distinguished first use from release testing. Sections 1–2 retained.
- v1.1 — 2026-09-19: linked the current-source Slice02 inventory.
- v1.0 — 2026-09-18: CDC established the original five-slice design; superseded
  sections 3–5 remain in Git at planning `32e6934`.
