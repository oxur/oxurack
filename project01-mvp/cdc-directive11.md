# CDC directive11 — fix configuration readback and persistent cable order

2026-10-11, Sofie/CDC to CRC through Duncan. Three contributors, Expedited Mode.
Initiated by Duncan's direct relay of CC's Slice01 return; no CRC escalation is
required merely to restate a report already received by CDC. Source inspected:
`a3633bc0d0f468e933cc0f1463fb637a623b33d1`; planning intake:
`9fdb2cd9320828e63288d2450f76b356381297f7`, both clean. Framework2.6.0.
Predecessor directive10 and Arc12/Slice01 initial cc-prompt.md remain historical;
this directive supersedes the two conflicting design choices and source boundary.

## Verdict, evidence and author accountability

Slice01 is open. CC delivered the authorized implementation and disclosed two real
integration conflicts. CDC independently reproduced:

- `cargo test --locked -p oxurack --test persistence live_configuration_saves_and_replays_in_fresh_racks -- --exact`:
  exit101, one failed/zero passed, at tests/persistence.rs:108, saved trace versus
  independently edited reference. Full transient log:
  `/tmp/oxurack-cdc-a3633bc-replay.log`.
- `cargo test --locked -p oxurack --lib persistence::tests::parallel_cable_tie_order_survives_snapshot_and_fresh_load -- --exact`:
  exit0, one passed. Source inspection shows this is a diagnostic reproduction:
  it asserts first=.25 and replay=.75. Its name does not make it acceptance proof;
  the correction must restore the intended equality assertion.

The initial CDC packet had two erroneous assumptions, not an invitation for CC
to improvise: it overlooked Arc10 Slice02's explicit effective-value getter
contract (plan parameter table and modulation tests), and assumed ascending Bevy
Entity order survives fresh allocation. The getter currently returns
state.engine.write_probability(); reconciliation uses clamp(write_base + CV).
The setter already stores write_base, so no new shadow parameter cache is needed.
Core validation allows duplicate endpoint cables on LastWins inputs, but both
propagation and new snapshot extraction order tied endpoints by Entity.

The product contract stays edit → save configuration → fresh equivalent replay.
Do not change the expected saved base from .25 to the momentary .5426204, remove
modulation from the fixture, reject previously valid parallel cables, or accept
.25→.75 as correct. These would weaken the promised feature. The two repairs below
are authorized dependencies of the already selected live-persistence work.

## Decision 1 — parameter readback is configuration

**Binding:** the registered `write_probability` parameter getter returns
`TuringMachineState.write_base`. `Oxurack::parameter`, parameter-event previous
values and persistence therefore describe the same configured knob value.
This intentionally supersedes the historical Arc10 registered-getter effective
semantics for new source. Preserve that older acceptance at its original revision.

The standalone engine's `write_probability()` and the adapter's step output port
`write_probability` keep their existing effective-value semantics and publication
latency. Consume/reconcile still applies clamp(base + CV) to the engine; no RNG,
phase, pending-step or musical algorithm change. Do not add another getter
registry or snapshot-only TM special case. Document parameter versus output in
the adapter README. Other getters remain unchanged unless a concrete new conflict
is found and returned; no generalized configuration/state reflection framework.

Recommended edit (uncompiled shape):

```rust
fn get_write_probability(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Float(state_ref(world, entity)?.write_base))
}
```

Update the existing `held_modulation_reconciles_targets_and_registered_base_edits`
test: with held+.25, after setting base.25 the registered getter is .25 and the
engine/next step remains .5; the already pending old step remains .75. Across
negative/positive/clamped modulation values, getter stays at the base. Retain
actual engine/output assertions, not just changed getter expectations.
Add/extend facade evidence that successful DirectApi mutation under nonzero CV
has previous=old base and new=new base, one event; rejected mutation changes
neither base nor effective state and emits no success event. Saved RON must
explicitly contain TM write_probability Float(.25), then pass the existing
independently configured fresh-replay test unchanged in substance.

## Decision 2 — tie order is logical, not allocator-derived

**Binding:** keep primary cable ordering by target instance/port, then source
instance/port. For identical endpoint tuples, cables loaded from Patch.cables
run in that vector's order. Saving emits their current effective order, so
fresh loading reconstructs it. No RON field/version change is needed.

Add core `CableOrder(pub usize)`, a public `Component` with Debug/Clone/Copy/
PartialEq/Eq/PartialOrd/Ord, representing an order within a loaded cable list.
Re-export alongside Cable. This is ordering metadata, not a persistent entity ID;
it is not separately serialized or added as a field to Cable/CableConfig.
`apply_patch_to_world` enumerates Patch.cables and spawns each cable together
with CableOrder(index). Keep the existing index/event/label wiring.

Both `rebuild_propagation_order` (including its system query) and snapshot
extraction include `Option<&CableOrder>` and use this full ordering:

```text
(target_name, target_port, source_name, source_port,
 absent_order, ordinal_or_zero, entity)
```

