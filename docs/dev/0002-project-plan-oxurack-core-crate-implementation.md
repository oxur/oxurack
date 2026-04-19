# Project Plan: `oxurack-core` Crate Implementation

## Context

The `oxurack-core` crate is Tier 2 of the oxurack architecture — the ECS world layer that provides the shared vocabulary every module is written against. It owns the value currency, port/cable/module entity model, the three-phase tick cycle, parameter registry, patch persistence, and the ECS-side bridge to the RT thread. In doc 0009's build order, this crate corresponds to Phase 0 deliverables P0-2 through P0-6 and P0-10, with patch persistence in Phase 3 (P3-1 through P3-5).

Design spec: `docs/design/01-draft/0007-oxurack-core-ecs-world-values-ports-and-cables.md`
Architecture: `docs/design/01-draft/0005-oxurack-system-architecture.md`
Module infrastructure: `docs/design/01-draft/0008-oxurack-module-infrastructure-authoring-modules-on-bevy.md`
Build order: `docs/design/01-draft/0009-module-catalog-and-build-order-v2.md`

The crate depends on Bevy (headless), serde, ron, smallvec, thiserror, and ahash. It does NOT depend on `oxurack-rt` as a Cargo dependency (except behind an optional `rt-bridge` feature flag). The RT bridge uses the rt crate's public types (`RtEvent`, `EcsCommand`) but communicates via queue handles passed as a resource.

---

## Dependencies

| Dependency | Version | Purpose |
|---|---|---|
| `bevy_ecs` | latest stable | Entity-component-system |
| `bevy_app` | latest stable | App, Plugin, schedules |
| `bevy_reflect` | latest stable | Type reflection for serialization |
| `bevy_utils` | latest stable | HashMap, HashSet re-exports |
| `serde` | `1` (features = ["derive"]) | Serialization traits |
| `ron` | latest stable | RON format |
| `smallvec` | `1` | Cable index, scale intervals |
| `thiserror` | `2` | Error derives (matching oxurack-rt) |
| `ahash` | `0.8` | Deterministic seed derivation |
| `rand` | `0.10` (features = ["small_rng"]) | Per-module RNG |
| `rtrb` | `0.3` (optional, `rt-bridge` feature) | RT queue types |
| `oxurack-rt` | path (optional, `rt-bridge` feature) | RT event/command types |

Dev: `pretty_assertions = "1"`.

All Bevy sub-crates use `default-features = false` to avoid pulling in rendering/windowing.

---

## Phase 1: Foundation Types (Pure Data, No Bevy Yet)

**Goal**: Define every core data type as plain Rust structs and enums with standard derives. No Bevy components, no ECS. Every type compiles and has exhaustive unit tests.

**Aligns with**: Doc 0009 deliverables P0-2 through P0-5. Doc 0007 sections 4-6, 10.

### Milestone 1.1 — Crate scaffold compiles

Create the full file tree from doc 0007 §3:

```
crates/oxurack-core/
├── Cargo.toml          # edition 2024, deps listed above
├── README.md
└── src/
    ├── lib.rs           # pub mod declarations + re-exports
    ├── value.rs         # Value, ValueKind, MidiMessage
    ├── port.rs          # PortDirection, MergePolicy, PortName
    ├── cable.rs         # CableTransform
    ├── module.rs        # ModuleKind, ModuleId
    ├── tick.rs          # empty
    ├── parameter.rs     # empty
    ├── patch.rs         # empty
    ├── scale.rs         # empty
    ├── rng.rs           # empty
    ├── event.rs         # empty
    └── error.rs         # CoreError, PatchError, TickError
```

**Accept**: `cargo build -p oxurack-core && cargo clippy -p oxurack-core -- -D warnings`

### Milestone 1.2 — Value types complete and tested

Implement in `value.rs`:

