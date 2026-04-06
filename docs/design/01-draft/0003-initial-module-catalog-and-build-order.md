---
number: 3
title: "Initial Module Catalog and Build Order"
author: "functional category"
component: All
tags: [change-me]
created: 2026-04-06
updated: 2026-04-06
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# Initial Module Catalog and Build Order

## Complete Module List

Thirteen modules organized by functional category. Each entry includes the
module's purpose, its underack/Eurorack lineage where applicable, key design
notes, and dependencies on other oxurack crates.

### Timing

#### 1. Clock

The heartbeat of the system. Emits tick pulses at a configurable BPM.

- **Outputs**: master tick (bool), division outputs (div2, div4, div8), multiplication outputs (x2, x3), current BPM
- **Parameters**: BPM (f32), swing amount (0.0--1.0)
- **Underack lineage**: Issues #6, #22, #23, #26
- **Notes**:
  - The existing `ClockDivider` in turingmachine is a building block. The full
    clock module adds BPM-driven timing, swing, and multiplied outputs.
  - Multiplication means emitting *more* ticks per beat (e.g., x2 = eighth
    notes when master is quarter notes). Implement as a counter that fires
    N times per master tick interval.
  - Swing offsets every other tick by a percentage of the beat interval.
    At swing=0.0, ticks are even. At swing=1.0, the off-beat tick is delayed
    to the next downbeat (maximum shuffle).
  - BPM changes should take effect on the next tick, not mid-interval. No need
    for smooth tempo ramps in v1.
  - In standalone use, the clock is driven by wall-clock time
    (`std::time::Instant`). In rack use, the rack drives it.
- **Dependencies**: oxurack-core

#### 2. Euclidean Rhythm Generator

Distributes a number of pulses as evenly as possible across a number of steps.

- **Outputs**: trigger pattern (bool per step), accent pattern (optional)
- **Parameters**: steps (1--64), pulses (0--steps), rotation (0--steps)
- **Notes**:
  - Bjorklund's algorithm. The math is simple; the musical results are
    surprisingly rich.
  - Should accept an external clock to advance steps (in rack mode, patched
    from the clock module).
  - Multiple instances with different pulse counts create polyrhythmic
    textures.
  - Consider outputting the full pattern as a `[bool; N]` for inspection,
    plus a per-tick trigger output.
- **Dependencies**: oxurack-core

### Modulation Sources

#### 3. Noise

Random value generation with multiple distribution algorithms.

- **Outputs**: value (u8, 0--127), bipolar value (i8, -64--63)
- **Parameters**: algorithm selection, range min/max, smoothing factor
- **Algorithms**:
  - Uniform: flat random distribution (simple `rng.random()`)
  - Gaussian/Normal: clustered around center, configurable spread (sigma)
  - Perlin noise: smooth, correlated random -- excellent for slow organic
    modulation of parameters like filter cutoff or pan
  - Simplex noise: similar to Perlin but computationally cheaper and without
    directional artifacts
- **Underack lineage**: Issues #10, #27
- **Notes**:
  - Perlin and Simplex are the high-value additions over basic uniform random.
    They produce *correlated* streams where adjacent values are close together,
    creating smooth, evolving modulation rather than jumpy randomness.
  - Use the executor/batch-generation pattern (see infrastructure doc) for
    Perlin and Simplex -- generating one sample at a time is expensive.
  - The `noise` Rust crate provides Perlin and Simplex implementations.
    Evaluate whether to depend on it or roll a minimal version.
  - Uniform and Gaussian can use `rand` directly.
- **Dependencies**: oxurack-core, rand, (optionally) noise crate

#### 4. LFO (Low-Frequency Oscillator)

Produces slowly-changing periodic waveforms for modulation.

