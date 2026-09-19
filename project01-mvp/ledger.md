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

## Restart assignment — 2026-09-17

The prior One-Contributor paragraph describes the migration only. Three contributors
and Expedited Mode now apply. P-01–P-06 remain open without upgraded evidence.
[Arc08/slice01](arc08-validation-and-release/slice01-restart-baseline/slice-plan.md) supplies current baseline and reconciliation
input for P-05/P-06; it cannot close P-01–P-04 or the project. Retiring historical
work as carried over does not satisfy P-01's capability-composition criterion.

## Scope and execution amendment — 2026-09-17

P-01 still concerns all eight inherited capabilities; carried-over closure does
not satisfy it. Successor arcs09–13 plus retained arcs07/08 now own that delivery.
The explicitly selected Scope workbench adds the milestone criterion below.
No original criterion, target or verifier is removed.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| P-07 | TM plays through the Bevy rack and Clock/LFO/Scope make it observable | Reproduce seeded static-patch MIDI stream, witness selected physical endpoint playback, then inspect known signals and MIDI with bounded workbench capture | correctness-grade | Operator TM-first/workbench-second selection; CDC directive03 | open | — | M1 precedes M2; physical observation and final timing gates remain distinct |

## Arc09 acceptance — 2026-09-18

[CDC review07](arc09-executable-rack/cdc-verification.md#cdc-repaired-candidate-acceptance--review07-2026-09-18)
accepts executable/sample/seed composition at `37deeace`; this is supporting evidence
for P-01/P-02/P-05/P-07, not their closure. All seven project rows remain open.
Arc10 now owns the planned Clock→TM→MIDI delivery and physical endpoint witness;
historical deferred rows retain successor/re-entry conditions.
