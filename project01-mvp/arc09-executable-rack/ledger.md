# Arc09 — composition ledger

Opened before implementation, 2026-09-17. Three contributors, Expedited Mode.
CC attestation and CRC slice acceptance do not substitute for CRC arc assembly
and independent CDC composition verification. All original project gates remain.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| E-01 | Installed behavior runs once per type/sample for independent module instances | Load two instances, install twice, run samples and compare counters/output traces; conflicting type cannot replace installed kind | correctness-grade | TM-01; arc05 carryover | open | — | Includes schema/spawn/setter/getter consistency |
| E-02 | Manual and live drivers share defined sample and phase semantics | Compare exact traces at fixed input schedule, gate edges, merge visibility, one-sample stateful latency and paused behavior; rendering frequency cannot alter manual trace | correctness-grade | TM-02/03; arc02 carryover | open | — | No claim of MIDI jitter or wall-clock exactness |
| E-03 | Seed reaches module construction and reproduces across fresh processes | Same patch+seed+locked build yields exact traces in two processes; changed seed/name and reordered instances are controlled checks | correctness-grade | TM-04; arc02 carryover | open | — | Seed restart differs from engine reset and live snapshot |
| E-04 | Child outcomes, regressions and inherited gaps are accounted for | Three slice ledger walks, full relevant feature checks, strict docs and explicit successor scope comparison | serious | Framework composition; baseline findings | open | — | CRC assembles; CDC independently verifies before arc close |
