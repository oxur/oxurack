---
number: 8
title: "oxurack Module Infrastructure: Authoring Modules on Bevy"
author: "string name"
component: All
tags: [change-me]
created: 2026-04-18
updated: 2026-04-18
state: Draft
supersedes: 4
superseded-by: null
version: 1.0
---

# oxurack Module Infrastructure: Authoring Modules on Bevy

## 0. Purpose

This document specifies how a module is **written**. It covers:

- What a "module crate" looks like on disk.
- The `OxurackModule` trait (thin wrapper over `bevy::Plugin`).
- How modules declare their ports, parameters, and tick system.
- How modules register themselves so the patch-loader can instantiate them
  by string name.
- Conventions for naming, feature flags, and dependencies.

It does **not** cover:

- The ECS world itself (see `0007-oxurack-core-ecs-world.md`).
- The specific modules we plan to ship (see `0009-module-catalog-and-build-order.md`).
- The RT thread (see `0006-oxurack-rt-real-time-thread.md`).

## 1. Goals

1. **A module is a small crate.** Writing a new module should feel like
   writing a ~200-line Rust crate, not understanding a framework.
2. **Standalone use must be preserved.** Every module must compile and run
   without `oxurack` the umbrella crate, depending only on `oxurack-core`.
3. **Bevy shows through, and that's fine.** Module authors will touch
   Bevy systems, components, and `Plugin::build`. We don't hide the
   substrate; we provide light-touch helpers.
4. **Naming is the author's choice.** `pub struct TuringMachineModule;`
   with `impl OxurackModule` and `impl Plugin` -- no suffix enforcement.
5. **One registration call, everything wires up.** `app.add_plugins(TuringMachineModule)`
   is enough: ports declared, parameters registered, tick system scheduled,
   spawner function available to the patch loader.

## 2. The OxurackModule Trait

```rust
/// A module in an oxurack rack. Every module must implement this trait
/// AND `bevy::Plugin` (via a blanket impl or manually).
pub trait OxurackModule: Send + Sync + 'static {
    /// Stable kind identifier. Used in patch files and the module registry.
    /// Must be unique across all registered modules. Convention: kebab-case
    /// matching the crate name minus the "oxurack-mod-" prefix.
    const KIND: &'static str;

    /// Human-readable name for UIs and error messages.
    const DISPLAY_NAME: &'static str;

    /// A brief description shown in module browsers.
    const DESCRIPTION: &'static str = "";

    /// Port metadata, evaluated at registration time. Declaring ports here
    /// lets the patch loader validate cable connections without instantiating.
    fn port_schema() -> &'static [PortSchema];

    /// Parameter metadata.
    fn parameter_schema() -> &'static [ParameterSchema];

    /// Instantiate this module into the world. Returns the module entity.
    /// Called by the patch loader once per ModuleConfig.
    fn spawn(
        world: &mut World,
        instance_name: &str,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError>;
}
```

### 2.1 Port Schema

```rust
pub struct PortSchema {
    pub name: &'static str,
    pub direction: PortDirection,
    pub value_kind: ValueKind,
    pub merge_policy: MergePolicy,   // meaningful for Input ports only
    pub description: &'static str,   // for docs and introspection
}
```

Ports are static per-module-kind. A module does not add or remove ports at
runtime; if dynamic I/O is needed (e.g., a mixer with N inputs), the module
exposes a parameter `input_count` and declares the maximum via schema, then
masks unused ports internally. This keeps patch serialization tractable.

### 2.2 Parameter Schema

```rust
pub struct ParameterSchema {
    pub name: &'static str,
    pub kind: ParameterKind,
    pub default: ParameterValue,
    pub range: Option<ParameterRange>,   // for numeric kinds, clamping info
    pub description: &'static str,
}

pub enum ParameterKind {
    Float,
    Int,
    Bool,
    String,
    Scale,
    /// An enum with known string variants ("major", "minor", ...).
    Choice(&'static [&'static str]),
}

pub enum ParameterRange {
    Float { min: f32, max: f32, step: Option<f32> },
    Int { min: i64, max: i64 },
}
```

