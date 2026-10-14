# Live controls — acceptance ledger

Opened 2026-10-14 before implementation. All five rows open. CC supplies proposed
results; CRC independently accepts. Authority: slice plan and cc-prompt.md.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LC-01 | Commands change actual live parameters and reject invalid input without corrupting the session | Real getter/setter/save values; malformed→valid input and typed validation; command replies/provenance | correctness-grade | Operator live controls | proposed-done | Direct commands verify Clock90, LFO frequency, TM base/waveform and saved values; malformed, invalid numeric, unknown/extra args and Scale rejected without snapshot/time mutation; valid follow-up and zero accepted; exact CLI values observed | Scalar/bool/string; scale via patch file |
| LC-02 | Waiting input and command processing coexist with continuous single-owner playback | Real 200+1000-sample subject/control/unchanged comparison; bounded fairness; paused 0; no wall-time catch-up | correctness-grade | Existing direct runner | proposed-done | Subject equals independently edited control through 1200 samples, MIDI and Scope; unchanged third differs; blocked stdin still permits owner ticks; batch capped at 8; paused cycles advance 0; late deadline advances at most one sample | Public read-only is_running, no shadow flag |
| LC-03 | Pause/reset/shutdown preserve semantics and release output ownership honestly | Exact cleanup and sample state; EOF/quit/Ctrl-C/read/send/write failure paths; Generated versus Sent; blocked-reader exit | correctness-grade | Arc12 lifetime/MIDI contract | proposed-done | Exact NoteOff/CC123/CC120 cleanup at same sample; repeated pause inert; resume preserves phase; reset preserves running; EOF/quit/Ctrl-C and read/disconnect/send/trace failures converge on cleanup; blocked-reader quit exits; Generated/Sent distinction asserted | No hidden second drain or stuck join |
| LC-04 | Scope inspection/export and save expose current state without interrupting simulation continuity | Full retained records, export collision/error, save/fresh configuration replay and continuing control | correctness-grade | Arc11/12 composition | proposed-done | Scope retains bounded rows and reports overwrite count; CSV collision preserves original bytes and live session; save reload matches independent fresh control trace at zero cursor; ongoing control trace remains equal; flush/error path visible | No performance checkpoint claim |
| LC-05 | Runnable terminal recipe, both feature modes and affected native gates pass | Actual idle-open stdin/play/edit/pause/save/quit subprocess; fake native output; default/no-graph/native/lint/docs and old example regressions | serious | Playable-first policy | proposed-done | Default and no-default example builds and actual idle-open subprocess smokes pass; fake sender/native selection tests pass; all-target suites, lint, Clippy, rustdoc, formatting and full workspace tests pass; old fixture regressions retained | No hardware/coverage/soak expansion |

## Execution notes

CC records actual required-reading coverage/readback and concise command results
here, linking the closing report on return. No prefilled completion claims.

### Pre-edit intake and contract readback — 2026-10-14

- Assignment: `arc07-live-interface/slice01-live-controls/cc-prompt.md`;
  executable Arc07 plan v2; Project01 current direction and roadmap; CDC
  Directive 13; Slice01 acceptance ledger and prompt. Source assignment pin:
  `27ca00f64c902b77c2c16d943f1a2b5f650f7f67`; source worktree was clean at
  intake. The implementation remains proposed-done until CRC disposition.
- Required workflow and Rust intake: collaboration-framework and
  project-management guides README; work-verification; engineering-methods
  implementation-prompt authoring; Rust guidelines `SKILL.md`, core idioms,
  all anti-patterns, error handling EH-01, concurrency CA-01/02/07, CLI guide
  README, output/UX CLI-27, and signal-handling CLI-43. Applicable contracts:
  user input, parsing, I/O, thread, and sender failures use explicit results;
  one bounded reader may block on stdin while the main owner alone mutates the
  rack and output session; preserve stdout for MIDI trace bytes and put
  diagnostics on stderr; avoid panic-driven expected paths and document timing
  bounds.
- Source readback: existing `turing-machine-midi.rs`, shared `playback.rs` and
  `workbench.rs`; `Oxurack` facade, `lifecycle.rs`, `commands.rs`, and
  `persistence.rs`; relevant output/session tests, Scope writer tests, and the
  parameter getter/setter/snapshot/save paths. `start`/`stop` preserve engine
  and sample state; `reset` appends voice silence without ticking or stopping;
  `advance_samples(1)` is the manual owner's one-sample unit and surfaces latched
  MIDI output faults; `drain_midi` is caller-owned and must finish before the
  next advance. `parameter` and `snapshot_patch` read actual live state;
  `set_parameter` is synchronous and typed; save serializes the live snapshot.
  Existing `playback::Session` owns note tracking, ordered cleanup, send
  accounting, and terminal finish. Scope's existing `write_csv` and
  `workbench::write_view` provide the serialization seam.
- Readback of the key live contract: idle stdin cannot stall playback because
  only its dedicated bounded reader blocks; the owner polls commands fairly and
  advances at most one due sample, rebasing deadlines after command/file work.
  Pause transitions through stop, reset, and the owner's one drain/delivery
  boundary, preserving sample time and engine phase; repeated pause is inert.
  Reset is a Panic command and preserves the current running state. Bad commands
  and recoverable file failures report and leave the same session usable; tick,
  output-send, input-read, and trace-write failures enter common cleanup.
  Scope rows distinguish Generated from successfully Sent events, and exports
  or patch saves do not replace or rewind the running rack.
- Required implementation and verification remain LC-01 through LC-05 as
  specified in the immutable prompt, including subject/control/unchanged
  comparisons and a real subprocess smoke with stdin held open. No hardware,
  coverage, or soak gate is added. No source mutation had occurred when this
  readback was recorded.

### CC execution evidence — 2026-10-14

- LC-01–LC-04: inline example/facade tests exercise actual loaded racks, command
  values and persisted patch values; malformed-input recovery; real subject,
  direct-control and unchanged-control comparison; pause/reset/resume/shutdown
  semantics; bounded Scope/export collision; and save/reload behavior. Evidence
  is in the corresponding test modules in the source commit.
- LC-05 commands passed in S: `cargo test --locked -p oxurack --all-targets`;
  same with `--no-default-features`; `cargo build --locked -p oxurack --examples`
  and same with `--no-default-features`; `make lint`; `cargo clippy --locked -p
  oxurack --all-features --all-targets -- -D warnings`;
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p oxurack --all-features
  --no-deps`; `cargo fmt --all -- --check`; `make test`; and `git diff --check`.
  Default and no-default actual subprocess smokes passed with stdin held open,
  commands/edits and malformed-line recovery, pause stability, save/export,
  MIDI-only stdout, quit/EOF, and saved-patch trace replay. Fake sender tests
  cover native send behavior without opening a hardware endpoint.
- Restricted `make test` first failed only four CoreMIDI initialization tests
  (`test_list_input_ports`, `test_list_output_ports`,
  `test_open_input_nonexistent_returns_error`,
  `test_open_nonexistent_port_returns_error`); 146 passed and 3 were ignored in
  the affected runtime suite. The prompt-authorized exact host retry passed the
  full workspace and doctests: runtime 150 passed, 3 existing ignored. The
  no-default test run emits one pre-existing unused-mut warning at
  `crates/oxurack/src/cables.rs:231`, outside this assignment. No other
  validation failures remain.
- Final `git diff --check` is recorded separately for S and W before commit.
  Correction count: 0/5. All five rows are returned proposed-done for CRC review;
  no acceptance or closure verdict is claimed. See [closing report](closing-report.md).
