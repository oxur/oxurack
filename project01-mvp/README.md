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

Arc09's [CDC review07](arc09-executable-rack/cdc-verification.md#cdc-repaired-candidate-acceptance--review07-2026-09-18)
accepts the repaired source `37deeace`. [Directive07](cdc-directive07.md) opens Arc10
and issues its first [Clock prompt](arc10-turing-machine-midi/slice01-musical-clock/cc-prompt.md).
CRC acknowledges and routes it through Duncan, then independently reviews CC.

Arc09 Slice04's checker remains scoped to its preserved assignment/ledger;
it does not validate the Clock open set or prove musical composition. New Clock
structural/evidence checks belong to that slice. Historical fixed-shape migration
checks remain tied to their recorded revisions.
