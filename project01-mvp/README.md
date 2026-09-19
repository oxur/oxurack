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

## Current composition handoff

Arc09's [CDC review](arc09-executable-rack/cdc-verification.md) keeps the arc open
for a reproduced stop-during-manual-batch defect. [Directive06](cdc-directive06.md)
assigns CRC to author a bounded fourth slice; no new CC prompt is issued yet.
Arc10 remains held. The inherited structural command below passes its preserved
slice01 assignment/import/ledger/link checks; it does not prove arc composition
or validate a future slice04 open set:

```sh
python3 project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/artifacts/verify-planning.py
```

Earlier fixed-shape migration checks remain historical. Preserve their evidence
and validate each new assignment against its own plan and ledger.
