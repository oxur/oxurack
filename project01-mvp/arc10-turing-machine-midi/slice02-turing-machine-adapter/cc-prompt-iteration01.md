# CC corrective iteration01 — complete Turing Machine adapter proof

## Assignment, authority and baseline

Execute project01-mvp / arc10-turing-machine-midi /
slice02-turing-machine-adapter, corrective pass **1/5**. Current assignment:
`arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt-iteration01.md`.
Predecessor: the preserved `cc-prompt.md` in this directory. Return to Sofie/CRC
through Duncan; Three-Contributor Workflow, Expedited Mode renewed 2026-10-10.

This is a **test and evidence correction**, not a production redesign. All
A-01–A-06 remain open. CRC reproduced focused gates and found F-01–F-05 in
[crc-verification.md](crc-verification.md); no production behavior defect is
claimed. Resolve the specified proof gaps under the unchanged [slice contract](slice-plan.md)
and [Arc10 design](../design.md) §1–2/§5. Slice03 remains held.

Source: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, clean baseline
`9ba55dd0a4c8aa7d67fe26c30f878d89ead34aa2`. Planning predecessor:
`2eb5ac138fedf21d4185d1cc2f1dd3df84c4972d`; current issue commit will include
this prompt, CRC review and metadata/checker updates. Record the actual issue
HEAD at intake. Unexpected source drift or unrelated dirt must be reconciled
with CRC before affected changes; do not reset it.

**Binding:** change test code only in the four source paths below. Production
code, core/Clock/RT, manifests, lockfile, fixture and issued prompts stay
byte-identical. If a new discriminating test exposes a production defect,
retain its complete failure, diagnose it against the existing contract and
return to CRC before modifying production. Do not weaken the assertion.

## Required reading and intake

Aliases, expanded in commands/receipts:

