# Arc10 — Turing Machine MIDI in the rack

Version 1.3 — 2026-10-10. Status: open; Slice01 accepted, Slice02 correction issued by CRC.
Depends on: Arc09 at source `37deeace90e7fe2dff633ada9d58ef9051577c3d`.
Delivers: M1 playback; software composition enables Arc11 observation workbench.
Workflow: Three contributors, Expedited Mode; Duncan relays CDC/CRC/CC packets.
Authority: Operator's TM-first selection and [directive08](../cdc-directive08.md) (future work); directive07 and issued prompts remain the authority for Slices01–02 as recorded in their assignments.

## Capability and boundaries

Load a static seeded RON patch containing Clock → Turing Machine → MIDI-out,
reproduce ordered MIDI offline across fresh processes, and play it on a selected
software or hardware endpoint with explicit failure/stop cleanup. Clock is pulled forward
from the workbench because TM needs a real pulse source. M1 is sample-quantized;
it does not satisfy the retained P99 <500 microsecond release timing target.
Arc09 installation, five-phase samples and pre-construction seed context are
accepted foundations. No new simulation or independent RT clock authority.

The detailed [M1 design](design.md) refines the existing execution contract.
Slice01's exact Clock contract lives in its [plan](slice01-musical-clock/slice-plan.md).
Later slices receive full API/test recipes before issue; their directories do
not exist yet. CRC details them within this design, escalating incompatible
changes through Duncan to CDC. No scope is delegated through an empty placeholder.

## Slice sequence

| Slice | Capability / dependencies | Current state |
| --- | --- | --- |
| slice01-musical-clock | Validated standalone sample Clock plus installed Gate output, fractional timing, edit/pause semantics; consumes Arc09 | Closed; CRC accepted source `a19cfd1` after iteration01 |
| slice02-turing-machine-adapter | Opt-in rack adapter around existing seeded engine, full capability inventory, scalar snapshots and rising-edge stepping; consumes Clock | Open; [iteration01 CC assignment](slice02-turing-machine-adapter/cc-prompt-iteration01.md), proof correction after CRC review of `9ba55dd` |
| slice03-midi-output | Installed MIDI-out, bounded drain and executable seeded RON-to-MIDI trace; consumes adapter | Planned; CRC authors after Slice02 acceptance |
| slice04-output-runtime | Extend the same example to real MIDI playback, stop/error cleanup and musical trial; consumes sink | Planned; direct single-owner runner, no new RT service |

Was: three remaining slices separating sink, asynchronous service and endpoint
acceptance. Now: two executable increments; the unissued Slice05 is absorbed by
Slice04, with its replay proof pulled into Slice03. No Slice05 directory or
retrospective close packet is needed. Plan detailed APIs only for the next slice;
a coherent end-to-end change may cross crate boundaries. Do not split it merely
to give each abstraction its own slice.

## Composition, inherited work and gates

[ledger.md](ledger.md) opens five composition rows. Expected slice denominator is
four; each actual slice owns its own rows. Do not infer future row counts from
Slice01's six. Arc close needs CRC assembly and CDC independent reproduction;
real endpoint observation remains an explicit Operator gate for claiming witnessed
M1 playback. An unavailable endpoint may be explicitly deferred at CDC close
under design section 5; it does not bar Arc11 software development.

[Successor routing](../artifacts/cdc-successors03.tsv) remains intact: original
RT/public API/catalog obligations are partially addressed here, while external
clock/timing, live persistence/runtime mutation, remaining catalog, interfaces,
coverage provenance/95% hardening, benchmarks, soak and musical acceptance keep
their Arc08/12/13/07/project owners. Deferred historical rows are not auto-closed.
Arc11 reuses this Clock and adds LFO/Scope; no second clock design is implied.

## Delivery and successors

Every remaining feature slice delivers runnable behavior and an example or an
observable improvement to one. Directive08 selects checks, bounds required reading
and places implementation reasoning in CC prompts. No new documentation validator,
coverage framework or evidence-only slice is a default prerequisite.

Async output/reconnect moves to Arc12 runtime API when concurrency requires it;
hardware jitter/soak/benchmarks stay Arc08. The old precise queue/atomic/join
mechanics are retired choices, not a backlog of mandatory implementation chores.
Arc11 begins with an observable LFO in the existing runnable rack, then bounded
Scope capture and a practical inspection view. Reuse Clock and the sample trace.
Persistence, Chord, remaining modules, live controls and synchronization retain
their existing project/backlog owners after those two musical milestones.

## Version history

- v1.3 — 2026-10-10: At Operator request during Slice02 review, recalibrated
  unissued Slice03 onward for unreleased software: four slices instead of five,
  direct playback, integrated witness, and proportionate proof. Slice02 untouched.

- v1.2 — 2026-10-10: CRC reviewed Slice02 and issued test/evidence correction
  iteration01 for incomplete strict parameters, publication/lifecycle and seed
  controls. Source remains `9ba55dd`; no architecture/acceptance/sequence change.
  Slice03 stays held pending acceptance.
- v1.1 — 2026-09-19: CRC accepted Slice01 after one corrective pass and opened
  the fully inventoried Slice02 adapter packet. Arc composition rows remain open.
- v1.0 — 2026-09-18: CDC accepted Arc09's repaired composition and opened the
  existing five-slice M1 roadmap. First-slice timing/edit semantics and later
  batch/control ownership are explicit; no inherited criterion removed.