Present order has absent_order=false; absent order has true and ordinal0.
Thus ordered cables precede unordered cables *within the same endpoint tuple*.
Different endpoints remain governed by names/ports. Equal explicit ordinals and
all unordered ties retain Entity as a final within-world fallback. This keeps
existing raw-world cable construction working; it does not assert cross-world
stability for missing/duplicate metadata. Snapshot resolves the actual fallback
order into a vector; reload assigns distinct ordinals and preserves that order.

Illustrative sketch, not compiled drop-in code:

```rust
// in patch application
for (ordinal, cable_config) in patch.cables.iter().enumerate() {
    // resolve endpoints and build Cable exactly as today
    let cable_entity = world.spawn((cable.clone(), crate::CableOrder(ordinal))).id();
    // existing CableIndex, CableConnected and labels follow
}

// after resolving the same four-name prefix in each consumer
let tie = match order {
    Some(order) => (false, order.0, cable_entity),
    None => (true, 0, cable_entity),
};
```

Do not reverse spawn iteration, depend on entity numeric direction, sort by
transform/label/value, or allocate a global stable-ID service. Rust TD-02/TD-03
support a small typed component for this distinct meaning; no extra dependency.
These are off-hot-path ordering/extraction changes, not a new scheduler.
Correct stale core comments claiming ID order or that all endpoint ties are invalid.
Direct raw-world edits to ordering require mark_propagation_order_dirty, just as
other topology edits do. Document that condition. Slice03's future runtime cable
API must assign an order after existing ordered cables and maintain the dirty
flag; detailed mutation/overflow policy belongs to that later packet.

### Discriminating ordering proof

Replace the diagnostic test with true acceptance: a loaded duplicate-endpoint
LastWins pair with vector contributions [.75,.25] yields .25 before and after
snapshot/save/fresh reload, repeated across a second save/reload. Assert serialized
vector order, not merely final equality. Remove the diagnostic .75 replay assertion.
Do not reassign transforms by sorted Entity in the main ordered-cable test; doing
so would override the input contract instead of testing patch-list order.

Also exercise a small core fixture where explicit CableOrder is deliberately
opposed to Entity ordering; the ordinal must win. Add an unrelated endpoint pair
to show names/ports precede ordinals. A compact fallback case (absent/equal order)
checks defined Entity fallback and snapshot-to-ordered-reload preservation. Reuse
existing inline/core tests, not another harness or package. Run ordering behavior
through the real core schedule; cover its Update and FixedUpdate installation
paths using existing test patterns. CRC may consolidate these cases in one test.

## CRC assignment, scope and required checks

Acknowledge this directive in the existing project handoff history. Inspect CC's
return and issue exactly one bounded `cc-prompt-iteration01.md` in the existing
Slice01 directory, using guide07 with a precise reading manifest, code shapes,
behavioral oracles and return requirements. Preserve initial prompt and CC return;
record iteration01 as active only when actually issued. No new slice, preliminary
investigation or new approval gate is needed for these resolved decisions.
CRC still performs normal independent review of all five LP rows; this CDC conflict
review is not full slice acceptance. Subsequent advancement remains held until that
review passes. Existing unaffected CC evidence is attestation, not independent proof.

Correction source scope (complete allowed path set; commit only changed paths):

- crates/oxurack-mod-turingmachine/src/rack.rs — base getter and adjacent tests/docs.
- crates/oxurack-mod-turingmachine/README.md — parameter versus effective output.
- crates/oxurack-core/src/cable.rs — CableOrder definition/documentation.
- crates/oxurack-core/src/lib.rs — re-export.
- crates/oxurack-core/src/patch.rs — attach vector ordinals and local tests/docs.
- crates/oxurack-core/src/tick.rs — identical ordering and focused schedule tests/docs.
- crates/oxurack/src/persistence.rs — match propagation order and repair tie test.
- crates/oxurack/tests/persistence.rs — explicit saved base/fresh replay assertions.
- README.md — clarify configuration readback and stable parallel-cable behavior.

No standalone engine, dependencies, lockfile, file schema, musical fixtures or
playback runtime change. Local helper/test mechanics may adapt within the resolved
semantics. Authorized files cover the actual repairs; escalate only a consequential
new issue, not routine borrow/type/compiler adjustments in those files.

Keep Slice01's required validation commands and both examples/fresh-process replay.
Add `cargo test --locked -p oxurack-mod-turingmachine --features rack` and standalone
`cargo test --locked -p oxurack-mod-turingmachine`; add
`cargo clippy --locked -p oxurack-mod-turingmachine --features rack --all-targets -- -D warnings`. Rerun core, default/no-default facade/library/
persistence/Scope/LFO/MIDI/example tests, make test/lint, strict docs and diff checks.
Native gates previously stopped at replay, so they must complete after repair;
no claim that their earlier partial run passed. Fresh existing original/LFO traces
must remain compatible; the getter change must not change engine MIDI behavior.
Known block advisory/RT ignores and any restricted/host failures stay explicit.
No new coverage/soak/endpoint campaign is imposed.

CRC authors the correction, CC implements/commits explicit paths, CRC independently
reviews and closes when justified. Commit planning separately. Return the issued
iteration path through Duncan; no new CDC round trip is required for these two
resolved repairs. Source implementation remains CC's task; no merge/push/release.
