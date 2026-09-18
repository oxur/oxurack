# Slice02 CC intake and test evidence

## Iteration intake

Date: 2026-09-17. Role: CC implementation under the Operator-selected
Three-Contributor Workflow and Expedited Mode.

| Item | State and loaded extent | Result |
| --- | --- | --- |
| Active prompt | Planning HEAD `bc641325f2d48c56516cfe7b3a202e4f5e19c539`; `cc-prompt.md`, required-full, loaded in one complete bounded read | Complete; current assignment is `arc09-executable-rack/slice02-simulation-samples/cc-prompt.md` |
| Slice plan and ledger | Same planning HEAD; `slice-plan.md` and `ledger.md`, required-full, loaded in complete bounded reads | Complete; six open rows S-01 through S-06 |
| Planning instructions | `AGENTS.md`, required-full, loaded completely | Complete; canonical planning worktree, explicit-path commits and CC/CRC/CDC role boundaries recorded |
| Arc plan | `../arc-plan.md`, required-full, loaded completely | Complete; slice01 is closed, slice02 is active, slice03 remains future work |
| Execution contract | `slice01-module-behavior-installation/artifacts/execution-contract.md`: opening authority and complete §1, §2 and §5 loaded in bounded ranges 1–107 and 185–201 | Complete for this slice; §2 governs the exact sample and phase boundary; §3–§4 are future work |
| Project plan | `project-plan.md`: Purpose/definition of done lines 1–35; current roadmap and quality dispositions lines 224–283; latest slice01 close/slice02 handoff lines 379–403 | Complete for required sections; project gates and successor ownership retained |
| Accepted predecessor review | `slice01-module-behavior-installation/crc-verification.md`, iteration02 independent close and Bubble-up loaded in complete ranges 72–120 | Complete; slice01 is independently closed; this slice remains open for CRC review |
| Prompt authoring and project management | Installed guide `engineering-methods/guides/07-implementation-prompt-authoring.md`, required intake/review sections lines 194–316; project-management `guides/README.md`, including Expedited Mode lines 87–122 | Complete; intake/readback, evidence strength, explicit commits and no-shortcuts rules applied |
| Rust guidance | Installed `rust-guidelines/SKILL.md` strength legend lines 477–488; API-08 and API-24; EH-01; OB-12 and OB-13; CG-L-01 | Complete; named driver/phase API, typed recoverable mismatch, bounded borrows and full all-target Clippy are binding checks |
| Testing guidance | Installed testing discipline and validation gates guides, both loaded completely | Complete; behavioral oracles, negative controls, feature matrix and separate gate evidence required |
| Source predecessor | Implementation worktree `codex/oxurack-module-behavior-installation`, clean at `6a31c0f99241eccb0dcf0f23e9f64ac9dec2f94a` | Complete; matches the prompt; main remains untouched at `a7ce6e6` |
| Source inspection | `oxurack/src/lib.rs` constructor, lifecycle, step and tests; core `src/lib.rs`; `src/tick.rs` phases/merge/order; `src/module.rs`; `src/error.rs`; `module_installation.rs`; existing behavior fixture | Complete in bounded source ranges covering all named symbols and callers |
| Bevy dependency inspection | Locked registry `bevy_time-0.18.1/src/{fixed,time,real,virt,lib}.rs`; named APIs `overstep`, `discard_overstep`, `advance_by`, `as_generic`, `pause`, `unpause`, `update_with_instant`, and `TimeUpdateStrategy::FixedTimesteps` loaded | Complete; no Bevy 0.19.1 source substituted |
| Conditional drift check | Trigger: source drift, extra allowlisted file, or unclear Bevy behavior | Not triggered at intake; source identity, path allowlist and named Bevy APIs resolved |

No required read was truncated without recovery. Source and planning states were
clean at intake. No edit was made to main, slice01 evidence, manifests,
lockfiles, RT/device code, seed derivation or future-slice material.

## Contract readback

