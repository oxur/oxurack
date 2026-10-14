# CC iteration01 — keep live commands fair and observations complete

2026-10-14. CRC → Duncan → CC; return implementation/evidence to CRC.
Three-Contributor Workflow, Expedited Mode. Execute this correction inside
Arc07/Slice01, not a new slice. Current assignment:
`arc07-live-interface/slice01-live-controls/cc-prompt-iteration01.md`.
Predecessor `cc-prompt.md` remains immutable. Initial CC return source
`a603a24d3e02c377557445bd2ef27e4203b33392`, planning
`9fdb513a2cc72ea9d4cafdc3c4f063408966e333`. Correction1 issued of5; none executed
yet. Five LC rows remain open; initial working claims are supporting evidence,
not previously accepted rows. This is implementation plus focused proof repair.

## Authority, roots and bounded reading

S=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, pinned a603a24 above.
W=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch planning;
P=W/project01-mvp; A=P/arc07-live-interface; D=A/slice01-live-controls.
K=`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
Planning baseline is this prompt's issuance descendant of9fdb513; check actual
HEAD/status in both worktrees. Unexpected implementation drift returns to CRC.
Root main is outside this assignment: CDC baseline was a7ce6e6; final CRC check
observes separate documentation-only commit1954fad49898e556f57b15997f306a65ff80c75e
there. Do not reset/change root main or copy that README over the assigned source.
The implementation remains clean at a603a24. CRC has changed planning only.

| Order | Input | Scope/purpose |
| --- | --- | --- |
| 1 | W/AGENTS.md; this prompt; D/slice-plan.md; D/ledger.md; D/crc-verification.md; P/cdc-directive13.md | Required-full: current assignment, F-01–F-05 and original design authority |
| 2 | D/cc-prompt.md | Required-full: continuing command, ownership, failure, capacity, validation and scope contract; this iteration resolves the concrete implementation gaps, without relaxing it |
| 3 | A/arc-plan.md “Current executable plan” to EOF; P/project-plan.md “Current direction” through roadmap table | Required-section: successor boundary and outstanding acceptance |
| 4 | K/collaboration-framework/SKILL.md; K/project-management/guides/README.md; K/engineering-methods/guides/07-implementation-prompt-authoring.md; K/rust-guidelines/SKILL.md | Required-full: workflow, complete intake/readback and domain guidance |
| 5 | K/rust-guidelines/guides/03-error-handling.md EH-01; guides/07-concurrency-async.md CA-01/02/07; guides/14-cli-tools/04-output-and-ux.md CLI-27; guides/11-anti-patterns.md AP-33 | Required complete named sections/examples/rationale: errors, channel/reader ownership and output |
| 6 | S/crates/oxurack/examples/rack-live.rs; rack_live/control.rs; rack_live/input.rs | Required-full production and inline tests: actual defects and extension homes |
| 7 | S/crates/oxurack/examples/turing_machine_midi/playback.rs Session/MidiSender/finish; workbench.rs write_view; S/crates/oxurack/src/commands.rs bpm/execute_command; persistence.rs snapshot_patch; lifecycle.rs reset; scope.rs capture record/CSV methods | Required complete named symbols and referenced type/helpers: reuse ownership, exact cleanup and observation |
| 8 | S/crates/oxurack/examples/patches/turing-machine-lfo.ron; turing-machine-midi.ron; S/Makefile; S/crates/oxurack/Cargo.toml; README.md “Run and edit a live rack” | Required-full except README named section: fixtures, gates, installed dependencies, changed UX |
| 9 | D/closing-report.md | Required-full initial attempt; preserve and append correction evidence |

Before dependent edits, record actual loaded ranges/source state and a short
source-cited contract readback in existing ledger execution notes. Recover omitted
or truncated reads; hashes/headings are not reads. No structured-data query,
separate receipt artifact, audit platform or new dependency is needed. Explain
why rebasing after commands must not erase an already due sample, why native
output still has a trace, and why a primary error cannot discard cleanup failures.
Resolve missing/conflicting required inputs with CRC; otherwise proceed under
this assignment without further permission.

## Applied guidance and repair scope

CRC inspected every changed frontend file at a603a24 and independently reproduced
F-01/F-02/F-04 with the actual unchanged production functions, plus F-03 with the
actual CLI. F-05 is missing discriminating evidence; no resume/save engine defect
was reproduced. Correct these in the existing example, preserving facade/runtime
semantics and all finite runners. No subscriptions, topology UI, controller, external
transport, catalog expansion or new public API belongs here. Those remain CDC13
successor work.

| Guide | Applied decision and observable proof |
| --- | --- |
| EH-01 MUST; AP-33 AVOID | Preserve primary and every cleanup cause in the returned failure even when stderr is broken; return reader-join failure rather than swallowing it; injected dual faults/panic test |
| CA-01 SHOULD | Join completed input worker and propagate panic; keep initial prompt's documented exception for an input-only reader still blocked at executable shutdown; do not wait forever |
| CA-02 MUST; CA-07 SHOULD | Retain one rack/output owner and bounded owned input messages; receiver disappearance stops further reader work; fairness is a real owner-cycle test |
| CLI-27 MUST | Trace all ordinary/reset/pause rack events to stdout in either backend; replies/status/help remain stderr; native success alone produces Sent |

## Repair recipe and binding behavior

### 1. Restore CLI and command observation (F-03; LC-01/04/05)

In `rack-live.rs::Cli/prepare_rack`, always configure bounded Scope before
playback, using existing selectors and positive capacity (default100000). No
selectors means MIDI-only capture. `--scope-port` and `--scope-capacity` work
without `--scope-view`. Keep the already documented `--scope-view` spelling
accepted as an optional compatibility flag; it cannot gate capture or become
required. It need not auto-print a view: interactive `scope` does that. Retain
list-outputs conflicts with execution/capture arguments, required patch and exact
endpoint selection. No `--scope-csv` flag is being introduced; correct that hint.
Trace-only still never initializes/enumerates MIDI endpoints.

In `control.rs::handle_input`, implement full status from actual snapshot:
completed sample, rack.is_running, `snapshot.bpm` (or the existing `rack.bpm()`),
then modules sorted by instance name with kind and all configured parameter values.
Sort parameter keys for readable deterministic output; reuse display_parameter.
Explicitly explain global BPM differs from a Clock's audible `bpm` parameter.
No tick/reset/save side effect. Local line formatting is discretionary; tests
must find Clock90 alongside global120 in a fixture whose Clock has been edited.
Help includes concrete get/set/save examples and states pause silences/stops
without rewind while reset silences/preserves running. README matches actual
flags/default capture, save overwrite behavior and native/trace output.

### 2. Make a real owner turn fair (F-01; LC-02/03/05)

`run` currently ticks before checking queued input, and rebases to now+1ms after
every status batch. Fix the combined production transition, not just the predicates.
Factor a private turn/cycle seam invoked by `run` and deterministic tests, with
injected time/ready-input as needed. Retain std channel32, line8192, max8 inputs,
one rack owner and bounded paused cancellation wait. No scheduler service.

**Binding ordering:** cancellation; service up to8 ready inputs in FIFO order;
quit/EOF/fatal ends immediately with no later command/tick; if still running,
admit at most one due sample and deliver it; wait for deadline/input. Ready pause
wins over a due sample; paused turns admit0. Check cancellation again after waits
and before admitting a sample. A queued flood cannot keep postponing playback.

**Binding pacing interaction:** long file/status/view work may delay wall time;
it cannot erase a sample opportunity that became due while that work ran. Process
that one opportunity, discard accumulated lateness, then set next deadline to the
post-tick current time+1ms. Never replay accumulated missed periods. Starting
from paused starts a new wall schedule without changing the engine cursor;
repeated play preserves engines and cannot suppress an already due opportunity.
Do not force an early sample merely because a cheap command ran. This restores
the original fairness/no-burst contract, not a hardware timing guarantee.

Recommended sketch, not compiled drop-in code (private types may differ):

```text
turn(now, pending_input):
    if cancelled: Exit
    apply at most8 inputs before sampling, preserving any due opportunity
    if quit/EOF/fatal/cancelled: Exit
    if transitioned from paused to playing: establish playback deadline
    if rack.is_running and current_time >= eligible_deadline:
        tick_once_and_deliver()
        next_deadline = post_tick_time + 1ms
    return next wait/deadline
