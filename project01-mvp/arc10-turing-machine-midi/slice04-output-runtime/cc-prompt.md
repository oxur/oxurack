# CC implementation — play the rack on a selected MIDI endpoint

## Assignment, authority and user outcome

Implement project01-mvp / arc10-turing-machine-midi / slice04-output-runtime,
initial pass. Current assignment: `arc10-turing-machine-midi/slice04-output-runtime/cc-prompt.md`.
Three contributors, Expedited Mode; Duncan relays your return to Sofie/CRC.
Source `S=/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, clean accepted predecessor
`2942c594a445b5de3ef8f131cdedaa6095aa1529`. Planning project
`P=/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp`,
predecessor `f19e2f1` (Slice03 CRC acceptance); record actual issue HEAD.
Resolve unexpected drift before dependent work; preserve unrelated changes.

**Binding:** directive08 and design v1.2 §§4–5 authorize extending the existing
example with a single-owner synchronous `midir` connection, monotonic pacing,
finite playback, interrupt/error cleanup and a real endpoint trial. Production
runner edits are authorized. No new RT service, worker, runtime mode, master
clock, transport messages, reconnection, persistence or scheduler changes.

From S, Duncan will run:

```bash
cargo run --locked -p oxurack --example turing-machine-midi -- --list-outputs
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron \
  --output-name 'EXACT ENDPOINT NAME' --samples 5000
