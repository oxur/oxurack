---
number: 1
title: "Turing Machine to MIDI - Module Design Doc"
author: "one position"
component: All
tags: [change-me]
created: 2026-04-04
updated: 2026-04-18
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# Turing Machine to MIDI - Module Design Doc

## Context

You are working in the `oxur/oxurack` Rust workspace. The workspace root has a
`Cargo.toml` that includes `crates/turingmachine` in its `[workspace.members]`.

This crate is a **MIDI-domain port of the Music Thing Modular Turing Machine Mk2**
(hardware schematic dated May 2016). It does **not** do voltage or audio
calculations. Instead it applies the same shift-register, randomisation, loop,
and clock logic to **MIDI data** — notes, velocities, CCs, gates, and timing.

The goal is a clean Rust library usable both:

1. From application code (DAW plugin, standalone app), and
2. From a REPL / live-coding session (ergonomic, low-ceremony API).

---

## 0. Workspace setup

Ensure `oxurack/Cargo.toml` exists and contains:

```toml
[workspace]
resolver = "2"
members = [
    "crates/turingmachine",
]
```

---

## 1. Crate scaffold

Create/update `crates/turingmachine/` with this layout:

```
crates/turingmachine/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    ├── error.rs            # Crate error types
    ├── shift_register.rs   # The core 16-bit shift register
    ├── write_knob.rs       # Probability / mutation logic
    ├── length.rs           # Loop-length selector
    ├── quantizer.rs        # Scale quantization (replaces DAC + resistor network)
    ├── clock.rs            # Clock divider / pulse logic
    ├── outputs.rs          # All output types (note, gate, velocity, cc, noise)
    └── engine.rs           # Top-level TuringMachine struct wiring it all together
```

---

## 2. `Cargo.toml`

```toml
[package]
name    = "turingmachine"
version = "0.1.0"
edition = "2024"
description = "MIDI-domain Turing Machine Mk2. Applies shift-register randomisation and looping to MIDI note, velocity, gate, and CC streams."
license = "MIT OR Apache-2.0"
repository = "https://github.com/oxur/oxurack"

[dependencies]
rand       = "0.9"
midir      = { version = "0.10", optional = true }
serde      = { version = "1", features = ["derive"], optional = true }
thiserror  = "2"

[features]
default    = []
midi-io    = ["midir"]
serde      = ["dep:serde"]

[dev-dependencies]
pretty_assertions = "1"
```

---

## 3. `src/error.rs`

Crate-level error type using `thiserror`. This provides structured errors
for MIDI I/O failures (feature-gated) and keeps the door open for future
error variants without breaking callers.

```rust
use std::fmt;

/// Crate-level error type.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    /// MIDI send failed (only when `midi-io` feature is enabled).
    #[cfg(feature = "midi-io")]
    Midi(midir::SendError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            #[cfg(feature = "midi-io")]
            ErrorKind::Midi(e) => write!(f, "MIDI send error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            #[cfg(feature = "midi-io")]
            ErrorKind::Midi(e) => Some(e),
        }
    }
}

#[cfg(feature = "midi-io")]
impl From<midir::SendError> for Error {
    fn from(e: midir::SendError) -> Self {
        Self { kind: ErrorKind::Midi(e) }
    }
}
```

---

## 4. `src/shift_register.rs`

This is the exact hardware equivalent of the four CD4015 ICs chained together
on the TuringBack schematic (SHFREG_MAIN_A, SHFREG_MAIN_B, SHFREG_EXT_A,
SHFREG_EXT_B — each is a 4-bit shift register, giving 16 bits total).

Rules directly from the schematic:

- On each clock pulse the register shifts **left** by one position.
- The new bit inserted at position 0 (rightmost) is decided by `WriteKnob`
  (see §5). It is either the **feedback bit** (from position `length-1`)
  unchanged, or a fresh random bit — chosen per-step by the probability.
- The display (LEDs on hardware, `bits()` here) shows all 16 positions,
  with bit 15 on the left and bit 0 on the right, **regardless of length**.