```

Carry the clock across a complete production turn. The old test's isolated
`process_ready_commands` count plus separate `sample_due` calls is insufficient.
Inject fake instants advanced by command/file completion; do not sleep in correctness
assertions. Production sampling/wait uses that same seam. Keep wait responsive
and avoid spinning when no input/sample is ready.

### 3. Deliver native trace and all failures (F-02/F-04; LC-03/05)

In `deliver_drained`, retain one complete caller-owned MIDI drain. Record Generated
once per drained event; when native, send in order and record Sent only after that
send succeeds. Independently write ordinary and explicit reset/pause rack cleanup
trace in drain order for both backends. Recommended per-event shape:

```rust
// Sketch: existing event/context types; not compiled as a replacement.
if let Some(session) = session.as_deref_mut() {
    session.send_performance(event)?;
    rack.record_scope_midi(event, MidiDelivery::Sent);
}
write_trace(trace, event)?;
```

Generated copying may still happen for the whole batch before delivery as now.
On a failed send do not record Sent or continue performance; if trace write fails
after native success, retain that failure and enter common cleanup. This matches
the existing finite runner's send-then-trace ordering. Never claim fallback
Session::finish CCs as newly generated rack events. A broken trace does not
prevent reset/native fallback cleanup attempts.

Replace first-error-only shutdown storage with a small private aggregate that
preserves the first terminal cause and collects subsequent reset/drain/send/
finish/trace-flush/diagnostic-flush failures with stage labels. Existing
`playback::PlaybackFailure` illustrates primary+cleanup Display/source behavior;
its type is private, so do not expose it or alter shared playback just to reuse it.
Recommended `LiveFailure { primary: Option<Box<dyn Error>>, cleanup: Vec<Box<dyn Error>> }`
with Display of every cause and Error::source returning primary. Local equivalent
private representation is fine if causes/ordering survive. Cleanup-only failure
also returns failure. Retain first cause when two failures happen in one tick.
Always attempt cleanup before returning, even if its diagnostics cannot be written.

In `input.rs::Reader::shutdown`, return a private result; when joining a completed
reader, convert panic to a visible worker failure and fold it into main's result
after owner cleanup. Do not block joining still-blocked stdin. In read_input/
deliver_line, detect failed channel send and return from the reader; receiver drop
unblocks bounded sends, and must not trigger further reads. Expected receiver-drop
shutdown is not itself a worker error. In `rack-live.rs::run`, combine control and
reader results without overwriting the first error or returning success for a
finished worker panic. No process::exit before owners drop.

### 4. Repair the narrow proof gaps (F-05; LC-02/03/04)

Extend the existing tests rather than invent another harness. Keep initial
subject/direct/unchanged200+1000 test and basic command/Scope collision regressions.
For actual pause/resume, use real subject/control racks to a dirty nonzero sample;
frontend pause on subject and the equivalent public stop/reset/drain on control,
assert exact cleanup/time then advance both through another1000 samples after
resume, comparing full MIDI and selected Scope. Control handles the same reset
boundary; do not compare silence against an uninterrupted dirty voice. Include
nonempty post-resume events and changing LFO values to reject reseeding/freezing.

For live Scope/save, after200 samples edit through the frontend/direct control,
call scope/save/scope-save only on subject, then advance both1000 samples and compare
full events/Scope. Saved configuration must start at0 and match an independently
configured fresh rack for2000 samples, with nonempty MIDI and divergence from the
unedited fresh fixture (edit Clock90 to make the negative control meaningful).
The prior100-sample empty MIDI comparison does not discharge replay. No exact
engine checkpoint claim. Retain capacity/overwrite/collision checks.

## Behavioral checks and validation

| Contract | Actual path / discriminating oracle |
| --- | --- |
| Observation | Plain --patch supports scope/scope-save without extra flag; selectors/capacity independently accepted. Paused status shows global120, Clock90, four kinds and real params, sorted; sample/capture unchanged. Help gives examples and exact pause/reset difference |
| Fairness | Production turn handles8 queued status/save-like inputs; fake clock becomes late during replies; exactly1 sample then future deadline, even with more queued status. Due pause/quit gives0; paused0; late clock no burst; repeated play cannot starve |
| Native trace | Real loaded rack, successful fake sender,300+ samples and explicit pause: exact native rack event bytes and trace equal trace-only equivalent; Scope Generated all, Sent exactly successful sends. Failing sender produces no Sent for rejected event |
| Error/shutdown | ReadError plus distinct trace-flush failure both visible; dirty trace-write failure after native NoteOn still attempts NoteOff/CC123/CC120/fallback. Broken stderr does not erase primary/cleanup causes; finished reader panic is failure; full-channel quit remains bounded and reader stops after receiver drop |
| Resume/save | Real subject/control/fresh/unchanged comparisons above, nonempty MIDI and full chosen Scope, not only cursor/getter equality |
| Actual CLI | Default and no-graph build, hold stdin open while samples advance; set/get, invalid→valid, help/full status, scope/export without enable flag, save, repeated pause, quit/EOF; bounded Ctrl-C exit with reader blocked. No endpoint audition |

Demonstrate the fairness, native-trace and secondary-error regressions reject
a603a24 (the CRC probes already supply baseline failure evidence; do not repair
source solely to create a negative baseline). Assertions must exercise your
production turn/delivery/shutdown, not unrelated shared-runner tests.

From S, rerun initial affected gates: `cargo test --locked -p oxurack --all-targets`
and same `--no-default-features`; `cargo build --locked -p oxurack --examples` and
same no-default; `make lint`; `cargo clippy --locked -p oxurack --all-features
--all-targets -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p
oxurack --all-features --no-deps`; `make test`; `git diff --check`. Preserve
restricted four CoreMIDI failures then retry exact make test through host approval;
three existing RT ignores and block advisory stay explicit. Do not fix unrelated
cables.rs unused-mut or suppress warnings. Reproduce pinned MIDI2000/LFO10000
hashes in CRC record. CRC's prior native results are independent initial evidence,
not fresh proof of a changed owner loop; unaffected facade runtime/lifetime code
and its prior acceptance remain intact without a new composition campaign.

## Permitted edits and return

Source **only**: `crates/oxurack/examples/rack-live.rs`,
`crates/oxurack/examples/rack_live/control.rs`,
`crates/oxurack/examples/rack_live/input.rs`, `README.md`.
Tests inline in those existing files. No facade/library/shared-playback/workbench/
oracle/fixture/Cargo/lockfile/RT changes. If an inspected dependency contradicts
this frontend-only route, stop affected work with a source-backed alternative to
CRC; do not change architecture or skip an oracle.
Planning: D/slice-plan.md execution notes, D/ledger.md, D/closing-report.md;
optional necessary support only D/artifacts. Preserve initial report/evidence in
dated sections, label this iteration and distinguish observations from claims.
No edits to CRC verdict, parent plans/rows or issued prompts. Keep all five LC rows
open with your new proposed-done evidence; no slice/arc acceptance or02 advancement.

Return intake/readback, actual per-row results and command exits including failed
attempts, self-review/bubble-up, changed paths and separate source/planning hashes.
Explain any private-mechanics variation. No hardware/coverage/soak/tooling expansion.
Commit exact changed authorized paths with `git commit --only -- <named paths>`;
no blanket staging, merge/push or new chat. Duncan relays the returned commits to
CRC for independent disposition. Both initial prompt and this prompt remain immutable.
