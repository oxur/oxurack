# Slice04 operator playback runbook

2026-10-10 — reviewed source `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`.
This supplies the remaining O-05 witness. Software checks O-01–O-04 are accepted;
the observations below have not yet been performed or accepted.

Run the command blocks in order in the **same terminal**. Allow roughly ten
minutes, including routing. These commands build/run the assigned implementation
worktree; the primary checkout on `main` does not contain this implementation.

## 1. Prepare and choose the destination

```bash
cd /Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation
git rev-parse HEAD
cargo build --locked -p oxurack --example turing-machine-midi
OXURACK_WITNESS_DIR=$(mktemp -d /tmp/oxurack-midi-witness.XXXXXX)
printf 'Witness files: %s\n' "$OXURACK_WITNESS_DIR"
./target/debug/examples/turing-machine-midi --list-outputs
```

Expected source hash: the one above. If it differs, keep the actual hash and tell
CRC before treating this as acceptance of the reviewed candidate. The build must
succeed. The last verified listing was:

```text
0|MIDI Bus 1
1|nanoKEY2 CTRL
```

Use your **fresh listing**, because ports can change. In your receiving DAW or
synth, select the corresponding MIDI input, enable monitoring/live input, and
load a sound-producing instrument listening on **MIDI channel 1**, or all channels.
The fixture's zero-based channel value is 0, which is musician-facing channel 1.
If using a virtual bus, route its receiving side to that instrument. An endpoint
name alone does not establish that a receiver or audible sound exists.

Set the exact name of the output you have routed. This example selects MIDI Bus 1;
replace the quoted name if you chose another listed destination:

```bash
OXURACK_MIDI_OUTPUT='MIDI Bus 1'
```

Keep this instrument's input isolated from other MIDI senders during the witness,
so notes and stop behavior can be attributed to this run. Note the receiver name,
preset, MIDI input/channel and routing. No DAW transport synchronization is needed;
this example sends notes, not MIDI clock or transport.

## 2. Establish the offline baseline

```bash
./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron \
  --samples 5000 > "$OXURACK_WITNESS_DIR/offline-5000.trace"
printf 'Offline exit: %s\n' "$?"
wc -l "$OXURACK_WITNESS_DIR/offline-5000.trace"
shasum -a 256 "$OXURACK_WITNESS_DIR/offline-5000.trace"
head -n 4 "$OXURACK_WITNESS_DIR/offline-5000.trace"
```

Expected: exit **0**, **34 rows**, SHA-256
`918f4883252988093f737571cbb862aa29a33b737d0db026c98a7d7fea652d17`.
First four rows:

```text
251|90 3E 15
376|80 3E 00
626|90 40 2C
751|80 40 00
```

This is a fast offline calculation: it should not send MIDI or make sound.
The unchanged fixture has master seed42, Clock120 BPM/PPQN4 and major-pentatonic
TM settings. Its 5,000 samples represent five seconds of logical rack time;
live wall time can be longer because late scheduling slows playback.

## 3. Listen through normal completion

```bash
./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron \
  --output-name "$OXURACK_MIDI_OUTPUT" --samples 5000 \
  > "$OXURACK_WITNESS_DIR/live-5000.trace"
printf 'Live exit: %s\n' "$?"
cmp "$OXURACK_WITNESS_DIR/offline-5000.trace" "$OXURACK_WITNESS_DIR/live-5000.trace"
printf 'Trace comparison exit: %s\n' "$?"
```

Expected: stderr identifies the selected connection; the receiving instrument
plays a sequence, the process finishes by itself, and both exits print **0**.
`cmp` prints nothing when the files match. After completion, listen for a stuck
note or continued retriggering. Record what actually happened, including silence,
unexpected sound, errors or a lingering note. A byte match alone is not the witness.

## 4. Interrupt while playing

First make a longer offline reference, then start the matching live run:

```bash
./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron \
  --samples 60000 > "$OXURACK_WITNESS_DIR/offline-60000.trace"
printf 'Long offline exit: %s\n' "$?"
./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron \
  --output-name "$OXURACK_MIDI_OUTPUT" --samples 60000 \
  > "$OXURACK_WITNESS_DIR/interrupted.trace"
printf 'Interrupted exit: %s\n' "$?"
```

While that second command is playing, press **Ctrl-C once**, preferably while a
note is sounding. Expect a normal exit **0**, no more new notes, and no stuck note
after cleanup. Record whether a note was sounding when you interrupted. If you
missed the audible portion, repeat this live command for a useful stop observation.

Compare its successful performance rows with the beginning of the longer reference:

```bash
OXURACK_INTERRUPTED_ROWS=$(awk 'END { print NR }' "$OXURACK_WITNESS_DIR/interrupted.trace")
printf 'Interrupted rows: %s\n' "$OXURACK_INTERRUPTED_ROWS"
head -n "$OXURACK_INTERRUPTED_ROWS" "$OXURACK_WITNESS_DIR/offline-60000.trace" \
  > "$OXURACK_WITNESS_DIR/expected-interrupted-prefix.trace"
cmp "$OXURACK_WITNESS_DIR/expected-interrupted-prefix.trace" "$OXURACK_WITNESS_DIR/interrupted.trace"
printf 'Interrupted prefix comparison exit: %s\n' "$?"
```

Expected: a nonempty, shorter performance trace and comparison exit **0**.
Cleanup note-offs/CC123/CC120 are sent separately and do not appear in these trace
files. The sender's software tests establish those cleanup attempts; your receiver
observation establishes whether playback stopped in the actual routing.

## 5. Start a fresh session

```bash
./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron \
  --output-name "$OXURACK_MIDI_OUTPUT" --samples 5000 \
  > "$OXURACK_WITNESS_DIR/restarted-5000.trace"
printf 'Restarted exit: %s\n' "$?"
cmp "$OXURACK_WITNESS_DIR/offline-5000.trace" "$OXURACK_WITNESS_DIR/restarted-5000.trace"
printf 'Restart comparison exit: %s\n' "$?"
```

Expected: the sequence starts afresh, not at the interrupted position, and both
exits are **0**. Listen for normal completion and any lingering note again.

## Report back

Copy this short form into our chat, preserving failures rather than interpreting
them as a pass. Keep the printed witness-directory path and trace files until review.

```text
Source hash:
Current output listing:
Selected output:
Receiver / preset / input channel / routing:
Offline exit / row count / hash:
Finite live exit / comparison exit / musical observation:
Normal completion: stopped? lingering note?
Ctrl-C: note sounding? exit / prefix comparison / stopped? lingering note?
Fresh session: exit / comparison / began afresh? normal stop?
Any stderr errors or unexpected behavior:
Witness files directory:
```

If connection succeeds but you hear nothing, report it and inspect receiver
routing/channel/monitoring before retrying. If listing, connection or send fails,
retain the diagnostic; stop and tell CRC rather than switching to an unrecorded
destination. Forced termination, disconnected-device silence, hardware jitter and
broader musical acceptance are not established by this witness. CRC records the
result in the existing slice close/review documents; this runbook does not close O-05.