This slice makes one authoritative 1 ms sample visible through a shared
`SampleTime` resource. A running sample begins by publishing the prior completed
count as its zero-based index, runs the strict named chain
`Produce -> Propagate -> MergeInputs -> Consume -> Observe`, then increments the
completed count only after Observe. Stopped schedules leave index, state and
phase traces unchanged. The `SimulationRunning` resource moves into core and
defaults to true only for a bare CorePlugin when no caller supplied a value;
`Oxurack::new()` explicitly overrides it to false. A private begin latch must
gate all five sets and finalization from one snapshot, so a mid-sample stop or
start cannot produce a partial pipeline.

The public umbrella owns only the driver/lifecycle policy: inherent named
`SimulationDriver`, `set_driver_mode`, `driver_mode`, `start`, `stop`,
`sample_time`, and `advance_samples`. Manual mode advances Virtual and Fixed
time together by exactly 1 ms per requested sample, exposes Fixed's generic Time
view during each direct FixedUpdate, and restores Virtual time afterward. Live
mode remains Bevy's fixed-time driver; a manual request in Live returns the exact
typed mismatch without changing mode or time. Stop and mode transitions pause
Virtual time and discard fixed overstep; Live re-entry rebases Real time before
unpausing only when running, preventing wall-time catch-up. `step()` remains an
App update and cannot generate a manual sample.

The internal merge moves into `MergeInputs` so Consume sees all current-sample
contributions, while zero-contribution inputs retain their previous value.
Stateful relay output is therefore delayed one sample. Observable cable and
topological tie order uses module instance names and port names, with no change
to ModuleId values or seed derivation. Named phase membership is the start/stop
guarantee; arbitrary unassigned Bevy systems remain outside it.

The six ledgers map to concrete proof: S-01 checks indices and completed counts
under manual and controlled live drives; S-02 checks the full producer/relay/sink
trace and phase markers; S-03 checks raw-core defaults, explicit false gating,
stopped construction and restart; S-04 checks driver exclusivity, no-op/error
paths and discarded overstep; S-05 perturbs entity creation and fan-in/order;
S-06 checks the exact source allowlist, docs, regressions, full gates and
preserved restricted failures/retries. The easily missed constraint is that a
green direct `FixedUpdate` run alone is insufficient: the same phase pipeline,
time visibility, stopped state and live grouping must be proved through the
public driver and raw CorePlugin compatibility paths.

## Validation evidence

Validation commands and raw outputs are recorded individually under
`artifacts/logs/`, with cwd, source HEAD, dirty/clean state, toolchain, actual
exit and command output. The validation table retains failed restricted attempts
before host retries.

## Iteration01 intake and evidence repair

Date: 2026-09-17. Active assignment:
`arc09-executable-rack/slice02-simulation-samples/cc-prompt-iteration01.md`.
The source worktree was clean at pinned intake revision
`8a73d33ad17cfad849db34d10b672fa4d894a79a`; the planning worktree was clean at
CRC-issued HEAD `881b834a83aaed1ed7663fa393074574b543dd6e`. After intake, only
the prompt-allowlisted test files and slice02 planning evidence are dirty.

### Required-read receipts

