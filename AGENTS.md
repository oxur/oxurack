# Planning worktree

Planning lives on branch `planning`, worktree `.worktrees/planning`, under
`project01-mvp/`, per the Operator's 2026-09-17 migration request. This branch
uses filtered documentation ancestry to preserve history instead of a blank
orphan root. Do not merge it into an implementation branch.

Start with `status/project01-mvp.md` for current status, then
`project01-mvp/project-plan.md`, the implementation README on `main`,
and Git history. Legacy docs are archaeology; their stale paths and old role
instructions are preserved historical content, not current assignments.
Consult `project01-mvp/artifacts/migration-manifest.tsv` for old-to-new paths.

The selected workflow is One-Contributor: Sofie handles design, code and
self-review; Duncan is Operator. Self-checks are attested, not independent.
Keep existing acceptance/approval requirements explicit. Do not manufacture
role handoffs, CC prompts or verification records for historical work.

Plans and ledgers live at their own project/arc/slice level. Supporting outputs
go in the closest owning `artifacts/`. Historical slices exist only where an
old document was moved; create future slices when actually planned. Load the
collaboration-framework project-management guides before planning/closing work.

The Operator requested a temporary cross-project status home on 2026-09-17:
`status/projectNN-<slug>.md`, starting with `status/project01-mvp.md`. Keep current
feature completion, gaps, discoveries and deferrals there until the forthcoming
status specification supersedes this convention. This explicit status home is
outside individual project directories; it does not replace plans or ledgers.
