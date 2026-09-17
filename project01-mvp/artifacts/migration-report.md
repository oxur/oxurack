# Planning migration — 2026-09-17

## Scope and decisions

Operator-authorized best-effort migration of every tracked Markdown file under
`docs/`, excluding the generated `docs/design/index.md`. This includes two
`.dustbin/overwritten` artifacts: 23 documents total. The implementation tree
was initially clean at `5cad90a6b8904a3ae615bcbd9ed3e277bd94d95a`.

The canonical home is branch `planning`, worktree `.worktrees/planning`, project
`project01-mvp`. The Operator explicitly allowed a history-preserving departure
from the framework's empty-orphan default. A temporary, independent local clone
was filtered before importing its branch. No existing branch was rewritten;
original commit IDs remain available in implementation history.

The filtered branch has unrelated ancestry and contains the document history,
including historical filenames. It is not intended to be merged into `main`.
The implementation cutover is a normal new commit removing the migrated files
and pointing README/CLAUDE.md at planning. The generated index is unchanged;
its old links are historical and are not maintained by this migration.
No remotes were pushed, tags created, or implementation code changed.

## Organization

Eight arcs describe RT/MIDI; core/persistence; events/literate patching; RackGraph;
public API; module catalog; live interface; and validation/release. Six slice
directories have existing documents: notifications, literate patching, graph
foundation, graph navigation, graph diagnostics, and standalone Turing Machine.

The two crate implementation plans are close matches for arc plans. The five
bucket specifications are close matches for slice plans, though they also contain
historical CC-facing instructions. These seven files were renamed canonically,
with only an appended Version History entry naming the original filename/path.
This records the Operator's archival exception to prompt-path preservation;
none is a currently issued assignment. No historical acceptance report or
fictional handoff was reconstructed. Original headings and stale links remain.

Sixteen design/inventory artifacts retain their filenames and exact bytes. The
catalog is a project artifact because it spans all infrastructure, modules,
interfaces and validation; a fresh project plan supplies current status. Archived
superseded/overwritten copies remain under clearly named artifact subdirectories.
The RT/core plans retain their phase breakdowns without empty phase directories.
Future work is an arc outline/backlog entry until an actual slice is selected.

The Operator selected One-Contributor mode. New ledgers are open reconciliation
and composition aids. They do not certify prior implementation or invent a need
to repeat completed work. Existing independent/Operator gates remain explicit.

## Provenance and reproduction

- [migration-manifest.tsv](migration-manifest.tsv): every old/new path, operation,
  and SHA-256 before/after.
- [history-commit-map.txt](history-commit-map.txt): original-to-filtered commit
  IDs from git-filter-repo; all-zero new IDs identify pruned commits.
- [migration-baseline.json](migration-baseline.json): exact source and filtered
  tips, exclusion, filter expression, and ancestry counts.
- [verify-migration.py](verify-migration.py): structural checks against the
  original objects and the imported history. Uses Python's standard library and Git.

History procedure, performed only in an independent temporary clone:

```sh
git clone --no-local --single-branch --branch main --no-tags SOURCE TEMP/history
git -C TEMP/history filter-repo \
  --path-regex '^docs/(?!([^/]+/)*index[.]md$).*[.]md$' \
  --preserve-commit-hashes --preserve-commit-encoding
git -C SOURCE fetch --no-tags TEMP/history refs/heads/main:refs/heads/planning
git -C SOURCE worktree add .worktrees/planning planning
```

The filter retains all historical Markdown paths under `docs/`, not just current
filenames, so status-folder moves remain in history. Non-document-only commits
are pruned; mixed commits keep their document changes and original messages.
References to implementation changes may therefore remain in commit messages,
but no implementation file snapshots are in the filtered ancestry. The original
source history remains the authority for those implementation claims.

After the migration rename commit, `git log --follow -- <new-path>` traverses
back into imported document history. For original implementation context, use
`git show <original-id>` in the shared repository. The commit map bridges hashes
changed by filtering. No historical link strings were rewritten; use the path
manifest or source baseline to locate their targets.

From the planning worktree:

```sh
python3 project01-mvp/artifacts/verify-migration.py
```

## Status limits

Read [status-and-backlog.md](status-and-backlog.md) for historical commit evidence,
the inventory A–I disposition and the old roadmap phase mapping. Bucket-getter
landed after inventory 0014. Shared MIDI types are not a MIDI-out rack module;
the standalone Turing Machine still needs its rack adapter. These source
cross-checks correct the new roadmap without altering the archival documents.

Migration verification is a self-check of document preservation and organization.
Rust tests, coverage, feature matrices, timing, hardware and musical acceptance
were not run. No project, arc or slice is newly declared independently accepted.

## Self-check results

The first structural run passed after rename commit `ebe1f1a`:

- 23/23 documents mapped; 16 byte-identical artifacts; seven plans changed only
  by their appended rename notes.
- Every nonempty document-tree state across all 73 original commits appears in
  the 17 retained filtered commits. All retained commits preserve author,
  committer, dates, messages and document blobs.
- No implementation files or generated index occur in the filtered ancestry;
  it has no shared ancestor with the original implementation tip.
- `git log --follow` reaches imported history for all 23 documents.
- All 52 new Markdown local links resolve; eight arcs and six document-backed
  slices exist, with no empty future slice placeholders.
- Main cutover was checked against the pinned baseline: exactly the 23 mapped
  originals were removed and only `.gitignore`, `README.md` and `CLAUDE.md` were
  additionally edited. The generated index retained its original bytes. All
  pre-existing Git refs were unchanged before the ordinary cutover commit.

The verifier can be rerun from the planning worktree using the command above.
These are attested structural results from Sofie's self-check, not independent
implementation acceptance.
