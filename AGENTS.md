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

The Operator selected Three-Contributor Workflow and Expedited Mode on
2026-09-17, superseding the migration's One-Contributor arrangement. Sofie in
the restart-planning conversation is CDC. CRC and CC are separate contexts
selected by Duncan (Operator); no context is implicitly created or assigned.
CDC owns architecture, arc plans and first slices; CRC reviews CC and advances
approved slices. All handoffs pass through the Operator. Read the project plan
and `project01-mvp/cdc-directive03.md` for the current assignment and gates.
Use `crc-verification.md` for new slice review, and both CRC and CDC records
for arc/project closure. Never relabel historical evidence as independent.
Expedited Mode requires explicit-path scoped commits, preserved prompts,
and immediate advancement only after applicable review/Operator gates pass.

Plans and ledgers live at their own project/arc/slice level. Supporting outputs
go in the closest owning `artifacts/`. Historical slices exist only where an
old document was moved; create future slices when actually planned. Load the
collaboration-framework project-management guides before planning/closing work.

The Operator requested a temporary cross-project status home on 2026-09-17:
`status/projectNN-<slug>.md`, starting with `status/project01-mvp.md`. Keep current
feature completion, gaps, discoveries and deferrals there until the forthcoming
status specification supersedes this convention. This explicit status home is
outside individual project directories; it does not replace plans or ledgers.
