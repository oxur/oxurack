# oxurack

[![][build-badge]][build]
[![][tag-badge]][tag]
[![Crates.io](https://img.shields.io/crates/v/oxurack)](https://crates.io/crates/oxurack)
[![Docs.rs](https://img.shields.io/docsrs/oxurack)](https://docs.rs/oxurack)
[![MIT / Apache-2.0](https://img.shields.io/crates/l/oxurack?v=2)](LICENSE)

[![][logo]][logo-large]

*Modular MIDI generation for electronic musicians/live coders who want the feel of a patch-cable session and the reproducibility of a saved file*

oxurack borrows the mental model of Eurorack — small modules, patch cables,
unified signals — and applies it to MIDI. Nothing here generates audio.
Instead, modules produce and transform streams of notes, gates,
velocities, and control values, routed through virtual cables to your
synthesizers, samplers, and DAW.

## Why

Generative and modular music-making tools tend to live on opposite ends of
a spectrum. On one end: live-coding environments that are expressive but
ephemeral — the performance and the patch are the same artifact, and
recalling what you did last Tuesday is a matter of guessing your own
past. On the other end: GUI-based modular racks like VCV Rack that are
beautifully reproducible but expensive to run next to a DAW, and where
capturing a live performance for later editing means bouncing audio
rather than keeping the structure.

oxurack aims for the middle:

- **Patches are files.** A patch is a declarative description of
  modules, parameters, and cable connections. It loads the same way every
  time, ticks deterministically from a known seed, and lives in git
  alongside the rest of your music.
- **The output is MIDI, not audio.** That means every performance — every
  parameter tweak, every patch change — can be recorded directly into a
  DAW as editable notes and controllers. Your generative session becomes
  a starting point, not a print.
- **CPU footprint is small.** oxurack isn't synthesizing sound, so it
  leaves room for your DAW and your soft synths on the same machine —
  the thing that drove the original motivation away from VCV Rack for
  live recording.
- **The ontology is constrained, on purpose.** Eurorack's uniform-voltage,
  single-signal-per-cable design is its strength: one vocabulary, reused
  across thousands of modules. oxurack aims for the same feel in the MIDI
  domain.

## What it lets you do

- Build a generative rhythm section from a Euclidean module, a noise
  source, and a Turing Machine; record the result into Logic Pro (or any
  DAW) as MIDI you can edit, quantize, and remix afterward.
- Put a scale quantizer, sample-and-hold, and LFO in front of a hardware
  synth and drive its pitch with self-similar melodies shaped by the
  instruments' physical controls.
- Save a patch you built during a late-night experiment, come back in a
  week, load it with the same seed, and pick up exactly where you were.
- Map a nanoKONTROL2 (or any MIDI controller) to module parameters and
  play oxurack like an instrument while it plays your synth.
- Sync to your DAW as a clock slave so your generative performance
  records cleanly into an existing session — or run oxurack as the
  master clock when you want it to drive everything else.
- Embed a single module — just the Turing Machine, say — inside a Rust
  project that has nothing to do with oxurack itself. Every module crate
  stands alone.

## Shape of the project

oxurack is a Cargo workspace with four kinds of crate:

- `oxurack-core` — the shared vocabulary: values, ports, cables, the
  tick cycle, and the patch file format.
- `oxurack-rt` — the real-time thread: MIDI I/O and clocking, as
  master or slave.
- `oxurack-mod-*` — one crate per module. Each depends only on
  `oxurack-core` and can be used on its own.
- `oxurack` — the umbrella: REPL, patch CLI, rack assembly.

Modules are Bevy plugins over the core crate. Adding one to a rack looks
like `app.add_plugins(TuringMachineModule)`. The rack loads patches from
RON files, drives them from its clock, and routes values between
modules every tick.

See the documents under [docs/design/](docs/design/index.md) for the full
architecture: the ECS world, the RT thread, the module-authoring
interface, and the build roadmap.

## Current state

oxurack is in early development. The Turing Machine Mk2 port
(`crates/turingmachine/`) is the first working module; the core
infrastructure, RT thread, and additional modules are being built out
against the v1 roadmap. APIs will change until v1.0 is tagged.

If you want to follow along or try the Turing Machine in isolation, its
crate has its own README with examples.

## Lineage

oxurack is the Rust successor to
[underack](https://github.com/ut-proj/underack), an LFE/Erlang project
that explored the same design space. The modular-MIDI stance and the
rough shape of the module catalog carry forward; the architecture is new,
built on Bevy ECS as the runtime substrate.

## License

MIT

[//]: ---Named-Links---

[logo]: assets/images/logo-v1-small.png
[logo-large]: assets/images/logo-v1.png
[build]: https://github.com/oxur/oxurack/actions/workflows/ci.yml
[build-badge]: https://github.com/oxur/oxurack/actions/workflows/ci.yml/badge.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/oxurack.svg
[tag]: https://github.com/oxur/oxurack/tags
