# Arc09 — composition ledger

Opened before implementation, 2026-09-17. Three contributors, Expedited Mode.
All three slices are CRC-accepted and the CRC arc assembly is ready as of
2026-09-18. CDC has reproduced E-01/E-03; E-02/E-04 remain open after review06;
CC attestation and CRC slice acceptance do not replace that gate. All original
project gates remain.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| E-01 | Installed behavior runs once per type/sample for independent module instances | Load two instances, install twice, run samples and compare counters/output traces; conflicting type cannot replace installed kind | correctness-grade | TM-01; arc05 carryover | done | [CDC review](cdc-verification.md#findings-and-row-walk) | CDC reproduced at 642313a; recheck regressions on remedial candidate. Includes schema/spawn/setter/getter consistency |
| E-02 | Manual and live drivers share defined sample and phase semantics | Compare exact traces at fixed input schedule, gate edges, merge visibility, one-sample stateful latency and paused behavior; rendering frequency cannot alter manual trace | correctness-grade | TM-02/03; arc02 carryover | open | [CRC readiness](crc-verification.md#composition-row-readiness) | CDC F-01: partial sample count/time mismatch; directive06 remediation pending. No claim of MIDI jitter or wall-clock exactness |
| E-03 | Seed reaches module construction and reproduces across fresh processes | Same patch+seed+locked build yields exact traces in two processes; changed seed/name and reordered instances are controlled checks | correctness-grade | TM-04; arc02 carryover | done | [CDC review](cdc-verification.md#findings-and-row-walk) | CDC reproduced at 642313a; recheck regressions on remedial candidate. Seed restart differs from engine reset and live snapshot |
| E-04 | Child outcomes, regressions and inherited gaps are accounted for | Three slice ledger walks, full relevant feature checks, strict docs and explicit successor scope comparison | serious | Framework composition; baseline findings | open | [CRC readiness](crc-verification.md#composition-row-readiness) | CDC F-01/F-02 require remediation, corrected accounting and reassembly before close |

## CDC review06 — 2026-09-18

See [independent verdict](cdc-verification.md) and [directive06](../cdc-directive06.md).
Arc remains open; two done rows are candidate-specific, not permission to advance.
