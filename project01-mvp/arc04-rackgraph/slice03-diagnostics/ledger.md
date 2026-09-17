# slice03-diagnostics — historical reconciliation

Opened 2026-09-17 during retrospective migration. These are current reconciliation
and composition obligations, not reconstructed historical acceptance records.
All rows start open. One-Contributor checks will be recorded as attested; any
required independent or Operator acceptance must be attributed when it occurs.
Implementation status and commit pointers are in the project backlog.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S-01 | Historical specified behavior is reconciled with the current implementation | Inspect slice plan and cited commits; run the applicable current crate tests on resumption | correctness-grade | Migrated historical document | deferred | `crc-verification.md`; restart R-02 | Historical behavior not independently accepted; re-enter under CDC-assigned successor |
| S-02 | Remaining differences and deferrals have explicit arc ownership | Compare historical scope with project backlog and owning arc plan | serious | Migration traceability | deferred | `crc-verification.md`; restart R-02 | Owning arc and concrete re-entry are in CRC review; successor ID pending CDC |

## Restart carryover proposal — 2026-09-17

S-01 and S-02 remained open until CRC reviewed the candidate in
[closing-report.md](closing-report.md). Both proposed dispositions are `deferred`,
not delivered. CDC owns the successor design; CRC owns reconciliation through
[arc08-validation-and-release/slice01-restart-baseline](../../arc08-validation-and-release/slice01-restart-baseline/slice-plan.md), R-02.
The Operator explicitly authorized carried-over closure; no verification gate
was waived. Original criteria and historical evidence remain unchanged.

## CRC disposition — 2026-09-17

CRC reviewed the two-row candidate and closed this historical slice as carried
over. Both rows are deferred with reasons, domain ownership and re-entry in
`crc-verification.md`; no historical behavior was accepted as done.