- `P=/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp`
- `S=/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
- `F=/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/collaboration-framework`
- `R=/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/rust-guidelines`

These installed skill paths supersede the predecessor's old skill-path aliases
for this execution. Framework 2.6.0 and Rust 2.0.0 were inspected by the author.
If absent, locate the installed copy and record its version/path; do not
silently substitute summaries or ignore required material.

| Order | Exact input / state | Scope and purpose |
| --- | --- | --- |
| 1 | This prompt, `slice-plan.md`, `ledger.md`, `crc-verification.md` at issue HEAD | Required-full before edits; active repair, unchanged criteria and observed gaps. |
| 2 | `P/cdc-directive07.md`; `P/arc10-turing-machine-midi/design.md` §1, §2 and §5 | Directive required-full; design required-section including all definitions in those sections. Existing sample/mapping/retained-gate authority; §3–4 future MIDI runtime are reference-only for this correction. |
| 3 | Predecessor `cc-prompt.md` §Steps 1, 3–7, Validation/evidence/commit protocol, Explicit stop conditions | Required-section; getters, strict parameters, action/publication and tests. Preserve the rest as reference-only history; current scope overrides its broader initial source allowlist. |
| 4 | `S/crates/oxurack-mod-turingmachine/src/rack.rs`; `S/crates/oxurack/tests/turing_machine.rs`; existing TM RON fixture | Required-full at `9ba55dd`; actual private state/helpers, public integration runners and static config. |
| 5 | `S/crates/oxurack-mod-turingmachine/src/engine.rs` from `impl TuringMachine` through `step_inner`, and its getter/move/reset tests; `quantizer.rs` from `impl Quantizer` through getters and getter test; `outputs.rs` and `length.rs` | Required-section for engine/quantizer, required-full for output/selector; standalone oracle and state ownership. No MIDI wrapper change. |
| 6 | `S/crates/oxurack-core/src/parameter.rs` through registry methods; `module.rs` trait/plugin registration sections; `lib.rs::build_core_plugin`; `tick.rs` SampleTime/SimulationRunning and propagation/merge definitions; `port.rs::spawn_port_on_module`; `cable.rs::Cable` | Required-section including associated type definitions; registered APIs, complete five-phase schedule, held ports and real Sum wiring. |
| 7 | `S/crates/oxurack/src/lib.rs` install/load/driver/start/stop/advance/into_app APIs; current umbrella Clock test lifecycle runners | Required-section; whole-sample public paths and consuming owner boundary. Public `Oxurack` exposes no World accessor. |
| 8 | `R/SKILL.md`, `R/guides/11-anti-patterns.md`; `02-api-design.md` API-23/API-39; `05-type-design.md` TD-05; `13-documentation.md` DC-05 | Entrypoint and anti-patterns required-full, other exact sections required-section. Apply the verified guidance below before Rust edits. |
| 9 | `F/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md` §5, Required reading and CC intake, Corrective iterations; `F/knowledge/testing/guides/01-testing-discipline.md` | Authoring required-section, testing required-full; discriminating oracles, complete loading and truthful return. |
| 10 | Current `artifacts/verify-planning.py`, CRC `results.tsv` and name-control probe/receipt; preserved iteration00 reports/results | Checker/probe/receipt required-full, reports/results required-data: inspect all six claimed rows and all recorded commands/failures/ignored counts. Historical logs reference-only unless investigating a claim; report exact omissions. |

Before dependent edits append an **iteration01 intake** to `artifacts/intake.md`:
source/planning branches/HEAD/cleanliness, prompt identity, versions, actual
loaded extents and conditional gaps, plus a concise source-cited readback.
Recover every truncated/omitted required portion with bounded contiguous reads.
File hashes, grep results and a predecessor receipt do not establish a read.
Revalidate/reload contracts after context recovery.

Readback must connect A-02–A-05 to the deep failure snapshot, real registered
setters, next-Produce latency/holding, consumed losing edges, retained RNG on
reset/pause, and name **content** comparison. Explain why `into_app(self)` ends
wrapper use and why an unchanged key rename cannot prove a changed stream.
Once coherent, execute without an additional approval request.

## Author reconnaissance and applied guidance

At `9ba55dd`, `rack.rs` owns a private `TuringMachineState` with engine,
write_base/modulation, three previous gates, pending `Option<StepOutputs>` and
all cached ports. `decode_spawn_parameters`, `set_parameter`, `apply_candidate`,
`publish_step` and `optional_output` already implement the prescribed shape.
The unit tests have private access but shallow snapshots/partial matrices.
The umbrella `Run` includes full `Trace` and `SampleTime`; `run_patch` already
supports Manual/Live groups. Its `configured_engine` fixes seed 42 and original
names, and `assert_delayed_standalone_match` checks odd samples only. Extend
these test helpers rather than adding a new API or replacing the engine.

`ParameterRegistry::set_parameter(&self, &mut World, Entity, &ModuleKind, &str,
ParameterValue)` and corresponding getter are verified in core. Bevy ECS
0.18.1 `World::resource_scope` permits temporarily removing a registry resource
while calling it with the World; this resolves the borrow boundary below.
`StepOutputs` derives Clone/Default/PartialEq; `TuringMachine` does **not** derive
Clone. Observe the engine and compare future seeded outputs; do not manufacture
a production Clone or serialize private RNG.

| Guide / verified strength | Applied decision and proof |
| --- | --- |
| API-23 MUST; API-39 MUST | Match `CoreError` variant, actual instance and edited parameter; exercise both spawn and registered setters before checking no mutation. Tests must reject silent legacy clamping/snapping and context loss. |
| TD-05 MUST; AP-06 AVOID | Keep private engine/adapter fields private. Inline test snapshots observe them without production accessors; public integration uses existing outputs/observer and wrapper lifecycle. |
| AP-12/AP-18 AVOID | End World borrows before mutation; intentionally clone only test snapshots/independent traces. `resource_scope` avoids cloning a registry to escape borrow conflicts. |
| DC-05 MUST | The guideline means `# Errors` documentation for Result APIs, not a general examples rule. Existing production signatures stay fixed; error tests enforce the already documented/approved conditions. Correct the broad DC-05 description in the current evidence rather than perpetuating it. |
| Testing discipline; authoring §5 | Full per-sample/forward-reference assertions must distinguish mutation, state reset, double ticking and identity-only changes. No new framework, blanket mutation suite or arbitrary coverage target. |

