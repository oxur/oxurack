# Slice02 implementation intake

## Assignment and state

The active assignment is Arc10 / Slice02, Turing Machine rack adapter. The
implementation source checkout is `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`, branch
`codex/oxurack-module-behavior-installation`, at source baseline
`a19cfd1a134c5934d270dc0ba72334d8f167fe2f`. The planning checkout is
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch `planning`, at
`eb45f3bee2efacbf7e735c9b74db8408d4a01876` before this intake. Both were clean
at preflight. The predecessor planning identity is `ef9d99f...` as named by the
prompt; the current planning head is the later Slice02 opening state.

## Required reading loaded

The active prompt, Slice02 plan, ledger, parent design sections 1, 2, and 5,
and `cdc-directive07.md` were loaded in full before implementation. The
prompt's required source files were read in bounded chunks: the Turing Machine
crate manifest, README, all source modules, the core module/parameter/port/
value/scale/tick/error/rng contracts, the umbrella manifest and public API, and
the Clock adapter, Clock integration test/fixture, seeded-spawn-context tests,
and sample-pipeline test. The prompt and plan are the authority for current
scope; the Clock and seeded tests are working patterns only.

The required guidance was reloaded before Rust edits: collaboration-framework
implementation-prompt authoring (concrete implementation recipe and test
oracles), testing discipline, and Rust guidelines 2.0.0 anti-patterns plus
API-23, API-39, TD-05, and DC-05. The concrete decisions applied here are:

* decode and validate all caller-controlled spawn/setter data before mutating
  ECS state or calling the legacy clamping setters; return the existing typed
  `CoreError` variants with the edited parameter named;
* keep adapter state fields private and expose only the requested validated
  engine getters; use cached port entities and a fixed snapshot helper so the
  sample loop performs no per-step allocation;
* test phase ownership and lifecycle through the public rack and a standalone
  seeded engine oracle, with negative fixtures that distinguish unknown keys,
  wrong variants, malformed scales, range errors, held gates, and reset/move/
  clock priority.

## Contract readback

The adapter is opt-in behind `rack` and must preserve the standalone engine and
the existing `midi-io` feature. It owns exactly 32 ports: four inputs and the
28 named outputs in the prompt, plus the nine parameters and their defaults.
The two `Scale` types are separate: core scales are validated structurally
(root 0..=11, nonempty, every interval below 12, strictly increasing and
unique) and then converted field-for-field into the engine scale; no legacy
normalisation may hide malformed input. The independent scale-output root and
the main root are preserved separately.

The easily missed runtime rule is phase ownership. Produce runs first and
publishes a pending snapshot from the previous sample; Propagate and MergeInputs
then deliver the current clock; Consume detects only false-to-true edges and
stores `tick`, `move_step`, or reset effects for the following Produce. Reset
has priority over move, which has priority over clock, and lower simultaneous
edges are consumed. A stopped sample leaves pending work and held output values
untouched; resume publishes the pending snapshot. A held-high gate cannot
retrigger after stop/resume.

Construction must use `ModuleSpawnContext::instance_seed()` through
`TuringMachine::with_seed` for patch loads; the legacy `spawn` path constructs
the zero-seed context and uses the same helper. Failed construction and failed
runtime parameter edits must leave the world and existing engine/ports/edges/
pending state unchanged. Runtime write modulation is a held bipolar offset,
clamped once and reconciled from the base write shadow each running sample, so
it cannot accumulate.

## Scope and evidence home

Source changes are limited to the ten paths named by the prompt. Planning
changes are limited to this intake, raw logs, `test-evidence.md`,
`validation-results.tsv`, `closing-report.md`, and the six proposed-done ledger
rows. No core, Clock, MIDI, graph, unrelated test/fixture, Makefile, prompt,
plan, directive, or design source may change. The source implementation commit
precedes the planning evidence commit; each uses an explicit path allowlist.

Validation retains the exact required Cargo, Clippy, rustdoc, `make check`,
outdated, diff, and planning-validator commands, their exits and outputs, plus
ignored-test denominators and any restricted-host retries. A green structural
check or replay is recorded as evidence for its checked property and is not
promoted to semantic acceptance or independent CRC/CDC acknowledgement.

## Files and receipt pointers

* Prompt: `cc-prompt.md` (required-full; active authority).
* Plan: `slice-plan.md` (required-full; A-01..A-06 contract).
* Ledger: `ledger.md` (required-full; rows remain `open` until independent
  review).
* Raw command receipts: `artifacts/logs/`.
* Test and validation evidence: `artifacts/test-evidence.md` and
  `artifacts/validation-results.tsv`.

## Corrective iteration01 intake — 2026-10-10

### Assignment identity and live state

Current assignment: `arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt-iteration01.md`.
Prompt SHA-256: `925395ed475225a96d39b3ee303eda5b875ea4e32b3855096649cacc88ee8fa2`.
The prompt authorizes corrective pass 1/5 as test/evidence work only. Source is
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, HEAD
`9ba55dd0a4c8aa7d67fe26c30f878d89ead34aa2`; clean at intake. Planning is
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch `planning`, HEAD
`32e693466827eeab1c842931d74772820c2f6032`; clean at intake. The root checkout
is `main` at `a7ce6e6`; the assignment source is the separate source worktree.
The predecessor prompt and iteration00 evidence remain immutable. Planning
predecessor named by CRC is `2eb5ac138fedf21d4185d1cc2f1dd3df84c4972d`.

### Required reads and extents

