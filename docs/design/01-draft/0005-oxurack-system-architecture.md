---
number: 5
title: "Oxurack System Architecture"
author: "patch cables"
component: All
tags: [change-me]
created: 2026-04-18
updated: 2026-04-18
state: Draft
supersedes: 2
superseded-by: null
version: 1.0
---

# Oxurack System Architecture

## Overview

Oxurack is a modular MIDI generation and transformation system implemented as
a Rust workspace. It applies the Eurorack mental model — independent modules
connected by patch cables — to the MIDI domain, producing note, velocity, gate,
and CC streams rather than audio or control voltages.

This document defines the v1 architecture. It replaces the initial architecture
doc (superseded as of this writing) with a substantially different substrate
commitment: oxurack is built on Bevy ECS for its non-real-time core, on a
dedicated real-time thread for MIDI clock and I/O, and on a uniform interior
value currency modelled directly on Eurorack voltages.

The system is designed for two primary use cases that must both be first-class:

1. **Library use** — individual module crates used directly from application
   code with minimal ceremony.
2. **Rack use** — multiple modules instantiated, wired together, and driven by
   a shared clock through the ECS runtime.

A musician who only needs a Turing Machine should never have to depend on the
rack infrastructure. A musician who wants a generative patch with ten modules
wired together should be able to express that declaratively and save the result
as a patch file.

---

## Design Principles

These carry forward from the superseded architecture doc, with two additions
and one reframing.

1. **Eurorack as metaphor, not constraint.** The modular model provides
   excellent ergonomics, but we are not limited by physical realities. Outputs
   can fan out freely. Input ports can accept many cables. Cables can carry
   metadata. Patches are data, not physical objects.

2. **Each crate stands alone.** Every module crate must be independently
   useful without the rack runtime. The rack is the coordinating layer, not a
   required dependency of any individual module's standalone API.

3. **Composition over configuration.** Complex behavior emerges from combining
   simple modules, not from configuring monolithic ones. When in doubt, ship
   more small modules rather than fewer big ones.

4. **Determinism by default.** Any configuration that accepts a seed must
   produce bit-identical output across runs and platforms. Reproducibility is
   non-negotiable for generative music intended to be recorded.

5. **No audio, no synthesis.** Oxurack generates MIDI. Sound comes from
   whatever receives that MIDI — hardware synths, DAW plugins, sample
   libraries. This constraint keeps the system lightweight and focused.
   Critically, it means oxurack's CPU footprint can remain small while a DAW
   handles the heavy audio work — the recording workflow depends on this
   separation.

6. **Programmer-friendly first.** The primary interface is Rust code. A CLI /
   REPL for interactive patch manipulation is a core deliverable. GUIs and
   visual patching may come later.

7. **Patches are first-class artifacts.** Patches can be saved, loaded,
   versioned, diffed, shared, and embedded in git repositories. Save/load is
   not an afterthought. The patch file format is serde-based RON or JSON,
   generated via `bevy_reflect` from the live ECS world.

8. **Both clock master and clock slave.** Oxurack must be usable as a
   standalone MIDI clock source *and* as a slave locked to an external master
   (typically a DAW). Slave mode is as important as master mode; the recording
   workflow depends on oxurack syncing to the DAW's transport.

---

## The Four Tiers

Oxurack is organised as four tiers of crates, each with a well-defined role
and a minimal set of downstream dependencies. Tiers below depend only on
lower tiers.

