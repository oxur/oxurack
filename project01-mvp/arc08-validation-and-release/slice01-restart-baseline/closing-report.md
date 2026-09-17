# CC closing report — restart baseline, iteration01

## Assignment and evidence status

Assignment executed: `project01-mvp/arc08-validation-and-release/slice01-restart-baseline/cc-prompt-iteration01.md`.

| Field | Value |
| --- | --- |
| Project / arc / slice | Project01 MVP / Arc08 validation and release / Slice01 restart baseline |
| Role | CC evidence pass, returned to CRC through Duncan |
| Source commit checked | `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` |
| Planning baseline before packet | `c613050` |
| Planning commit | The final commit produced by the exact scoped command below; record with `git rev-parse HEAD` after commit |
| Evidence status | Attested by CC; proposed-done only pending CRC reproduction |
| R-row status | R-01 through R-06 remain open |

The implementation checkout was already at the required source pin and remained
clean. Cargo.lock SHA-1 was
`fcc8f7816cc14a1cb5f0df72016b13115404f85f` before and after. No Rust source,
test, manifest, lockfile or Makefile file was changed.

## Validation result

The native and locked build, lint, format, workspace test and feature probes
passed. The complete result table, timestamps, exit codes and raw-log pointers
are in [validation-results.tsv](artifacts/validation-results.tsv).

The following failures or limits remain explicit:

* The first offline `make check` attempt was blocked by a sandbox Cargo registry
  permission error; the preserved rerun with normal tool escalation passed.
* `RUSTDOCFLAGS=-Dwarnings cargo doc --locked --all-features --no-deps --workspace`
  failed on unresolved links to `NonZeroUsize` and `self.probability`.
* Direct `cargo outdated --root-deps-only --ignore-external-rel` completed after
  escalation and reported available direct dependency updates. No update was
  applied and the native wrapper's green behavior was not treated as proof of
  dependency freshness.
* `make coverage` and explicit `cargo llvm-cov --locked --workspace
  --all-features --lib --summary-only` failed at four `oxurack-rt` MIDI
  initialization tests. Each run reached 146 passing and 3 ignored tests but
  produced no overall coverage percentage or denominator.
* Three RT tests remain ignored because they require RT scheduling permission or
  the named quality-run setup. Physical MIDI, DAW, 10/50/100-instance
  benchmarks, the 24-hour soak and Operator musical acceptance were not run.

No `make deps`, ignored-test run, hardware setup, publication, merge or push was
performed. Those actions are outside this evidence-only assignment or require
the retained named setup.

## Six-row walk

| Row | CC result | Evidence / next owner |
| --- | --- | --- |
| R-01 | Attested observation complete; open | `environment.md`, `validation-results.tsv` and logs 01–61 enumerate the matrix, failed attempts and artifact self-checks. CRC reruns runnable checks and verifies limits. |
| R-02 | Attested accounting complete; open | `reconciliation.md` preserves all 24 rows, exact criteria, owners, destinations, reasons and re-entry. CRC compares it with six slice and six arc ledgers and candidate reports. |
| R-03 | Attested source observations complete; open | `baseline-findings.md` checks TM-01–TM-08, WB-01–WB-03 and the listed command/graph/persistence gaps at the pinned source. CRC independently source-checks the anchors. |
| R-04 | Attested failure and obligation accounting complete; open | The findings and result table retain documentation, dependency, coverage, ignored, hardware, benchmark, soak and musical gates. CRC verifies owner and re-entry for each. |
| R-05 | Attested preservation checks complete; open | `verify-planning.py` and `git diff --check` passed. Imported plans, prompts, ledgers and carryover register were preserved; no implementation or dependency file changed. |
| R-06 | Attested close packet complete; open | This report inventories the exact assignment, six rows, artifacts, raw logs, blockers and scoped commit. CRC reviews; CDC and Operator retain milestone authority. |

CC does not close these rows, accept historical behavior, create a CRC
verification file, or choose M1/M2. The slice-plan assignment is marked
returned for CRC review and records one executed refinement of the maximum five.

## TM and workbench dependency disposition

