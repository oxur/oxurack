---
number: 9
title: "Module Catalog and Build Order (v2)"
author: "other modules"
component: All
tags: [change-me]
created: 2026-04-18
updated: 2026-04-18
state: Draft
supersedes: 3
superseded-by: null
version: 1.0
---

# Module Catalog and Build Order (v2)

## 0. Purpose

This document enumerates the modules oxurack will ship for its 1.0 milestone,
groups them into build phases, and specifies the order in which to implement
them. It supersedes `0003-initial-module-catalog-and-build-order.md`, which
was written against the older trait-based architecture.

The order prioritizes:

1. **Usability at every phase.** Every phase produces a rack that can make
   actual MIDI sound when connected to a DAW or hardware synth.
2. **Underack lineage.** Modules that exist in underack are preserved with
   matching names and semantics, unless there's a specific reason to diverge.
3. **Infrastructure before modules.** No module gets built before the
   infrastructure it depends on is in place.

## 1. The Catalog

oxurack 1.0 will ship 12 module crates. Each falls into one of five categories:

**Infrastructure (not modules, but called out here for ordering):**

- `oxurack-rt` -- real-time thread
- `oxurack-core` -- ECS, values, ports, cables
- `oxurack` -- umbrella crate + REPL + patch CLI

**Generators** (produce values, no inputs required):

| Crate | Kind | Purpose |
|---|---|---|
| `oxurack-mod-turingmachine` | `turingmachine` | Generative shift-register sequencer |
| `oxurack-mod-noise` | `noise` | Pseudo-random value generator (multiple distributions) |
| `oxurack-mod-lfo` | `lfo` | Low-frequency oscillator (sine, tri, saw, square) |
| `oxurack-mod-sequencer` | `sequencer` | Fixed-pattern step sequencer |
| `oxurack-mod-euclidean` | `euclidean` | Euclidean rhythm generator |

**Transformers** (take values in, produce modified values out):

| Crate | Kind | Purpose |
|---|---|---|
| `oxurack-mod-quantizer` | `quantizer` | Snap Float values to a musical Scale |
| `oxurack-mod-range` | `range` | Scale/offset/clamp Float and Bipolar values |
| `oxurack-mod-sample-hold` | `sample-hold` | Latch a value on trigger |
| `oxurack-mod-arpeggiator` | `arpeggiator` | Generate arpeggio from held notes |

**Clock** (drives the rack):

| Crate | Kind | Purpose |
|---|---|---|
| `oxurack-mod-clock` | `clock` | Clock divider/multiplier on top of master clock |

**I/O** (boundary with the MIDI world):

| Crate | Kind | Purpose |
|---|---|---|
| `oxurack-mod-midi-out` | `midi-out` | Send MIDI to external devices via RT thread |
| `oxurack-mod-midi-in` | `midi-in` | Receive external MIDI, emit as port values (v1.1) |

### 1.1 Deferred to v1.1 or Later

These were on the original list but deferred:

- **BatchGenerator-backed modules** -- decided against batching in v1.
  Covered in `0007-oxurack-core-ecs-world.md` §4.2.
- **MIDI input module** (`midi-in`) -- depends on RT thread's MIDI input
  path being exercised. Shipping as v1.1.
- **Patch serialization UI / GUI editor** -- out of scope for 1.0.
- **VST/AU/CLAP plugin wrapper** -- out of scope.

## 2. Build Phases

### Phase 0 -- Skeleton (Week 0-1)

Before any modules: the workspace itself.

```
Phase 0 deliverables:
  [P0-1] oxurack workspace created with Cargo.toml at root.
  [P0-2] oxurack-core skeleton with Value, ValueKind, MidiMessage.
  [P0-3] oxurack-core Port, PortDirection, MergePolicy, CurrentValue.
  [P0-4] oxurack-core Cable, CableTransform.
  [P0-5] oxurack-core Module, ModuleId, ModuleKind.
  [P0-6] CorePlugin with TickPhase sets (empty systems).
  [P0-7] oxurack-rt skeleton with Runtime, ClockMode, RtHandles.
  [P0-8] Master clock implementation (slave deferred to Phase 4).
  [P0-9] RtEvent queue plumbing (lock-free rtrb queues).
  [P0-10] oxurack-core drain_rt_events_system -> TickNow events.
  [P0-11] oxurack umbrella crate with empty main.
```

**Exit criterion:** `cargo run -p oxurack` starts a Runtime, master clock
fires `TickNow` events at 120 BPM, ECS world processes them (no modules
yet -- just verifying the clock tick machinery works end-to-end).

### Phase 1 -- First Real Module (Week 2)

Port the existing Turing Machine Mk2 crate to the Bevy plugin architecture.
This is our highest-confidence module and our end-to-end test of the
module authoring story.

