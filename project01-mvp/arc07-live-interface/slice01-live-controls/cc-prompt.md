# CC implementation — control the playing workbench

2026-10-14. CDC → Duncan → CC; CRC independently reviews. Three contributors,
Expedited Mode. Initial assignment, correction 0/5:
`arc07-live-interface/slice01-live-controls/cc-prompt.md`.
Implement an actual terminal-controlled Clock/LFO/TM/MIDI instrument using the
accepted facade. This is software delivery, not a discovery/reporting assignment.
Duncan selected live controls during Arc12 acceptance; directive13 opens this work.

S=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, baseline
`27ca00f64c902b77c2c16d943f1a2b5f650f7f67`.
W=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch planning;
P=W/project01-mvp; A=P/arc07-live-interface; D=A/slice01-live-controls.
K=`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
Plans are this issuance's committed descendants of a5a8e65. Main a7ce6e6 is not the
implementation checkout. Check worktree/HEAD/status before editing; unexpected
source drift goes to CRC for reconciliation rather than copying old receipts.

## Outcome and binding boundary

Add `cargo run -p oxurack --example rack-live -- --patch PATH` with optional exact
MIDI output selection. A musician can play, change Clock/TM/LFO parameters, inspect
Scope, save the current patch and pause/quit with silence, while one synchronous
owner continues delivering samples. Without an endpoint the same runner produces
a paced MIDI trace. Initial state is **paused at sample 0**, waiting for `play`.

The first increment is a small line-oriented terminal frontend. Completion,
history editing, GUI/TUI, controller mapping, watch/subscription bus, external
transport, live topology/removal and interactive replacement/unload are later
Arc07 slices. CLI initial load plus live save are delivered now. Scale editing
remains patch-file input; scalar/bool/string parameters are editable. No new crate,
dependency, runtime service or RON schema is needed. The only facade addition is
the read-only public is_running observation described below.

Use existing `SimulationDriver::Manual`, `advance_samples(1)`, `execute_command`,
parameter getters, Scope and sole MIDI drain. No `App::update`, facade-consuming
`into_app`, reloading to implement a knob, or extra output owner. Old finite
`turing-machine-midi`, `rack-session` and save/rewire examples retain behavior.

## Required reading and preflight

Paths use the roots above; all source is pinned to 27ca00f unless this assignment
explicitly names a new file. Read in order before dependent edits.

| Order | Exact input | Scope and purpose |
| --- | --- | --- |
| 1 | W/AGENTS.md; this prompt; D/slice-plan.md; D/ledger.md; P/cdc-directive13.md | Required-full: current authority, implementation and return contract |
| 2 | P/project-plan.md “Current direction” through the roadmap table; A/arc-plan.md “Current executable plan — v2.0” through its end | Required-section: next capability and retained obligations |
| 3 | K/collaboration-framework/SKILL.md; K/project-management/guides/README.md; K/engineering-methods/guides/07-implementation-prompt-authoring.md | Required-full: workflow, Expedited Mode, intake/readback and execution discipline |
| 4 | K/rust-guidelines/SKILL.md | Required-full; source-specific Rust guidance applies |
| 5 | K/rust-guidelines/guides/03-error-handling.md EH-01; guides/07-concurrency-async.md CA-01, CA-02, CA-07; guides/14-cli-tools/04-output-and-ux.md CLI-27 | Required complete named sections including examples/rationale; failure, ownership and output rules below |
| 6 | S/crates/oxurack/examples/turing-machine-midi.rs; turing_machine_midi/playback.rs; turing_machine_midi/workbench.rs | Required production code through each cfg(test), plus complete playback tests for sender failure/cancellation/cleanup/deadline and workbench writer/error tests. Read referenced fixture/helper definitions too. Reuse real dispatch and presentation seams |
| 7 | S/crates/oxurack/src/lib.rs start/stop/set_driver_mode/advance_samples/drain_midi/Scope methods; lifecycle.rs reset; commands.rs public queue/execute/BPM implementations; persistence.rs parameter/set/snapshot/save; S/crates/oxurack-core/src/parameter.rs ParameterValue | Required complete symbols and referenced helpers/types, not every unrelated inline test. Existing behavior is the contract |
| 8 | S/crates/oxurack/examples/patches/turing-machine-lfo.ron and turing-machine-midi.ron; S/crates/oxurack/Cargo.toml; S/Makefile | Required-full: actual instruments, installed dependencies and commands |
| 9 | S/crates/oxurack/examples/turing_machine_midi/oracle.rs | Conditional required-full if sharing playback.rs under rack-live; its tests reference the root oracle module |
| 10 | P/cdc-directive12.md and Arc12 reports | Reference-only historical design/evidence; existing source behavior and this prompt synthesize what this frontend needs |

Before edits record source state, actually loaded extents and a short source-cited
contract readback in the existing D/ledger.md execution-notes section, later linked
from closing-report.md. Recover truncated tool output with bounded complete reads.
No structured-data query or new receipt document is required. Explain the tricky
interaction: input may block, but the rack owner must not; pause silences without
rewinding; recoverable input errors keep the instrument available. Once coherent,
proceed under this authority. Missing/conflicting required material goes to CRC.

## Source-grounded recipe and applied guidance

Existing `playback::Session<S>` owns native delivery, active notes and fallback
cleanup. Its `send_performance` is private and `play` calls terminal `finish`;
**do not call `play(..., 1)` repeatedly**. That would terminate the session each
sample. Expose the existing delivery/deadline helpers `pub(crate)` only as needed,
and keep a single Session until this CLI exits. Reuse `drain_generated`, exact
endpoint selectors and `workbench::write_view` rather than copying a second sender
or graph. The workbench writer takes `&Oxurack` data, so it need not stop sampling
or consume the facade to report. It observes a bounded retained window.

New files: `examples/rack-live.rs` for CLI/backend/startup/shutdown;
`examples/rack_live/input.rs` for bounded stdin reading/parser;
`examples/rack_live/control.rs` for synchronous commands and owner loop.
Private organization may adapt within these files. `#[path]` imports of existing
playback/workbench are appropriate. If sharing playback tests, also import the
existing oracle as a cfg(test) root sibling: their `super::super::oracle` path
must resolve in both examples. Do not silently disable tests to make sharing work.

