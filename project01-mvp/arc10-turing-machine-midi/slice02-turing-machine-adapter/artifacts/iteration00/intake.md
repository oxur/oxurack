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
