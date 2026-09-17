# Slice01 — restart baseline

Version 1.1 — 2026-09-17. Status: open, initial assignment issued for Operator relay.
Project01 MVP / arc08 validation and release. CDC: Sofie, restart-planning context.
CC and CRC: separate contexts to be selected/identified by Duncan at handoff.
Source: main `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
Planning predecessor: `f0cf3a5172e02c717c7f924c84156f5fe0405733`; governing state is
the planning commit introducing this open set, plus any explicitly reviewed changes.

## Outcome and limits

Produce a reproducible current engineering baseline and a complete disposition
map for the reconstructed work. This is a bounded evidence/documentation slice:
no Rust implementation, dependency updates, product redesign or acceptance of
historical behavior. It fits one execution context because it collects named
checks and preserves unresolved requirements; it does not audit every historical
feature to completion or repair every failure it discovers.

Read [project plan](../../project-plan.md), [arc plan](../arc-plan.md),
[ledger](ledger.md), [CDC directive](../../cdc-directive01.md), and the current
[status](../../../status/project01-mvp.md). The source scope is the entire six-crate
workspace at the pinned revision, Cargo manifests/lockfile, Makefile and existing
tests/docs; the reconciliation scope is the six historical slices and arcs01–06.
Historical command snippets are evidence, not new instructions.

In scope: exact tool/source state; build/lint/test/doc/feature/coverage observations;
24 row-qualified carryovers; explicit failures, ignored tests and hardware gaps;
current source evidence for known runtime/module gaps; a handoff CRC can reproduce.
Out of scope: fixing Rust/tests/Makefile, installing or upgrading global tools,
editing Cargo.lock, choosing a product milestone, running a hardware/DAW session,
24-hour soak, changing original criteria, package publication, merges or pushes.
Hardware/soak targets remain in the project/arc ledgers with re-entry conditions.

## Steps and verification

1. Check current branch, commit and dirty state before execution. If implementation
   HEAD differs from the pin, record the diff and ask CRC to reconcile; do not
   silently treat old evidence as current. Use the existing source checkout for
   read-only inspection/builds. Do not disturb unrelated staged files.
2. Capture each command below separately, with cwd, revision, tool versions,
   timestamp, exit code, raw log path and pass/fail/blocked/not-run status. Continue
   independent checks after a failure. Preserve all attempts; do not edit outputs.
3. Reconcile the inherited row inventory against the original ledgers, migration
   sources and status/backlog. Keep original criterion, source pointer, destination,
   owner, reason and re-entry visible. Flag missing criteria or contradictions;
   do not declare unverified behavior done. This is administrative accounting;
   deferred behavioral verification is still a requirement.
4. Source-check the specific known gaps listed below and cite file:line plus
   symbol and pinned revision. Distinguish code existence, tests that assert a stub,
   current passing behavior and missing end-to-end acceptance.
5. Self-review the diff and artifacts, record every R row and bubble-up, commit
   only listed planning outputs, return the exact assignment and commit to CRC
   through Duncan. CC does not accept its own rows or close other arcs.

## Baseline command contract

Run from the implementation repository root. Start with `make help`, `make info`,
`make check-tools`, `rustc -Vv`, `cargo -V`, `git status --short`,
`git rev-parse HEAD`, and `cargo metadata --locked --no-deps --format-version 1`.
Record actual six-crate membership and feature names; do not guess from old docs.

| Check | Command / handling |
| --- | --- |
| Repository combined gate | Preflight `command -v cargo-binstall` and `command -v cargo-outdated`. If either is absent, record `make check` blocked because its prerequisite installs tools. Otherwise run `make check` with `CARGO_NET_OFFLINE=true`; this native target does not use --locked, so first record a clean Cargo.lock state/hash and compare it afterward; locked direct probes below are separately required. Do not run `make deps`. |
| Build | `cargo build --locked --workspace` (explicit locked observation of Makefile's build check) |
| Lint | `cargo clippy --locked --all-features --workspace -- -D warnings` |
| Format | `cargo fmt --all -- --check` |
| Workspace tests, features enabled | `cargo test --locked --all-features --workspace` |
| Workspace defaults | `cargo test --locked --workspace` |
| Workspace minimal features | `cargo test --locked --workspace --no-default-features` |
| Umbrella graph disabled independently | `cargo test --locked -p oxurack --no-default-features` (workspace feature unification is not proof of this cell) |
| Core bridge disabled independently | `cargo test --locked -p oxurack-core --no-default-features` |
| Core bridge enabled independently | `cargo test --locked -p oxurack-core --features rt-bridge` |
| TM serde and MIDI separately | `cargo test --locked -p oxurack-mod-turingmachine --features serde`; then the same with `--features midi-io` |
| Docs | `cargo doc --locked --all-features --no-deps --workspace` with `RUSTDOCFLAGS=-Dwarnings` |
| Repository coverage observation | `make coverage` if cargo-llvm-cov exists, preserving its actual scope and output; otherwise exact blocked command/tool reason |
| Explicit workspace coverage | `cargo llvm-cov --locked --workspace --all-features --lib --summary-only`; retain per-crate/file and overall denominator, exclusions and ignored counts. If installed version rejects flags, record failure before documenting supported equivalent. |
| Dependency gate truth | `cargo outdated --root-deps-only --ignore-external-rel` directly with stdout/stderr and exit captured if installed. The Makefile suppresses stderr/does not reliably propagate command failure; a green wrapper does not prove this observation passed. |
| Ignored / hardware coverage | Inventory `#[ignore]`, their tests, prerequisites and owning retained gate; do not run physical I/O tests without the named setup. |