```
Phase 1 deliverables:
  [P1-1] oxurack-mod-turingmachine crate extracted from existing
         turingmachine crate.
  [P1-2] Scale type moved from turingmachine to oxurack-core.
  [P1-3] TuringMachineState as a Component.
  [P1-4] tick_turingmachine system in TickPhase::Produce.
  [P1-5] TuringMachineModule impl OxurackModule + Plugin.
  [P1-6] Parameter setters for length, write_probability, scale, seed.
  [P1-7] Integration test: spawn TM, tick 100 times, assert
         output structure and determinism.
```

**Exit criterion:** A test rack with one Turing Machine ticks, produces
outputs, and is bit-identical across runs for the same seed.

### Phase 2 -- MIDI Output (Week 3)

Get actual MIDI bytes flowing to an external device.

```
Phase 2 deliverables:
  [P2-1] midir integration in oxurack-rt.
  [P2-2] MIDI device enumeration API.
  [P2-3] RT-thread MIDI output path: EcsCommand::MidiOut -> midir send.
  [P2-4] oxurack-mod-midi-out crate.
  [P2-5] MidiOut module: accepts note_in (Float), gate_in (Gate),
         velocity_in (Float), channel parameter.
  [P2-6] Integration: TuringMachine -> MidiOut -> external synth.
  [P2-7] Jitter measurement harness.
```

**Exit criterion:** Duncan runs a 2-module rack (TuringMachine + MidiOut)
on macOS, hears actual notes from a hardware synth or DAW instrument, and
measures jitter within the P99 budget (<500µs).

### Phase 3 -- Patch Persistence (Week 4)

Save and load the rack to a file.

```
Phase 3 deliverables:
  [P3-1] bevy_reflect integration for all core types.
  [P3-2] Patch struct + ModuleConfig + CableConfig serialization.
  [P3-3] RON save path.
  [P3-4] RON load path with validation (unknown module kinds, illegal
         merges, feedback cycles).
  [P3-5] Integration test: save a patch, load it in a fresh process, tick
         both, assert byte-identical output streams.
  [P3-6] oxurack CLI: `oxurack load <patch.ron>`.
```

**Exit criterion:** The Phase 2 rack can be saved to a RON file, loaded
in a fresh process, and produces bit-identical MIDI output.

### Phase 4 -- Slave Clock (Week 5)

Make oxurack follow Logic Pro's MIDI clock.

```
Phase 4 deliverables:
  [P4-1] oxurack-rt ClockMode::Slave implementation.
  [P4-2] Phase-locked loop for smoothing external clock jitter.
  [P4-3] Transport state machine (Start/Stop/Continue).
  [P4-4] Integration with Logic Pro: enable External Sync, verify
         oxurack follows transport and tempo.
  [P4-5] Runtime clock-mode switching (master <-> slave).
  [P4-6] oxurack CLI: `oxurack --clock-slave <input-port>`.
```

**Exit criterion:** Duncan records a live oxurack performance into Logic
Pro with Logic as clock master. Tempo changes in Logic are followed
smoothly.

### Phase 5 -- Essential Generators (Weeks 6-8)

Flesh out the generator catalog.

```
Phase 5 deliverables:
  [P5-1] oxurack-mod-noise: uniform, gaussian, pink (pick one for v1).
  [P5-2] oxurack-mod-lfo: sine, triangle, saw-up, saw-down, square.
         Free-run and clock-sync modes.
  [P5-3] oxurack-mod-sequencer: 16-step fixed pattern, per-step values
         for note/vel/gate/slide.
  [P5-4] oxurack-mod-euclidean: k-of-n rhythm generator with rotation.
  [P5-5] oxurack-mod-clock: divider (/2, /3, /4, ...) and multiplier
         (*2, *3, ...) driven by clock_in port.
  [P5-6] Per-module integration tests.
```

**Exit criterion:** Duncan can build a patch using Noise + Turing Machine
- LFO + Euclidean + MidiOut that generates musically interesting output.

### Phase 6 -- Transformers (Weeks 9-10)

Add the modules that shape and filter values.

```
Phase 6 deliverables:
  [P6-1] oxurack-mod-quantizer: standalone scale quantizer (Float -> Float).
  [P6-2] oxurack-mod-range: scale/offset/clamp with both Float and
         Bipolar modes.
  [P6-3] oxurack-mod-sample-hold: latches input on rising gate.
  [P6-4] oxurack-mod-arpeggiator: up/down/random order, note-held state.
```

**Exit criterion:** Patch complexity takes another step up. Quantizer can
reshape Noise output; S&H can freeze an LFO on a trigger; Arpeggiator can
take a chord's notes and produce a sequence.

### Phase 7 -- REPL (Weeks 11-12)

The live-tweaking interface.

