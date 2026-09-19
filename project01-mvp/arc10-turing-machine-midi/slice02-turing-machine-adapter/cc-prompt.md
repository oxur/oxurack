# CC initial assignment — Turing Machine rack adapter

## Assignment identity and authority

Implement project01-mvp / arc10-turing-machine-midi /
slice02-turing-machine-adapter. Exact assignment:
`arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt.md`.
Three-Contributor Workflow, Expedited Mode. Source baseline:
`a19cfd1a134c5934d270dc0ba72334d8f167fe2f`; planning predecessor:
`ef9d99fe7ab4f38ca46a069c4e87cd4d2e10873f`. CC implements and records
proposed-done evidence; Sofie/CRC independently reproduces A-01–A-06.

The complete [slice plan](slice-plan.md), [ledger](ledger.md), parent
[design](../design.md) §2/§5 and [directive07](../../cdc-directive07.md) are
binding. Implement the full adapter inventory. A reduced audible subset, a
different Scale policy, or deferred port/action mapping requires CRC/CDC
resolution before source edits continue.

## Required reading and intake

Use these aliases in your notes and expand them in recorded commands:

- `P=/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp`
- `S=/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
- `F=/Users/oubiwann/.codex/skills/collaboration-framework`
- `R=/Users/oubiwann/.codex/skills/rust-guidelines`

Before edits, record source/planning HEAD, branches and cleanliness; confirm
this prompt is the current Slice02 assignment; create `artifacts/intake.md` with
loaded extents and a concise contract readback. Recover every truncated read.

Read in this order:

1. This prompt, `slice-plan.md`, `ledger.md`, parent `design.md` sections 1, 2
   and 5, and `cdc-directive07.md`: required-full.
2. Source crate required-full: TM Cargo.toml, README, and every file under
   `crates/oxurack-mod-turingmachine/src/`. Record why `midi_io.rs` is retained
   but excluded from the rack path.
3. Framework definitions required-full: core `module.rs`, `parameter.rs`,
   `port.rs`, `value.rs`, `scale.rs`, `tick.rs`, `error.rs`, and `rng.rs`;
   umbrella Cargo.toml and the install/load/driver/start/stop/advance/into_app
   portions of `crates/oxurack/src/lib.rs`.
4. Working patterns required-full: Clock `rack.rs`, umbrella `clock.rs` test and
   Clock RON fixture; core and umbrella seeded-spawn-context tests; sample
   pipeline tests covering Manual/Live grouping, order, stop and reinstall.
5. `R/SKILL.md` and `R/guides/11-anti-patterns.md`: required-full before Rust
   changes. Apply Rust 2.0.0 sections API-23/API-39 (fallible boundaries and
   error context), TD-05 (domain/newtype/state design), and DC-05 (public API
   docs/examples). Record the concrete decisions. Do not merely name sections.
6. `F/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md`
   sections Concrete implementation recipe and Design the tests and their
   oracles; `F/knowledge/testing/guides/01-testing-discipline.md`: required-full.

Your readback must explain the five-phase one-sample latency, why wide Gate high
is not a clock event stream, why core Scale and engine Scale cannot be aliased,
why adapter validation is stricter than legacy clamping, and why comparison to
a separately configured standalone engine is the principal oracle.

## Step 1 — package and public inspection getters

Add optional `rack` dependencies on `oxurack-core`, Bevy app and Bevy ECS using
the exact workspace-adjacent versions/settings already used by Clock. Add
`rack = [...]` without changing default, `midi-io`, or `serde` behavior. Gate a
new private `rack` module and publicly re-export `TuringMachineModule` only when
the feature is enabled. Update crate docs/README to name the adapter, complete
port/parameter inventory and sample-delayed semantics.

Add read-only getters to `Quantizer`, then delegate from `TuringMachine` exactly
as specified in the plan. A suitable compiled shape is:

```rust
impl Quantizer {
    #[must_use]
    pub fn scale(&self) -> &Scale { &self.scale }

    #[must_use]
    pub const fn root(&self) -> u8 { self.root }

    #[must_use]
    pub fn range(&self) -> RangeInclusive<u8> { self.range.clone() }
}

