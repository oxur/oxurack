# Slice02 test evidence

This packet records CC proposed-done evidence for the Arc10 Turing Machine
rack adapter. Independent CRC reproduction remains the acceptance gate; the
six ledger rows remain `open`.

## Provenance and readback

The source worktree is
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, with implementation commit
`9ba55dd0a4c8aa7d67fe26c30f878d89ead34aa2`. The planning worktree is
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch `planning`; its
pre-evidence head was `eb45f3bee2efacbf7e735c9b74db8408d4a01876`. The source
worktree was clean after the source commit. The planning worktree contains the
intake and raw logs for this packet before the evidence commit.

The active authority was
`arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt.md`, with
source baseline `a19cfd1a134c5934d270dc0ba72334d8f167fe2f` and planning
predecessor `ef9d99fe7ab4f38ca46a069c4e87cd4d2e10873f`. The required full
readback and contract decisions are recorded in `artifacts/intake.md`.

The assignment requires a complete 32-port schema: four inputs and 28 scalar
outputs, plus nine parameters. The adapter uses cached entities and private
state, and it installs Consume and Produce systems in the supplied schedule.
Patch construction uses only `ModuleSpawnContext::instance_seed()` through
`TuringMachine::with_seed`; the legacy direct spawn path uses the same helper
through a zero-seed context.

The phase contract is five-part and has one sample of latency. Produce first
takes the prior pending snapshot, then propagation delivers the current input,
and Consume detects rising edges and stores the action result for the next
Produce. A wide Gate is a held level, so only a false-to-true transition is an
event. Reset has priority over move and clock, and all edge histories update
before the priority decision so lower simultaneous edges are consumed. A
stopped sample preserves pending and held state; resume publishes pending work
once. This is why a Clock rise at sample 0 produces a Turing Machine `step` at
sample 1.

Core `oxurack_core::Scale` and the engine `Scale` are distinct types with
different ownership and APIs. The adapter validates the core value strictly
(root 0..=11, nonempty intervals, each interval below 12, strictly increasing
and unique), then copies all intervals, name and root into the engine type.
The adapter validates before invoking legacy clamping/snapping setters so
malformed or out-of-range input cannot be hidden. Getters reconstruct actual
engine state; invalid setters preserve engine state, adapter shadows, edges,
pending output and held ports.

The principal oracle is a separately configured standalone
`TuringMachine::with_seed(context.instance_seed())`, not another adapter or a
helper reused by both paths. The umbrella tests compare every published
StepOutputs field and the delayed scalar ports to that independent reference,
including changed-seed/name and fresh-process controls.

The Rust guidance applied before edits was `rust-guidelines` 2.0.0: API-23
fallible boundaries and contextual typed errors, API-39 strict validation at
the boundary, TD-05 private state with domain ownership, and DC-05 public API
documentation/examples. The implementation-prompt guidance required a
concrete recipe and discriminating oracles; the testing guidance required
behavioral coverage with retained denominators. The adapter therefore keeps
state private, validates complete candidates before mutation, and uses fixtures
where wide gates, delayed publication, modulation accumulation, ordering and
fresh-process seed behavior distinguish common wrong implementations.

## Implementation and behavioral evidence

The source commit changes exactly the ten paths authorized by the prompt:

* the Turing Machine manifest, README, public feature gate, engine getters,
  quantizer getters and new `src/rack.rs`;
* the umbrella dev-dependency, RON fixture, integration test and lockfile.

`src/rack.rs` implements all 32 ports and nine parameters, lexical unknown-key
precedence, strict spawn/setter validation, Scale conversion, seed derivation,
base-plus-held-modulation reconciliation, edge priority, delayed Option
publication and no-mutation failures. The focused rack test suite has 120
passing tests and zero ignored tests. It covers parameter boundaries and wrong
variants, malformed scales, endpoint attribution, complete registration,
direct Option mapping, held gates, reset/move/clock priority, modulation
non-accumulation and pending publication.

