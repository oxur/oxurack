# arc03-events-and-literate-patching — ledger

Opened 2026-09-17 during retrospective migration. These are current reconciliation
and composition obligations, not reconstructed historical acceptance records.
All rows start open. One-Contributor checks will be recorded as attested; any
required independent or Operator acceptance must be attributed when it occurs.
Implementation status and commit pointers are in the project backlog.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| A-03-01 | Commands and patch loads produce consistent notifications and persisted labels | Run notification/label round-trip tests and an ordered mixed-event consumer demonstration | correctness-grade | Project roadmap; historical plans | open | — | Inspect stronger cross-type ordering need; runtime mutation remains |
| A-03-02 | Child work and deferred findings are accounted for at arc composition | Walk current child ledgers and project backlog; compare delivered scope with arc capability | serious | Framework composition discipline | open | — | Historical children remain unaccepted; future children planned only as needed |

## Restart disposition — 2026-09-17

Both rows remain open pending CRC assembly and CDC independent arc review.
Candidate disposition is `deferred` for both; evidence is insufficient for a
`done` claim. The [restart register](../arc08-validation-and-release/slice01-restart-baseline/artifacts/carryover-register.tsv)
retains each row, original capability owner, receiving R-02 and re-entry condition.
Source criteria are preserved. CRC must account for children before writing the
arc close; CDC verifies the carryover rather than signing off its own proposal.