## 3. How a Module Crate Is Structured

```
oxurack-mod-turingmachine/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs          # re-exports TuringMachineModule
    ├── module.rs       # impl OxurackModule, impl Plugin
    ├── state.rs        # TuringMachineState component
    ├── systems.rs      # tick_system, parameter setter fns
    └── ports.rs        # port constants + schema
```

The canonical layout. Small modules (like `oxurack-mod-range`) will collapse
several of these files into one.

### 3.1 Cargo.toml

```toml
[package]
name = "oxurack-mod-turingmachine"
version = "0.1.0"
edition = "2021"

[dependencies]
oxurack-core = { path = "../oxurack-core", version = "0.1" }
bevy_ecs = "0.18"
bevy_app = "0.18"
bevy_reflect = "0.18"
rand = { version = "0.8", default-features = false, features = ["small_rng"] }
smallvec = "1"
serde = { version = "1", features = ["derive"] }

[features]
default = []
# Reserved for future module-specific features.
```

No dependency on `oxurack` (umbrella). No dependency on other module crates.
A module is a leaf in the crate DAG.

### 3.2 lib.rs

```rust
//! Turing Machine Mk2 as an oxurack module.
//!
//! See the README and oxurack design doc 0001 for the algorithm details.

mod module;
mod ports;
mod state;
mod systems;

pub use module::TuringMachineModule;
pub use state::TuringMachineState;
pub use ports::*;   // port name constants for external use
```

### 3.3 module.rs -- The Full Plugin

```rust
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use oxurack_core::{
    CoreError, MergePolicy, OxurackModule, Port, PortDirection, PortSchema,
    ParameterKind, ParameterRange, ParameterSchema, ParameterValue,
    TickPhase, ValueKind,
};

use crate::{ports::*, state::TuringMachineState, systems::*};

pub struct TuringMachineModule;

impl OxurackModule for TuringMachineModule {
    const KIND: &'static str = "turingmachine";
    const DISPLAY_NAME: &'static str = "Turing Machine Mk2";
    const DESCRIPTION: &'static str =
        "A generative shift-register sequencer inspired by Music Thing's \
         Turing Machine. Produces note, velocity, and gate streams.";

    fn port_schema() -> &'static [PortSchema] {
        &[
            PortSchema {
                name: PORT_NOTE_OUT,
                direction: PortDirection::Output,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::Reject, // N/A for outputs
                description: "Quantized note value (0.0..=1.0 over range)",
            },
            PortSchema {
                name: PORT_GATE_OUT,
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Gate for current step",
            },
            PortSchema {
                name: PORT_VEL_OUT,
                direction: PortDirection::Output,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::Reject,
                description: "Velocity for current step",
            },
            PortSchema {
                name: PORT_CLOCK_IN,
                direction: PortDirection::Input,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Max, // gate OR
                description: "Advance-step trigger",
            },
            PortSchema {
                name: PORT_WRITE_IN,
                direction: PortDirection::Input,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::LastWins,
                description: "CV modulation of write probability",
            },
        ]
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        &[
            ParameterSchema {
                name: "length",
                kind: ParameterKind::Int,
                default: ParameterValue::Int(16),
                range: Some(ParameterRange::Int { min: 1, max: 32 }),
                description: "Shift-register length",
            },
            ParameterSchema {
                name: "write_probability",
                kind: ParameterKind::Float,
                default: ParameterValue::Float(0.25),
                range: Some(ParameterRange::Float {
                    min: 0.0, max: 1.0, step: Some(0.01)
                }),
                description: "Probability of flipping a bit on step",
            },
            ParameterSchema {
                name: "scale",
                kind: ParameterKind::Scale,
                default: ParameterValue::Scale(oxurack_core::Scale::chromatic()),
                range: None,
                description: "Quantization scale applied to note output",
            },
            ParameterSchema {
                name: "seed",
                kind: ParameterKind::Int,
                default: ParameterValue::Int(0),
                range: None,
                description: "RNG seed (0 = derive from master seed)",
            },
        ]
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        parameters: &std::collections::HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let state = TuringMachineState::from_parameters(parameters)?;
        let entity = oxurack_core::spawn_module::<Self>(world, instance_name, state)?;
        Ok(entity)
    }
}

impl Plugin for TuringMachineModule {
    fn build(&self, app: &mut App) {
        oxurack_core::register_module::<Self>(app);
        app.add_systems(Update, tick_turingmachine.in_set(TickPhase::Produce));
        app.register_type::<TuringMachineState>();
    }
}
```

