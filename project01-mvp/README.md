# project01-mvp

Start with [project-plan.md](project-plan.md), then
[status-and-backlog.md](artifacts/status-and-backlog.md). The implementation
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
The generated `docs/design/index.md` was excluded and remains on `main`.

Arcs01/02 retain their old phase/milestone breakdowns; no individual phase files
existed to move into slices. Their current interpretation is in the project
roadmap, backlog and sibling ledgers. Arcs07/08 are outline plans; no future
slice directories have been manufactured.

From the planning worktree, check the migration with:

```sh
python3 project01-mvp/artifacts/verify-migration.py
```
