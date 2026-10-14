# Arc07 Slice01 — independent CRC verification

2026-10-14, Sofie/CRC; Three-Contributor Workflow, Expedited Mode.
**Current verdict: iteration01 accepted at e582aed6e87c234799921cc66a5dd55214f76afd;
LC-01–LC-05 done, Slice01 closed.** Initial failed review below is preserved;
see the dated iteration01 verification for the current evidence and bubble-up.

## Initial review — 2026-10-14 (a603a24)

Source `a603a24d3e02c377557445bd2ef27e4203b33392`; CC planning
`9fdb513a2cc72ea9d4cafdc3c4f063408966e333`, both worktrees clean at intake.
Initial assignment `cc-prompt.md`, SHA256
`db9e7e9501e778a0c38eb92cccc9c4839cf26a5a903eb440a45d150f642a3189`.
Parent source27ca00f is the CDC-accepted Arc12 implementation. CRC authored no
source repair or root-main change. At final state check, root main is clean at
1954fad49898e556f57b15997f306a65ff80c75e, a separate docs-only commit after the
recorded a7ce6e6 baseline. The assigned implementation worktree remains clean at
a603a24; that root documentation update is outside this review and has no code/
fixture/dependency impact on these results.

## Review method and findings

Read the complete issued prompt, current plan/ledger/report and all six changed
source files (new frontend plus README/facade/shared-helper delta). Checked CDC13,
Arc07's retained responsibilities and actual facade reset/BPM behavior. Applied
Rust2.0 EH-01, CA-01/02/07, CLI-27 and AP-33 plus framework2.6 guide07 and testing
behavior/oracle discipline. CC's intake/readback is attestation; native execution
and the following observed contradictions determine this verdict.

### F-01 — high: valid command streams can starve sampling

`rack_live/control.rs:42,64,280`: status sets rebase; every input batch schedules
its deadline one millisecond into the future before checking sampling again.
Continuous status batches repeatedly postpone that future. The helper test at
`each_cycle_caps_ready_commands_and_admits_only_one_due_sample` checks a counter
and clock predicates separately; it never runs the actual combined scheduling
transition. Also, the loop tests a due sample before servicing already queued
pause/quit input, contrary to the assigned input-before-sample ordering.

Reproduced through the actual owner: start a real loaded rack, queue40 `status`
commands then quit in a 64-slot test channel; a diagnostic writer delays each
flush2ms to make the prescribed synchronous-I/O boundary unambiguous. Result:
**0 completed samples**, after processing all40 replies. This is a transient
wall-clock fault reproduction, not a proposed sleeping correctness test or a
latency measurement. The corrective test must inject time into a real production
turn and distinguish command-first ordering, fairness and no catch-up together.

### F-02 — medium: native delivery suppresses its stdout trace

`control.rs:363–368` writes trace only in the no-session branch. Using a real LFO
rack and successful fake MidiSender for300 `tick_once` calls yields one Generated
MIDI event and successful native delivery, but **empty trace**. The assignment
requires ordinary and explicit reset/pause cleanup in trace order in native mode
too. Existing shared playback tests exercise a different runner and cannot
certify the new delivery function. Preserve Sent only after successful send.

### F-03 — medium: specified terminal observation is incomplete

Actual `rack-live --patch .../turing-machine-lfo.ron`, with help/status/scope/
scope-save/quit, exits0 but reports capture disabled. `rack-live.rs:58,69,113`
adds an unassigned `--scope-view` prerequisite; even the error suggests an absent
`--scope-csv` option. Original CLI contract configures capture at startup, defaults
to MIDI-only capture and permits scope-port/capacity without that extra gate.
Status (`control.rs:280`) reports sample/running and instance names only, omitting
global BPM, kinds and current parameters; help lists verbs without the required
examples and pause/reset distinction. Scope/save work when explicitly enabled,
so this is a frontend contract repair, not a missing facade capability.

### F-04 — medium: shutdown silently loses secondary failures

`control.rs:80–100` uses get_or_insert for reset/drain/flush failures. Injected
ReadError `CRC_PRIMARY_READ` plus trace.flush failure `CRC_SECONDARY_TRACE_FLUSH`
returns only `stdin read failed: CRC_PRIMARY_READ`; the cleanup cause disappears.
The contract requires primary plus cleanup failures even with broken diagnostic
output. Statically, `input.rs:53` also discards completed-reader join failure;
shutdown returns no result, so that failure cannot affect executable status.
The permitted blocked-stdin detach exception does not authorize suppressing a
completed worker panic. No real terminal panic was reproduced; this latter
point is a source-grounded error-propagation gap. `deliver_line` also ignores
send failure and resumes reading: return promptly when the receiver is gone.

