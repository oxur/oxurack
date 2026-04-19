---
number: 7
title: "oxurack-core: ECS World, Values, Ports, and Cables"
author: "arrival order"
component: All
tags: [change-me]
created: 2026-04-18
updated: 2026-04-18
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# oxurack-core: ECS World, Values, Ports, and Cables

## 0. Purpose

This document specifies `oxurack-core` -- the crate that owns the Bevy ECS
world, the value currency, the port and cable abstractions, and the tick
cycle. It is the shared vocabulary that every module crate is written against.

It does **not** specify:

- The real-time thread (see `0006-oxurack-rt-real-time-thread.md`).
- The module-authoring interface on top of Bevy plugins (see
  `0008-oxurack-module-infrastructure.md`).
- Which concrete modules exist (see `0009-module-catalog-and-build-order.md`).

## 1. Relationship to the Rest of the System

```
+------------------------------------------------------+
| oxurack-rt     (RT thread, clock, MIDI I/O)          |
+------------------------------------------------------+
| oxurack-core   (THIS DOC: ECS, values, ports, cables)|
+------------------------------------------------------+
| oxurack-mod-*  (modules; depend on core only)        |
+------------------------------------------------------+
| oxurack        (umbrella + REPL)                     |
+------------------------------------------------------+
```

`oxurack-core` is the fattest of the core crates in terms of semantics and the
thinnest in terms of dependencies. It depends on `bevy_ecs`, `bevy_app`,
`bevy_reflect`, `bevy_utils`, plus `serde`, `ron`, `smallvec`, and
`thiserror`. It does **not** depend on `oxurack-rt`; the RT thread bridges to
core by sending `RtEvent`s through a queue that a core system consumes.

## 2. Design Principles

Restated here because they bind the decisions below:

1. **Decomposed CV is the interior currency.** Inside the world, values are
   Eurorack-style scalars (Float, Gate, Bipolar). MIDI is a boundary format
   living at I/O modules.
2. **Ports are first-class.** A cable connects port entities, not module
   entities.
3. **Merge semantics live on input ports.** A port declares how incoming
   cables combine.
4. **Cables are plain entities with components.** Not Bevy relationships.
5. **Determinism is non-negotiable.** Tick ordering, RNG seeding, and merge
   strategies must produce bit-identical output across runs for a given seed
   and patch.

## 3. Crate Layout

```
oxurack-core/
├── Cargo.toml
├── src/
│   ├── lib.rs              # re-exports + CorePlugin
│   ├── value.rs            # Value enum, ValueKind, conversions
│   ├── port.rs             # Port component, PortDirection, MergePolicy
│   ├── cable.rs            # Cable component, CableTransform
│   ├── module.rs           # Module component + ModuleId
│   ├── tick.rs             # Tick phases, scheduling, topology sort
│   ├── parameter.rs        # Parameter registry, setter dispatch
│   ├── patch.rs            # Patch serialization via bevy_reflect + RON
│   ├── scale.rs            # Musical Scale type (extracted from turingmachine)
│   ├── rng.rs              # Seed derivation helpers
│   ├── event.rs            # Cross-system events (RtEvent mirror, etc.)
│   └── error.rs            # CoreError, PatchError, TickError
└── tests/
    ├── topology.rs
    ├── merge_semantics.rs
    ├── determinism.rs
    └── patch_roundtrip.rs
```

## 4. The Value Currency

