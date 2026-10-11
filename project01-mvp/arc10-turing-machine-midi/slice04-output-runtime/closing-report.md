# Slice04 output runtime — CC closing report

## Intake and authority

Assignment: `arc10-turing-machine-midi/slice04-output-runtime/cc-prompt.md`, SHA-256 `4e5be5ac8afcc1f60d01e553d90b290f45453b27a71b9c75c25d9b109d959f0e`. Source `S=/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation` is clean on `codex/oxurack-module-behavior-installation` at accepted Slice03 source `2942c594a445b5de3ef8f131cdedaa6095aa1529`. Planning `P=/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, project root `project01-mvp`, was clean at issue HEAD `0347eeeac2bd0552976578e1e72135e476c282bd`; assigned planning predecessor is `f19e2f1` (Slice03 CRC acceptance). Host is `lappy.local`.

Applied framework 2.6.0 and Rust guidelines 2.0.0 from `/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/`. Full reads: active prompt, Slice04 plan/ledger, directive08, Arc10 design v1.2 §§1–5, Arc10 plan v1.7 named sections, current project plan v3.19 direction/handoff, Slice03 CRC review, source example/fixture/integration test/umbrella manifest, and the assigned facade and buffer symbols. Registry source was the pinned Cargo cache: midir 0.11.0 `MidiOutput`/port/connection and error definitions; ctrlc 3.5.1 handler docs and functions; clap/clap_builder 4.6.6 derive attributes/types and required/conflict methods. Rust 2.0.0 entrypoint and AP chapter were read in the preceding Slice03 turn; the assigned API/error/type/docs/CLI sections, including CLI-43, and framework intake/Proportionate Proof guidance were applied. No required input was intentionally truncated; an initial combined parent-plan read was repeated as smaller complete reads after tool output truncation.

Cargo.lock before edits, for the seven required packages:

| Package | Version | Locked dependencies |
| --- | --- | --- |
| `midir` | 0.11.0 | alsa, cc, coremidi, jni, jni-min-helper, js-sys, libc, log, ndk-context, parking_lot, wasm-bindgen, web-sys, windows |
| `ctrlc` | 3.5.1 | dispatch2, nix, windows-sys 0.61.2 |
| `clap` | 4.6.6 | clap_builder, clap_derive |
| `clap_builder` | 4.6.6 | anstream, anstyle, clap_lex, strsim |
| `clap_derive` | 4.6.4 | heck, proc-macro2, quote, syn 3.0.7 |
| `clap_lex` | 1.1.1 | none |
| `oxurack` | 0.1.0 | bevy_app, bevy_ecs, bevy_time, clap, oxurack-core, oxurack-graph, oxurack-mod-clock, oxurack-mod-midiout, oxurack-mod-turingmachine, pretty_assertions |

Source-cited readback: the existing example creates a stopped fresh rack, installs Clock/TM/MidiOut, loads the RON patch, switches to Manual, advances one complete sample, drains output, and writes `sample|bytes`; its closure always stops on error. The facade can return a tick error after a finalized sample, so any queued prefix remaining at that point must be deliberately discarded before cleanup. The rack sink tracks logical voice state, not endpoint-delivered notes; only a successful connection send may update the runner's active-note table. Mark a channel used before each send attempt because a send error does not prove that no bytes reached the endpoint. Record successful note ownership immediately after send and before the fallible trace write. On cancellation, do not interrupt a sample already advanced or an OS send already in progress; finish that call, record success, then drop the remaining drain and prevent further sends/samples. Stop the rack, discard the remaining drain, then make all cleanup attempts. The first sample is due immediately; on late dispatch or completion beyond the next 1 ms deadline, rebase to finish plus 1 ms instead of catching up. This is monotonic pacing around manual samples, with no jitter or blocking-send timeout claim. O-05 requires Duncan's observation on a named software or hardware endpoint; listing, a recording sender, and an offline trace are not that witness.

Authority is directive08 and design v1.2 §§4–5. The accepted endpoint implementation is in the existing example, with private helper/tests under `examples/turing_machine_midi/`; only umbrella dev dependencies, required lock edges, root README, and directly related umbrella tests are otherwise in scope. No public API, RT service, worker, transport output, reconnection or scheduler change is planned. O-01–O-05 stay open until independent review; CC will propose evidence only. An unavailable endpoint is a specific O-05 gap for CRC/CDC, not a mock pass.

## Result

## Source candidate

Source commits: `829feb99279b11b3f46a11e513aba577eacd6fd1` (`feat(example): add selected MIDI playback`) and follow-up `65522f116a12a9ac96d35cfe25ed9c184f5bddcb` (`test(example): cover zero-sample playback`). Final source head is `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`; the first source commit follows the accepted Slice03 source directly. The seven authorized paths across those two source commits are:

- `Cargo.lock`
- `README.md`
- `crates/oxurack/Cargo.toml`
- `crates/oxurack/examples/turing-machine-midi.rs`
- `crates/oxurack/examples/turing_machine_midi/playback.rs`
- `crates/oxurack/examples/turing_machine_midi/oracle.rs`
- `crates/oxurack/tests/midi_output.rs`

The example now has three modes: the unchanged unpaced offline trace (default 2,000 samples; zero is valid), host output listing, and live playback selected by one exact unique name or a zero-based index from one current port snapshot. Live setup validates a fresh stopped rack, registers one flag-only Ctrl-C handler, enumerates and opens the selected endpoint, and only then starts the rack. A private same-thread sender session tracks successful sounding notes and attempted channels, sends ordered best-effort cleanup after stopping/discarding, aggregates cleanup errors with the primary cause, and flushes trace output after cleanup. Its 1 ms `Instant` pacing rebases after late work rather than catching up. The independent seeded Turing Machine byte oracle was extracted and reused by the pre-existing integration test and the runner test. No production module/core/RT/MIDI-wire code or literal fixture changed.

The Cargo.lock delta adds only umbrella dev-dependency edges for the already locked `ctrlc 3.5.1` and `midir 0.11.0`; the seven-package lock query was repeated and no versions changed. The implementation avoids another direct helper dependency by testing against copied fields of an actual loaded `MidiOutputEvent`.

## Validation evidence

All commands below ran from S after the final source edits unless marked as a host retry:

- `cargo test --locked -p oxurack --example turing-machine-midi`: 11 passed.
- `cargo test --locked --no-default-features -p oxurack --example turing-machine-midi`: 11 passed.
- `cargo test --locked -p oxurack --test midi_output`: 2 passed.
- `cargo test --locked --no-default-features -p oxurack --test midi_output`: 2 passed.
- `make test`: passed on the host; the final RT library result was 150 passed, 0 failed, 3 ignored. The initial sandbox run failed four existing CoreMIDI initialization tests (`test_list_output_ports`, `test_open_nonexistent_port_returns_error`, `test_open_input_nonexistent_returns_error`, `test_list_input_ports`) with “MIDI support could not be initialized”; 146 tests passed and the same 3 were ignored. The exact full gate was retried on the host and passed. The 3 ignores are existing RT timing/scheduling tests.
- `make lint`: passed, including workspace Clippy and rustfmt.
- `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' make docs`: passed.
- `cargo build --locked -p oxurack --example turing-machine-midi`: passed.
- `git diff --check`: passed before the source commit.