```
+-------------------------------------------------------------------+
| Tier 4: Applications                                              |
|   oxurack              (binary: REPL + TUI + main event loop)     |
|   oxurack-tui          (ratatui oscope, patch browser)            |
|   future: plugin hosts, VST/AU/CLAP wrappers                      |
+-------------------------------------------------------------------+
| Tier 3: Module Crates                                             |
|   oxurack-mod-clock, oxurack-mod-euclidean, oxurack-mod-noise,    |
|   oxurack-mod-lfo, oxurack-mod-quantizer, oxurack-mod-range,      |
|   oxurack-mod-sample-hold, oxurack-mod-sequencer,                 |
|   oxurack-mod-arpeggiator, oxurack-mod-midi-output,               |
|   oxurack-mod-turingmachine (refactor of existing crate)          |
+-------------------------------------------------------------------+
| Tier 2: The ECS World                                             |
|   oxurack-core         (Value enum, Cable/Port entities,          |
|                         hyperport model, three-phase tick,        |
|                         parameter registry, patch persistence)    |
|                        depends on Bevy (headless, minimal deps)   |
+-------------------------------------------------------------------+
| Tier 1: The Real-Time Thread                                      |
|   oxurack-rt           (MIDI clock master + slave, PLL,           |
|                         transport messages, MIDI I/O via midir,   |
|                         SPSC lock-free queues, RT priority)       |
|                        no dependency on Bevy                      |
+-------------------------------------------------------------------+
```

Dependency rules:

- `oxurack-rt` depends on no other oxurack crate. It is self-contained and
  reusable outside this project if anyone wants just the RT MIDI clock.
- `oxurack-core` depends on Bevy and on `oxurack-rt`'s public API types
  (the shared SPSC queue message types live in a tiny `oxurack-rt-abi` crate
  that both import, so neither owns the other).
- Each module crate depends on `oxurack-core` only. No module crate depends
  on another module crate. No module crate depends on `oxurack-rt` directly;
  everything is mediated through core.
- Application crates depend on whichever tiers they need. The main `oxurack`
  binary depends on all four.

---

## Tier 1: The Real-Time Thread (oxurack-rt)

The single concern of this crate is tight-timing MIDI. It owns one or more
dedicated OS threads elevated to real-time priority (via Mozilla's
`audio_thread_priority` crate, which handles the platform-specific policy
calls: SCHED_FIFO on Linux, THREAD_TIME_CONSTRAINT_POLICY on macOS,
SetThreadPriority on Windows).

These threads do three things:

**Clock management.** Either generate a 24-PPQN MIDI clock locked to an
internal tempo (master mode) or track an external master's clock via a
phase-locked loop and tempo estimator (slave mode). Transport messages
(Start 0xFA, Stop 0xFC, Continue 0xFB) and Song Position Pointer (0xF2) are
handled here.

**MIDI I/O.** Input ports (for clock slave and for MIDI input modules) and
output ports (for MIDI output modules) are opened via `midir`. Messages are
scheduled and emitted with microsecond-level precision.

**Queue bridging.** Two lock-free SPSC queues (via `rtrb`) move data across
the RT / non-RT boundary without locks:

- **`rt → ecs`**: clock tick events, incoming MIDI messages, transport
  state changes. Read by the Bevy world on its own tick.
- **`ecs → rt`**: scheduled MIDI output messages (with microsecond timestamps
  relative to the next clock tick). Emitted by the RT thread at the
  appropriate moment.

The RT thread is deliberately simple. It does no pattern generation, no
routing, no cable propagation. Those concerns live above in core and modules.
This discipline is what gives the RT thread a chance of meeting its timing
budget on a machine that is simultaneously running a DAW and whatever else.

See design doc 0006 for the full RT thread specification.

---

## Tier 2: The ECS World (oxurack-core)

The core crate is a thin layer on top of Bevy ECS, configured headless (no
windowing, no rendering, no audio plugins — just the scheduler, entity/
component storage, reflection, and change detection).

The core commitments:

**Modules as Bevy plugins with domain naming.** Each module crate defines a
struct implementing Bevy's `Plugin` trait. The struct is named after the
domain concept (e.g., `TuringMachineModule`, `ClockModule`, `EuclideanModule`),
not after the Bevy convention suffix. `App::add_plugins(TuringMachineModule)`
works exactly as written. Bevy does not enforce any naming convention on
`Plugin` implementers; this is idiomatic.

**Ports as first-class entities.** Each module instance owns a set of port
entities as children. A port is a real thing with type, direction, merge
policy, and a current value. Ports are queryable, observable via change
detection, and the locus of merge semantics when multiple cables converge.

