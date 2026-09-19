# Arc09 — composition ledger

Opened before implementation, 2026-09-17. Three contributors, Expedited Mode.
All three slices are CRC-accepted and the CRC arc assembly is ready as of
2026-09-18. E-01–E-04 remain open for independent CDC composition verification;
CC attestation and CRC slice acceptance do not replace that gate. All original
project gates remain.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| E-01 | Installed behavior runs once per type/sample for independent module instances | Load two instances, install twice, run samples and compare counters/output traces; conflicting type cannot replace installed kind | correctness-grade | TM-01; arc05 carryover | open | [CRC readiness](crc-verification.md#composition-row-readiness) | CRC reproduced; CDC composition gate pending. Includes schema/spawn/setter/getter consistency |
| E-02 | Manual and live drivers share defined sample and phase semantics | Compare exact traces at fixed input schedule, gate edges, merge visibility, one-sample stateful latency and paused behavior; rendering frequency cannot alter manual trace | correctness-grade | TM-02/03; arc02 carryover | open | [CRC readiness](crc-verification.md#composition-row-readiness) | CRC reproduced; CDC composition gate pending. No claim of MIDI jitter or wall-clock exactness |
| E-03 | Seed reaches module construction and reproduces across fresh processes | Same patch+seed+locked build yields exact traces in two processes; changed seed/name and reordered instances are controlled checks | correctness-grade | TM-04; arc02 carryover | open | [CRC readiness](crc-verification.md#composition-row-readiness) | CRC reproduced; CDC composition gate pending. Seed restart differs from engine reset and live snapshot |
| E-04 | Child outcomes, regressions and inherited gaps are accounted for | Three slice ledger walks, full relevant feature checks, strict docs and explicit successor scope comparison | serious | Framework composition; baseline findings | open | [CRC readiness](crc-verification.md#composition-row-readiness) | CRC assembly complete; CDC independently verifies before arc close |