- Positions beyond `length` are still shifted and stored; they just are not
  the source of the feedback bit. This matches the hardware exactly — all
  four CD4015s always clock.

```rust
/// Models the four chained CD4015 4-bit shift registers (16 bits total).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftRegister {
    /// Bit 15 = oldest, bit 0 = most recently clocked in.
    bits: u16,
}

impl ShiftRegister {
    pub fn new() -> Self { Self { bits: 0 } }
    pub fn new_random(rng: &mut impl rand::Rng) -> Self { /* fill with random bits */ }

    /// Clock in `new_bit` (the bit decided by the write knob).
    /// Shifts left; new_bit enters at position 0.
    pub fn clock(&mut self, new_bit: bool) { ... }

    /// The feedback bit: the bit at position `length - 1`.
    /// On hardware this is the tap that gets re-injected at the input
    /// via the CD4016 quad switch.
    pub fn feedback_bit(&self, length: usize) -> bool { ... }

    /// Read all 16 bits for display / inspection.
    pub fn bits(&self) -> u16 { self.bits }

    /// The 8 most-significant active bits (positions length-1..length-8,
    /// clamped). This is what the hardware DAC0808 receives.
    /// For MIDI we use this as a raw 8-bit value (0–255) before quantization.
    pub fn dac_byte(&self, length: usize) -> u8 { ... }

    /// Bit at position 7 of the dac_byte — maps to PULSE_OUT on hardware.
    /// True = gate high.
    pub fn pulse_bit(&self, length: usize) -> bool { ... }

    /// All 16 individual bits as an array (index 0 = oldest / leftmost LED).
    pub fn to_bools(&self) -> [bool; 16] { ... }

    /// Reset to zero (equivalent of hardware RESET jack).
    pub fn reset(&mut self) { self.bits = 0; }
}

impl Default for ShiftRegister {
    fn default() -> Self { Self::new() }
}
```

---

## 5. `src/write_knob.rs`

Models the **WRITE knob** and its CV input — the TL072 comparator + CD4016
quad-switch section on TuringFront (top-left of the front schematic).

The probability drives four CD4016 switches: at full CW all four switches
route the feedback bit back unchanged (locked loop); at full CCW all four
route the noise source instead (fully random); at noon each step is a
coin flip.

```rust
/// Probability of *keeping* (looping) the feedback bit vs substituting noise.
/// 0.0 = fully random (all noise), 1.0 = locked loop, 0.5 = 50 ⁄ 50.
#[derive(Debug, Clone, PartialEq)]
pub struct WriteKnob {
    /// 0.0 – 1.0
    probability: f32,
}

impl WriteKnob {
    /// Create with a probability value. Clamps to 0.0–1.0.
    pub fn new(probability: f32) -> Self { ... }

    /// Set from a normalised value 0.0–1.0 (maps to the physical knob range).
    /// Clamps to 0.0–1.0.
    pub fn set_probability(&mut self, value: f32) { ... }

    /// Modulate by a signed offset (−1.0..+1.0), as if patching the CV_IN jack.
    /// Clamps to 0.0–1.0 after application.
    pub fn modulate(&mut self, offset: f32) { ... }

    /// Given the current feedback bit, return the bit that should be clocked in.
    /// Implements the CD4016 randomised invert/loop switch logic:
    ///   - With probability `self.probability` → return `feedback_bit` unchanged
    ///   - Otherwise                           → return a fresh random bit
    pub fn resolve(&self, feedback_bit: bool, rng: &mut impl rand::Rng) -> bool { ... }

    pub fn probability(&self) -> f32 { self.probability }
}
```

---

## 6. `src/length.rs`

Models the **ALPS-SR8V rotary switch** labelled LENGTH on the panel. The
hardware has exactly 9 positions. Implement them exactly as labelled:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LengthSelector {
    position: usize, // 0–8
}

impl LengthSelector {
    /// The nine valid loop lengths, matching the hardware rotary switch positions.
    pub const VALID_LENGTHS: [usize; 9] = [2, 3, 4, 5, 6, 8, 10, 12, 16];

