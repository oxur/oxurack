# Slice02 — Turing Machine rack adapter

Version 1.2 — 2026-10-10. Status: closed; CRC accepted iteration01 at `b2ffb16`.
CRC authors and reviews, CC implements, CDC reviews Arc10 composition.
Current assignment:
`arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt-iteration01.md`.
Accepted source: `b2ffb16d342d9a6cc800bab2713813d9646fb505`, clean
`.worktrees/codex-oxurack-module-behavior-installation` on
`codex/oxurack-module-behavior-installation`.
Current planning predecessor: `2eb5ac138fedf21d4185d1cc2f1dd3df84c4972d`.
Authority: [directive07](../../cdc-directive07.md), [Arc10 design](../design.md),
and accepted [Slice01 Clock](../slice01-musical-clock/crc-verification.md).

## Outcome and boundary

Add an opt-in rack adapter to the existing standalone Turing Machine crate.
A seeded RON patch installs the existing engine through `ModuleSpawnContext`,
accepts Clock/reset/move edges plus write modulation, and publishes every
`StepOutputs` field as one coherent scalar snapshot on the following Produce
phase. Default standalone builds remain free of Bevy/core. The existing engine,
RNG algorithm and historical `midi-io` wrapper remain the musical authority.

This slice has no MIDI-event sink, RT queue, endpoint, note lifecycle, new core
signal kind, global transport, persistence checkpoint, LFO, Scope or UI. It
does not claim Arc10 T-01/T-02 composition until later MIDI slices assemble the
path. A fresh-process scalar trace here proves adapter seeding only.

Six rows A-01–A-06 in [ledger.md](ledger.md) define completion.

## Actual-source inventory and API amendments

The source at `a19cfd1` exposes exactly these engine operations:

| Engine surface | Adapter mapping | Validation / observable state |
| --- | --- | --- |
| `with_seed(u64)` | `spawn_with_context` uses `context.instance_seed()` | Patch-driven construction never calls `new()` |
| `tick()` | rising `clock` Gate input | At most one action/step per sample; advances divider and step count |
| `move_step()` | rising `move` Gate input | One register step; divider and step count unchanged |
| `reset()` | rising `reset` Gate input | Clears register/dividers/count; retains RNG and settings; emits no snapshot |
| `set_write(f32)` | `write_probability` Float parameter | Strict finite 0..=1 at adapter boundary |
| `modulate_write(f32)` | `write_modulation` Bipolar input | Each Consume computes `clamp(base + input, 0, 1)` and applies only the delta from actual engine probability |
| `set_length(usize)` | `length` Int parameter | Exact member of 2,3,4,5,6,8,10,12,16; no adapter snapping |
| `set_length_position(usize)` | `length_position` Int parameter | Strict 0..=8; if both spawn keys exist, position applies after length |
| `set_scale(Scale)` + `set_root(u8)` | `scale` core Scale plus `root` Int | Scale applies intervals/name/root; explicit root applies after scale |
| `set_note_range(RangeInclusive<u8>)` | `note_min` and `note_max` Int parameters | Each 0..=127 and min <= max; explicit pair built before mutation |
| `set_scale_output_scale(Scale)` + `set_scale_output_root(u8)` | `scale_output_scale` plus `scale_output_root` | Same conversion; explicit root applies after scale |
| `register_bits()` | inspection/output | Existing getter retained |
| `current_length()` | `length` getter/output | Existing getter retained |
| `write_probability()` | effective write getter/output | Existing getter retained; includes current modulation |
| `step_count()` | test/lifecycle inspection | Existing getter retained |

To support real parameter getters without shadowing engine truth, add these
read-only standalone methods and matching `Quantizer` accessors:

```rust
impl Quantizer {
    pub fn scale(&self) -> &Scale;
    pub fn root(&self) -> u8;
    pub fn range(&self) -> RangeInclusive<u8>;
}

impl TuringMachine {
    pub fn length_position(&self) -> usize;
    pub fn scale(&self) -> &Scale;
    pub fn root(&self) -> u8;
    pub fn note_range(&self) -> RangeInclusive<u8>;
    pub fn scale_output_scale(&self) -> &Scale;
    pub fn scale_output_root(&self) -> u8;
}
```

The returned range is an owned clone. These are observations only; existing
setters retain standalone clamping/snapping compatibility. Adapter validation
is intentionally strict before invoking them.

## Parameter schema and deterministic application

The adapter type is `TuringMachineModule`, kind `turing_machine`, display name
`Turing Machine`. Its complete parameter schema is:

| Name | Variant / default | Setter and getter |
| --- | --- | --- |
| `write_probability` | Float 0.5 | set strict base; getter returns actual effective engine probability |
| `length` | Int 16 | exact length; getter current exact length |
| `length_position` | Int 8 | strict rotary position; getter actual position |
| `scale` | Scale chromatic root 0 | set intervals/name and embedded root; getter rebuilds core Scale from engine |
| `root` | Int 0 | strict 0..=11; getter actual main root |
| `note_min` | Int 36 | strict 0..=127 and <= current/candidate max |
| `note_max` | Int 84 | strict 0..=127 and >= current/candidate min |
| `scale_output_scale` | Scale chromatic root 0 | set secondary intervals/name/root; getter rebuilds actual core Scale |
| `scale_output_root` | Int 0 | strict 0..=11; getter actual secondary root |

Spawn validation first rejects the lexically first unknown key without decoding.
It then decodes/validates in the table order and mutates only a local candidate.
Apply candidate operations in that same order after seeded engine construction.
`length_position` therefore wins when both length forms are supplied; explicit
`root` values win over roots embedded in their corresponding Scale values.
Wrong variants and invalid/cross-field values identify the actual instance and
responsible parameter. Failed spawn creates no module or port. Failed setters
leave engine, adapter base/modulation, edge history, pending snapshot and held
ports unchanged.

A supported core Scale has root <=11, a nonempty strictly increasing interval
list with every value <12, and any optional name. Reject malformed deserialized
values instead of silently normalizing. Convert core to engine by copying all
intervals and using the core name or `Custom`; apply its root separately.
Convert engine to core by copying all intervals, current root and engine name.
This preserves every supported interval and root; only absent display name is
canonically returned as `Some("Custom")` because the engine Scale always owns a
String name.

## Port inventory and scalar encoding

All ports use their stated `ValueKind`. Output merge policy is Reject. Clock,
reset and move inputs use Reject; write modulation uses Bipolar + Sum.

| Direction | Port(s) | Kind | Mapping |
| --- | --- | --- | --- |
| input | `clock` | Gate | rising edge calls `tick()` |
| input | `write_modulation` | Bipolar | held -1..=1 offset from base write probability |
| input | `reset` | Gate | rising edge calls `reset()` |
| input | `move` | Gate | rising edge calls `move_step()` |
| output | `step` | Gate | true for exactly one Produce when a tick/move snapshot is published |
| output | `note`, `note_valid` | Raw, Gate | `Option<u8>` as Raw value or 0 plus validity |
| output | `velocity`, `velocity_valid` | Raw, Gate | `Option<u8>` as Raw value or 0 plus validity |
| output | `gate` | Gate | `StepOutputs::gate` |
| output | `scale_note`, `scale_note_valid` | Raw, Gate | `Option<u8>` as Raw value or 0 plus validity |
| output | `pulse_1`…`pulse_6` | Gate | `pulses[0]`…`pulses[5]` |
| output | `gate_1`…`gate_8` | Gate | `gates[0]`…`gates[7]` |
| output | `div2`, `div4` | Gate | corresponding divider fields |
| output | `noise_cc` | Raw | zero-extended u8 |
| output | `register_bits` | Raw | exact u16 |
| output | `length` | Raw | checked usize-to-u16; invariant max 16 |
| output | `write_probability` | Float | exact step snapshot probability |

The 4 inputs plus 28 outputs are the complete 32-port schema. No output is
deferred. Validity gates keep future `None` meaningful even though the current
engine normally returns Some notes/velocity.

## Phase, conflict and lifecycle contract

The private component owns the engine, base write value, current modulation,
three previous Gate states, optional pending `StepOutputs`, and cached entities
for all ports. No per-sample lookup by string, allocation, blocking or OS RNG.

Produce consumes one pending snapshot. If present, write all 27 snapshot scalar
fields and set `step=true` in the same system iteration. If absent, write only
`step=false` and retain every snapshot field. Consume reads the four inputs,
reconciles effective write modulation, computes all rising edges against stored
history, and updates every history bit. It then permits at most one engine action
with fixed priority `reset > move > clock`:

- reset clears engine register/dividers/count, discards any not-yet-published
  pending snapshot and leaves held published outputs untouched;
- move stores its returned snapshot for next Produce;
- clock stores its returned snapshot for next Produce;
- simultaneous lower-priority edges are consumed and do not replay later.

This makes a Clock high at sample 0 visible to TM Consume at sample 0 and the
completed TM snapshot visible with `step=true` at Produce sample 1. The later
MIDI sink will receive it during that sample's propagation/consume phases.
At most one engine advance occurs per sample; a wide Clock pulse cannot retrigger.

