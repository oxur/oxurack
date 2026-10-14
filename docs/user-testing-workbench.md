# User testing: Clock, LFO, Scope and saved patches

2026-10-14. For the code landed on `main` through `27ca00f` (README refresh
`1954fad`). Run from the main checkout, not the active implementation worktree.

**You can test these now.** Clock drives the Turing Machine, LFO modulates it,
and Scope captures signals and MIDI. The terminal Scope view appears when a run
ends; CSV lets you inspect the entire retained window or plot it in a spreadsheet.
Interactive play/pause/knob commands are the next slice, not a prerequisite here.
This Clock produces internal rack pulses; these examples do not send MIDI Clock
or START/STOP to synchronize a DAW.

Start with sections 1–3 for an offline tour. Add section 4 to hear the instrument,
then try persistence in section 5. Section 6 is optional. Commands go in the
**same terminal**, in order. Stop on unexpected errors and keep their messages.
No repository fixtures are edited; all captures and experimental patches go into
a fresh temporary directory. Python 3 is used for small CSV summaries and copies.

## 1. Prepare

```bash
cd /Users/oubiwann/lab/oxur/oxurack
git branch --show-current
git rev-parse HEAD
cargo build --locked -p oxurack \
  --example turing-machine-midi --example save-live-patch \
  --example annotate-live-patch --example rewire-live-patch --example rack-session
export OXURACK_TRY_DIR=$(mktemp -d /tmp/oxurack-workbench.XXXXXX)
printf 'Keep these results: %s\n' "$OXURACK_TRY_DIR"
git rev-parse HEAD > "$OXURACK_TRY_DIR/source.txt"
cp crates/oxurack/examples/patches/turing-machine-lfo.ron "$OXURACK_TRY_DIR/original.ron"
```

Expect branch `main` and a successful build. Keep the actual commit in your report
if it is newer than the baseline above. The commands below use the just-built
binary, so Cargo messages do not obscure the Scope view. If you use a custom
`CARGO_TARGET_DIR`, substitute its `debug/examples` directory for `./target/debug/examples`.

## 2. Inspect Scope and the LFO

Capture ten seconds of simulation, without opening a MIDI output:

```bash
./target/debug/examples/turing-machine-midi \
  --patch "$OXURACK_TRY_DIR/original.ron" --samples 10000 \
  --scope-view --scope-rows 8 --scope-capacity 100000 \
  --scope-port clock_main.pulse --scope-port lfo_main.bipolar_out \
  --scope-port tm_main.write_modulation --scope-port tm_main.note \
  --scope-csv "$OXURACK_TRY_DIR/scope.csv" \
  > "$OXURACK_TRY_DIR/original.trace" 2> "$OXURACK_TRY_DIR/scope-view.txt"
printf 'Scope exit: %s\n' "$?"
cat "$OXURACK_TRY_DIR/scope-view.txt"
wc -l "$OXURACK_TRY_DIR/original.trace" "$OXURACK_TRY_DIR/scope.csv"
```

Expect exit **0**, a fast silent run, **70 MIDI trace rows**, and **40,071 CSV
lines** including the header. The terminal view should show:

```text
completed_samples=10000 capacity=100000 retained=40070 overwritten=0
retained_sample_span=0..=9999
retained_counts signal=40000 generated=70 sent=0
```

The recent tail is only eight rows per category, not the entire waveform.
`sent=0` is correct: no output was selected. The LFO runs at **0.5 Hz**, so one
cycle spans **2,000 samples / two seconds**. Inspect a few points:

```bash
python3 - <<'PY'
import csv, os
from pathlib import Path
rows = list(csv.DictReader((Path(os.environ['OXURACK_TRY_DIR']) / 'scope.csv').open()))
for r in rows:
    if r['module'] == 'lfo_main' and int(r['sample']) in (0, 500, 1000, 1500, 2000):
        print('sample', r['sample'], 'LFO', r['value'])
PY
```