impl TuringMachine {
    #[must_use]
    pub fn length_position(&self) -> usize { self.length.position() }

    #[must_use]
    pub fn scale(&self) -> &Scale { self.quantizer.scale() }

    #[must_use]
    pub fn root(&self) -> u8 { self.quantizer.root() }

    #[must_use]
    pub fn note_range(&self) -> RangeInclusive<u8> { self.quantizer.range() }

    #[must_use]
    pub fn scale_output_scale(&self) -> &Scale { self.scale_quantizer.scale() }

    #[must_use]
    pub fn scale_output_root(&self) -> u8 { self.scale_quantizer.root() }
}
```

Exact formatting is local discretion. Return types, ownership and observed
state are binding. Test every new getter after existing setters; include custom
Scale intervals/name, nonzero roots, range and both length forms.

## Step 2 — adapter state, schema and spawn

Create `src/rack.rs`. Use cached Entity handles, not child/string scans during
systems. The private state should make ownership visible. This sketch omits the
individual output fields only for space:

```rust
#[derive(Component, Debug)]
struct TuringMachineState {
    engine: TuringMachine,
    base_write_probability: f32,
    write_modulation: f32,
    previous_clock: bool,
    previous_reset: bool,
    previous_move: bool,
    pending: Option<StepOutputs>,
    ports: TuringMachinePorts,
}

#[derive(Debug)]
struct TuringMachinePorts {
    clock: Entity,
    write_modulation: Entity,
    reset: Entity,
    move_step: Entity,
    step: Entity,
    // one Entity for every remaining output in slice-plan.md
}
```

`TuringMachineModule::spawn` must remain as legacy compatibility, but patch
construction must override `spawn_with_context` and use only
`TuringMachine::with_seed(context.instance_seed())`. Factor both through a
private validated helper; never call `new()` on the context path. Legacy
`spawn` constructs `ModuleSpawnContext::new(0, instance_name)` and uses that
derived instance seed, so direct trait callers are deterministic and remain
name-separated without inventing a second derivation.

Decode parameters into a private candidate before creating entities. First
find the lexically first unknown key without decoding. Then decode in fixed
plan order. Validate signed integers before casts, floats for finiteness/range,
both range endpoints together, and Scale structure. A useful Scale conversion
shape is:

```rust
fn engine_scale(
    instance: &str,
    parameter: &str,
    scale: &oxurack_core::Scale,
) -> Result<(crate::Scale, u8), CoreError> {
    validate_core_scale(instance, parameter, scale)?;
    let name = scale.name().unwrap_or("Custom").to_owned();
    Ok((crate::Scale::new(scale.intervals().to_vec(), name), scale.root()))
}

