---
number: 2
title: "Oxurack System Architecture"
author: "patch cables"
component: All
tags: [change-me]
created: 2026-04-06
updated: 2026-04-06
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# Oxurack System Architecture

## Overview

Oxurack is a modular MIDI generation and transformation system implemented as a
Rust workspace. It applies the Eurorack mental model -- independent modules
connected by patch cables -- to the MIDI domain, producing note, velocity, gate,
and CC streams rather than audio or control voltages.

The system is designed for two primary use cases:

1. **Library use** -- individual module crates used directly in application code.
2. **Rack use** -- multiple modules instantiated, wired together, and driven by
   a shared clock through the orchestration layer.

Both use cases must be first-class. A musician who only needs a Turing Machine
should never have to depend on the rack infrastructure. A musician who wants a
generative patch with six modules wired together should not have to manually
tick each one.

## Design Principles

1. **Eurorack as metaphor, not constraint.** The modular model provides
   excellent ergonomics, but we are not limited by physical realities. Outputs
   can fan out freely. Modules can have dynamic I/O. Patches are data, not
   physical cables.

2. **Each crate stands alone.** Every module crate must be independently useful
   without the rack or routing infrastructure. The rack is opt-in orchestration,
   not a required runtime.

3. **Composition over configuration.** Complex behavior should emerge from
   combining simple modules, not from configuring monolithic ones.

4. **Determinism by default.** Any configuration that accepts a seed must
   produce bit-identical output across runs and platforms. Reproducibility is
   non-negotiable for generative music.

5. **No audio, no synthesis.** Oxurack generates MIDI. Sound comes from
   whatever receives that MIDI -- hardware synths, DAW plugins, sample
   libraries. This constraint keeps the system lightweight and focused.

6. **Programmer-friendly first.** The primary interface is Rust code. GUIs and
   visual patching may come later; the core must be fully usable from code and
   a REPL-like workflow.

## System Layers

From bottom to top:

```
+------------------------------------------------------------------+
|  Application / REPL / DAW Plugin                                 |
+------------------------------------------------------------------+
|  Rack Orchestration (optional)                                   |
|  - Rack struct: holds modules, routes cables, drives clock       |
|  - Patch configuration: declarative wiring                       |
+------------------------------------------------------------------+
|  Module Crates (standalone)                                      |
|  - turingmachine, clock, noise, sequencer, quantizer, ...        |
|  - Each implements the Module trait                              |
|  - Each is independently usable without the rack                 |
+------------------------------------------------------------------+
|  Core Crate (oxurack-core)                                       |
|  - Module trait definition                                       |
|  - Value types, port types, routing primitives                   |
|  - BatchGenerator / executor pattern                             |
+------------------------------------------------------------------+
|  MIDI I/O (midir, feature-gated)                                 |
|  - Device enumeration                                            |
|  - Message transmission                                          |
+------------------------------------------------------------------+
```

## Core Abstractions

### Values

The "voltage" of oxurack. MIDI operates in well-defined integer ranges:

- **Note**: 0--127 (but we avoid 0 in practice)
- **Velocity**: 1--127
- **CC**: 0--127
- **Gate**: bool (on/off)
- **Trigger**: bool (momentary pulse)

Modules produce and consume these values. Unlike analog voltages, MIDI values
are discrete integers with hard bounds. This simplifies range checking but means
modules must handle quantization and clamping explicitly.

A module's outputs are not a single value but a structured snapshot of
everything the module produces on a given step. This is already the pattern
established by `StepOutputs` in the Turing Machine crate.

### Modules

Independent units that process inputs and produce outputs. Each module:

- Has a name and type identifier.
- Declares what parameters it accepts (write probability, BPM, scale, etc.).
- Produces a structured output on each tick.
- Maintains its own internal state.
- Can be used standalone (just construct and call `tick()`).
- Can optionally participate in a rack (receiving routed inputs, contributing
  outputs to the routing table).

Modules are **not** concurrent by default. The rack ticks them sequentially in
dependency order. Concurrency is an optimization that can be added later for
independent subgraphs, but the default is single-threaded determinism.

### The Rack

The top-level orchestration container. A rack:

- Holds a collection of named module instances.
- Owns a master clock that drives the tick cycle.
- Maintains a routing table mapping output ports to input ports.
- On each tick: advances the clock, ticks modules in dependency order, applies
  routing between steps.