    pub fn new() -> Self { Self { position: 8 } } // default = 16 (full loop)

    /// Set by position index (0 = length 2, 8 = length 16).
    /// Clamps to 0–8.
    pub fn set_position(&mut self, pos: usize) { ... }

    /// Set by desired length value; rounds to nearest valid length.
    pub fn set_length(&mut self, len: usize) { ... }

    /// Increment/decrement position (for a MOVE-style step through lengths).
    /// Saturates at bounds.
    pub fn increment(&mut self) { ... }
    pub fn decrement(&mut self) { ... }

    pub fn length(&self) -> usize { Self::VALID_LENGTHS[self.position] }
    pub fn position(&self) -> usize { self.position }
}

impl Default for LengthSelector {
    fn default() -> Self { Self::new() }
}
```

---

## 7. `src/quantizer.rs`

This replaces both the **DAC0808** voltage-output stage (TuringFront) and the
**Volts expander** resistor network that would translate voltage to pitch. In
the MIDI domain we map the raw 8-bit DAC byte to a MIDI note number,
optionally quantizing to a musical scale and transposing to a root.

The 8-bit value (0–255) is first scaled to MIDI note range, then snapped
to the nearest pitch in the chosen scale.

```rust
/// A musical scale as a set of semitone offsets within an octave (0–11).
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Scale {
    /// Semitone offsets, sorted, values in 0..=11. Must be non-empty.
    intervals: Vec<u8>,
    /// Display name e.g. "Major", "Pentatonic Minor".
    name: String,
}

impl Scale {
    pub fn new(intervals: Vec<u8>, name: impl Into<String>) -> Self { ... }

    // Built-in scales — implement all of these:
    pub fn chromatic()         -> Self { ... } // [0,1,2,3,4,5,6,7,8,9,10,11]
    pub fn major()             -> Self { ... } // [0,2,4,5,7,9,11]
    pub fn natural_minor()     -> Self { ... } // [0,2,3,5,7,8,10]
    pub fn harmonic_minor()    -> Self { ... } // [0,2,3,5,7,8,11]
    pub fn pentatonic_major()  -> Self { ... } // [0,2,4,7,9]
    pub fn pentatonic_minor()  -> Self { ... } // [0,3,5,7,10]
    pub fn blues()             -> Self { ... } // [0,3,5,6,7,10]
    pub fn dorian()            -> Self { ... } // [0,2,3,5,7,9,10]
    pub fn phrygian()          -> Self { ... } // [0,1,3,5,7,8,10]
    pub fn lydian()            -> Self { ... } // [0,2,4,6,7,9,11]
    pub fn mixolydian()        -> Self { ... } // [0,2,4,5,7,9,10]
    pub fn whole_tone()        -> Self { ... } // [0,2,4,6,8,10]
    pub fn diminished()        -> Self { ... } // [0,2,3,5,6,8,9,11]
    pub fn augmented()         -> Self { ... } // [0,3,4,7,8,11]

    /// Snap a MIDI note number to the nearest note in this scale,
    /// given a root note (0 = C, 1 = C#, etc.)
    pub fn quantize(&self, raw_note: u8, root: u8) -> u8 { ... }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quantizer {
    scale: Scale,
    root:  u8,           // MIDI note 0–11 (C=0)
    range: RangeInclusive<u8>, // MIDI note output range (default 36..=84, C2–C6)
}

impl Quantizer {
    pub fn new(scale: Scale, root: u8) -> Self { ... }
    pub fn set_scale(&mut self, scale: Scale) { ... }
    /// Set root note (0–11, C=0). Clamps to 0–11.
    pub fn set_root(&mut self, root: u8) { ... }
    /// Set the output note range (e.g. `36..=84` for C2–C6).
    pub fn set_range(&mut self, range: RangeInclusive<u8>) { ... }

    /// Map an 8-bit DAC value (0–255) → quantized MIDI note (0–127).
    /// Steps:
    ///   1. Scale 0–255 linearly to the configured note range.
    ///   2. Snap to nearest in-scale pitch.
    pub fn note_from_dac(&self, dac: u8) -> u8 { ... }