| Material | Loaded extent and state | Readback / status |
| --- | --- | --- |
| `cc-prompt-iteration01.md` | Required-full, 158/158 lines at planning HEAD `881b834a` | Complete; preserves six-row scope and requires transition, fixed-core, discriminating ordering, root-close and current-slice checker repairs. |
| `slice-plan.md` | Required-full, 183/183 lines at planning HEAD `881b834a` | Complete; current assignment is iteration01, six rows remain open, slice03 remains held. |
| `ledger.md` | Required-full, 16/16 lines | Complete; S-01 through S-06 remain open. |
| `crc-verification.md` | Required-full, 60/60 lines | Complete; S-01/S-02 reproduced; S-03 fixed-core/stopped-live negatives, S-04 transition, S-05 ordering policy, and S-06 evidence remained open. |
| preserved `cc-prompt.md` | Required-full, 229/229 lines | Complete; original sample/driver/phase/ordering contract and validation matrix remain binding. |
| prior `artifacts/closing-report.md` | Required-full, 160/160 lines | Complete; retained unchanged as prior-attempt evidence, with its misplaced artifact-home close identified by CRC. |
| prior `artifacts/test-evidence.md` | Required-full, 77/77 lines | Complete; prior intake/readback loaded; its future-tense closing sentence is repaired by this section. |
| prior `artifacts/validation-results.tsv` | Required-full, 23/23 lines | Complete; 22 original attempts retained and appended to by iteration01 validation rows. |
| `../arc-plan.md` | Required-full, 48/48 lines | Complete; slice02 is open, slice03 is planned, and no arc scope amendment is authorized here. |
| `../slice01.../artifacts/execution-contract.md` | Opening definitions, complete §1, §2 and §5 loaded in bounded ranges 1–110 and 185–201 | Complete for this assignment; §2 binds sample/driver/phase/ordering behavior and §5 retains quality owners. §3–§4 are future work. |
| planning `AGENTS.md` | Required-full, 26/26 lines | Complete; canonical planning worktree, role boundaries and explicit scoped commits apply. |
| `../../project-plan.md` | Required sections loaded: lines 1–35, 224–283 and 379–403 | Complete; project DoD, roadmap, quality dispositions and slice02 handoff retain their authority. |
| project-management guides | README Expedited Mode; canonical worktree/iteration and verification sections; closing-slices complete 103/103 lines | Complete; root `closing-report.md` is first-class, iteration prompts are preserved, and CRC review remains independent. |
| implementation-prompt guide | Required sections for test/oracle design and CC intake/review loaded in bounded ranges 141–316 | Complete; required reads, discriminating tests, current-validator limits and close shape applied. |
| Rust guidance | `SKILL.md` strength legend; API-08/API-24, EH-01, OB-12/OB-13 and CG-L-01 entries loaded | Complete; retain named driver APIs, typed recoverable errors, scoped borrows and full Clippy. |
| testing guides | `01-testing-discipline.md` 91/91 and `03-validation-gates.md` 82/82 | Complete; controlled stimuli, intermediate observations, negative controls and separate gate evidence applied. |
| pinned source | `sample_pipeline.rs` named fixture/tests and transition paths; umbrella driver/transition tests; core plugin compatibility tests; tick ordering/phase paths; ModuleId and `derive_seed` definitions; affected callers loaded at `8a73d33` | Complete; current tests can observe public App traces, inline tests retain private world/step access, and name/ID policy conflict is now forced by candidate selection. |
| Bevy 0.18.1 | `TimeUpdateStrategy::ManualDuration`, fixed/virtual/real time APIs and schedule behavior loaded from the locked registry | Complete; no ambiguity remained; `ManualDuration(0)` and `ManualDuration(1 ms)` provide controlled no-sleep transition stimuli. |
| inherited `slice01/.../verify-planning.py` | Complete 140-line read including `main` | Complete as inherited evidence only; it is hard-coded to slice01 assignment/ledger and cannot validate this slice. |

The first combined guide/source output was truncated by the tool; the omitted
required sections were recovered with contiguous bounded reads before edits.
No conditional drift trigger remained after rechecking both HEADs: the pinned
source was clean, the planning assignment and allowlist were present, and the
locked Bevy behavior was resolved. No manifest, lockfile, RT/device, seed or
future-slice material was touched.

### Iteration01 contract readback and proof delta

S-03 now has a fixed-schedule negative that inserts `SimulationRunning(false)`
before `CorePlugin::fixed()`, runs FixedUpdate with all five counters at zero
and `SampleTime` at None/0, then enables the resource and requires one complete
sample. The umbrella transition test uses the public rack methods plus a
controlled `TimeUpdateStrategy::ManualDuration`: after one manual sample,
zero elapsed Live update leaves the trace/state and index unchanged, one fresh
millisecond adds exactly index 1, and the analogous stop/start sequence
discards injected overstep, runs no phase while stopped, preserves state on a
zero elapsed restart update and resumes at index 2.