* Current iteration01 prompt, Slice02 plan, ledger, CRC verification, project
  status, project plan, CDC directive07, and Arc10 design §§1, 2, 5 were read
  in full. The required predecessor `cc-prompt.md` sections (Steps 1, 3–7,
  validation/evidence/commit protocol, and stop conditions) were read; unrelated
  initial-assignment history remains reference-only. Iteration00 test evidence,
  validation matrix, checker and complete six-row CRC findings were inspected.
* Source at `9ba55dd`: `rack.rs` production declarations/schema/spawn/systems/
  validation/setter/getter implementation (including bounded reads through
  `decode_scale`, `set_parameter`, and `apply_candidate`), the complete
  `#[cfg(test)]` module, complete umbrella `tests/turing_machine.rs`, and the
  unchanged TM RON fixture. `engine.rs` operations/getters and its existing
  getter/move/reset tests; `quantizer.rs` getter implementation and configured
  getter test; `outputs.rs`/`length.rs` definitions were consulted. No source
  drift or mismatch with the pinned baseline was observed.
* Core readback covered `ParameterRegistry` setter/getter signatures,
  `ParameterValue` and module context/spawner/registration; the complete
  `OxurackModule` registration contract; `build_core_plugin`'s chained
  Produce→Propagate→MergeInputs→Consume→Observe schedule and latched running
  gate; `SimulationRunning`/`SampleTime`; port spawning/current values; cable
  fields; Scale shape; and public Oxurack install/load/start/stop/advance/
  consuming `into_app` paths. `resource_scope` is the verified registry borrow
  boundary. The prompt's named Bevy APIs were read from the checked-in locked
  source/dependency surface; no new API is required.
* Installed guidance: collaboration-framework 2.6.0 authoring guide §5,
  required-reading/CC-intake and corrective-iteration sections; testing
  discipline 2.6.0 read in full; rust-guidelines 2.0.0 entrypoint and full
  anti-patterns chapter, plus API-23/API-39, TD-05, and DC-05. The exact
  versions are present in installed version history.

### Contract readback and applied decisions

F-01 through F-05 in CRC review01 identify missing proof, not a demonstrated
production behavior defect. A-01–A-06 remain open. Source edits are restricted
to test code in `rack.rs`, `engine.rs`, `quantizer.rs`, and
`crates/oxurack/tests/turing_machine.rs`; production, manifests, lockfile,
fixture, prompts, plans and the protected CRC evidence cannot change. A newly
exposed production failure stays intact as evidence and returns to CRC before
any production edit.

For A-02, a deep private observation must include actual engine configuration
and dynamic state, adapter base/modulation/edge history, full pending
`StepOutputs`, and all 32 named port values. Registry tests must invoke each
registered setter and assert its getter result. The `ParameterRegistry`
contains function pointers keyed by module kind/name, so private rack tests can
exercise the actual registered functions using `World::resource_scope`; this
avoids cloning or weakening the registry API. Invalid input must be run through
both spawn and registered setters, with spawn entity counts and subsequent
seeded output checked as well as immediate deep-state equality.

For A-03, `Produce` is before `Propagate`, `MergeInputs`, and `Consume` in one
latched sample. TM Consume therefore records a completed engine action as
pending, and the next executed Produce publishes all 28 output values together
with `step=true`. Idle Produce clears only `step`; optional `None` publishes
Raw(0)/valid=false, while `Some(0)` publishes Raw(0)/valid=true. A complete
24-sample standalone oracle must calculate expected held values independently
and compare every sample, including initial zero values and idle samples.

For A-04, Gate values are levels and only false→true transitions are actions;
all three edge histories advance before reset > move > clock priority, which
consumes losing edges. Reset calls the existing engine reset, preserving its
settings/RNG; pause gates the complete five-phase sample and must leave
SampleTime and all private state unchanged. The continuing reference must
advance only on executed samples. Modulation is a held offset; repeated samples
reconcile `clamp(base + offset)` and cannot accumulate. The test will exercise
real two-cable Sum propagation/merge using unrelated Bipolar source modules.

For A-05, the name negative must compare equal-length corresponding `values`
rows for `tm_primary`/`tm_renamed`, also compare the renamed rows to an
independently configured engine seeded by the renamed context, and retain
secondary equality. A new unrelated no-port observer and removal of the
existing observer each get their own baseline comparison. `into_app(self)`
consumes the public wrapper, so all stop/resume/reinstall calls must finish
before conversion; no former wrapper call or private world access follows it.
Renaming a key alone cannot establish a seed change because unequal map keys
make whole-map inequality true even when sample contents are identical.

Guidance decisions: API-23 uses actual typed `CoreError` variants/instance and
parameter; API-39 tests strict boundary validation before legacy clamping;
TD-05/AP-06 keep adapter state private and observe it only from existing unit
test privacy; AP-12/AP-18 shorten immutable borrows and intentionally clone
only snapshots/traces needed as independent observations; DC-05 refers to
`# Errors` on Result-returning APIs, not a general examples obligation. No
production signature is changed. Testing discipline requires real paths,
discriminating values and complete accounting of failures/ignored counts.

CRC's iteration00 partial reproduction may be retained only where source,
fixture, dependency and behavior inputs are unchanged; it does not waive this
prompt's full matrix. Restricted CoreMIDI/DNS behavior and 3 ignored RT tests
must remain visible beside successful host retries. Planning changes are
limited to iteration01 intake/test evidence, new iteration01 raw logs, the
validation matrix, canonical closing report and proposed-done ledger notes.
The current checker plus all same-predicate negative controls are required;
structural success is not behavioral acceptance. Source commits precede
planning commits, both use explicit `git commit --only` paths, and both
worktrees must end clean. No row closes in this execution context.