    /// Map an 8-bit DAC value → velocity (1–127, never 0 to avoid note-off).
    /// Uses the lower 7 bits, scaled to 1–127.
    pub fn velocity_from_dac(&self, dac: u8) -> u8 { ... }
}
```

---

## 8. `src/clock.rs`

Models the **clock input stage** (TL074 Schmitt comparator on TuringFront)
and the **clock divider outputs** present on the VCV Rack module (½, ¼ of
the master clock, also shown as "4½" and "1½" division sockets).

In the MIDI domain, "clock" is a step tick — callers call `tick()` and the
engine decides which outputs fire. The dividers use simple counter logic.

```rust
use std::num::NonZeroU32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClockDivider {
    /// Division factor. 1 = pass-through, 2 = half speed, etc.
    division: NonZeroU32,
    counter:  u32,
}

impl ClockDivider {
    pub fn new(division: NonZeroU32) -> Self { ... }

    /// Advance by one master tick. Returns `true` if this divider fires.
    pub fn tick(&mut self) -> bool { ... }

    /// Reset counter (RESET jack equivalent).
    pub fn reset(&mut self) { self.counter = 0; }

    pub fn division(&self) -> NonZeroU32 { self.division }
}
```

---

## 9. `src/outputs.rs`

All output types the engine produces, modelling the full set of jacks on
both the physical Mk2 and the Stellare VCV module.

```rust
/// A single step's worth of outputs from the Turing Machine.
/// All fields are `Option<T>` — `None` means the jack did not fire this step.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct StepOutputs {
    // ── Main outputs (both hardware and VCV) ──────────────────────────────

    /// CV OUT equivalent: the quantized MIDI note derived from the 8-bit DAC byte.
    pub note: Option<u8>,

    /// Velocity derived from the lower bits of the DAC byte (1–127).
    pub velocity: Option<u8>,

    /// PULSE OUT equivalent: true when bit 7 of the DAC byte is high.
    /// Maps to a MIDI Note On (paired with `note` and `velocity`) or a trigger CC.
    pub gate: bool,

    // ── Scale / Volts expander equivalent ────────────────────────────────

    /// Same note but independently quantizable (the Volts expander allowed
    /// a different resistor-network scale). Here it uses a separate Scale config.
    pub scale_note: Option<u8>,

    // ── Pulses expander equivalent (AND gates on TuringBack) ─────────────
    /// Six pulse outputs, one per AND-gate pair (PULSE1–PULSE6).
    /// Each is the AND of two adjacent shift-register bits, matching the
    /// CD4081 logic on the TuringBack schematic.
    pub pulses: [bool; 6],

    // ── Gates expander equivalent (CD4050 buffers on TuringBack) ─────────
    /// Eight individual gate outputs, one per shift-register bit (bits 0–7),
    /// buffered and available individually. These map directly to the
    /// GATE1–GATE8 sockets on the hardware expander.
    pub gates: [bool; 8],

    // ── Clock divider outputs (VCV Rack module additions) ─────────────────
    /// Fires every 2 master clocks (½ speed). Equivalent to the "½" socket.
    pub div2: bool,
    /// Fires every 4 master clocks (¼ speed). Equivalent to the "¼" socket.
    pub div4: bool,

    // ── Noise output ──────────────────────────────────────────────────────
    /// A random CC value (0–127) each step. Maps to NOISE OUT on hardware.
    /// Callers route this to a CC number of their choice.
    pub noise_cc: u8,

    // ── State snapshot (for display / inspection) ─────────────────────────
    /// The raw shift register bits at this step (bit 15 = leftmost LED).
    pub register_bits: u16,
    /// The loop length active this step.
    pub length: usize,
    /// The write probability active this step.
    pub write_probability: f32,
}
```

---

## 10. `src/engine.rs`

The top-level `TuringMachine` struct. This wires everything together
exactly as the hardware does, following the signal path:

```
CLOCK tick
  → ShiftRegister::clock(WriteKnob::resolve(feedback_bit, rng))
  → dac_byte → Quantizer::note_from_dac  → StepOutputs::note
                Quantizer::velocity_from_dac → StepOutputs::velocity
  → pulse_bit → StepOutputs::gate
  → bits[0..7] → individual AND pairs   → StepOutputs::pulses[0..5]
  → bits[0..7] → individual buffers     → StepOutputs::gates[0..7]
  → ClockDivider(2)::tick               → StepOutputs::div2
  → ClockDivider(4)::tick               → StepOutputs::div4
  → rand::random::<u8>()                → StepOutputs::noise_cc
