# Restart baseline — acceptance ledger

Opened 2026-09-17, before CC execution. Six rows. Three-Contributor Workflow,
Expedited Mode. CC supplies attested evidence; rows remain proposed until CRC
independently checks them. Use open/done/deferred/no-op, with a reason and concrete
re-entry for deferrals. Baseline observation acceptance does not accept product
behavior, retire unresolved quality gates or authorize release.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| R-01 | Source/tool state and every required validation observation are reproducible | Inspect environment.md, validation-results.tsv and raw logs; rerun all runnable commands; account for each matrix cell | correctness-grade | Current source and Makefile | open | `artifacts/environment.md`, `artifacts/validation-results.tsv`, raw logs 01–61 (attested) | CRC must rerun runnable cells; docs, coverage and dependency findings remain open |
| R-02 | All 24 inherited rows have exact traceability, ownership and re-entry without a completion upgrade | Run verify-planning.py; compare all six slice and six arc ledgers to carryover-register.tsv and reconciliation.md; inspect original requirements and unresolved deltas | correctness-grade | `artifacts/reconciliation.md`, `artifacts/logs/36-verify-planning.log`, `artifacts/logs/61-verify-planning-final.log` (attested) | All 24 rows remain open; CRC must compare source ledgers and candidate reports |
| R-03 | Known runtime/module/introspection gaps are checked against pinned source | Read cited symbols and tests in baseline-findings.md; reproduce each bounded search and distinguish stubs from operational behavior | serious | Current status discoveries | open | `artifacts/baseline-findings.md`, source logs 41–53 (attested) | TM-01–TM-08 and WB-01–WB-03 are observations and re-entry blockers, not implementation approval |
| R-04 | Failures, ignored tests, coverage limits and hardware/soak obligations remain explicit | Compare every failed/blocked/not-run result to owner and re-entry in baseline-findings.md; preserve 95 percent target, P99 target, benchmarks and 24h soak | correctness-grade | Project P-03/P-05 and arc08 acceptance | open | `artifacts/baseline-findings.md`, `artifacts/validation-results.tsv`, logs 29–35 (attested) | CRC must verify docs/MIDI coverage failures and retained hardware, benchmark, soak and musical gates |
| R-05 | Planning changes preserve imported history and carryover scope | Run verify-planning.py; git diff --check; inspect explicit file diff and preserved prompts; verify no implementation or dependency edits | serious | Migration preservation and Expedited Mode | open | `artifacts/logs/36-verify-planning.log`, `artifacts/logs/37-planning-diff-check.log`, logs 58–61, environment state (attested) | No implementation or dependency files were changed; CRC review pending |
| R-06 | Complete close packet enables independent review and design follow-up | Walk all six rows in closing-report.md; inventory every artifact/log and commit; compare bubble-up to arc08 plan and flag milestone decision for CDC/Operator | serious | Framework slice-close contract | open | `closing-report.md` (attested) | CC returns the packet to CRC through Duncan; CDC/Operator retain milestone authority |

## R-03 clarification — 2026-09-17

Operator priority amendment and directive02 add the source-inspection targets
TM-01–TM-08 and WB-01–WB-03 from artifacts/turing-machine-dependency-map.md to R-03.
The criterion remains observation of known gaps, not approval or implementation
of every proposed contract. Six-row denominator unchanged; all rows still open.
