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
| P-07 | TM plays through the Bevy rack and Clock/LFO/Scope make it observable | Reproduce seeded static-patch MIDI stream, witness selected software or hardware MIDI endpoint playback and inspect known signals and MIDI with bounded workbench capture | correctness-grade | Operator TM-first/workbench-second selection; CDC directive03 | open | — | Witness remains required for this row; Arc11 may proceed with the device gap explicit under directive08; P-03 hardware timing unchanged |

## Arc09 acceptance — 2026-09-18

[CDC review07](arc09-executable-rack/cdc-verification.md#cdc-repaired-candidate-acceptance--review07-2026-09-18)
accepts executable/sample/seed composition at `37deeace`; this is supporting evidence
for P-01/P-02/P-05/P-07, not their closure. All seven project rows remain open.
Arc10 now owns the planned Clock→TM→MIDI delivery and physical endpoint witness;
historical deferred rows retain successor/re-entry conditions.

## Delivery-policy amendment — 2026-10-10

Directive08, authorized by Duncan's Arc10 progress review, amends P-07's initial
endpoint to software OR hardware (was physical-only) and permits workbench work
while the witness is unavailable; it does not close P-07. P-03/P-04/P-05 retain
physical timing, final musical acceptance and release validation obligations.
These are not entry gates for M1/M2. All seven project rows remain open. Arc10's
unissued endpoint-only slice is absorbed into playback; future async output is
owned by Arc12 runtime API, re-entering when concurrent live operation needs it.

## Arc10 CRC assembly / musical witness — 2026-10-11

[CRC assembly](arc10-turing-machine-midi/crc-verification.md) at65522f1 accounts
for four accepted children/22 rows and an actual MIDI Bus1 → PatchWork/Kontakt
omni musical witness. Trace/replay/stop composition supplies P-02/P-07 support;
CDC's Arc10 gate remains pending. P-07 still requires Arc11 LFO/Scope observation;
P-03 physical timing, P-04 saved/live controls/controller acceptance, P-05 release
quality and all other criteria retain their obligations. All seven rows remain
open; no inherited deferred row or release gate is implicitly closed.

## Arc10 acceptance — 2026-10-11

[CDC verification](arc10-turing-machine-midi/cdc-verification.md) accepts five
composition rows at65522f1 and reconciles four children/22 accepted rows plus
Duncan's actual MIDI Bus1 musical witness. This supports P-01/P-02/P-05/P-07;
it does not close them. All seven project rows remain open. Arc11 now owns the
LFO/Scope workbench, starting with slice01-lfo under directive09. Other successor
and release obligations are unchanged.

## Arc11 CRC composition return — 2026-10-11

[Arc11 assembly](arc11-observation-workbench/closing-report.md) and
[CRC verification](arc11-observation-workbench/crc-verification.md) at `72fe6b5`
account for three accepted children/14 rows and fresh integrated Clock/LFO/TM/
MIDI terminal/CSV, determinism, bounds and known-signal proof. All four W rows
remain open for CDC's independent composition gate. This supplies candidate
P-07 workbench evidence alongside accepted Arc10 musical witnessing at65522f1;
P-07 and all seven project rows remain open. Arc12/13/07/08 retain their owners,
with no implicit physical timing, broad musical, quality or release acceptance.
