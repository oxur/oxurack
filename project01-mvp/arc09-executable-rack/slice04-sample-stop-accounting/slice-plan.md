# Slice04 — sample stop accounting

Version 1.0, 2026-09-18. Status: open; six rows await CC implementation and
independent CRC review. Workflow: CDC design / CRC review / CC implementation,
Expedited Mode. Source predecessor: clean
`642313a62db437ee2bf0b3d5b97fcdcffa16782a` in
`.worktrees/codex-oxurack-module-behavior-installation`. Planning predecessor:
`6a1bc1340ae6eecb8ed606eb6c1aa8323f7a7119`.
Depends on the three CRC-accepted arc09 slices and the failed composition finding
in [CDC review06](../cdc-verification.md). Authority:
[directive06](../../cdc-directive06.md). Arc10 remains held.

## Outcome and boundary

`Oxurack::advance_samples` reports and advances only complete manual samples when
a module changes the shared `SimulationRunning` resource during a batch. The
current sample remains latched through all five phases; the final shared value
decides whether the next sample starts. A stopped rack is stable across repeated
calls, and `start()` resumes at the next sample without resetting module or seeded
state.

The only allowed source path is `crates/oxurack/src/lib.rs`: the production
method and rustdoc plus focused helpers/tests in its existing `cfg(test)` module.
No public API, error, dependency, manifest, lockfile, core, graph, module, RT,
MIDI or device change belongs here. The production repair should remain a local
completed counter and a running check at the loop head before any clock advance.

## Binding behavior

1. A Live driver returns the exact `ManualSampleRequiresManualDriver` error before
   any mutation, including count zero and a stopped rack. Manual count zero is a
   no-op.
2. Manual mode checks `SimulationRunning` before each attempted sample and before
   either clock advances. A false value ends the batch with `Ok(completed)`.
3. Once a sample starts, core's existing latch governs it through Produce,
   Propagate, MergeInputs, Consume, Observe and finalization. An intermediate
   false does not truncate the sample. The final value governs the next sample;
   false followed by true within one sample allows the next sample.
4. Virtual and Fixed time advance one 1 ms period only for a runnable sample.
   Generic `Time` is fixed during the schedule and restored to virtual afterward,
   including the last completed sample before a stop.
5. Repeated stopped calls preserve counts, clocks, state, RNG/ports and trace.
   `start()` resumes the next index/state. Returned progress equals the completed
   count delta. Normal manual/live, mode-switch and raw-core contracts remain.

Do not clamp the old requested return after executing the whole batch, rewind
clock or state, or poll running independently in each phase. Those approaches
violate the clock and phase-latch contract.

## Proof design

Tests stay in the umbrella library's private test module so they can retain the
rack and inspect `rack.app`. Install a minimal registered module and load real
RON through the existing patch helper. Its context-aware spawn copies the derived
seed into owned state. Systems record phase/index/state/time observations and a
control resource changes running at a selected phase and sample.

Separate Produce, Consume and Observe cases stop during sample zero of a
three-sample request. Each must return one, complete index zero once, emit all
five phase markers once, advance Fixed and Virtual by exactly 1 ms, restore
generic `Time` to virtual and mutate state once. A stop at index one in a larger
batch must return and advance exactly two. A return-only fix still fails these
clock observations.

Repeated stopped calls must be full no-ops. After `start()` and one sample, the
trace, seeded state, clocks and counts must equal an uninterrupted two-sample
reference. Add a false-then-true case to prove final shared state, rather than an
intermediate write, controls the next sample. Preserve existing no-stop, zero,
Live error, raw-core latch, mode-switch and manual/grouped-live evidence.

## Acceptance and validation

The [six-row ledger](ledger.md) is binding. Run focused stop-accounting tests in
default and no-default modes, core and umbrella suites, locked workspace matrices,
all-targets Clippy, formatting, strict rustdoc, `make check`, dependency audit,
source diff and the current-slice planning checker. Record every attempt and any
restricted-access failure plus host retry. Passing does not claim the retained
95% coverage, hardware timing, physical MIDI or musical acceptance gates.

## Assignment history

Current assignment: `arc09-executable-rack/slice04-sample-stop-accounting/cc-prompt.md`

| Date | Prompt | Predecessor | Reason | Disposition |
| --- | --- | --- | --- | --- |
| 2026-09-18 | cc-prompt.md | directive06; source `642313a`; planning `6a1bc13` | Repair partial progress/time accounting after a shared mid-sample stop | Issued for relay; not executed |

Executed corrective refinements: 0 of 5. This initial assignment is not a
corrective pass. CC evidence remains proposed-done until CRC independently
reproduces M-01–M-06.

## Author readiness — CRC

| Check | Result / pointer |
| --- | --- |
| Source-grounded | Ready: inspected the current `advance_samples` loop, driver/running methods, private umbrella tests, public sample pipeline, core phase latch and CDC's failing default/no-default probes at source `642313a`. |
| Design-complete | Ready: directive06 fixes driver precedence, per-sample running check, complete-sample latch, exact partial return, clock ownership, stopped no-op and resume semantics without a new API. |
| Guideline-applied | Ready: [CC prompt](cc-prompt.md#applied-guidance) applies transition/partial-result authoring guidance, Rust EH-01/API-24/OB-02/OB-12/CG-L-01 and behavior-first testing to concrete symbols. |
| Executable | Ready: one source path, exact loop mechanics, real-RON private harness, phase controls, expected values and complete commands are named; CC chooses only private helper names. |
| Falsifiable | Ready: old code fails returned count and elapsed-time oracles; a return clamp still fails clocks; per-phase abort fails the five-marker oracle; state reset fails resumed/reference equality. |
| Coherent and portable | Ready: prompt carries bounded reading/data, pinned revisions, explicit scope, deterministic tests without sleep, exact validation/evidence and return contracts. |

All six author-readiness checks pass. No design blocker remains.

## Version history

- v1.0 — 2026-09-18: CRC acknowledged directive06, created the bounded open set
  and issued the guide-authored initial assignment. Six rows remain open.
