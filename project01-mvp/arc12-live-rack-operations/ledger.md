# Arc12 — composition ledger

Opened2026-10-11 before implementation. Four planned children; four open rows.
CRC assembles composition; CDC independently accepts. No child completion inferred.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| R-01 | Current live configuration survives save and fresh reload with seeded musical behavior | Run edit/save/reload on real Clock/LFO/TM/MIDI; compare fresh equivalent configuration and unchanged saving-rack continuation | correctness-grade | Arc02 live snapshot; Operator selection | open | Slice01 CRC accepted at fabfa1cb; slice01-live-persistence/crc-verification.md | Child evidence only; full arc assembly/CDC gate pending |
| R-02 | Runtime metadata/topology changes compose with events, graph freshness and persistence | Execute live edits and rejected edits; assert notifications, immediate graph queries and reloaded patch | correctness-grade | Arcs03/04 | open | — | Reconcile all inherited child rows |
| R-03 | Public lifecycle and runtime operations preserve ownership through replacement/unload/reset | Actual running rack and MIDI owner checks; old entities retired and explicit failures; retained API/command obligations dispositioned | correctness-grade | Arc05; discovered append-only loading | open | — | No implicit async platform prerequisite |
| R-04 | Children, legacy obligations and native checks reconcile without scope loss | Four child walks, inherited successor table, fresh composed demonstration and retained/fresh checks | serious | Framework composition | open | — | Full project/release gates remain separate |

## Child advancement — 2026-10-12

Slice01 LP-01–LP-05 independently done. All four arc rows remain open until
composition and CDC review. Slice02 detailed/issued within the approved roadmap;
legacy carryover rows retain their criteria and final reconciliation owner.