Use a task-specific target directory if needed to avoid another build. Missing
cached dependencies/network/tool permission is a blocker with a precise re-entry
condition, not a pass. Request normal tool escalation when available for necessary
checks; do not automatically install tools. Record lockfile hash before/after;
if a command alters it, retain the diff as a finding and stop further mutation,
without reverting unrelated work. Repository Make targets are retained in the
matrix; locked direct probes add evidence, never conceal a blocked native gate.

Known source-gap checks: AddCable/RemoveCable and queued LoadPatch/SavePatch
NotImplemented; Panic/SetBpm successful no-ops; live-world-to-Patch capture versus
saving a supplied Patch; TuringMachine standalone versus rack adapter; shared MIDI
types versus MIDI-out module; graph parameters_of/getter follow-up and feature
freshness tests; setter schema-validation TODO. Record any contradiction with
status rather than preserving a stale claim. Full code audit/hardening is future work.

## Exit and evidence policy

R-01–R-06 require complete, honest evidence and independently checked accounting.
A recorded failed/blocked test can satisfy baseline *observation completeness*;
it cannot satisfy that test's product acceptance or justify closing a capability
as done. All failures receive an owner, severity/impact and exact re-entry step.
Missing required observations without explanation fail this slice. CRC must
reproduce runnable checks and verify blockers; unresolved access can prevent CRC
acceptance. Historical carryover review and arc gates remain distinct from this
slice's baseline acceptance. Do not make completion depend on the unselected
science-rig/Chord alternatives.

## Artifact and file contract

Supporting home: `artifacts/` in this slice. Pre-existing planning inputs:
`carryover-register.tsv`, `verify-planning.py`, `planning-check.txt`.
CC outputs: `artifacts/environment.md`, `artifacts/validation-results.tsv`,
`artifacts/baseline-findings.md`, `artifacts/reconciliation.md`, and raw logs in
`artifacts/logs/`. Results TSV columns: check_id, command, cwd, source_commit,
started_at, exit_code, status, log_path, limitation. Do not check in compiled
outputs, Cargo target trees or credentials. List each log filename explicitly in
`closing-report.md` and the commit command; use no wildcard staging.
First-class CC updates: this slice's `ledger.md`, `slice-plan.md` assignment status,
and `closing-report.md`. The issued `cc-prompt.md` remains immutable. CC may update
carryover-register.tsv only to correct evidence/ownership within the approved
scope, preserving all original row IDs and criteria; structural changes escalate.
CRC owns verification files and routine status updates; CDC owns structural plans.

## Assignment history

Current assignment: `arc08-validation-and-release/slice01-restart-baseline/cc-prompt-iteration01.md`.

| Issued | Prompt | Predecessor | Reason | Disposition |
| --- | --- | --- | --- | --- |
| 2026-09-17 | cc-prompt.md | none | Establish current evidence before feature work and historical closure | superseded before execution by iteration01; no refinement consumed |

| 2026-09-17 | cc-prompt-iteration01.md | cc-prompt.md | Operator selected TM-first and workbench-second; focus dependency observations | active; awaiting Operator relay/CC execution |

Refinements executed: 0 of maximum 5. No historical budget is reset by this new
slice; any discovered prior open refinement remains tracked in reconciliation.

## Version History

- v1.0 — 2026-09-17: CDC prepared a documentation/evidence-only first slice;
  retained original acceptance obligations and Operator milestone authority.

## Priority amendment — v1.1

Directive02 supersedes directive01's pending milestone decision. Read
[the dependency map](artifacts/turing-machine-dependency-map.md) and independently
source-check TM-01–TM-08 and WB-01–WB-03 as R-03 observations. Do not implement
those contracts in this evidence-only slice. R-02 still accounts for all 24 inherited
rows. No model or time budget was reset; replacing the unstarted initial prompt
consumed no executed refinement pass.

- v1.1 — 2026-09-17: Operator chose TM-in-framework with MIDI, then Clock/LFO/Scope;
  preserved initial prompt and issued iteration01 for focused baseline evidence.