The rack is optional. It lives in the umbrella crate (`oxurack`)
and depends on `oxurack-core` plus whatever module crates the user wants.

### Cables (Routing)

Connections between module outputs and module parameters. A cable specifies:

- **Source**: module name + output field (e.g., `"noise"` + `noise_cc`).
- **Destination**: module name + parameter setter (e.g., `"tm"` + `set_write`).
- **Transform** (optional): a mapping function applied to the value in transit
  (e.g., scale 0--127 to 0.0--1.0 for a probability input).

Cables are data, not function pointers. The rack evaluates them between module
ticks. Fan-out is free (one output to many inputs). Fan-in requires a merge
strategy (last-write-wins, sum, average) -- start with last-write-wins.

Unlike underack's ETS-based cable registry with caching and sync concerns,
oxurack cables are owned by the rack struct and evaluated synchronously. No
shared mutable state, no cache invalidation, no concurrency hazards.

### Ports

Modules declare their inputs and outputs through port metadata:

- **Output ports**: named fields on the module's step output struct. Read-only
  from the routing layer's perspective.
- **Input ports**: named parameters that can be set on the module. These
  correspond to the module's setter methods.

Port metadata enables the rack to validate cable connections at patch-load time
rather than at runtime.

## Tick Cycle

The rack's main loop for a single step:

```
1. Master clock advances.
2. For each module in dependency order:
   a. Apply any incoming cable values (call setter methods).
   b. Call module.tick().
   c. Record the module's outputs in the routing table.
3. If MIDI output modules exist, they transmit during their tick.
4. Return the combined outputs (or fire callbacks).
```

Dependency order is determined by the cable graph. Modules with no incoming
cables tick first. Cycles are rejected at patch-load time (they represent
feedback loops that require a one-step delay -- a future feature).

## Crate Organization

```
oxurack/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── oxurack/                  # Umbrella + rack orchestration (optional)
│   ├── oxurack-core/             # Module trait, value types, routing primitives
│   ├── turingmachine/            # Turing Machine Mk2 (existing)
│   ├── clock/                    # Master clock + dividers/multipliers
│   ├── noise/                    # Random value generation
│   ├── lfo/                      # Low-frequency oscillator
│   ├── sequencer/                # Step sequencer
│   ├── quantizer/                # Standalone scale quantizer
│   ├── range/                    # Value range scaler
│   ├── sample-hold/              # Sample & Hold
│   ├── euclidean/                # Euclidean rhythm generator
│   ├── arpeggiator/              # MIDI arpeggiator
│   └── midi-output/              # Generic MIDI output module
```

Every module crate depends on `oxurack-core` for the `Module` trait but has no
dependency on `oxurack` or on other module crates. The rack crate depends
on `oxurack-core` and re-exports it.

## State and Determinism

Each module owns its state entirely. There is no global mutable state. The rack
holds modules by value (or boxed trait objects) and is the single owner.

For determinism:

- Every module that uses randomness must accept a seed.
- The rack can be constructed with a master seed that derives per-module seeds
  deterministically (e.g., hash of master seed + module name).
- Ticking order is deterministic (topological sort of the cable graph, with
  stable tie-breaking by module name).

## MIDI I/O

MIDI transmission is handled by dedicated output modules, not by the rack
itself. An output module:

- Receives note, velocity, gate, and CC values via cables from other modules.
- Converts them to MIDI messages.
- Sends via `midir` (feature-gated).

This keeps MIDI I/O at the edges. The core system is pure computation with no
I/O side effects, making it easy to test and reason about.

## Future Considerations

These are explicitly out of scope for the initial architecture but should not
be precluded by design decisions:

- **MIDI input modules**: accept external MIDI as a modulation source.
- **MIDI clock sync**: slave to external MIDI clock from a DAW.
- **Patch serialization**: save/load patches as TOML or JSON files.
- **Visual patch editor**: web-based or TUI for creating and monitoring patches.
- **Feedback loops**: cables that introduce a one-step delay, enabling
  self-modulating patches.
- **Parallel tick execution**: tick independent subgraphs concurrently.
- **Plugin hosting**: wrap the rack as a VST/AU/CLAP plugin.
