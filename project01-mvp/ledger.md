# Project01 MVP — composition ledger

Opened 2026-09-17 during retrospective migration. These are current reconciliation
and composition obligations, not reconstructed historical acceptance records.
All rows start open. One-Contributor checks will be recorded as attested; any
required independent or Operator acceptance must be attributed when it occurs.
Implementation status and commit pointers are in the project backlog.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| P-01 | All eight arc capabilities compose into a usable rack | Walk arc ledgers and demonstrate one integrated saved patch through the live interface | serious | Project DoD | open | — | No arc closure claimed during migration |
| P-02 | Seeded patches reproduce across fresh sessions | Load a representative saved patch in fresh processes and compare emitted MIDI streams | correctness-grade | Catalog phases 1/3; README | open | — | Record versions, seed and fixture; cross-version compatibility needs explicit policy |
| P-03 | Master/slave MIDI timing meets the named target | Measure P99 <500 μs on named hardware; record transport and tempo changes with Logic Pro | correctness-grade | RT plan; catalog phases 2/4 | open | — | Real MIDI/DAW evidence required |
| P-04 | Modules, persistence and live controls support real musical work | Operator records a session using saved patches, runtime edits and controller mapping | serious | README; catalog success criteria | open | — | Operator musical acceptance remains explicit |
| P-05 | Quality, documentation and resource obligations are dispositioned | Current workspace gates, inherited 95% coverage target, 10/50/100-module benchmarks, 24h soak, module examples and error review | correctness-grade | Legacy plans; catalog phase 8 | open | — | Record failures/unrun checks; do not infer release authority |
| P-06 | All legacy requirements and deferrals have current owners | Compare migration manifest, catalog phase mapping and inventory A–I disposition against plans | serious | Migration request; scope discipline | open | — | All retained; speculative extensions require an explicit scope decision |
