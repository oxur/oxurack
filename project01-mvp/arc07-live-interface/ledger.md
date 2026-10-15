# arc07-live-interface — ledger

Opened 2026-09-17 during retrospective migration. These are current reconciliation
and composition obligations, not reconstructed historical acceptance records.
All rows start open. One-Contributor checks will be recorded as attested; any
required independent or Operator acceptance must be attributed when it occurs.
Implementation status and commit pointers are in the project backlog.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| A-07-01 | CLI, REPL and MIDI controller edits operate a persisted running patch | Demonstrate load/save/connect/set/get/watch and controller mapping against a running rack | correctness-grade | Project roadmap; historical plans | open | — | No current CLI/REPL behavior certified |
| A-07-02 | Child work and deferred findings are accounted for at arc composition | Walk current child ledgers and project backlog; compare delivered scope with arc capability | serious | Framework composition discipline | open | — | Historical children remain unaccepted; future children planned only as needed |

## Workflow transition — 2026-09-17

This arc is planned, not started or eligible for closure. Both rows remain open.
The introductory One-Contributor note is historical; the project plan now selects
three contributors and Expedited Mode.


## CDC opening — 2026-10-14

Arc12 independently accepted 27ca00f; Operator selects live controls. Arc07 v2.0
opens four planned software increments: controls, session observation, controller/
config, external transport. Both A rows remain open; Slice01's 5 LC rows open.
Closure must compose the full capability, not merely the first terminal. Historical
PatchUnloaded/common observer ordering and metadata/config residuals are retained
as explicit successor obligations in arc-plan.md/directive13. Three contributors,
Expedited Mode; CRC reviews slices and CDC independently gates the arc.

## CRC child closure / design return — 2026-10-14

Slice01's five LC rows are independently done at e582aed6e87c234799921cc66a5dd55214f76afd
([CRC verification](slice01-live-controls/crc-verification.md)); one correction
executed. This delivers terminal playback/editing/Scope/save, not full Arc07.
Both A-07-01/A-07-02 remain open: controller, session observation/metadata/completion,
external transport and complete child/residual composition remain outstanding.
[Escalation14](../crc-escalation14.md) follows directive13's explicit design return
before02 implementation. No current CC assignment or arc acceptance claimed.

## CDC design return — 2026-10-14

[Directive14](../cdc-directive14.md) resolves escalation14 for02 authoring, source
e582aed. Slice01's five rows remain done; both arc rows stay open. Runtime watch/
unload ordering and current tag/full-list note UX are 02; persistent per-note
identity/authorship/history moves explicitly to 05; real completion to 06. Existing
controller/config03 and transport04 obligations remain. CRC acknowledges and
creates02's actual open set next; no placeholder successor acceptance or CC prompt
is inferred from this planning return.