The sketches below are source-checked **recommended shapes**, not compiled
drop-in implementations. Their observations and expected values are binding.
Private helper names and table-driven organization are local discretion.
No new concurrency/resource protocol applies: production remains unchanged;
child-process/temp-file ownership follows the existing compiled test runners.

## 1. Complete registered parameter and preservation proof (F-01, A-02)

In `rack.rs::tests`, retain the existing schema/validation tests and extend
their assertions. Compare every schema entry to the plan's ordered table:
all 32 names/directions/kinds/policies, all nine names/defaults, and exact
setter/getter registration names with no duplicates. The existing normal
dependency-tree check remains the standalone isolation oracle.

Invoke **every actual registered setter and getter**, checking readback against
explicit expected values. Recommended borrow shape inside this private test:

```rust
world.resource_scope(|world, registry: bevy_ecs::change_detection::Mut<ParameterRegistry>| {
    registry.set_parameter(world, entity, &kind, "root", ParameterValue::Int(7))?;
    assert_eq!(registry.get_parameter(world, entity, &kind, "root")?, ParameterValue::Int(7));
    Ok::<(), CoreError>(())
}).unwrap();
```

Build a reusable test-only deep observation: actual engine scales/names/roots,
range, length/position, probability, register/count; adapter write_base/current
modulation, every prior-gate bit, full cloned pending snapshot, and all 32
CurrentValues keyed by declared port name. Snapshot in a short immutable borrow
scope, then release it before setters/update. Do not compare only pending
presence. Entity handles are identity aids, not musical output expectations.

Use compatible valid operations and assert both aliases after each length edit:
all nine selector members `[2,3,4,5,6,8,10,12,16]` with positions `[0..8]`;
positions select that exact list. Write probability 0/0.25/1; main/secondary
root 0/7/11; supported custom intervals `[0,2,5,7,9]` with embedded root 4 and
name, then explicit root 7; absent Scale name reads back `Some("Custom")`.
Set min to 0 then max to 127, and exercise equal range `64..=64` in a valid
order. Test spawn presence separately: length-only 12 selects position7 rather
than redundant default8; length12+position2 selects length4; embedded nonzero
Scale roots survive without explicit root, explicit roots win when supplied,
for both quantizers. Do not apply redundant schema defaults as supplied values.

Drive this rejection population through **both spawn and registered setters**:

| Population | Expected error / setup |
| --- | --- |
| Each parameter's other existing ParameterValue variants (Float, Int, Bool, String, Scale) | Reject every wrong variant with actual instance and that parameter. Provide one well-formed payload per wrong variant. |
| Float probability -0.01, 1.01, NaN, +infinity, -infinity | InvalidParameterValue/write_probability, not clamped success. |
| Length -1, 0, 1, 7, 9, 17, i64::MIN/MAX; position -1, 9, i64::MIN/MAX | InvalidParameterValue for the edited name; validate before narrowing. |
| Both roots -1/12 and both endpoints -1/128 | InvalidParameterValue for the edited name. |
| Scale root12, empty intervals, `[2,1]`, `[1,1]`, `[12]`, for both scales | InvalidParameterValue for the Scale parameter; no normalization. |
| Spawn min85 only, max35 only, and pair min100/max90 | At fixed decode order, inverted range identifies note_min for min-only and note_max for max-only/pair. Setter min85 against max84 and max35 against min36 attribute the edited endpoint. |
| Invalid known probability plus unknown alpha/zeta, opposite insertion orders | UnknownParameter actual instance/alpha in both maps, before known decoding. Also include zeta alone with invalid write_probability to distinguish unknown-first from sorted-interleaved decoding. |

For each failed spawn count Module and Port components **and all entities**
before/after in a World with an existing unrelated entity: unchanged, with no
orphan module/port. No partial candidate entities may survive.

