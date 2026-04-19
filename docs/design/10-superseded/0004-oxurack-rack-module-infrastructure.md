---
number: 4
title: "Oxurack Rack Module Infrastructure"
author: "one step"
component: All
tags: [change-me]
created: 2026-04-06
updated: 2026-04-18
state: Superseded
supersedes: null
superseded-by: 8
version: 1.0
---

# Oxurack Rack Module Infrastructure

## Purpose

This document specifies the shared infrastructure needed before building new
modules. The `oxurack-core` crate provides the `Module` trait, value types,
the batch-generation pattern, and routing primitives that all modules depend on.

The `oxurack` crate (built later) provides the orchestration layer --
it depends on `oxurack-core` but no module crate depends on it.

---

## 1. The Module Trait

### Requirements

- Every module must be usable standalone (construct, configure, tick, read
  outputs) without importing the rack.
- Every module must be usable within a rack (the rack calls tick, reads
  outputs, routes values to other modules' parameters).
- The trait must support heterogeneous collections (`Vec<Box<dyn Module>>`)
  for the rack to hold mixed module types.

### Design

```rust
/// The core trait that all oxurack modules implement.
///
/// A module is an independent unit that produces outputs on each tick
/// and accepts parameter changes between ticks.
pub trait Module: std::fmt::Debug {
    /// The concrete output type for this module.
    ///
    /// Standalone users work with this directly for full type safety.
    type Output: Clone + std::fmt::Debug;

    /// Advance the module by one step and return its outputs.
    fn tick(&mut self) -> Self::Output;

    /// Reset the module to its initial state.
    fn reset(&mut self);

    /// Return the module's name (used for routing and display).
    fn name(&self) -> &str;

    /// Export the current tick's outputs as dynamic port values.
    ///
    /// Used by the rack for cable routing. Standalone users can ignore this.
    fn output_ports(&self) -> Vec<Port>;

    /// Receive a value on a named input port.
    ///
    /// The module interprets the port name and applies the value
    /// (e.g., "write_probability" -> set_write(value.as_f32())).
    /// Returns `true` if the port name was recognized.
    fn receive(&mut self, port: &str, value: Value) -> bool;

    /// List the input ports this module accepts.
    fn input_port_names(&self) -> Vec<PortDescriptor>;

    /// List the output ports this module produces.
    fn output_port_names(&self) -> Vec<PortDescriptor>;
}
```

### Extracting a trait object for the rack

The associated `Output` type prevents direct use of `dyn Module` in
heterogeneous collections. The rack needs a type-erased wrapper:

```rust
/// Type-erased module for use in the rack.
///
/// Wraps any `Module` impl, forwarding tick/reset/routing calls
/// and converting the concrete Output to dynamic port values.
pub trait DynModule: std::fmt::Debug {
    fn tick(&mut self);
    fn reset(&mut self);
    fn name(&self) -> &str;
    fn output_ports(&self) -> Vec<Port>;
    fn receive(&mut self, port: &str, value: Value) -> bool;
    fn input_port_names(&self) -> Vec<PortDescriptor>;
    fn output_port_names(&self) -> Vec<PortDescriptor>;
}
```

A blanket impl or wrapper struct bridges `Module` to `DynModule`:

```rust
struct ModuleBox<M: Module> {
    module: M,
    last_output: Option<M::Output>,
}

impl<M: Module> DynModule for ModuleBox<M> {
    fn tick(&mut self) {
        self.last_output = Some(self.module.tick());
    }
    fn output_ports(&self) -> Vec<Port> {
        self.module.output_ports()
    }
    // ... delegate remaining methods
}
```

The rack holds `Vec<Box<dyn DynModule>>`. Users who want type safety use
the module directly; the rack uses the erased interface.

---

## 2. Value Types

### The Value enum

Values transmitted through cables. Covers all MIDI-domain data:

```rust
/// A value transmitted between modules through the routing system.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    /// Integer in 0..=127 (MIDI data byte range).
    Midi(u8),
    /// Boolean gate or trigger.
    Gate(bool),
    /// Floating-point parameter (0.0..=1.0 for normalized, or arbitrary).
    Float(f32),
    /// Signed integer for bipolar values (-64..=63).
    Bipolar(i8),
    /// Raw unsigned integer (for register bits, step counts, etc.).
    Raw(u16),
}

impl Value {
    /// Interpret as a MIDI byte (0--127), clamping if necessary.
    pub fn as_midi(&self) -> u8 { ... }

    /// Interpret as a float.
    pub fn as_f32(&self) -> f32 { ... }

    /// Interpret as a gate/bool.
    pub fn as_gate(&self) -> bool { ... }
}
```

### Port and PortDescriptor

```rust
/// A named output value from a module.
#[derive(Debug, Clone)]
pub struct Port {
    pub name: &'static str,
    pub value: Value,
}

/// Metadata describing a port (for validation and documentation).
#[derive(Debug, Clone)]
pub struct PortDescriptor {
    pub name: &'static str,
    pub kind: ValueKind,
    pub description: &'static str,
}

/// The expected type of a port's value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    Midi,
    Gate,
    Float,
    Bipolar,
    Raw,
}
```

---

## 3. Scale and Quantization (extracted from turingmachine)

The `Scale` struct and its `quantize()` method are needed by at least the
turingmachine, standalone quantizer module, and potentially the sequencer.
They belong in oxurack-core.

### What moves

From `crates/turingmachine/src/quantizer.rs`:

- `Scale` struct (intervals, name, all 14 built-in constructors, `quantize()`)

### What stays

- `Quantizer` struct (the DAC-byte-to-note mapper with range) stays in the
  turingmachine crate. It is specific to the Turing Machine's DAC model.
  The standalone quantizer module will have its own wrapper.

### Migration path

1. Create `crates/oxurack-core/` with `Scale` and quantization logic.
2. Update turingmachine's `Cargo.toml` to depend on `oxurack-core`.
3. Change `use crate::quantizer::Scale` to `use oxurack_core::Scale` in
   turingmachine code.
4. Re-export `oxurack_core::Scale` from turingmachine's public API so
   downstream users are not broken.

---

## 4. The BatchGenerator (Executor Pattern)

Ported from underack's executor concept (issues #9, #20). Some modules
(Perlin noise, Simplex noise, complex LFOs) have expensive per-sample
computation. The batch generator pre-computes values into a buffer and
serves them cheaply.

### Design

```rust
use std::collections::VecDeque;

/// Pre-computes values in batches for efficient per-tick consumption.
///
/// The generator function is called to fill a batch whenever the
/// internal buffer runs low. Consumers call `next()` on each tick
/// to get the next value.
pub struct BatchGenerator<T> {
    buffer: VecDeque<T>,
    batch_size: usize,
    low_water: usize,
    generate: Box<dyn FnMut(usize) -> Vec<T>>,
}

impl<T> BatchGenerator<T> {
    /// Create a new generator.
    ///
    /// - `batch_size`: how many values to generate per batch.
    /// - `low_water`: refill when the buffer drops below this count.
    /// - `generate`: a closure that produces `n` values.
    pub fn new(
        batch_size: usize,
        low_water: usize,
        generate: impl FnMut(usize) -> Vec<T> + 'static,
    ) -> Self {
        let mut gen = Self {
            buffer: VecDeque::with_capacity(batch_size),
            batch_size,
            low_water,
            generate: Box::new(generate),
        };
        gen.refill();
        gen
    }

    /// Get the next pre-computed value.
    ///
    /// Automatically triggers a refill if the buffer is running low.
    pub fn next(&mut self) -> T {
        if self.buffer.len() <= self.low_water {
            self.refill();
        }
        self.buffer
            .pop_front()
            .expect("buffer should never be empty after refill")
    }

    /// Force a refill of the buffer.
    pub fn refill(&mut self) {
        let batch = (self.generate)(self.batch_size);
        self.buffer.extend(batch);
    }

    /// Discard all buffered values and refill.
    pub fn flush(&mut self) {
        self.buffer.clear();
        self.refill();
    }
}
```

### Usage in modules

```rust
// In the noise module:
let perlin = BatchGenerator::new(
    256,    // generate 256 samples per batch
    64,     // refill when 64 or fewer remain
    move |n| {
        (0..n)
            .map(|i| {
                let t = (offset + i as f64) * frequency;
                // perlin_noise_1d(t) returns f64 in -1.0..1.0
                let raw = perlin_noise_1d(t);
                ((raw + 1.0) * 63.5) as u8  // map to 0--127
            })
            .collect()
    },
);

// On each tick:
let value = perlin.next();  // O(1) amortized
```

### Determinism

The generate closure captures its own RNG or noise state. For deterministic
output, seed the captured state. The BatchGenerator itself is agnostic to
the source of randomness.

---

## 5. Routing Primitives

These types support the rack's cable system. They live in oxurack-core so
the rack crate and any future routing utilities can use them, but module
crates never need to import them directly.

### Cable

```rust
/// A connection from one module's output port to another module's input port.
#[derive(Debug, Clone)]
pub struct Cable {
    /// Source module name.
    pub from_module: String,
    /// Source output port name.
    pub from_port: String,
    /// Destination module name.
    pub to_module: String,
    /// Destination input port name.
    pub to_port: String,
    /// Optional value transform applied in transit.
    pub transform: Option<CableTransform>,
}

/// A transformation applied to a value as it passes through a cable.
#[derive(Debug, Clone)]
pub enum CableTransform {
    /// Linear range mapping.
    Scale {
        in_min: f32,
        in_max: f32,
        out_min: f32,
        out_max: f32,
    },
    /// Invert the value (127 - x for MIDI, 1.0 - x for float).
    Invert,
    /// Clamp to a range.
    Clamp { min: f32, max: f32 },
}

impl CableTransform {
    /// Apply the transform to a value.
    pub fn apply(&self, value: Value) -> Value { ... }
}
```

### Patch

A complete routing configuration (for future serialization):

```rust
/// A complete patch configuration: modules and their wiring.
#[derive(Debug, Clone)]
pub struct Patch {
    /// Module configurations (name -> type + initial parameters).
    pub modules: Vec<ModuleConfig>,
    /// Cable connections.
    pub cables: Vec<Cable>,
    /// Master seed for deterministic playback (optional).
    pub seed: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ModuleConfig {
    pub name: String,
    pub module_type: String,
    pub params: Vec<(String, Value)>,
}
```

---

## 6. Crate Layout

```
crates/oxurack-core/
├── Cargo.toml
└── src/
    ├── lib.rs              # Re-exports
    ├── module.rs            # Module trait, DynModule, ModuleBox
    ├── value.rs             # Value, Port, PortDescriptor, ValueKind
    ├── scale.rs             # Scale (moved from turingmachine)
    ├── batch_generator.rs   # BatchGenerator
    └── routing.rs           # Cable, CableTransform, Patch, ModuleConfig
```

### Cargo.toml

```toml
[package]
name = "oxurack-core"
version = "0.1.0"
edition = "2024"
description = "Core traits and types for the oxurack modular MIDI system."
license = "MIT"
repository = "https://github.com/oxur/oxurack"

[dependencies]
# Intentionally minimal. No rand, no midir, no heavy deps.
# Scale quantization uses only std.
```

The core crate should have **zero external dependencies** if possible. The
`Scale` quantization is pure arithmetic. The `BatchGenerator` uses only
`std::collections::VecDeque`. The `Module` trait uses only `std::fmt`.

Module crates depend on oxurack-core plus whatever they specifically need
(rand, noise crate, etc.). This keeps the dependency tree shallow.

---

## 7. Implementation Order

1. **Create `crates/oxurack-core/`** with the `Value`, `Port`, and
   `PortDescriptor` types. These are simple and uncontroversial.

2. **Move `Scale`** from turingmachine to oxurack-core. Update turingmachine
   to depend on and re-export it. Run tests to confirm nothing broke.

3. **Define the `Module` trait** with a minimal surface: `tick()`, `reset()`,
   `name()`. Add the port introspection methods (`output_ports()`,
   `receive()`, etc.) once the first two modules (turingmachine + clock)
   confirm the design works.

4. **Implement `BatchGenerator`**. Test with a simple counter closure to
   verify the refill logic. The noise module will be its first real consumer.

5. **Add routing types** (`Cable`, `CableTransform`, `Patch`). These are
   pure data structures with no behavior beyond `CableTransform::apply()`.
   They are needed by the rack but can be defined early.

6. **Implement `DynModule`** and `ModuleBox`. Needed when the rack crate
   begins, but can be deferred until then.

7. **Refactor turingmachine** to implement `Module`. This validates the trait
   design with a real, complex module.

---

## 8. Open Questions

### Should the Module trait require Send + Sync?

If we ever want to tick independent module subgraphs in parallel, modules
must be `Send`. For now, single-threaded is fine. **Recommendation**: add
`Send` to the trait bound from the start. It costs nothing for well-designed
modules (no `Rc`, no raw pointers) and avoids a breaking change later.

### Should the rack own modules or borrow them?

Own them (`Vec<Box<dyn DynModule>>`). Borrowing creates lifetime complexity
that provides no benefit -- the rack is the natural owner of its modules.

### How should the rack handle tick ordering for cycles?

Reject cycles at patch-load time. If a future version wants feedback loops,
introduce a `Delay` pseudo-module that breaks the cycle by holding values
for one tick. This is how hardware Eurorack works (propagation delay is
inherent), and it is a well-understood pattern.

### Should Value be Copy?

Yes. `Value` is 5 bytes (enum discriminant + largest variant). Making it
`Copy` avoids unnecessary cloning in the routing hot path. The `#[derive]`
in the design above already includes `Copy`.