```

```rust
#[derive(Debug)]
pub struct TuringMachine {
    register:       ShiftRegister,
    write_knob:     WriteKnob,
    length:         LengthSelector,
    quantizer:      Quantizer,
    scale_quantizer: Quantizer,   // independent second quantizer for scale_note
    div2:           ClockDivider,
    div4:           ClockDivider,
    rng:            rand::rngs::SmallRng,
    step_count:     u64,
}

impl TuringMachine {
    /// Create with default settings:
    ///   length=16, write_probability=0.5, scale=chromatic, root=C, range=C2–C6
    pub fn new() -> Self { ... }

    /// Create with a seeded RNG for reproducible sequences.
    pub fn with_seed(seed: u64) -> Self { ... }

    // ── Main controls ─────────────────────────────────────────────────────

    /// Advance by one step. This is the CLOCK jack. Returns all outputs.
    #[must_use]
    pub fn tick(&mut self) -> StepOutputs { ... }

    /// Reset the shift register to zero and reset clock dividers.
    /// Equivalent to patching the RESET jack.
    pub fn reset(&mut self) { ... }

    /// Manually advance the shift register by one step *without* updating
    /// clock dividers or incrementing step_count. Equivalent to the VCV
    /// Rack MOVE button / jack.
    #[must_use]
    pub fn move_step(&mut self) -> StepOutputs { ... }

    // ── Parameter setters ─────────────────────────────────────────────────

    /// Set write probability (0.0 = random, 1.0 = locked).
    pub fn set_write(&mut self, probability: f32) { ... }

    /// Apply a CV-style offset to write probability (−1.0..+1.0).
    pub fn modulate_write(&mut self, offset: f32) { ... }

    /// Set loop length by position (0–8) or value (snaps to nearest valid).
    pub fn set_length_position(&mut self, pos: usize) { ... }
    pub fn set_length(&mut self, len: usize) { ... }

    /// Set the main quantizer's scale and root.
    pub fn set_scale(&mut self, scale: Scale) { ... }
    /// Set root note (0–11, C=0). Clamps to 0–11.
    pub fn set_root(&mut self, root: u8) { ... }
    /// Set the output note range (e.g. `36..=84` for C2–C6).
    pub fn set_note_range(&mut self, range: RangeInclusive<u8>) { ... }

    /// Set the secondary scale output's quantizer independently.
    pub fn set_scale_output_scale(&mut self, scale: Scale) { ... }
    pub fn set_scale_output_root(&mut self, root: u8) { ... }

    // ── Inspection ────────────────────────────────────────────────────────

    pub fn register_bits(&self) -> u16 { ... }
    pub fn current_length(&self) -> usize { ... }
    pub fn write_probability(&self) -> f32 { ... }
    pub fn step_count(&self) -> u64 { ... }
}

/// Human-readable display of the register state.
/// Shows '0' and '1' characters, left = oldest, right = newest.
/// Marks the active loop window with brackets, e.g.:
///   `1011[00110101]0011101` where [] = active loop length.
impl fmt::Display for TuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { ... }
}

impl Default for TuringMachine {
    fn default() -> Self { Self::new() }
}
```

---

## 11. `src/lib.rs`

Private modules with public API re-exported at crate root:

```rust
mod shift_register;
mod write_knob;
mod length;
mod quantizer;
mod clock;
mod outputs;
mod error;
mod engine;

