# Arc12 Slice01 — live configuration persistence

Version1.1 — 2026-10-11. Status: open; initial CC implementation returned, two design conflicts resolved by directive11; CRC correction authoring pending.
Last issued assignment (returned, not a new execution request): `arc12-live-rack-operations/slice01-live-persistence/cc-prompt.md`.
CDC author; CC implementation; CRC independent review; Duncan relays.
Initial source72fe6b52197b7a138afc3d5d7ac970ddfdf66981; current CC return
a3633bc0d0f468e933cc0f1463fb637a623b33d1, planning return9fdb2cd.
CRC must issue cc-prompt-iteration01.md before renewed CC implementation; that
file is not yet issued by this CDC directive.

## Deliverable and contract

Expose parameter reads/writes and current-configuration snapshot/save on Oxurack.
Use actual registered getters and active ECS modules/cables/labels, preserving
loaded seed/version/BPM; return existing Patch for caller-owned serialization.
A small `save-live-patch` example edits an already advancing LFO rack and saves a
patch usable immediately by `turing-machine-midi --patch ...`. It is real product
usage, not new proof machinery. Exact APIs, extraction ordering, failure semantics,
worked code shapes and discriminating tests are in the required-full prompt.

In scope: active patch extraction, parameter provenance, fallible save, scoped
identity retained on successful load, public docs and example, real round trips.
Out: engine/RNG checkpoints, in-place reload repair, schema redesign, interactive
REPL, command queues, async I/O, new dependencies, global tempo control and broad
metadata/cable mutation APIs. Those exclusions retain the arc/backlog owners.
Snapshot uses active PatchHandle membership, not every entity left by prior loads.
Saving while running is allowed between calls; it never ticks, drains or resets.

## Binding correction and initial-author readiness amendment

[Directive11](../../cdc-directive11.md) supersedes the initial prompt's effective
getter assumption and Entity-based tie-preservation recipe. The registered TM
parameter getter must return write_base; engine/output keep effective probability.
Core CableOrder records patch-vector ordinal; propagation and snapshot share the
specified name/port/ordinal/Entity fallback order. Source scope explicitly expands
to the adapter, core cable/patch/tick paths and their tests/docs. No goal, fixture,
file schema or seeded engine semantics are weakened.

CDC independently reproduced the failed fresh-replay test and the passing diagnostic
that asserts a .25→.75 tie reversal. Both remain defects to repair. The historical
readiness statement below overstated resolution of these two interactions; this
amendment corrects it rather than presenting the original investigation as complete.
CRC applies guide07 and records correction readiness at issuance; no acceptance or
new iteration execution is claimed now. All five rows remain open.

## Acceptance and evidence

Five LP rows in ledger.md; CRC reproduces actual edit/save/replay and failure
behavior under the prompt's default/no-default feature and native checks. Tests
use current modules and existing RON. No preset correction rounds; concrete
contract defects require repair, discretionary hardening is not a new gate.

CC returns closing-report.md and evidence in ledger.md (rows stay open pending
CRC), source and scoped planning commit IDs, actual failed/unrun/passing commands,
self-review and intake/readback. Concise evidence goes in these existing records.
No new durable support artifact expected; if needed, its home is this slice's
artifacts/. Source, tests, example and README live only in source worktree.

## Author readiness — CDC 2026-10-11

1. Source-grounded: inspected facade load/install, core patch data/apply/file I/O,
   registries/getters, dispatch/event provenance, cable propagation ordering,
   RackMeta and all four module integration points at72fe6b5.
2. Design-complete: prompt fixes active membership, getter failure, configuration
   restart semantics, cable ordering/ties, metadata normalization, direct mutation
   provenance, unsupported disabled cables, header commit point and file failure.
3. Guideline-applied: Rust2.0 API-02/API-23, EH-01 and AP-42/43/44 applied in the
   prompt to owned results, typed errors, borrowed names and concrete sync APIs.
4. Executable: public parameter/owned snapshot API, private extraction helper,
   shared setter emission and complete runnable example recipe; no new dependency.
5. Falsifiable: live versus stale values, default omission, incomplete getters,
   exact error/no-event paths, cable tie ordering, unchanged continuation and fresh
   saved-config replay have discriminating tests with feasible inline/public homes.
6. Coherent/portable: bounded full prompt/plan/ledger/arc/directive reading, precise
   source sections and required-data-free manifest. The packet and bounded source reads are scoped to this implementation,
   leaving recovery headroom; no legacy-plan dump.
   Sketches are uncompiled guidance. No unresolved binding design choice.

## Assignment history

| Date | Prompt | Predecessor / reason | Disposition |
| --- | --- | --- | --- |
| 2026-10-11 | arc12-live-rack-operations/slice01-live-persistence/cc-prompt.md | directive10; Arc11 CDC accepted; Operator selected live persistence | Returned at a3633bc/9fdb2cd; replay failed and tie-order diagnostic conflicts; directive11 resolves design for CRC iteration01 |

## Version history

- v1.1 — 2026-10-11: CC return exposed two CDC design gaps; directive11 authorizes focused adapter/ordering repairs. Initial prompt and CC evidence preserved; CRC owns correction issuance and acceptance.

- v1.0 — 2026-10-11: CDC opened the first live-rack slice with edit/save/fresh-replay
  outcome and explicit configuration rather than continuation semantics.
