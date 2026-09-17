# slice01-turing-machine — historical reconciliation

Opened 2026-09-17 during retrospective migration. These are current reconciliation
and composition obligations, not reconstructed historical acceptance records.
All rows start open. One-Contributor checks will be recorded as attested; any
required independent or Operator acceptance must be attributed when it occurs.
Implementation status and commit pointers are in the project backlog.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S-01 | Historical specified behavior is reconciled with the current implementation | Inspect slice plan and cited commits; run the applicable current crate tests on resumption | correctness-grade | Migrated historical document | open | — | No implementation or acceptance is inferred from this migration |
| S-02 | Remaining differences and deferrals have explicit arc ownership | Compare historical scope with project backlog and owning arc plan | serious | Migration traceability | open | — | Preserve old requirements; record changed decisions before new implementation |

## Restart carryover proposal — 2026-09-17

S-01 and S-02 remain open until CRC reviews the candidate in
[closing-report.md](closing-report.md). Both proposed dispositions are `deferred`,
not delivered. CDC owns the successor design; CRC owns reconciliation through
[arc08-validation-and-release/slice01-restart-baseline](../../arc08-validation-and-release/slice01-restart-baseline/slice-plan.md), R-02.
The Operator explicitly authorized carried-over closure; no verification gate
was waived. Original criteria and historical evidence remain unchanged.
