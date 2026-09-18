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
before host retries. This file will be extended after implementation and the
source commit with the six-row proposed walk and sample traces.