fn core_scale(scale: &crate::Scale, root: u8) -> oxurack_core::Scale {
    oxurack_core::Scale::new(
        scale.intervals().iter().copied(),
        root,
        Some(scale.name().to_owned()),
    )
}
```

Validation requires root <=11, nonempty intervals, every interval <12 and
strictly increasing unique order. Preserve every supported interval/root.
Do not compare built-in names or collapse custom scales into an enum.

Build all 32 ports from the exact plan table. Use loops to construct arrays of
six/eight Entity handles if that keeps the schema and state auditable. Static
schema ordering must be inputs in plan order, then outputs in plan order.
Use `pulse_1`…`pulse_6` and `gate_1`…`gate_8`, never zero-based public names.

Apply a validated candidate in fixed order after seeded engine construction:

```text
set_write
set_length
set_length_position        # only when supplied; wins over length
set_scale + set_root       # embedded root
set_root                   # explicit root wins when supplied
set_note_range
set_scale_output_scale + set_scale_output_root
set_scale_output_root      # explicit root wins when supplied
```

Defaults are the engine defaults. Do not apply the schema's redundant length
position/root defaults as if explicitly supplied; track presence. Failed spawn
must leave Module and Port counts unchanged.

## Step 3 — registered setters/getters and mutation safety

Register setters/getters for all nine schema parameters. Each setter:

1. resolves the actual module instance name and private state fallibly;
2. decodes into locals without mutation;
3. constructs and validates the complete affected candidate;
4. applies the engine operation and adapter shadow update only after success.

Wrong entity/type returns `UnknownModule`; wrong value variant/range/Scale
returns `InvalidParameterValue` with the actual instance and edited parameter.
For range edits, attribute `min > max` to the endpoint being edited. For Scale,
the Scale parameter owns malformed embedded root/interval errors. Invalid edits
must preserve the engine, base/modulation, edge bits, pending snapshot and every
held port.

`write_probability` stores the new base and synchronously reconciles the engine
against the held adapter modulation:

```rust
fn reconcile_write(state: &mut TuringMachineState) {
    let target = (state.base_write_probability + state.write_modulation)
        .clamp(0.0, 1.0);
    let delta = target - state.engine.write_probability();
    state.engine.modulate_write(delta);
}
```

The getter returns `state.engine.write_probability()`, the effective actual
value. Other getters reconstruct actual engine state; never return schema
defaults or stale candidate fields. `length` and `length_position` both observe
the same selector. Scale getters use the conversion above with actual roots.

## Step 4 — Consume action system

Register one Consume system and one Produce system in the supplied schedule.
Do not hardcode Update/FixedUpdate and do not inspect SimulationRunning.

Consume reads exact variants from the cached input ports. A wrong raw-world
variant is an invariant violation and may use a clear `expect`/panic; validated
patch routing prevents it. Clamp a summed Bipolar modulation to -1..=1 before
storing it, reconcile effective write every running sample, then compute rising
edges and update all three edge histories before applying priority.

The action core should follow this shape:

```rust
let reset_rise = reset && !state.previous_reset;
let move_rise = move_gate && !state.previous_move;
let clock_rise = clock && !state.previous_clock;
state.previous_reset = reset;
state.previous_move = move_gate;
state.previous_clock = clock;

if reset_rise {
    state.engine.reset();
    state.pending = None;
} else if move_rise {
    state.pending = Some(state.engine.move_step());
} else if clock_rise {
    state.pending = Some(state.engine.tick());
}
```

The order and simultaneous-edge consumption are binding. A held-high Gate does
not retrigger. Reset retains RNG and settings because the standalone method does;
do not reconstruct the engine. Stop/resume preservation comes from core's
latched schedule gate, so adding local running logic is a defect.

## Step 5 — coherent Produce publication

At the beginning of each Produce run, take the pending snapshot. With `None`,
set only `step=false`. With `Some`, set `step=true` and write every mapped field
before the system iteration finishes. Use one helper so unit tests can audit a
single complete mapping. The Option mapping must be explicit:

```rust
fn option_raw(value: Option<u8>) -> (Value, Value) {
    match value {
        Some(value) => (Value::Raw(u16::from(value)), Value::Gate(true)),
        None => (Value::Raw(0), Value::Gate(false)),
    }
}
```

Publish length with a checked conversion justified by the validated selector
invariant. Do not allocate a per-step map/vector, emit MIDI, or clear held
snapshot values when `step=false`. The exact expected latency for the integrated
fixture is Clock high in Produce sample 0, TM consumes/runs at sample 0, and TM
publishes snapshot plus step in Produce sample 1.

## Step 6 — focused adapter tests

Tests inside `rack.rs` may inspect private state. Cover:

- exact 32-port schema: direction, kind, merge policy and public order;
- exact nine-parameter schema/defaults and registered setter/getter names;
- lexically first unknown key before invalid known values in opposing map orders;
- every wrong variant, numeric boundary, malformed Scale, invalid range and
  failed-spawn zero-entity/zero-port mutation;
- every valid setter/getter, length/position aliasing, scale/root precedence,
  note endpoint edits, custom interval/root round trip and secondary quantizer;
- invalid setters preserve a deep observable snapshot of engine getters,
  edge/modulation/pending state and port values;
- identical context seed/config versus a separately configured standalone
  engine across at least 12 ticks, comparing complete `StepOutputs`/ports;
- note/velocity/scale-note `None` mapping through a direct publication-helper
  unit test even though current engine steps return Some;
- wide-clock rising-edge behavior, reset/move/clock priority, discarded lower
  edges, move divider/count behavior and reset held-output behavior;
- modulation at -1, 0, positive and summed/clamped values; repeat a constant
  value for multiple samples to reject cumulative drift.

Do not expose production test accessors. Use private helpers/components directly.

## Step 7 — real umbrella fixture and replay tests

Add the TM crate as an umbrella dev-dependency with `rack`. Add a real RON
fixture containing at least two accepted Clock instances and two TM instances,
explicit Clock parameters, distinct TM parameters/scales, cables from Clock
`pulse` to TM `clock`, and an unrelated module/consumer sufficient to perturb
allocation. Use master seed 42. No MIDI module appears yet.

In `crates/oxurack/tests/turing_machine.rs`, install Clock and TM twice before
load, run through public `Oxurack`, and record a complete named per-sample scalar
trace in Observe. Use 1000 BPM / 30 PPQN / width 1 for exact Clock rises
0,2,4,… and assert TM `step` samples 1,3,5,… . Record all 28 output values, not
only note/gate.

Build these discriminating runners:

```text
manual groups: [24] and [1, 2, 3, 5, 13]
controlled Live groups with same total using TimeUpdateStrategy::FixedTimesteps
reordered fixture: reverse module order and cable order, retain instance names
fresh child: run this test binary under a child-mode environment variable and
             print canonical lexical-instance/sample rows