The public RON fixture contains two Clock instances, two distinct Turing
Machine instances, explicit scales and parameters, real Clock-to-Turing
Machine cables, and an unrelated module. Its five integration tests pass with
zero ignored tests. They compare manual groups `[24]` and `[1,2,3,5,13]`,
controlled Live groups, reversed module/cable order, all 28 output fields,
standalone seeded traces, fresh child-process output, changed seed/name
controls, stop/resume pending behavior and reinstall after progress.

The focused exact trace has Clock rises at samples `0,2,4,...` and delayed
Turing Machine steps at `1,3,5,...`; the tests compare complete snapshots at
those delayed samples. The standalone package default suite passes 112 tests,
and the rack feature suite passes 120 tests. The normal dependency tree shows
no Bevy or `oxurack-core` edge.

## Validation observations

The complete command matrix, exit codes, result categories and raw-log paths
are in `artifacts/validation-results.tsv`. The individually named raw logs
retain command output. Passing source-specific and umbrella results include:

* default Turing Machine tests, normal dependency tree, rack-feature tests,
  focused umbrella integration, umbrella default/no-default tests and core
  tests;
* workspace Clippy with `-D warnings`, format check, both strict rustdoc
  commands, and `git diff --check`;
* the planning checker with its positive case and four negative controls.

The two restricted workspace test matrices reached 146 passing tests, 4
failing tests and 3 ignored tests. The same four existing `oxurack-rt` MIDI
tests failed in both restricted runs because MIDI support could not be
initialized:
`test_list_input_ports`, `test_list_output_ports`,
`test_open_input_nonexistent_returns_error`, and
`test_open_nonexistent_port_returns_error`. `make check` reaches the same
restricted host-limited test failure. The required elevated retries then pass:
each workspace matrix reports 150 RT tests passed and 3 ignored, and the
elevated `make check` completes all build, lint, format and test gates. No
RT/MIDI source was changed. The ignored count of 3 is retained in both
restricted and host-retry logs; Slice02-focused suites report zero ignored
tests.

`cargo outdated --root-deps-only --ignore-external-rel` exited 1 because the
restricted environment could not resolve `index.crates.io`; the raw log keeps
the DNS error. The elevated retry completed and listed available updates
without changing dependencies. Workspace Clippy and rustdoc retain the
dependency future-incompatibility warning for `block v0.1.6`, while the
required commands exit successfully.

These host and network observations are validation limits, not claims about
the Turing Machine adapter's behavior. They remain explicit for CRC/CDC
review. No acceptance, release, MIDI-device, Slice03 or Arc10 composition
claim is made.

## Corrective iteration01 — CC proposed-done evidence

Current assignment: `arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt-iteration01.md`.
Source candidate committed from the pinned clean `9ba55dd0a4c8aa7d67fe26c30f878d89ead34aa2`
baseline as `b2ffb16d342d9a6cc800bab2713813d9646fb505`. The source worktree is
clean after that commit. The planning worktree began at
`32e693466827eeab1c842931d74772820c2f6032`; its iteration01 changes remain the
current proposed evidence until the following explicit-path commit. All ledger
rows remain open for CRC.

### Corrected behavioral proof

* **F-01 / A-02:** `schema_declares_complete_port_and_parameter_inventory`
  compares every port's ordered name/direction/kind/policy, all nine defaults,
  and unique registered names. `registered_setters_and_getters_cover_every_valid_value_and_alias`
  calls every registered function and checks explicit readback, including all
  nine selector lengths/positions, custom main and output scales, roots,
  ranges, endpoint equality and effective probability. Spawn precedence covers
  length-only, both length forms, embedded roots and explicit-root override.
  `every_invalid_spawn_variant_and_range_is_rejected_without_orphans` and
  `every_invalid_spawn_and_registered_setter_is_atomic_with_seeded_continuation`
  exercise the five value variants, all specified numeric/Scale/range negatives,
  actual error module/parameter, unrelated-entity and whole-entity/module/port
  counts, a complete 32-port/deep-state snapshot, and subsequent output against
  a separately configured standalone engine. Unknown-key tests include
  opposite insertion orders, invalid known values and zeta-only precedence.
  `every_registered_accessor_rejects_missing_and_non_turing_entities` covers
  missing, bare and non-TM module entities. The engine and quantizer getter
  tests verify post-setter scale/root/range values, owned range snapshots and
  both length forms.