Expect approximately **0, +1, 0, −1, 0**. Tiny floating-point differences around
zero are normal. For a visual waveform, open `scope.csv` in your spreadsheet,
filter to `record=signal`, `module=lfo_main`, `port=bipolar_out`, then create an
XY scatter chart with lines: x=`sample`, y=`value`. You should see five sine cycles.
Do not plot the mixed signal/MIDI rows as one series. Filter to
`tm_main.write_modulation` for a separate view of modulation arriving at the TM;
the cable scales the LFO by 0.5 and the rack's sample pipeline can shift timing.

Useful feedback: can you identify the latest values, distinguish signals from
MIDI, and understand what the retained window contains? What would make the view
more useful while experimenting?

## 3. Test Clock tempo and pulse width

Create four variants of the same patch. Only the **Clock module parameters**
change; the patch-level BPM metadata intentionally stays at 120.

```bash
python3 - <<'PY'
import os
from pathlib import Path
root = Path(os.environ['OXURACK_TRY_DIR'])
source = (root / 'original.ron').read_text()
needle = '"bpm": Float(120.0), "ppqn": Int(4), "pulse_width_ms": Int(1)'
assert source.count(needle) == 1, 'Fixture changed; inspect its Clock parameters first'
for name, bpm, width in [('slow', 60, 1), ('normal', 120, 1), ('fast', 240, 1), ('wide', 120, 5)]:
    replacement = f'"bpm": Float({bpm}.0), "ppqn": Int(4), "pulse_width_ms": Int({width})'
    (root / f'{name}.ron').write_text(source.replace(needle, replacement))
PY
for variant in slow normal fast wide; do
  ./target/debug/examples/turing-machine-midi \
    --patch "$OXURACK_TRY_DIR/$variant.ron" --samples 1000 \
    --scope-csv "$OXURACK_TRY_DIR/clock-$variant.csv" --scope-port clock_main.pulse \
    > "$OXURACK_TRY_DIR/clock-$variant.trace" || break
done
python3 - <<'PY'
import csv, os
from pathlib import Path
root = Path(os.environ['OXURACK_TRY_DIR'])
for name in ('slow', 'normal', 'fast', 'wide'):
    rows = [r for r in csv.DictReader((root / f'clock-{name}.csv').open())
            if r['record'] == 'signal' and r['module'] == 'clock_main']
    highs = [int(r['sample']) for r in rows if r['value'] == 'true']
    edges = [t for i, t in enumerate(highs) if i == 0 or t != highs[i-1] + 1]
    print(name, 'high samples=', len(highs), 'rising edges=', edges)
PY
```

Each file contains 1,000 signal samples, plus any MIDI records. Expect:

| Variant | BPM / PPQN | Rising edges in the first second | High samples |
| --- | --- | --- | --- |
| slow | 60 / 4 | 0, 250, 500, 750 | 4 |
| normal | 120 / 4 | 0, 125, 250, 375, 500, 625, 750, 875 | 8 |
| fast | 240 / 4 | 16 edges, alternating 63/62-sample spacing, starting 0, 63, 125, 188 | 16 |
| wide | 120 / 4 | Same rising edges as normal | 40 (five high samples per pulse) |

PPQN means pulses per quarter note; at PPQN 4 these are sixteenth-note triggers.
The Clock starts high at sample zero. The fractional 62.5 ms fast interval is
represented on the 1 ms sample grid. These are **simulation-time observations**,
not a measurement of hardware MIDI jitter. The TM can rest or repeat, so audible
note gaps are not a reliable substitute for inspecting the Clock pulses.

## 4. Listen to tempo changes and check stopping

List outputs and select the exact name of the destination you have routed:

```bash
./target/debug/examples/turing-machine-midi --list-outputs
OXURACK_MIDI_OUTPUT='MIDI Bus 1'
```

