# Slice03 MIDI output — CC closing report

## Intake and authority

Assignment: `arc10-turing-machine-midi/slice03-midi-output/cc-prompt.md` (SHA-256 `20dff54f31beaecefefca5745b7cad03b516d763b0170e5bc0ddd62b3652a35d`). Source `S` is clean on `codex/oxurack-module-behavior-installation` at accepted predecessor `b2ffb16d342d9a6cc800bab2713813d9646fb505`; planning `P` is clean on `planning` at issue HEAD `6c3b2d2cf2658a6cad5297eabcdc2c288131325f`. Host: `lappy.local`. Framework 2.6.0 and Rust guidelines 2.0.0 resolve from `/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/`.

Required reading loaded: this prompt, Slice03 plan and ledger in full; directive08 in full; Arc10 design v1.2 §§1–3/5; project-plan v3.17 current direction and current handoff; Arc10 plan v1.5 Capability, Slice sequence, Composition and Delivery; source APIs enumerated below; framework intake/Expedited Mode and validation-gates guidance; Rust AP/API/error/type/docs/CLI guidance named by the prompt. No required input was truncated; no conditional implementation dependency is currently unresolved. Historical prompt packets/logs and Arc10 design §4 were reference-only.

Source readback: `Oxurack::advance_samples` checks manual mode and running, then advances both Bevy clocks and runs `FixedUpdate`; completion is currently counted by loop iterations. Core latches running in `begin_sample_system` before Produce and finalizes only after Observe, while all five phase sets share that latch. TM publishes a coherent pending snapshot in Produce; propagation and input merge precede Consume. `MidiWire` constructors mask status/data, so sink validation must precede construction. The facade owns the App and offers no public mutable World; a borrowed resource `Drain` is the narrow integration boundary. Patch application validates before spawning, but is not transactional after a later spawn failure.

Contract readback: M-01/M-02 require a strict static seven-input/four-parameter sink whose valid state is read through getters, whose per-sample transitions preserve lexical instance order and emit literal off/on/CC sequences, and whose inputs are validated before mutating that sink. M-03 requires bounded preallocated storage, admission before the existing sample latch and before either facade clock advances, terminal retained faults, and completion counts derived from finalized `SampleTime`; an error discovered in Consume still finalizes that latched sample. The easily missed constraints are that `step` is level-valued per sample (consecutive highs are two events), idle samples ignore unused scalar ports, and a returned `Drain` borrow prevents advancing until dropped; callers must fully consume it. M-04 requires the actual installed seeded RON composition and two fresh executions checked against a separately configured standalone engine plus independently assembled MIDI expectations. M-05 stays proposed-done only: source commit precedes evidence commit, five ledger rows remain open for CRC, and no endpoint playback is claimed.

Implementation inputs inspected at `b2ffb16`: `crates/oxurack/src/lib.rs` (constructor/install/load/start/stop/advance/into_app); core `lib.rs::build_core_plugin`, `tick.rs` phase/latch/SampleTime, `error.rs::TickError`; module spawn/hook/schema, port CurrentValue/spawn, patch validation/application and parameter registry/value; TM and Clock rack schemas/hooks; MidiWire constructors/bytes; existing umbrella TM fixture, runner and standalone oracle. Manifest baseline has workspace `crates/*`, Bevy 0.18, Clock/TM as dev dependencies; no clap package is currently locked. `CLAUDE.md` points to stale Rust skill paths and an incorrect docs target; Make gates are `test`, `lint`, and `docs`.

## Result

### Delivery and proposed disposition

Source commit: `2942c594a445b5de3ef8f131cdedaa6095aa1529` (`feat(midiout): add installed bounded MIDI output`) on `codex/oxurack-module-behavior-installation`. The source worktree is clean at that commit. This is CC proposed-done evidence only; all five ledger rows remain **open** for CRC review.

| Row | CC disposition | Evidence |
| --- | --- | --- |
| M-01 | Proposed done | `cargo test --locked -p oxurack-mod-midiout` (4 passed); `cargo test --locked -p oxurack --test midi_output` (2 passed); strict seven-port/four-parameter schema, getter, invalid-config-before-entity, channel/route boundary and duplicate-channel assertions. |
| M-02 | Proposed done | Module tests cover held/idle values, per-sample high steps, retrigger, note release, invalid-velocity fallback 64, zero velocity, independent CC validity, Raw 0/127 and Raw 128 rejection, lexical sink order and preservation of an earlier sink prefix. |
| M-03 | Proposed done | Facade tests cover no-drain admission before either clock, request-relative completed count, terminal error retention, no-sink grouped advancement, Consume fault finalization, ordinary stop/drain/resume and idempotent reinstall. Update and FixedUpdate tests cover the actual guard before the existing sample latch. |
| M-04 | Proposed done | Installed seeded Clock→TM→MIDI RON integration is compared against an independently seeded TM engine and literal MIDI expectations in default and no-default feature runs. The built example produced 13 ordered trace rows; two fresh runs were byte-identical. |
| M-05 | Proposed done | Exact source diff and allowed-path inventory, current intake/readback, gate receipts, decisions, limitations and bubble-up are recorded here. `git diff --check`, lint, strict Clippy and warning-free rustdoc passed. |

### Source paths

The source commit contains exactly these 14 paths:

```text
CLAUDE.md
Cargo.lock
README.md
crates/oxurack-core/src/error.rs
crates/oxurack-core/src/lib.rs
crates/oxurack-core/src/tick.rs
crates/oxurack-mod-midiout/Cargo.toml
crates/oxurack-mod-midiout/README.md
crates/oxurack-mod-midiout/src/lib.rs
crates/oxurack/Cargo.toml
crates/oxurack/examples/patches/turing-machine-midi.ron
crates/oxurack/examples/turing-machine-midi.rs
crates/oxurack/src/lib.rs
crates/oxurack/tests/midi_output.rs
```

### Validation receipts

All required gates were run from `S` against the candidate. Final results:

| Command | Result |
| --- | --- |
| `cargo test --locked -p oxurack-mod-midiout` | Pass: 4 passed, 0 failed. |
| `cargo test --locked -p oxurack --test midi_output` | Pass: 2 passed, 0 failed. |
| `cargo test --locked -p oxurack --no-default-features --test midi_output` | Pass: 2 passed, 0 failed. |
| `cargo test --locked -p oxurack-mod-clock` | Pass: 7 unit tests and 1 doctest. |
| `cargo test --locked -p oxurack-mod-turingmachine` | Pass: 112 passed, 0 failed. |
| `cargo tree --locked -p oxurack-mod-turingmachine --edges normal` | Pass; normal dependencies remain `rand 0.10.1` and `thiserror 2.0.18`. |
| `make test` | Host retry passed; RT crate reports 150 passed, 0 failed, 3 ignored. Workspace/doc tests passed. |
| `make lint` | Pass: Clippy and format checks. |
| `cargo clippy --locked -p oxurack-mod-midiout -p oxurack-core -p oxurack --all-features --all-targets -- -D warnings` | Pass. |
| `RUSTDOCFLAGS='-D warnings' make docs` | Pass; output generated at `target/doc/oxurack`. |
| `git diff --check` | Pass. |
| `cargo build --locked -p oxurack --example turing-machine-midi` | Pass. Two fresh 2000-sample runs compared byte-for-byte; each emitted 13 rows, beginning `251|90 3E 15`, `376|80 3E 00`, `626|90 40 2C`, `751|80 40 00`. |
| Example negative/zero controls | `--samples 0` produced empty stdout. Missing and malformed patch each exited 1 with stderr and no stdout. |

The first restricted `make test` run failed only in the four known RT CoreMIDI initialization cases: `test_open_input_nonexistent_returns_error`, `test_list_output_ports`, `test_open_nonexistent_port_returns_error`, and `test_list_input_ports`. The host retry passed those cases. The three existing ignored RT tests were `priority::tests::test_priority_elevation_effects_on_scheduler`, `thread::tests::test_timing_loop_jitter`, and `thread::tests::test_timing_loop_no_drift`; they remained ignored. During implementation, `make lint` identified a `drop_non_drop` borrow-release pattern; strict Clippy identified two `clone_on_copy` calls and a collapsible test conditional. These were corrected without suppressions, and the final gates above passed. One intermediate literal transition assertion also caught a stale expected held CC value; the corrected assertion passed.

The lockfile now resolves the example's Clap derive stack, including `clap 4.6.6`, `clap_builder 4.6.6`, and `clap_derive 4.6.4`; no unrelated dependency upgrade was made. The built CLI stays headless: it reads a patch, advances the facade, streams validated bytes, writes errors to stderr, and stops on every fallible exit path. No endpoint, device, wall-clock timing or sent-note cleanup claim was tested or made.

### Applied guidance and limits

- AP-23: an owned bounded `Vec` with a 16-sink maximum and synchronous borrowed drain; no real-time or lock-free claim.
- API-23/EH-01 and AP-25/AP-33: concrete cloneable tick faults, first-fault retention, explicit completed-sample accounting and visible facade inspection.
- API-39: decode signed integers and validate parameter/data ranges before narrowing, wire construction, channel reservation or entity creation.
- TD-05: private configuration, active-note and buffer state; only the event record is public data.
- DC-05: public fallible admission and advance docs describe errors, partial completion and terminal recovery.
- CLI-05: clap derive parsing, stdout trace only, stderr failures and nonzero `ExitCode`; no custom parser or async runtime.

Behavior stays within Slice03: one installed output sink, bounded queued messages, explicit drain, and a seeded headless trace. MIDI endpoint I/O, pacing, cleanup of notes actually sent, and physical playback remain Slice04 work. The facade conservatively requires draining before starting another sample. The guard prevents the next sample in externally owned Live Apps but makes no promise to stop their wall clock. A fault in a later lexical sink retains the earlier prefix; there is no whole-sample rollback.

### Bubble-up to arc

Delivered the planned capability: `Oxurack` can install/load `midi_out`, produce bounded and validated channel messages from real Clock→Turing Machine cables, drain them with sample indices, and run the seeded example reproducibly. The only new runtime module dependency is the local `oxurack-mod-midiout` crate; Clap is an example-only dev dependency. The Turing Machine's normal dependency tree and accepted Clock/TM production behavior remain unchanged. No specified Slice03 scope was omitted or expanded. CRC must independently review and accept; this report does not advance Arc10 or Slice04.

Evidence artifact inventory: this `closing-report.md` and proposed-done notes in `ledger.md`; no extra test evidence format or tracked trace dump was created. Temporary runner outputs were kept under `/private/tmp`.