### 4.1 The Value Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum Value {
    /// Continuous unipolar scalar. Range: 0.0..=1.0 (outside range is clamped
    /// at merge-time, not production-time, so modules can over-produce briefly).
    Float(f32),

    /// Discrete on/off. The lingua franca for triggers, gates, and clock pulses.
    Gate(bool),

    /// Continuous bipolar scalar. Range: -1.0..=1.0.
    Bipolar(f32),

    /// A MIDI message. Only produced/consumed at MIDI I/O boundary modules.
    /// Carrying it through the interior is allowed but discouraged; prefer
    /// decomposing at input and recomposing at output.
    Midi(MidiMessage),

    /// Opaque u16 "anything". Escape hatch. Modules may define arbitrary
    /// semantics per-port. Merge policy for Raw is always `Reject` --
    /// Raw values never merge.
    Raw(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ValueKind {
    Float,
    Gate,
    Bipolar,
    Midi,
    Raw,
}

impl Value {
    pub fn kind(&self) -> ValueKind { /* ... */ }

    /// Lossy conversion for cable transforms. Returns None for cross-kind
    /// conversions that don't make sense (e.g., Gate -> Midi).
    pub fn try_coerce(&self, target: ValueKind) -> Option<Value> { /* ... */ }
}
```

### 4.2 Commitment: No SIMD Batching in v1

The old `0004` doc reserved space for a `BatchGenerator<T>` abstraction that
would let modules produce a batch of values per tick. In v2 we commit to
**one value per port per tick**. If a module needs to emit a burst (e.g., an
arpeggiator spitting out 16 notes in a single tick), it either:

- Schedules those notes across future ticks (preferred), or
- Sends a `Midi` value that carries a multi-note payload at the MIDI boundary.

Rationale: modules are already non-trivial; batching adds an axis of
complexity (who allocates, who drains, who copies) that we don't yet have a
concrete workload demanding. It can be added in v3 as an opt-in port trait
without breaking existing modules.

### 4.3 MidiMessage

```rust
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum MidiMessage {
    NoteOn { channel: u8, note: u8, velocity: u8 },
    NoteOff { channel: u8, note: u8, velocity: u8 },
    ControlChange { channel: u8, controller: u8, value: u8 },
    PitchBend { channel: u8, value: i16 },      // -8192..=8191
    ProgramChange { channel: u8, program: u8 },
    ChannelPressure { channel: u8, pressure: u8 },
    PolyKeyPressure { channel: u8, note: u8, pressure: u8 },
    Clock,
    Start,
    Stop,
    Continue,
    SongPosition { position: u16 },
    SystemExclusive, // payload deferred to v2
}
```

`MidiMessage` is mirrored between `oxurack-rt` and `oxurack-core`. Keeping
both in sync is a manual discipline for now; in v3 we can extract a
`oxurack-midi` crate that both depend on if drift becomes painful.

## 5. Entity Model

Three primary entity kinds live in the world:

```
Module Entity
  ├─ ModuleId (component)
  ├─ ModuleName (component)
  ├─ <module-specific state components>
  └─ Children: Port entities (via bevy_ecs hierarchy)

Port Entity
  ├─ Port (component)
  ├─ ParentModule (component, entity ref)
  ├─ CurrentValue (component, per tick)
  └─ Children: Cable entities on the input side (via PortCables)

Cable Entity
  ├─ Cable (component: source, target, transform)
  └─ no children
```

Using Bevy's hierarchy crate for module-to-port parentage is fine (it's a
tree, which matches its intent). We do **not** use hierarchy or relationships
for cable connections -- those are queried manually via an index (see §7.4).

### 5.1 Module Component

```rust
#[derive(Component, Debug, Clone, Reflect)]
pub struct Module {
    pub kind: ModuleKind,          // "turingmachine", "noise", "clock", ...
    pub instance_name: String,     // user-provided, unique per rack
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct ModuleId(pub u64);      // stable across a session; derived from
                                   // hash(patch_seed, instance_name)
```

Per-module state (e.g., `TuringMachineState`) lives as additional components
on the same entity. Module crates declare those components themselves; the
core trait requires only `Module` and `ModuleId`.

### 5.2 Port Component

```rust
#[derive(Component, Debug, Clone, Reflect)]
pub struct Port {
    pub name: PortName,               // "main_out", "cv_in", "gate_trig", ...
    pub direction: PortDirection,
    pub value_kind: ValueKind,
    pub merge_policy: MergePolicy,    // only meaningful for Input ports
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum PortDirection { Input, Output }

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct CurrentValue(pub Value);
```

`PortName` is a newtype over `SmolStr` (or `CompactString`) for cheap
cloning; we reserve the right to intern it later if profiling demands.

### 5.3 Cable Component

```rust
#[derive(Component, Debug, Clone, Reflect)]
pub struct Cable {
    pub source_port: Entity,          // an Output port entity
    pub target_port: Entity,          // an Input port entity
    pub transform: Option<CableTransform>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum CableTransform {
    /// out = in * factor + offset
    Affine { factor: f32, offset: f32 },
    /// out = -in (preserves type; only legal for Float/Bipolar)
    Invert,
    /// out = in.clamp(min, max)
    Clamp { min: f32, max: f32 },
    /// out = Gate(in > threshold)     (Float -> Gate)
    Threshold { threshold: f32 },
    /// out = if in { 1.0 } else { 0.0 }  (Gate -> Float)
    GateToFloat,
    /// out = (in * 2.0) - 1.0          (Float -> Bipolar)
    Unipolar,
    /// out = (in + 1.0) / 2.0          (Bipolar -> Float)
    Bipolarize,
}
```

Transforms are total on valid kind pairs and return `None` on mismatch; the
tick system skips cables whose transform produces `None` and records a
diagnostic (rather than panicking).

## 6. Merge Policies

A port with zero incoming cables reads the module's last-written default (or
`Value::Float(0.0)` at cold start). A port with one cable reads that cable's
transformed value. A port with **multiple** cables applies its merge policy.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum MergePolicy {
    /// Reject multi-fan-in at patch-load time. This is the DEFAULT for most
    /// ports because it forces explicit intent when combining signals.
    Reject,

    /// For Float: arithmetic mean of all inputs.
    /// For Bipolar: arithmetic mean of all inputs.
    /// Rejected at patch-load for Gate/Midi/Raw.
    Average,

    /// For Float: clamped sum.
    /// For Bipolar: clamped sum.
    /// For Gate: logical OR.
    /// Rejected at patch-load for Midi/Raw.
    Sum,

    /// For Gate: logical OR.
    /// For Float: max().
    /// For Bipolar: max().
    /// Rejected at patch-load for Midi/Raw.
    Max,

    /// For Midi: interleave by arrival order (cable iteration order).
    /// For others: rejected at patch-load.
    Interleave,

    /// For all types: the last cable (in tick-evaluation order) wins.
    /// This is deterministic because cable iteration order is stable (see §7).
    /// Useful for "priority" patterns: patch the override cable last.
    LastWins,
}
```

Merges must be **commutative for correctness** wherever order-independence
matters. `LastWins` and `Interleave` are explicitly order-dependent; the core
guarantees that cable iteration order for a given (patch, tick) is stable,
so determinism is preserved. For `Sum`/`Average`/`Max`/`OR` we rely on
floating-point associativity being "good enough" for f32 signals; patches
that hit ULP-level differences across cable-order permutations should use
`LastWins`.

Validation runs at patch-load time and produces `PatchError::IllegalMerge`
before any tick executes.

## 7. The Tick Cycle

### 7.1 Three Phases

```
┌─────────────────────────────────────────────────────────┐
│ PHASE 1: PRODUCE                                        │
│   - For each module M in topological order:             │
│       - Read M's input ports (CurrentValue)             │
│       - Run M's tick system                             │
│       - Write M's output ports (CurrentValue)           │
├─────────────────────────────────────────────────────────┤
│ PHASE 2: PROPAGATE                                      │
│   - For each cable C in stable order:                   │
│       - If C.enabled:                                   │
│           - Read source_port.CurrentValue               │
│           - Apply C.transform                           │
│           - Contribute to target_port's merge buffer    │
├─────────────────────────────────────────────────────────┤
│ PHASE 3: CONSUME                                        │
│   - For each input port P with ≥1 contribution:         │
│       - Apply P.merge_policy across contributions       │
│       - Write merged Value to P.CurrentValue            │
│   - For each input port P with 0 contributions:         │
│       - Retain previous CurrentValue (sample-and-hold)  │
└─────────────────────────────────────────────────────────┘
```

Phase ordering within a tick is enforced by Bevy system set labels:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TickPhase {
    Produce,
    Propagate,
    Consume,
}
```

Module crates register their tick systems under `TickPhase::Produce`.
The propagate and consume phases are owned by `oxurack-core` and not
directly extensible.

### 7.2 Module Ordering Within Produce

Topological sort of the module graph, where edges are derived from cables
(source module → target module). Tie-breaking uses `ModuleId` (which is
derived from a stable hash, so the ordering is deterministic across runs).

Cycles in the cable graph are handled in §7.5.

### 7.3 Cable Ordering Within Propagate

Cables are iterated in order of `(target_port_entity, insertion_index)`.
Insertion index is tracked in a resource (`CableInsertionOrder`) updated
whenever a cable is spawned. This gives us:

- Stable, deterministic iteration.
- "Last cable patched in wins" for `LastWins` merge.
- "First-to-last" interleaving for `Interleave` merge.

### 7.4 Cable Indexing

Naive cable iteration scans all cable entities each tick. For racks with
≥100 cables that's fine, but we want O(1) lookup by target port for merge
contribution. Maintain a resource:

```rust
#[derive(Resource, Default)]
pub struct CableIndex {
    /// target_port_entity -> Vec<cable_entity>, sorted by insertion order
    pub by_target: HashMap<Entity, SmallVec<[Entity; 4]>>,
    /// source_port_entity -> Vec<cable_entity>, for debug/visualization
    pub by_source: HashMap<Entity, SmallVec<[Entity; 4]>>,
}
```

Maintained by an observer on cable spawn/despawn. Tick systems read it,
never mutate it.

### 7.5 Feedback Cycles -- Commitment for v1

**Decision: v1 rejects feedback cycles at patch-load time.**

A cycle in the cable graph is an error. Users wanting feedback patterns must
wait for v2, which will introduce an explicit `FeedbackDelay` cable transform
that introduces a one-tick delay and breaks the cycle. This keeps topology
sort simple, keeps determinism obvious, and doesn't preclude the feature.

Detection: Tarjan's strongly-connected-components on load; any SCC of size
>1 (or a self-loop) is `PatchError::FeedbackCycle { modules: Vec<String> }`.

### 7.6 Tick Invocation

`oxurack-core` exposes a tick trigger event:

```rust
#[derive(Event, Debug, Clone, Copy)]
pub struct TickNow { pub frame: u64 }
```

The RT thread (or test code, or a manual driver) emits a `TickNow` event
through the `RtEvent::TickNow` → `tick_driver_system` bridge. The rack's
`Update` schedule runs once per emitted `TickNow`. No polling.

## 8. Parameters vs. Ports

A **port** is a tick-granularity value surface: it carries a single `Value`
per tick, participates in cables, and has a merge policy.

A **parameter** is a control-rate value: the kind of thing a user tweaks
from the REPL or turns a nanoKONTROL2 knob for. Parameters are:

- Named strings on a module ("write_probability", "bpm", "scale").
- Not tickable -- changing a parameter is an out-of-band event.
- Serialized with the patch.
- Implemented as regular components on the module entity, with a registry
  that maps `(ModuleId, ParameterName)` to a setter function.

```rust
#[derive(Resource, Default)]
pub struct ParameterRegistry {
    setters: HashMap<(ModuleKind, ParameterName),
                     fn(&mut World, Entity, ParameterValue) -> Result<(), CoreError>>,
}
```

A `ParameterValue` is a smaller enum than `Value`:

```rust
pub enum ParameterValue {
    Float(f32),
    Int(i64),
    Bool(bool),
    String(String),
    Scale(Scale),
}
```

The boundary between "this should be a port" and "this should be a
parameter" is:

- If it makes sense to modulate this from another module (via a cable), make
  it a port, optionally with a parameter setter that writes the same
  component (so REPL+cable both work).
- If it's pure configuration (like "which scale to quantize to"), make it a
  parameter only.

### 8.1 Dual-Surface Fields

A field exposed as both a port and a parameter:

- The parameter is the **default**; applied when there are zero cables.
- The port is the **override**; applied when ≥1 cable is connected.
- This is the VCV Rack convention (knob + CV input sharing a destination).

## 9. Scale Type (Moved from turingmachine)

The `Scale` type, previously defined in the Turing Machine crate, moves
here. Its API remains identical:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub struct Scale {
    pub root: u8,                    // 0..=11
    pub intervals: SmallVec<[u8; 12]>,
    pub name: Option<String>,
}

impl Scale {
    pub fn quantize(&self, note: u8) -> u8 { /* ... */ }
    pub fn chromatic() -> Self { /* ... */ }
    pub fn major(root: u8) -> Self { /* ... */ }
    pub fn minor(root: u8) -> Self { /* ... */ }
    // ... existing constructors
}
```

The turingmachine crate re-exports `oxurack_core::Scale` for back-compat and
eventually drops the re-export on a major version.

A future `oxurack-mod-quantizer` module uses this type.

## 10. RNG and Determinism

```rust
pub fn derive_seed(master_seed: u64, instance_name: &str) -> u64 {
    // SipHash-1-3 (via ahash), or blake3, or xxhash -- stable algorithm choice.
    // We pick one and document it; changing it is a breaking change.
}
```

Every module that uses randomness:

- Takes a seed in its constructor.
- Stores the RNG as a component.
- Uses only that RNG for random choices during tick.

The rack's master seed is stored in `Patch::master_seed` and set once per
patch load. Per-module seeds are derived via `derive_seed(master_seed,
module.instance_name)`. Renaming a module changes its RNG stream, which is
both a feature (explicit seed control) and a hazard (accidental rename
breaks reproducibility). We document the hazard and ship tools to preserve
a stable seed mapping during rename if needed.

## 11. Patch Persistence

Patches serialize to RON (Rust Object Notation) via `bevy_reflect`:

```ron
Patch(
    version: "1.0",
    master_seed: 42,
    bpm: 120.0,
    clock_mode: Master,
    modules: [
        ModuleConfig(
            kind: "turingmachine",
            instance_name: "tm1",
            parameters: {
                "length": Int(16),
                "write_probability": Float(0.25),
                "scale": Scale((root: 0, intervals: [0,2,4,5,7,9,11], name: Some("major"))),
            },
        ),
        ModuleConfig(
            kind: "noise",
            instance_name: "rng1",
            parameters: {},
        ),
    ],
    cables: [
        CableConfig(
            source: ("rng1", "out"),
            target: ("tm1", "cv_in"),
            transform: Some(Affine(factor: 1.0, offset: 0.0)),
        ),
    ],
)
```

Load flow:

1. Parse RON into `Patch` struct.
2. Validate: every `ModuleKind` is registered; every port reference
   resolves; merge policies are legal for their kinds; no feedback cycles.
3. Spawn module entities (each module's registered spawner function runs).
4. Spawn port entities as children.
5. Spawn cable entities; update `CableIndex`.
6. Set master seed; derive per-module seeds.
7. Emit `PatchLoaded` event. The rack is now tickable.

Save flow is the inverse: query all `Module`, `Port`, `Cable` entities,
emit the RON representation.

RON is chosen over JSON/TOML because:

- It's Rust-native (bevy_reflect supports it out of the box).
- It handles enums cleanly (critical for `Value` and `CableTransform`).
- It's readable enough for hand-editing (though that's not the primary use).

## 12. Bridging to the RT Thread

`oxurack-core` owns the ECS-side of the `oxurack-rt` queue integration:

### 12.1 RtEvent Consumption

A system `drain_rt_events_system` runs at the start of each frame in the
`PreUpdate` schedule:

```rust
fn drain_rt_events_system(
    mut handles: ResMut<RtHandles>,
    mut tick_events: EventWriter<TickNow>,
    mut transport_events: EventWriter<TransportChanged>,
    mut midi_in_events: EventWriter<MidiInReceived>,
) {
    while let Some(event) = handles.try_recv() {
        match event {
            RtEvent::TickNow { frame } => tick_events.send(TickNow { frame }),
            RtEvent::Transport(t)      => transport_events.send(TransportChanged(t)),
            RtEvent::MidiIn(msg)       => midi_in_events.send(MidiInReceived(msg)),
            // ...
        }
    }
}
```

This translates RT-thread messages into Bevy events, which the rest of the
ECS consumes via `EventReader`.

### 12.2 EcsCommand Production

A system `flush_midi_output_system` runs at the end of each frame in
`PostUpdate`, collecting `EcsCommand::MidiOut(..)` messages produced by MIDI
output modules and pushing them to the RT queue. If the queue is full, we
drop with a diagnostic (not panic; RT must never be blocked by ECS).

### 12.3 Command Interface for the REPL

REPL commands (also defined in `oxurack`, not `oxurack-core`, but the
command types live here for module crates to reference) go through a
similar channel:

```rust
#[derive(Event, Debug, Clone)]
pub enum CoreCommand {
    LoadPatch(PathBuf),
    SavePatch(PathBuf),
    SetParameter { module: String, param: String, value: ParameterValue },
    AddCable { source: (String, String), target: (String, String), transform: Option<CableTransform> },
    RemoveCable { source: (String, String), target: (String, String) },
    SetClockMode(ClockMode),
    SetBpm(f32),
    Panic,       // all MIDI notes off, stop transport
}
```

Core provides `apply_core_command_system` that handles these events.

## 13. Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("patch error: {0}")]
    Patch(#[from] PatchError),

    #[error("tick error: {0}")]
    Tick(#[from] TickError),

    #[error("parameter '{param}' not found on module '{module}'")]
    UnknownParameter { module: String, param: String },

    #[error("parameter '{param}' on module '{module}' rejected value: {reason}")]
    InvalidParameterValue { module: String, param: String, reason: String },
}

#[derive(Debug, thiserror::Error)]
pub enum PatchError {
    #[error("unknown module kind: '{0}'")]
    UnknownModuleKind(String),

    #[error("duplicate instance name: '{0}'")]
    DuplicateInstanceName(String),

    #[error("unknown port '{port}' on module '{module}'")]
    UnknownPort { module: String, port: String },

    #[error("merge policy {policy:?} is not legal for value kind {kind:?} on port '{module}::{port}'")]
    IllegalMerge { module: String, port: String, kind: ValueKind, policy: MergePolicy },

    #[error("feedback cycle detected through modules: {0:?}")]
    FeedbackCycle(Vec<String>),

    #[error("cable source and target value kinds are incompatible: {source:?} -> {target:?}")]
    KindMismatch { source: ValueKind, target: ValueKind },

    #[error("RON parse error: {0}")]
    Deserialize(#[from] ron::error::SpannedError),
}

#[derive(Debug, thiserror::Error)]
pub enum TickError {
    #[error("module '{0}' panicked during tick")]
    ModulePanic(String),

    #[error("RT queue full; dropped {0} MIDI events this frame")]
    MidiQueueOverflow(usize),
}
```

Errors during `tick()` do not crash the rack. A module panic is caught (the
tick runs inside `std::panic::catch_unwind`), the module is marked as
"faulted", and subsequent ticks skip it with a warning. The REPL can
`reset <module>` to reinstate it.

## 14. Testing Strategy

### 14.1 Unit Tests

- `value.rs`: coercion table (every valid kind→kind conversion, every invalid
  one returns `None`).
- `cable.rs`: transform application correctness across all valid kind pairs.
- `port.rs`: merge policy correctness for each `(kind, policy)` pair.

### 14.2 Integration Tests

- **Topology**: construct a 10-module patch with known dependencies,
  verify tick order is topologically correct and deterministic across runs.
- **Merge semantics**: wire three Noise modules to one Turing Machine
  `cv_in` with `Average` merge; verify the averaged value matches the
  expected formula.
- **Feedback detection**: construct a cycle, verify `PatchError::FeedbackCycle`
  with correct module list.
- **Determinism**: same patch + same seed → bit-identical output across 10,000
  ticks across three runs. Assert byte-equal output streams.

### 14.3 Patch Roundtrip

- Serialize a complex patch to RON, deserialize, tick both, verify outputs
  are identical for ≥1000 ticks.

### 14.4 Property-Based

Using `proptest`:

- Arbitrary patches (random module kinds, random cable topology that avoids
  cycles) must either load successfully or fail with a well-defined error.
- For loadable patches: N ticks never panic.

## 15. Performance Notes

- `CurrentValue` is `Copy` (16 bytes). Per-tick ECS writes are cheap.
- `CableIndex` uses `SmallVec<[Entity; 4]>` so most ports (≤4 incoming cables)
  avoid heap allocation in the merge buffer.
- Topological sort runs only when the cable graph changes (on patch load and
  on `AddCable`/`RemoveCable`). Cached thereafter in a `TickOrder` resource.
- Bevy system parallelism: `TickPhase::Propagate` and `TickPhase::Consume`
  run single-threaded (they touch the full port set). `TickPhase::Produce`
  *could* run in parallel for modules with disjoint port sets, but v1 runs
  it single-threaded for determinism and simplicity. Parallel produce is a
  v2 optimization guarded by a feature flag.

At target loads (≤50 modules, ≤200 cables, ≤1kHz tick rate), single-threaded
execution fits comfortably in a fraction of a millisecond on a modest
machine. This leaves the RT thread with headroom to do its job.

## 16. Re-exports and Public API Surface

`oxurack-core`'s `lib.rs`:

```rust
pub use self::value::{Value, ValueKind, MidiMessage};
pub use self::port::{Port, PortDirection, PortName, MergePolicy, CurrentValue};
pub use self::cable::{Cable, CableTransform};
pub use self::module::{Module, ModuleId, ModuleKind};
pub use self::tick::{TickPhase, TickNow};
pub use self::parameter::{ParameterRegistry, ParameterName, ParameterValue};
pub use self::patch::{Patch, ModuleConfig, CableConfig, ClockMode};
pub use self::scale::Scale;
pub use self::rng::derive_seed;
pub use self::event::{
    RtEvent, TransportChanged, MidiInReceived, CoreCommand,
};
pub use self::error::{CoreError, PatchError, TickError};

pub struct CorePlugin;

impl bevy::prelude::Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TickNow>()
           .add_event::<TransportChanged>()
           .add_event::<MidiInReceived>()
           .add_event::<CoreCommand>()
           .init_resource::<CableIndex>()
           .init_resource::<TickOrder>()
           .init_resource::<ParameterRegistry>()
           .configure_sets(Update, (
               TickPhase::Produce,
               TickPhase::Propagate,
               TickPhase::Consume,
           ).chain())
           .add_systems(Update, (
               propagate_cables_system.in_set(TickPhase::Propagate),
               consume_ports_system.in_set(TickPhase::Consume),
           ))
           .add_systems(PreUpdate, drain_rt_events_system)
           .add_systems(PostUpdate, flush_midi_output_system);
    }
}
```

Every downstream consumer adds `CorePlugin` to their `App` and gets the
full tick machinery.

## 17. Open Questions

1. **Raw(u16) as a true escape hatch or over-engineering?** We preserved it
   from `0004` on the theory that occasionally a module will want to pipe
   through something that doesn't fit the scalar shapes. If nothing in the
   v1 module catalog uses it, consider dropping it before 1.0.

2. **Should merge policy live on the `Cable` (per-cable override) as well as
   the `Port`?** Current design says "no": merge policy is a property of the
   input. A cable that wanted different semantics at a different destination
   can use a `CableTransform`. But this is worth revisiting once real
   patches exist.

3. **Hot-reload of patches.** Current design spawns/despawns everything on
   load. For live performance, we may want delta-reload: diff new patch
   against current world and apply minimum changes. Deferred to v2.

4. **Observability.** How does a user inspect live port values? A
   `DebugInspector` resource that snapshots `CurrentValue` for all ports
   once per tick is cheap; rendering it (TUI? egui? REPL print?) is an
   `oxurack` crate concern, not core.

5. **Port aliases.** Should a module be able to declare `"cv"` as an alias
   for `"cv_in"` in its port table? Simplifies patching but adds a lookup
   layer. Deferred; flat names are fine for v1.

## 18. Lineage

This design keeps the structural decisions of the old `0004` doc that
survived scrutiny:

- `Value` enum with `Float`/`Gate`/`Bipolar`/`Midi`/`Raw` variants
- `ValueKind` as a parallel tag enum
- `CableTransform` (extended with more variants)
- `Scale` extraction from `turingmachine` to `core`
- `Patch` + per-module config for serialization

And replaces:

- The `Module` trait with associated `Output` type → Bevy component
  composition
- `DynModule`/`ModuleBox` dynamic dispatch → Bevy systems over entity
  queries
- Scalar per-module tick → three-phase ECS schedule
- Cable fan-in hazards → declared `MergePolicy` on input ports

The old `0004` identified the problem space correctly; the old trait-based
solution just pre-dated the decision to use Bevy. With Bevy as the
substrate, most of `0004`'s "rack owns everything" machinery becomes the
ECS World itself.
