# CC prompt — Slice04 sample stop accounting

You are CC implementing one bounded remediation slice in the Oxurack source
worktree. Follow the Three-Contributor Workflow in Expedited Mode. This prompt is
the current assignment; read it completely before editing. Your implementation
and evidence are proposed-done until CRC independently reproduces every row.

## Pinned intake and role

- Source worktree: `.worktrees/codex-oxurack-module-behavior-installation`
- Required source predecessor:
  `642313a62db437ee2bf0b3d5b97fcdcffa16782a`
- Planning worktree: `.worktrees/planning`
- Planning baseline before this issue:
  `6a1bc1340ae6eecb8ed606eb6c1aa8323f7a7119`. The required planning HEAD is
  the CRC commit that issues this prompt, as relayed by Duncan.
- Current assignment, relative to `project01-mvp`:
  `arc09-executable-rack/slice04-sample-stop-accounting/cc-prompt.md`
- Source branch remains `codex/oxurack-module-behavior-installation`; planning
  branch remains `planning`. Do not touch main, push, merge, release or tag.

Before changing anything, report the two HEADs, branches and cleanliness in your
evidence. The source must match its exact predecessor; the current planning HEAD
must contain this issued prompt and the baseline ancestry above. Stop and return
to CRC if either condition fails or either worktree contains unrelated changes.
Do not reset, clean or absorb someone else's work.

## Required reading and readback

Read these paths before implementation. Unless a subsection is named, read the
whole file.

1. This prompt, [slice plan](slice-plan.md), [ledger](ledger.md), and
   [current checker](artifacts/verify-slice04-planning.py).
2. Repository `AGENTS.md`, [arc plan](../arc-plan.md), and
   [directive06](../../cdc-directive06.md).
3. [CDC verification](../cdc-verification.md): Verdict, Independent reproduction,
   Findings and Bubble-up/next action sections.
4. [CDC review06 notes](../artifacts/cdc-review06/README.md),
   `../artifacts/cdc-review06/probe-final-default-mid-stop.log`, and
   `../artifacts/cdc-review06/probe-final-no-default-mid-stop.log`. Read the
   diagnostic source only to understand the failure; do not copy its dense
   harness into production or preserve its deliberate failure as an ignored test.
5. [Project plan](../../project-plan.md), focusing on current roadmap v3.8 and
   CDC composition return; [current status](../../../status/project01-mvp.md),
   focusing on its Current block; [arc closing report](../closing-report.md) and
   [CRC assembly](../crc-verification.md), including dated addenda.
6. Installed collaboration-framework files:
   - `/Users/oubiwann/.codex/skills/collaboration-framework/knowledge/project-management/guides/README.md`;
   - `.../project-management/guides/02-canonical-planning-worktree.md`;
   - `.../project-management/guides/04-closing-slices.md`;
   - `.../engineering-methods/guides/07-implementation-prompt-authoring.md`;
   - `.../engineering-methods/templates/cc-implementation-prompt.md`.
7. Installed Rust files: `/Users/oubiwann/.codex/skills/rust-guidelines/SKILL.md`;
   `guides/02-api-design.md` API-24; `guides/03-error-handling.md` EH-01;
   `guides/04-ownership-borrowing.md` OB-02 and OB-12; and
   `guides/15-cargo/07-lints-and-formatters.md` CG-L-01. Read installed testing
   `/Users/oubiwann/.codex/skills/testing/SKILL.md`,
   `guides/01-testing-discipline.md` and `guides/03-validation-gates.md` in full.
8. In source, read:
   - `crates/oxurack/src/lib.rs`: `SimulationDriver`, `Oxurack` construction/time
     setup, `set_driver` through `is_running`, `advance_samples`, and the entire
     inline `cfg(test)` helper/test regions touching driver, time or patch load;
   - `crates/oxurack/tests/sample_pipeline.rs` in full;
   - `crates/oxurack-core/src/tick.rs`: `TickPhase`, `SampleTime`, running latch,
     phase chain and finalization.

Capture these source-audit results before editing:

```bash
rg -n 'advance_samples\(|resource_mut::<SimulationRunning>|SimulationRunning\(|TickPhase::(Produce|Propagate|MergeInputs|Consume|Observe)' \
  crates/oxurack/src/lib.rs \
  crates/oxurack/tests/sample_pipeline.rs \
  crates/oxurack-core/src/tick.rs
```

Also capture the authoritative review tables without summarizing away failures:

```bash
jq -r '.[] | [.feature,.case,.exit,.result,.log] | @tsv' \
  .worktrees/planning/project01-mvp/arc09-executable-rack/artifacts/cdc-review06/probe-results.json
jq -r '.[] | [.name,.exit,(.host_access // false),.log] | @tsv' \
  .worktrees/planning/project01-mvp/arc09-executable-rack/artifacts/cdc-review06/checks.json
```

Before editing, write a short intake note in `artifacts/test-evidence.md` naming
the two pinned revisions, the allowed source path, the five binding rules below,
and the concrete guidance applied. If any required file, symbol or command is
missing, stop and report that mismatch instead of guessing.