```
Phase 7 deliverables:
  [P7-1] oxurack REPL: rustyline-based prompt, command parser.
  [P7-2] Commands: load, save, list, connect, disconnect, set, get,
         panic, quit.
  [P7-3] Command completion for module names, port names, parameters.
  [P7-4] Value inspection: `watch <module>.<port>` streams live values.
  [P7-5] NanoKONTROL2 binding: map CC inputs to parameter setters via
         a config file.
```

**Exit criterion:** Duncan can load a patch, tweak parameters from the
REPL and a nanoKONTROL2, and see/hear the changes live.

### Phase 8 -- Polish (Weeks 13-14)

Bug fixes, documentation, benchmarks, a blog post.

```
Phase 8 deliverables:
  [P8-1] Every module has a README and usage example.
  [P8-2] Performance benchmarks: tick latency at 10, 50, 100 modules.
  [P8-3] Jitter measurement across platforms (at minimum: macOS).
  [P8-4] Error messages reviewed for actionability.
  [P8-5] Long-running soak test (tick for 24h, assert no panics, no leaks).
  [P8-6] Tag v1.0.0.
```

**Exit criterion:** v1.0.0 is tagged. Duncan uses it for a full recording
session and is not embarrassed by it.

## 3. Module Specifications at a Glance

Condensed form; each module gets a proper design doc as it's implemented.

### 3.1 turingmachine

Status: existing crate being ported. Full spec in
`0001-turing-machine-to-midi-module-design-doc.md`.

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `note_out` | out | Float | 0.0..=1.0, multiplied by parameter `range` |
| `gate_out` | out | Gate | |
| `vel_out` | out | Float | 0.0..=1.0, scaled to MIDI 0..=127 at output |
| `clock_in` | in | Gate | Advance on rising edge |
| `write_in` | in | Float | CV modulation of write probability |

**Parameters:** `length`, `write_probability`, `range`, `scale`, `seed`.

### 3.2 noise

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `out` | out | Float | 0.0..=1.0 |
| `bipolar_out` | out | Bipolar | -1.0..=1.0 |
| `gate_out` | out | Gate | Fires on each generation tick |
| `trigger_in` | in | Gate | If connected: only emit on rising edge |

**Parameters:** `distribution` (Choice: uniform, gaussian, pink), `seed`.

### 3.3 lfo

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `out` | out | Float | 0.0..=1.0 |
| `bipolar_out` | out | Bipolar | -1.0..=1.0 |
| `freq_in` | in | Float | CV modulation of frequency |
| `sync_in` | in | Gate | Reset phase on rising edge |

**Parameters:** `waveform` (Choice: sine, tri, saw_up, saw_down, square),
`frequency_hz`, `phase_offset`, `pulse_width` (for square).

### 3.4 sequencer

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `note_out` | out | Float | |
| `gate_out` | out | Gate | |
| `vel_out` | out | Float | |
| `clock_in` | in | Gate | Advance on rising edge |
| `reset_in` | in | Gate | Reset to step 0 |

**Parameters:** `length`, `steps` (array of per-step note/vel/gate),
`direction` (Choice: forward, backward, ping_pong, random).

### 3.5 euclidean

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `gate_out` | out | Gate | |
| `clock_in` | in | Gate | Advance on rising edge |

**Parameters:** `pulses` (k), `steps` (n), `rotation`.

### 3.6 clock

A module that lives inside the rack and produces clock gates at rates
derived from the master clock. Not to be confused with `oxurack-rt`'s master
clock -- this is a divider/multiplier seen by other modules as just another
Gate source.

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `out` | out | Gate | Divided/multiplied clock |
| `reset_in` | in | Gate | Realign to master clock |

**Parameters:** `divisor` (Int: positive = divide, negative = multiply),
`offset` (Int).

### 3.7 quantizer

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `out` | out | Float | Quantized to scale |
| `in` | in | Float | |

**Parameters:** `scale`.

### 3.8 range

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `out` | out | Float | Scaled input |
| `bipolar_out` | out | Bipolar | Scaled input in bipolar range |
| `in` | in | Float | |

**Parameters:** `factor`, `offset`, `min`, `max`.

### 3.9 sample-hold

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `out` | out | Float | Last latched value |
| `in` | in | Float | |
| `trigger_in` | in | Gate | Latch on rising edge |

**Parameters:** `initial_value`.

### 3.10 arpeggiator

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `note_out` | out | Float | Current arp note |
| `gate_out` | out | Gate | |
| `note_in` | in | Float | Add this note to the held set |
| `gate_in` | in | Gate | If low: release corresponding note |
| `clock_in` | in | Gate | Advance arp on rising edge |

**Parameters:** `pattern` (Choice: up, down, up_down, random), `octaves`.