The selected TM-first path cannot reach M1 from the current source without a
rack adapter. The standalone TM has deterministic construction and extensive
outputs, but the rack registry only stores metadata/spawners, the TM crate has
no core/Bevy dependency, and no rack system maps `TickNow` to `TuringMachine::tick`.
The missing M1 dependencies are the adapter/lifecycle, port and parameter
schema mapping, musical clock handoff, patch-seed handoff, ordered MIDI-out
delivery, and runtime ownership/shutdown contract.

The second workbench path cannot reach M2 from the current source because no
rack Clock, LFO or Scope module is present. RT clock infrastructure, core value
types, graph queries and graph-maintenance freshness tests are useful substrate
observations but do not supply the three workbench modules or a bounded signal
history. Live-world patch capture and queued load/save/cable commands also
remain explicit gaps. These findings bubble to Arc08 and the owning Arc01,
Arc02, Arc04, Arc05 and Arc06 follow-up work; no cross-arc design was made in
this slice.

The full 95 percent coverage target, RT P99 target, 10/50/100 benchmarks,
24-hour soak, named hardware jitter evidence, docs/examples obligations and
Operator music session remain in the project and arc plans. The baseline does
not reduce them or authorize release.

## Artifact and commit inventory

The four CC artifact files are:

* `artifacts/environment.md`
* `artifacts/validation-results.tsv`
* `artifacts/baseline-findings.md`
* `artifacts/reconciliation.md`

The raw logs are individually named and preserved:

* `artifacts/logs/01-make-help.log`
* `artifacts/logs/02-make-info.log`
* `artifacts/logs/03-make-check-tools.log`
* `artifacts/logs/04-rustc-version.log`
* `artifacts/logs/05-cargo-version.log`
* `artifacts/logs/06-implementation-status-before.log`
* `artifacts/logs/07-implementation-head.log`
* `artifacts/logs/08-cargo-metadata.log`
* `artifacts/logs/09-command-cargo-binstall.log`
* `artifacts/logs/10-command-cargo-outdated.log`
* `artifacts/logs/11-lockfile-status-before.log`
* `artifacts/logs/12-lockfile-hash-before.log`
* `artifacts/logs/13-lockfile-diff-before.log`
* `artifacts/logs/14-planning-status-before.log`
* `artifacts/logs/15-planning-head.log`
* `artifacts/logs/16-make-check-offline.log`
* `artifacts/logs/17-make-check-offline-escalated.log`
* `artifacts/logs/18-cargo-build-locked-workspace.log`
* `artifacts/logs/19-cargo-clippy-locked-all-features.log`
* `artifacts/logs/20-cargo-fmt-check.log`
* `artifacts/logs/21-cargo-test-locked-all-features.log`
* `artifacts/logs/22-cargo-test-locked-workspace-defaults.log`
* `artifacts/logs/23-cargo-test-locked-workspace-no-defaults.log`
* `artifacts/logs/24-cargo-test-locked-oxurack-no-defaults.log`
* `artifacts/logs/25-cargo-test-locked-core-no-defaults.log`
* `artifacts/logs/26-cargo-test-locked-core-rt-bridge.log`
* `artifacts/logs/27-cargo-test-locked-tm-serde.log`
* `artifacts/logs/28-cargo-test-locked-tm-midi-io.log`
* `artifacts/logs/29-cargo-doc-locked-all-features.log`
* `artifacts/logs/30-cargo-outdated-direct.log`
* `artifacts/logs/31-cargo-outdated-direct-escalated.log`
* `artifacts/logs/32-make-coverage.log`
* `artifacts/logs/33-cargo-llvm-cov-locked-workspace.log`
* `artifacts/logs/34-ignored-tests-inventory.log`
* `artifacts/logs/35-hardware-and-soak-markers.log`
* `artifacts/logs/36-verify-planning.log`
* `artifacts/logs/37-planning-diff-check.log`
* `artifacts/logs/38-lockfile-status-after.log`
* `artifacts/logs/39-lockfile-hash-after.log`
* `artifacts/logs/40-implementation-status-after.log`
* `artifacts/logs/41-source-tm-adapter-search.log`
* `artifacts/logs/42-source-workbench-inventory.log`
* `artifacts/logs/43-source-persistence-command-search.log`
* `artifacts/logs/44-source-midi-graph-search.log`
* `artifacts/logs/45-source-ignored-tests.log`
* `artifacts/logs/46-source-midi-graph-search-corrected.log`
* `artifacts/logs/47-source-patch-rng-lines.log`
* `artifacts/logs/48-source-event-bridge-lines.log`
* `artifacts/logs/49-source-rng-lines.log`
* `artifacts/logs/50-source-tm-engine-lines.log`
* `artifacts/logs/51-source-patch-lines.log`
* `artifacts/logs/52-source-event-lines.log`
* `artifacts/logs/53-source-core-bridge-lines.log`
* `artifacts/logs/54-validation-tsv-shape.log`
* `artifacts/logs/55-artifact-log-count.log`
* `artifacts/logs/56-planning-status-current.log`
* `artifacts/logs/57-planning-diff-check-current.log`
* `artifacts/logs/58-cached-diff-check.log`
* `artifacts/logs/59-cached-name-status.log`
* `artifacts/logs/60-cached-stat.log`
* `artifacts/logs/61-verify-planning-final.log`