S-04's old overstep-only assertion could pass while a later live update replayed
time; `oxurack/src/lib.rs::test_live_transitions_and_stop_start_discard_catch_up`
now observes `SampleTime`, `LiveStepState`, `LiveSystemRuns` and the controlled
updates. The integration
`sample_pipeline.rs::live_reentry_has_no_zero_elapsed_catch_up_sample` observes
the real RON pipeline and phase/trace state after Live→Manual→Live.

S-05's old `a`/`b` fixture could pass under either sort key. Both the LastWins
fixture and `oxurack-core/src/tick.rs::test_topo_no_cables` now select a
lexical/ModuleId opposition, assert the IDs before driving the real paths, and
assert the lexical-last source/name after both spawn orders. The expected
numeric-ID winner is the other value, so the rejected implementation fails the
same predicate. `derive_seed` and ModuleId production code remain unchanged.

S-06 now has the canonical current-slice checker at
`artifacts/verify-slice02-planning.py` and the first-class root
`slice02-simulation-samples/closing-report.md`; the earlier artifact report is
preserved. The checker uses one predicate for the valid tree and deliberate
missing-root-close and stale-assignment copies. The initial checker exercise
returned `valid_exit=0`, `missing_close_exit=1`, and
`stale_assignment_exit=1`, with the structural-only limitation stated in its
output.

### Iteration01 clean source and final gates

The scoped source repair is committed at
`5c1ce2ccb5e0ba36ac9fd2605a047e0ff45928b2` on
`codex/oxurack-module-behavior-installation`; the source worktree was clean
before every recorded gate. The source commit contains only the four
allowlisted test files:

- `crates/oxurack/src/lib.rs`
- `crates/oxurack-core/src/lib.rs`
- `crates/oxurack-core/src/tick.rs`
- `crates/oxurack/tests/sample_pipeline.rs`

Focused logs 23–28 prove the repaired transition, fixed-plugin false-value,
sample re-entry, LastWins, and topological-order predicates. Complete package
logs 29–34 pass: 412 core unit tests, 5 core integration tests, 2 core
doctests, 15 umbrella unit tests, 5 module-installation tests, 5 sample
pipeline tests, and umbrella doctests. Logs 35–46 retain the complete
workspace, host-retry, Make, and dependency-audit outcomes; logs 47–53 retain
diff, planning, and final tool-preflight outcomes. The original validation
rows and logs 01–22 remain unchanged.

The source trace remains the exact producer/relay/sink sequence recorded above:
sample indices 0, 1, and 2 have producer outputs 1, 2, and 3; relay outputs
0, 1, and 2; sink outputs 0, 0, and 1; and completed counts 1, 2, and 3.
The new transition oracle additionally proves that zero elapsed Live re-entry
adds no sample, one fresh millisecond adds exactly one, stopped controlled
updates change no phase or state, and restart resumes at the next contiguous
index. The fixed-plugin negative proves a preinserted false
`SimulationRunning` remains false until enabled. The ordering oracles select
names whose lexical and ModuleId orders oppose and prove lexical-last
selection under both spawn permutations.

The restricted environment still fails four inherited MIDI initialization
tests in the all-features and no-default workspace runs and in restricted
`make check`; host retries pass. Restricted dependency audit fails only on
`index.crates.io` DNS and the host retry passes. `cargo-binstall --version`
remains a recorded parser-usage exit, while `cargo outdated --version` passes.
The inherited slice01 verifier remains structural-only evidence. The current
slice checker uses one predicate and returns `valid_exit=0`,
`missing_close_exit=1`, and `stale_assignment_exit=1`; it does not claim
semantic acceptance or CRC disposition.