**Cables as entities.** A cable is an entity with a `Cable` component that
references a source port entity and a target port entity, plus optional
transform metadata. Cables are not Bevy relationships (those are tree-shaped
and cannot carry edge metadata); they are plain entities. Cables-as-entities
support arbitrary graph topology, including cycles (with per-cable policy
for feedback), and serialize naturally via `bevy_reflect`.

**Three-phase tick cycle.** Each oxurack tick has three Bevy system phases:

1. **Produce.** Every module's tick system runs. Systems read the module's
   input ports and write fresh values to the module's output ports.
2. **Propagate.** Every active cable reads its source port's current value,
   applies its transform, and writes into the target port's pending-inputs
   accumulator.
3. **Consume.** Every input port that received values applies its merge
   policy (sum, OR, interleave, priority, reject) and finalises a single
   current value for the next tick's producer phase to read.

This cleanly separates concerns: modules don't need to know about cables;
cables don't need to know about modules; ports don't need to know about
either.

**Decomposed CV as interior currency.** See the Value Currency Commitment
section below. Internally, cables carry scalar values of type
Float / Gate / Bipolar / Midi / Raw — not fused MIDI events.

**Patch persistence via reflection.** A patch is a serde-serializable
snapshot of the relevant entities (modules, ports, cables) and their
components. `bevy_reflect`'s `TypeRegistry` + `ReflectDeserializer` handles
save/load to RON or JSON.

See design doc 0007 for the full core specification.

---

## Tier 3: Module Crates (oxurack-mod-*)

Each module crate defines one oxurack module. The crate's public surface is:

- A plugin struct implementing Bevy's `Plugin` trait (domain-named, no
  suffix convention).
- The component types the module uses (state, parameters, port markers).
- The tick system(s) that advance the module on each oxurack tick.
- Port declarations (names, value types, directions, merge policies).
- A standalone API for use outside the rack (construct, configure, tick,
  read outputs) that does not require the ECS world to be running.

Crates never depend on each other. Shared types (`Value`, `Scale`,
`CableTransform`, etc.) live in `oxurack-core`. A module that needs another
module's output should receive it as a cable value, not as a direct
dependency.

The standalone API for each module is preserved because Principle 2 (each
crate stands alone) remains load-bearing. In practice, the ECS-facing plugin
layer wraps a pure-Rust core that does the actual computation — the same
way `MidiTuringMachine` in the existing `turingmachine` crate wraps the
pure `TuringMachine`.

See design doc 0008 for the module infrastructure specification and design
doc 0009 for the module catalog.

---

## Tier 4: Applications

The top tier composes the lower tiers into runnable programs.

**`oxurack` (main binary).** The default entry point. Spins up the RT thread
(`oxurack-rt`), constructs the Bevy app (`oxurack-core` plus whichever
module crates are linked in), loads any requested patch, and enters the main
loop. The main loop calls `app.update()` on the Bevy app each iteration,
drains the RT-to-ECS queue into Bevy events, and serves a CLI / REPL for
interactive patch manipulation. When built with the `tui` feature, the TUI
oscope becomes available as an alternative front-end.

**`oxurack-tui` (optional crate).** Ratatui-based terminal UI providing:
a live oscope showing values on selected ports with change-detection-driven
repaint; a patch browser for loading saved patches; a command-line surface
integrated with the REPL. Depends on `oxurack-core` and on ratatui, no
direct dep on Bevy beyond what core re-exports.

**Future application targets.** VST/AU/CLAP plugin hosts wrapping the
oxurack runtime for in-DAW use are deferred to post-v1. The architecture
does not preclude them: the ECS world is already headless and update-driven.

---

## Threading Model

Oxurack uses a two-world threading model, strictly separated:

**The RT world** (one or more dedicated OS-priority threads, owned by
`oxurack-rt`). Zero heap allocation in steady state. No locks. No syscalls
that may block unpredictably. All data exchange with the rest of the program
via SPSC lock-free queues. The RT world's only job is to be on time.