Note the division:

- `impl OxurackModule`: declarative metadata and spawn logic. Pure data and
  deterministic instantiation.
- `impl Plugin`: imperative wiring into Bevy. Calls the helper
  `register_module::<Self>(app)` from `oxurack-core` which populates the
  `ModuleRegistry` resource and registers parameter setters for every
  `ParameterSchema`.

### 3.4 The register_module Helper

Provided by `oxurack-core`:

```rust
pub fn register_module<M: OxurackModule>(app: &mut App) {
    let registry = app.world_mut().resource_mut::<ModuleRegistry>();
    registry.register(ModuleRegistration {
        kind: M::KIND,
        display_name: M::DISPLAY_NAME,
        description: M::DESCRIPTION,
        port_schema: M::port_schema(),
        parameter_schema: M::parameter_schema(),
        spawner: |world, name, params| M::spawn(world, name, params),
    });

    // Register standard parameter setters for each schema entry that
    // corresponds to a plain component field.
    for param in M::parameter_schema() {
        // (Module-specific setters registered by the module itself via
        // `register_parameter_setter::<M, _>(app, "name", setter_fn)`)
    }
}
```

Module authors register custom setters explicitly when a parameter doesn't
correspond 1:1 with a field (e.g., "length" requires reallocating the
shift register):

```rust
oxurack_core::register_parameter_setter::<Self, _>(
    app, "length", set_length_param,
);
```

### 3.5 ports.rs

```rust
//! Port name constants. Using constants (rather than string literals
//! sprinkled through the code) catches typos at compile time and makes
//! port renames one-line changes.

pub const PORT_NOTE_OUT: &str = "note_out";
pub const PORT_GATE_OUT: &str = "gate_out";
pub const PORT_VEL_OUT: &str  = "vel_out";
pub const PORT_CLOCK_IN: &str = "clock_in";
pub const PORT_WRITE_IN: &str = "write_in";
```

### 3.6 state.rs

```rust
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use oxurack_core::{CoreError, ParameterValue, Scale};
use rand::rngs::SmallRng;
use smallvec::SmallVec;

#[derive(Component, Debug, Clone, Reflect)]
pub struct TuringMachineState {
    pub register: SmallVec<[bool; 32]>,
    pub length: usize,
    pub write_probability: f32,
    pub scale: Scale,
    pub rng_seed: u64,
    #[reflect(ignore)]
    pub rng: SmallRng,
    pub step_index: usize,
}

impl TuringMachineState {
    pub fn from_parameters(
        params: &std::collections::HashMap<String, ParameterValue>,
    ) -> Result<Self, CoreError> {
        // ... read params with defaults, return populated state
    }
}
```

Note the `#[reflect(ignore)]` on the RNG field: `bevy_reflect` can't (and
shouldn't) serialize RNG internal state. The seed is serialized; the RNG is
reconstructed from the seed on patch load.

### 3.7 systems.rs

```rust
use bevy_ecs::prelude::*;
use oxurack_core::{CurrentValue, Port, PortDirection, Value};

use crate::state::TuringMachineState;
use crate::ports::*;

/// Tick system for Turing Machine. Queries all TM modules' states and their
/// child ports; produces outputs during Phase 1 (Produce).
pub fn tick_turingmachine(
    mut tm_q: Query<(Entity, &mut TuringMachineState, &Children)>,
    mut port_q: Query<(&Port, &mut CurrentValue)>,
) {
    for (_tm_entity, mut state, children) in tm_q.iter_mut() {
        // Read inputs first
        let clock_in = read_input_port(&port_q, children, PORT_CLOCK_IN);
        let write_mod = read_input_port(&port_q, children, PORT_WRITE_IN);

        // Advance the TM only on a rising clock edge.
        let should_tick = matches!(clock_in, Some(Value::Gate(true)));
        if !should_tick { continue; }

        let effective_write = effective_write_probability(
            &state, write_mod,
        );

        state.advance_step(effective_write);

        let (note_val, gate_val, vel_val) = state.outputs();

        // Write outputs
        write_output_port(&mut port_q, children, PORT_NOTE_OUT, Value::Float(note_val));
        write_output_port(&mut port_q, children, PORT_GATE_OUT, Value::Gate(gate_val));
        write_output_port(&mut port_q, children, PORT_VEL_OUT, Value::Float(vel_val));
    }
}

// Helper fns (read/write port by name) would live in oxurack-core as utilities.
```

