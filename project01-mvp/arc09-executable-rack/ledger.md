# Arc09 — composition ledger

Opened before implementation, 2026-09-17. Three contributors, Expedited Mode.
The four-slice/24-row denominator is accepted. CDC independently closed
E-01–E-04 at source `37deeace` in review07. Prior candidate dispositions
remain historical below. CC attestation and CRC slice acceptance do not replace that gate. All
original project gates remain.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| E-01 | Installed behavior runs once per type/sample for independent module instances | Load two instances, install twice, run samples and compare counters/output traces; conflicting type cannot replace installed kind | correctness-grade | TM-01; arc05 carryover | done | [CDC review07](cdc-verification.md#cdc-repaired-candidate-acceptance--review07-2026-09-18) | Reproduced/reconciled at 37deeace; see final four-row walk |
| E-02 | Manual and live drivers share defined sample and phase semantics | Compare exact traces at fixed input schedule, gate edges, merge visibility, one-sample stateful latency, paused behavior and partial stop accounting; rendering frequency cannot alter manual trace | correctness-grade | TM-02/03; arc02 carryover | done | [CDC review07](cdc-verification.md#cdc-repaired-candidate-acceptance--review07-2026-09-18) | Reproduced/reconciled at 37deeace; see final four-row walk |
| E-03 | Seed reaches module construction and reproduces across fresh processes | Same patch+seed+locked build yields exact traces in two processes; changed seed/name and reordered instances are controlled checks | correctness-grade | TM-04; arc02 carryover | done | [CDC review07](cdc-verification.md#cdc-repaired-candidate-acceptance--review07-2026-09-18) | Reproduced/reconciled at 37deeace; see final four-row walk |
| E-04 | Child outcomes, regressions and inherited gaps are accounted for | Four slice ledger walks/24 rows, full relevant feature checks, strict docs and explicit successor scope comparison | serious | Framework composition; baseline findings | done | [CDC review07](cdc-verification.md#cdc-repaired-candidate-acceptance--review07-2026-09-18) | Reproduced/reconciled at 37deeace; see final four-row walk |

## CDC review06 — 2026-09-18

See [independent verdict](cdc-verification.md) and [directive06](../cdc-directive06.md).
Arc remains open; two done rows are candidate-specific, not permission to advance.

## CRC reassembly — 2026-09-18

Slice04 M-01–M-06 are independently done at `37deeace`; the four-slice/24-row
package is reassembled. All E rows are shown open for the new candidate so the
prior candidate-specific CDC results are not silently promoted. CDC must now
independently review the repaired composition before arc close or Arc10 opening.

## CDC final gate — 2026-09-18

Review07 accepts all four rows on 37deeace; four slices/24 child rows compose.
Arc09 is closed. Historical open statements above describe earlier candidates.
Project/physical/quality gates remain; Arc10 opens under directive07.