**The ECS world** (one thread running the Bevy scheduler, plus whatever
parallel worker threads Bevy's scheduler spawns for system parallelism).
Normal Rust — allocation is fine, locks are fine, the scheduler runs many
systems in parallel automatically based on component read/write disjointness.
The ECS world's job is to maintain the rack state and respond to clock ticks.

**The CLI / REPL world** (main thread, typically). Handles user input, sends
commands to the ECS world via a crossbeam channel that a Bevy system drains
each tick, renders output. Does not touch the world directly.

Bevy's `Commands` are `Send + Sync`, which means the REPL thread can queue
ECS mutations safely. But timing-sensitive paths (RT thread's emission of
MIDI messages) do not go through Bevy — they go through the SPSC queue.

---

## Value Currency Commitment

This is the most consequential semantic commitment in the architecture.

**Inside oxurack, cables carry decomposed, scalar values — not fused MIDI
events.** A note is represented as (at minimum) a `Float` pitch cable, a
`Float` velocity cable, and a `Gate` cable. This matches the Eurorack
mental model exactly: in hardware, a melody is carried by a pitch CV cable
and a gate cable, and velocity (if present) is a third cable.

The `Value` enum stays as drafted in the superseded infrastructure doc:

```rust
pub enum Value {
    Midi(u8),      // single MIDI data byte 0..=127
    Gate(bool),    // boolean gate/trigger
    Float(f32),    // continuous value (typically 0.0..=1.0 or bipolar)
    Bipolar(i8),   // signed value -64..=63 (modulation sources)
    Raw(u16),      // shift register bits, step counts, escape hatch
}
```

`Midi(u8)` is only used for single-byte MIDI data (e.g., a note number that
is already constrained to MIDI range). A *MIDI event stream* is a boundary
format — it lives only at the edges of the rack, inside MIDI input and
MIDI output modules. The vast majority of cables in any patch will be
`Float` or `Gate`.

This commitment has several consequences:

- **Polyphony is parallel-cables.** N-voice polyphony means N pitch cables,
  N velocity cables, N gate cables. No "poly cable carrying 16 channels" —
  the Value type stays scalar, the polyphony lives in port multiplicity.
- **MIDI complexity is quarantined.** Channels, message types, running
  status, SysEx, MPE encoding — all of it lives inside MIDI I/O module
  implementations. The interior of the rack knows nothing of MIDI's protocol
  details.
- **Module authoring stays simple.** A module author writes a tick function
  that reads scalar values from input ports and writes scalar values to
  output ports. No MIDI parsing, no channel routing.
- **Boundary modules carry the translation burden.** A `MidiOutputMono`,
  `MidiOutputPoly`, `MidiOutputMPE`, or `MidiOutputDrumMap` module takes the
  relevant decomposed cables and assembles them into well-formed MIDI.
  The choice of output module *is* the choice of output encoding strategy.

---

## Patches as First-Class Artifacts

A patch is a serializable snapshot of a rack configuration: which modules
are present (with their initial parameter values), which cables exist
between which ports, and metadata such as the master seed for deterministic
playback.

Patches use Bevy's reflection system. Every module component, port
component, cable component, and parameter bundle derives `Reflect` and is
registered in the `TypeRegistry` at app startup. The save operation walks
the relevant entities and serialises to RON (default) or JSON. The load
operation reverses the process, spawning entities with the recorded
components.

Patch file format: RON by default (human-readable, comment-friendly,
git-diffable). Suggested extension: `.oxpatch`. Size on disk for a patch of
~20 modules with ~50 cables is expected to be a few kilobytes.

Patch versioning: `bevy_reflect` handles missing or extra fields gracefully.
A patch from an older oxurack version loads in a newer one as long as the
module crates remain compatible. Breaking changes in a module crate require
a migration pass — design doc 0008 covers this.

---

## Clock Modes

Three clock modes are supported:

**Master.** The RT thread generates a 24-PPQN clock at the configured
tempo and emits MIDI Clock (0xF8), Start, Stop, and Continue messages on
the configured output port(s). Internal clock events are pushed into the
rt-to-ecs queue at each tick. This is the classic standalone mode — useful
for driving external synths without a DAW.

**Slave.** The RT thread opens a MIDI input port, parses incoming 0xF8 /
transport / SPP messages, runs a phase-locked loop on the incoming tick
stream to estimate tempo and smooth jitter, and emits clock events into
the rt-to-ecs queue based on the smoothed estimate. This is the DAW-recording
mode — the DAW is the master, oxurack follows, the DAW records the MIDI
that oxurack generates.

**Passthrough / chained.** The RT thread is simultaneously a slave (to an
upstream master) and a master (emitting to downstream devices), possibly
with division or multiplication. Useful when oxurack sits between a DAW
and hardware synths in a chain. Not required for v1 but the architecture
permits it.

From the ECS world's point of view, all three modes are identical: the
world receives clock tick events on its queue and updates accordingly.
The master / slave distinction is entirely confined to the RT thread.

See design doc 0006 for the full clock specification, including PLL design
choices for slave mode.

---

## What This Doc Does Not Cover

The following are addressed in companion docs and should be read together
with this one:

- **0006** — the RT thread in detail: MIDI I/O, clock modes, PLL design,
  SPSC queue protocols, timing budget.
- **0007** — the ECS core in detail: entity layouts, component schemas,
  three-phase tick, port/cable lifecycle, patch persistence.
- **0008** — module infrastructure: plugin trait pattern, parameter
  registry, port declaration API, merge policy vocabulary, authoring
  conventions, standalone-vs-ECS API split.
- **0009** — the v2 module catalog and build order: what gets built when,
  which modules depend on which infrastructure, and the end-to-end
  milestones along the way.

The following are explicitly deferred past v1:

- Visual patch editor (web or native GUI).
- VST / AU / CLAP plugin hosting.
- Live module *type* definition (as opposed to live module instantiation
  — which is v1). No embedded scripting language in v1.
- MIDI 2.0 support.
- Distributed / networked racks.

---

## Lineage

Oxurack is the Rust successor to Duncan's LFE project `underack`
(<https://github.com/ut-proj/underack>), which established the cable-based
modular-MIDI ontology this architecture inherits. Key underack concepts
carried forward:

- Cables as first-class named connections between modules.
- Patches exportable as data (underack's `underack-cables:export`).
- The module vocabulary (clock, noise, sample-and-hold, etc.).
- The "generation and transformation of MIDI" framing, as opposed to
  audio synthesis.

Key departures from underack:

- Rust with Bevy ECS as the runtime substrate, rather than LFE on BEAM.
- Dedicated RT thread for MIDI clock and I/O, rather than relying on the
  BEAM scheduler's soft-real-time guarantees.
- Ports as first-class entities with declared merge semantics, rather than
  the ETS-based cable registry.
- Master *and* slave clock modes from v1, rather than master-only.

The existing `turingmachine` crate (design doc 0001, state Final) is
preserved and will be refactored to integrate with `oxurack-core` as part
of the v1 work, with its public standalone API unchanged.

---

## Open Questions

**Q1. Feedback cycles.** Cables-as-entities permit arbitrary graph topology,
including cycles. Do we allow feedback cables with an implicit one-tick
delay (like real hardware Eurorack's propagation delay), or reject cycles
at patch-load time as the superseded doc 0002 did? Leaning toward: allow,
with the cycle-breaking one-tick delay declared explicitly per cable.
Design doc 0007 will commit.

**Q2. BatchGenerator.** The superseded doc 0004 introduced a
`BatchGenerator<T>` pattern for amortising expensive sample generation
(Perlin noise, etc.). In the Bevy ECS model, this can be either a
per-module implementation detail or a shared utility. Design doc 0008 will
decide whether it belongs in core or in the individual modules that need
it.

**Q3. Live module definition.** V1 supports live module *instantiation*
(spawn a `TuringMachineModule` entity at runtime) but not live module
*type* definition (write a new module in a scripting language from the
REPL). The latter is out of scope for v1, but leaving room for it (via
dylib hot-reload of native Rust modules, or an embedded scripting layer
alongside native modules) costs little at the architecture level. Design
docs 0007 and 0008 will note what commitments would need revisiting if
this becomes a v2 goal.

**Q4. VST/AU/CLAP plugin hosting.** Out of scope for v1 but the
architecture does not preclude it. The RT thread can plausibly be
replaced by (or delegate to) the plugin host's audio-thread scheduling
when running in that context. Deferred.
