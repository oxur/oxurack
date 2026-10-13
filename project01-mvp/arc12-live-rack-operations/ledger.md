# Arc12 — composition ledger

Opened2026-10-11 before implementation. Four planned children; four open rows.
CRC assembles composition; CDC independently accepts. No child completion inferred.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| R-01 | Current live configuration survives save and fresh reload with seeded musical behavior | Run edit/save/reload on real Clock/LFO/TM/MIDI; compare fresh equivalent configuration and unchanged saving-rack continuation | correctness-grade | Arc02 live snapshot; Operator selection | open | Slice01 CRC accepted at fabfa1cb; slice01-live-persistence/crc-verification.md | Child evidence only; full arc assembly/CDC gate pending |
| R-02 | Runtime metadata/topology changes compose with events, graph freshness and persistence | Execute live edits and rejected edits; assert notifications, immediate graph queries and reloaded patch | correctness-grade | Arcs03/04 | open | Slice02 five M rows CRC accepted at20c60ef; Slice03 five G rows CRC accepted atdeafdd228; respective crc-verification.md records | Reconcile all inherited child rows |
| R-03 | Public lifecycle and runtime operations preserve ownership through replacement/unload/reset | Actual running rack and MIDI owner checks; old entities retired and explicit failures; retained API/command obligations dispositioned | correctness-grade | Arc05; discovered append-only loading | open | — | No implicit async platform prerequisite |
| R-04 | Children, legacy obligations and native checks reconcile without scope loss | Four child walks, inherited successor table, fresh composed demonstration and retained/fresh checks | serious | Framework composition | open | — | Full project/release gates remain separate |

## Child advancement — 2026-10-12

Slice01 LP-01–LP-05 independently done. All four arc rows remain open until
composition and CDC review. Slice02 detailed/issued within the approved roadmap;
legacy carryover rows retain their criteria and final reconciliation owner.

Slice02 M-01–M-05 independently done at20c60ef. Child evidence contributes to
R-02; no arc/legacy acceptance is inferred. Runtime topology and Slice04 remain.

Slice03 runtime cable/graph prompt issued on accepted20c60ef with five G rows
open. It supplies topology composition; Slice04/final CDC assembly remain held.

Slice03 return981964b/544b89d independently checked2026-10-12: native tests,
host retry and actual300-sample/82-row fresh replay pass. All five G rows stay open
for F-01–F-04 required interaction/reconciliation proof in iteration01. No parent
acceptance, scope change or Slice04 opening; see Slice03 crc-verification.md.


Slice03 iteration01 independently accepted2026-10-12 atdeafdd228: five G rows done,
new interaction checks pass in native core/graph/both facade modes and host workspace
retry; unchanged musical/docs/build evidence retained at981964b with original date.
All four arc rows stay open. Slice04 lifecycle contract decisions await CDC in
project-root crc-escalation12.md; no parent or historical reconciliation acceptance.
