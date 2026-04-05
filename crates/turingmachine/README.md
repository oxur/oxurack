# turingmachine

A MIDI-domain port of the
[Music Thing Modular Turing Machine Mk2](https://musicthing.co.uk/pages/turing.html).
Instead of voltages and analog shift registers, this crate applies the same
shift-register randomisation, looping, and clock-division logic to MIDI note,
velocity, gate, and CC streams.

The library is designed for two primary use cases:

1. Embedding in an application (DAW plugin, standalone sequencer).
2. Live-coding / REPL experimentation with a minimal-ceremony API.

## Signal chain

The diagram below shows the hardware signal path and its Rust equivalent.
Each box is a struct re-exported at the crate root.

```
                          CLOCK tick
                              |
                              v
                   +---------------------+
                   |   ShiftRegister     |  16-bit (four CD4015 ICs)
                   |   .clock(new_bit)   |
                   +---------------------+
                              ^
                              |
              +-------------------------------+
              |  WriteKnob.resolve(fb, rng)   |  CD4016 quad switch + TL072
              |  probability 0.0..=1.0        |  comparator
              +-------------------------------+
                              ^
                              |
                   feedback_bit(length)
                   from LengthSelector          ALPS-SR8V 9-pos rotary
                              |
                              v
              +-------------------------------+
              |   dac_byte (8 bits)           |  DAC0808 equivalent
              +-------------------------------+
               /          |           \
              v           v            v
   +-------------+  +------------+  +------------+
   | Quantizer   |  | Quantizer  |  | pulse_bit  |
   | note_from_  |  | velocity_  |  | (bit 7)    |
   | dac()       |  | from_dac() |  +------------+
   +-------------+  +------------+        |
         |                |               v
         v                v        StepOutputs::gate
  StepOutputs::note  ::velocity
                                   bits[0..7]
                                    /       \
                                   v         v
                          pulses[0..5]   gates[0..7]
                          (CD4081 AND)   (CD4050 buf)

              +---------------+  +---------------+
              | ClockDivider  |  | ClockDivider  |
              | division=2    |  | division=4    |
              +---------------+  +---------------+
                     |                  |
                     v                  v
              StepOutputs::div2  StepOutputs::div4

              +-------------------------------+
              |  rng.random() & 0x7F          |  2N3904 noise source
              +-------------------------------+
                              |
                              v
                    StepOutputs::noise_cc
```

## Quick start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
turingmachine = { path = "crates/turingmachine" }
```

Create an engine, tick it, and read the outputs:

```rust
use turingmachine::{TuringMachine, Scale};

let mut tm = TuringMachine::new();

// Optionally configure before ticking.
tm.set_scale(Scale::pentatonic_minor());
tm.set_root(2);               // D
tm.set_length(8);              // 8-step loop
tm.set_write(0.5);             // 50 % chance of mutation each step
tm.set_note_range(48..=72);    // C3--C5

let out = tm.tick();

if out.gate {
    println!(
        "Note ON: note={}, vel={}",
        out.note.unwrap_or(0),
        out.velocity.unwrap_or(0),
    );
}
```

For **deterministic / reproducible** sequences, use a seeded constructor:

```rust
use turingmachine::TuringMachine;

let mut tm = TuringMachine::with_seed(42);
// Two engines built with the same seed produce identical output.
```

## Live-coding example

A tight loop simulating a sequencer session. Each tick prints the register
state and the current note.

```rust
use turingmachine::{TuringMachine, Scale};
use std::thread;
use std::time::Duration;

fn main() {
    let mut tm = TuringMachine::with_seed(123);
    tm.set_scale(Scale::dorian());
    tm.set_root(0);          // C
    tm.set_length(8);
    tm.set_write(0.8);       // mostly looping, occasional mutation

    for step in 0..32 {
        let out = tm.tick();

        print!("step {:>3}  reg {}  ", step, tm);

        if out.gate {
            println!(
                "NOTE {:>3}  vel {:>3}",
                out.note.unwrap_or(0),
                out.velocity.unwrap_or(0),
            );
        } else {
            println!("--rest--");
        }

        // Simulate a 120 BPM clock (500 ms per beat).
        thread::sleep(Duration::from_millis(500));
    }
}
```

## MIDI I/O example (feature-gated)

Enable the `midi-io` feature to get `MidiTuringMachine`, which wraps the
engine and a `midir` output connection. Each `tick()` sends Note On / Note
Off and CC messages to a real MIDI port.

```toml
[dependencies]
turingmachine = { path = "crates/turingmachine", features = ["midi-io"] }
```

```rust
use turingmachine::{TuringMachine, Scale};
// MidiTuringMachine is only available with the "midi-io" feature.
// use turingmachine::MidiTuringMachine;

use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let midi_out = midir::MidiOutput::new("turingmachine")?;
    let ports = midi_out.ports();
    let port = ports.first().expect("no MIDI output port found");
    let conn = midi_out.connect(port, "tm-out")?;

    let engine = TuringMachine::new();
    // let mut mtm = MidiTuringMachine::new(engine, conn, 0); // channel 0

    // mtm.route_noise_to_cc(1);   // modulation wheel

    for _ in 0..64 {
        // let out = mtm.tick()?;   // sends MIDI automatically
        thread::sleep(Duration::from_millis(500));
    }

    // mtm.all_notes_off()?;        // clean up
    Ok(())
}
```

## Parameter reference

| Method                          | Hardware equivalent                | Description                                       |
|---------------------------------|------------------------------------|---------------------------------------------------|
| `set_write(f32)`                | WRITE knob (TL072 + CD4016)       | Probability of keeping the feedback bit. 0.0 = fully random, 1.0 = locked loop, 0.5 = coin flip. |
| `modulate_write(f32)`           | CV_IN jack (R22/R23 attenuverter) | Signed offset applied to write probability. Clamped to 0.0--1.0 after application. |
| `set_length(usize)`             | LENGTH rotary (ALPS-SR8V)         | Sets loop length to the nearest valid value: 2, 3, 4, 5, 6, 8, 10, 12, or 16. |
| `set_length_position(usize)`    | LENGTH rotary position (0--8)     | Sets the rotary switch position directly. 0 = length 2, 8 = length 16. |
| `set_scale(Scale)`              | *(no hardware equivalent)*        | Replaces the main quantizer's musical scale.       |
| `set_root(u8)`                  | *(no hardware equivalent)*        | Root note for quantization. 0 = C, 1 = C#, ..., 11 = B. Clamped to 0--11. |
| `set_note_range(RangeInclusive<u8>)` | *(no hardware equivalent)*   | MIDI note output range. Default is `36..=84` (C2--C6). |
| `set_scale_output_scale(Scale)` | Volts expander resistor network   | Independent scale for the secondary quantizer output. |
| `set_scale_output_root(u8)`     | Volts expander resistor network   | Independent root note for the secondary quantizer. |
| `reset()`                       | RESET jack (async reset to CD4015)| Clears the shift register, resets clock dividers, zeroes step count. |
| `move_step()`                   | MOVE jack/button (VCV addition)   | Advances the register one step without ticking clock dividers or step count. |

## Scale reference

Fourteen built-in scales are available as constructors on `Scale`:

| Constructor                | Intervals (semitones)            |
|----------------------------|----------------------------------|
| `Scale::chromatic()`       | 0 1 2 3 4 5 6 7 8 9 10 11       |
| `Scale::major()`           | 0 2 4 5 7 9 11                   |
| `Scale::natural_minor()`   | 0 2 3 5 7 8 10                   |
| `Scale::harmonic_minor()`  | 0 2 3 5 7 8 11                   |
| `Scale::pentatonic_major()`| 0 2 4 7 9                        |
| `Scale::pentatonic_minor()`| 0 3 5 7 10                       |
| `Scale::blues()`           | 0 3 5 6 7 10                     |
| `Scale::dorian()`          | 0 2 3 5 7 9 10                   |
| `Scale::phrygian()`        | 0 1 3 5 7 8 10                   |
| `Scale::lydian()`          | 0 2 4 6 7 9 11                   |
| `Scale::mixolydian()`      | 0 2 4 5 7 9 10                   |
| `Scale::whole_tone()`      | 0 2 4 6 8 10                     |
| `Scale::diminished()`      | 0 2 3 5 6 8 9 11                 |
| `Scale::augmented()`       | 0 3 4 7 8 11                     |

Custom scales can be created with `Scale::new(intervals, name)`.

## Output reference

Every call to `tick()` returns a `StepOutputs` struct. The table below lists
each field with its type and the hardware jack it models.

| Field              | Type             | Hardware equivalent                        | Description |
|--------------------|------------------|--------------------------------------------|-------------|
| `note`             | `Option<u8>`     | CV OUT (DAC0808 + TL074 output amp)        | Quantized MIDI note number (0--127). |
| `velocity`         | `Option<u8>`     | *(derived from DAC byte)*                  | MIDI velocity (1--127, never 0). |
| `gate`             | `bool`           | PULSE OUT (CD4050 buffer BUFFER2F)         | True when bit 7 of the DAC byte is high. |
| `scale_note`       | `Option<u8>`     | SCALE OUT (Volts expander resistor net)    | Independently quantized MIDI note (may use a different scale). |
| `pulses`           | `[bool; 6]`      | PULSE1--PULSE6 (CD4081 AND gates)          | `pulses[n] = bits[n] AND bits[n+1]` for n in 0..6. |
| `gates`            | `[bool; 8]`      | GATE1--GATE8 (CD4050 buffers)              | Individual gate outputs for shift register bits 0--7. |
| `div2`             | `bool`           | 1/2 clock out (VCV addition)               | Fires every 2 master clocks. |
| `div4`             | `bool`           | 1/4 clock out (VCV addition)               | Fires every 4 master clocks. |
| `noise_cc`         | `u8`             | NOISE OUT (2N3904 transistor + TL074)      | Random CC value (0--127) each step. |
| `register_bits`    | `u16`            | LED display                                | Raw shift register state. Bit 15 = oldest, bit 0 = newest. |
| `length`           | `usize`          | LENGTH rotary readback                     | Active loop length at this step. |
| `write_probability`| `f32`            | WRITE knob readback                        | Active write probability at this step. |

## Hardware-to-MIDI mapping

For the complete mapping between hardware signals and this crate's types:

| Hardware signal          | Hardware component             | MIDI equivalent in this crate       |
|--------------------------|-------------------------------|--------------------------------------|
| CV_OUT (0--5V, 8-bit DAC)| DAC0808 + TL074 output amp    | `StepOutputs::note` (quantized)      |
| PULSE_OUT (bit 7 gate)   | CD4050 buffer BUFFER2F        | `StepOutputs::gate`                  |
| NOISE_OUT (white noise)  | 2N3904 transistor + TL074     | `StepOutputs::noise_cc` (random u8)  |
| LENGTH rotary            | ALPS-SR8V 9-position switch   | `LengthSelector` (same 9 values)     |
| WRITE knob               | TL072 comparator + CD4016     | `WriteKnob::probability` (0.0--1.0)  |
| CV_IN (write mod)        | R22/R23 attenuverter          | `TuringMachine::modulate_write()`    |
| RESET jack               | Async reset to CD4015         | `TuringMachine::reset()`             |
| MOVE jack/button (VCV)   | Not on hardware; VCV addition | `TuringMachine::move_step()`         |
| SCALE out (VCV)          | Volts expander resistor net   | `StepOutputs::scale_note`            |
| PULSE1--PULSE6 (expander)| CD4081 AND gates (TuringBack) | `StepOutputs::pulses[0..5]`          |
| GATE1--GATE8 (expander)  | CD4050 buffers (TuringBack)   | `StepOutputs::gates[0..7]`           |
| 1/2 clock out (VCV)      | Not on hardware; VCV addition | `StepOutputs::div2`                  |
| 1/4 clock out (VCV)      | Not on hardware; VCV addition | `StepOutputs::div4`                  |

## Features

| Feature   | Default | Description |
|-----------|---------|-------------|
| `midi-io` | no      | Enables `MidiTuringMachine` wrapper backed by `midir` for real MIDI port I/O. |
| `serde`   | no      | Enables `Serialize` / `Deserialize` on public types. |

## License

MIT OR Apache-2.0