- `MidiMessage` enum (doc 0007 §4.3): NoteOn, NoteOff, ControlChange, PitchBend, ProgramChange, ChannelPressure, PolyKeyPressure, Clock, Start, Stop, Continue, SongPosition, SystemExclusive. All Copy. This is oxurack-core's own structured MidiMessage — distinct from `oxurack_rt::MidiMessage` (compact 4-byte format). Conversion between them comes in Phase 6.
- `Value` enum (doc 0007 §4.1): Float(f32), Gate(bool), Bipolar(f32), Midi(MidiMessage), Raw(u16). `#[non_exhaustive]`.
- `ValueKind` enum: Float, Gate, Bipolar, Midi, Raw. `#[non_exhaustive]`.
- `Value::kind() -> ValueKind`
- `Value::try_coerce(&self, target: ValueKind) -> Option<Value>` — coercion table:
  - Float<->Bipolar (scale), Float->Gate (threshold 0.5), Gate->Float (0.0/1.0), Bipolar->Gate (threshold 0.0)
  - Anything involving Midi or Raw returns None (except identity)
- Size assertions: Value ≤ 16 bytes

**Tests**: Exhaustive coercion table (every kind pair), kind() for each variant, identity coercions.

### Milestone 1.3 — Port and cable types

Implement in `port.rs`:
- `PortName` newtype over `String` (use String for Reflect compatibility). From<&str>, Display, AsRef<str>.
- `PortDirection`: Input, Output. `#[non_exhaustive]`.
- `MergePolicy` (doc 0007 §6): Reject, Average, Sum, Max, Interleave, LastWins. `#[non_exhaustive]`.
- `MergePolicy::is_valid_for(kind: ValueKind) -> bool` — validation per doc 0007 §6 table.

Implement in `cable.rs`:
- `CableTransform` (doc 0007 §5.3): Affine, Invert, Clamp, Threshold, GateToFloat, Unipolar, Bipolarize. `#[non_exhaustive]`.
- `CableTransform::apply(&self, input: Value) -> Option<Value>` — returns None on kind mismatch.

**Tests**: MergePolicy validity (6 policies × 5 kinds = 30 cases), CableTransform::apply for each variant with valid/invalid inputs.

### Milestone 1.4 — Module and error types

Implement in `module.rs`:
- `ModuleKind` newtype over `String`. From<&str>, Display.
- `ModuleId(u64)`. PartialOrd, Ord for topo sort tie-breaking.

Implement in `error.rs` (doc 0007 §13):
- `CoreError`, `PatchError`, `TickError` — all via thiserror, `#[non_exhaustive]`.

**Tests**: Construct each error variant, verify Display. Test From conversions.

**Phase 1 exit**: All foundation types compile, tested, correct Display/Debug.

---

## Phase 2: ECS Integration

**Goal**: Add Bevy Component/Reflect/Resource derives. Create CorePlugin with three-phase tick system sets. Implement CableIndex. Define the entity model.

**Aligns with**: Doc 0009 deliverable P0-6. Doc 0007 §§5, 7.1, 7.4, 16.

### Milestone 2.1 — Bevy derives on all types

Add `Reflect` to all types participating in patch serialization: Value, ValueKind, MidiMessage, PortDirection, MergePolicy, CableTransform, ModuleKind, ModuleId.

Design decision: PortName and ModuleKind use `String` internally (not SmolStr) for Reflect compatibility. Document that internal representation may change.

### Milestone 2.2 — Component types for the entity model

- `Port` component (doc 0007 §5.2): name, direction, value_kind, merge_policy. `Component + Reflect`.
- `CurrentValue(pub Value)` component. `Component + Copy + Reflect`.
- `Cable` component (doc 0007 §5.3): source_port (Entity), target_port (Entity), transform, enabled. `Component + Reflect`.
- `Module` component (doc 0007 §5.1): kind, instance_name. `Component + Reflect`.
- `ModuleId` — add `Component` derive.

**Tests**: Spawn entities in a Bevy World with each component, verify queries.

### Milestone 2.3 — CableIndex resource

Implement in `cable.rs`:
- `CableIndex` resource (doc 0007 §7.4):
  - `by_target: HashMap<Entity, SmallVec<[Entity; 4]>>`
  - `by_source: HashMap<Entity, SmallVec<[Entity; 4]>>`
  - `cables_targeting(port) -> &[Entity]`, `cables_from(port) -> &[Entity]`
  - `rebuild()` from query
