# project01-mvp

Start with [current status](../status/project01-mvp.md), then
[project-plan.md](project-plan.md). The migration's
[status-and-backlog.md](artifacts/status-and-backlog.md) retains the original
inventory mapping as a dated baseline. The implementation
README and source history live on `main` in the repository root.

This is a best-effort reconstruction, not retrospective process certification.
Two crate implementation plans now serve as historical arc plans; five bucket
specifications serve as historical slice plans. Other legacy documents keep their
names under the nearest `artifacts/`. Superseded and overwritten copies remain
clearly separated. Only canonical-plan renames received appended version notes.

The [migration report](artifacts/migration-report.md),
[path/hash manifest](artifacts/migration-manifest.tsv), and
[commit map](artifacts/history-commit-map.txt) preserve provenance. Historical
links remain verbatim and can be resolved through the manifest or source Git
history; this migration does not promise they resolve from their new locations.
The generated `docs/design/index.md` was excluded from migration and subsequently
removed from `main` in `a7ce6e6`.

Arcs01/02 retain their old phase/milestone breakdowns; no individual phase files
existed to move into slices. Their current interpretation is in the project
roadmap, backlog and sibling ledgers. Arcs07/08 are outline plans; no future
slice directories have been manufactured.

From the planning worktree, check the migration with:

```sh
python3 project01-mvp/artifacts/verify-migration.py
```

## Restart history

The outline-only statement above describes migration. The restart baseline was
[arc08/slice01](arc08-validation-and-release/slice01-restart-baseline/slice-plan.md),
governed by [directive02](cdc-directive02.md); that observation slice is now accepted.
The migration validator checks a frozen migration shape (six slices and exact
import hashes), so it is not a validator for evolved plans. Preserve it as
historical evidence. The restart checker below applies to its recorded baseline tree:

```sh
python3 arc08-validation-and-release/slice01-restart-baseline/artifacts/verify-planning.py
```

That checker separately proved imported text/artifacts were preserved,
checked carryover row coverage and planning links, and verified its open set.
Those are structural self-checks only, not behavioral or independent acceptance.

## Current implementation handoff

The baseline slice and its checker are preserved evidence of the restart shape.
Current slice is [arc09/slice01](arc09-executable-rack/slice01-module-behavior-installation/slice-plan.md),
with [directive05](cdc-directive05.md) assigning CRC to author the detailed
corrective iteration02 after partial review. Directive03 and exact successor routing still
govern the design and deferred historical rows. Current structural command from
the planning worktree:

```sh
python3 project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/artifacts/verify-planning.py
```

Run earlier snapshot checkers only against their recorded commits; their fixed
arc/slice counts are historical assertions, not constraints on authorized new work.

The structural command above currently fails its obsolete assignment assertion.
Directive05 authorizes CC to repair that check and CRC to reproduce the complete
result; do not report it passed until that work is verified.
