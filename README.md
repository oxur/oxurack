# oxurack

Rust crates built with [midir](https://crates.io/crates/midir), inspired by
Eurorack modules for use with hardware and software MIDI devices.

Oxurack reimagines classic analog module designs in the MIDI domain. Instead
of control voltages and audio signals, these crates generate and transform
MIDI note, velocity, gate, and CC streams -- suitable for driving
synthesizers, DAW plugins, or live-coding sessions.

## Modules

| Crate | Status | Inspired by | Description |
|-------|--------|-------------|-------------|
| [turingmachine](crates/turingmachine/) | **Available** | Music Thing Modular Turing Machine Mk2 | Shift-register randomisation and looping applied to MIDI note/velocity/gate/CC streams. 14 built-in scales, clock dividers, pulse and gate expander outputs. |
| clock | Planned | Various | Master clock with tempo, swing, and multiple division outputs. |
| sequencer | Planned | Classic step sequencers | Step sequencer with per-step note, velocity, gate length, and probability. |
| noise | Planned | Noise / S&H modules | Random MIDI value generation -- sample-and-hold, random walk, and weighted distributions. |
| quantizer | Planned | Pitch quantizers | Standalone scale quantizer for external MIDI streams (the Turing Machine has its own built in). |
| euclidean | Planned | Euclidean rhythm generators | Euclidean rhythm patterns mapped to MIDI triggers and gates. |
| arpeggiator | Planned | Arp modules | MIDI arpeggiator with classic modes (up, down, up-down, random, order). |

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
├── Cargo.toml          # Workspace root
├── crates/
│   └── turingmachine/  # MIDI Turing Machine Mk2
└── docs/
    └── design/         # Module design documents
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