- Cable spawn/despawn observers maintaining the index.

**Tests**: Spawn cables → index updated. Despawn cable → index updated. Empty port → empty slice.

### Milestone 2.4 — TickPhase system sets and CorePlugin skeleton

Implement in `tick.rs`:
- `TickPhase` system set: Produce, Propagate, Consume (chained).

Implement in `lib.rs`:
- `CorePlugin` struct implementing Plugin: registers CableIndex, TickPhase sets, observers.
- All public re-exports per doc 0007 §16.

**Tests**: App + CorePlugin builds. CableIndex resource present. System set ordering verified.

### Milestone 2.5 — Module/port spawn helpers and seed derivation

Implement in `module.rs`:
- `spawn_module_entity(world, kind, instance_name) -> Entity`
- `spawn_port_on_module(world, module_entity, name, direction, kind, merge_policy) -> Entity`
- `Value::default_for_kind(kind) -> Value`

Implement in `rng.rs`:
- `derive_seed(master_seed: u64, instance_name: &str) -> u64` — via ahash, documented as stable algorithm.
- `ModuleId::from_instance_name(name) -> Self`

**Tests**: Spawn helpers produce correct entities. derive_seed is deterministic, differs for different inputs.

**Phase 2 exit**: CorePlugin builds, entities spawn correctly, CableIndex maintained.

---

## Phase 3: Tick Cycle

**Goal**: Three-phase tick: Produce → Propagate → Consume. Topological sort. Merge execution. This is the heart of the rack.

**Aligns with**: Doc 0007 §§7.1-7.5.

### Milestone 3.1 — Propagate system

Implement in `tick.rs`:
- `MergeBuffers` resource: `HashMap<Entity, SmallVec<[Value; 4]>>`.
- `propagate_cables_system`: iterate cables in stable order (from CableIndex), read source CurrentValue, apply transform, contribute to target's merge buffer.

**Tests**: Single cable (no transform), cable with transform, disabled cable skipped, incompatible transform skipped, multi-cable to same target.

### Milestone 3.2 — Consume system

Implement in `tick.rs`:
- `consume_ports_system`: for each input port with contributions, apply merge policy, write CurrentValue. Zero contributions → retain (sample-and-hold).
- Merge implementations: Reject (take first if >1), Average (mean), Sum (clamped), Max, LastWins, Interleave (take last for v1).

**Tests**: Each merge policy with correct expected values. Zero-contribution sample-and-hold. Bipolar clamping. Gate OR for Sum/Max.

### Milestone 3.3 — Topological sort and TickOrder

Implement in `tick.rs`:
- `TickOrder` resource: `Vec<Entity>` of modules in topo order.
- `compute_tick_order(...)` — Kahn's algorithm, tie-break by ModuleId.
- Cycle detection: `PatchError::FeedbackCycle`.
- `rebuild_tick_order_system` triggered on cable changes.

**Tests**: Linear chain, diamond, disconnected modules, cycle detection, self-loop, determinism across runs.

### Milestone 3.4 — Wire into CorePlugin + integration tests

Register MergeBuffers, TickOrder, propagate/consume systems in CorePlugin.

**Integration tests**:
- 2 modules + 1 cable: value propagates from source to target.
- 3 sources + Average merge: correct mean computed.
- TickNow event triggers full Produce→Propagate→Consume.

### Milestone 3.5 — TickNow event and tick driver

Implement `TickNow` event (doc 0007 §7.6). Module tick systems use `run_if(any_tick_event)` conditions. Register in CorePlugin.

**Tests**: Send TickNow → Produce/Propagate/Consume execute in order.

**Phase 3 exit**: Complete tick cycle working. Topo sort validated. Merges correct.

---

## Phase 4: Parameter System, Scale, RNG, Events, Module Infrastructure

