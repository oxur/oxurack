# Arc11 — musical observation workbench

Version 1.8 — 2026-10-11. Status: open for CDC composition; all three slices closed, 14/14 child rows independently accepted; CRC packet ready.
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
| slice01-lfo | Standalone and installed five-waveform LFO; real RON patch modulates TM write probability and plays through the existing runner | Arc10; [closed slice](slice01-lfo/crc-verification.md), five L rows done at c4b517c |
| slice02-scope-capture | Bounded in-memory scalar/Gate/Raw observation at Observe, with sample indices; actual emitted MIDI captured without competing with the runner's drain; a runnable capture/export | LFO; [accepted capture](slice02-scope-capture/crc-verification.md), five S rows done at `6c56e84` |
| slice03-workbench-integration | Small usable terminal inspection view tying patch, selected signals and MIDI together; clear capture bounds, reset/new-run and export instructions | Capture; [CRC acceptance](slice03-workbench-integration/crc-verification.md), all four B rows done; B-04 at `72fe6b5`, retained B-01–B-03 at `22500fa` |

The first slice's exact API and numerical semantics live in its plan/prompt.
Slice01 is independently accepted after one correction. Its native gates,
modulated fixture and supplied-schedule behavior reproduce at c4b517c.
No design, dependency or sequencing change surfaced; CRC issued Slice02 on
this accepted source. Its local details use a mixed record-capacity FIFO, exact
PatchHandle selectors, an Observe resource and caller-owned MIDI copies, with
CSV flags in the existing runner. These detail the approved capture design;
Slice02 is independently accepted at `6c56e84`, including capture/trace
equivalence and affected native gates. No structural amendment is needed;
Slice03 is issued on accepted capture with an immutable post-stop view and
unchanged CSV/MIDI output. CRC reproduced terminal/CLI/failure behavior at `22500fa` and accepted B-01–B-03.
Was: B-04 remained open because README batch continuation could latch pending MIDI.
Now: iteration01 repairs per-sample drain/copy and its executable continuation
passes independently at `72fe6b5`; all 14 child rows are done. Production contracts
are unchanged. CRC assembles the workbench below; CDC independently gates arc closure.
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

[ledger.md](ledger.md) has four arc criteria. CRC assembled three child verdicts
and freshly reproduced the integrated candidate at `72fe6b5`;
[closing report](closing-report.md) and [CRC verification](crc-verification.md)
are ready for Duncan to relay. All four W rows remain open for CDC. Previously
assembly was held for B-04; that recipe/test correction is now independently closed.

CDC independently runs the integrated workbench and checks meaningful sample/MIDI
observations. No second endpoint musical witness is mandatory for an LFO-only
change; Duncan can audition each runnable increment immediately. Arc10's witness
stays at its original revision, and affected software behavior gets fresh tests.
Native gates and proportionate review follow directive08. Coverage, timing, soak,
remaining catalog/Chord, live persistence and broader interfaces retain their
Arc08/13/12/07 owners. Nothing here closes their historical deferred rows.

## Version history

- v1.8 — 2026-10-11: CRC assembled all three accepted children and reproduced the candidate workbench at `72fe6b5`. W-01–W-04 proposed-done/open pending CDC; composition packet ready, no project closure or next-arc issue.

- v1.7 — 2026-10-11: Slice03 correction accepted at `72fe6b5`; all three children/14 rows closed. F-01 repaired in recipe/test only; no structural amendment. Arc composition ready for assembly and independent CDC gate.

- v1.6 — 2026-10-11: Slice03 initial review accepted three B rows at `22500fa`. F-01 leaves B-04 open for a documented continuation loop repair/test; native and terminal commands reproduce. No architecture or sequencing amendment; arc closure held.

- v1.5 — 2026-10-11: Issued terminal Slice03 at `6c56e84` after Scope acceptance. Private example renderer, explicit view/CSV CLI and post-stop failure handling detail the approved third increment; no architecture/parent acceptance change.

- v1.4 — 2026-10-11: Scope Slice02 accepted at `6c56e84`, all five S rows done. Runnable capture/CSV preserves musical behavior; no design amendment. Terminal Slice03 next; parent rows remain open.

- v1.3 — 2026-10-11: Issued Slice02 after LFO closure. Resolved local capture
  API/selector/record-capacity/lifecycle/CSV details within directive09; reused
  the actual runner and sole drain owner. Five S rows open; parent rows unchanged.

- v1.2 — 2026-10-11: Slice01 correction accepted at c4b517c, all five L rows
  done; Scope authoring released. Routine child status update; design unchanged.

- v1.1 — 2026-10-11: CRC accepted four LFO rows at a965386; one correction
  restores the binding invariant policy and completes the requested schedule
  proof. Scope remains held; arc design and parent criteria unchanged.

- v1.0 — 2026-10-11: Opened after Arc10 acceptance. Delivers LFO modulation first,
  then bounded capture and a practical view, with no parallel output runtime.