For each invalid setter establish non-default configuration, nonzero held
modulation, high edge history and a real pending step; assert the deep observation
is identical immediately after rejection. Then resume the same scripted inputs
and compare full pending/published/future seeded snapshots with an untouched
control (and the separately configured standalone reference). This catches
hidden RNG advancement that getters cannot expose. All registered accessors
must return UnknownModule for a missing entity, a bare non-TM entity, and a
non-TM Module entity; no panic. Use private tests, not new production hooks.

In `engine.rs` and `quantizer.rs` **existing cfg(test) sections only**, complete
getter checks after their respective setters, including both length forms,
replacement scales/nonzero roots and the owned range. This proves actual state,
not only constructor defaults. No standalone algorithm changes.

## 2. Complete real publication/held-value proof (F-02, A-03)

Inside `rack.rs::tests`, feed a deliberately constructed StepOutputs snapshot
through the **real `publish_step` path** (for example by inserting `pending`
then running Produce via the complete schedule). Use distinct scalar/array
sentinels, `Some(0)` versus None and `Some(127)` combinations for all three
optional fields. Assert every output entity's actual value: absent fields
Raw0/validfalse, Some0 Raw0/validtrue, all others exact; steptrue. On the next
no-action Produce, stepfalse and every other output identical. Test all three
optional fields as absent at least once; testing only `optional_output` is
insufficient. Keep the previously published snapshot while testing an idle run.

In the umbrella `assert_delayed_standalone_match`, compare **all 24 samples**.
Sample0: declared initial zero/false output values. Independently tick the
reference on clock rises0,2,... and publish its preceding pending snapshot at
sample1,3,... . On sample2,4,... retain the last snapshot with only stepfalse.
Compare all28 named values and exact sample indices at every observation.
Do not calculate expected idle values by reading the previous adapter row.
Retain the separate standalone engine and 12-tick oracle for each TM.

## 3. Action priority, modulation and paused high inputs (F-03, A-04)

Use the real CorePlugin schedule and private input entities in `rack.rs`.
Construct a separately seeded/configured standalone engine; explicitly drive
its `tick`, `move_step`, `reset` from the scripted expected actions, not from
adapter observations. For example:

| Executed sample | clock/reset/move | Expected Consume action |
| --- | --- | --- |
| 0 | true/false/false | tick; count1, pending first tick |
| 1 | true/false/false | none; Produce publishes tick once |
| 2 | false/false/false | none; Produce stepfalse, held scalars |
| 3 | true/false/true | move wins; count stays1, pending move |
| 4 | true/false/true | none; Produce publishes move once, losing clock does not replay |
| 5 | false/false/false | none |
| 6 | true/true/true | reset wins; register/count0, pendingNone; held scalars survive |
| 7 | true/true/true | none; losing clock/move do not replay |
| 8 | false/false/false | none |
| 9 | true/false/false | tick on continued RNG/config after reset |
| 10 | false/false/false | Produce publishes that tick |

Compare pending **full StepOutputs** and all published ports at every sample.
Move's divider outputs are false but do not reset/advance divider internals;
follow with enough true ticks to distinguish continued divider phase from reset.
Add separate reset-only and move-only cases so priority cannot hide a broken
individual path. Verify reset retains settings and future seeded RNG sequence;
never compare it with a newly constructed/reseeded engine as the expected reset.
Wide clock high must span at least two executed samples and produce one tick.

For modulation, base0.5 and held offsets -1, -0.25, 0, +0.25, +1 produce
targets0,0.25,0.5,0.75,1. Repeat each for several samples, check actual getter
and next snapshot, and return offset to0. Under held +0.25, registered base
edit to0.25 changes getter synchronously to0.5; a preexisting pending snapshot
keeps its old probability, the following step reflects0.5. Include invalid
base edits preserving the full dynamic observation.

Exercise **real two-cable Sum** into write_modulation: test-only Bipolar output
ports on unrelated modules, `Cable { source_port, target_port, transform: None,
enabled: true }`, then run CorePlugin's propagation/merge/consume chain. Use
sources +0.75/+0.75 (effective1), -0.75/-0.75 (effective0), and +0.25/-0.25
(effective base0.5), repeat while held. Use existing spawn helpers; no custom
merge or production source module. Observe the merged input and engine target.
Source-port values are held; no new hot-path allocation is introduced.

