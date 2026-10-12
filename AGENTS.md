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
and `project01-mvp/cdc-directive10.md` for current Arc12/live-persistence
work; directive08 retains the playable-first and proportionate-proof policy.
Arc10 and Arc11 are accepted; P-07 is done.
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

## CC implementation prompt authoring

Before CDC or CRC issues initial/corrective implementation instructions, read and
apply collaboration-framework's engineering-methods guide
`knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md`
and companion `knowledge/engineering-methods/templates/cc-implementation-prompt.md`.
The author inspects source and relevant domain chapters, resolves consequential
decisions, supplies a file/symbol recipe and behavioral test oracles, and records
the six readiness checks in the slice plan. Asking CC to read skills is insufficient.
Preserve issued prompts. Framework 2.6.0 and directive08 govern authoring/proof
from Arc10/Slice03 onward. Earlier directives/prompts are historical unless the
active assignment names a still-binding section. Read the current section at
the top of project-plan.md; do not treat its appended historical "current" notes
as simultaneous instructions.

This is unreleased music software. Future feature prompts authorize implementation
and related integration changes; a blanket "production code protected" rule is
inappropriate. Authors resolve strategic decisions and supply worked code shapes;
CC may adapt local mechanics within approved behavior. Reuse real execution tests,
retain unaffected independent evidence where justified, and justify any new proof
tooling. Deliver runnable musical behavior in each feature slice. Reports stay in
the existing records with concise command/results; no new process layer by default.
