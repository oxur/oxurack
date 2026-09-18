# CC assignment — arc09/slice02 iteration01 evidence repair

2026-09-17. Current assignment:
`arc09-executable-rack/slice02-simulation-samples/cc-prompt-iteration01.md`.
This preserved refinement follows `cc-prompt.md`. CC executes the bounded
test/evidence repairs below and returns to Sofie/CRC for independent review.
Three-Contributor Workflow and Expedited Mode remain in force. This is a
continuation of the same six-row slice, not new architecture or acceptance.

## Outcome, authority, pin

Keep the accepted sample behavior and repair its proof. Review
[`crc-verification.md`](crc-verification.md) (initial-assignment partial review):
S-01/S-02 behavior reproduced; S-03 needs two explicit negatives; S-04/S-05
need discriminating oracles; S-06 needs the canonical CC report and a
current-slice planning check. Do not change production behavior merely to
make a test pass. If a new test exposes a real defect, report the exact trace
to CRC before changing public behavior or widening source scope.

Pinned clean source: `8a73d33ad17cfad849db34d10b672fa4d894a79a` in
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`.
Planning predecessor: the CRC commit that issues this prompt; inspect HEAD
and this file on intake. Check both worktrees for drift and dirty files before
dependent edits. The original `cc-prompt.md`, slice plan, ledger, CDC §2
contract and full validation matrix still govern. No slice03, MIDI/RT,
seed, manifest or lockfile work is authorized.

## Required reading and intake

Before edits, load each required item in bounded complete reads; recover any
truncated portion. Record revision, actual loaded extent, conditional status
and a concise source-cited readback in `artifacts/test-evidence.md`. The
readback must explain how the new checks reject the old or incorrect behavior.
Reading receipts alone are not proof. On a resumed context, revalidate HEAD
and reload required material no longer in context.

| Order | Material | Extent and purpose |
| --- | --- | --- |
| 1 | This prompt, `slice-plan.md`, `ledger.md`, `crc-verification.md` | Required-full; current assignment, six rows, review findings. |
| 2 | Preserved `cc-prompt.md`, existing `artifacts/closing-report.md`, `artifacts/test-evidence.md`, `artifacts/validation-results.tsv` | Required-full; original constraints, CC's prior claims and evidence gaps. The old report is prior-attempt evidence, not the canonical root close file. |
| 3 | `../arc-plan.md`; `../slice01-module-behavior-installation/artifacts/execution-contract.md` | Required-full arc plan; required-section complete §2 and §5 plus opening definitions from the CDC contract. Preserve sample/driver/ordering and quality authority. |
| 4 | Planning `AGENTS.md`; `../../project-plan.md` | Required-full AGENTS; required-section Purpose/DoD, current roadmap, quality dispositions and latest CRC handoff in the project plan. |
| 5 | Installed collaboration-framework `knowledge/project-management/guides/README.md`, `guides/02-canonical-planning-worktree.md`, `guides/04-closing-slices.md`; `knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md` | Required-section: Expedited Mode; first-class filenames and iteration preservation; CC close shape; required reading/data, discriminating tests and predecessor-specific checker limits. |
| 6 | Installed `rust-guidelines/SKILL.md`, `guides/02-api-design.md` API-08/API-24, `guides/03-error-handling.md` EH-01, `guides/04-ownership-borrowing.md` OB-12/OB-13, `guides/15-cargo/07-lints-and-formatters.md` CG-L-01; installed testing `guides/01-testing-discipline.md` and `03-validation-gates.md` | Required-section Rust strengths/patterns; required-full testing guides. Apply only to affected test and validation work. |
| 7 | Source `crates/oxurack/tests/sample_pipeline.rs`, `crates/oxurack/src/lib.rs` driver methods and transition tests, `crates/oxurack-core/src/lib.rs` plugin/compatibility tests, `src/tick.rs` sort/phase code, `src/module.rs` `ModuleId` derivation, `src/rng.rs` `derive_seed` | Required-section, complete named symbols and callers at pinned HEAD. Establish test access and the competing name/ID orders before editing. |
| 8 | `../slice01-module-behavior-installation/artifacts/verify-planning.py` | Required-section: complete current-assignment, row and close-file checks plus `main`; identify hard-coded slice01 assumptions. Historical checker is inherited evidence only. |

Conditional: if source/planning drift, an extra file, or ambiguous Bevy behavior
appears, load affected definitions/callers and locked Bevy 0.18.1 time APIs
before dependent work, record the trigger and ask CRC to reconcile scope.
Historical migration material and old iterations are reference-only.

## Source reconnaissance and applied guidance

At `8a73d33`, `sample_pipeline.rs::run_live` consumes `Oxurack` via
`into_app`, so an integration test can inspect `SampleTime`, observed trace,
`Time<Fixed>` and `Time<Virtual>` from the returned `App`, but can no longer
call rack methods. `oxurack/src/lib.rs` inline tests retain private `world`
access and `Oxurack::step`, making them a suitable place for mode transitions.
`test_driver_transitions_discard_overstep_without_catch_up` currently asserts
only `overstep == 0`, never the next live update. The LastWins fixture uses
names `a` and `b` without asserting their `ModuleId` relationship. Numeric
IDs still come from `ModuleId::from_instance_name`, while propagation and
topological tie-breaks now use `Module.instance_name`.

Rust API-08/API-24 (SHOULD/MUST) support retaining the named driver and
existing public surface; EH-01 (MUST) keeps the exact recoverable mismatch
error. OB-12/OB-13 (SHOULD) favor scoped world borrows in the added tests;
CG-L-01 (SHOULD) requires full all-targets Clippy after test edits. Testing
discipline requires a controlled stimulus and intermediate observation: zero
overstep alone cannot prove no later sample, and identical results alone
cannot prove which sort key won. No new public accessor is justified.

## Binding repair sequence and falsifiable oracles

1. Extend `crates/oxurack/src/lib.rs`'s existing transition unit test and/or
   `crates/oxurack/tests/sample_pipeline.rs`. Use controlled Bevy
   `TimeUpdateStrategy::FixedTimesteps` (or an equivalent no-sleep stimulus)
   after Live→Manual→Live. Before the first fresh live 1 ms stimulus, assert
   that a zero-elapsed update adds **zero** samples and leaves the previous
   trace and state intact; then one controlled 1 ms stimulus adds exactly one
   sample at the next index. Repeat the analogous stop→start sequence after
   injecting/discarding overstep. Assert sample count, next index and module
   trace/state, not only `Time<Fixed>::overstep`. For a stopped Live rack,
   supply controlled live updates while stopped, assert zero phase and state
   change, then start and prove the next index is contiguous. Keep the
   separately asserted manual `step()` no-sample and wrong-driver error.
   This rejects replayed wall/fixed backlog and partial stopped execution.
2. Extend `crates/oxurack-core/src/lib.rs` compatibility tests with
   `CorePlugin::fixed()` and explicitly preinserted
   `SimulationRunning(false)` before plugin registration. Run FixedUpdate;
   assert all five phase counters remain zero and SampleTime stays None/0;
   set true and assert one complete sample. This rejects a fixed-plugin
   compatibility path that overwrites the caller's false state.
3. Make S-05 tests prove the ordering *policy*. In the LastWins fixture,
   select two valid distinct instance names whose lexical order and actual
   `ModuleId::from_instance_name` numeric order oppose each other. Assert
   that opposition **before** driving propagation; assert the exact value
   belonging to the lexically last source wins under both spawn orders and
   with an unrelated entity. A bounded deterministic candidate search inside
   the test may select the pair; assert the resulting names/IDs and do not
   edit seed derivation. The rejected numeric-ID sort must yield the other
   value. Likewise strengthen the independent-module/topological tie test
   in `crates/oxurack-core/src/tick.rs` with an opposing name/ID pair and
   assert exact name order after permuted module/entity creation. Maintain
   cycle behavior. These tests must exercise the actual propagation and
   `compute_tick_order` paths, not reimplement their sorting in the oracle.
4. Move CC's first-class close record to
   `slice02-simulation-samples/closing-report.md` as a CC-authored updated
   report. Preserve the earlier `artifacts/closing-report.md` as prior-attempt
   evidence or replace it with a short immutable-history pointer; do not
   silently erase the first return. Identify iteration01, source/planning
   commits, six-row proposed walk, complete artifact inventory, all actual
   validation outcomes, and Bubble-up to arc09. Correct the future-tense
   sentence at the end of `artifacts/test-evidence.md` by adding the actual
   iteration01 intake, checks, traces and deviations. Do not edit CRC's
   verification record.
5. Validate the current slice's planning structure. The existing slice01
   `verify-planning.py` checks slice01 assignment/rows, so its passing result
   is inherited only. Add a bounded current-slice checker under this slice's
   `artifacts/` or provide an exact reproducible shell/Python command in the
   evidence file that checks the **slice02** current assignment/history,
   preserved prompts, six ledger IDs/statuses, root close report, artifact
   inventory paths and local links. Use the same predicate for valid and
   deliberately missing/misplaced close-record and stale-assignment negative
   cases; retain exits. Do not rewrite slice01's checker or claim structural
   checks prove behavior or independent acceptance.

Only test files already in the original source allowlist may change:
`crates/oxurack/src/lib.rs`, `crates/oxurack-core/src/lib.rs`,
`crates/oxurack-core/src/tick.rs`, and
`crates/oxurack/tests/sample_pipeline.rs`. Other original allowlisted
production files are read-only for this refinement unless CRC first reviews
a demonstrated defect. Planning evidence stays under this slice; canonical
root `closing-report.md` is the required exception to the supporting-artifact
home. No new crate dependency, manifest, lockfile, RT/device, seed-derivation
or future-slice changes.

## Validation and return

Run the focused tests for each new oracle under default and no-default
umbrella features as applicable; rerun core, package and module-installation
tests. On the final clean source commit, run the original prompt's complete
workspace all-features/no-default matrices, all-targets strict Clippy,
format, strict workspace and no-default rustdoc, `make check`, separate
`cargo outdated`, diff check and the current-slice structural check. Preserve
actual restricted failures, host retries, ignored tests, exits and raw logs
as separate rows in `artifacts/validation-results.tsv`; do not erase the
earlier 22 attempts. If a gate cannot run, record its exact command and
re-entry condition, without treating a weaker check as equivalent.

Commit source and planning separately with explicit path staging and
`git commit --only` limited to this assignment's changed files. Inspect
staged/cached diffs before each commit; preserve unrelated work. Return the
exact source and planning commits, clean/dirty status, new tests and traces,
negative-control results, every S-row's proposed disposition, gate table and
the canonical close report path. CC's result remains proposed-done until CRC
reproduces it. Slice03 remains held.