**Goal**: Parameter registry, Scale extraction, seed derivation, event types, and the OxurackModule trait. Everything module authors need.

**Aligns with**: Doc 0007 §§8-10. Doc 0008 §§2-4.

### Milestone 4.1 — Parameter registry

Implement in `parameter.rs`:
- `ParameterName` newtype.
- `ParameterValue` enum: Float, Int, Bool, String, Scale. `#[non_exhaustive]`.
- `ParameterSchema`: name, kind, default, range, description.
- `ParameterRegistry` resource: maps (ModuleKind, ParameterName) → setter function.
- `register_setter()`, `set_parameter()`.

**Tests**: Register/call setter. Unknown parameter → CoreError. Invalid value → CoreError.

### Milestone 4.2 — Scale type extracted from turingmachine

Implement in `scale.rs`:
- Port `Scale` from turingmachine with: SmallVec<[u8; 12]> intervals, root field, Optional name, Reflect + Serialize + Deserialize derives.
- All built-in constructors: chromatic, major, minor, pentatonic, blues, etc.
- `Scale::quantize(&self, raw_note: u8) -> u8` (root now on Scale, not Quantizer).

**Tests**: Port all existing Scale tests, RON round-trip.

### Milestone 4.3 — RNG seed derivation (complete)

Extend `rng.rs`:
- `derive_module_rng(master_seed, instance_name) -> SmallRng` convenience.
- Document determinism contract.

**Tests**: Identical sequences for same inputs, different for different inputs.

### Milestone 4.4 — Event types

Implement in `event.rs`:
- `TransportChanged(TransportState)`, `MidiInReceived`, `CoreCommand`.
- Register all in CorePlugin.

**Tests**: Construct each variant, send/receive in test App.

### Milestone 4.5 — Module infrastructure types

Implement in `module.rs`:
- `PortSchema` (doc 0008 §2.1).
- `OxurackModule` trait (doc 0008 §2): KIND, DISPLAY_NAME, port_schema(), parameter_schema(), spawn().
- `ModuleRegistry` resource + `register_module()` helper.
- `spawn_module()` helper.

**Tests**: Dummy module → register → spawn → verify components and ports.

**Phase 4 exit**: Parameter registry, Scale, RNG, events, module infrastructure all working.

---

## Phase 5: Patch Persistence

**Goal**: RON serialization. Load/save. Validation. Round-trip determinism.

**Aligns with**: Doc 0007 §11. Doc 0009 P3-1 through P3-5.

### Milestone 5.1 — Patch data structures

Implement in `patch.rs`:
- `Patch` struct: version, master_seed, bpm, modules, cables.
- `ModuleConfig`: kind, instance_name, parameters.
- `CableConfig`: source (module, port), target (module, port), transform.
- Serde Serialize/Deserialize.

**Tests**: Construct → RON serialize → deserialize → round-trip equality.

### Milestone 5.2 — Patch validation

Implement `validate_patch(patch, registry) -> Result<(), PatchError>`:
- Unknown module kind, duplicate names, unknown ports, illegal merges, kind mismatches, feedback cycles.

**Tests**: Valid patch → Ok. One test per error variant.

### Milestone 5.3 — Patch load (RON → World)

Implement `load_patch(world, patch)` and `load_patch_from_file(world, path)`:
- Validate → clear → spawn modules → spawn ports → spawn cables → set seeds → rebuild TickOrder → emit PatchLoaded.

**Tests**: Load 1 module, load 2 modules + cable, invalid patch → error + unchanged world.

### Milestone 5.4 — Patch save (World → RON)

Implement `save_patch(world) -> Patch` and `save_patch_to_file(world, path)`.

**Tests**: Save 2 modules + cable → correct Patch struct. Human-readable RON output.

### Milestone 5.5 — Round-trip integration test

`tests/patch_roundtrip.rs`: Define test module, create patch, load in App A, tick 100x, save, load in App B, tick 100x, assert bit-identical outputs.

**Phase 5 exit**: Patches serialize/deserialize. Validation catches all errors. Round-trip determinism proven.

---

