# oxurack-core

ECS world, values, ports, and cables for [oxurack](https://github.com/oxur/oxurack).

This crate defines the foundational types for the oxurack modular synthesizer:
signal values (audio CV, gates, MIDI), port descriptors, cable transforms,
module identifiers, and error types. It forms the bridge between the real-time
audio thread (`oxurack-rt`) and the Bevy ECS world.

## Status

Early development. APIs will change.

## License

MIT OR Apache-2.0