- **Outputs**: value (u8, 0--127), bipolar value (i8, -64--63), gate (bool, high when value > midpoint)
- **Parameters**: waveform, rate (Hz or tempo-synced division), phase offset, amplitude, center offset
- **Waveforms**: sine, triangle, square, sawtooth (up/down), random (sample & hold per cycle)
- **Notes**:
  - Rate in two modes: free-running (Hz) and tempo-synced (e.g., "1 cycle per
    4 beats"). Tempo-synced mode requires knowing the current BPM, either
    passed as a parameter or received via cable from the clock.
  - Phase offset (0.0--1.0) allows multiple LFOs at the same rate to create
    phase-shifted patterns.
  - For the random waveform: generate a new random value at the start of each
    cycle and hold it. This is effectively a slow sample-and-hold.
  - Output scaling: internally compute as f32 (-1.0 to 1.0), then map to the
    configured output range.
  - In standalone use, advance by time delta. In rack use, advance by tick
    (the rack provides the time delta).
- **Dependencies**: oxurack-core

### Value Transformation

#### 5. Quantizer (standalone)

Constrains input MIDI values to a musical scale.

- **Outputs**: quantized note (u8)
- **Parameters**: scale, root note, input value
- **Notes**:
  - The Turing Machine already has a built-in quantizer (`quantizer.rs`).
    The standalone quantizer crate should re-use or share that `Scale` type
    and quantization logic.
  - Option A: extract the `Scale` and quantization code into oxurack-core so
    both turingmachine and the standalone quantizer depend on it.
  - Option B: the standalone quantizer crate depends on the turingmachine
    crate's public `Scale` type. This creates an undesirable coupling.
  - **Recommendation**: Option A. Move `Scale` and its `quantize()` method
    into oxurack-core. The turingmachine `Quantizer` struct stays in its
    crate but uses `core::Scale`. The standalone quantizer module also uses
    `core::Scale`.
  - In rack use, this module sits between a value source (noise, LFO) and
    a MIDI output, snapping continuous values to scale degrees.
- **Dependencies**: oxurack-core

#### 6. Range / Scaler

Maps values from one range to another.

- **Outputs**: scaled value (u8 or f32)
- **Parameters**: input min/max, output min/max, curve (linear, exponential, logarithmic)
- **Underack lineage**: Issue #30
- **Notes**:
  - The Eurorack equivalent is an attenuverter or voltage scaler.
  - Essential utility for patching: noise produces 0--127, but you want
    velocity in 60--100 or write probability in 0.3--0.8.
  - Curve modes: linear (default), exponential (more resolution at low end),
    logarithmic (more resolution at high end). Start with linear only.
  - Should clamp output to the destination range -- never produce out-of-bounds
    values.
  - This could be a function in oxurack-core rather than a full module, but
    making it a module allows it to participate in rack routing.
- **Dependencies**: oxurack-core

#### 7. Sample & Hold

Captures an input value when triggered and holds it until the next trigger.

- **Outputs**: held value (u8)
- **Parameters**: input value source, trigger source
- **Underack lineage**: Issue #31
- **Notes**:
  - Classic generative tool: patch noise into the value input, clock into the
    trigger. On each clock tick, a new random value is captured and held.
  - The "hold" behavior means the output stays constant between triggers. This
    creates stepped, staircase-like modulation from continuous sources.
  - In standalone use: call `sample(value)` to capture and `held()` to read.
  - In rack use: trigger input comes from clock or euclidean, value input from
    noise or LFO.
  - Very small module. Could be part of a "utilities" crate that bundles
    several small modules, or standalone.
- **Dependencies**: oxurack-core

### Sequencing

#### 8. Step Sequencer

Fixed-length sequence of values that advances on each clock tick.

- **Outputs**: current value (u8), gate (bool), end-of-sequence trigger (bool)
- **Parameters**: step values (Vec), step gates (Vec<bool>), step probabilities (Vec<f32>), length, direction (forward, reverse, ping-pong, random)
- **Underack lineage**: Issue #32
- **Notes**:
  - Per-step probability: each step has a chance of firing (0.0 = skip,
    1.0 = always). Enables generative variation from a fixed sequence.
  - Ratcheting: a step can fire multiple times within its time slot (2x, 3x,
    4x). Creates rapid repeated notes -- a classic Eurorack technique.
  - Direction modes add variety: ping-pong bounces back and forth, random
    jumps to a random step each tick.
  - End-of-sequence trigger is useful for chaining sequences or resetting
    other modules.
  - Length can be changed at runtime, allowing live performance manipulation.
- **Dependencies**: oxurack-core, rand (for random direction and probability)

#### 9. Arpeggiator

Cycles through a set of held notes in a pattern.

- **Outputs**: current note (u8), velocity (u8), gate (bool)
- **Parameters**: held notes (Vec<u8>), mode (up, down, up-down, down-up, random, order), octave range (1--4), gate length (fraction of step)
- **Notes**:
  - Input is a chord (set of MIDI notes). The arpeggiator cycles through them
    in the selected pattern, optionally spanning multiple octaves.
  - In standalone use: set the chord, tick to advance, read the current note.
  - In rack use: could receive note input from a sequencer or manually set
    chords.
  - Octave range: at range=2, the arp plays the chord in the original octave,
    then repeats it one octave up, then cycles back.
  - Gate length as a fraction of the step: 0.5 = note on for half the step,
    then off. 1.0 = legato (tied). Requires the arp to track sub-step timing,
    or simply output a gate duration alongside the gate bool.
- **Dependencies**: oxurack-core

### Output

#### 10. MIDI Output

Converts value streams to MIDI messages and transmits them to external devices.

- **Outputs**: (side effect: MIDI transmission). Also echoes what was sent for monitoring.
- **Parameters**: MIDI device, channel (0--15), message type config, note-on/off pairing logic
- **Underack lineage**: Issue #33
- **Notes**:
  - The turingmachine crate already has `MidiTuringMachine` which handles MIDI
    output for a single module. The generic MIDI output module generalizes this
    for any module's outputs.
  - Accepts note, velocity, gate, and CC values via rack cables (or direct
    API in standalone use).
  - Handles note-on / note-off pairing: when gate goes high, send note-on.
    When gate goes low, send note-off for the last note. This logic is already
    proven in `midi_io.rs`.
  - Multiple MIDI output modules can coexist in a rack, each sending to a
    different device/channel.
  - Feature-gated behind `midi-io` as with the existing implementation.
- **Dependencies**: oxurack-core, midir (optional)

### Already Implemented

#### 11. Turing Machine (exists)

Shift-register randomisation and looping applied to MIDI streams. Already
fully implemented with 14 built-in scales, clock dividers, pulse and gate
expander outputs.

- **Refactoring needed**: implement the `Module` trait once oxurack-core
  defines it. Extract `Scale` and quantization logic to oxurack-core.
- **Dependencies**: oxurack-core (after refactor), rand

---

## Recommended Build Order

The order balances three concerns: musical usefulness at each step,
infrastructure dependencies, and the ability to test end-to-end.

### Phase 0: Core Infrastructure (prerequisite)

See the infrastructure design doc. Must be done before any new modules.

- Define the `Module` trait in oxurack-core.
- Move `Scale` + quantization to oxurack-core.
- Implement `BatchGenerator` for the executor pattern.
- Refactor turingmachine to use oxurack-core types.

### Phase 1: Clock

**Why first**: every other module needs a clock to be musically useful. The
clock is the reference implementation for the `Module` trait, just as it was
for underack.

After this phase: you can tick the clock standalone and inspect its outputs.

### Phase 2: Noise + Range

**Why together**: noise alone produces 0--127 values. Range lets you scale them
to useful destinations. Together they form the first patchable pair.

After this phase: clock -> noise -> range -> meaningful modulation values.

### Phase 3: Quantizer + MIDI Output

**Why together**: the quantizer makes noise musically useful (constraining to
scales), and MIDI output lets you hear the results.

After this phase: clock -> noise -> quantizer -> MIDI output = first audible
generative patch. This is the critical milestone -- the system makes music.

### Phase 4: Sample & Hold + LFO

**Why together**: sample & hold creates stepped modulation from any source.
LFO creates smooth periodic modulation. These two cover the main modulation
strategies.

After this phase: rich modulation capabilities. Clock -> LFO -> parameter
modulation; clock -> S&H(noise) -> stepped random melodies.

### Phase 5: Step Sequencer

**Why here**: with clock, quantizer, and MIDI output already working, the
sequencer adds composed (non-random) melodic content.

After this phase: both generative and composed workflows are supported.

### Phase 6: Euclidean + Arpeggiator

**Why last**: these are specialized rhythm/melody modules that add variety
but are not essential for the core workflow. They depend on having a solid
clock and MIDI output already in place.

After this phase: full module library.

### Phase 7: Rack Orchestration

**Why last**: every module works standalone. The rack is the convenience layer
that wires them together. Building it last means it can be designed based on
real experience with the modules rather than speculation.

---

## Things to Watch Out For

### Shared Scale type

The `Scale` struct and `quantize()` logic currently live in the turingmachine
crate. At least two other modules need them (standalone quantizer, potentially
sequencer for per-step quantization). **Extract to oxurack-core before building
new modules** to avoid dependency tangles.

### Clock in standalone vs. rack mode

Standalone: the clock needs to track wall-clock time (`Instant::now()`,
`Duration`). Rack: the rack drives ticks and the clock just counts.
Design the clock module to support both -- probably via a `tick()` method
(rack-driven) and a `tick_realtime(&mut self, now: Instant)` method
(standalone wall-clock mode).

### Output type consistency

The turingmachine has `StepOutputs` with specific fields. Other modules will
have different output shapes. The `Module` trait needs a way to handle this
without losing type safety. Options:

- **Associated type**: `type Output;` on the trait. Preserves types but makes
  `Vec<Box<dyn Module>>` harder (need a common output enum or trait object).
- **Common output struct**: every module returns the same `ModuleOutputs` bag
  with optional fields. Simple but wastes space and loses specificity.
- **Output ports as key-value pairs**: `HashMap<&str, Value>`. Flexible but
  loses compile-time safety.

**Recommendation**: associated type for standalone use (maximum type safety),
with a `to_port_values(&self) -> Vec<(&str, Value)>` method for rack routing.
The rack works with dynamic port values; direct users work with concrete types.

### Noise crate evaluation

The `noise` crate (crates.io) provides Perlin and Simplex. It's well-maintained
but pulls in dependencies. Evaluate size and compile-time impact. If too heavy,
a minimal Perlin implementation is ~50 lines and may be preferable.

### Swing implementation

Swing is deceptively tricky. The naive approach (delay every other tick by
a fixed amount) works for simple patterns but interacts poorly with division
outputs. Define swing as a property of the master clock only, applied before
division. Divided outputs inherit the swing pattern naturally.

### Ratcheting in the sequencer

Ratcheting (multiple triggers per step) means the sequencer needs sub-step
timing. In rack mode this is straightforward (emit multiple triggers in one
tick with timing offsets). In standalone mode, the caller needs to handle
the sub-step timing. Consider outputting a `Vec<Duration>` of trigger times
within the step, or simply a repeat count and let the caller subdivide.

### MIDI output decoupling

The generic MIDI output module should not depend on any specific module
crate. It receives note/velocity/gate/CC values through the rack's routing
or through a simple API. This is already the pattern in `MidiTuringMachine`
but generalized.

### Avoiding a God Crate

As modules multiply, resist the urge to put shared utilities in oxurack-core
unless they are genuinely needed by multiple crates. A "utils" crate that
grows without discipline becomes a dependency bottleneck. Core should contain:
the Module trait, Scale/quantization, Value types, BatchGenerator. Nothing
else unless two or more modules need it.
