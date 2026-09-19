# Slice01 — musical Clock

Version 1.2 — 2026-09-19. Status: closed; CRC accepted after corrective pass 1/5.
CDC authors, CC implements, CRC reviews.
Accepted assignment: `arc10-turing-machine-midi/slice01-musical-clock/cc-prompt-iteration01.md`.
Source baseline: `37deeace90e7fe2dff633ada9d58ef9051577c3d`, clean
`.worktrees/codex-oxurack-module-behavior-installation` on its existing branch.
Planning baseline before this issue: `f6c778e`; exact full hash in Arc09 review07.
Authority: [directive07](../../cdc-directive07.md); [arc plan](../arc-plan.md).
Artifact home: this slice's `artifacts/`. No source implementation by CDC.

## Outcome and boundaries

A reusable deterministic 1 ms standalone Clock and its optional rack adapter
produce validated musical pulses with exact rounding, stop/resume and edit
behavior. This is the TM pulse prerequisite, not a MIDI Clock byte generator.
Six rows C-01–C-06 in [ledger.md](ledger.md) define completion. TM, MIDI sink,
RT, core changes, UI, global SetBpm and patch schema changes are excluded.

## Binding numeric and public API contract

New package `oxurack-mod-clock`, version 0.1.0, edition 2024, workspace metadata
matching adjacent crates. Default features empty. Optional `rack` enables
oxurack-core, bevy_app and bevy_ecs using existing versions/settings. Standalone
normal dependency only thiserror=2; no Bevy/core dependency in default builds.

Public standalone types (derive Debug; Config Clone/Copy/PartialEq/Eq):

```rust
pub struct ClockConfig { /* private canonical values */ }
impl ClockConfig {
    pub fn try_new(bpm: f32, ppqn: u16, pulse_width_ms: u16) -> Result<Self, ClockError>;
    pub fn bpm(&self) -> f32;
    pub fn ppqn(&self) -> u16;
    pub fn pulse_width_ms(&self) -> u16;
}
pub struct Clock { /* private state */ }
impl Clock {
    pub fn new(config: ClockConfig) -> Self;
    pub fn config(&self) -> ClockConfig;
    pub fn set_config(&mut self, config: ClockConfig);
    pub fn next_sample(&mut self) -> bool;
}
```

Both Config and Clock implement Default (120 BPM, 1 PPQN, width 10 ms).
ClockError is a non-exhaustive Debug/thiserror enum with unit variants
NonFiniteBpm, BpmOutOfRange, InvalidPpqn, InvalidPulseWidth, PulseOverlap.
Validate in that order. BPM is finite in [1,1000]; PPQN is integer [1,96]; width
is integer [1,1000] ms. Canonical BPM is `(f64::from(bpm)*1000.0).round()` as u32
after range checks; positive ties round upward. Getter returns canonical/1000
as f32. M1 resolution is explicitly 0.001 BPM; rejected values are never clamped.
Let D=60,000,000 and R=canonical_milli_bpm × ppqn in u64. Require
`D / R >= width + 1` with integer floor. This guarantees at least one low sample
between every pair of pulses and rejects >one-trigger-per-sample configurations.

Each new/reset epoch has rising samples `ceil(k*D/R)` for k=0,1,2,…, relative
to its first sample. Pulse at n is high for exactly width consecutive samples
starting at n, then low. Use a bounded integer phase accumulator, never repeated
rounded periods, float wall time or a growing product of elapsed samples.
Recommended pseudocode (contract sketch, not compiled):

```text
next_sample:
  if restart_pending:
    restart_pending=false; fresh=true; phase=0; high_remaining=0; return false
  if fresh: fresh=false; rising=true
  else:
    phase += R
    rising = phase >= D
    if rising: phase -= D
  if rising: high_remaining=width
  high = high_remaining > 0
  if high: high_remaining -= 1
  return high
```

Phase starts zero. Bounds ensure at most one subtraction and no u64 overflow;
phase remains <D; hot path allocates nothing and blocks on nothing.

## Binding edits and lifecycle

set_config compares canonical Config values. Equal config is a true no-op.
A changed config stores it and sets restart_pending; it never changes an already
published port immediately. Next executed sample is forced low, then the following
sample starts a fresh epoch with a rising edge. Multiple edits before that sample
coalesce into one low sample using the last successfully installed config.
Changing back before execution still retains the pending restart; an equal setter
must not clear it. This one-low-sample separator is essential for downstream edge
detection. Do not merely reset phase while retaining a high Gate.

Rack stop pauses state and preserves the held port; start resumes the next sample
without resetting phase or inserting an extra pulse. This is a signal-source
contract, not note cleanup. Shared mid-sample stop completes the latched sample;
adapter must not inspect SimulationRunning independently. Invalid edits leave
config, phase, pending restart and current output unchanged. Parameter edits are
synchronous at a quiescent World boundary; no cross-thread ownership or queue.

## Rack adapter and parameter boundary

Behind rack, export zero-sized `ClockModule` implementing OxurackModule;
KIND="clock", DISPLAY_NAME="Clock". One output `pulse`: Gate, Reject merge.
Parameters: `bpm` Float(120.0), `ppqn` Int(1), `pulse_width_ms` Int(10).
Each has setter/getter. No tempo resource or Patch.bpm inheritance. Document
explicit Clock bpm in patch examples even when equal to patch metadata.