Replace `MIDI Bus 1` if needed. Your previous MIDI Bus 1 → PatchWork/Kontakt route
is a useful starting point if it still exists. Enable input/monitoring and choose
a sound on **MIDI channel 1** (the fixture stores this as zero-based channel 0).
No DAW transport sync is required. Use a reasonably short sound so release tails
are easy to distinguish from stuck notes.

Run the three tempos separately, listening to each before starting the next:

```bash
./target/debug/examples/turing-machine-midi \
  --patch "$OXURACK_TRY_DIR/slow.ron" --output-name "$OXURACK_MIDI_OUTPUT" \
  --samples 8000 > "$OXURACK_TRY_DIR/live-slow.trace"
printf 'Slow exit: %s\n' "$?"
```

```bash
./target/debug/examples/turing-machine-midi \
  --patch "$OXURACK_TRY_DIR/normal.ron" --output-name "$OXURACK_MIDI_OUTPUT" \
  --samples 8000 > "$OXURACK_TRY_DIR/live-normal.trace"
printf 'Normal exit: %s\n' "$?"
```

```bash
./target/debug/examples/turing-machine-midi \
  --patch "$OXURACK_TRY_DIR/fast.ron" --output-name "$OXURACK_MIDI_OUTPUT" \
  --samples 8000 > "$OXURACK_TRY_DIR/live-fast.trace"
printf 'Fast exit: %s\n' "$?"
```

Expect successful exits and approximately eight seconds of playback each. The
Clock trigger rate doubles each time; LFO speed remains unchanged, so the music
need not be a simple time-stretched copy. Each run should stop producing notes
at completion. Report unexpected silence, lingering notes or routing errors.

Now capture a live session and press **Ctrl-C once while a note is sounding**:

```bash
./target/debug/examples/turing-machine-midi \
  --patch "$OXURACK_TRY_DIR/original.ron" --output-name "$OXURACK_MIDI_OUTPUT" \
  --samples 60000 --scope-view --scope-capacity 150000 \
  --scope-port clock_main.pulse --scope-port lfo_main.bipolar_out \
  --scope-csv "$OXURACK_TRY_DIR/live-interrupted.csv" \
  > "$OXURACK_TRY_DIR/live-interrupted.trace"
printf 'Interrupted exit: %s\n' "$?"
```

Expect exit **0**, no further retriggers, and Scope/CSV output after stopping.
The completed sample count depends on when you interrupted. This time the Scope
should contain both generated and successfully sent performance events. Those
are software observations; hearing the receiving instrument stop is your part of
the test. Sender fallback cleanup is separate from the performance trace and
Scope history; neither file is a complete receipt of every cleanup send.

## 5. Edit, save and replay

This scripted example changes the running rack after 200 samples, saves it, and
continues to sample 300. It does not send to a device:

```bash
./target/debug/examples/save-live-patch \
  --patch "$OXURACK_TRY_DIR/original.ron" --save "$OXURACK_TRY_DIR/edited.ron" \
  > "$OXURACK_TRY_DIR/edit-summary.txt"
printf 'Save example exit: %s\n' "$?"
cat "$OXURACK_TRY_DIR/edit-summary.txt"
./target/debug/examples/turing-machine-midi \
  --patch "$OXURACK_TRY_DIR/edited.ron" --samples 10000 --scope-view \
  --scope-port lfo_main.bipolar_out \
  --scope-csv "$OXURACK_TRY_DIR/edited.csv" > "$OXURACK_TRY_DIR/edited-1.trace"
./target/debug/examples/turing-machine-midi \
  --patch "$OXURACK_TRY_DIR/edited.ron" --samples 10000 \
  > "$OXURACK_TRY_DIR/edited-2.trace"
cmp "$OXURACK_TRY_DIR/edited-1.trace" "$OXURACK_TRY_DIR/edited-2.trace"
printf 'Fresh replay comparison: %s\n' "$?"
```