Guidance actually applied by CDC:

- EH-01 MUST: syntax/value/file errors are recoverable replies, not panics. Fatal
  delivery/tick/I/O errors retain their cause and trigger cleanup before exit.
- CA-02 MUST / CA-07 SHOULD: move owned input messages through a bounded std channel;
  never share `&mut Oxurack`, the sender, Scope or a World with the input reader.
- CA-01 SHOULD: join an already finished reader and surface unexpected termination.
  **Justified exception:** do not block process shutdown joining a thread stuck
  in terminal stdin read. It owns only input/channel state, no rack/MIDI/file output;
  dropping its handle on quit/Ctrl-C is allowed at this executable boundary. Native
  resources are cleaned up by the main owner first. This is not a reusable library
  thread-lifecycle API or an asynchronous MIDI service.
- CLI-27 MUST: stdout is only MIDI trace (`sample|XX XX XX`), stderr is command
  replies/help/status/Scope/errors. Flush replies so redirected sessions are usable.

## Command contract

Case-sensitive verbs, exact separate module and parameter tokens. Leading/trailing
whitespace ignored; blank lines ignored. No shell expansion, quoting language or
implicit file execution. For PATH/value tails, preserve interior spaces after
trimming the tail; users do not add shell quotes inside the interactive session.
Errors explain the actual unsupported/invalid input and leave the session open.

