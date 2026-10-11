# Slice01 LFO closing report

## Intake and authority

Assignment: `arc11-observation-workbench/slice01-lfo/cc-prompt.md`, SHA-256 `888cdb0b5d907b614cbb1356b9fea506a6be15a1d0a7942b1c543f15980bfeb9`. Sibling `slice-plan.md` SHA-256 `0dfa880d6a6fcc2fea5e98fdc2d4476ffece5c9f9e8d5a526756279a127c4af2`; `ledger.md` SHA-256 `3d2165996915baeb27e543239445d461eec2b6934c99840ff817959a2d81a381`.

Planning worktree `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch `planning`, was clean at issue HEAD `a0f77af320310d73b58627db4062f8822680da66`. Source worktree `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`, branch `codex/oxurack-module-behavior-installation`, was clean at required predecessor `65522f116a12a9ac96d35cfe25ed9c184f5bddcb` before implementation. Host: `lappy.local`.

Required reads completed: the full assignment, slice plan and ledger; full `cdc-directive09.md` and Arc11 plan; the named policy sections of `cdc-directive08.md` and current project direction/handoff; source `CLAUDE.md`, clock package/lib/adapter, core parameter registry and tick timing definitions, TM rack port/Produce/Consume/reconciliation implementation, and the complete MIDI example, fixture, integration test and oracle. Rust-guidelines `SKILL.md` and requested AP-08/AP-23/AP-25/AP-33, EH-01, TD-04/TD-05/TD-12 and DC-01 were read, along with the complete validation-gates and slice-closing/row-closure/evidence-strength guides.

Source-cited contract readback: the standalone oscillator evaluates current phase before advancing by one 1 ms step, clamps finite frequency CV to `[0,1]` and treats non-finite CV as neutral, and resets on rising sync before evaluating that sample. Configuration is validated and private; edits preserve phase and sync history. The source pipeline separates Produce, Propagate/Merge, Consume and Observe (`crates/oxurack-core/src/tick.rs:37-124`). Clock's adapter registers against the supplied schedule and validates before mutation (`crates/oxurack-mod-clock/src/rack.rs:79-179,220-318`). TM's Produce publishes a pending step and Consume reads the current bipolar write modulation and reconciles base plus modulation (`crates/oxurack-mod-turingmachine/src/rack.rs:163-240,604-652`). Thus LFO output reaches TM in the same sample's Consume; LFO cable controls are latched for the following LFO Produce. No TM modification or second running gate was needed.

## Result and source candidate

Source commit: `a965386` (`feat(lfo): add standalone and rack oscillator`), based directly on accepted predecessor `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`. Source commit contains only these 11 paths:

- `Cargo.lock`
- `README.md`
- `crates/oxurack/Cargo.toml`
- `crates/oxurack/examples/turing-machine-midi.rs`
- `crates/oxurack/examples/turing_machine_midi/oracle.rs`
- `crates/oxurack/examples/patches/turing-machine-lfo.ron`
- `crates/oxurack/tests/lfo.rs`
- `crates/oxurack-mod-lfo/Cargo.toml`
- `crates/oxurack-mod-lfo/src/lib.rs`
- `crates/oxurack-mod-lfo/src/lfo.rs`
- `crates/oxurack-mod-lfo/src/rack.rs`

The new standalone crate supplies the five canonical waveform names, validated config and typed errors, bounded f64 phase, unipolar/bipolar samples, finite CV behavior, and rising-edge sync. Its optional `rack` feature adds four Reject-merge ports and four registered setters/getters, strict spawn validation, retained control inputs, and supplied-schedule Produce/Consume systems. The existing MIDI example now installs it. The new patch keeps seed 42, sends a 0.5 Hz sine LFO through the existing affine 0.5 depth to TM write modulation, and leaves the original patch unchanged. README documents offline and exact-name live commands. TM production code and native output selection were not changed; automated tests opened no MIDI endpoint.

Cargo.lock only adds the local `oxurack-mod-lfo` package and its umbrella dev-dependency edge. `cargo tree --locked -p oxurack-mod-lfo --edges normal` confirms the standalone default dependency tree contains only `thiserror` (Bevy and core remain optional).

## Validation evidence

All commands below ran against the final source candidate, from S, unless the result explicitly distinguishes an earlier authoring attempt.

- `cargo test --locked -p oxurack-mod-lfo`: 4 unit tests and 1 executable doc test passed.
- `cargo test --locked -p oxurack-mod-lfo --features rack`: 4 unit tests and 1 doc test passed.
- `cargo test --locked -p oxurack --test lfo`: 5 passed. Covers the loaded accelerated 24-sample patch against an independent seeded TM/literal probability and MIDI oracle, a modulation-cable-removed control, 10,000-sample fresh trace repeatability, Clock→two LFO sync/offset observation, stop/resume/idempotent reinstall, registered edits/getters, invalid setter atomicity and invalid spawn with lexical unknown-key precedence.
- `cargo test --locked -p oxurack --no-default-features --test lfo`: 5 passed.
- `cargo test --locked -p oxurack --test midi_output --example turing-machine-midi`: 2 MIDI-output integration tests and 11 example tests passed.
- `cargo tree --locked -p oxurack-mod-lfo --edges normal`: passed; no unintended normal dependency edges.
- `make test`: final host run passed all workspace tests. In `oxurack-rt`, 150 passed, 0 failed, 3 existing timing/scheduling tests were ignored. The initial restricted run failed four existing CoreMIDI discovery/open tests (`test_list_input_ports`, `test_list_output_ports`, `test_open_input_nonexistent_returns_error`, `test_open_nonexistent_port_returns_error`) because MIDI support could not initialize; 146 RT tests passed and the same 3 were ignored. The exact gate was retried with host access and passed. No endpoint was opened by LFO checks.
- `make lint`: passed (Clippy and format checks).
- `cargo clippy --locked -p oxurack-mod-lfo -p oxurack --all-features --all-targets -- -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' make docs`: passed.
- `cargo build --locked -p oxurack --example turing-machine-midi`: passed.
- Built example, new fixture: `cargo run --locked -p oxurack --example turing-machine-midi -- --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --samples 10000`, run twice. `cmp` passed; both traces contain 70 rows and SHA-256 `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
- Built example, original fixture: `cargo run --locked -p oxurack --example turing-machine-midi -- --patch crates/oxurack/examples/patches/turing-machine-midi.ron --samples 2000`. The 13-row output SHA-256 is `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`, matching the pinned compatibility hash. The original fixture file is unchanged.
- `git diff --check`: passed before source commit.

During implementation, an early standalone run exposed the zero-width validation boundary and incorrect test expectations; a rack-feature compile exposed a borrow issue; early installed-path tests exposed RON String syntax and test-phase expectations; and the first strict Clippy pass exposed test-only lint issues. Each was corrected before the final commands above. The restricted `make test` failure and exact successful host retry are both retained above. Workspace runs emitted the existing future-incompatibility notice for locked `block v0.1.6`; it did not fail a gate.

## Five-row proposed disposition

Opening and closing count: five ledger rows. CC recommends proposed-done for every row, with evidence still `attested`; CRC reproduction and acceptance are pending. The ledger stays open until that review.

| ID | CC proposed status | Evidence and qualification |
| --- | --- | --- |
| L-01 | Proposed done | Standalone quarter-phase, cycle, square-width, CV clamp/non-finite, zero-rate hold, sync edge, validation and exact names: focused crate tests; attested. |
| L-02 | Proposed done | Real installed Clock→two LFO cable sync observed in Observe, including offsets, next-sample reset, stop/resume and idempotent reinstall: `tests/lfo.rs`; attested. |
| L-03 | Proposed done | All four registry getters/setters; valid edits preserve phase; bad types/ranges/NaN/name leave config/continuation unchanged; unknown spawn keys win lexically and failed spawn adds no entities: `tests/lfo.rs`; attested. |
| L-04 | Proposed done | New seeded fixture loads and modulates TM; accelerated 24-sample MIDI trace equals independent seeded/literal-schedule oracle and differs from cable-removed control; fresh 10,000-sample traces repeat; original 2,000-sample trace matches pinned hash. No endpoint audition was required by this slice; attested. |
| L-05 | Proposed done | Focused feature matrix, MIDI/example tests, full host workspace tests, lint, all-target strict Clippy, strict docs, example build, normal dependency tree and diff check pass. Restricted CoreMIDI failure, successful host retry and three ignores are disclosed. No extra coverage/freshness gate was run; attested. |

## Artifact inventory and bubble-up to Arc11

No additional supporting artifact was produced under the slice `artifacts/` directory. Temporary example traces were written under `/private/tmp` for comparison; they are validation outputs, not durable slice artifacts. The source crate, fixture, tests and README change are in the separate source commit above.

1. **Assigned capability:** Slice01 delivers the standalone and installed LFO plus the playable seeded TM modulation route assigned by Arc11's slice breakdown. The existing MIDI example can load the new fixture for offline trace or user-selected live playback.
2. **Plan feedback:** no unanticipated dependency, ordering change, scope correction or re-sequencing surfaced. Existing Affine transform and core phase schedule support the intended path. No Arc plan change is indicated; Scope capture and workbench remain later slices and open.
3. **Silent-drop check:** the issued scope was checked against the source commit and all five ledger rows. Standalone behavior, adapter timing/state, registered parameters, seeded fixture/MIDI path, docs and named native gates are present. No requested item was deferred or omitted. CRC review remains pending and no Arc11 or next-slice status is advanced.

## Planning commit

Planning commit: pending. The planning commit will contain only this `closing-report.md` and sibling `ledger.md`; no plan or Arc document is modified.

## Iteration01 intake and contract readback — 2026-10-11

Current assignment: `cc-prompt-iteration01.md`, SHA-256
`1d777451c607735d538a29057b5141ed588428c6a512058a1780d3571b7c8a2c`.
The issued initial prompt and original report above are preserved. Live source
worktree is clean at `a9653866f6ba4e4f238137ad24c2c5a8720d3f83` on
`codex/oxurack-module-behavior-installation`; planning worktree is clean at
`90f48bfdd80da58b55cbfd7d55fd45de98b53d00` on `planning` before this report
edit. Assignment is limited to `crates/oxurack-mod-lfo/src/rack.rs` and
`crates/oxurack/tests/lfo.rs`; L-02 stays open pending CRC. Scope/Slice02 stays
held. No implementation began before this intake/readback.

Required-reading coverage: read this iteration prompt, sibling slice plan,
ledger and CRC verification in full; CDC directive09 and Arc11 plan in full;
source `CLAUDE.md` in full; initial prompt sections “Source-grounded design and
engineering choices”, “Recommended API and implementation sequence”, “Tests and
checks”, and “Scope, discretion and return”; both authorized source files in
full; CorePlugin/CorePluginFixed, Port/CurrentValue/spawn initialization,
TickPhase/SampleTime/running latch/finalization, and LFO `next_sample`/CV/sync
sections; installed Rust-guidelines and testing skill entrypoints, AP-24/AP-25/
AP-33, and the complete validation-gates guide. The applicable guidance treats
broken internal invariants as panics, user/configuration errors as Results, and
prohibits silent fallback of incompatible values. CorePlugin owns sample gates
and the five-phase chain; typed port constructors initialize Float(0)/Gate(false).

Source-cited contract readback: `slice-plan.md` “Binding rack integration and
timing” requires the private adapter to retain controls from Consume, publish
in Produce, and regard missing/wrong-typed internal values as invariant bugs;
the standalone engine separately maps correctly typed non-finite frequency CV
to neutral zero. Current `rack.rs` decoders (`:246-265`) instead map wrong
variants to 0/false. `crates/oxurack-core/src/port.rs` (`:108-159`) confirms
ports carry typed `CurrentValue` initialized by `Value::default_for_kind`.
`crates/oxurack-core/src/lib.rs` (`:118-200`) confirms `CorePlugin` selects
Update, `CorePlugin::fixed()` selects FixedUpdate, and both install the same
core sample phases and running gate; `tick.rs` (`:37-123`) defines the five
ordered phases and completed-sample accounting. Existing `tests/lfo.rs` uses
the actual module and FixedUpdate for lifecycle/oracle checks but lacks the
requested direct Update schedule proof. `lfo.rs` (`:201-247`) preserves typed
Float NaN neutralization in `next_sample`. Therefore this pass adds four
isolated wrong-type regressions with diagnostic evidence and common actual-module
literal-sequence checks through each supplied schedule; it does not change
engine CV semantics, scheduling, or public APIs.

## Iteration01 implementation and evidence — 2026-10-11

Source commit: `c4b517c89f51d44c0eb1bec4a7b891984d3e0a92`
(`fix(lfo): enforce input invariants and verify supplied schedules`), based on
`a9653866f6ba4e4f238137ad24c2c5a8720d3f83`. Its complete path list is exactly:

- `crates/oxurack-mod-lfo/src/rack.rs`
- `crates/oxurack/tests/lfo.rs`

The private Float/Gate input decoders now panic with the input name, required
Value variant and actual value when internal state has the wrong type. Missing
component `expect`s and typed Float NaN handling are unchanged. Integration
tests build an App with the real CorePlugin/CorePluginFixed and call
`LfoModule::register_systems` on the supplied interned schedule; they use the
actual spawned module and discover its ports by `Port.name`. No copied scheduler,
facade schedule substitution, `App::update`, engine/core change or public API
change was introduced.

### F-01/F-02 regression evidence

Before the adapter fix, the four new isolated corruption tests were run against
baseline `a965386` with
`cargo test --locked -p oxurack --test lfo schedule_rejects_wrong_ -- --nocapture`.
All four failed as expected because the test did not panic: Update `freq_in =
Gate(true)`, Update `sync_in = Float(0.5)`, FixedUpdate `freq_in = Gate(true)`,
and FixedUpdate `sync_in = Float(0.5)`. Port lookup and schedule setup completed;
none failed from a missing port. After the change all four pass only with their
expected diagnostic text (`LFO freq_in must carry Value::Float` or `LFO sync_in
must carry Value::Gate`), proving the invariant panic on both supplied schedules.

The direct adapter sequence runs independently on Update and FixedUpdate with
`saw_up`, 62.5 Hz, zero offset, width 0.5 and no cables. For both, a stopped
schedule run leaves completed samples at 0 and both typed output defaults
unchanged; four running runs publish bipolar `[-1.0, -0.875, -0.75, -0.625]`
and unipolar `[0.0, 0.0625, 0.125, 0.1875]` with completed count 4; one stopped
run changes neither outputs nor count; after resume the next output is bipolar
`-0.5`, unipolar `0.25`, and count 5. This checks that the supplied schedule
actually owns the adapter execution, that output is evaluated before phase
advance, and that core stop/resume gating is honored without a second adapter
gate.

A separate run per supplied schedule starts `saw_up` at base 0 Hz, sets the
actual `freq_in` port to correctly typed `Float(NaN)` before the first sample,
then runs three samples. It publishes finite unipolar 0 and bipolar -1 throughout
with completed count 3 and no invariant panic. This preserves the engine's
typed-nonfinite CV-neutral policy. Existing loaded-patch, Clock/two-instance
sync and latency, parameter, stop/resume/reinstall and cable-removed oracles all
remain in `tests/lfo.rs` and pass.

One initial post-fix focused run had 11/12 tests pass because the NaN case used
the shared 62.5 Hz setup while asserting the zero-rate held-phase result. The
test helper was parameterized and that case now explicitly uses base 0 Hz, as
the assignment specifies; the corrected test and subsequent complete focused
runs pass. The first `make lint` run passed Clippy but reported one rustfmt-only
line-wrap difference; the formatting was corrected and the unchanged gate then
passed.

### Validation receipt

All commands ran from S, the source worktree, against `c4b517c` unless noted.

| Command | Result |
| --- | --- |
| `cargo test --locked -p oxurack-mod-lfo` | 4 unit tests + 1 doc test passed |
| `cargo test --locked -p oxurack-mod-lfo --features rack` | 4 unit tests + 1 doc test passed |
| `cargo test --locked -p oxurack --test lfo` | 12 passed; includes both supplied schedules and prior LFO oracles |
| `cargo test --locked -p oxurack --no-default-features --test lfo` | 12 passed |
| `cargo test --locked -p oxurack --test midi_output --example turing-machine-midi` | 2 integration tests + 11 example tests passed |
| `make test` restricted attempt (before the final typed-output-default assertion was added) | 146 RT tests passed, 4 existing CoreMIDI discovery/open tests failed to initialize MIDI, 3 pre-existing RT tests ignored; other displayed workspace test groups passed |
| `make test` unchanged host-access retry | Passed; RT 150 passed, 0 failed, 3 pre-existing timing/scheduling tests ignored; workspace and doc tests passed |
| `make lint` | First attempt, before the final typed-output-default assertion: Clippy passed, formatting check found one line-wrap difference. After correction, Clippy and format checks passed; lint reran on the final source commit |
| `cargo clippy --locked -p oxurack-mod-lfo -p oxurack --all-features --all-targets -- -D warnings` | Passed |
| `RUSTDOCFLAGS='-D warnings' make docs` | Passed; docs generated |
| `cargo build --locked -p oxurack --example turing-machine-midi` | Passed |
| `git diff --check` | Passed before commit |

The built example was run as two fresh processes against
`turing-machine-lfo.ron` with `--samples 10000`; `cmp` passed, each output had
70 rows and SHA-256
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`. The
original `turing-machine-midi.ron` was run with `--samples 2000`; it had 13 rows
and SHA-256
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`. Temporary
outputs were under `/private/tmp`; no MIDI endpoint was opened. The existing
`block0.1.6` future-incompatibility advisory appeared during Cargo commands; it
did not fail a gate. Three existing RT ignores remain visible above; no new
ignore or lint suppression was added.

### Five-row proposed disposition

The denominator remains five. The four prior CRC-done rows stay done at their
reviewed revision; new CC runs refresh adapter-dependent evidence where noted.
L-02 is CC-attested proposed-done only and remains open for CRC acceptance.

| ID | Current status | Iteration evidence and qualification |
| --- | --- | --- |
| L-01 | CRC done at a965386 | Engine-only oscillator behavior and dependency boundary are unchanged; no engine path was edited |
| L-02 | Open; CC proposed-done at c4b517c | Four invariant regressions reproduced red at a965386 and green with diagnostics at c4b517c; literal actual-module Update/FixedUpdate sequences, stopped state, resume and typed NaN neutralization pass; CRC review remains pending |
| L-03 | CRC done at a965386 | Registered setter/getter, edit, invalidation and atomic-spawn tests reran as part of the 12 passing LFO integration tests |
| L-04 | CRC done at a965386 | Loaded accelerated modulation/cable-removed oracle reran; both 10,000-sample fixture processes and original 2,000-sample compatibility hash reproduced |
| L-05 | CRC done at a965386 | All assigned native gates were rerun. Restricted CoreMIDI failure and unchanged host retry are retained; host retry passed. Explicit source inventory is the two authorized paths |

No additional source, design or scope effect emerged. No supporting artifact was
needed; no new file was created. Scope capture remains held pending CRC review
and acceptance. CC proposes L-02 done but does not change its open ledger status,
close Slice01/Arc11, advance Scope or claim CRC acceptance.

Planning commit: pending. It will contain exactly this `closing-report.md` and
the sibling `ledger.md` L-02 evidence update.