Built-executable controls:

- Two fresh 2,000-sample fixture traces compared byte-for-byte and each matched the accepted Slice03 SHA-256 `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`; the output has 13 rows.
- `--samples 0` produced an empty stdout file.
- Missing patch returned nonzero with no trace rows (`patch error: I/O error: No such file or directory`).
- `--patch ... --output-index 999` first failed in the restricted sandbox at MIDI initialization; the host retry exited nonzero with `MIDI output index 999 is out of range; run --list-outputs for a fresh index` and no stdout rows.
- Host `--list-outputs` on the final built executable succeeded and printed exactly `0|MIDI Bus 1` and `1|nanoKEY2 CTRL`. The sandbox listing attempt itself failed at MIDI initialization, so this host result is the relevant endpoint inventory.
- Inline CLI checks cover listing without patch, required patch, list/selector conflicts, selector mutual exclusion, negative index, empty name and valid zero samples. Pure selection checks cover exact `AB`, ambiguous duplicate `A`, nonmatching substring `B`, duplicate index 2 and out-of-range index 3.
- Runner tests use the actual loaded fast fixture and same `play` path: 24-sample performance trace compared with the independent oracle, zero-sample finish with no sends, note-off retry and successful-off ownership, failed first note-on/channel cleanup, ascending multi-channel cleanup despite failure, terminal/idempotent finish, primary plus cleanup error, cancellation during a successful send, fresh-session replay, and on-time/late deadline values. Cleanup bytes never enter the trace.