### F-05 — medium: continuation/replay claims need discriminating proof

No product defect reproduced for pause/resume phase or save/fresh replay.
However, the new pause/resume test restarts and resets without actually advancing
and comparing resumed engines; it checks time/running only. The save replay test
at `control.rs:1019` compares100 startup samples, before this edited Clock's first
MIDI event, so two empty vectors pass. Its post-save live continuation lacks an
independent control. Bounded-Scope tail checks are not that control.
The report's new-runner Sent/flush assertions are likewise stronger than its tests:
shared finite-runner tests are useful regressions but do not exercise live delivery;
FailingWriter fails both write/flush and an empty first tick only proves a flush
failure, not cleanup after a dirty trace-write failure. Extend existing real-rack
checks narrowly; no new verifier platform or proof campaign is required.

## Independent execution receipts

Commands ran from the source worktree at a603a24, without source changes:

| Check | Result |
| --- | --- |
| `cargo test --locked -p oxurack --all-targets` | exit0; facade64, rack-live31, old MIDI example26; all groups pass |
| Same `--no-default-features` | exit0; facade62, rack-live31, old MIDI example26; all groups pass; pre-existing unused-mut cables.rs:231 |
| `cargo build --locked -p oxurack --examples`, same no-default | both exit0 |
| `make lint` | exit0 |
| `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings` | exit0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p oxurack --all-features --no-deps` | exit0 |
| `make test`, restricted | exit2; RT146 passed/4 known CoreMIDI initialization failures/3 ignored |
| Exact `make test`, approved host retry | exit0; RT150 passed/0 failed/3 existing ignores; workspace/doctests pass |
| `git diff --check`, source | exit0 |
| Three added transient behavior probes against copied unchanged production source/current Cargo-JSON rlibs | compile retry exit0; all3 behavioral assertions fail, exit101: F-01/F-02/F-04 |

The first transient rustc attempt lacked a borrow on load_patch's Path argument;
that harness compile error was corrected, not counted as a source failure. No
changed production logic was substituted into the probes. Their setups/results
are described above; temporary source/results live under
`/tmp/oxurack-crc-arc07-slice01-witness`. Native logs use
`/tmp/oxurack-crc-arc07-slice01-*.log`. These are supporting scratch traces, not
new permanent tooling or prerequisites to reconstruct the method.

Actual subprocesses: default explicit-Scope run holds stdin open, plays, sets
Clock90/TM.25/LFO1, rejects NaN then reads back1, pauses at841, repeats status at841,
saves/exports and quits0:6 trace rows,848 CSV lines. No-default explicit-Scope
smoke also exits0, pause stable232/232 (before the first note; not claimed as a
musical-output witness). Plain EOF exits0. An early Ctrl-C attempt exits0 with
zero rows (not dirty cleanup proof); a second attempt waits for the first note,
then SIGINT with stdin still open exits0 and emits exactly:

```text
251|90 3E 15
256|80 3E 00
256|B0 7B 00
256|B0 78 00
```

Fresh finite MIDI2000:13rows,
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
LFO10000:70rows,
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Both pinned musical traces remain intact. `block v0.1.6` future-incompatibility
advisory remains; no warning suppressed. No MIDI endpoint was opened.

## Five-row disposition and bubble-up

| Row | Reproduced supporting evidence | Unresolved / disposition |
| --- | --- | --- |
| LC-01 | Real Clock/LFO/TM command/getter/save values and invalid-input recovery; native inline tests + actual CLI | open: F-03 complete status/help contract |
| LC-02 | 200+1000 subject/direct/unchanged test and blocked-reader playback pass | open: F-01 actual turn fairness/admission; F-05 resumed engine continuation |
| LC-03 | Literal pause cleanup, repeated pause and actual dirty SIGINT cleanup pass | open: F-02 native trace; F-04 error propagation; F-05 production delivery/fault proof |
| LC-04 | Bounded Scope and export collision pass when enabled; actual save/export | open: F-03 default capture; F-05 nonempty fresh replay and live control continuation |
| LC-05 | Feature/native/lint/docs/build gates and described actual CLI smokes pass | open: affected real-turn/CLI/native/error regressions required before acceptance |

All five counted, no deferred/no-op rows and no acceptance yet. Initial CC report
and original prompt are preserved. No artifacts outside the canonical planning
home were committed. Corrective iteration01 targets these frontend defects and
proof gaps only. The arc breakdown/architecture and directive13 residual owners
remain valid; no plan amendment or CDC escalation is needed for these in-scope
repairs. Slice01 and both Arc07 rows remain open. Slice02 remains held; CDC's
observer/lifecycle/metadata design return is due only after Slice01 closure.

Final handoff metadata reconciliation, before relay: the unrelated root-main
README update was identified after review commit c29f29a. Corrected the unrelayed
iteration01 draft's root-state note without changing its source pin, scope or
oracles. No second iteration was issued or executed; original cc-prompt.md remains
byte-identical. All worktrees are clean after the scoped metadata follow-up.

## Iteration01 independent verification — 2026-10-14

**Accepted source:** `e582aed6e87c234799921cc66a5dd55214f76afd`.
CC planning return: `518897ed5978e9e6b3fece776a1fe8bb1ca63537`, clean at intake.
Assignment: `cc-prompt-iteration01.md`, SHA256
`91b126d3a2c7de111584ba8719e3c6361afb93446895663c5b943fc235fb6a74`.
Initial prompt retains its db9e7e95… SHA above; both match issuance39b8427 bytes.
One corrective pass executed1/5. No source repair by CRC. Root main is separately
clean at docs-only cd3119ca7ca3884d4a6705904cb1dc42f9d87bad; source checkout
stays the named implementation worktree, not that documentation branch state.

### Delta, intake and independent results

Reviewed the full four-path delta a603a24→e582aed: README, rack-live.rs,
rack_live/control.rs and input.rs only. No facade/core/engine/RT/shared-helper/
fixture/dependency change. Read CC's required-reading/readback and correction
history: CC disclosed an early draft patch restored before completing intake,
then reapplied after readback. Final scope/contract is coherent; reading receipts
remain attestation. This review checks behavior rather than inferring it from
that receipt or the report's green gate summary.

The real owner_turn services up to8 inputs before one sample opportunity, tests
cancellation and preserves due time across repeated play/status. Only a paused→
running transition establishes a fresh deadline; post-tick scheduling discards
accumulated lateness. Default capture/status/help follow the assigned observation
contract. Native delivery writes trace after successful send and marks Sent only
for success. LiveFailure preserves primary and later cleanup stages, and completed
reader panic now reaches the executable result. Receiver drop stops the reader.

Recompiled the **same three initial CRC probes**, keeping their expectations
unchanged and substituting only the copied current production source plus fresh
Cargo-JSON-resolved rlibs. All3 now pass (compile0, tests0). This independently
reproduces repair of the status-flood starvation, native empty trace and discarded
secondary trace-flush failures. The transient sleeping writer remains a failure
reproduction; the native production-turn tests supply deterministic fake-time
correctness checks. No permanent probe or verification framework was added.

Fresh commands from S at e582aed:

| Command / check | Actual result |
| --- | --- |
| `cargo test --locked -p oxurack --all-targets` | exit0;20 target groups,212 passed/0 failed; facade64, rack-live41, old MIDI example26 |
| Same `--no-default-features` | exit0;20 groups,210 passed/0 failed; facade62, rack-live41; existing cables.rs:231 unused-mut warning |
| `cargo build --locked -p oxurack --examples`, same no-default | both exit0 |
| `make lint` | exit0 |
| `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings` | exit0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p oxurack --all-features --no-deps` | exit0 |
| `make test`, restricted | exit2; RT146 passed/4 known CoreMIDI init failures/3 ignored |
| Exact `make test`, approved host retry | exit0;35 reported groups,1051 passed/0 failed/3 existing ignores, including RT150 and workspace/doctests |
| `git diff --check` source and scoped planning | exit0 |