## Problem and required result

The current `advance_samples(count)` checks `SimulationRunning` once at entry,
then always executes `count` iterations and returns `count`. If a module stops
the shared simulation during sample zero of a request for three, core correctly
finishes that latched sample, but the umbrella starts two more clock iterations.
CDC observed a return of three and 3 ms clock advance for one completed sample.

Repair this interaction with these binding rules:

1. Live always returns exactly `CoreError::ManualSampleRequiresManualDriver`
   before mutation, even for count zero or a stopped rack. Manual count zero is
   a no-op.
2. Manual checks `SimulationRunning` before each attempted sample and before any
   `Time` advance. False ends the batch and returns `Ok(completed)`.
3. A started sample remains latched through Produce, Propagate, MergeInputs,
   Consume, Observe and finalization. A mid-sample stop never truncates it. The
   final shared value controls the next sample; false then true within the same
   sample permits continuation.
4. Advance Virtual and Fixed exactly 1 ms for each sample that will run. Expose
   fixed generic `Time` during the schedule and restore virtual generic `Time`
   after it, including the last sample before stop. A module stop is partial
   success, never a new error.
5. Repeated calls while stopped preserve clocks, counts, module/RNG/port state
   and trace. `start()` resumes the next index without reconstruction or reset.
   Return value and completed-count delta agree.

Normal uninterrupted manual requests still return `count`. Existing raw-core
phase latch, mode switching, overstep discard and controlled Live behavior remain.
This slice does not define wall-clock freezing when callers manipulate the raw
resource outside rack lifecycle APIs.

## Applied guidance

| Guidance | Concrete application |
| --- | --- |
| Prompt authoring: interacting transitions and partial results | Test stop requests inside Produce, Consume and Observe, then compare return, completed count, clocks, phase trace and mutable state. A success exit alone is insufficient. |
| Rust EH-01 | Represent an expected early batch boundary as `Ok(completed)`; retain the exact typed Live misuse error. Do not add a stop error. |
| Rust API-24 | Preserve the inherent `advance_samples(&mut self, u64) -> Result<u64, CoreError>` API and document partial progress precisely. |
| Rust OB-02 and OB-12 | Keep each mutable `World` borrow scoped inside an iteration. End test borrows before calling `stop`, `start` or `advance_samples`; do not duplicate state to evade borrowing. |
| Rust CG-L-01 | No manifest/dependency change is allowed. Keep deterministic helpers local to the existing test module. |
| Testing discipline | Use contract-derived exact values and a reference execution. The predecessor must fail both count and clock oracles, and a return-only clamp must still fail. No sleeps, wall-time tolerance or implementation-computed expectation. |

Apply the Rust guidance; record it in the closing evidence. If a guideline and
the fixed design appear to conflict, stop and ask CRC rather than redesigning.

## Implementation recipe

Edit only `crates/oxurack/src/lib.rs`.

Keep the driver check first so Live errors before every mutation. Keep a manual
zero-count fast path. Replace the requested-count `for` result with a local
completed counter. At the top of each attempted iteration, read
`SimulationRunning`; if false, break before advancing Virtual or Fixed time.
For a runnable sample, preserve the existing sequence: advance both clocks one
`SampleTime::period()`, copy Fixed into generic `Time`, run `FixedUpdate`, copy
Virtual back into generic `Time`, then increment completed. Return that counter.
Scope the mutable-world borrow so the next loop-head running check is legal.

Do not:

- run the old loop and clamp its return afterward;
- undo elapsed time, module state or trace after execution;
- add running checks inside phase systems or alter core's latch;
- move the completed increment before schedule finalization/generic-time restore;
- add an error, public accessor, feature, dependency or manifest change.

Update the method rustdoc so callers know the request is an upper bound, the
return is the number of complete samples, a shared stop finishes its already
latched sample and prevents the next one, and only Live misuse errors. Keep any
example consistent with the actual public API.

## Focused test harness

Add focused private tests/helpers to the existing `cfg(test)` module in the same
file. Reuse its temporary real-RON save/load route. Register a minimal module,
install it, and load a patch through `Oxurack::load_patch`; do not insert only a
synthetic resource and call the scheduler directly for the main regression.
Use `spawn_with_context` so the instance state copies `instance_seed()` into an
owned deterministic state value before construction returns. No new random crate
is needed: a simple explicit wrapping recurrence/counter can make continuation
and reset visible.

Record phase markers with sample indices for Produce, Propagate, MergeInputs,
Consume and Observe, plus state and clock observations. Use a test control
resource to choose the stop phase/sample and optionally a later phase that writes
running true. Retain the `Oxurack` value so tests can inspect `rack.app`; do not
consume it with `into_app`. End every `World` borrow before rack method calls.

The following oracles are required:

- Three separate cases stop in Produce, Consume and Observe during sample index
  zero of `advance_samples(3)`. Each returns one; `SampleTime` has completed 1
  and current index 0; every one of the five phases appears exactly once; mutable
  state advances once; Fixed and Virtual each advance exactly 1 ms; generic
  `Time` is restored to the Virtual value.
