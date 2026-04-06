# oxurack

Rust crates built with [midir](https://crates.io/crates/midir), inspired by
Eurorack modules for use with hardware and software MIDI devices.

Oxurack reimagines classic analog module designs in the MIDI domain. Instead
of control voltages and audio signals, these crates generate and transform
MIDI note, velocity, gate, and CC streams -- suitable for driving
synthesizers, DAW plugins, or live-coding sessions.

## Architecture

The system is organized in four layers:

```
+------------------------------------------------------------------+
|  Application / REPL / DAW Plugin                                 |
+------------------------------------------------------------------+
|  Rack Orchestration (oxurack)              [ optional ]          |
|  - holds modules, routes cables, drives clock                    |
+------------------------------------------------------------------+
|  Module Crates                             [ standalone ]        |
|  - turingmachine, clock, noise, lfo, sequencer, ...              |
|  - each implements the Module trait                              |
|  - each is independently usable without the rack                 |
+------------------------------------------------------------------+
|  Core (oxurack-core)                                             |
|  - Module trait, Value types, Scale/quantization                 |
|  - BatchGenerator, routing primitives                            |
+------------------------------------------------------------------+
```

Every module crate stands alone -- you can use the Turing Machine without
the rack, the clock without the noise module, etc. The rack is an optional
orchestration layer that wires modules together through a cable routing
system, driving them from a shared clock.

Design documents live in
[docs/design/](docs/design/) and cover the
[system architecture](docs/design/01-draft/0002-oxurack-system-architecture.md),
[module catalog](docs/design/01-draft/0003-initial-module-catalog-and-build-order.md), and
[core infrastructure](docs/design/01-draft/0004-oxurack-rack-module-infrastructure.md).

## Modules

### Available

| Crate | Inspired by | Description |
|-------|-------------|-------------|
| [turingmachine](crates/turingmachine/) | Music Thing Modular Turing Machine Mk2 | Shift-register randomisation and looping applied to MIDI note/velocity/gate/CC streams. 14 built-in scales, clock dividers, pulse and gate expander outputs. |

### Planned -- Timing

| Crate | Inspired by | Description |
|-------|-------------|-------------|
| clock | Various | Master clock with configurable BPM, swing, division (div2/4/8) and multiplication (x2/x3) outputs. |
| euclidean | Euclidean rhythm generators | Bjorklund's algorithm mapped to MIDI triggers and gates. |

### Planned -- Modulation

| Crate | Inspired by | Description |
|-------|-------------|-------------|
| noise | Noise / S&H modules | Random value generation: uniform, Gaussian, Perlin, and Simplex noise algorithms. |
| lfo | LFO modules | Low-frequency oscillator with sine, triangle, square, saw, and random waveforms. Free-running or tempo-synced. |

### Planned -- Transformation

| Crate | Inspired by | Description |
|-------|-------------|-------------|
| quantizer | Pitch quantizers | Standalone scale quantizer for external MIDI streams. |
| range | Attenuverters / scalers | Maps values from one range to another (e.g., noise 0--127 to velocity 60--100). |
| sample-hold | Sample & Hold modules | Captures input value on trigger, holds until next trigger. |

### Planned -- Sequencing

| Crate | Inspired by | Description |
|-------|-------------|-------------|
| sequencer | Classic step sequencers | Step sequencer with per-step note, velocity, gate length, probability, and ratcheting. |
| arpeggiator | Arp modules | MIDI arpeggiator with classic modes (up, down, up-down, random, order) and octave range. |

### Planned -- Output

| Crate | Inspired by | Description |
|-------|-------------|-------------|
| midi-output | Eurorack output modules | Generic MIDI output module: converts value streams to Note On/Off and CC messages. |

### Planned -- Infrastructure

| Crate | Description |
|-------|-------------|
| oxurack-core | Module trait, Value types, Scale/quantization, BatchGenerator, routing primitives. |
| oxurack | Rack orchestration: holds modules, manages cables, drives the tick cycle. |

## Quick start

```rust
use turingmachine::{TuringMachine, Scale};

let mut tm = TuringMachine::with_seed(42);
tm.set_scale(Scale::pentatonic_minor());
tm.set_length(8);
tm.set_write(0.8);

for _ in 0..16 {
    let out = tm.tick();
    if out.gate {
        println!("note={} vel={}", out.note.unwrap(), out.velocity.unwrap());
    }
}
```

See the [turingmachine README](crates/turingmachine/README.md) for the full
API, signal chain diagram, and MIDI I/O examples.

## Workspace layout

```
oxurack/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── oxurack-core/       # Core traits and types (planned)
│   ├── turingmachine/      # MIDI Turing Machine Mk2
│   └── ...                 # Future module crates
└── docs/
    └── design/             # Architecture and module design documents
```

## Features

Each crate may expose optional Cargo features:

| Feature   | Description |
|-----------|-------------|
| `midi-io` | Real-time MIDI port I/O via `midir`. |
| `serde`   | `Serialize` / `Deserialize` on public types. |

## Building

```sh
make build          # debug build
make test           # run all tests
make check          # build + lint + test
make docs           # generate API docs
```

## License

MIT
