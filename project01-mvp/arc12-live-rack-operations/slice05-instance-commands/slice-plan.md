# Arc12 Slice05 — instance removal, handles and explicit commands

Version1.1 — 2026-10-13. Status: open, CRC initial review/correction.
Current assignment: `arc12-live-rack-operations/slice05-instance-commands/cc-prompt-iteration01.md`.
Current source baseline: `7404fe227e561746dbda76543b59893feeacfe15` on the assigned
implementation branch/worktree. Slice04 accepted, all six LF rows done after
three corrections. Depends on accepted Arc12 children01–04. Blocks Arc12
composition, not the existing playable instrument/workbench. Three contributors,
Expedited Mode; CDC directive12 D3/D4 and retained D1/D2 are the approved design.
Main a7ce6e6 remains protected. Six IC rows: IC-01/02/03/05 independently done;
IC-04/06 open. Iteration01 issued, first correction1/5; zero corrections executed.

## Delivered capability and design

Remove one stopped/drained instance while preserving surviving engines, time,
metadata, Scope and fault; retire incident topology and owned children, enqueue
only selected MIDI cleanup and release the sink reservation. Public generic
handles and named refs validate rack lifetime/instance identity before use.
Execute all existing commands synchronously or through an explicit bounded FIFO
with typed, exactly-once results; reset cancellation is observable. Persist global
BPM independently of audible Clock parameter. Deliver a runnable rack-session
example using handles/removal/queued persistence and seeded fresh replay.

The prompt's sections1–5 supply the concrete API, implementation sequence,
file/symbol wiring, mutation boundaries, literal tests and executable recipes.
Those details instantiate unchanged CDC12, not another architecture. Binding:
opaque private Arc identities, active-membership/type validation, selected silence
before infallible removal commit, queue64 total outstanding including cancellations,
execution-time resolution and no sample acknowledgment. Private helper organization
is discretionary inside the exact prompt fence. The core loader remains unchanged
in schema/validation; no phase checkpoint or mutable public World is introduced.

## Scope, verification and retained ownership

Production and tests across facade ownership/persistence/provenance/Scope seams,
new handles.rs/commands.rs, core hook/topology/index/event/error/export wiring,
MIDI-out selected cleanup/release and graph incoming/outgoing/name-map maintenance.
New public integration tests and writer-based rack-session example; README commands.
Exact authorized paths and separate explicit-path commit instructions are in the
prompt; no inferred source scope. New dependencies/manifest/lockfile/schema changes,
standalone engines, native RT/transport, old fixture/example edits and main are out.

Six IC rows cover removal, identities, command/BPM semantics, FIFO/cancellation,
selected MIDI cleanup and runnable integration/native validation. Use real fixtures,
private inline tests only for hidden state, native core/MIDI-out/graph and facade
all-targets in both feature modes, make test/lint, strict all-target Clippy/docs,
example builds and default/no-graph/fresh saved-patch replay. Preserve restricted
CoreMIDI failures and exact host retries; no warning suppression/new ignores.
No new validator, coverage campaign or hardware gate. Artifacts: none expected;
transient outputs under /tmp. If a meaningful durable failure receipt is needed,
its default home is this slice's artifacts/, with reason and inventory in the
closing report. No empty supporting directory or prefilled review/close verdict.

A-05-01's removal/handles/queued-operation obligations advance here; historical
transport/power aliases and wire START/STOP/CONTINUE remain Arc07, re-entering before
external transport interfaces. Module-specific Port/Knob/value enums and historical
construction/convenience forms remain Arc13, re-entering with public catalog
vocabulary; generic handles alone do not fulfill that entire historical API.
A-05-02 requires inherited reconciliation and final arc composition. Arc12
R-01–R-04 and projectP-01–P-06/release obligations remain open; accepted P-07 and
child evidence keep their original scope/provenance. CDC retains arc acceptance.

## Exit and records

CC records pre-edit itemized intake/readback, applied Rust guidance, exact attempts/
results, all six proposed dispositions, artifact inventory and bubble-up in
closing-report.md/ledger.md. Its evidence is attested; rows remain open until CRC
reproduces the behavior. CRC records independent review in crc-verification.md.
No Slice05 or parent closure merely from passing structural checks or CC claims.
After acceptance CRC assembles Arc12's five children/composition and inherited
walk for CDC's independent gate; CRC does not author a new arc unilaterally.

## Author readiness — Sofie/CRC, 2026-10-13

1. Source-grounded: inspected ec85ebe App/constructor/installation/commit, lifecycle
   admission/reset, live persistence/provenance, actual ModuleRegistry side-map and
   cable mutation/index, MIDI buffer/channel/silence and graph handlers. Bevy0.18.1
   ChildOf linked-spawn and World::despawn, petgraph0.7.1 edges_directed checked
   against Cargo.lock/local sources; unrelated installed0.20 source was discarded.
2. Design-complete: CDC12's removed-instance/lifetime/queue/global-BPM contracts
   instantiated as exact public refs/Handle/result types and methods, typed failures,
   all-fallible-preflight/cleanup followed by commit, private token rotation and
   preservation, Pending/Completed FIFO/reset-tail interaction. No held design choice.
