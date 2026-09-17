# CC assignment — restart baseline, initial pass

You are CC (Code Contributor), in a separate execution context. Execute
Project01 / Arc08 / Slice01, initial assignment dated 2026-09-17.
Exact prompt: `arc08-validation-and-release/slice01-restart-baseline/cc-prompt.md`
relative to `.worktrees/planning/project01-mvp`. Predecessor: none.
Return results to CRC through Duncan; CDC is Sofie in the restart-planning context.
Three-Contributor Workflow and Expedited Mode are explicitly selected.

Read this prompt from disk and confirm it is the current assignment in
[slice-plan.md](slice-plan.md). Read that entire plan, [ledger.md](ledger.md),
[project-plan.md](../../project-plan.md), [arc-plan.md](../arc-plan.md),
[cdc-directive01.md](../../cdc-directive01.md), planning AGENTS.md and current
status. Load collaboration-framework and its required planning/verification
routes, plus testing validation gates. Load Rust guidance if source interpretation
requires it. Do not execute historical implementation instructions as assignments.

Your outcome is R-01–R-06: current validation observations, exact inherited-row
accounting, bounded source-gap findings and a reproducible close packet. No row is
already accepted. The plan contains the complete command matrix, source pin,
scoping, artifact filenames and failure policy; they are binding parts of this
assignment. Execute the matrix, preserving each attempt and its exit status.
`make check` may install tools and mask dependency failures: use the plan's
preflight/blocked handling and independent locked observations, not a fabricated
passing wrapper result. No skipped validation, weaker evidence, inferred source
scope, scope reduction/change or timeline interpretation; no Operator gate override.

Do not modify Rust, tests, manifests/lockfile, Makefile, historical slice/arc plans,
issued prompts, migration artifacts, or acceptance definitions. Do not install
global tools, modernize dependencies, push, merge, publish, operate hardware or
choose between product milestones. A baseline failure is a finding to return to
CRC; unexpected scope/design work requires CDC/Operator, not opportunistic repair.
Preserve all historical findings and any existing iteration accounting.

Durable outputs go in this slice's `artifacts/`. Create environment.md,
validation-results.tsv, baseline-findings.md, reconciliation.md and individually
named raw logs under artifacts/logs/. The existing carryover-register.tsv is the
24-row starting inventory; inspect primary documents, not just this generated list.
Preserve original obligations and qualify missing evidence. Existing imported
historical requirements remain authoritative even when a carryover is proposed.

When finished, write `closing-report.md`: identify this assignment, exact source
and planning state, commands/results (including failures/unrun attempts), walk
R-01–R-06, inventory artifacts, list exact changed files, and bubble findings up to
arc08 and the project. Do not claim independent acceptance or write CRC/CDC verdicts.
Mark CC results attested and the candidate proposed-done only when warranted.
If blocked, report the exact command/row and re-entry condition.

Commit before CRC review, with explicit paths only. Allowed planning files:
- this slice's ledger.md, slice-plan.md and closing-report.md;
- artifacts/environment.md, artifacts/validation-results.tsv,
  artifacts/baseline-findings.md, artifacts/reconciliation.md;
- artifacts/carryover-register.tsv only for in-scope, explained corrections;
- each individually named artifacts/logs/ file, listed in the report before staging.

From the planning worktree, use `git add -- <each exact project-relative file>`;
inspect `git diff --cached -- <same exact files>`; then
`git commit --only -m "Record restart baseline evidence" -- <same exact files>`.
Replace placeholders with literal filenames; never use `git add .`, `-a` or a
whole directory path. Preserve others' staged changes. Source changes are forbidden,
so there is no implementation commit. A blocked commit is reported, not hidden.

Return to Duncan for CRC relay: this exact prompt path, source SHA, planning commit,
six-row disposition summary, failed/blocked gates and CRC's next actions.
Do not close historical units yourself. CRC owns review and child carryover checks;
CRC then assembles arc packets for CDC through the Operator. The issued prompt is
immutable; corrections require cc-prompt-iteration01.md or the next unused suffix.
