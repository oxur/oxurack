# Arc12 — composition ledger

Opened2026-10-11 before implementation. Five accepted children/27 rows as of
2026-10-14, final source27ca00f. CRC composition packet assembled; all four arc
rows remain open for independent CDC verification and historical disposition.
See closing-report.md and crc-verification.md; no CDC verdict inferred.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| R-01 | Current live configuration survives save and fresh reload with seeded musical behavior | Run edit/save/reload on real Clock/LFO/TM/MIDI; compare fresh equivalent configuration and unchanged saving-rack continuation | correctness-grade | Arc02 live snapshot; Operator selection | open | CRC operational reproduction at27ca00f,2026-10-14; closing-report.md and crc-verification.md; accepted child ledgers | Current composed behavior reproduced; independent CDC acceptance pending, row remains open. |
| R-02 | Runtime metadata/topology changes compose with events, graph freshness and persistence | Execute live edits and rejected edits; assert notifications, immediate graph queries and reloaded patch | correctness-grade | Arcs03/04 | open | CRC operational reproduction at27ca00f,2026-10-14; closing-report.md and crc-verification.md; accepted child ledgers | Current composed behavior reproduced; independent CDC acceptance pending, row remains open. |
| R-03 | Public lifecycle and runtime operations preserve ownership through replacement/unload/reset | Actual running rack and MIDI owner checks; old entities retired and explicit failures; retained API/command obligations dispositioned | correctness-grade | Arc05; discovered append-only loading | open | CRC operational reproduction at27ca00f,2026-10-14; closing-report.md and crc-verification.md; accepted child ledgers | Current composed behavior reproduced; independent CDC acceptance pending, row remains open. |
| R-04 | Children, legacy obligations and native checks reconcile without scope loss | Five child walks, inherited successor table, fresh composed demonstration and retained/fresh checks | serious | Framework composition | open | CRC operational reproduction at27ca00f,2026-10-14; closing-report.md and crc-verification.md; accepted child ledgers | Five children/27 rows and eighteen inherited rows accounted; original notification/future-feature dispositions require CDC gate. |

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


CDC directive12 — 2026-10-12: escalation resolved, no row accepted. R-03 now
composes lifecycle04 and instance-command05; R-04 walks five children and explicit
Arc07 separate-transport / Arc13 typed-vocabulary dispositions. CRC authors04;
source remains deafdd228. See ../cdc-directive12.md for design and proof. Historical
Arc05 A-05-01/02 remain deferred, not discharged by this routing amendment.


CRC directive12 acknowledgement / Slice04 issuance — 2026-10-12: clean source
deafdd228 and CDC planning67e9150 inspected. Approved Slice04 open set issued in
slice04-runtime-api with six LF rows, all open, plus exact source/read/test scope.
Failure-preserving lifetimes and bounded single-owner MIDI cleanup are assigned
implementation, not delivered/accepted. R-01–R-04 remain open; Slice05 stays
unopened pending04 acceptance. Historical deferred rows and Arc07/13 routes retain
the directive's re-entry obligations. No source or structural design change by CRC.


CRC Slice04 iteration01 review — 2026-10-12: source1242a60/planning9081268,
fresh native gates and exact host retry/musical72-row replay pass. LF-05/LF-06
independently done; LF-01–LF-04 open for remaining assigned interaction checks.
Iteration02 issued, one correction executed/two issued. R-01–R-04 all remain
open; no parent/legacy acceptance or Slice05 opening. See child CRC review.


CRC Slice04 iteration02 review — 2026-10-12: source03825e5/planning18abe11,
fresh native gates and exact host retry pass; LF-01/LF-02 accepted, LF-05/LF-06
retained, LF-03/LF-04 open. Collected iteration03 test/evidence repair issued;
two corrections executed, third issued. R-01–R-04 remain open, Slice05 held;
no parent/legacy acceptance or structural amendment. See child CRC review.

CRC Slice04 closure / Slice05 issuance — 2026-10-13: source ec85ebe, CC planning
972e3b7. Six LF rows done after3/5 corrections; fresh affected gates/host retry pass,
restricted failures and historical musical/docs evidence preserved. Final approved
child slice05-instance-commands opens six IC rows for removal/handles/commands/BPM
and actual session. R-01–R-04 remain open; CDC12 design unchanged. No inherited,
project/release acceptance or source/main changes by CRC.

CRC Slice05 initial review — 2026-10-13: source7404fe2/CCa3f350e. Four IC rows
independently done, IC-04/06 open with correction01 issued. Native gates/host retry
and ordinary session/fresh/no-graph/fixture hashes pass; exact public queue API,
example failure/cleanup/docs and immediate reset-failure evidence need correction.
R-01–R-04 remain open; five-child composition/CDC acceptance not yet assembled.
See child crc-verification.md. No architectural or inherited-scope amendment.

CRC final-child/composition return — 2026-10-14: Slice05 iteration01 accepted
at27ca00f/CC818d146; all27 child rows done. Current runtime composition and native
host/feature/recipe evidence reproduced; no new product defect. closing-report.md
and crc-verification.md account for all eighteen inherited Arc02–05 rows and
remaining historical questions. R-01–R-04 stay open until CDC verifies the same
candidate and dispositions residuals. No cdc-verification.md authored by CRC;
no new arc/project/release acceptance or source/main change.