Pause private apps separately with clock, reset and move held high **after**
their edge has been consumed. Set SimulationRunning false, run zero/nonzero
requested samples through the existing schedule, check complete deep state and
SampleTime unchanged; restore true with the same input levels and compare the
continued run with an uninterrupted executed-sample reference. No invented
rise on resume. For pending stop, stop after a Consume-created snapshot, leave
the next execution gated, then prove publication once followed by stepfalse.
If stopping during an executing sample, core latching finishes all five phases;
do not expect a partial phase pause or add local SimulationRunning checks.

## 4. Public continued lifecycle and real seed negatives (F-03/F-04, A-04/A-05)

Extend the existing public fixture runners in `turing_machine.rs`. Retain all
Manual/Live grouping comparisons and fresh child byte equality. Generalize the
standalone reference to an explicit master seed and instance name; configuration
is still written independently from the fixture's known settings.

Replace the name control's whole-map inequality with corresponding contents:

```rust
let baseline_rows = &baseline.trace.rows["tm_primary"];
let renamed_rows = &renamed_run.trace.rows["tm_renamed"];
assert_eq!(baseline_rows.len(), renamed_rows.len());
assert!(baseline_rows.iter().zip(renamed_rows).any(|(a, b)| a.values != b.values));
assert_eq!(baseline.trace.rows["tm_secondary"], renamed_run.trace.rows["tm_secondary"]);
// Also compare renamed_rows to the independent standalone engine seeded from
// ModuleSpawnContext::new(42, "tm_renamed"), including held/idle samples.
```

For changed master seed43, compare output values for the same primary/secondary
names and verify the new independent standalone expectations. The name control
must reject CRC's identity-only counterexample; do not include instance names
or differently sized populations in the content inequality oracle.

For unrelated allocation, compare the baseline with a patch adding an extra
instance of the already installed no-port TraceObserver before the TMs, and a
patch removing the existing `trace_unrelated`; retain both TM names/config/
cables/seed. Trace rows and SampleTime must match exactly. Preserve reversal
control separately. The tracked RON fixture stays unchanged; perturb its clone
only. Retain a changed-seed/name check in fresh child-process runs, using the
existing environment/current_exe pattern if adapting it; keep exact canonical
rows and nonzero child failures visible, no parser that hides differences.

Replace the two-sample lifecycle assertion with a complete24-executed-sample
subject and uninterrupted reference. Subject: load/install twice/start Manual,
advance1 (pending tick), stop, assert advance0 and advance3 return0 with unchanged
SampleTime; reinstall Clock/TM after progress; start and advance23. Only then
consume each wrapper with `into_app` and compare full Trace and SampleTime to
the reference, including every standalone expectation. Also pause after a
published step and at a Clock-high boundary. Assert exact step indices, so
duplicate systems/reset/RNG/pending loss cannot pass. Do not reuse a consumed
wrapper or call its private world methods from this integration file.

## 5. Preserve and repair evidence (F-05, A-06)

CRC has retained byte-identical iteration00 reports/intake/results in
`artifacts/iteration00/`; its old raw logs and issued prompt remain immutable.
Do not change these or CRC review/probe/receipts. Append iteration01 intake
and test evidence; update the canonical close to the current candidate with
the current prompt identity, six-row walk, full commands/results and explicit
`## Bubble-up to arc` disposition. Distinguish test gaps from demonstrated bugs.
Include this standalone field, replacing the placeholder with the final
40-character source candidate hash enclosed in one pair of backticks:

```text
Source commit: `<40-character final source hash>`
```
Link the preserved initial report; do not relabel earlier unsupported claims
as accepted evidence. Correct current guideline descriptions where misnamed.

Append new rows to `artifacts/validation-results.tsv` and use new raw log names
so prior failed/retried attempts remain. Record actual dirty source state for
pre-commit checks and exact final source/evidence commits; avoid circular claims
that a report contains its own commit hash. Current ledger stays **open** with
iteration01 proposed-done notes until CRC accepts. Do not edit slice/arc/project
plans, status or either prompt from the CC execution context.