# Alternatively: --output-index 0, using the index from a fresh listing.
# Existing unpaced offline trace command remains valid:
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron --samples 2000
```

## Required reading and preflight

Resolve installed equivalents of
`F=/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/collaboration-framework`
and `R=/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/rust-guidelines`.
Author applied framework 2.6.0 / Rust 2.0.0. Historical prompts/logs are reference-only.

| Order | Exact input / state | Required scope and purpose |
| --- | --- | --- |
| 1 | This prompt, `slice-plan.md`, `ledger.md` | Full; assignment, five criteria, behavior and evidence authority. |
| 2 | `P/cdc-directive08.md`; `P/arc10-turing-machine-midi/design.md` | Directive full; complete design §§3–5, necessary clock/publication definitions §§1–2. Lifecycle and witness authority. |
| 3 | `P/project-plan.md` current direction/handoff/version v3.19; Arc10 `arc-plan.md` v1.7 Capability, Slice sequence, Composition, Delivery; Slice03 `crc-verification.md` | Named parent sections; predecessor review full. Retain verified sink/engine behavior, not historical gates. |
| 4 | `S/crates/oxurack/examples/turing-machine-midi.rs`, its `examples/patches/turing-machine-midi.ron`, umbrella Cargo.toml, `tests/midi_output.rs` | Full at accepted source; actual executable, dependencies, fixture and independent oracle. |
| 5 | `S/crates/oxurack/src/lib.rs` new/try_install/install/load_patch/start/stop/driver_mode/set_driver_mode/sample_time/drain_midi/midi_output_error/advance_samples and helper with_completed_samples; `S/crates/oxurack-mod-midiout/src/lib.rs` MidiOutputEvent/MidiOutputBuffer definitions and methods | Complete named symbols and necessary types; borrow lifetimes, terminal faults, discard behavior and finalized errors. No need to reread unrelated facade tests. |
| 6 | Pinned local registry `midir-0.11.0/src/common.rs` MidiOutput/MidiOutputPort/MidiOutputConnection impls, `errors.rs` InitError/PortInfoError/ConnectError/SendError; `ctrlc-3.5.1/src/lib.rs` crate docs, set_handler/try_set_handler/init_and_set_handler; clap 4.6.6 `src/_derive/mod.rs` Terminology, Arg Attributes and Arg Types; clap_builder 4.6.6 Arg required_unless_present/conflicts_with/conflicts_with_all | Complete named definitions/docs; locate Cargo registry on your host. Verified owner-consuming connect, synchronous mutable send and process-global signal registration. Recheck actual lock versions; if changed, inspect those versions before dependent edits. |
| 7 | `R/SKILL.md`, `R/guides/11-anti-patterns.md`; `02-api-design.md` API-23/API-39, `03-error-handling.md` EH-01, `05-type-design.md` TD-05, `13-documentation.md` DC-05; `14-cli-tools/README.md`, `02-argument-parsing.md` CLI-05, `08-advanced-topics.md` CLI-43 | Entrypoint/anti-patterns/CLI README full; other complete named sections, relative to R/guides. Apply Result/owned-state/validation/CLI guidance. CLI-43's second-interrupt forced-exit example is not this assignment's repeated-stop policy. |
| 8 | `F/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md` Required reading and CC intake / §5; `F/knowledge/testing/guides/03-validation-gates.md`; `S/Makefile` test/lint/docs recipes | Named authoring sections; testing guide full; Make named recipes/dependencies. Proportionate native proof, examples included. |
| 9 | `S/Cargo.lock` | Required-data: project name/version/dependencies for every package whose name is in {midir, ctrlc, clap, clap_builder, clap_derive, clap_lex, oxurack}, before/after edits (7 rows at issue); inspect complete results and lockfile delta, no unrelated upgrade. Not a full prose read. |

Before dependent edits, put a concise intake in this slice's `closing-report.md`:
source/planning identity, cleanliness, loaded extents/recovered truncation,
versions, separate data-query coverage, and source-cited contract readback.
Explain successful-send note ownership, sample/send cancellation boundaries,
discard before cleanup, late deadline rebasing and witness limits. Hashes,
headings and another context's reading do not establish full loading. Recover
lost binding context before continuing. Resolve unavailable/conflicting inputs;
coherent preflight is authority to proceed, not another permission gate.

## Source reconnaissance and applied Rust decisions

At 2942c59 the example owns a stopped fresh Oxurack, installs Clock/TM/MidiOut,
loads RON, advances one manual sample and consumes the borrowed Drain. Its
closure plus unconditional rack.stop already protects error exits. Keep that
actual path. advance_samples errors may include a finalized sample and queued
prefix; a playback failure must discard that prefix rather than send it later.
The sink's logical note state is not the endpoint's sent-note state.

Cargo.lock already contains midir 0.11.0 and ctrlc 3.5.1 transitively; clap 4.6.6
is an umbrella dev dependency. Add **dev dependencies** `midir = "0.11"` and
`ctrlc = "3.5"` to umbrella Cargo.toml, retaining existing resolutions. No new
normal MIDI-I/O dependency in the standalone TM, no midi-io bypass. No new
thiserror/async/test-tool dependency is needed for this small application.

Verified midir methods: MidiOutput::new(&str) -> Result, ports() -> Vec<port>,
port_name(&port) -> Result<String, PortInfoError>, connect(self, &port, &str)
-> Result<MidiOutputConnection, ConnectError<MidiOutput>>, and connection.send
(&mut self, &[u8]) -> Result<(), SendError>. The connection remains on the runner
thread. ctrlc::try_set_handler accepts FnMut + Send + 'static and can fail for
an existing handler; it creates its own signal-notification thread. Oxurack::new
adds Time/Core/optional Graph plugins, not TerminalCtrlCHandlerPlugin. Do not
install a second Bevy Ctrl-C plugin or call App::run here.

| Read guidance | Applied choice and proof |
| --- | --- |
| AP-23 AVOID | Owned small note/channel tables and a direct loop; no speculative queue/atomic service. Only the notification flag crosses threads. |
| EH-01 MUST; AP-25/AP-33 AVOID | OS/config/write failures are Result; always stop/discard/attempt cleanup and report both primary and cleanup errors. Failure-after-good-note tests distinguish swallowed errors. |
| API-23 MUST | No new library API. Existing concrete CoreError remains. Private application error aggregation may contain Box<dyn Error>; this is not a library return-type exception. Preserve operation/message context and original primary cause. |
| API-39 MUST | Typed CLI indices, exact unambiguous name lookup and real patch validation before start; no narrowing/masking/fallback selection. |
| TD-05 MUST | Private Session fields and send/finish methods own transitions; no mutable world or public backend API. |
| DC-05 MUST | Document private fallible helpers' relevant errors and partial success, plus runnable README limitations; no claim Drop establishes silence. |
| CLI-05 MUST / CLI-43 SHOULD | Clap derive; one fallible interrupt registration with a flag-only handler. Repeated interrupt requests remain harmless; no forced second-Ctrl-C exit. |

## Binding behavior and recommended implementation

### CLI, preparation and compatibility

Keep `--patch PATH --samples N` as unpaced trace mode, default N = 2000, N = 0 valid.
Add `--list-outputs`, `--output-name STRING`, `--output-index usize`. Listing
needs no patch and emits current zero-based indices and exact names; empty
inventory prints no entries and succeeds. Initialization/name-query/write
failures are visible/nonzero. Listing conflicts with patch and either selector;
selectors conflict with each other. Patch is required unless listing; playback
requires exactly one selector, inferred by its presence, no extra --play flag.
Use clap derive attributes (`required_unless_present = "list_outputs"`,
`conflicts_with[_all]`) and test Cli::try_parse_from. Empty name rejects; compare
nonempty names exactly without trimming/case-folding/substring matching.

Playback preparation order: construct/install/load/validate **fresh stopped rack**,
register Ctrl-C handler once, enumerate/select/connect endpoint, then start manual
advancement. A load/spawn failure drops this unused rack; core loading is not
transactional. Never send performance data or advance before successful preparation.
N = 0 still validates/opens the selected endpoint, then sends nothing (no used channels).

Enumerate a single ports snapshot and its names. `--output-index` must exist in
that snapshot; indices can change between commands, which help/README must say.
`--output-name` must have exactly one exact match; none or duplicate rejects with
context and suggests fresh listing/index. Failed port_name is an error, not a
silently skipped port. Select before consuming MidiOutput in connect. Failed
connect is propagated before start; no implicit fallback/reconnect/new session.
One narrow pure selection helper over a names slice permits duplicate-name tests;
this is not a generalized endpoint backend.

Playback stdout uses the existing `sample|SS DD DD` format **only for successfully
sent performance messages**, in order. No cleanup rows/headers on stdout; endpoint
identity, stop/failure details go to stderr. Update sent-note ownership immediately
after send succeeds, before the fallible trace write. Otherwise a broken pipe
could leave a successfully sent note unowned. Finish cleanup before reporting a
trace/flush error. Existing trace-mode write failures remain nonzero.

### Send ownership and cleanup

Recommend private helpers in `crates/oxurack/examples/turing_machine_midi/playback.rs`,
loaded by an explicit #[path] module from the existing hyphenated example. Inline
tests there run via `cargo test --example turing-machine-midi`. Keep module names
local; no new public library surface or standalone sample engine. A narrow private
sender trait can have associated Error: std::error::Error + 'static and a send
method taking [u8;3]; production forwards to MidiOutputConnection::send(&bytes),
tests record attempts and inject failures. This is the only output send seam;
no generic runtime/backend/clock framework. Use a pure deadline helper for pacing.

Recommended private call shapes (names/organization adaptable):

```rust
trait MidiSender {
    type Error: std::error::Error + 'static;
    fn send(&mut self, bytes: [u8; 3]) -> Result<(), Self::Error>;
}
impl MidiSender for midir::MidiOutputConnection {
    type Error = midir::SendError;
    fn send(&mut self, bytes: [u8; 3]) -> Result<(), Self::Error> {
        midir::MidiOutputConnection::send(self, &bytes)
    }
}
fn load_rack(path: &std::path::Path) -> Result<Oxurack, oxurack::CoreError>;
fn play<S: MidiSender, W: std::io::Write>(
    rack: &mut Oxurack, session: &mut Session<S>, samples: u64,
    cancel: &std::sync::atomic::AtomicBool, trace: &mut W,
) -> Result<(), PlaybackFailure>;
```

These are illustrative signatures/adapter, not compiled here. load_rack installs
and loads while stopped; play owns start/loop/stop/discard/finish and accepts an
already opened Session. Inline tests retain rack and Session after play, inspect
sample_time/drain and recording attempts, and assert stopped behavior with
assert_eq!(rack.advance_samples(1).unwrap(), 0) without needing private World access. Only main registers
the OS handler/opens the real endpoint; sender faults exercise actual play.

Recommended shapes (illustrative, not compiled; behavior is binding):

```rust
struct Session<S> {
    sender: S,
    active: [Option<u8>; 16], // only successfully sent sounding note per channel
    used: [bool; 16],        // channels on which a performance send was attempted
    finished: bool,
    cleanup_errors: Vec<SendFailure>, // bounded by 16 offs + 32 channel CCs
}
struct SendFailure { bytes: [u8; 3], detail: String }
struct PlaybackFailure {
    primary: Option<Box<dyn std::error::Error>>, // send/rack/trace/timing error
    cleanup: Vec<SendFailure>,
}
```

All sink events are validated three-byte NoteOff/NoteOn/CC messages. The sender
gets those bytes directly; do not reinterpret the TM or add transport. For each
performance attempt mark its channel used **before** calling send (a failed OS
call need not prove no bytes arrived). Only after Ok update active: positive
NoteOn records note, NoteOff or velocity-zero NoteOn clears its matching note;
CC leaves note ownership unchanged. Failed send leaves active unchanged, stops
remaining performance sends and causes unsuccessful exit. Do not mark a failed
new note active; cleanup CCs cover its attempted channel, without asserting that
an OS error proves the endpoint quiet.

Illustrative successful-send commit boundary, inside the private Session:

```rust
// ensure_playing returns an application error after finish; no further send.
self.ensure_playing()?;
let channel = usize::from(bytes[0] & 0x0f);
self.used[channel] = true;
self.sender.send(bytes)?; // attach bytes/sample context; error skips mutation below
match bytes[0] & 0xf0 {
    0x90 if bytes[2] > 0 => self.active[channel] = Some(bytes[1]),
    0x80 | 0x90 if self.active[channel] == Some(bytes[1]) => {
        self.active[channel] = None;
    }
    _ => {} // CC does not change the sounding-note ledger
}
// The runner may now write this successfully delivered event to its trace.
```

The one-note-per-channel table relies on the accepted sink's ordered old-off/new-on
stream and distinct static channels; do not use it as a general polyphonic backend.

Normal limit, cancellation, rack fault, send failure or trace failure all converge:
stop rack advancement, discard the full remaining drain/pending performance
buffer, then finish the session. In finish, mark terminal **before** attempting
cleanup; no further performance send is permitted. For ascending channels send
an explicit [0x80|channel, note, 0] for each tracked active note, then for ascending
used channels [0xB0|channel,123,0] followed by [0xB0|channel,120,0]. Attempt **all**
remaining cleanup messages after a cleanup failure. Report each failed message;
never replace the original error with the cleanup error. At most 48 cleanup
attempts and error records. Repeated finish does not resend; it exposes the same
recorded outcome. A fresh invocation reconstructs rack and session; stopped
state/unsent performance are never replayed. Drop releases the connection only.

```rust
// Illustrative control flow. Scope the drain so it ends before rack.stop().
let primary = (|| {
    for _ in 0..samples {
        wait_until_deadline();
        if cancel.load(Ordering::Relaxed) { break; }
        let started = Instant::now();
        if rack.advance_samples(1)? == 0 { break; }
        for event in rack.drain_midi() {
            if cancel.load(Ordering::Relaxed) { break; }
            session.send_and_record(event.message.to_bytes())?;
            write_trace(event)?; // ownership already recorded if this fails
        }
        if cancel.load(Ordering::Relaxed) { break; }
        deadline = next_deadline(deadline, started, Instant::now())?;
    }
    Ok(())
})(); // errors here still reach the unconditional finish path
rack.stop();
drop(rack.drain_midi()); // deliberate discard; no stale send or fault reset
let cleanup = session.finish(); // all offs/CC attempts, cached on repeated calls
// Combine primary + cleanup; flush/report only after the cleanup path executes.
```

A cancelled send batch drops its remaining Drain. An OS send already in progress
returns before cancellation can take effect; record its success before checking
again. Rack advance itself is a full latched sample; do not truncate its five
phases or add phase-local stop checks. Stop checks before each next sample and
performance message prevent additional work once observed. Cleanup ignores the
cancel flag so repeated Ctrl-C does not abandon cleanup. Successful finite or
interrupt stop exits0; any preparation/performance/trace/cleanup failure exits
nonzero with context. No promise after SIGKILL, nor a timeout for a blocking OS send.

### Pacing and interrupt

Use std::time::Instant/Duration and thread::sleep; first sample is due now.
The period is 1 ms. Wait until the monotonic deadline (recheck after sleep), then
check cancellation before advancing. Compute the normal next deadline as previous
+1ms. If dispatch started after its deadline **or** finished after that next
normal deadline, rebase to finished+1ms. Otherwise keep the normal deadline.
Use checked Instant additions and propagate any timing error through cleanup.
This deliberately slows under late scheduling; it never executes an unbounded
catch-up loop. Pacing is wall-clock scheduling around the manual sample API,
not a new logical clock; idle samples count toward --samples. No jitter target.

One ctrlc::try_set_handler closure captures Arc<AtomicBool> and merely stores true
(Relaxed suffices: notification carries no accompanying data). Register outside
unit-tested runner calls, once in the live executable. Failure is a pre-play
error, never ignored. Do not sleep-spin, send MIDI from the handler, mutate the
rack there, or install per-session test handlers. Test cancellation by flipping
the same supplied flag during a recording sender's successful call.

## Implementation sequence and tests

1. Add the two umbrella dev dependencies and necessary lockfile edges without
   upgrading existing versions. Refactor the current example's shared fresh-rack
   preparation/trace formatting only as needed; keep the literal fixture unchanged.
2. Implement CLI relationships and snapshot selection/opening; wire list, offline
   trace and playback branches into existing run/main/ExitCode with clear errors.
3. Add the private Session/send/finish and actual paced manual runner above.
   Inline tests invoke that **same** runner with a recording/failing sender and
   actual loaded fixture/rack, not a copied loop. No mutable World accessor needed.
   A short fast-clock temporary clone of the existing RON can make fault/stop tests
   quick; use deserialize_patch/save_patch_to_file, as in tests/midi_output.rs.
4. Document listing, exact selection, finite samples/interrupts, monotonic slowdown,
   cleanup/errors and witness limits in root README. Run checks, then arrange the
   real musical trial with Duncan; record its outcome or specific availability gap.

| Row / real setup and action | Required observation / rejected mistake |
| --- | --- |
| O-01 names ["A","AB","A"] | Exact name AB selects index1; A is ambiguous; name B does not substring-match. Index2 selects that snapshot's second A; index3 and empty name reject. CLI list needs no patch, incompatible flags/missing required patch/negative index reject. Invalid RON and an impossible index through actual executable exit nonzero/no trace; preparation/open errors cannot start rack. OS-connect error propagation also inspected. |
| O-02 real loaded fast Clock→TM→sink, recording sender, finite 24 samples | Successful performance attempts and trace equal Slice03's independent engine/literal-byte oracle (reuse expected logic, no production transition as oracle); exact original order including retrigger. Completed sample count24, drained buffer empty, rack stopped after finish. Pure deadline check at due0/start0/finish0.2ms -> next1ms; overdue start5ms/finish5.2ms -> next6.2ms, and no immediately due next sample. No wall-clock percentile test. |
| O-03 note 60/channel 0 succeeds, next new-note send fails | Through Session and actual runner fault path: stop new sends, discard pending performance, preserve primary error. If preceding note-off succeeded, no redundant explicit off for 60; if it failed, cleanup must retry off 60. Use literal synthetic messages to distinguish both ledgers, plus a real-rack failure after a good note to prove runner convergence. Used-channel cleanup includes CC123/120 even when the first attempted note failed. |
| O-03 active channel 1 note 64 and channel 0 note 60; interrupt/limit | Cleanup exactly [0x80,60,0],[0x81,64,0],[0xB0,123,0],[0xB0,120,0],[0xB1,123,0],[0xB1,120,0]. Fail first cleanup off: later off and all CCs still attempted; all failures plus primary retained, nonzero outcome. Second finish sends nothing and preserves failure result. Interrupt flag set by successful sender stops remaining performance batch/next sample, then cleans up; fresh rack/session reproduces beginning without stale messages. |
| O-04 actual offline executable and native gates | Two fresh 2000-sample traces remain equal to accepted 13-row trace; samples0 empty; missing/malformed patch fail. Default/no-default example tests and existing MIDI integration pass; all-target Clippy covers helper/example. Playback0 makes no performance or cleanup sends in the recording path. No new harness/validator. |
| O-05 named real endpoint + Duncan | Run the documented fixture/seed 42 command on a selected software OR hardware destination; record endpoint/index or name, receiver/synth routing, command/sample limit, observed musical output, stop/interrupt and absence/presence of stuck notes. Compare performance byte trace with offline run at the same sample count (cleanup is outside stdout). A recording sender or virtual endpoint with no real Operator observation is not this witness. |

The short send ledger tests explain why active ownership changes only after
successful delivery; actual runner tests establish convergence, not just the
private helper. Local test names/helper organization are discretionary. Additional
credible regression fixes in the same component are allowed; acceptance changes
and major architecture/scope changes return through Duncan to CDC.

Run from S; record command/source/exits/failed attempts and concise results:

```bash
cargo test --locked -p oxurack --example turing-machine-midi
cargo test --locked -p oxurack --no-default-features --example turing-machine-midi
cargo test --locked -p oxurack --test midi_output
cargo test --locked -p oxurack --no-default-features --test midi_output
make test
make lint
cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
cargo build --locked -p oxurack --example turing-machine-midi
git diff --check
```

Use the built executable for fresh traces/CLI controls and host `--list-outputs`.
Reproduce default 2000-sample output with SHA-256
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b` at unchanged
fixture, accepted at `2942c59`. Retain unaffected Clock/TM standalone/mapping evidence
at Slice03/Slice02 pins after checking no relevant dependency/production delta;
no need to repeat their full matrices. Known restricted CoreMIDI failures require
host retry; retain failures and three existing RT timing/scheduling ignores.
Genuine new failures stay open. No make check/check-deps/outdated, coverage/jitter/
soak gates, copied probe crate, planning validator or new evidence format.

