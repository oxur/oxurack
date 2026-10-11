# Arc11 — musical observation workbench

Version 1.1 — 2026-10-11. Status: open; Slice01 corrective iteration01 issued by CRC.
Depends on Arc10, independently accepted at source `65522f1`.
Authority: Operator's TM-first/workbench-second selection; directives08/09.
Three contributors, Expedited Mode. CDC owns arc/first-slice design; CRC reviews
and authors subsequent slices within this contract; Duncan relays handoffs.

## Capability and boundaries

Make the playable TM rack an inspectable experiment: add an LFO that actually
modulates it, capture bounded signal/MIDI observations, and provide a small
workbench view/export for comparing what modules produced. Reuse Clock, the
1 ms five-phase scheduler, seeded RON and direct MIDI runner. No second scheduler,
async output framework, signal-processing platform or polished GUI prerequisite.
P-07 remains open until this composition is demonstrated; release gates are later.

## Three executable slices

| Slice | Result | Depends on / state |
| --- | --- | --- |
| slice01-lfo | Standalone and installed five-waveform LFO; real RON patch modulates TM write probability and plays through the existing runner | Arc10; [open set](slice01-lfo/slice-plan.md), four L rows done, L-02 open for private input invariant repair and supplied-schedule proof |
| slice02-scope-capture | Bounded in-memory scalar/Gate/Raw observation at Observe, with sample indices; actual emitted MIDI captured without competing with the runner's drain; a runnable capture/export | LFO; planned, CRC details on accepted source |
| slice03-workbench-integration | Small usable terminal inspection view tying patch, selected signals and MIDI together; clear capture bounds, reset/new-run and export instructions | Capture; planned, CRC details on accepted source |

The first slice's exact API and numerical semantics live in its plan/prompt.
Only that slice is issued now. CRC reviewed a965386 and issued
[slice01 iteration01](slice01-lfo/cc-prompt-iteration01.md); native gates and the
modulated fixture pass, but L-02 must close before Scope is issued. Future slices
receive concrete recipes when next.
The workbench is a developer/musician inspection tool; full CLI/REPL/controller
editing remains Arc07. Do not create an evidence-only composition slice.

## Binding design for later prompt authors

Scope observes after Consume in TickPhase::Observe, indexed by SampleTime, and
never advances time or changes the sampled signals. Storage has an explicit
positive capacity and overwrites oldest records when full, reporting how many
were overwritten. Stopped racks do not generate phantom samples. Validate selected
ports/types and capture settings before play; do not silently probe missing ports.

Capture a snapshot of scalar/Gate/Raw values and actual MIDI events distinctly.
A held Value::Midi is not an event stream. The existing runner remains the sole
MIDI drain owner: record a copy when it drains, labeled generated vs successfully
sent where relevant. Do not introduce a second consumer that steals notes. CSV
export plus a compact terminal view is sufficient. Use one sample-index convention
across scalar observations and messages; wall-clock timing is not a replacement.

CRC resolves exact capture API, selector and display details within this design
before each prompt. Changes to scheduling, ownership, scope or accepted behavior
return through Duncan; private data structures and display formatting are local
choices. Reuse existing execution paths, tests and RON fixtures. A simple test
observer for Slice01 is not the public Scope implementation.

## Composition and proof

[ledger.md](ledger.md) has four arc criteria. CRC assembles three child verdicts;
CDC independently runs the integrated workbench and checks meaningful sample/MIDI
observations. No second endpoint musical witness is mandatory for an LFO-only
change; Duncan can audition each runnable increment immediately. Arc10's witness
stays at its original revision, and affected software behavior gets fresh tests.
Native gates and proportionate review follow directive08. Coverage, timing, soak,
remaining catalog/Chord, live persistence and broader interfaces retain their
Arc08/13/12/07 owners. Nothing here closes their historical deferred rows.

## Version history

- v1.1 — 2026-10-11: CRC accepted four LFO rows at a965386; one correction
  restores the binding invariant policy and completes the requested schedule
  proof. Scope remains held; arc design and parent criteria unchanged.

- v1.0 — 2026-10-11: Opened after Arc10 acceptance. Delivers LFO modulation first,
  then bounded capture and a practical view, with no parallel output runtime.
