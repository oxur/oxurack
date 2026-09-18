# Slice02 — deterministic simulation samples

Version 1.2, 2026-09-18. Status: closed; six rows independently accepted after iteration01.
Workflow: CDC design / CRC review / CC implementation, Expedited Mode.
Source predecessor: clean `6a31c0f99241eccb0dcf0f23e9f64ac9dec2f94a` in
`.worktrees/codex-oxurack-module-behavior-installation`; no implementation of
this slice is claimed. Planning predecessor: `4fb9f2f755ab261634f93509d8a74ab19aabefc3`.
Depends on [accepted slice01](../slice01-module-behavior-installation/crc-verification.md#iteration02-independent-close).
Authority: [CDC execution contract §2](../slice01-module-behavior-installation/artifacts/execution-contract.md#2-simulation-musical-clocks-and-visibility)
and [arc09](../arc-plan.md). This slice details that approved design; any change
to sample duration, compatibility, failure policy or arc scope returns to CDC.

## Outcome and boundaries

One rack sample is exactly 1 ms and carries a zero-based logical sample index.
The same `FixedUpdate` phase pipeline is used by explicit manual samples and
Bevy's live fixed-time loop. Modules can observe a stable `SampleTime` resource.
`Produce -> Propagate -> MergeInputs -> Consume -> Observe` is a strict chain;
Consume sees all current-sample merged inputs and updates state for the next
Produce. Stateful stages therefore add one sample of computational latency.
The rack begins stopped. Start/stop gates the entire named module pipeline,
without resetting sample index or module state. Only one driver owns a rack at a
time, and switching drivers discards pending live overstep rather than replaying
elapsed wall time. `Oxurack::step()` retains its app-update return type and its
zero-or-more fixed-sample live meaning.

This slice does not add the musical Clock, TM adapter, seed handoff, MIDI-out,
device access, live snapshot/save, full transport, or physical timing promises.
The old 95% coverage target remains a project gate owned by Q-02/Q-03; this
slice must add behavior tests without claiming that inherited gate passed.

## Binding API and state decisions within CDC §2

- Define core `SimulationRunning(pub bool)` with `Default(false)` and re-export
  that same type from umbrella, preserving its public name. Bare `CorePlugin`
  and `CorePlugin::fixed()` install it as `true` only when absent, retaining raw
  core's prior running-by-default behavior and honoring an explicitly inserted
  false value. `Oxurack::new()` overrides it to false before any app update.
- Define core `SampleTime` as a `Clone + Copy` resource with private state, a 1 ms period,
  `period() -> Duration`, `last_index() -> Option<u64>` and
  `completed_samples() -> u64` accessors.
  Before the first sample the values are `None`/0. At the beginning of each
  running sample, publish index equal to the prior completed count; after
  Observe, increment the completed count, so the first visible index is 0.
  All five phases of a sample see that same index and the previous completed
  count. A stopped schedule does not advance either field. Overflow
  is an impossible-in-practice invariant failure, never wrapping silently.
- Re-export core `SampleTime` from umbrella and define the named
  `SimulationDriver { Live, Manual }` there. Initial driver is Live. Expose
  inherent methods
  `set_driver_mode(&mut self, SimulationDriver) -> &mut Self`,
  `driver_mode(&self) -> SimulationDriver`, `start(&mut self) -> &mut Self`,
  `stop(&mut self) -> &mut Self`, `sample_time(&self) -> SampleTime`, and
  `advance_samples(&mut self, count: u64) -> Result<u64, CoreError>`.
  `advance_samples` in Live mode returns the concrete
  `CoreError::ManualSampleRequiresManualDriver` without changing mode/time;
  in Manual mode it runs exactly `count` samples when running and returns count,
  or returns zero unchanged when stopped. Count zero in Manual mode is a no-op.
  The new error is a recoverable caller condition documented in core and umbrella.
- `set_driver_mode(Manual)` pauses Bevy virtual time and clears the fixed
  overstep. Manual `advance_samples` directly drives `FixedUpdate`, advances
  Bevy `Time<Virtual>` and `Time<Fixed>` together by 1 ms per sample, and
  temporarily exposes Fixed's generic `Time` view during the schedule. Virtual
  remains paused against wall updates. `step()` may still process non-fixed app
  schedules in Manual mode, but cannot generate a sample. On entry to Live,
  clear overstep and rebase Bevy real time while virtual time is paused, then
  unpause only if `SimulationRunning` is true. Stop also pauses virtual time and
  discards overstep; start in Live resumes from the current wall instant. Thus
  stopped/manual wall time never becomes a later catch-up burst. Document that
  `into_app` transfers lifecycle control to the caller, who must preserve the
  named phase/running contract.
- Keep `TickPhase::{Produce,Propagate,Consume}` names and add `MergeInputs`
  and `Observe`. A private sample-begin gate snapshots `SimulationRunning`
  before Produce; all five sets and sample-finalize use that same snapshot so
  a mid-sample state change takes effect at the next sample, never halfway
  through this one. Begin changes index only when active. Move
  `consume_ports_system` (renamed if useful) from
  Consume to MergeInputs; retain its existing sample-and-hold behavior for
  zero contributions. Update docs and test module hooks to put sample behavior
  in the appropriate named set. Unassigned arbitrary Bevy systems cannot be
  assumed gated: module-author docs must state the phase membership rule.
- For any externally observable order in this slice, use stable module
  `instance_name` and port names, not ECS Entity order or `ModuleId`'s current
  AHasher-derived numeric order. In core propagation and topological tie-breaks,
  replace ModuleId-based ordering with lexicographic instance-name ordering;
  valid patches have unique names. No topological same-sample execution is
  implied, and no seed algorithm/ModuleId value changes in this slice. Module
  systems that emit ordered output must sort their own instance observations;
  arc10 owns ordered MIDI batches.

## Implementation route and source scope

1. In `oxurack-core/src/tick.rs`, add `SampleTime`, the two new phase labels,
   sample-begin/finalize bookkeeping, latched running gate and changed merge docs. Replace
   the propagation and `compute_tick_order` tie-break keys using `Module` names;
   retain cycle detection and the existing public function signature.
2. In `oxurack-core/src/lib.rs`, re-export the two core resources and configure
   the five-set chain on both Update and FixedUpdate variants. Insert core's
   default running state without replacing caller state, schedule sample-begin
   before Produce and sample-finalize after Observe, and apply the latched gate
   to all five sets and finalize; register port merge in
   MergeInputs. Do not change
   `TickNow`, RT bridge, patch loading, or other schedules.
3. In `oxurack-core/src/error.rs`, add only the manual-driver mismatch error.
   In `oxurack-core/src/module.rs`, clarify that installed behavior joins a
   named phase to receive start/stop gating and sample semantics.
4. In `oxurack/src/lib.rs`, move the public running type to its core re-export,
   add private driver state and the inherent APIs above. Keep `step()`'s
   signature. Use bounded world borrows when advancing fixed/generic time and
   running schedules. Restore generic virtual time after manual execution.
5. Update the existing direct-fixed rack tests and
   `oxurack/tests/module_installation.rs` fixtures to call `start()` (or set
   shared running state when already consumed into App) and join Produce. Add
   focused test fixtures under `oxurack/tests/` for phase/merge/latency,
   manual/live trace equivalence, pause/mode transitions and stable ordering.
   The fixture should load a real RON patch with at least producer, stateful
   relay and observer/sink; tests may inspect the world after public load and
   drive controlled Bevy time without sleep.

Literal source allowlist: `crates/oxurack-core/src/tick.rs`, `lib.rs`,
`error.rs`, `module.rs`; `crates/oxurack/src/lib.rs`,
`crates/oxurack/tests/module_installation.rs`, and new bounded
`crates/oxurack/tests/sample_pipeline.rs` plus its RON fixture. A need to edit
another source file, manifest, lockfile, scheduler outside the named phases,
RT/device path, seed derivation or future slice requires CRC review before
affected edits. Planning evidence stays in this slice's `artifacts/`.

## Acceptance and quality gates

The [six-row ledger](ledger.md) names exact oracles. Test default and
no-default features. Preserve the accepted slice01 integration behavior under
the new stopped-on-construction contract by explicitly starting fixtures.
Use `TimeUpdateStrategy::FixedTimesteps` or equivalent controlled Bevy time to
compare live sample traces without sleeping; test render-update groupings such
as three single-sample updates and a one-plus-two grouping. Observe sample
indices and values, not just final totals. Run focused/package/workspace tests,
all-targets Clippy, format, strict docs, `make check`, separate `cargo outdated`,
diff check and the planning verifier. Record restricted failures and host retries
with actual exits and logs. No ignored test, warning suppression or target waiver.

## Assignment history

Current assignment: `arc09-executable-rack/slice02-simulation-samples/cc-prompt-iteration01.md`

| Date | Prompt | Predecessor | Reason | Disposition |
| --- | --- | --- | --- | --- |
| 2026-09-17 | cc-prompt.md | accepted slice01 / CDC contract §2 | First exact-sample and phase implementation | CRC partial review: S-01/S-02 behavior reproduced; S-03–S-06 remain open at source `8a73d33` |
| 2026-09-17 | cc-prompt-iteration01.md | CRC partial review at `8a73d33` / planning `85ea14d` | Distinguishing transition and ordering tests, fixed-core negative, current-slice checker and canonical CC close record | Accepted by CRC at source `5c1ce2c`; six rows done; slice03 may open |

Executed corrective refinements: 1 of 5. Both issued prompts are preserved.
The [CC closing report](closing-report.md) and
[CRC independent close](crc-verification.md#iteration01-independent-close--2026-09-18)
record the final disposition. The checker requiring open ledger rows is a
pre-acceptance predicate; it was run before CRC changed statuses to done.

## Iteration01 author readiness — CRC

| Check | Result / pointer |
| --- | --- |
| Source-grounded | Ready: CRC inspected the eight-file source diff at `8a73d33`, current test bodies, ModuleId derivation and Bevy 0.18.1 `FixedTimesteps` update path. |
| Design-complete | Ready: repair only proof and close-set placement within the existing CDC §2 contract; production behavior, public API, seed and scope changes remain held for review. |
| Guideline-applied | Ready: [iteration01](cc-prompt-iteration01.md#source-reconnaissance-and-applied-guidance) maps Rust API/ownership/error and testing rules to specific test homes and oracles. |
| Executable | Ready: the five-step recipe uses inline rack tests where `world` and `step` remain accessible, integration tests for RON traces, and a bounded slice02 checker. |
| Falsifiable | Ready: next live 0/1 ms updates must show no catch-up; opposing name/ID orders force numeric and lexical implementations to disagree; misplaced/stale planning negatives exercise the current-slice predicate. |
| Coherent and portable | Ready: required-full/section manifest, conditional Bevy read, pinned state, intake/readback, explicit files, complete matrix, scoped commits and CRC return are in the prompt. |

## Author readiness — CRC

| Check | Result / pointer |
| --- | --- |
| Source-grounded | Ready: source reconnaissance in the prompt names `CorePlugin::fixed`, `build_core_plugin`, `TickPhase`, `consume_ports_system`, `Oxurack::step`, Bevy 0.18.1 fixed-time APIs and affected tests at `6a31c0f`. |
| Design-complete | Ready: this plan fixes time/index, start/stop, driver transitions, raw-core compatibility, merge order and stable ordering; CC owns private helper names only. |
| Guideline-applied | Ready: the prompt maps Rust API-08/API-24, EH-01, OB-12 and CG-L-01 plus testing discipline to these decisions and their proof. |
| Executable | Ready: the five-step route above and prompt's ordered file/symbol recipe cover initialization, schedule wiring, time switching and existing test adaptation. |
| Falsifiable | Ready: the ledger and prompt specify sample-indexed multi-stage traces, stopped/mode negatives, merge visibility, ordering perturbations and the full unchanged gate matrix. |
| Coherent and portable | Ready: the prompt carries required-full/section manifest and CC intake/readback; the contract §2 boundary and arc sequence are explicit. Source and planning roots are resolvable. |

## Version History

- v1.0 — 2026-09-17: CRC opened the next approved arc09 slice after independent
  slice01 close, with explicit 1 ms sample, phase, running, driver and ordering
  contracts, six ledger rows and guide-authored CC assignment.
- v1.1 — 2026-09-17: CRC's independent partial review retained all six rows
  open and issued iteration01 for discriminating transition/ordering tests and
  canonical current-slice evidence, without changing CDC §2 or arc scope.
- v1.2 — 2026-09-18: CRC independently reproduced iteration01 and closed all
  six rows. No scope or architecture change bubbled into arc09.