Core's latched running sample owns execution. Do not inspect
`SimulationRunning` inside adapter systems. Stop therefore preserves engine RNG,
register, edge histories, pending/held snapshot, base setting and modulation.
Resume continues. `reset` is not seed restart. Reloading/reconstructing the
patch with the same master seed/name is seed restart and begins the same stream.
Reinstalling the module type does not add systems or reset instances.

## Behavioral proof matrix

Tests must distinguish this contract from common wrong implementations:

1. Compare every published port for at least 12 clock edges against a separate
   `TuringMachine::with_seed(context.instance_seed())` reference configured by
   public setters. A two-sample-wide Clock must still yield one snapshot per rise.
2. Assert exact sample indices: Clock rises 0,2,4… under the accepted 1000/30/1
   fixture; TM step triggers 1,3,5… and each full snapshot equals the standalone
   step at that index. Compare Manual single/grouped and controlled Live groups.
3. Exercise reset, move and clock separately and simultaneously. Prove priority,
   lower-edge consumption, move's unchanged step_count/dividers, reset's retained
   settings/RNG and held output, and next-Produce publication.
4. Drive write modulation through negative/positive/summed inputs; prove the
   effective target is `clamp(base + input)`, does not accumulate each sample,
   returns to base at zero, and appears in getter plus next snapshot.
5. Exercise every parameter at spawn and through its registered setter/getter,
   including length/position precedence, embedded/explicit roots, arbitrary
   supported scales, range boundaries, lexical unknowns, wrong variants and
   malformed Scale/range rejection without mutation.
6. Run a child-process trace twice and compare exact bytes; change master seed
   and instance name as negative controls. Reverse module order and insert an
   unrelated entity; named traces must remain equal. Compare adapter steps with
   the standalone reference, not another adapter path.
7. Stop with a pending snapshot and with inputs high, issue stopped requests,
   resume, and compare executed-sample traces to an uninterrupted reference.
   Reinstall after progress and prove no duplicate systems or state reset.

Avoid assertions limited to final state, row counts or adapter-vs-adapter
agreement. Preserve complete named traces and the independently derived
standalone expectations.

## Corrective iteration01 — preserved execution scope

CRC independently reviewed the initial CC return on 2026-10-10; the full
[review](crc-verification.md) preserves reproduced checks, limitations and
F-01–F-05. All six rows stay open. The initial implementation appears to follow
the approved adapter design, but the parameter/schema/preservation matrix,
absent/idle publication, conflicts/modulation, continuing lifecycle and real
name/unrelated-allocation negatives need discriminating evidence.

The current [iteration01 assignment](cc-prompt-iteration01.md) is a test/evidence
correction, pass 1/5. Source edits are restricted to cfg(test) portions of
TM `rack.rs`, `engine.rs`, `quantizer.rs`, and umbrella `tests/turing_machine.rs`.
Production sections, manifests/dependencies, tracked fixture and other source
paths stay byte-identical to `9ba55dd`. A new test revealing a production failure
returns to CRC before production edits. This explicit current fence supersedes
the initial ten-path implementation allowlist below; no behavior or acceptance
change is authorized. All initial quality gates remain required, plus focused
umbrella TM tests in no-default mode and the current-return checker.

CRC preserved the initial reports/intake/matrix/checker under
`artifacts/iteration00/`; old logs and issued prompts are immutable. CC may
append iteration01 intake/test evidence/results, write fresh iteration01 logs,
update the canonical close and proposed-done ledger notes only. CRC review and
`artifacts/crc-review01/` remain independent, protected evidence. The updated
existing checker defaults to issuance checks; `--require-return` enforces the
current return identity, source field, six-row walk and bubble-up. Historical
return is intentionally rejected in that mode until CC actually returns.

### Iteration01 author readiness — 2026-10-10

1. **Source-grounded:** CRC inspected the complete adapter/public integration,
   engine/getter diff and core registry/phase/merge APIs at clean `9ba55dd`;
   the independent name-control probe reproduces the false-positive predicate.
2. **Design-complete:** no new production behavior; the prompt resolves the
   actual snapshot population, conflict script, numeric/Scale matrix, seeded
   reference and resume ownership within the existing contract.
3. **Guideline-applied:** framework2.6.0 guide/template and testing discipline;
   Rust2.0.0 anti-patterns and verified API-23/API-39, TD-05, DC-05 sections.
   Prompt guidance table corrects the prior broad DC-05 label and ties each
   decision to the strict boundary, private state and observable proof.