// Re-export the public API at crate root.
pub use engine::TuringMachine;
pub use outputs::StepOutputs;
pub use quantizer::{Quantizer, Scale};
pub use length::LengthSelector;
pub use write_knob::WriteKnob;
pub use clock::ClockDivider;
pub use shift_register::ShiftRegister;
pub use error::Error;
```

---

## 12. MIDI I/O layer (feature-gated)

When the `midi-io` feature is enabled, add `src/midi_io.rs` and expose:

```rust
/// Wraps a TuringMachine and a midir output connection.
/// On each call to `tick()` it fires the appropriate MIDI messages
/// on the configured channel.
#[derive(Debug)]
pub struct MidiTuringMachine {
    engine:        TuringMachine,
    conn_out:      midir::MidiOutputConnection,
    channel:       u8,        // 0–15
    note_cc:       Option<u8>, // if Some, also send note as CC on this number
    velocity_cc:   Option<u8>, // if Some, send velocity as CC
    noise_cc:      Option<u8>, // if Some, send noise as CC
    gate_note:     u8,         // the note number to use for gate-only messages
    last_note:     Option<u8>, // for sending Note Off before next Note On
}

impl MidiTuringMachine {
    pub fn new(engine: TuringMachine, conn_out: midir::MidiOutputConnection, channel: u8) -> Self;

    /// Tick the engine and immediately send MIDI messages.
    /// Sends:
    ///   - Note Off for previous note (if gate was high)
    ///   - Note On  with quantized note + velocity (when gate is high)
    ///   - CC messages for noise, note-as-CC, velocity-as-CC (always)
    #[must_use]
    pub fn tick(&mut self) -> Result<StepOutputs, Error>;

    /// Flush a Note Off for any currently-sounding note. Call before stopping.
    pub fn all_notes_off(&mut self) -> Result<(), Error>;

    /// Set MIDI channel (0–15). Clamps to 0–15.
    pub fn set_channel(&mut self, channel: u8);
    pub fn route_noise_to_cc(&mut self, cc: u8);
    pub fn route_note_to_cc(&mut self, cc: u8);
    pub fn route_velocity_to_cc(&mut self, cc: u8);
}
```

---

## 13. Tests

Write unit tests in each module's `#[cfg(test)]` block. At minimum:

### `shift_register.rs` tests

- `clock_shifts_left`: clocking in `true` then `false` moves bits correctly.
- `feedback_bit_respects_length`: for length=4, bit 3 is the feedback.
- `dac_byte_is_bits_0_to_7`: confirm dac_byte equals the lower 8 bits.
- `pulse_bit_is_bit_7`: confirm pulse_bit == (dac_byte >> 7) & 1.
- `reset_clears_register`: after reset, bits() == 0.

### `write_knob.rs` tests

- `probability_1_always_keeps`: at prob=1.0, resolve always returns the feedback bit.
- `probability_0_ignores_feedback`: at prob=0.0, resolve returns random bits (run
  1000 iterations, confirm at least some are different from the feedback bit).
- `modulate_clamps`: modulate(+999.0) clamps to probability=1.0.

### `quantizer.rs` tests