### 3.11 midi-out

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `note_in` | in | Float | 0.0..=1.0 mapped to MIDI 0..=127 |
| `gate_in` | in | Gate | Note-on on rising edge, note-off on falling |
| `velocity_in` | in | Float | 0.0..=1.0 mapped to 1..=127 |
| `cc_in` | in | Float | Optional continuous CC output |
| `pitch_bend_in` | in | Bipolar | Optional pitch bend |

**Parameters:** `midi_device_name`, `channel`, `cc_number` (for cc_in),
`note_offset` (transpose), `base_note`, `note_range`.

### 3.12 midi-in (v1.1)

**Ports:**

| Name | Dir | Kind | Notes |
|---|---|---|---|
| `note_out` | out | Float | Last note received |
| `gate_out` | out | Gate | Any note held? |
| `velocity_out` | out | Float | |
| `cc_out` | out | Float | Last CC value (for configured CC number) |

**Parameters:** `midi_device_name`, `channel`, `cc_number`.

## 4. Lineage from Underack

For reference: how this catalog maps to the underack Elixir modules Duncan
previously built. Most-to-least direct correspondence:

| oxurack module | underack module | Notes |
|---|---|---|
| turingmachine | `Underack.Modules.TuringMachine` | Preserved |
| noise | `Underack.Modules.Noise` | Preserved; distributions match |
| lfo | `Underack.Modules.LFO` | Preserved |
| sequencer | `Underack.Modules.Sequencer` | Simplified (no sub-step modes in v1) |
| euclidean | `Underack.Modules.Euclidean` | Preserved |
| clock | `Underack.Clock.Divider` | Moved from core to a module |
| quantizer | `Underack.Modules.Quantizer` | Extracted from TM-internal logic |
| range | `Underack.Modules.Range` | Preserved |
| sample-hold | `Underack.Modules.SampleHold` | Preserved |
| arpeggiator | `Underack.Modules.Arpeggiator` | Preserved |
| midi-out | `Underack.MIDI.Output` | Moved from core to a module |
| midi-in | `Underack.MIDI.Input` | Moved from core to a module |

Modules in underack that did **not** make the v1 cut (deferred or
reconsidered):

- `Slew` -- considered; replaceable by an `LFO` in a particular config.
  Hold for v1.1 if demand appears.
- `Clock Divider` / `Clock Multiplier` as separate modules -- merged into
  one `clock` module with a signed divisor.
- Various audio-adjacent modules (filters, VCAs) that don't make sense in
  the MIDI domain.

## 5. Dependency DAG

```
oxurack-core   <--- everybody
     ^
     |
oxurack-rt  (uses core's types, doesn't depend on bevy_app)
     ^
     |
oxurack (umbrella)
  ├── CorePlugin from oxurack-core
  ├── RtBridge setup from oxurack-rt
  ├── REPL
  └── re-exports of every oxurack-mod-* crate

oxurack-mod-turingmachine   --> oxurack-core
oxurack-mod-noise           --> oxurack-core
oxurack-mod-lfo             --> oxurack-core
oxurack-mod-sequencer       --> oxurack-core
oxurack-mod-euclidean       --> oxurack-core
oxurack-mod-clock           --> oxurack-core
oxurack-mod-quantizer       --> oxurack-core
oxurack-mod-range           --> oxurack-core
oxurack-mod-sample-hold     --> oxurack-core
oxurack-mod-arpeggiator     --> oxurack-core
oxurack-mod-midi-out        --> oxurack-core
oxurack-mod-midi-in         --> oxurack-core    (v1.1)
```

No module crate depends on another module crate. No module crate depends
on `oxurack` (the umbrella). This means any module can be used standalone
by a downstream project that wants, e.g., just a Turing Machine without
the rack.

## 6. Out of Scope for v1

Listed here so the scope boundary is explicit:

- **GUI / visual patch editor.** Possibly v2 as an egui app.
- **Web interface** / browser-based editor.
- **VST/AU/CLAP plugin wrapper.**
- **Patch browser / preset library management.**
- **MPE polyphony management.**
- **MIDI 2.0 support.**
- **Feedback cycles.** (Rejected at load; planned for v2.)
- **Parallel module execution.** (Single-threaded tick; v2 optimization.)
- **WASM plugin module loading.**
- **Per-tick BatchGenerator.** (May revisit if profiling demands.)

## 7. Measuring Success

v1.0 is successful if:

1. Duncan uses it for real musical work -- not just as a tech demo.
2. A live-recorded session into Logic Pro feels good and sounds good.
3. Patches are reproducible across sessions, machines, and OxuRack versions
   (with a stable seed).
4. Jitter budget is met on target hardware.
5. CPU and memory footprint leave room for a DAW on the same machine
   (the thing that drove us away from VCV Rack).
6. The module-authoring story is smooth enough that Duncan writes a new
   module during the v1 cycle and doesn't fight the infrastructure.

Success for v1.0 is not: everyone else adopts it. Adoption is a v2
concern.