- A stop during sample index one of a request larger than two returns two, has
  completed-count and both clock deltas of two periods, and contains exactly two
  complete phase sets.
- After a stopped result, call `advance_samples` repeatedly while false. Each
  returns zero and leaves all counts, time resources, generic time, module state,
  derived-seed state, port/state snapshot and trace byte-for-byte/equality stable.
- Call `start()` and advance one. It reaches index one/count two. Compare its
  per-module phase/state trace and final state with an independently constructed,
  uninterrupted two-sample patch using the same master seed and instance name.
- In one sample, set running false in an earlier phase and true in a later phase.
  A multi-sample request continues because the final value is true.
- Preserve or explicitly cover uninterrupted requested count, manual zero no-op,
  Live error before mutation for ordinary/zero/stopped calls, raw-core full-sample
  latch, driver mode switching and manual/grouped-Live equivalence. Existing
  public integration tests may supply unaffected coverage, but run and cite the
  exact tests; do not claim a test proves an assertion it does not inspect.

Choose private helper names that fit the current module. Include assertion
messages or structured comparisons that make count, time, phase, seed/state and
resume failures distinguishable.

## Scope and file ownership

Allowed source change:

- `crates/oxurack/src/lib.rs`

Allowed planning outputs:

- `project01-mvp/arc09-executable-rack/slice04-sample-stop-accounting/closing-report.md`
- files below `project01-mvp/arc09-executable-rack/slice04-sample-stop-accounting/artifacts/`

Do not edit the prompt, slice plan, ledger, parent arc/project/status, prior slice
reports, CDC review, checker, manifests or lockfile. Do not change core, graph,
module, RT, MIDI or device source. If correct implementation truly requires any
other path, stop and return a scope request before editing it.

Keep source and planning commits separate. Stage explicit paths, inspect the
cached diff, and use `git commit --only` with those same explicit paths. Do not
stage unrelated files.

## Validation commands

Run every command from the repository root unless its path makes the planning
worktree explicit. Preserve command, feature set, exit, environment and log path.
If sandbox MIDI/timing access or DNS fails, preserve that attempt and retry the
same command on the authorized host; do not substitute a narrower command.

```bash
cargo test --locked -p oxurack stop_accounting -- --nocapture
cargo test --locked -p oxurack --no-default-features stop_accounting -- --nocapture
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack
cargo test --locked -p oxurack --no-default-features
cargo test --locked --workspace --all-features
cargo test --locked --workspace --no-default-features
cargo clippy --locked --workspace --all-features --all-targets -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --all-features --no-deps
RUSTDOCFLAGS=-Dwarnings cargo doc --locked -p oxurack --no-default-features --no-deps
make check
cargo outdated --root-deps-only --ignore-external-rel
git diff --check 642313a62db437ee2bf0b3d5b97fcdcffa16782a..HEAD
python3 .worktrees/planning/project01-mvp/arc09-executable-rack/slice04-sample-stop-accounting/artifacts/verify-slice04-planning.py --negative-cases
```

`cargo outdated` is a separate dependency observation; a registry/DNS failure is
not a source-test failure. Do not update dependencies. Record ignored tests and
warnings. Do not run or claim coverage, hardware timing, physical MIDI, soak,
benchmarks or musical acceptance unless separately authorized.

## Evidence and completion contract

Create `artifacts/test-evidence.md` with intake, implementation summary, exact
test/oracle mapping, every validation attempt and exit, restricted failures and
host retries, ignored tests/warnings, source diff/scope inspection and remaining
gates. Create `artifacts/validation-results.tsv` with at least columns
`command`, `feature_set`, `environment`, `exit`, `result`, `log`, `notes`. Put
full command logs under `artifacts/`; retain failures as well as successful
retries. Run the current checker with its negative cases after evidence exists.

Create `closing-report.md` in the slice root. It must state:

- exact source predecessor and source commit;
- exact planning predecessor and planning evidence commit(s);
- explicit changed-source path list and planning-artifact list;
- row-by-row M-01–M-06 proposed disposition with evidence links;
- return/count/time/phase/state/resume results, including the uninterrupted
  reference and false-then-true case;
- all validation denominators, failures/retries and limitations;
- 0 executed corrective refinements, because this is the initial assignment;
- CC status **proposed-done pending CRC review**, with no slice/arc acceptance or
  arc10 advancement claim.

Before committing, inspect `git status --short`, the unstaged source diff, staged
path list and cached diff in each checkout. Commit source first using only
`crates/oxurack/src/lib.rs`. Then write evidence with that immutable source hash
and commit only the allowed planning outputs. If a metadata-only follow-up is
needed to record the final planning hash, keep it explicit and report it.

Return to Duncan with source commit, planning commit(s), worktree cleanliness,
validation summary, preserved environmental failures/retries, and project-relative
links to the closing report, evidence matrix and key focused tests. Do not mark
ledger rows done; CRC owns independent reproduction and disposition.
