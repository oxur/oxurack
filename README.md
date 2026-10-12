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

Planning lives on the dedicated `planning` branch in `.worktrees/planning`.
Start with [project01-mvp](.worktrees/planning/project01-mvp/project-plan.md)
for the current roadmap and reconstructed status. The historical design and
implementation documents are preserved there with their Git history.
If the worktree is absent, create it with
`git worktree add .worktrees/planning planning` after fetching the branch.

## Current state

oxurack is in early development. The Turing Machine Mk2 port
(`crates/oxurack-mod-turingmachine/`) is the first working module; the core
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

### Run a Turing Machine MIDI patch

The installed `midi_out` sink can stream a deterministic sample-indexed MIDI
byte trace without opening a MIDI device. The default run advances 2,000
1 ms samples; set `--samples 0` for an empty trace or choose another finite
sample count:

```bash
cargo run -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron --samples 2000
```

Each output row is `sample|STATUS DATA1 DATA2`, with uppercase hexadecimal
bytes. The example drains each completed sample before advancing the rack.

The LFO-modulated patch keeps the same seeded Turing Machine and MIDI route,
with a slow sine LFO changing its write probability. Run its sample-indexed
trace offline:

```bash
cargo run -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --samples 10000
```

`Oxurack::parameter` reads a registered getter from the active module instance;
`set_parameter` applies its normal validation and emits one `DirectApi`
`ParameterChanged` message when successful. `snapshot_patch` returns an owned
configuration made from the active modules' current getters, labels, cables and
loaded patch identity. `save_patch` snapshots first and then uses the existing
RON writer. These synchronous calls can run between samples while the rack is
stopped or running. They do not advance time, consume MIDI, clear Scope, or reset
module state. Getter and I/O failures are returned to the caller.

A saved patch is a seeded configuration, not an engine checkpoint: it preserves
the current parameter values and the loaded version, seed and global BPM, but
does not save LFO phase, Turing Machine registers/RNG position, pending MIDI or
Scope records. Loading it in a fresh rack starts the seed from its initial phase.
For the Turing Machine, registered parameter reads and saved patches retain the
configured `write_probability` knob; its engine and same-named step output use
the effective value after CV modulation and clamping. The step output is
published one sample after the step is computed. Parallel cables with identical
endpoints retain their loaded patch-vector order through propagation and saving.
Raw ECS edits to a cable's `CableOrder` must call
`oxurack_core::mark_propagation_order_dirty` before the next sample.
This example edits the running LFO fixture after 200 samples and saves at that
point; it then continues to sample 300:

```bash
cargo run --locked -p oxurack --example save-live-patch -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --save /tmp/oxurack-edited.ron
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch /tmp/oxurack-edited.ron --samples 10000 \
  --scope-view --scope-port lfo_main.bipolar_out
```

The second command runs the saved configuration in a fresh process. Its LFO
phase begins at zero while its frequency is 1 Hz.

Capture selected scalar ports and the runner's generated MIDI events to a
bounded CSV file. Capacity counts mixed records; when full, the oldest rows are
discarded. Signal rows name the selected module and port. MIDI rows distinguish
events generated by the rack from events successfully sent by a live software
sender; `sent` does not mean a synthesizer rendered or recorded the event.

```bash
cargo run -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --samples 10000 --scope-csv /tmp/turing-machine-lfo.csv \
  --scope-port clock_main.pulse \
  --scope-port lfo_main.bipolar_out \
  --scope-port tm_main.write_modulation \
  --scope-port tm_main.note \
  --scope-capacity 100000
```

This run retains 40,000 signal records and 70 generated MIDI events (40,070
records total, plus the CSV header) when the seeded fixture produces its full
trace. The example prints capacity, retained rows, and overwritten rows to
stderr. Leaving out `--scope-port` captures MIDI only. Leaving out
`--scope-capacity` uses 100,000 records. `clear_scope` resets retained rows
and the overwrite count without resetting sample time; loading a patch
successfully disables the old selectors, so configure again for the new patch.

For a terminal view without creating a file, add `--scope-view`. The view is
printed to stderr after the finite run stops (and after live MIDI cleanup). It
shows the patch path, authoritative completed sample count, mixed-record
capacity, retained/overwritten counts, retained sample span, latest value for
each selected signal, and separate recent signal and MIDI tails. Use
`--scope-rows` to choose 1–1000 rows in each tail; the default is 12. The latest
per-selector section is additional to those tails.

```bash
cargo run -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --samples 10000 --scope-view \
  --scope-port clock_main.pulse \
  --scope-port lfo_main.bipolar_out \
  --scope-port tm_main.write_modulation \
  --scope-port tm_main.note
```