`read_input_port`/`write_output_port` are helper functions provided by
`oxurack-core` that look up a child port entity by name and read/write its
`CurrentValue`. These are slightly slower than direct entity refs (one
hashmap lookup per port), but simpler to write. Performance-critical
modules can cache port entity refs in their state on first tick.

## 4. Parameter Setters

A parameter setter is a function invoked by the REPL or patch loader to
mutate module state:

```rust
pub type ParameterSetter = fn(
    world: &mut World,
    module_entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError>;
```

Example for the Turing Machine's "length" parameter:

```rust
fn set_length_param(
    world: &mut World,
    module_entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError> {
    let length = match value {
        ParameterValue::Int(n) if (1..=32).contains(&n) => n as usize,
        ParameterValue::Int(n) => return Err(CoreError::InvalidParameterValue {
            module: query_name(world, module_entity),
            param: "length".into(),
            reason: format!("length must be 1..=32, got {}", n),
        }),
        _ => return Err(CoreError::InvalidParameterValue {
            module: query_name(world, module_entity),
            param: "length".into(),
            reason: "expected Int".into(),
        }),
    };

    let mut state = world.get_mut::<TuringMachineState>(module_entity)
        .ok_or_else(/* ... */)?;
    state.resize_register(length);
    Ok(())
}
```

Setters are allowed to have side effects (e.g., reallocating a register) but
must be idempotent for the same input value (a patch-load that sets
`length=16` three times in a row must leave the module in the same state as
setting it once).

## 5. Spawn Flow

When the patch loader encounters a `ModuleConfig` with kind `turingmachine`:

```
1. Look up ModuleRegistry.get("turingmachine") -> ModuleRegistration.
2. Call registration.spawner(world, instance_name, &parameters).
3. The spawner:
   a. Builds the initial TuringMachineState from parameters.
   b. Calls oxurack_core::spawn_module::<Self>(world, name, state) which:
      - Spawns the module entity with (Module, ModuleId, TuringMachineState).
      - For each port in Self::port_schema(), spawns a child port entity
        with (Port, ParentModule(module_entity), CurrentValue(default)).
      - Returns the module entity.
4. Patch loader records (instance_name -> module_entity) in its local map.
```

After all modules are spawned, the loader spawns cables (which need module
entities to already exist).

## 6. Conventions

### 6.1 Naming

- Crate names: `oxurack-mod-{kind}` where `{kind}` matches `Module::KIND`.
- Struct name: `{ModuleName}Module` or any clear name the author chooses
  (no framework enforcement).
- Port names: `snake_case`. Convention: `<purpose>_{in|out}` (e.g.,
  `note_out`, `clock_in`, `cv_in`).
- Parameter names: `snake_case`. Often match port names for dual-surface
  fields.

### 6.2 Determinism Obligations

A module author who uses randomness MUST:

- Store the RNG seed in state (serializable).
- Reconstruct the RNG from the seed on load (not store the RNG's internal
  state).
- Use `oxurack_core::derive_seed(master_seed, instance_name)` if the
  parameter seed is 0.
- Use a stable RNG (`SmallRng` is a fine default, but lock the choice at
  1.0; changing it is a breaking change for patch reproducibility).

### 6.3 I/O Prohibitions

A module MUST NOT:

- Open files, sockets, audio devices, or MIDI ports from a tick system.
- Spawn threads.
- Perform blocking operations (sleep, condvar wait, etc.).
- Panic on recoverable errors (return `Err(CoreError::...)` instead; panics
  are caught but mark the module faulted).

MIDI I/O specifically is handled by the RT thread and a dedicated
MIDI-output module that communicates with it via queues. Module authors do
not talk to `midir` directly.

### 6.4 Allocation

In-tick allocation is allowed but discouraged. Preferred patterns:

- Allocate once in `spawn`, reuse buffers in tick.
- `SmallVec<[_; N]>` for small unbounded collections.
- `thread_local!` RNG avoidance: RNGs live in state.

The RT thread has strict allocation prohibitions; the ECS tick does not,
because it runs on a worker thread that's tolerant of occasional GC/alloc
hiccups. But we still care, because predictable runtime helps scale.

## 7. Feature Flags

`oxurack-core` exposes:

```toml
[features]
default = ["ron"]
ron = ["dep:ron", "bevy_reflect/ron"]
std = []
```

Module crates typically don't need features of their own, but may expose:

- `simd` for optional SIMD-accelerated value generation (if applicable).
- Specific algorithm variants (`xoshiro256` vs. `small_rng`).

## 8. Documentation Obligations

Every module crate ships:

- A `README.md` covering: what it models, key parameters, example patches.
- Rustdoc on all public types.
- At least one integration test exercising the module end-to-end inside a
  test rack.
- Optionally, a `examples/` directory with runnable demos.

## 9. Testing a Module

### 9.1 Unit Tests

Module authors write tests for their algorithm directly against the state
type, without Bevy:

```rust
#[test]
fn shift_register_preserves_pattern_at_zero_write_prob() {
    let mut state = TuringMachineState::new(16, 0.0, Scale::chromatic(), 42);
    let before = state.register.clone();
    for _ in 0..32 { state.advance_step(0.0); }
    assert_eq!(state.register, before);
}
```

### 9.2 Integration Tests

Module authors write tests that construct a minimal Bevy App, add the
module plugin + `CorePlugin`, spawn an instance, emit `TickNow` events, and
assert port values:

```rust
#[test]
fn integration_tick_produces_gate_outputs() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, CorePlugin, TuringMachineModule));
    let tm_entity = oxurack_core::spawn_test_module::<TuringMachineModule>(
        &mut app.world_mut(),
        "tm1",
        &default_params(),
    ).unwrap();
    // ... tick and assert
}
```

`oxurack-core` provides `spawn_test_module` as a convenience that skips the
full patch-load machinery.

## 10. Stability and Versioning

- `OxurackModule::KIND` must be stable across module versions. Changing it
  breaks patch files.
- `PortSchema.name` and `ParameterSchema.name` must be stable across minor
  versions; renames require a major version and a migration note.
- Adding new optional parameters (with defaults) is a minor version bump.
- Adding new ports is a minor version bump.
- Removing ports or parameters is a major version bump.

A module crate's semver tracks its public API plus these stability
guarantees. The `oxurack` umbrella re-exports module crates and bumps its
own version accordingly.

## 11. What About Dynamic Loading?

We considered but rejected runtime-loadable module DLLs for v1. Rationale:

- Rust doesn't have a stable ABI; dylib boundaries are fragile.
- The workflow we're optimizing is "edit module code, `cargo run`",
  which is already fast enough with `bevy_dylib` for development.
- Live patching (changing parameters and cables of already-loaded modules)
  covers 90% of the live-experimentation use case.

A v2 feature could add WASM-based plugin support via `wasmtime`, which has
a stable ABI and good sandboxing. Deferred.

## 12. Summary

A module is:

1. A Rust crate depending only on `oxurack-core`.
2. A struct implementing `OxurackModule` (metadata + spawn logic) and
   `Plugin` (scheduling + registration).
3. A `Component` for per-instance state.
4. One or more Bevy systems that run in `TickPhase::Produce`.
5. Zero or more parameter setter functions registered with core.

`app.add_plugins(TuringMachineModule)` is the only user-facing integration
point. From that one line, the patch loader can spawn turing machines by
name, the REPL can set their parameters, and the rack can route cables to
and from them.
