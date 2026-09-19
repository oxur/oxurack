# Arc09 — composition ledger

Opened before implementation, 2026-09-17. Three contributors, Expedited Mode.
The four-slice/24-row denominator is CRC-accepted at the child level, with the
integrated repaired candidate at source `37deeace`. CDC previously reproduced
E-01/E-03 on `642313a`; E-01–E-04 now await independent review of the new
candidate. CC attestation and CRC slice acceptance do not replace that gate. All
original project gates remain.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| E-01 | Installed behavior runs once per type/sample for independent module instances | Load two instances, install twice, run samples and compare counters/output traces; conflicting type cannot replace installed kind | correctness-grade | TM-01; arc05 carryover | open | [CRC reassembly](crc-verification.md#repaired-composition-reassembly--2026-09-18) | CDC reproduced on `642313a`; unchanged regressions pass on `37deeace`; CDC rechecks final candidate |
| E-02 | Manual and live drivers share defined sample and phase semantics | Compare exact traces at fixed input schedule, gate edges, merge visibility, one-sample stateful latency, paused behavior and partial stop accounting; rendering frequency cannot alter manual trace | correctness-grade | TM-02/03; arc02 carryover | open | [CRC reassembly](crc-verification.md#repaired-composition-reassembly--2026-09-18) | CDC F-01 repaired: partial return/count/time and latched phase behavior reproduce; no MIDI jitter or wall-clock exactness claim |
| E-03 | Seed reaches module construction and reproduces across fresh processes | Same patch+seed+locked build yields exact traces in two processes; changed seed/name and reordered instances are controlled checks | correctness-grade | TM-04; arc02 carryover | open | [CRC reassembly](crc-verification.md#repaired-composition-reassembly--2026-09-18) | CDC reproduced on `642313a`; seeded resume/reference and regressions pass on `37deeace`; CDC rechecks final candidate |
| E-04 | Child outcomes, regressions and inherited gaps are accounted for | Four slice ledger walks/24 rows, full relevant feature checks, strict docs and explicit successor scope comparison | serious | Framework composition; baseline findings | open | [CRC reassembly](crc-verification.md#repaired-composition-reassembly--2026-09-18) | Four slices/24 rows CRC-accepted; corrected iteration accounting and reassembly await CDC |

## CDC review06 — 2026-09-18

See [independent verdict](cdc-verification.md) and [directive06](../cdc-directive06.md).
Arc remains open; two done rows are candidate-specific, not permission to advance.

## CRC reassembly — 2026-09-18

Slice04 M-01–M-06 are independently done at `37deeace`; the four-slice/24-row
package is reassembled. All E rows are shown open for the new candidate so the
prior candidate-specific CDC results are not silently promoted. CDC must now
independently review the repaired composition before arc close or Arc10 opening.