Expect successful commands and comparison **0** (`cmp` prints nothing on equality).
The saved LFO frequency is **1 Hz**, so the replay has ten cycles in ten seconds;
the TM's configured write probability remains **0.5**, as in the original fixture. Inspect `edited.ron` in an
editor. It should be readable and reusable. This saves the current configuration,
not the engine cursor: both fresh runs begin at the seed's initial state.

To hear that saved patch, reuse any finite live command from section 4 with
`--patch "$OXURACK_TRY_DIR/edited.ron"`. It need not sound like a continuation of
the first run; a fresh replay deliberately starts over.

## 6. Optional: small windows and other landed operations

Try a deliberately tiny Scope buffer:

```bash
./target/debug/examples/turing-machine-midi \
  --patch "$OXURACK_TRY_DIR/original.ron" --samples 10000 \
  --scope-view --scope-port lfo_main.bipolar_out --scope-capacity 3 \
  > "$OXURACK_TRY_DIR/tiny.trace" 2> "$OXURACK_TRY_DIR/tiny-view.txt"
cat "$OXURACK_TRY_DIR/tiny-view.txt"
cmp "$OXURACK_TRY_DIR/original.trace" "$OXURACK_TRY_DIR/tiny.trace"
printf 'Tiny-buffer trace comparison: %s\n' "$?"
```

Expect **retained=3**, **overwritten=10067**, span **9997..=9999**, and comparison
**0**. The buffer retains only the latest three signal rows here; music generation
is unchanged. This is useful feedback on whether the window/overwrite display
makes sense, not a requirement to inspect thousands of rows.

You can also exercise metadata, cable edits and the complete lifecycle recipe:

```bash
./target/debug/examples/annotate-live-patch \
  --patch "$OXURACK_TRY_DIR/original.ron" --save "$OXURACK_TRY_DIR/annotated.ron"
./target/debug/examples/rewire-live-patch \
  --patch "$OXURACK_TRY_DIR/original.ron" --save "$OXURACK_TRY_DIR/rewired.ron"
./target/debug/examples/rack-session \
  --patch "$OXURACK_TRY_DIR/original.ron" --save "$OXURACK_TRY_DIR/session.ron" \
  --samples 2000 > "$OXURACK_TRY_DIR/session.trace"
./target/debug/examples/turing-machine-midi \
  --patch "$OXURACK_TRY_DIR/session.ron" --samples 2000 \
  > "$OXURACK_TRY_DIR/session-fresh.trace"
cmp "$OXURACK_TRY_DIR/session.trace" "$OXURACK_TRY_DIR/session-fresh.trace"
printf 'Session replay comparison: %s\n' "$?"
```

Expect successful commands and comparison **0**. Inspect the saved files:
`annotated.ron` should retain readable labels/notes/tags; `rewired.ron` contains
the edited LFO→TM cable. `rack-session` exercises handles, removal, queued
commands and lifetime replacement; its phase-labelled cleanup diagnostics on
stderr are expected. These are scripted demonstrations; interactive editing is
what the forthcoming live-controls work adds.

## Report back

Skip whatever does not interest you; mark it untried rather than pass/fail.
Copy the result directory somewhere durable if you want to keep these experiments
beyond temporary-file cleanup. Share this short form and any surprising output:

```text
Source commit / machine:
Results directory:
Scope: counts matched? view understandable? waveform looked right?
Clock: slow/normal/fast/wide pulse summaries matched?
MIDI output / receiver / preset / channel:
Listening: tempo differences, normal stop, anything unexpected?
Ctrl-C: note sounding? stopped cleanly? lingering note? errors?
Saved patch: readable? fresh replay matched? sounded useful?
Optional checks tried and results:
What felt awkward / what would you most like to control next?
```

The command examples and offline expected values were checked against the landed
code. Your listening, routing and usability observations remain to be collected;
this runbook does not claim hardware timing, DAW sync or release acceptance.