Use the existing checker already updated by CRC: default mode checks the issued
packet, `--require-return` additionally checks current return identity/row walk/
bubble-up. Run both and all same-predicate negative controls after updates.
Structural success is not Rust behavior or acceptance. If the checker itself
misdiagnoses a valid return, retain the error and return to CRC; do not relax it.

## Validation and exact path fences

Rerun the full initial matrix from **source worktree**, preserving every exit,
ignored-test denominator, restricted failure and required successful host retry:

```bash
cargo test --locked -p oxurack-mod-turingmachine
cargo tree --locked -p oxurack-mod-turingmachine --edges normal
cargo test --locked -p oxurack-mod-turingmachine --features rack
cargo test --locked -p oxurack --test turing_machine
cargo test --locked -p oxurack --no-default-features --test turing_machine
cargo test --locked -p oxurack
cargo test --locked -p oxurack --no-default-features
cargo test --locked -p oxurack-core
cargo test --locked --workspace --all-features
cargo test --locked --workspace --no-default-features
cargo clippy --locked --workspace --all-features --all-targets -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS='-Dwarnings' cargo doc --locked --workspace --all-features --no-deps
RUSTDOCFLAGS='-Dwarnings' cargo doc --locked -p oxurack --no-default-features --no-deps
make check
cargo outdated --root-deps-only --ignore-external-rel
git diff --check
```

From the Slice02 directory run:

```bash
python3 artifacts/verify-planning.py
python3 artifacts/verify-planning.py --require-return
```

The added focused no-default integration command ensures repaired public tests
run with both graph modes. Counts are observations, not predeclared pass quotas.
Known baseline restrictions: four RT MIDI initialization failures under the
sandbox, three ignored RT timing tests, crates.io DNS under restricted access
and `block v0.1.6` future-incompatibility warning. Do not convert them into
unqualified passes or suppress them. Preserve complete host retry receipts.

Authorized source changes, **test-only regions**:

- `crates/oxurack-mod-turingmachine/src/rack.rs` cfg(test) module only;
- `crates/oxurack-mod-turingmachine/src/engine.rs` cfg(test) module only;
- `crates/oxurack-mod-turingmachine/src/quantizer.rs` cfg(test) module only;
- `crates/oxurack/tests/turing_machine.rs` integration-test file only.

Audit the diff against `9ba55dd`: every byte outside these test regions must
remain unchanged, and there must be no other changed source path. Report the
audit and actual changed paths; do not change files merely to reach four paths.
No Cargo/lock/fixture/README/core/Clock/RT edits, lint suppressions, ignores,
weakened assertions, production hooks, MIDI sink work, merge, push or release.

Authorized planning changes within this slice only:
`artifacts/intake.md`, `artifacts/test-evidence.md`,
`artifacts/validation-results.tsv`, new `artifacts/logs/iteration01-*.log`,
`closing-report.md`, and `ledger.md` proposed-done notes only. CRC's issued
checker and historical/probe/review files are protected. Use exact explicit
file lists with `git commit --only`: source first, planning evidence second.
Finish both worktrees clean, with no unrelated work included or discarded.

Return current prompt path, exact commits, changed paths/protected-region audit,
row-by-row observations/test names, complete gates and retries/ignored counts,
intake/readback, Rust decisions, deviations, limitations and bubble-up findings.
Unsupported or failing criteria stay open. Any production failure or need for
new scope/API/architecture returns to CRC/CDC through Duncan before edits.

## Retention and readiness

CRC's independently reproduced initial subset in `crc-verification.md` may be
proposed for retention if its production/fixture/dependency inputs stay exact.
CC still runs all assigned commands; old doer attestations alone cannot close
a row. Final CRC decides what applies to the corrected candidate.

The slice plan records the author's six readiness checks. This prompt supplies
source-grounded test shapes, exact discriminating values, actual ownership paths
and bounded reading. The unchanged behavior contract leaves no architecture
decision to CC; local test organization may vary without weakening observations.