## Five-row proposed disposition

| ID | CC proposed status | Evidence and remaining qualification |
| --- | --- | --- |
| O-01 | Proposed done | CLI parser and selector tests pass. Actual missing-patch and out-of-range index controls fail nonzero without trace; host listing returned the two exact names above. Same output object/snapshot is inspected in source through connect. |
| O-02 | Proposed done | Actual 24-sample loaded fixture runner trace equals the independent oracle; rack completes 24 samples, drains empty, and is stopped. Deadline checks establish 1 ms due-time progression and late rebase. Native endpoint musical behavior remains O-05. |
| O-03 | Proposed done | Send-ledger tests and same-runner fault/cancel paths pass; every cleanup attempt is recorded, ordered, and best effort. Primary send cause remains in the error chain and all cleanup failures are reported. |
| O-04 | Proposed done | Compatibility hash, CLI controls, targeted feature matrices, full host test gate, lint, strict Clippy/docs, build and diff check pass. Preserve the initial sandbox CoreMIDI failures and host retry above. |
| O-05 | Open — operator witness | Actual inventory exists, but no endpoint was selected or connected during this implementation pass and there is no Duncan musical/routing/stop observation. This is not an endpoint-unavailable claim and no recording-sender test substitutes for it. |

Re-entry command for Duncan to select and observe a listed destination (the command below uses the first current name; refresh the listing immediately before using an index):

```bash
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron \
  --output-name 'MIDI Bus 1' --samples 5000
```

The live command prints successfully sent performance rows to stdout and connection/failure context to stderr. Record the selected software/hardware receiver and routing, audible musical behavior, interrupt/stop outcome, stuck-note observation, and compare the performance rows with the offline command at the same sample count. A fresh listing and the alternate exact name `nanoKEY2 CTRL` are available; no destination was guessed or opened here.

Applied Rust choices followed the intake table: `Result` and contextual source-preserving send errors, private owned session state, typed clap fields and exact selection, a flag-only process handler, and documented cleanup/witness limitations. No lint suppression or public API was added. No broader arc bubble-up was found; O-05 remains the stated operator gate. No separate durable test artifact was created; this report is the evidence record. Planning worktree was clean at issue HEAD except for this pre-edit-created report; the evidence update is authorized only in this slice report/ledger scope. CRC independently accepts; this proposal does not advance Arc10 or Arc11.

## Operator witness addendum — 2026-10-11 (compiled by CRC)

This dated addendum preserves CC's original report above and records the
subsequent Operator result required by design§5; it is not CC's implementation
attestation. Source remains `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`, fixture
`crates/oxurack/examples/patches/turing-machine-midi.ron`, master seed42.

Duncan refreshed the listing and explicitly selected `MIDI Bus 1`. Receiver:
Blue Cat's PatchWork hosting Kontakt8, Electric Storm Deluxe (Melody),
Dreamscapes (Melody)/36 Dreamscapes, Drive To Space sound preset; Bloom/Infinity
in the post chain. Duncan confirms PatchWork/Kontakt omni receive. The command
was the built example with `--patch` above and `--output-name 'MIDI Bus 1'`,
using `--samples 5000` for finite and restarted runs, and `--samples 60000` for
the interrupted run. Each live run printed its selected endpoint and returned0.

Duncan reports “it sounded amazing” and “everything played out exactly as
expected” after normal completion, Ctrl-C and fresh-session runs. CRC reads this
as the requested audible/stop/restart confirmation, not hardware timing or a
claim about forced termination. Musical observation is Operator-witnessed.

CRC independently read and preserved all six trace files, reproduced both fresh
offline references and confirmed finite/restarted5,000 traces are identical
34-row streams, hash `918f4883252988093f737571cbb862aa29a33b737d0db026c98a7d7fea652d17`.
Interrupted output is the exact four-row60,000-reference prefix. Screenshot
and traces live in `artifacts/operator-witness-2026-10-11/`;
[CRC verification](crc-verification.md) records hashes and evidence distinctions.

Final child disposition: O-01–O-05 done; zero open/deferred/no-op. Slice04
closed by CRC after independent software review and explicit Operator witness.
No source changes during witnessing. Arc10's four children are ready for
composition assembly and CDC review. Hardware timing, release coverage/soak,
live persistence/controls and Arc11 observation work retain their planned owners.
