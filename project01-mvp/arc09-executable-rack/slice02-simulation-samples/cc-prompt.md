# CC assignment — arc09/slice02 deterministic samples

2026-09-17. Three contributors, Expedited Mode. Sofie/CRC assigns implementation
and independently reviews CC's proposed-done return. Current project-relative
assignment: `arc09-executable-rack/slice02-simulation-samples/cc-prompt.md`.

## Outcome, pin and required reading

Implement one indexed 1 ms sample shared by manual and live driving, the strict
`Produce -> Propagate -> MergeInputs -> Consume -> Observe` chain, stopped rack
construction, start/stop gating, exclusive drivers without wall-time catch-up,
and name-stable observable ordering. [CDC execution-contract §2](../slice01-module-behavior-installation/artifacts/execution-contract.md#2-simulation-musical-clocks-and-visibility)
is binding. Clock, TM adapter, seed derivation, MIDI-out, RT/device work and
hardware timing are not in this slice.

Inspected clean source predecessor: `6a31c0f99241eccb0dcf0f23e9f64ac9dec2f94a`,
branch `codex/oxurack-module-behavior-installation`, worktree
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`.
Main remains `a7ce6e6`; do not edit/merge it. Planning predecessor before this
issue: `4fb9f2f755ab261634f93509d8a74ab19aabefc3`. Confirm both current
states before work and reconcile drift with CRC before dependent edits.

Load the manifest into the executing context **before dependent edits**. A
truncated tool result requires contiguous recovery; filenames, hashes, grep
hits and another agent's report are not reading. Record each loaded extent,
state, gap/conditional result and a concise contract readback in
`artifacts/test-evidence.md` before implementation. The readback must connect
API, phase, driver, failure and validation rules to S-01–S-06 and the source.

| Order | Material | Required extent and purpose |
| --- | --- | --- |
| 1 | This prompt, `slice-plan.md`, `ledger.md`, current planning HEAD | Required-full: active contract and six rows. |
| 2 | Planning worktree `AGENTS.md` | Required-full: worktree, role and commit rules. |
| 3 | `../arc-plan.md` | Required-full: arc sequence and composition boundary. |
| 4 | `../slice01-module-behavior-installation/artifacts/execution-contract.md` | Required-section: opening authority, complete §1 installation boundary, complete §2 current design, §5 Q-01/Q-02/Q-03. §3–§4 are future work. |
| 5 | `../../project-plan.md` | Required-section: Purpose/DoD, Current roadmap v3.0 with quality dispositions, and latest CRC close/slice02 handoff. Historical migration directions are reference-only. |
| 6 | `../slice01-module-behavior-installation/crc-verification.md` | Required-section: iteration02 independent close and bubble-up; accepted predecessor only. |
| 7 | Installed collaboration-framework `knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md` and `knowledge/project-management/guides/README.md` | Required-section: “Required reading and CC intake”, “Review evidence”, and Expedited Mode. |
| 8 | Installed `rust-guidelines/SKILL.md` strength legend; `guides/02-api-design.md` API-08/API-24; `guides/03-error-handling.md` EH-01; `guides/04-ownership-borrowing.md` OB-12/OB-13; `guides/15-cargo/07-lints-and-formatters.md` CG-L-01 | Required-section: apply and record actual strengths; load further guidance before relying on it. |
| 9 | Installed `testing/guides/01-testing-discipline.md` and `03-validation-gates.md` | Required-full: behavioral and gate evidence. |
| 10 | Pinned source `oxurack/src/lib.rs` constructor/step/tests; core `src/lib.rs` plugin; `src/tick.rs` phases/merge/order; `src/module.rs` identity/trait; `src/error.rs`; `oxurack/tests/module_installation.rs`; locked Bevy 0.18.1 `bevy_time/src/{fixed,time,real,virt,lib}.rs` named APIs | Required-section: complete named symbols, callers and affected tests before edits. Do not substitute Bevy 0.19.1. |

Conditional: source drift, extra file or unclear Bevy behavior triggers complete
reads of the affected definitions/callers and a CRC reconciliation before that
change. Earlier slice01 prompts and migration documents are reference-only.
Record whether the trigger occurred.

## Inspected source and applied guidance

At `6a31c0f`, `Oxurack::new` installs TimePlugin and `CorePlugin::fixed`, sets
`Time<Fixed>` to 1 ms, and initializes private umbrella
`SimulationRunning(false)`. `step()` calls `app.update()`; `into_app(self)`
consumes the rack. `build_core_plugin` chains three phases on Update or
FixedUpdate and places `consume_ports_system` in Consume. That system merges
Propagate's buffer and preserves an input without contributions. Both
`rebuild_propagation_order` and `compute_tick_order` use numeric `ModuleId`
ties, while `Module.instance_name` is already present. The accepted slice01
fixture registers an unassigned system and runs FixedUpdate on a stopped rack;
the tests must join Produce and explicitly start. Locked Bevy 0.18.1 provides
`Time<Fixed>::overstep/discard_overstep/advance_by/as_generic`, Virtual
`pause/unpause`, Real `update_with_instant`, and
`TimeUpdateStrategy::FixedTimesteps`. Bevy's live loop advances fixed time;
direct `run_schedule(FixedUpdate)` does not.

| Guidance actually read | Decision, reason and check |
| --- | --- |
| Rust API-08 SHOULD / API-24 MUST | Named `SimulationDriver` and inherent `Oxurack` sample methods make mode and ownership explicit; check signatures and wrong-driver behavior. |
| Rust EH-01 MUST | Return a typed, recoverable driver mismatch; a stopped manual request is a valid zero-sample result. Assert the error path changes nothing. |
| Rust OB-12/OB-13 SHOULD | Scope world/time borrows before running the schedule or switching mode; compile and transition tests reject aliasing errors. |
| Rust CG-L-01 SHOULD | Run full workspace all-targets Clippy because new tests are part of the gate; `make check` is not its substitute. |
| Testing discipline/validation gates | Use indexed traces and negative controls; retain real restricted failures and host retries rather than a green wrapper claim. |

## Binding design and implementation recipe

1. In `crates/oxurack-core/src/tick.rs`, add `MergeInputs` and `Observe` to
   `TickPhase`; define `SampleTime` as a `Clone + Copy` resource with private 1 ms
   period, `period() -> Duration`, `last_index() -> Option<u64>` and
   `completed_samples() -> u64`.
   Initially `None`/0; sample-begin publishes the prior completed count as
   its index (first index 0), and sample-finalize increments completed count
   after Observe. All phases see the same index and previous completed count.
   A stopped sample changes nothing; do not silently wrap u64.
   Define shared `SimulationRunning(pub bool)` with `Default(false)`. A private
   sample gate snapshots that value at begin; a mid-sample change takes effect
   on the next sample, preventing a partial pipeline.
2. In core `src/lib.rs`, re-export those resources and configure the strict
   five-set chain on the selected Update
   or FixedUpdate schedule. Begin snapshots running state before Produce; gate
   all five named sets and finalize on that same snapshot, and finalize after
   Observe. Begin changes index only when active. Move port merge from Consume
   to MergeInputs; preserve
   Propagate's rebuild ordering and zero-contribution sample-and-hold. Insert
   running `true` only if a bare CorePlugin App has no caller-supplied state;
   preinserted false remains false. Preserve `TickNow`, RT wiring and patch
   loading. In `module.rs`, document named-phase membership as the module
   author's start/stop guarantee; arbitrary unassigned Bevy systems cannot be
   promised gating.
3. In core `tick.rs`, sort propagation by target instance name/port then
   source instance name/port, and topological ties by instance name, while
   retaining `compute_tick_order`'s signature and cycle behavior. Valid patches
   have unique names; raw duplicate-name ties may use a deterministic fallback.
   Do not change `ModuleId` values or seed derivation. No topological
   same-sample execution is introduced. Test LastWins with permuted entity
   creation and an unrelated entity.
4. In `crates/oxurack-core/src/error.rs`, add only
   `CoreError::ManualSampleRequiresManualDriver`. In `crates/oxurack/src/lib.rs`,
   re-export the shared running type and `SampleTime` (removing the old local
   resource definition/import), define named
   `SimulationDriver { Live, Manual }`, and implement the exact inherent API in
   [slice-plan “Binding API”](slice-plan.md#binding-api-and-state-decisions-within-cdc-2).
   `step() -> ()` remains an App update. Virtual time is paused whenever the
   rack is stopped or Manual, and fixed overstep is discarded at stop/mode
   transitions. On Live re-entry, rebase Real to now while Virtual is paused,
   clear overstep, then unpause only if running. In Manual/running mode,
   `advance_samples(count)` advances `Time<Virtual>` and `Time<Fixed>` together
   by 1 ms for each direct FixedUpdate, exposes Fixed's generic Time view during
   the schedule and restores virtual Time after. Live-mode calls return the
   exact error without mutation;
   stopped or zero-count calls run none. Keep install/load/into_app signatures.
5. In the existing umbrella and module-installation tests, start the rack
   before direct fixed runs (or set the core resource after `into_app`) and
   register behavior systems in Produce. Retain old conflict, reinstall,
   graph and accessor oracles. Add a RON-loaded producer→stateful relay→sink
   fixture and `sample_pipeline.rs` integration tests. Place marker systems in
   all five sets; Observe captures indexed port/state tuples. Use controlled
   Bevy `FixedTimesteps` for live traces, never sleeps. Test both feature modes
   and bare CorePlugin compatibility.

Illustrative **uncompiled** manual loop, binding only in order and visibility:

```text
if mode != Manual: return ManualSampleRequiresManualDriver
if !running || count == 0: return Ok(0)
for each sample:
    Time<Virtual>.advance_by(1ms)  // requested logical time, not wall delta
    Time<Fixed>.advance_by(1ms)
    generic Time = fixed.as_generic()
    end world/resource borrows
    world.run_schedule(FixedUpdate)  // begin + five gated phases + finalize
    generic Time = virtual.as_generic()
return Ok(count)
```

No public App accessor, manifest/dependency, RT/device, seed, or future-slice
change is authorized. Private helper names and fixture components are local
discretion. A material mismatch requires CRC review of exact evidence and
affected S rows before changing public behavior.

## Falsifiable oracles

With a producer emitting 1,2,3 at indices 0,1,2 and a relay/sink whose Consume
stores current merged input for its next Produce, an initially zeroed chain
must yield this exact Observe trace and phase order on each running sample:

| Index | Producer out | Relay in | Relay out | Sink in | Sink out |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0 | 1 | 1 | 0 | 0 | 0 |
| 1 | 2 | 2 | 1 | 1 | 0 |
| 2 | 3 | 3 | 2 | 2 | 1 |

Capture each tuple, not only the final values. Include a disconnected input
that holds its prior value. Compare manual three-sample trace with controlled
live `1+1+1` and `1+2` update groupings. Before start and after stop, manual
requests return 0 and change no phase/sample/state; restart resumes the next
index without resetting state. Live-mode manual request returns exact error
with no mutation. Manual `step()` produces zero samples. A Live→Manual→Live
switch with pending overstep produces no catch-up sample; test stop→start too.
Bare core runs by default, while preinserted false gates all five phases.
LastWins and topological tie cases retain named results after spawn permutation
and unrelated entity insertion. These controls reject early Consume, duplicate
samples, wrong gating and unstable entity-based ordering.

## Scope, gates, evidence and return

Literal allowed source paths:

```text
crates/oxurack-core/src/tick.rs
crates/oxurack-core/src/lib.rs
crates/oxurack-core/src/error.rs
crates/oxurack-core/src/module.rs
crates/oxurack/src/lib.rs
crates/oxurack/tests/module_installation.rs
crates/oxurack/tests/sample_pipeline.rs
crates/oxurack/tests/fixtures/sample-pipeline.ron
```

Write `artifacts/test-evidence.md`, `artifacts/validation-results.tsv`,
individually named `artifacts/logs/`, and a `closing-report.md` with a six-row
walk, exact commit/path lists, artifact inventory and “Bubble-up to arc09”
answering delivered capability, discovered gaps and specified versus delivered
scope. Record each command's cwd, HEAD plus dirty/clean state, toolchain,
actual exit and raw output. Preserve every failed attempt and host retry;
rerun clean-candidate gates after source commit. Do not edit slice01 evidence.

Run each gate separately from the source worktree; add focused test filters
for actual test names and record those exact commands:

```text
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack --test sample_pipeline
cargo test --locked -p oxurack --no-default-features --test sample_pipeline
cargo test --locked -p oxurack --test module_installation
cargo test --locked -p oxurack --no-default-features --test module_installation
cargo test --locked -p oxurack
cargo test --locked --workspace --all-features
cargo test --locked --workspace --no-default-features
cargo clippy --locked --workspace --all-features --all-targets -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --all-features --no-deps
RUSTDOCFLAGS=-Dwarnings cargo doc --locked -p oxurack --no-default-features --no-deps
make check
cargo outdated --root-deps-only --ignore-external-rel
git diff --check
python3 /Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/artifacts/verify-planning.py
```

Preflight `cargo-binstall` and `cargo-outdated`; never run `make deps`.
Restricted MIDI/timing and DNS failures remain real recorded attempts before
host retries. `make check` does not prove direct outdated succeeded. Do not
run ignored physical tests, lower the 95% target, suppress warnings or infer
hardware acceptance. An unexplained required-gate failure keeps S-06 open.

Inspect staged name/status and cached diff, then `git add --` and
`git commit --only` naming literal source paths. Commit planning evidence
separately with explicit paths; preserve unrelated work. Return through Duncan
the prompt path, source branch/worktree/commit, planning commit, six-row
proposed walk, sample traces, all gate attempts and retries, scope diff and
bubble-up. CC does not accept its own rows or advance slice03.
