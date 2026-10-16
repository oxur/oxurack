# User test: edit and observe a live rack session

This run checks live session commands, bounded ordered observation, and MIDI
trace ownership without opening a MIDI device. Use a terminal in the repository
root. The fixture is deterministic; musical listening and external MIDI routing
are separate operator checks.

```bash
cargo run --locked -p oxurack --example rack-live -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --scope-port lfo_main.bipolar_out \
  > /tmp/oxurack-session-midi.txt 2> /tmp/oxurack-session-status.txt
```

At the prompt, enter these lines one at a time:

```text
watch on
play
set clock_main bpm 90
label set rack tags ["demo"]
find any ["demo"]
cables
pause
load crates/oxurack/tests/fixtures/session-observation.ron
status
scope-select a_lfo.bipolar_out
label get rack display_name
label set rack notes ["added", "second note"]
unpatch z_clock pulse a_lfo sync_in 0
patch z_clock pulse a_lfo sync_in
watch off
watch on
unload
status
play
load /definitely/missing/session-observation.ron
load crates/oxurack/tests/fixtures/session-observation.ron
load crates/oxurack/examples/patches/turing-machine-lfo.ron
play
pause
watch off
quit
```

Expected behavior:

- Replies, watch rows, and Scope views appear on stderr. The MIDI trace file on
  stdout contains only `sample|STATUS DATA1 DATA2` rows and should be nonempty
  after the first `play` on the musical fixture.
- Watch starts at `Now`, writes at most 32 retained rows during one owner turn,
  and reports sequence, batch, final-in-batch marker, lifetime, sample, source,
  and owned change. `watch off` drops its cursor; another `watch on` starts at
  the then-current end.
- `find` uses exact tags; after setting the rack tag it returns the rack for
  `demo`. `cables` lists stable IDs and current occurrences.
- `restart` reports a new lifetime at sample zero while retaining the exact
  loaded file origin. Loading the observation fixture reports a MIDI-only Scope
  fallback because the remembered `lfo_main` selector is absent; the explicit
  `scope-select` then captures `a_lfo`. Failed load after unload keeps the rack
  empty, and the later musical reload starts a fresh lifetime. The same MIDI
  sender session stays connected through these operations.
- `quit` performs stop/reset and drains cleanup before returning.

The prompt and input reader remain ordinary line-oriented stdin. This test does
not claim Tab completion, persistent note history, history editing, or terminal
completion behavior; those belong to later Arc07 slices. A short synchronous
file write may delay the next sample; the owner admits at most one due sample
after a command and discards accumulated wall-clock lateness.

Report back with the operating system, terminal, commands used, whether the
expected stdout/stderr split held, any unexpected observation gap or command
error, whether sample/lifetime counters matched, and whether you separately
tested physical MIDI routing or listening. Do not include private patch contents
unless needed to reproduce a problem.
