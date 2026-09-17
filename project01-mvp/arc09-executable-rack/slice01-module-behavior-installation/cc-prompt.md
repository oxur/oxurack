# CC implementation assignment — module behavior installation

Project01 / Arc09 / Slice01, initial pass, 2026-09-17. Exact path relative to
project01-mvp: `arc09-executable-rack/slice01-module-behavior-installation/cc-prompt.md`.
No predecessor. You are CC, separate from CDC/Sofie and CRC. Return to CRC through
Duncan. Three contributors and Expedited Mode apply.

Read the full [slice plan](slice-plan.md), [ledger](ledger.md),
[arc plan](../arc-plan.md), [binding design](artifacts/execution-contract.md),
[project plan](../../project-plan.md), [directive03](../../cdc-directive03.md) and
planning AGENTS.md. Confirm this is the active assignment before starting.
Use collaboration-framework, Rust API/trait/error guidance and testing discipline.

Execute I-01–I-06 at source `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` in a
separate implementation worktree/branch; planning remains on planning. If source
has moved, identify the exact delta and reconcile with CRC before code changes.
Implement the section-1 behavior hook, fallible try_install, once-per-type
registration, collision error and compatible install convenience. Prove execution
with two loaded instances, repeated install, kind conflict, independent racks,
correct schedule and feature-off tests. Repair only the bounded rustdoc comments
specified in the plan. No scheduler, seed, adapter, MIDI/device, dependency or
unrelated refactor work is authorized in this slice.

All six rows are open. The complete command matrix, source allowlist and artifact
contract in slice-plan.md are binding. Run every required check; preserve failed
attempts and authorized reruns, distinguishing restricted MIDI access from code
failures. Reproduce diagnostics rather than masking them. No shortcuts, skipped
validation, weaker evidence, inferred source scope, scope reduction, timeline
interpretation or Operator-gate overrides. Global coverage remediation is assigned
to explicit arc08 successors; do not call 93.47% or noisy profiles an accepted 95%.

Write supporting outputs only to this slice's artifacts/ as named by the plan.
Write closing-report.md with this assignment, source/planning revisions and dirty
state, exact source/evidence files, each command result, I-01–I-06 row walk,
artifact inventory, and bubble-up to arc09 (including what remains for slice02/03).
Record all evidence as attested; CRC owns independent acceptance/verifier records.

Commit source changes before review with only the literal eight-path source
allowlist (omit untouched paths). Then commit this slice's allowed planning files
and each individually named log. For each checkout, inspect `git diff --cached --
<exact paths>` and use `git commit --only -m "..." -- <same exact paths>` after
explicit-path staging. Never stage a directory, `git add .`, `-a`, another
contributor's files or generated Cargo output. No push, merge or release.

Return exact prompt path, source commit/branch/worktree, planning commit, six-row
status and remaining failures to Duncan for CRC. A prior report is not execution
of this assignment. If blocked, name the row, exact blocker and re-entry; do not
expand scope or mark an unrun check passed. Subsequent corrections use new
cc-prompt-iterationNN.md files; maximum five executed refinements.
