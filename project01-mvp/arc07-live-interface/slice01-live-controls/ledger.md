# Live controls — acceptance ledger

Opened 2026-10-14 before implementation. All five rows open. CC supplies proposed
results; CRC independently accepts. Authority: slice plan and preserved prompts.
Evidence cells retain CC initial attestation, not CRC acceptance; the dated CRC
disposition below and crc-verification.md identify reproduced support and gaps.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LC-01 | Commands change actual live parameters and reject invalid input without corrupting the session | Real getter/setter/save values; malformed→valid input and typed validation; command replies/provenance | correctness-grade | Operator live controls | open | Direct commands verify Clock90, LFO frequency, TM base/waveform and saved values; malformed, invalid numeric, unknown/extra args and Scale rejected without snapshot/time mutation; valid follow-up and zero accepted; exact CLI values observed. CC iteration01 proposed: status/help test checks sorted live parameters and global/Clock BPM distinction; default/no-default subprocesses verify invalid NaN then valid Clock90 and LFO1 readback, malformed recovery, save, and command replies on stderr | Scalar/bool/string; scale via patch file |
| LC-02 | Waiting input and command processing coexist with continuous single-owner playback | Real 200+1000-sample subject/control/unchanged comparison; bounded fairness; paused 0; no wall-time catch-up | correctness-grade | Existing direct runner | open | Subject equals independently edited control through 1200 samples, MIDI and Scope; unchanged third differs; blocked stdin still permits owner ticks; batch capped at 8; paused cycles advance 0; late deadline advances at most one sample. CC iteration01 proposed: production owner-turn fake-clock tests cover eight FIFO commands, due pause/quit, long diagnostics, one overdue sample/rebase, repeated play and paused no-tick; actual pause/resume is compared to a public stop/reset/drain control through 1000 resumed samples with MIDI/Scope equality and changing LFO values | Public read-only is_running, no shadow flag |
| LC-03 | Pause/reset/shutdown preserve semantics and release output ownership honestly | Exact cleanup and sample state; EOF/quit/Ctrl-C/read/send/write failure paths; Generated versus Sent; blocked-reader exit | correctness-grade | Arc12 lifetime/MIDI contract | open | Exact NoteOff/CC123/CC120 cleanup at same sample; repeated pause inert; resume preserves phase; reset preserves running; EOF/quit/Ctrl-C and read/disconnect/send/trace failures converge on cleanup; blocked-reader quit exits; Generated/Sent distinction asserted. CC iteration01 proposed: native/trace-only byte parity includes explicit pause; Scope Generated/Sent agrees with successful sends; combined read/flush errors retain both causes; dirty NoteOn trace-write failure still attempts cleanup; completed reader panic is surfaced; full-channel quit and actual dirty-voice SIGINT exit are bounded | No hidden second drain or stuck join |
| LC-04 | Scope inspection/export and save expose current state without interrupting simulation continuity | Full retained records, export collision/error, save/fresh configuration replay and continuing control | correctness-grade | Arc11/12 composition | open | Scope retains bounded rows and reports overwrite count; CSV collision preserves original bytes and live session; save reload matches independent fresh control trace at zero cursor; ongoing control trace remains equal; flush/error path visible. CC iteration01 proposed: default Scope/MIDI-only and independent selector/capacity tests; live Scope/save/export subject matches the same live control after 1000 more samples; saved config starts at zero and matches independently loaded config for 2000 samples with nonempty MIDI/Scope and diverges from unedited fixture | No performance checkpoint claim |
| LC-05 | Runnable terminal recipe, both feature modes and affected native gates pass | Actual idle-open stdin/play/edit/pause/save/quit subprocess; fake native output; default/no-graph/native/lint/docs and old example regressions | serious | Playable-first policy | open | Default and no-default example builds and actual idle-open subprocess smokes pass; fake sender/native selection tests pass; all-target suites, lint, Clippy, rustdoc, formatting and full workspace tests pass; old fixture regressions retained. CC iteration01 proposed: latest default/no-default all-targets, example builds, lint, strict Clippy and rustdoc pass; restricted make test records four expected CoreMIDI init failures, exact host retry passes; MIDI2000 and LFO10000 hashes match pinned receipts; default/no-default CLI, EOF and dirty-voice SIGINT subprocesses pass | No hardware/coverage/soak expansion |

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

### CRC initial disposition — 2026-10-14