- `chromatic_passthrough`: chromatic scale returns the linearly-scaled note unchanged.
- `major_snaps_correctly`: note 61 (C# / Db) with C major root → should snap to 60 (C).
- `range_respected`: note_from_dac(0) >= *range.start() and note_from_dac(255) <=*range.end().
- `velocity_never_zero`: velocity_from_dac(any) >= 1.

### `engine.rs` tests

- `deterministic_with_seed`: two engines with the same seed produce the same sequence
  over 32 ticks.
- `locked_loop_repeats`: at write=1.0, length=8, after 8 ticks the sequence repeats
  exactly (ticks 1–8 == ticks 9–16).
- `fully_random_no_repeat`: at write=0.0, over 64 ticks, assert that not all steps
  are identical (probability of false failure is astronomically low).
- `reset_zeroes_register`: after reset(), register_bits() == 0.
- `display_shows_brackets`: `to_string()` (via `fmt::Display`) output contains
  '[' and ']' with exactly `length` characters between them.
- `step_count_increments`: step_count() goes 0→1 after first tick.
- `pulses_are_and_of_adjacent_bits`: verify pulses[n] == (gates[n] && gates[n+1])
  for a known register state.

### `clock.rs` tests

- `div2_fires_every_other_tick`.
- `div4_fires_every_fourth_tick`.
- `reset_restarts_counters`.

---

## 14. README.md

Write a README covering:

1. **What this is** — MIDI Turing Machine, inspired by Music Thing Modular Mk2.
2. **Signal chain diagram** (ASCII art) mapping hardware blocks to Rust structs.
3. **Quick start** — `TuringMachine::new()`, calling `tick()`, reading outputs.
4. **Live-coding example** — a tight loop simulating a sequencer REPL session.
5. **MIDI I/O example** (feature-gated) — sending notes to a real MIDI port.
6. **Parameter reference** — table of all controls with hardware equivalents.
7. **Scale reference** — list all built-in scales.
8. **Output reference** — table of all `StepOutputs` fields with hardware jack equivalents.

---

## 15. Implementation notes and constraints

- **No `unsafe`.**
- **No `unwrap()` in library code** — use `Result`/`Option` properly everywhere.
- **Determinism**: when a seed is provided via `TuringMachine::with_seed()`, the
  output must be bit-for-bit reproducible across runs and platforms. Use
  `rand::rngs::SmallRng` seeded with `rand::SeedableRng::seed_from_u64`.
- The **pulse outputs** (StepOutputs::pulses) must replicate the CD4081 AND-gate
  logic from TuringBack exactly: `pulses[n] = bits[n] && bits[n+1]` for n in 0..6,
  giving 6 pulse outputs (the hardware has 6 AND gates: AND_1A/B, AND_2A/B, AND_1C/D
  from the four CD4081 packages, combining adjacent bits).
- The **gate outputs** (StepOutputs::gates) are the 8 individual bits 0..7 from the
  shift register, mirroring the CD4050 buffer chain (BUFFER1A–BUFFER2E on TuringBack).
- **MIDI note 0** (C-1) must never be sent as a Note On. Quantizer must clamp output
  to minimum 1 (or respect the configured range.min which defaults to 36).
- The `fmt::Display` output must always be exactly 16 characters of '0'/'1' plus 2
  bracket characters, never shorter or longer.
- All `pub` types must implement `Debug`. `Clone` where it makes sense.
  `Default` where `new()` takes no arguments.

---

## 16. Mapping table: hardware → MIDI

For reference during implementation:

| Hardware signal          | Hardware component             | MIDI equivalent in this crate       |
|--------------------------|-------------------------------|--------------------------------------|
| CV_OUT (0–5V, 8-bit DAC) | DAC0808 + TL074 output amp    | `StepOutputs::note` (quantized)      |
| PULSE_OUT (bit 7 gate)   | CD4050 buffer BUFFER2F        | `StepOutputs::gate`                  |
| NOISE_OUT (white noise)  | 2N3904 transistor + TL074     | `StepOutputs::noise_cc` (random u8)  |
| LENGTH rotary            | ALPS-SR8V 9-position switch   | `LengthSelector` (same 9 values)     |
| WRITE knob               | TL072 comparator + CD4016     | `WriteKnob::probability` (0.0–1.0)   |
| CV_IN (write mod)        | R22/R23 attenuverter          | `TuringMachine::modulate_write()`    |
| RESET jack               | Async reset to CD4015         | `TuringMachine::reset()`             |
| MOVE jack/button (VCV)   | Not on hardware; VCV addition | `TuringMachine::move_step()`         |
| SCALE out (VCV)          | Volts expander resistor net   | `StepOutputs::scale_note`            |
| PULSE1–PULSE6 (expander) | CD4081 AND gates (TuringBack) | `StepOutputs::pulses[0..5]`          |
| GATE1–GATE8 (expander)   | CD4050 buffers (TuringBack)   | `StepOutputs::gates[0..7]`           |
| ½ clock out (VCV)        | Not on hardware; VCV addition | `StepOutputs::div2`                  |
| ¼ clock out (VCV)        | Not on hardware; VCV addition | `StepOutputs::div4`                  |