4. **Executable:** unit tests retain private app access; registry uses verified
   Bevy resource_scope; integration completes all wrapper actions before its
   consuming into_app. Sketches are explicitly source-checked, not compiled.
5. **Falsifiable:** real name-content inequality rejects an identity-only rename;
   complete idle/pending/continued traces distinguish clearing, lost publication,
   duplicate systems and reseeding; both spawn and registered APIs exercise
   strict rejection/preservation. Same-predicate checker negatives remain.
6. **Coherent/portable:** current prompt/plan/ledger/status agree; installed skill
   paths resolve on this host, a bounded required-reading manifest preserves
   the governing text with implementation/recovery headroom. Current source
   fence and permitted evidence updates are explicit; older issued documents
   and attested claims stay historical. MIDI/Arc11 remain future work.

## Initial source scope and required validation

Authorized source paths:

- `crates/oxurack-mod-turingmachine/Cargo.toml`
- `crates/oxurack-mod-turingmachine/README.md`
- `crates/oxurack-mod-turingmachine/src/lib.rs`
- `crates/oxurack-mod-turingmachine/src/engine.rs`
- `crates/oxurack-mod-turingmachine/src/quantizer.rs`
- `crates/oxurack-mod-turingmachine/src/rack.rs` (new)
- `crates/oxurack/Cargo.toml`
- `crates/oxurack/tests/turing_machine.rs` (new)
- `crates/oxurack/tests/fixtures/turing-machine.ron` (new)
- `Cargo.lock`

No core, Clock, RT, MIDI, graph, existing test/fixture or Makefile source change.
An additional path or changed signal/phase contract stops for CRC/CDC review.

Required gates include default standalone and rack-feature tests/tree; umbrella
default/no-default package tests; core tests; both workspace feature matrices;
all-target Clippy with denied warnings; format; strict workspace and no-default
umbrella rustdoc; `make check`; dependency freshness observation; diff check;
exact allowlist/protected-path audit; and the current planning checker. Preserve
restricted MIDI/DNS failures plus successful host retries. Existing ignored RT
tests remain counted and explained.

## Author readiness

1. Source-grounded: CRC read every engine operation and `StepOutputs` field,
   current Scale/Quantizer forms, core Value/parameter/module/port APIs, Clock
   adapter and actual manual/Live integration patterns at source `a19cfd1`.
2. Design-complete: all settings, actions, outputs, validation, Scale conversion,
   conflict order, phase latency, modulation and stop/reset semantics are fixed.
3. Guideline-applied: collaboration-framework 2.5.0 prompt-authoring 1.15.0,
   testing discipline and Rust 2.0.0 API/type/error/docs guidance shape the
   boundary, getters, fallible conversion and discriminating tests.
4. Executable: existing APIs support the adapter without a core change; exact
   public method shapes, port names, parameter names and system phases are given.
5. Falsifiable: standalone-reference, exact-index, wide-pulse, conflict,
   nonaccumulating modulation, fresh-process and lifecycle controls reject the
   principal wrong implementations.
6. Coherent/portable: one crate adapter and one umbrella fixture fit a bounded
   slice. MIDI event/lifetime work remains separately owned with no silent
   capability deferral from this adapter.

## Assignment history and versions

- 2026-09-19: initial `cc-prompt.md` issued by CRC after accepting Slice01 and
  completing directive07's required actual-source inventory. Initial source
  `a19cfd1a134c5934d270dc0ba72334d8f167fe2f`, planning predecessor
  `ef9d99fe7ab4f38ca46a069c4e87cd4d2e10873f`; correction count0/5.
  Returned by CC at `9ba55dd`, planning `516afcd`/`2eb5ac1`.
  Disposition: returned for correction after CRC review, preserved unchanged.
- 2026-10-10: current
  `arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt-iteration01.md`
  issued by CRC, predecessor `cc-prompt.md`, reason F-01–F-05 missing/weak
  behavioral and return evidence. Disposition: returned by CC at `b2ffb16`, planning `61671a3`, independently
  accepted by CRC on 2026-10-10; corrective pass1/5 executed. Slice03 released
  for authoring under directive08.
- v1.2 — 2026-10-10: accepted corrected test/evidence packet after independent
  reproduction; six rows done; production/fixture/dependency bytes unchanged.
- v1.1 — 2026-10-10: independent partial review and bounded test/evidence
  corrective iteration01; acceptance/architecture unchanged.
- v1.0 — 2026-09-19: opened complete adapter mapping, lifecycle and proof set.
