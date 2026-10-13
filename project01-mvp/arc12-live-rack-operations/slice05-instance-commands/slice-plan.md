# Arc12 Slice05 — instance removal, handles and explicit commands

Version1.0 — 2026-10-13. Status: open, initial CC assignment issued.
Current assignment: `arc12-live-rack-operations/slice05-instance-commands/cc-prompt.md`.
Source baseline: `ec85ebe99c09b8a2ea64d55da0346af4ae317a67` on the assigned
implementation branch/worktree. Slice04 accepted, all six LF rows done after
three corrections. Depends on accepted Arc12 children01–04. Blocks Arc12
composition, not the existing playable instrument/workbench. Three contributors,
Expedited Mode; CDC directive12 D3/D4 and retained D1/D2 are the approved design.
Main a7ce6e6 remains protected. Six IC rows open; zero corrections executed.

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
| 2026-10-13 | arc12-live-rack-operations/slice05-instance-commands/cc-prompt.md | CRC accepted Slice04 ec85ebe/CC planning972e3b7; CDC12 D3/D4; Duncan relay/Expedited Mode | Active initial implementation; six IC rows open, no acceptance |

## Version history

- v1.0 — 2026-10-13: open final approved child after Slice04 acceptance; concrete
  removal/identity/command/cancellation/BPM/runnable scope and six IC rows issued.