* **F-02 / A-03:** `real_produce_publication_maps_options_and_holds_all_values_on_idle`
  inserts distinct sentinel `StepOutputs` into private pending state and runs
  the actual CorePlugin Produce path. It checks all 28 fields, all three
  absent-option pairs, `Some(0)` versus `None`, and that idle Produce clears
  only `step`. `assert_delayed_standalone_match_for` checks every named output
  at all 24 sample indices: initial zeros, independently ticked odd-sample
  publication and even-sample held values with `step=false`.
* **F-03 / A-04:** `consume_priority_consumes_losing_edges_and_matches_scripted_engine`
  follows samples 0–10 with an independently action-driven engine, checking
  each pending `StepOutputs`, engine count/register and every published field;
  it covers wide high, move-over-clock, reset-over-both and consumed losing
  edges. `standalone_move_and_reset_actions_work_without_competing_edges`
  proves each operation alone, retained configuration/RNG stream and held
  scalar behavior. `held_modulation_reconciles_targets_and_registered_base_edits`
  exercises offsets -1/-0.25/0/+0.25/+1 for repeated samples, following
  snapshots and a registered base edit with an older pending snapshot.
  `two_real_sum_cables_drive_write_modulation_through_core_phases` wires two
  actual Bipolar output ports into the adapter through Cable, Propagate and
  MergeInputs, checking +0.75/+0.75, -0.75/-0.75, and +0.25/-0.25 repeatedly.
  `pause_preserves_consumed_high_edges_pending_outputs_and_sample_time` pauses
  private Clock/reset/move levels after edge consumption and verifies full
  state and SampleTime stability and no invented resume edge.
* **F-04 / A-05:** the public test now compares baseline and renamed primary
  rows by equal-length corresponding contents, checks the renamed rows against
  a separately configured engine using the renamed context, and preserves the
  unchanged secondary trace. Seed43 compares the same named rows against its
  standalone expectation and must change primary values. Fresh child runs
  compare canonical output bytes twice and independently change seed/name
  without accepting key-only inequality. The order test separately inserts a
  no-port observer before the TMs and removes the existing observer; both
  preserve named TM traces and SampleTime. Manual/Live grouping and reversal
  comparisons remain.
* **F-05 / A-06:** the source commit changed exactly the four authorized test
  paths. A `git show 9ba55dd:<path>` byte comparison confirms each protected
  production prefix is identical for `engine.rs`, `quantizer.rs`, and `rack.rs`;
  the umbrella file is itself an integration-test file. The tracked fixture,
  manifests and lockfile are unchanged. CRC review/probe/receipts and every
  iteration00 report/log remain preserved.

### Corrected candidate gates and limits

The focused source suite reports 129 passed, 0 failed, 0 ignored; both default
and no-default umbrella Turing Machine integration commands report 5 passed,
0 failed, 0 ignored. The standalone default suite reports 112 passed and the
normal dependency tree has no Bevy/core. `oxurack` default/no-default, core
(414 tests), Clippy `-D warnings`, format, both strict rustdoc commands, and
`git diff --check` pass.

Restricted all-features and no-default workspace runs each report 146 passed,
4 failed, 3 ignored (153 total). The four failures are the same existing
`oxurack-rt` CoreMIDI initialization tests. Restricted `make check` exits 2 on
those failures. Required host retries pass: both workspace feature matrices,
`make check`, and `cargo outdated --root-deps-only --ignore-external-rel` exit 0;
the workspace logs show 150 RT tests passed and 3 ignored. Restricted
`cargo outdated` exits 1 because `index.crates.io` cannot resolve. Both
restricted and host receipts are retained. The `block v0.1.6`
future-incompatibility warning remains visible where emitted.

The planning checker is structural only. Its default mode and `--require-return`
mode each run the built-in same-predicate negative controls; neither constitutes
CRC behavior acceptance. No production defect was observed and no design or
scope change is proposed. Slice03 remains held. A-01–A-06 are CC proposed-done
only and still open pending CRC reproduction and acceptance.