Reviewed source a603a24d3e02c377557445bd2ef27e4203b33392 / CC planning
9fdb513a2cc72ea9d4cafdc3c4f063408966e333. **All five LC rows remain open**;
CC's initial proposed-done evidence above is preserved as attestation.
[CRC verification](crc-verification.md) reproduces native/mode/ordinary CLI
success and F-01–F-04 failures/gaps; F-05 needs stronger continuation/replay proof.
LC-01: status/help; LC-02: combined owner fairness/resume; LC-03: native trace,
shutdown and actual delivery/fault proof; LC-04: default Scope and nonempty saved/
live continuation; LC-05: affected integration regressions. Current CC assignment
is [iteration01](cc-prompt-iteration01.md), correction1/5 issued, none executed.
No acceptance/closure/deferral or successor advancement; parent gates unchanged.

### CC iteration01 intake and contract readback — 2026-10-14

- Assignment/source preflight: read `cc-prompt-iteration01.md` in full
  (243 lines), preserved `cc-prompt.md` (309), `slice-plan.md` (111), this
  ledger (107), CRC verification (160), CDC Directive13 (57), Arc07 current
  executable plan through EOF (155), Project01 current direction through the
  roadmap (lines 13–341), and initial `closing-report.md` (38). Source S is
  clean at pinned `a603a24d3e02c377557445bd2ef27e4203b33392`; planning W is
  clean at iteration issuance descendant `39b8427ee7d0fedfb841857648e94e717fdee348`
  (predecessor CC planning `9fdb513a2cc72ea9d4cafdc3c4f063408966e333`). Root
  main is separately at docs-only `1954fad49898e556f57b15997f306a65ff80c75e`;
  it is outside this assignment and untouched. Before completing this intake,
  a draft source patch was briefly applied; it was saved outside the repository
  and the four authorized source paths restored to the pinned clean state. No
  source mutation remains at this intake point; the recorded patch is reapplied
  only after this readback.
- Required skill intake: collaboration-framework 2.6.0; project-management
  guides README and iteration preservation/execution sections; work-verification
  independent-verification correction rule; engineering-methods guide07;
  Rust-guidelines 2.0.0 and its AP-33, EH-01, CA-01/02/07, and CLI-27 sections.
  Applied rules: failures must remain visible and source chained; join completed
  workers and report panic, while preserving the explicit bounded exception for
  a reader still blocked on stdin; retain bounded owned channel messages and one
  rack/output owner; stdout carries MIDI data for either backend and stderr carries
  replies. No dependencies, public API, shared playback, facade, runtime, or
  finite-runner changes are authorized.
- Complete implementation reads: assigned `rack-live.rs` (282 lines),
  `rack_live/control.rs` (1245), `rack_live/input.rs` (132), and README's live
  section (heading through the next top-level section). Also read the complete
  relevant `Session::send_performance/finish`, `workbench::write_view`,
  `Oxurack::{bpm,execute_command,snapshot_patch,reset,configure_scope,
  record_scope_midi,drain_midi,advance_samples}`, Scope CSV/MIDI-record paths,
  both named patch fixtures, `crates/oxurack/Cargo.toml`, and `Makefile`.
  `snapshot_patch` reads current getters into sorted module configuration but
  does not tick; `reset` appends voice silence and preserves running/time;
  `advance_samples(1)` requires a complete caller drain before the next sample;
  Session send tracking happens only after a successful send and `finish` sends
  fallback note/controller cleanup without adding rack-generated Scope events.
- CRC's initial record reproduces F-01–F-04 and identifies F-05 as a proof gap,
  not a reproduced phase/save defect. All five LC rows are open. Correct only
  the assigned four frontend/README files: configure Scope at startup with
  default 100000/MIDI-only and optional compatibility `--scope-view`; render
  status from a side-effect-free snapshot with global BPM plus sorted module
  kinds/parameters; make the production owner turn process at most eight FIFO
  inputs before at most one due sample; keep lateness eligible through command
  work, then schedule from post-tick time to avoid catch-up; sample on native
  and trace-only paths after successful native send; and aggregate primary plus
  every cleanup/reader error. Completed input-reader panic is reported; only a
  still-blocked stdin reader may detach after owner cleanup.
- The discriminating proof must invoke that same owner-turn seam with a fake
  clock and advancing command writer: due pause/quit wins, paused admits zero,
  long work admits one due sample and future deadline, and repeated play cannot
  move an already due deadline. Real rack tests compare native trace to
  trace-only plus Generated/Sent evidence; a failing native send has no Sent.
  Shutdown tests combine ReadError with trace-flush failure and dirty NoteOn
  trace-write failure with NoteOff/CC123/CC120 cleanup. F-05 tests compare
  frontend pause/resume to a public stop/reset/drain control for 1000 resumed
  samples, then compare live Scope/save continuation with a control and replay
  the saved config against an independently edited fresh rack for 2000 samples,
  including nonempty MIDI and an unedited negative control. Existing finite MIDI
  and LFO fixture digests are inherited unchanged from CRC's recorded receipt;
  fresh affected/default/no-default/native/host gates remain required.