Counts are the printed groups of each named invocation, not coverage percentages
or independent populations to add across runs. Existing block v0.1.6 advisory and
three RT ignores remain explicit. No warning suppression or endpoint opening.
Logs: `/tmp/oxurack-crc-arc07-iteration01-*.log`; copied-source probe and actual CLI
outputs under `/tmp/oxurack-crc-arc07-iteration01-witness`. These are supporting
scratch, not durable dependencies. Existing native tests are the repeatable checks.

Actual default CLI, **no --scope-view**: startup paused0 with stdin open; help/
status/Scope; play until real MIDI; reject NaN then Clock90/TM.25/LFO1; getters;
pause and repeated pause/status570/570; Scope/save/export and collision; clean
quit0. Trace6 rows, MIDI-only CSV7 lines, default capacity100000. Full status shows
Clock90 beside global120 and sorted module kinds/parameters. Export collision
returns an error and later getter still succeeds. Stdout contains only trace rows.
Actual no-graph CLI selects lfo_main.bipolar_out/capacity16 **without an enable
flag**; play until a real note, edit/get, pause/status258/258, Scope/export/quit0:
4 trace rows,17 CSV lines and honest overwritten records. Idle input remained open.
Plain EOF exits0. Dirty SIGINT while stdin stays open exits0, with:

```text
251|90 3E 15
254|80 3E 00
254|B0 7B 00
254|B0 78 00
```

Actual saved file from nonzero live sample570 and a separately edited original
fixture each replay fresh for2000 samples:8 nonempty MIDI rows, identical SHA256
`307f3ae91bb6c2e76b5736e8c9cb25ae8db0ff7fb945106eac0f5d0b997e9830`.
The independent fixture applies Clock90/TM.25/LFO1 to original patch text rather
than copying the saved file. Native real-rack test additionally compares full
Scope and negative unchanged-control replay; pause/resume matches public
stop/reset/drain control through1000 subsequent samples with changing LFO values.

Fresh current example builds retain MIDI2000/13 rows and LFO10000/70 rows:
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b` and
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
These are fresh runs; initial historical evidence is retained at its original
revision/date only for unchanged facade is_running and prior runtime contracts.
All affected frontend claims were exercised freshly, not accepted by retention.

### Final five-row walk

| Row | Independently reproduced verifier and result | Disposition |
| --- | --- | --- |
| LC-01 | Typed real command/getter/save and invalid-input recovery suites; actual NaN→valid/getter, complete sorted status/help with global120 versus Clock90 | done at e582aed |
| LC-02 | Actual owner_turn fake-time eight-input/due pause/quit/one-late-sample/repeated-play tests; original flood probe now passes; blocked-reader playback and real1000-sample resumed control/Scope | done at e582aed |
| LC-03 | Native/trace-only parity including pause, Generated/Sent accounting, failing sender without Sent, literal dirty cleanup, combined failure probe, completed worker panic and bounded shutdown tests; actual EOF/SIGINT | done at e582aed |
| LC-04 | Default/selected bounded capture, export collision and live Scope/save continuation to1200 with control; nonempty2000-sample independent config replay plus unchanged negative control; actual CLI save/fresh replay | done at e582aed |
| LC-05 | Both feature/all-target/build/real CLI modes, current lint/strict docs/Clippy, unchanged host retry and pinned finite musical traces | done at e582aed |

F-01–F-05 resolved;5/5 rows done,0 deferred,0 no-op, no remaining reproduced
Slice01 defect. Initial failed review/report/prompt and correction history remain
preserved. Slice01 closed after one correction; not Arc07 or project acceptance.

### Bubble-up to Arc07

The delivered increment is a real terminal instrument: paused startup, continuous
single-owner playback despite waiting input, live scalar edits, honest status/
Scope/save and explicit endpoint opt-in with orderly silence. Source/ledger/report
now support the promised Slice01 capability. No new architecture/arc restructuring
was required by the repairs; the original four-increment outline remains approved.
CC's close report describes delivery and retained boundaries; its artifact inventory
is reconciled here: only canonical slice records, no durable supporting artifacts.
No hidden test harness, new source dependency or release/timing claim was added.

The silent-drop walk distinguishes remaining Arc07 work from delivered01:
lifecycle/topology/metadata editing, ordered watch across patch replacement,
per-note/authorship/tag-query representation, REPL completion, controllers/config
and external transport remain named successors. A-07-01/A-07-02 and P-01–P-06
remain open; P-07 retains prior acceptance. Directive13 explicitly requires CDC's
observer/lifecycle/metadata decisions **before**02 implementation authoring.
CRC inspected those seams at current source and returns the concrete questions
in [crc-escalation14.md](../../crc-escalation14.md). No Slice02 CC assignment is
issued. This design gate is unrelated to the now-closed corrective loop.