| Input | Action/result |
| --- | --- |
| `help` | List these commands with concrete examples and pause/reset distinction |
| `play` | `rack.start()`, rebase wall deadline; repeated play preserves state |
| `pause` | `rack.stop()`, then `rack.reset()`, immediately deliver/drain cleanup; preserves seed/phase/sample/Scope/configuration. Repeated pause emits no rack cleanup without new dirty output |
| `reset` | Execute CoreCommand::Panic and immediately drain/deliver silence, preserving current running flag/time/phase. Music can retrigger on subsequent samples |
| `get MODULE PARAM` | Registered getter and readable value, including configured TM base |
| `set MODULE PARAM VALUE` | Read current getter variant, parse corresponding Float/Int/Bool/String; pass CoreCommand::SetParameter to execute_command. Float must be finite, Int strict i64, Bool exactly true/false, String is raw trimmed tail. Scale/unknown variants give explicit unsupported-edit error. Setter remains final validation authority |
| `status` | Running/paused, completed samples, global BPM, sorted instance/kind names and current parameters from snapshot; no tick or reset. Explain global BPM is separate from Clock's audible parameter |
| `scope` | Existing bounded view, rows12, current sample and startup patch label; no mutation of capture |
| `save PATH` | Existing current-configuration SavePatch command; overwrite semantics match existing facade and are documented; reply only after success |
| `scope-save PATH` | Write current Scope CSV with explicit flush; create_new (existing destination gives a recoverable error, preventing capture from truncating the input/saved patch) |
| `quit` | Terminal shutdown below; no later queued input executed |

All no-argument verbs reject extra arguments. Missing args and unknown verbs are
recoverable. `get/set` use separate tokens so dots in instance names are not parsed
as separators. A successful `set` reply reads back the accepted registered value;
failure emits no success. Both get and set report the module/parameter. Read-only
MIDI-out parameters may reject writes through their existing registry contract.
No implicit global-to-Clock BPM synchronization; use `set clock_main bpm 90`.

CLI flags: required `--patch PATH` except `--list-outputs`; exclusive
`--output-name NAME` / `--output-index N`, following existing exact selection rules.
`--list-outputs` conflicts with execution/output/capture options. Optional repeatable
`--scope-port MODULE.PORT` using the existing rsplit parser; `--scope-capacity`
positive default 100000. Always configure capture at startup while stopped; no
selectors means MIDI-only capture. No `--samples` or finite-run alias in rack-live;
EOF, quit or Ctrl-C ends it. The old example retains its finite flags unchanged.
Startup invalid patch/selector/endpoint returns failure before playing. Trace-only
mode must not initialize midir or enumerate endpoints.

## Owner loop, capacity, failure and shutdown

Use a std `sync_channel(32)` carrying owned `Input::{Line, RejectedLine, Eof, ReadError}` messages.
The input thread owns stdin only. Cap a line at 8192 bytes excluding newline; read
in bounded chunks, drain an oversized line through newline and send one recoverable
line-too-long reply, then accept later lines. Do not allocate unbounded `read_line`
strings before checking length. Invalid UTF-8 is a recoverable rejected line;
underlying read errors are fatal. EOF is ordered after all complete input lines;
an unterminated final nonempty line is processed before EOF. Sender disconnection
without explicit EOF/ReadError is an unexpected input-worker failure, not success.
Input backpressure blocks only that thread. Receiver drop unblocks a blocked send;
never unwrap a send after shutdown.

One main loop owns rack, Session (if MIDI selected), writers and cancellation.
Process at most 8 ready inputs before the next sample opportunity; a flood cannot
starve sampling. While paused use recv_timeout at most 20ms to remain Ctrl-C responsive.
While running service input between samples, wait to the next deadline, advance
one sample, drain the complete batch, record Generated; native backend sends in
order and records Sent **only after successful send**. Trace-only never records
Sent. Write trace for ordinary and explicit reset/pause cleanup in drain order.
Boundary cleanup carries its existing completed-sample stamp; no invented tick.
After long command/file I/O rebase the deadline; no catch-up burst. No microsecond
jitter guarantee: synchronous save/view may delay wall time while simulation state
remains continuous. Do not require a scheduler thread to hide that tradeoff.