The view and CSV can be requested together. The sample-indexed MIDI trace stays
on stdout, the human view goes to stderr, and the CSV keeps the same schema and
retained records as a CSV-only run. With the fixture above and capacity 100,000,
the retained view reports 10,000 completed samples, 40,070 records (40,000
signals and 70 generated events), and no overwritten records. The CSV has
40,071 lines including its header. `--scope-view` alone opens no file.

```bash
cargo run -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --samples 10000 --scope-view --scope-rows 12 \
  --scope-csv /tmp/oxurack-workbench.csv --scope-capacity 100000 \
  --scope-port clock_main.pulse \
  --scope-port lfo_main.bipolar_out \
  --scope-port tm_main.write_modulation \
  --scope-port tm_main.note \
  > /tmp/oxurack-workbench.trace
```

The two tails describe only the records still retained in the one mixed FIFO.
When capacity is reached, old records are evicted; a retained window can begin
mid-sample and can include a generated event without its sent counterpart.
Counts, span and latest values never reconstruct evicted history. A `generated`
MIDI row was drained from the rack by the runner. A `sent` row means the live
software sender accepted that event; neither label means a synthesizer rendered
or recorded audio. Signal values are sampled scalar/Gate/Raw snapshots; a held
MIDI-valued port is not an event history.

For example, selecting only `lfo_main.bipolar_out` with `--scope-capacity 3`
and `--samples 10000` retains the last three signal records (samples 9997–9999)
and evicts the earlier 10,067 mixed records. The completed count remains 10,000
and the MIDI trace remains the same 70 rows. With `--samples 0`, selected ports
are shown as unobserved, both tails say `(none retained)`, and CSV contains its
header only. These are retained-window observations, not a live redraw.

`clear_scope` clears captured records and the overwrite count only. It leaves
the rack's sample clock and module phases in place; stopping and starting also
continues the same rack. A new process loads the seeded patch into a fresh rack
and starts again from its initial phase and sample index. To clear and continue
an already configured rack:

```rust
rack.stop();
rack.clear_scope();
rack.start();
for _ in 0..100 {
    if rack.advance_samples(1)? == 0 {
        break;
    }
    let events: Vec<_> = rack.drain_midi().collect();
    for event in events {
        rack.record_scope_midi(event, oxurack::MidiDelivery::Generated);
    }
}
rack.stop();
// Inspect/export the retained continuation. A fresh rack/process reseeds.
```

This records generated events in Scope after each per-sample drain; live sending
uses the existing runner, which owns its drain and records successful sends.

Scope options are validated before playback: `--scope-port` and
`--scope-capacity` require either `--scope-csv` or `--scope-view`, while
`--scope-rows` requires `--scope-view`. Scope options cannot be combined with
`--list-outputs`. A requested CSV file is opened before the rack starts or a
live output is connected, so selector/configuration/file failures prevent the
run.

The same capture flags work with live output selection. For example, replace the
sample-only command above with a current endpoint name selected from
`--list-outputs`:

```bash
cargo run -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --output-name "MIDI Bus 1" --samples 10000 \
  --scope-csv /tmp/turing-machine-live.csv \
  --scope-port lfo_main.bipolar_out
```

CSV export happens after stop and live cleanup, including when playback fails;
the primary playback error is retained if export also fails. Scope rows do not
change the existing stdout performance trace.

For live playback, choose an exact endpoint name from the current host's list:

```bash
cargo run -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --output-name "MIDI Bus 1" --samples 10000
```

To list the MIDI outputs available to the current host:

```bash
cargo run -p oxurack --example turing-machine-midi -- --list-outputs
```

Choose one listed output by its exact, unique name or its zero-based index to
play the patch live. Refresh the listing before using an index because
available indices can change between runs:

```bash
cargo run -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron \
  --output-index 0 --samples 2000
```

Live playback also ends at the sample limit. Press Ctrl-C to stop after the
current MIDI send and trace row; the example then attempts active-note offs,
All Notes Off, and All Sound Off on channels it used. Cleanup is best effort:
a disconnected endpoint or forced process termination can prevent delivery.
Live samples are paced at 1 ms and do not catch up in bursts after a slow rack
step or MIDI send. The sample-indexed trace records successfully sent patch
messages only; it does not prove that a synthesizer rendered or recorded them.

## License

MIT

[//]: ---Named-Links---

[logo]: assets/images/logo-v1-small.png
[logo-large]: assets/images/logo-v1.png
[build]: https://github.com/oxur/oxurack/actions/workflows/ci.yml
[build-badge]: https://github.com/oxur/oxurack/actions/workflows/ci.yml/badge.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/oxurack.svg
[tag]: https://github.com/oxur/oxurack/tags