Spawn first decodes all overrides into a candidate config, validates combined
values, then spawns module/port/private state. Defaults apply only to absent
keys; unknown keys return UnknownParameter, wrong variants/ranges/cross-field
combinations return InvalidParameterValue with actual instance name and parameter.
For deterministic errors, reject unknown keys in lexical order, then validate
bpm, ppqn, width and overlap (attribute overlap to pulse_width_ms on spawn).
No casts before signed range checks. Validate all before creating any Clock
entity. Whole-patch rollback is not promised: earlier unrelated modules may
already exist under core's existing loader contract.

Setters obtain module identity/state fallibly (UnknownModule for missing Clock
state/entity), decode strict expected variant, construct full candidate with the
other current values, validate, then call set_config. Attribute overlap to the
parameter being edited. Getters read engine.config(), never schema defaults.
Do not remove ParameterRegistry permanently to satisfy borrows; copy setter
pointer or use World::resource_scope in tests as existing core patterns do.

Use spawn_module_entity / spawn_port_on_module. Private component owns Clock and
output Entity. register_systems adds one query system to the supplied schedule
in TickPhase::Produce; query all instances, call next_sample once, write Gate
CurrentValue. Legacy spawn is sufficient (no random state); default context
forwarding preserves behavior. Do not spawn in the hook, register per instance,
hardcode Update, add an extra running gate or modify core's phase schedule.

## Verification and exit

The [prompt](cc-prompt.md) carries exact sample oracles, integration recipes,
commands and explicit allowed paths. All six ledger rows require fresh CC
evidence then independent CRC reproduction. Preserve failed/unrun attempts.
Existing coverage/hardware/soak gates remain; no new lower target. CRC may open
Slice02 only after acceptance and its complete author-readiness check.

## Author readiness — CDC, 2026-09-18

1. Source-grounded: inspected workspace wildcard membership; existing TM/umbrella
   manifests; core module, error, parameter, spawn and phase APIs; umbrella
   sample-pipeline tests; no current Clock crate. Bevy 0.18 APIs used by those
   compiled tests are the recipe, not guessed external signatures.
2. Design-complete: numeric canonicalization, fractional phase, low-sample bound,
   aggregate validation, patch BPM distinction, stop/edit interactions and error
   mutation boundaries above are binding. Pure synchronous module has no device
   resource, task cancellation or teardown beyond owned state.
3. Guideline-applied: prompt cites actually read API-23/39, TD-05, DC-05 and
   testing discipline; concrete candidate-before-mutation and exact error tests.
4. Executable: pure engine then optional adapter, then umbrella dev dependency
   and real RON integration. Consume-edge observer installs as a fixture module;
   wrapper is retained for reinstall/stop/resume before consuming into_app for trace;
   edits use a core App in rack.rs tests with direct World access.
5. Falsifiable: nonintegral 137-BPM trace rejects per-period rounding; edit while
   high rejects a lost edge; invalid aggregate config cannot partially mutate;
   grouped-live and opposing spawn orders reject update/entity authority.
6. Coherent/portable: prompt manifest bounds shared docs, names local installed
   guide paths and intake record. Packet is one small state machine + adapter;
   roughly 8–12k tokens of contract/source reads leaves implementation/refinement
   headroom. Private helpers only are local discretion; no unresolved current-
   slice design. Arc runtime mechanics are design constraints, not this slice.

## Assignment history

- 2026-09-18: initial cc-prompt.md issued by CDC after independent Arc09 close;
  no predecessor within this slice. Returned by CC at source `3009b1ef`; CRC
  review found one reproduced diagnostic defect plus incomplete C-01–C-04
  behavioral evidence. Superseded by iteration01; all rows remain open.
- 2026-09-19: `cc-prompt-iteration01.md` issued by CRC after the independent
  iteration00 review; predecessor `cc-prompt.md`; active corrective pass 1/5.
  It retains the design/source fence and repairs spawn diagnostics plus the
  missing standalone, adapter and manual/Live discriminating oracles.
- 2026-09-19: CC returned source `a19cfd1` and evidence `ef9d99f`; CRC
  independently reproduced the corrected behavior and gates. Accepted and
  closed after corrective pass 1/5.

## CRC corrective author readiness — iteration01, 2026-09-19

1. Source-grounded: reviewed all three implementation/test files and reproduced
   the invalid-BPM attribution through the public spawn trait. Existing compiled
   sample-pipeline tests supply the Live runner shape.
2. Design-complete: the existing contract already fixes validation precedence,
   parameter attribution, edit preservation and driver equivalence. No new
   architecture or product behavior is introduced.
3. Guideline-applied: collaboration-framework 2.5.0 / engineering-methods
   prompt-authoring 1.15.0, Rust 2.0.0 API-23/API-39/TD-05 and testing discipline
   inform the typed boundary repair and behavior-first test matrix.
4. Executable: iteration01 gives the two-stage decoder and error-mapping shape,
   exact affected files and viable private/unit versus public integration homes.
5. Falsifiable: mixed invalid/unknown maps reject the old interleaved decoder;
   exact error parameters reject the current mapping; full traces distinguish
   grouping, Live, reinstall and reset/catch-up errors.
6. Coherent/portable: all six rows remain open, both prompts are preserved, the
   original nine-path fence governs and only three existing files should change.

## Version history

- v1.0 — 2026-09-18: CDC opened the planned musical Clock prerequisite with
  exact timing, lifecycle and error contracts before CC implementation.
- v1.1 — 2026-09-19: CRC returned iteration00 for a reproduced spawn-error
  attribution defect and incomplete discriminating tests; issued corrective
  iteration01 without changing the approved Clock design or source fence.
- v1.2 — 2026-09-19: CRC accepted source `a19cfd1`, closed C-01–C-06 and
  advanced to the separately inventoried Slice02 adapter packet.