Recommended control spine, design pseudocode (not compiled):

```rust
loop {
    if cancelled() { break; }
    for _ in 0..8 {
        let Some(input) = next_ready_input() else { break };
        match apply_input(input, &mut rack, &mut backend, &mut replies)? {
            Continue => {}, // invalid user command already replied, no fatal error
            Rebase => deadline = now(),
            Quit => break 'session,
        }
    }
    if rack_is_running() && now() >= deadline {
        let tick_result = rack.advance_samples(1);
        deliver_pending(&mut rack, &mut backend, &mut trace)?;
        tick_result?; // never forget output finalized before a tick error
        deadline = next_deadline_without_catchup(...)?;
    }
    wait_for_deadline_or_input_with_bounded_cancel_latency(...);
}
// On every exit: stop; attempt reset; drain/deliver cleanup; native fallback finish;
// flush; report primary + cleanup failures; drop connection; then exit.
```

The sketch's label/helpers are placeholders for private implementation, not public
API requirements. Check cancellation after waiting and before admitting another
sample. Quit/EOF from an input batch stops immediately; later lines do not execute.
`play/pause/reset` changes take effect before the next sample. The actual facade
has a **private** `is_running(&self) -> bool` at lib.rs:455. Make this exact getter
public with rustdoc and must_use; it reads SimulationRunning without mutation.
This small source-authorized addition supplies truthful status, including module-
initiated stop, rather than a frontend shadow flag. Test read-after-start/stop and
existing shared-stop behavior; no mutable World accessor. For deterministic unit
tests factor a private single-cycle/command function with injected input and time;
production calls the same functions. Do not sleep in correctness assertions.

Parse/lookup/setter/save/export failure replies are nonterminal and preserve
running state; input I/O, sampling, trace/reply writer or native send failure is
terminal. If sending/tracing a drained batch fails, do not continue musical sample
execution; retain first error and proceed to cleanup. Do not turn an output fault
into a successful command failure. On exit, stop/reset and attempt all cleanup,
then native Session::finish once (idempotent existing fallback; extra fallback CCs
are allowed, not claimed as new rack events). Cleanup must run even if stdout or
stderr is broken. Never call process::exit before owners drop. Quit/EOF/Ctrl-C
without errors returns success; terminal/cleanup errors return failure with primary
cause plus cleanup errors. A receiver disconnect/reader panic has a visible failure.
Check JoinHandle::is_finished before joining; tests can join finite injected readers.
Ctrl-C handler sets an atomic only, never touches the rack or sends MIDI.

## Discriminating tests and actual use

Use native example unit tests and existing real patch; add no permanent verifier.
Use an in-memory MidiSender and fallible writers as existing playback tests do.
These checks are the LC row oracles, not a demand for hundreds of duplicate cases.

1. **LC-01:** direct command tests on the real loaded rack: get/set Clock90,
   LFO1 Hz, TM base.25 and waveform triangle; assert exact registered values and
   saved file values. Reject missing/bad input, NaN, LFO rate -1, unknown names, extra args and Scale editing with no snapshot/time
   change. LFO lfo.rs validates frequency in 0..=100; zero is valid, so do not use
   zero as the negative oracle. Malformed then valid input recovers.
2. **LC-02:** two real racks through 200 samples; apply frontend edits to subject
   and the same public API edits directly to control, then 1000 more. Compare all
   MIDI/Scope and sample 1200; include an unchanged third rack to show the chosen
   edit actually changes observable behavior. Input reader blocked/no newline
   must not stop main samples. Queued command flood: one cycle handles at most 8
   and still admits one due sample. Paused cycles advance0; play resumes without
   reseed/catch-up, injected late clock advances at most one due sample per cycle.
3. **LC-03:** dirty note + pause produces literal NoteOff/CC123/CC120 at current
   boundary, no time/phase rewind; second pause no new rack cleanup. Resume keeps
   engines; reset while playing preserves running. Test EOF/quit/Ctrl-C and reader
   error through common shutdown; terminal send/trace failures still attempt
   cleanup and keep primary error. Successful native sends alone get Sent; trace
   has Generated only. Include quit while reader is blocked to reject a hanging join.
