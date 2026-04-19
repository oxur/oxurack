# oxurack-rt

Real-time MIDI clock and I/O thread for [oxurack](https://github.com/oxur/oxurack).

This crate handles MIDI clock generation (master mode) and tracking (slave mode with PLL),
MIDI message I/O via midir, and the lock-free queue bridge to the ECS world. It runs on
dedicated OS threads elevated to real-time priority.

## Status

Early development. APIs will change.

## License

MIT OR Apache-2.0
