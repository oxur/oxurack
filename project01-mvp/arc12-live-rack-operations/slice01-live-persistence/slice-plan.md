# Arc12 Slice01 — live configuration persistence

Version1.2 — 2026-10-11. Status: open; directive11 acknowledged, CRC iteration01 issued for CC implementation; all five rows await independent review.
Current issued assignment: `arc12-live-rack-operations/slice01-live-persistence/cc-prompt-iteration01.md`.
CDC author; CC implementation; CRC independent review; Duncan relays.
Initial source72fe6b52197b7a138afc3d5d7ac970ddfdf66981; current CC return
a3633bc0d0f468e933cc0f1463fb637a623b33d1, planning return9fdb2cd.
The initial prompt/return remain historical. Iteration01 is now issued, refinement
1/5; no implementation or acceptance under the new assignment is claimed.

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
CRC applied guide07 and records correction readiness below. The issued iteration
does not claim execution or acceptance. All five rows remain open.

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
| 2026-10-11 | arc12-live-rack-operations/slice01-live-persistence/cc-prompt-iteration01.md | initial prompt/CC return; directive11 resolves configured getter and logical cable order | Active CC implementation correction1/5; all LP rows open; CRC reproduction after return |

## Correction readiness — CRC 2026-10-11

1. **Source-grounded:** inspected clean source a3633bc, planning37d41f6 and the
   actual initial return. Read the TM base/set/get/consume/publication/test paths;
   core Cable, exports, patch apply/validation, cached propagation/merge and
   schedules; facade persistence/tests, shared event mutation and example/fixture.
   CRC reproduced the failed independent-reference replay and inspected the
   passing diagnostic's explicit .25→.75 assertion. Offline first event251
   confirms the pending-event252 oracle. These are author checks, not LP acceptance.
2. **Design-complete:** directive11 fixes configured getter semantics and the
   full name/port/optional-ordinal/Entity key in loader, propagation and snapshot.
   The prompt preserves effective engine/output latency, seeded reconstruction,
   active membership, typed failures and pending/Scope ownership. No unresolved
   architecture choice is assigned to CC; future cable API/replacement work stays
   in Slice03/04.
3. **Guideline-applied:** loaded engineering guide07/template, testing validation
   gates and installed Rust2.0 guidance. Verified TD-02/03 SHOULD, API-02 SHOULD,
   API-23/EH-01 MUST, AP-42/44 AVOID and AP-43 CONSIDER. The prompt maps them to
   semantic CableOrder, owned facade values, existing typed errors/private
   observations and concrete synchronous mechanics; these are applied decisions,
   not merely a request for CC to read skills.
4. **Executable:** supplied checked symbol/file sequence, illustrative getter,
   component/bundle and complete sort-key shapes, optional-query wiring and real
   Update/FixedUpdate installation. Nine exact source paths cover the repairs;
   existing examples suffice. Rust sketches are uncompiled, not a drop-in claim.
5. **Falsifiable:** held CV separates configured base from engine and pending
   outputs; facade events prove base provenance. Ordinals deliberately oppose
   Entity order while names/ports remain primary. Loaded [.75,.25] ties preserve
   literal.25 and saved vectors across two reloads, with compact fallback proof.
   Extend the original snapshot-only preservation test to include save at200/252;
   compare once-drained event251 and continuation. Independent fixture-derived
   replay remains unchanged in substance; all initial native gates rerun.
6. **Coherent/portable:** active prompt/plan/ledger agree, originals preserved;
   manifest names exact full/section reads and local skill paths, approximately
   35–50k tokens plus execution/recovery headroom. Existing closing-report/ledger
   receive concise intake, failed/partial history and final evidence. No new
   validator/evidence tree or blanket retention of CC attestation. All five rows
   and subsequent advancement remain gated by independent CRC review.

## Version history

- v1.2 — 2026-10-11: CRC acknowledged directive11 and issued one guide-authored
  correction at source a3633bc. Configured getter/logical ordering repairs and
  original save-preservation proof are specified; no criterion relaxed or accepted.

- v1.1 — 2026-10-11: CC return exposed two CDC design gaps; directive11 authorizes focused adapter/ordering repairs. Initial prompt and CC evidence preserved; CRC owns correction issuance and acceptance.

- v1.0 — 2026-10-11: CDC opened the first live-rack slice with edit/save/fresh-replay
  outcome and explicit configuration rather than continuation semantics.