4. **LC-04:** commands `scope`, save and scope-save at nonzero sample preserve
   continuing control trace. Export contains current retained records; capacity
   overwrite count is honest. Saved configuration replays like independently
   edited fresh rack, not its prior engine cursor. Existing export destination
   preserves original bytes and session remains usable. Simulated flush failures
   are visible; stdout trace contains no command/help text.
5. **LC-05:** build/run actual rack-live in default and no-graph modes via piped
   stdin. A short Python subprocess smoke can wait for readiness, send play, let
   it run, set/get, pause/status twice, save, scope, quit; require samples>0, paused
   sample unchanged, exact edited values, clean exit and a fresh saved-file trace.
   This is a temporary test command, not a new application/harness. Also EOF and
   malformed-line recovery. Prove the process remains running while stdin is open
   but idle; do not feed play/set/quit at once and call a zero-sample run playback.
   Build native output mode and fake-send tests; no new physical/endpoint gate.

README gives an actual interactive recipe, including output listing/selection,
commands for the existing fixture, pause's silence behavior, trace versus Sent,
CSV create-new behavior and synchronous timing limits. Example:

```text
play
set clock_main bpm 90
set tm_main write_probability 0.25
set lfo_main frequency_hz 1
get tm_main write_probability
scope
save /tmp/my-live-patch.ron
pause
status
quit
```

Required commands from S (record actual results, no skipped failures):
`cargo test --locked -p oxurack --all-targets` and same `--no-default-features`;
`cargo build --locked -p oxurack --examples` and same no-default;
`make lint`; `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings`;
`RUSTDOCFLAGS='-D warnings' cargo doc --locked -p oxurack --all-features --no-deps`;
`make test`; `git diff --check`. Restricted make test has four known CoreMIDI
initialization failures; retain them and run the exact host command through the
available approval mechanism. CDC 27ca00f host baseline passes. Existing RT three
ignores and block advisory remain explicit. Existing finite fixture hashes/tests
must stay green; do not rerun hardware/soak/coverage campaigns for this frontend.

## Scope and return

Permitted source paths: new `crates/oxurack/examples/rack-live.rs`,
`crates/oxurack/examples/rack_live/input.rs`,
`crates/oxurack/examples/rack_live/control.rs`; existing
`crates/oxurack/examples/turing_machine_midi/playback.rs` (shared helper visibility
and necessary factored error/delivery seam, with regression tests),
`crates/oxurack/examples/turing-machine-midi.rs` only if sharing requires wiring,
`crates/oxurack/src/lib.rs` only for public is_running documentation/visibility
and its focused observation test; `README.md`. Frontend tests live inline in the
example modules; the getter test belongs beside the facade tests. Workbench/oracle/fixtures
are read-only reusable inputs. Cargo/dependencies/lockfile, other core/facade behavior,
standalone engines and RT implementation are outside this increment.
If a real missing public capability prevents delivery, give CRC the smallest
source-backed alternative; don't simulate it or create a platform first.

Planning edits: D/slice-plan.md assignment notes, D/ledger.md CC evidence,
D/closing-report.md; optional support under D/artifacts only if necessary.
Artifacts: none expected beyond existing reports/temporary command logs. No
prefilled CRC verdict or acceptance edits. Return all five LC rows as proposed-done
only after evidence, explicit failures, intake/readback, self-review and bubble-up.

Commit source and planning separately with **explicit named files** and
`git commit --only -- <those paths>`; never blanket-stage other work. Include each
actually changed authorized file, no literal brace expansion/wildcard in the final
commit list. No merge/push/release/native endpoint operation is assigned. Report
both hashes, worktree cleanliness and exact assignment to Duncan for CRC review.
Preserve this issued prompt; corrections receive numbered iteration files.