```

Compare Manual/Live/reordered traces and SampleTime. For each named TM, build a
separate standalone engine with the known derived instance seed, apply the same
settings, tick on independently calculated Clock rise samples, and compare all
snapshot fields at the delayed step samples. Do not derive expected output by
reading adapter state or replaying the same helper.

Run the child trace twice and compare exact stdout bytes. Then change patch
master seed and an instance name independently and assert the relevant named
trace differs. Suppress ordinary test-harness noise using the established
current-executable child pattern; child nonzero status is a failure.

Add lifecycle cases:

- stop after Consume has created a pending snapshot but before another executed
  sample can publish it; stopped manual requests change no state/time; resume
  publishes exactly once and thereafter matches an uninterrupted executed-
  sample reference;
- stop while clock/reset/move inputs are held high and prove resume does not
  invent a second rise;
- reinstall Clock/TM after progress and prove one execution per instance,
  preserved state and unchanged trace relative to reference.

If the public wrapper cannot stop between phase systems, prove pending behavior
inside `rack.rs` with a core schedule and keep wrapper lifecycle at whole-sample
boundaries. Do not invent a new public phase-step API.

## Validation, evidence and commit protocol

Run and preserve exact commands/results for at least:

```bash
cargo test --locked -p oxurack-mod-turingmachine
cargo tree --locked -p oxurack-mod-turingmachine --edges normal
cargo test --locked -p oxurack-mod-turingmachine --features rack
cargo test --locked -p oxurack --test turing_machine
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
python3 artifacts/verify-planning.py
```

The normal TM dependency tree must remain free of Bevy/core. A separate feature
tree may show the optional adapter dependencies. Run workspace/Make/outdated
host retries when the restricted environment causes the known CoreMIDI, clock
permission or crates.io DNS failures; retain both attempts. Record ignored-test
denominators. No ignore, warning suppression, relaxed assertion or generated
coverage artifact belongs in source.

Write planning evidence only under this slice: `artifacts/intake.md`, raw logs,
`artifacts/test-evidence.md`, `artifacts/validation-results.tsv`, and
`closing-report.md`. Update the six ledger rows only as CC proposed-done language;
their status column stays open until CRC. Run the checker including its negative
controls after evidence updates.

Commit source first using `git commit --only` with the exact ten-path allowlist;
then commit planning evidence with an explicit file list. Do not blanket stage.
Both worktrees must finish clean. Return exact commits, changed paths, every
command/result including failures/retries, row-by-row disposition, Rust-guidance
application, deviations and bubble-up findings. No merge, push, release, device
interaction, Slice03 work or acceptance claim.

## Explicit stop conditions

Stop and return to CRC before widening scope if any of these occurs: core API or
signal-kind change appears necessary; the engine cannot expose actual state with
read-only getters; arbitrary supported Scale intervals/root cannot round trip;
full 32-port mapping cannot fit coherently; phase conflict requires a different
action order; or deterministic replay disagrees with a separately configured
standalone engine. Do not choose a smaller mapping or new architecture locally.