- This pass is correction 1/5 inside Slice01. The original prompt and CRC verdict
  remain immutable; all five rows stay open until independent CRC disposition.
  Slice02 and Arc07 advancement remain held. This readback is CC intake, not
  independent acceptance.

### CC iteration01 execution evidence — 2026-10-14

- Correction1/5 completed on the assigned S pin. Source changes are limited to
  `README.md`, `crates/oxurack/examples/rack-live.rs`,
  `crates/oxurack/examples/rack_live/control.rs`, and
  `crates/oxurack/examples/rack_live/input.rs`. No facade, dependency, fixture,
  shared playback/workbench, RT, or oracle path changed. Planning changes are
  limited to this ledger, `slice-plan.md`, and `closing-report.md`.
- LC-01: status/help unit coverage verifies full sorted live values and global
  BPM versus Clock BPM; CLI default and no-default runs exercise status/help,
  malformed recovery, rejected NaN then accepted Clock90/LFO1, scope, save and
  CSV. All command replies are on stderr. LC-02: production turn tests use fake
  time and slow diagnostics to prove the eight-command FIFO cap, due pause/quit
  precedence, paused zero ticks, one due sample after delayed work, post-tick
  rebase, and repeated play fairness. A real dirty subject pause/reset/resume
  matches public stop/reset/drain control cleanup and the following 1000 MIDI
  samples plus selected Scope values; LFO values change over the run. LC-03:
  native/trace-only event rows match through explicit pause; Generated/Sent
  records match attempted/successful delivery; sender rejection has no Sent;
  primary and cleanup failures remain visible together; dirty NoteOn trace
  failure still attempts NoteOff/CC123/CC120; reader panic and full-channel
  shutdown are exercised. A no-default CLI SIGINT with stdin open after a dirty
  NoteOn exits0 and emits NoteOff/CC123/CC120 at sample252. LC-04: Scope is
  configured by default; live Scope/save/export leaves the subject equal to its
  direct control after another1000 samples; saved configuration starts at0 and
  equals a separately prepared/loaded equivalent config over2000 samples, with
  nonempty MIDI/Scope and difference from the unedited fixture. LC-05: the
  required commands below and actual default/no-default subprocess behaviors
  pass. No native endpoint was opened.
- Fresh pinned trace receipts, both reproduced byte-for-byte: MIDI2000 is13
  rows, SHA-256
  `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`; LFO10000
  is70 rows, SHA-256
  `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
- Final affected validation in S: `cargo test --locked -p oxurack --all-targets`
  exit0 (facade64; rack-live41; old MIDI example26 and all integration groups
  pass); same with `--no-default-features` exit0 (facade62; rack-live41; old
  MIDI example26 and all integration groups pass; existing
  `crates/oxurack/src/cables.rs:231` unused-mut warning). Both
  `cargo build --locked -p oxurack --examples` modes exit0; `make lint` exit0;
  strict all-feature/all-target Clippy exit0; warning-denying rustdoc exit0;
  `git diff --check` exit0. `make test` first ran restricted and exited2 only
  for four known CoreMIDI initialization failures (146 RT tests passed,4 failed,
  3 ignored); exact prompt-authorized host retry exited0 with RT150 passed,0
  failed,3 existing ignored and workspace/doctests passing.
- Preserved correction/debug trail: the first targeted compile found six boxed
  error type mismatches; source return/aggregation types were corrected. The
  next focused suite found four assertion/setup issues: pause cleanup had been
  omitted from one trace comparison, the pause subject had no dirty voice,
  the injected sender fixture lacked Scope configuration, and replay's fresh
  direct-set control initialized differently from a config-loaded rack. Tests
  now compare both backends through pause, pause immediately after a real
  NoteOn, configure MIDI-only Scope for the sender failure, and independently
  prepare the equivalent patch before loading. Subsequent focused run passed
  41/41. Initial `make lint` failed formatting; `cargo fmt --all` corrected it.
  Strict Clippy then identified one collapsible-if in the modified input reader;
  it was simplified, after which Clippy and lint passed. These were compile,
  formatting, lint, and test-harness corrections; no product defect was
  reproduced.
- This is CC's proposed-done return for correction1/5 only. The five LC rows
  remain **open**, prior CC and CRC evidence above remains intact, CRC has not
  independently reviewed this iteration, and Slice02/Arc07 advancement remains
  held. Exact-path source commit: `e582aed6e87c234799921cc66a5dd55214f76afd`.
  The planning commit hash and final S/W cleanliness are reported in the handoff.