## Witness gate, scope and return

O-05 is an Operator gate; device availability does not prevent software work.
After the live path exists, present actual endpoint listing and the runnable
command to Duncan for selection/routing/witness if not already supplied. Do not
send to a guessed device, create system MIDI endpoints or change a DAW silently.
If unavailable, complete all software work/gates, leave O-05 explicitly open
with the exact blocker and re-entry command, and return it to CRC. Only CDC may
approve the design§5 T-04 deferral to Arc08/Arc11 advancement; CC cannot call a
mock pass a witness or close the arc. A tool-permission/environment failure is
not the same as absence of a usable endpoint on the host.

Authorized source: umbrella Cargo.toml, Cargo.lock, existing example and private
helpers/tests under examples/turing_machine_midi/, root README.md, and directly
related umbrella tests if needed. Existing module/core/RT/MIDI-wire production
and literal musical fixture remain inherited; no behavior change there is
needed for direct output. This is component scope for the runner, not a blanket
production fence. If the baseline API genuinely defeats the design, report that
mismatch before changing the public contract. No merge/push/release, new ignored
regression or lint suppression.

Planning return scope: this slice's closing-report.md, ledger.md proposed notes,
optional artifacts/test-evidence.md or named failure/retry logs under artifacts/.
No edits to this issued prompt, slice-plan, parent plans/ledgers or prior packets.
Intake, command results, row walk, applied guidance and Bubble-up to arc belong
in the existing report. Inventory actual durable supporting artifacts (or none).

Commit source first and evidence separately. Inspect actual changed paths; use
explicit same-path `git add -- <files>` and `git commit --only -- <files>` lists,
never globs or git add .; preserve unrelated staging/dirt. Return prompt/source/
evidence hashes, exact path lists, five-row proposed status with executed oracle
results, environment failures/retries/ignores, deviations and endpoint witness
or specific gap. CRC independently accepts; do not advance Arc10/Arc11 yourself.