3. Guideline-applied: guide07/template and PM close/open/row-closure discipline;
   actual Rust API-02/07/23, EH-01/14, TD-02/03/05/06/07/17 read/applied to owned
   snapshots, private identity, named recoverable errors, hook panic exclusion,
   generic marker and enum queue capacity. Testing proportionate/native proof
   governs the six-row mapping. No runtime/dependency imported from skill defaults.
4. Executable: concrete core prepare/commit seam shares entity disconnect commits,
   erased hook copied before World mutation, selected MIDI append then release;
   facade tokens initialized/swapped only on success; mutable borrows end before
   shared helpers. Queue pop precedes Panic so only pending tail cancels. Illustrative
   sketches are explicitly uncompiled; existing native fixture/helper APIs checked.
5. Falsifiable: both-direction/self-edge and guarded same-name graph tests, exact
   sample300 selected cleanup, survivor/control/Scope continuity, failed-operation
   identity preservation, wrong Rust type/cross-rack/stale refs; queue64/65/ID overflow,
   cancelled-capacity/results exactly once, real FIFO load/edit/save/error recovery;
   global133 versus Clock120; runnable saved/fresh/default-no-graph comparison.
6. Coherent/portable: prompt carries bounded ordered text manifest/current scope,
   pre-edit intake home/readback and65–90k budget including recovery; original04
   acceptance and required inherited-owner distinctions are explicit. Exact source
   allowlist/commits, six open rows, fresh affected gates and retained historical
   claims agree across prompt/plan/ledger. No ceremony/proof platform or endpoint.

## Assignment history

| Date | Prompt | Predecessor / authority | Disposition |
| --- | --- | --- | --- |
| 2026-10-13 | arc12-live-rack-operations/slice05-instance-commands/cc-prompt.md | CRC accepted Slice04 ec85ebe/CC planning972e3b7; CDC12 D3/D4; Duncan relay/Expedited Mode | Returned7404fe2/a3f350e; CRC four rows done/two open, superseded by iteration01; no slice acceptance |
| 2026-10-13 | arc12-live-rack-operations/slice05-instance-commands/cc-prompt-iteration01.md | CRC initial review F-01–F-04 at7404fe2; initial prompt preserved | Active first correction1/5; exact queue API, bounded session fixes/docs and immediate failure observation |

## Initial CRC review and iteration01 readiness — 2026-10-13

Four behavior rows independently done at7404fe2. IC-04/06 remain open for the
specific mismatches in crc-verification.md: exact public queue API/opaque IDs,
private queue states, predictable missing-load input, final cleanup recipe,
immediate failed-reset observation and required new API docs. Native gates,
host retry, fixture hashes and ordinary session/fresh/no-graph replay all pass.
No architecture or scope amendment, no new composition acceptance or source
repair by CRC. CC's initial report remains preserved; artifacts none.

Six author checks for iteration01:

1. Source-grounded: full actual delta/new modules/runner/tests inspected at7404fe2;
   external required-API compile/forged-ID witnesses and valid existing sibling
   reproduce failures; actual300-sample held final note/status inspected. Corrected
   CRC witness used actual baseline cable count after an initial bad assumption.
2. Design-complete: original exact API restored, not redesigned; private slot
   enum preserves pop-before-Panic/cancelled capacity; reserved temporary missing
   directory never overwrites siblings; final reset/drain after ordinary comparison
   records subject/reference cleanup only to status/Scope. No open design question.
3. Guideline-applied: guide07/template and actual Rust API-02/07/23, EH-01/14,
   TD-03/05/06/07 applied to owned typed outcomes, private IDs, explicit slot states
   and useful failure docs. Native proportionate proof; no new validator platform.
4. Executable: sections F-01–F-04 specify exact files/symbols, consumer/export
   migration, worked Rust queue/cleanup bodies, atomic directory reservation,
   public doctests and an inline immediate invalid-world observation without
   snapshotting a dead sink. Ten named files, doc-only fences where appropriate.
5. Falsifiable: required-API positive compile and opaque/non-exhaustive negative
   doctests; valid pre-existing sibling cannot alter outcomes/bytes; exact final
   sample300 cleanup, nonempty stdout retained; failed-reset projection captured
   immediately before/after Err before collection. Existing accepted checks stay.
6. Coherent/portable: full current prompt/plan/ledger/review plus bounded authority
   and source manifest, itemized pre-edit intake/readback and45–65k budget. Initial
   prompt immutable; four accepted rows dated and proposed for retention, affected
   facade/docs/example/native recipes fresh. Zero executed/one issued correction;
   Slice05 and CDC arc gate held. Only routine planning/status updates.

## Version history

- v1.1 — 2026-10-13: CRC accepts four rows at7404fe2, keeps IC-04/06 open and
  issues bounded iteration01 F-01–F-04 under unchanged CDC12.

- v1.0 — 2026-10-13: open final approved child after Slice04 acceptance; concrete
  removal/identity/command/cancellation/BPM/runnable scope and six IC rows issued.