First-class planning updates are this report, `ledger.md` and the assignment
status in `slice-plan.md`. The issued `cc-prompt.md` and
`cc-prompt-iteration01.md`, imported planning history and
`artifacts/carryover-register.tsv` remain preserved.

The exact scoped commit sequence from the planning worktree uses `git add --`
with every path listed in the commit command below, followed by cached-diff
inspection and that same explicit path list in `git commit --only`.

```text
git commit --only -m "Record restart baseline evidence" -- \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/ledger.md \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/slice-plan.md \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/closing-report.md \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/environment.md \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/validation-results.tsv \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/baseline-findings.md \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/reconciliation.md \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/01-make-help.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/02-make-info.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/03-make-check-tools.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/04-rustc-version.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/05-cargo-version.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/06-implementation-status-before.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/07-implementation-head.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/08-cargo-metadata.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/09-command-cargo-binstall.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/10-command-cargo-outdated.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/11-lockfile-status-before.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/12-lockfile-hash-before.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/13-lockfile-diff-before.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/14-planning-status-before.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/15-planning-head.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/16-make-check-offline.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/17-make-check-offline-escalated.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/18-cargo-build-locked-workspace.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/19-cargo-clippy-locked-all-features.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/20-cargo-fmt-check.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/21-cargo-test-locked-all-features.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/22-cargo-test-locked-workspace-defaults.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/23-cargo-test-locked-workspace-no-defaults.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/24-cargo-test-locked-oxurack-no-defaults.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/25-cargo-test-locked-core-no-defaults.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/26-cargo-test-locked-core-rt-bridge.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/27-cargo-test-locked-tm-serde.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/28-cargo-test-locked-tm-midi-io.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/29-cargo-doc-locked-all-features.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/30-cargo-outdated-direct.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/31-cargo-outdated-direct-escalated.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/32-make-coverage.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/33-cargo-llvm-cov-locked-workspace.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/34-ignored-tests-inventory.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/35-hardware-and-soak-markers.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/36-verify-planning.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/37-planning-diff-check.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/38-lockfile-status-after.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/39-lockfile-hash-after.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/40-implementation-status-after.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/41-source-tm-adapter-search.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/42-source-workbench-inventory.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/43-source-persistence-command-search.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/44-source-midi-graph-search.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/45-source-ignored-tests.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/46-source-midi-graph-search-corrected.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/47-source-patch-rng-lines.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/48-source-event-bridge-lines.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/49-source-rng-lines.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/50-source-tm-engine-lines.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/51-source-patch-lines.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/52-source-event-lines.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/53-source-core-bridge-lines.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/54-validation-tsv-shape.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/55-artifact-log-count.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/56-planning-status-current.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/57-planning-diff-check-current.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/58-cached-diff-check.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/59-cached-name-status.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/60-cached-stat.log \
  project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/logs/61-verify-planning-final.log
```

CRC receives this exact assignment, the final planning commit hash, and the
raw evidence through Duncan. CDC decides the milestone and any successor
slice; the present report is not an independent acceptance record.