## Phase 6: RT Bridge, CoreCommand System, Polish

**Goal**: ECS-side RT bridge systems. CoreCommand dispatch. Documentation and coverage.

**Aligns with**: Doc 0007 §§12-13. Doc 0009 P0-10, P2-3.

### Milestone 6.1 — RT bridge (optional feature)

`oxurack-rt` as optional dep behind `rt-bridge` feature flag. `RtBridge` resource wrapping rtrb queues.

### Milestone 6.2 — drain_rt_events_system

Gated by `#[cfg(feature = "rt-bridge")]`. Converts `oxurack_rt::RtEvent` → core events (TickNow, TransportChanged, MidiInReceived). Includes `oxurack_rt::MidiMessage` → `oxurack_core::MidiMessage` conversion.

**Tests**: Inject mock events → verify core events emitted.

### Milestone 6.3 — flush_midi_output_system

`MidiOutputQueue` resource. Flushes accumulated MIDI commands to RT queue each PostUpdate. Drops on queue full (no panic).

**Tests**: Push commands → appear on RT queue. Full queue → dropped with count.

### Milestone 6.4 — CoreCommand dispatch

`apply_core_command_system`: handles LoadPatch, SavePatch, SetParameter, AddCable, RemoveCable, SetBpm, Panic.

**Tests**: Each command variant modifies world correctly.

### Milestone 6.5 — Documentation and error polish

- `#[non_exhaustive]`, `#[must_use]` audit.
- Module-level doc comments on every file.
- `cargo doc -p oxurack-core --no-deps` clean.
- README with CorePlugin setup example.

### Milestone 6.6 — Coverage and final verification

- 95%+ coverage on oxurack-core.
- `make check` for full workspace.
- `cargo doc --workspace --no-deps` clean.

**Phase 6 exit**: RT bridge functional. CoreCommand works. Docs complete. Coverage 95%+. Ready for first module consumer.

---

## Dependency Graph

```
Phase 1 (Foundation types)
    |
    v
Phase 2 (ECS integration)
    |
    v
Phase 3 (Tick cycle)
    |              \
    v               v
Phase 4 (Params,   Phase 5.1-5.2 (Patch structs + validation)
 Scale, RNG,        |
 Events, Module     |
 infrastructure)    |
    |               |
    +-------+-------+
            |
            v
     Phase 5.3-5.5 (Patch load/save/roundtrip)
            |
            v
     Phase 6 (RT bridge, CoreCommand, polish)
```

Phases 4 and 5.1-5.2 can be developed in parallel.

---

## Testing Strategy

| Level | Location | CI? |
|-------|----------|-----|
| Unit tests (types, coercion, transforms, merges, topo sort) | `#[cfg(test)]` in each file | Yes |
| Integration tests (end-to-end tick, patch round-trip) | `tests/` directory | Yes |
| RT bridge tests | `#[cfg(test)]` gated by `rt-bridge` feature | Yes (with feature) |
| Determinism tests | `tests/determinism.rs` | Yes |
| Patch format tests | `tests/patch_roundtrip.rs` with golden files | Yes |

Coverage target: 95%+ per module. All tests run without hardware.

---

## Key Patterns

- `thiserror` derive for errors (matching `oxurack-rt`)
- Edition 2024, MIT OR Apache-2.0
- `#[non_exhaustive]` on all public enums
- `&str` not `&String` for parameters
- Bevy `Component` on entity-stored types, `Resource` on world-stored types, `Reflect` on serialized types, `Event` on events
- `default-features = false` on all Bevy sub-crate deps

---

## Verification Checklist

1. `make check` passes for full workspace
2. `make coverage` shows 95%+ on oxurack-core
3. `cargo doc -p oxurack-core --no-deps` clean
4. Dummy module registers, spawns, ticks, ports read/written correctly
5. Patch saves to RON, loads in fresh App, produces bit-identical output
6. Feedback cycles rejected at patch-load
7. All merge policies correct for valid kind/policy pairs
8. Topological sort deterministic across runs
9. `derive_seed` consistent across runs and platforms
