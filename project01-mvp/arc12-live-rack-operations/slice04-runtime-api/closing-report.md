# Slice04 CC closing report

## Intake and authority

- Assignment: `cc-prompt.md`, SHA-256 `285eecac303b5aa849f49072b1c56d940af20f1bb6c8b2cb9ee0abb97d14fd60`.
- Source root S: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`, branch `codex/oxurack-module-behavior-installation`, pinned predecessor `deafdd228fb44720878c700f8ff62b2a59ddbad6`.
- Planning root W: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch `planning`; CDC authoring ancestor `67e9150dff5fd45df0f3882c3439ce4ff3e5a3dc`.
- Main R remains protected at `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. At intake all three worktrees were clean; source and planning HEADs were confirmed against the issued packet. No dependent edit preceded this readback.

## Read manifest and source map

The complete issued prompt and its slice plan/ledger were read. Intake followed the prompt's full-file and contiguous-section extents: W `AGENTS.md`; S `CLAUDE.md`; CDC directives08 and12 (directive12 D3 retained only as successor05 boundary); current Project01 direction/handoff through CRC acknowledgement; Arc12 and Arc05 plans/ledgers; escalation Findings/Decisions; collaboration framework 2.6.0 entrypoint; engineering-methods guide01 named sections and guide07; project-management README and guides01/02/03/06; work-verification guides01/02/03/05; testing entrypoint and guides01/03; Rust entrypoint 2.0.0 and complete AP-01…AP-70 chapter; Rust API-02/API-23, EH-01, TD-02/TD-03; CLI-05/CLI-55/CLI-27; source files and exact ranges named in prompt items13–19. Large files were read in bounded chunks, and any truncated output was recovered by narrower reads. Pinned Bevy 0.18.1 source verified `World::try_query` takes `&self`, with read-only `QueryState::iter(&World)`.

Required caller inventory used source root S and `rg -n` over `crates` for `load_patch`, `start`, `stop`, `try_install`, `MidiOutputBuffer`, capacity/reservation/admission, `Panic`, and `SetBpm`; the complete returned match set, including tests/examples outside the allowlist, was inspected. A pre-load test-only stop probe inserts custom resources and phase systems; it must be installed after the candidate load. The module installation fixture registers its own hook before load, which is the intended recipe source. Existing facade examples and persistence/MIDI integration tests exercise the affected callers. No required repair was identified outside the fourteen-path allowlist; graph, cables/labels, MIDI wire, engine modules, fixtures, and manifests remain read-only.

## Contract readback before implementation

At the pinned source, `apply_patch_to_world` can mutate before a late spawn error, and `load_patch` applies into the currently owned World. The new facade therefore builds a fresh configured Bevy App, replays only successful module-type recipes in their original order, and applies the patch there. Core's standalone mutating loader remains explicitly nontransactional; facade rollback comes from dropping a failed candidate. No arbitrary old World resources, instance state, or schedules are cloned.

Lifecycle admission checks `SimulationRunning` first, then pending MIDI count, deliberately ignoring a previously latched output fault. Initial/replacement load performs those checks before file I/O. Restart snapshots the current live configuration only after admission; it never rereads the original file. Candidate validation, spawn, output/fault checks, graph maintenance, clock rebasing, and cleanup reservation/attachment all precede the owner swap. Fallible errors leave the old owner, Scope, output, fault, and recipes intact. Ordinary user panics/globals/non-owned handles are outside rollback guarantees.

Successful public module installs are recorded as an ordered erased recipe only after registry, setter/getter, and hook installation. Same-type install is idempotent; kind conflict returns before mutation. Hooks execute once per App lifetime, including an abandoned failed candidate, with all base resources installed first. Reset is narrower: preserve the current App, running/driver/time/engine/config/Scope and first fault; append voice silence without ticking or reseeding. Restart replaces a loaded App from `snapshot_patch`; unload replaces it with an empty App and clears loaded identity. Successful new lifetimes start stopped at sample0, clear Scope/fault, retain selected driver and installed recipes, and do not call `App::update` or execute a sample.

MIDI output has a separate fixed 128-event storage reservation (80 normal worst-case plus48 cleanup) while its normal logical limit remains five events per configured sink and at most16 sinks. Dirty means any committed ordinary message, including CC; active note implies dirty. Cleanup is staged read-only from the old World, ordered by numeric channel, with NoteOff velocity0 (if active), CC123=0, CC120=0, timestamped at the old completed-sample boundary. Reset checks the whole batch and room before appending, then clears state; replacements copy cleanup into the candidate, never using old Entity IDs against new state. Cleanup remains drainable at zero sinks and must be drained before next-sample admission. Event timestamps are lifetime-local.

The exact source boundary is the fourteen paths in prompt §6. Successor05 handles command facade/removal/queued behavior;07 owns transport;13 owns typed catalog vocabulary. Slice04 handles only the explicit low-level `Panic`/`SetBpm` honesty repair: dispatcher returns concrete `NotImplemented` errors. No lifecycle command API, transport support, teardown hook, schema, dependency, or protected production patch-loader rewrite is in scope. No tactical divergence from the issued contract is planned.

## Evidence and disposition

All six LF rows remain **open**. This report records CC's proposed disposition and does not claim CRC acceptance or close Slice04, Arc12, inherited A-05 rows, or parent release gates.

| LF row | Proposed disposition | Evidence |
| --- | --- | --- |
| LF-01 | CC proposed-done; CRC disposition pending | `tests/lifecycle.rs`: same-channel repeated load; [1,0] succeeds, [1,1] fails without changing snapshot; next channel1 candidate succeeds; late spawner writes resource/entity/ModuleAdded then errors; old Scope/sample/snapshot/message count remain, and 1000-sample subject trace equals control. Structural `Module`/`Port`/`Cable`/`ChildOf`/PatchHandle/CableIndex/graph counts match a separate fresh rack. |
| LF-02 | CC proposed-done; CRC disposition pending | `tests/lifecycle.rs::install_recipes_keep_order_reinstall_and_collision_contract`: hook resource order `[1,2]`, same-type idempotence, collision preflight, and two installed schedules run once each in one sample. Late-spawner fixture records three per-App hook invocations: original install App, successful load candidate, failed candidate. Setter/getter coverage remains in existing module installation and persistence suites. |
| LF-03 | CC proposed-done; CRC disposition pending | MIDI-out unit tests prove channel order, note-off/CC123/CC120 bytes, CC-only/released dirty sink, clean-sink omission, idempotent reset, 128 pre-reserved slots, 16-sink 80+48 peak, unchanged Vec capacity, next-sample backpressure, and full-room error preserving events/voice state for retry. Facade test proves reset while running preserves completed time/state; private fault fixture proves reset retains first fault. |
| LF-04 | CC proposed-done; CRC disposition pending | Lifecycle tests cover running-before-file error, exact pending count admission, repeated empty unload, reload, live snapshot restart with Manual driver retained, Scope clearing only on successful replacement, late failure preservation, and old engine continuation. Getterless `restart()` returns the original snapshot error and retains active PatchHandle/sample. |
| LF-05 | CC proposed-done; CRC disposition pending | `CoreError` has concrete lifecycle variants; dispatcher tests assert Panic/SetBpm `NotImplemented`; docs on facade, module hook and core apply function describe per-App hooks, candidate isolation and low-level partial mutation. `patch.rs` change is documentation only; production validation/apply body is unchanged. Candidate failure test compares old/new message counts. |
| LF-06 | CC proposed-done; CRC disposition pending | `tests/lifecycle.rs::edited_saved_lifetime_replay_matches_fresh_trace_and_scope` compares literal edited parameter/topology/label configuration, replay MIDI and both selected Scope signal streams against a separately loaded saved-patch reference. The CLI produced a 72-row stdout trace equal byte-for-byte to a fresh `turing-machine-midi` run and to no-default-features lifecycle trace. Scope observed 20,000 signals, including effective probability0.5, with zero overwrites; unload cleanup went through the existing drain. Ordinary default mode retained its 300-sample summary. Original fixture outputs were 13/70 rows and matched pinned SHA-256 hashes below. Required suites and native gates are listed in the gate record. |

No row is accepted/closed here. The graph-feature and no-graph outputs agreed; no graph production changes were needed. Arc05 A-05-01/A-05-02, Arc12 R-01–R-04, all parent gates, and release gates remain open. Slice05 command/removal/handle behavior, Slice07 transport, and Slice13 typed vocabulary were not implemented.

## Gates and demonstrations

All commands were run from S on branch `codex/oxurack-module-behavior-installation`. Current source revision is recorded after commit below.

- `cargo test --locked -p oxurack-core`: pass; 424 unit tests, 5 full-cycle integration tests, 9 seeded-context tests, 2 doctests.
- `cargo test --locked -p oxurack-mod-midiout`: pass; 7 unit tests, 0 doctests.
- `cargo test --locked -p oxurack --all-targets`: pass; 48 facade unit tests, all integration suites including 8 lifecycle tests, and 26 `turing-machine-midi` example tests. The lifecycle integration suite was also rerun directly after its final addition.
- `cargo test --locked -p oxurack --no-default-features --all-targets`: pass; 46 facade unit tests and all corresponding no-graph integration/example suites. The final 8-test lifecycle suite also passed directly with no default features. Existing no-graph `cables.rs:231 unused_mut` warning remains outside the source allowlist; no source repair was needed.
- `make test`: restricted run has 4 RT CoreMIDI initialization failures (`MIDI support could not be initialized`), 146 passed and 3 ignored in `oxurack-rt`; unchanged command retried with host access and passed, with 150 passed, 0 failed, 3 ignored. The failures are confined to native enumeration/open tests in read-only `crates/oxurack-rt/src/midi_io.rs`.
- `make lint`: pass; Clippy and formatting checks pass.
- `cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings`: pass.
- `RUSTDOCFLAGS='-D warnings' make docs`: pass after fixing one broken intra-doc link discovered on the first run.
- `cargo build --locked -p oxurack --example save-live-patch --example turing-machine-midi`: pass.
- `cargo build --locked -p oxurack --no-default-features --example save-live-patch`: pass.
- `git diff --check`: pass. `block 0.1.6` future-incompatibility advisory was visible in Cargo output; three existing RT ignored tests were not expanded.

Lifecycle CLI outputs were saved under `/tmp/oxurack-lifecycle.w9TgGe` during execution. Default-feature lifecycle output and no-graph lifecycle output compare equal; default lifecycle and fresh reference compare equal; each trace has72 rows. Scope retained exactly20,000 selected signal observations with zero overwritten records and observed effective `tm_main.write_probability=0.5`. The ordinary example printed its original frequency edit/save/completed-sample summary at300. Expected unchanged outputs: MIDI13 rows SHA-256 `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`; LFO70 rows SHA-256 `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`. Missing patch, malformed RON, and unusable save-directory runs exited1 with useful I/O/parse errors.

The initial compile exposed `resource_scope`'s `Mut<ModuleRegistry>` wrapper and was corrected by borrowing `&registry`. Example compilation exposed a missing `std::io::Write` import, and rustdoc found one unqualified intra-doc link; each was corrected and its gate rerun. Strict Clippy then identified a type-complexity warning in the fresh Scope comparison; named trace aliases resolved it. Intermediate discriminating tests also corrected fixture assumptions: MIDI output did not occur at sample0, a pair of successful module hooks correctly schedules two systems, and replacement cleanup must be drained between real-lifetime repeats. The failing attempts were retained in the work session; no assertion was weakened to accept a leak.

## Boundary, limits, and commits

Final source diff paths were limited to the eleven necessary allowlisted paths: README; core error/event/module/patch; MIDI-out; facade lib/lifecycle/persistence; lifecycle integration tests; and save-live-patch example. `scope.rs` required no change. No fixture, graph/cable production, wire, engine, manifest, lockfile, endpoint, or future-slice path changed. A fresh App drops owned ECS state and reservations; no guarantee is claimed for arbitrary user panic, global state, non-owned handles, or arbitrary Drop behavior. The tested peak is the bounded 16-sink case, and the native host retry is not an endpoint delivery test.

Source commit: `f6fdeb513331a8d8962876bdf4766cedfce63f59` (`feat(oxurack): add explicit rack lifecycle operations`), containing only the eleven listed source paths. Planning authoring baseline: `674c5b3dffb42fd95de52b58297fb5b3ed239f25`; the planning commit records this report and the ledger only. All six LF rows remain open with only CC proposed-done attestation; CRC must independently reproduce and accept before any closure or Slice05 start.

## Iteration01 intake and contract readback

CRC returned the f6fdeb5 implementation for one corrective iteration in
`cc-prompt-iteration01.md` (SHA-256
`bf2e933c80c5274acc98cea7c3f1f9767200c106ad115207f750b51f98da2c0e`). Planning
intake is `2ff534c70271183ac4963eade0616d3d2f67cd96`; source is exactly
`f6fdeb513331a8d8962876bdf4766cedfce63f59`; protected main remains
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Worktree identities, branches,
prompt hash, and clean source/main states were checked before edits. I read the
iteration prompt in full, Arc12 current lifetime sections and child/ledger
status, the full prior Slice04 closing report and CRC findings, Arc05 carryover
ledger, and the applicable skill/guidance material. Required source ranges were
read at their pinned baseline: lifecycle facade and integration suite, MIDI
buffer/consumer/cleanup and tests, save-live-patch example, core event and
patch-apply event ordering, cable/index definitions, GraphView membership and
port projections, Scope record/capture API, and persistence failure fixtures.
No structured data/query source is required by this assignment. The Bevy
dependency/API pin is unchanged; no API expansion or dependency edit is planned.

Readback: this is a test-and-diagnostic correction, not a production lifecycle
repair. Preserve the approved isolated-App owner boundary, operation admission
precedence, fresh lifetime reset and driver/recipe retention, and caller-owned
MIDI drain. Tests must inspect immediate failure and success boundaries across
the complete owner projections, per-App hooks, real reset output, all lifecycle
operations, candidate rejection and typed message payloads, and actual example
dispatch/capture. MIDI tests must call production generation and silence paths,
including CC-only/released state and the literal 80+48 event maximum. The
example must capture the retired sample before unload and report 300. Scope
continues to the six paths named in the iteration allowlist, with tests in the
specified modules; any newly reproduced implementation defect must be reported
before considering an out-of-scope production change. All six LF rows stay open
for CRC disposition, and Slice05 stays held.

## Iteration01 evidence and proposed disposition

This section records the corrective iteration and supersedes the earlier
section's stale source path list, commit identity, test counts, and first-pass
gate summaries. The prior section is retained as historical evidence for the
returned implementation. Iteration source was committed as
`1242a60fa3a997f95b29195de3b28f7e9a2864ff` on branch
`codex/oxurack-module-behavior-installation`; the commit contains exactly the
six source paths in iteration01 §6. No production lifecycle, MIDI module, or
core event behavior was repaired. The example's private dispatcher/reporting
path changed to make execution testable and to retain the retired sample before
unload.

| LF row | Iteration evidence | Proposed disposition |
| --- | --- | --- |
| LF-01 | `tests/lifecycle.rs::repeated_real_patch_replacement_matches_fresh_structure_and_graph` compares sorted modules, ports, cables/order, handles, exact `CableIndex` projections, and graph membership/ports against a separately loaded rack. Late-spawner rejection compares old snapshot, exact membership/index/Scope/sample, typed notification payloads, and continuation trace against a matched control. Output and latched-fault candidates are also rejected before owner swap. After late failure, a valid channel1 replacement succeeds, proving reservation release. | CC proposed-done; CRC disposition pending. |
| LF-02 | The facade replay test verifies ordered hook effects and one actual system execution per sample across load/restart/unload/reload, same-type idempotence, and collision rejection. Failed candidate hooks are checked as candidate-local effects. The controlled Live-driver replacement test proves rebasing without catch-up bursts across all replacement operations. | CC proposed-done; CRC disposition pending. |
| LF-03 | MIDI-out tests exercise production generation and silence paths: literal NoteOff/CC123/CC120 bytes and timestamps, channel order, CC-only and released-note dirty states, clean omission, idempotence, no allocation/capacity change, and room/invariant failure preserving events and voice state. Sixteen sinks generate 80 normal messages then 48 cleanup messages in the fixed 128-slot Vec while logical admission stays 80. Facade reset drains the exact pending note-on plus cleanup; the sink fault diagnostic is separately asserted at its pre-finalization sample boundary. | CC proposed-done; CRC disposition pending. |
| LF-04 | Running and pending-output precedence is checked for load, restart, and unload, with exact pending payload/snapshot/Scope preservation on rejection. Real backpressure faults are produced after a full sample, then each operation is shown to clear fault and Scope only after successful replacement, start at sample0, retain Manual driver, and carry exact cleanup at the retired sample boundary. Existing getterless snapshot failure and controlled live rebase checks remain active. | CC proposed-done; CRC disposition pending. |
| LF-05 | Candidate success checks exact `ModuleAdded`, `CableConnected`, `ParameterChanged`, `LabelChanged`, and `PatchLoaded` payloads/provenance at the active boundary; rejection leaves prior typed queues unchanged. Core `Panic` and `SetBpm` tests confirm `NotImplemented` and no changes to populated World resources, entities, time, running state, or notification queues. | CC proposed-done; CRC disposition pending. |
| LF-06 | Parsed example arguments now reach the actual private dispatcher with captured output writers. Tests cover default summary compatibility, writer error, lifecycle capture, missing/malformed patch and unusable save directory in both modes, and invalid CLI arguments. Scope verifies exactly 20,072 mixed records: 20,000 ordered signal observations and 72 generated MIDI observations byte-equal to the emitted trace, with no overwrite. The runtime check stores sample300 before unload and separately reports new lifetime sample0. | CC proposed-done; CRC disposition pending. |

All six rows remain **open**. CRC must independently reproduce and accept before
any row closure or Slice05 start.

### Final iteration gates

All following commands were rerun from source root S after the final source
edits. `RUSTDOCFLAGS='-D warnings' make docs` was retained from CRC's successful
run at the returned source because this iteration leaves production library
documentation, fixtures, and public API unchanged, as permitted by iteration01.

- `cargo test --locked -p oxurack-core`: pass (425 unit, 5 full-cycle, 9 seeded-context, 2 doctests).
- `cargo test --locked -p oxurack-mod-midiout`: pass (9 unit tests).
- `cargo test --locked -p oxurack --all-targets`: pass (52 facade unit tests, every integration suite including 9 lifecycle tests, 4 `save-live-patch` tests, 26 `turing-machine-midi` tests, and remaining example targets).
- `cargo test --locked -p oxurack --no-default-features --all-targets`: pass (50 facade unit tests and every no-graph integration/example suite, including 9 lifecycle tests and 4 `save-live-patch` tests). The existing `cables.rs:231 unused_mut` warning is outside the allowlist.
- Restricted `make test`: 4 `oxurack-rt` MIDI port tests failed initialization with `MIDI support could not be initialized`; 146 passed and 3 remained ignored. The unchanged authorized host-access retry passed: 150 passed, 0 failed, 3 ignored. The retry does not claim hardware endpoint delivery.
- `make lint`: pass (Clippy and format).
- `cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings`: pass.
- `cargo build --locked -p oxurack --example save-live-patch --example turing-machine-midi`: pass.
- `cargo build --locked -p oxurack --no-default-features --example save-live-patch`: pass.
- `git diff --check`: pass.

The final default-feature lifecycle stdout, no-default-features lifecycle
stdout, and fresh saved-patch runner stdout all contain 72 rows, compare
byte-for-byte equal, and have SHA-256
`20d86b836d7540a2ac6bf725f5045dfe6304fa7a027d703f22e1fe93f27613ba`. Stderr
reports `retired at sample 300; cleanup events=3`, then `new lifetime sample=0`,
and the 10,000-sample replay with 20,000 signal records and 2 final silence
events. The ordinary default example still emits its frequency edit, save path,
and `completed_samples=300` summary. The original MIDI13 and LFO70 trace hashes
remain respectively
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b` and
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.

Planning authoring HEAD at intake was `2ff534c70271183ac4963eade0616d3d2f67cd96`;
this corrective report and the ledger note are committed separately in the
planning worktree. Protected main remains
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Arc05 A-05-01/A-05-02, Arc12
R-01–R-04, Slice04's six CRC dispositions, parent gates, and release gates
remain open; Slice05 remains held.

## Iteration02 itemized intake and contract readback

CRC issued `cc-prompt-iteration02.md`, SHA-256
`9ce58e39fb0fc04df4279ac6131585ec5d38cab2c48d286f82bdf5f4797b5e23`, for
corrective iteration 2 of 5. At intake, source S was clean at
`1242a60fa3a997f95b29195de3b28f7e9a2864ff` on
`codex/oxurack-module-behavior-installation`; planning W was clean at
`abaced3ec260ff0acc6c020a216cb1c953d9f8ac`; protected main R remained
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. The only source paths authorized
are `crates/oxurack/src/lib.rs` (`cfg(test)`),
`crates/oxurack/src/lifecycle.rs` (`cfg(test)`),
`crates/oxurack/tests/lifecycle.rs`, and
`crates/oxurack-mod-midiout/src/lib.rs` (`cfg(test)`). Planning edits are
limited to this report and `ledger.md`. No implementation or example change is
authorized.

| Required read or contract | Exact extent and result |
| --- | --- |
| Iteration02 prompt | Entire 316-line prompt, read in two contiguous bounded chunks; requirements and exact allowlist read back above. |
| Project and Rust operating guidance | W `AGENTS.md`, S `CLAUDE.md`, project management README/guides 02, 03, and 07, work-verification guide05, testing skill/guides01 and03, Rust-guidelines skill 2.0.0 and `guides/11-anti-patterns.md` (all 2,188 lines in bounded chunks). No policy or signature change was needed. |
| Current planning contract and history | Slice plan, current ledger, complete CRC verification, prior closing report, iteration01 prompt sections identified by iteration02, current directive12 required sections, directive08 cited sections, project-plan status, Arc12 plan and ledger, Arc05 ledger. Read in full where required, otherwise to the exact named spans; iteration01 and prior CRC evidence remain historical. |
| Lifecycle and facade test homes | `lifecycle.rs` all 995 lines; `lib.rs` test module spans 522–990 and facade helpers/tests 1,900–2,292, plus production lifecycle facade entry points. The initial broad `lib.rs` output truncated; all required spans were recovered in smaller bounded reads before edits. Existing failed-candidate, hook replay, raw MIDI facade, and actual backpressure test helpers were inspected. |
| Public lifecycle integration home | `tests/lifecycle.rs` all 963 lines, in four contiguous bounded reads. Existing late spawner, graph projection, Scope, restart/replay, and fresh-rack comparison helpers inspected. |
| MIDI output test home | `crates/oxurack-mod-midiout/src/lib.rs` tests 828–1,244 and test landmarks through line 1,643. Fixed-capacity, actual generation, silence, room-check, invariant, and 16-sink tests inspected. |
| Snapshot failure and graph component APIs | `persistence.rs` exact snapshot/failure tests 881–1,018; `cable.rs` Cable/CableIndex 132–211; `port.rs` Port/ChildOf spawning 104–170; `cables.rs` index projection and graph mutation test span 220–280. No new API assumptions introduced. |
| Scope capture, fixtures, and commands | `scope.rs` public capture contract 1–220; behavior-installation and LFO patch fixtures in full; Makefile in full. Only test code will change, so existing trace/docs and example receipts remain eligible for retention. |
| Dataset/query evidence | None used or required. No structured dataset query was run. |

Contract readback: LF-01 needs the old owner checked at the candidate's exact
late error boundary, and cross-World comparisons must use names and complete
graph projections instead of comparing newly allocated Entity bits. LF-02's
ordered registration evidence must be replayed in every replacement App while
real behavior systems prove one execution per admitted sample; restart,
nonempty unload, and reload remain distinct lifecycle paths. LF-03 must attempt
all three lifecycle calls while output is pending and while running, then
assert the complete retained state and exact cleanup bytes before and after
drain; the MIDI peak oracle must inspect all 128 actual events while pending.
LF-04 must distinguish restart from unload/reload, compare failed restart and
empty rack no-op paths at the immediate boundary, reject an actual late
candidate while a real first fault is retained, and resume a fresh sample-zero
note after each successful recovery. Stop/start and reset continuation probes
must compare actual Clock/LFO/Turing Machine/Scope signals and generated MIDI
against a matched control without filtering release events. Any new failing
oracle is preserved and reported through Duncan before an out-of-scope product
repair is considered.

API note: Bevy `Time<Virtual>` in the locked dependency exposes elapsed time and
delta but has no overstep accessor. The interaction snapshot therefore records
Virtual elapsed/delta/pause state and Fixed elapsed/overstep, without adding or
changing a production API. The live-snapshot expected value is built from the
original fixture and independently edited fields, then ordered according to
the snapshot contract; getter-backed schema defaults are included explicitly.

Planned test-only edits follow those contracts in the four allowed files. The
required gates will be run exactly as listed in iteration02, retaining any
restricted-host failure and recording an authorized host retry separately.
LF-05 and LF-06 stay done; LF-01–LF-04 remain open with CC's proposed status
only after all gates pass. Arc05 A-05-01/A-05-02 and Arc12 R-01–R-04 stay open;
Slice05 remains held. No cross-feature or public API work is in scope.

## Iteration02 evidence and proposed disposition

Source S was committed as `03825e5077b7cd621185ae70b47020da321706c4` on
`codex/oxurack-module-behavior-installation`, parent
`1242a60fa3a997f95b29195de3b28f7e9a2864ff`. It contains only the four
iteration02 allowlisted source paths. `lib.rs` and `lifecycle.rs` changes are
inside `cfg(test)` modules; the public lifecycle integration and MIDI module
changes are test-only. No production function, example, fixture, public API,
documentation, or core event behavior changed. Protected main remains
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.

| LF row | Iteration02 interaction evidence | CC disposition |
| --- | --- | --- |
| LF-01 | Inline `CandidateLateFailure` now writes a candidate-only Resource marker before its entity/message/error effects; the retained World is checked immediately after the exact `Err`, before ticks or success. The public late-failure test compares full scalar Scope records/configuration/overwrite count against its matched control before and after 1,000 per-sample drained observations, then retains same-subject channel1 recovery. Cross-World structure projections use module/port names, merge policies, complete ChildOf child rosters/counts, Cable enabled/order, all source/target index endpoints, complete `target_ports`, and named graph membership; raw identities are used only in same-owner checks. | CC proposed-done; CRC disposition open. |
| LF-02 | The inline live-counter fixture runs ordered hook probes and two real behavior instances across load → restart → nonempty unload → original reload. It asserts exact `[1,2]` hook order in each App, totals `2/3`, edited `6/6`, restarted `4/3`, and reloaded `2/3`; per-sample behavior/hook counts, getter state, idempotent reinstall, collision rejection, empty unload, and fresh stopped/time state are checked at each replacement boundary. | CC proposed-done; CRC disposition open. |
| LF-03 | The raw-port facade test checks load/restart/unload precedence while running with one unread event and while stopped with `PendingMidi{count:1}`. It snapshots patch, driver, sample/time resources, Scope selectors/capacity/overwrites/all records, installation order, messages, fault and pending count; reset appends literal cleanup at sample1, a second reset preserves count4 and full typed state, drain matches the exact four-event vector, stop/start alone emits nothing, and retrigger is a lone sample1 NoteOn. Reset is also exercised while running and on an unloaded rack with unread output. The 16-sink test inspects all 128 pending generated+cleanup events at SampleTime2, unchanged storage capacity/no fault/cleared voice state, rejects a private 129th append without mutation, repeats actual silence, and then drains the full timestamped vector. | CC proposed-done; CRC disposition open. |
| LF-04 | Restart now loads a unique LFO patch copy, runs 300 samples, edits parameters/cable/description, independently constructs the full expected snapshot (including getter-backed schema defaults and snapshot ordering), deletes the input file, then restarts and matches a fresh rack over 10,000 scalar/MIDI samples with zero Scope overwrites. Additional checks cover disabled-cable snapshot rejection, getterless failure retention, running/stopped unloaded restart precedence, unread-output initial-load precedence, empty loaded-patch retirement and already-empty reset/unload no-ops with a retained marker/first fault, and nonempty unload cleanup blocking a second unload until drain. A real sample252 backpressure fault is combined with an actual late-spawner candidate failure and immediate old snapshot/owner/Scope/time/driver/message/fault comparisons; each successful load/restart/unload path carries cleanup at 252 and produces a fresh literal sample0 NoteOn without stale NoteOff or fault. Paired Clock/LFO/Turing Machine/Scope controls compare stop/start and reset continuation at sample300 through later MIDI retrigger/CC and a future clock edge, with no release filtering. | CC proposed-done; CRC disposition open. |
| LF-05 | No implementation or event-contract changes were made; prior independently accepted evidence remains unchanged. | CRC accepted; done retained. |
| LF-06 | No example/runner/fixture changes were made; prior independently accepted evidence remains unchanged. | CRC accepted; done retained. |

### Final iteration02 gates

All commands below were run fresh from S after the final source edit. Focused
and required suite results:

- `cargo test --locked -p oxurack-mod-midiout`: pass, 9 unit tests and 0
  doctests.
- `cargo test --locked -p oxurack --all-targets`: pass, 56 library unit tests,
  all 12 integration suites, and all example targets including 4 save-runner
  and 26 MIDI example tests. The lifecycle integration suite passes all 10
  tests.
- `cargo test --locked -p oxurack --no-default-features --all-targets`: pass,
  54 library unit tests, all integration suites and example targets; lifecycle
  passes all 10 tests. The existing unrelated
  `crates/oxurack/src/cables.rs:231 unused_mut` warning remains unsuppressed.
- Restricted `make test`: fails only the 4 `oxurack-rt` CoreMIDI initialization
  tests (`MIDI support could not be initialized`); 146 passed and 3 existing
  tests were ignored. The unchanged authorized host retry passes: RT 150
  passed, 0 failed, 3 ignored; the complete workspace and doctests pass.
- `make lint`: pass (workspace Clippy and format check).
- `cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings`: pass.
- `git diff --check`: pass.

### Iteration02 command attempts retained

- Initial `cargo fmt --all -- --check` exited 1 with formatting suggestions in
  the four edited files; `cargo fmt --all` applied those suggestions. Final
  `make lint` format check passes.
- The first default all-target compile exited 101 on test-harness API/typing
  mistakes: `Time<Virtual>` has no overstep method, `MidiOutputBuffer.events`
  is private outside its crate, `Messages` uses `iter_current_update_messages`,
  and inline tests cannot call integration-only helpers. The test code was
  corrected without product edits.
- The first default test run exposed three test-oracle mistakes: a Debug-string
  snapshot comparison depended on HashMap order, live MIDI configuration has
  no setter, and expected restart/GraphView projections omitted getter-backed
  defaults or confused merge policy with graph value kind. Expected state is
  now typed and independently built; configuration is supplied at patch load;
  projection fields are normalized by names. Final default/no-graph all-target
  suites and both lifecycle suites pass.
- Initial strict Clippy exited 101 on three `type_complexity` findings in the
  graph projection. Local aliases fixed those; final strict Clippy passes.
- The first restricted and final restricted `make test` attempts both exited
  2 with the same four RT CoreMIDI initialization failures. Both unchanged
  authorized host retries pass; no ignore or warning suppression was added.
- The first no-default run reported two graph-only aliases as unused. They are
  now feature-gated; final no-default all-target output retains only the
  pre-existing `cables.rs:231 unused_mut` warning.

Retained evidence remains applicable because this iteration changed only the
allowlisted tests and preserved accepted assertions: CRC's docs and original
fixture evidence at `f6fdeb513331a8d8962876bdf4766cedfce63f59`; CRC iteration01's
candidate/error/message and actual example receipts at
`1242a60fa3a997f95b29195de3b28f7e9a2864ff`; the default/no-graph/fresh 72-row
trace SHA-256 `20d86b836d7540a2ac6bf725f5045dfe6304fa7a027d703f22e1fe93f27613ba`,
ordinary save300 behavior, six negative CLI cases, example builds, original
MIDI13/LFO70 trace hashes, and all previous CRC dispositions. No new dataset
query or supporting artifact was created.

All six rows are accounted for: LF-05/LF-06 remain independently accepted and
done; LF-01–LF-04 remain **open** with CC's proposed-done evidence pending CRC.
Arc05 A-05-01/A-05-02, Arc12 R-01–R-04, project/release gates, and Slice04
acceptance remain open. Slice05 stays held. No acceptance or advancement is
claimed by this return.

## Iteration03 intake and contract readback — 2026-10-13

Prompt identity: `arc12-live-rack-operations/slice04-runtime-api/cc-prompt-iteration03.md`,
SHA-256 `e6b00776e4c29b0926fd740d734bff87e4119d9c761f1262e2e0c533a6609e2c`.
Source S was clean on `codex/oxurack-module-behavior-installation` at
`03825e5077b7cd621185ae70b47020da321706c4`; planning W was clean on `planning`
at `abd98d7055bb0b72f94509846dd2362e5cedfbba`; protected main R remained
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. S is the only source-edit home;
planning changes are limited to this report and the slice ledger.

### Itemized intake and loaded extent

| Order | Input | Loaded extent and use |
| --- | --- | --- |
| 1 | `W/AGENTS.md`; `S/CLAUDE.md` | Both complete. Confirmed planning/source roles, worktree separation, and Rust/testing routing. |
| 2 | Iteration03 prompt; `D/slice-plan.md`; `D/ledger.md` | All complete. Confirmed two test paths, precise reset/control/replay/error oracles, seven fresh gates, split commits, and four accepted/two open row baseline. |
| 3 | `D/crc-verification.md` | Full Iteration02 independent review, Iteration01 Delta/intake/retained evidence, and Fresh independent checks loaded. Findings match the assigned missing first-reset comparisons, reset control, saved replay restoration, and immediate disabled-cable error state. |
| 4 | `P/cdc-directive12.md` | D1 lifecycle semantics, complete D2, and D4 source boundary/discriminating proof/guidance loaded. Reset preserves first fault and clock/Scope/configuration while appending silence; successor05 remains excluded. |
| 5 | `P/cdc-directive08.md` | Complete Prompt quality and Checks/correction policy sections loaded. Applied bounded test repair, actual behavior gates, honest failure retention, and no new proof machinery. |
| 6 | Project and arc plans/ledger | Project top deliverable table and Current direction; arc Capability, Five delivery slices, Current lifetime contract, and full arc ledger loaded. Parent and successor gates remain unchanged. |
| 7–11 | Collaboration-framework and project-management/testing/Rust skills and named guides | Required full skill/guides and named sections were read in the immediately preceding Slice04 iteration turn and reused: collaboration-framework 2.6, project-management README and guides 02/03/07, work-verification guide05, testing skill and guides01/03, Rust-guidelines 2.0 and anti-pattern guide11. AP-12/18, AP-09/42, AP-44 and AP-35 are applied as named by this prompt. |
| 12 | `S/crates/oxurack/src/lib.rs` | Imports/cfg(test) imports; full `set_port_value`; complete `midi_facade_denies_unread_next_sample_before_either_clock_advances`, `pending_output_rejects_lifecycle_and_reset_preserves_then_cleans_exact_state` including `RetainedState`, and `failed_restart_snapshot_and_unloaded_restart_preserve_admission_boundaries` loaded. Existing state representation and exact first-fault/reset boundaries inspected. |
| 13 | `S/crates/oxurack/tests/lifecycle.rs` | Imports/constants/rack; complete current restart and stop/start/reset control tests, capture/replay helpers and aliases loaded. |
| 14 | Source at `1242a60` | Complete `edited_saved_lifetime_replay_matches_fresh_trace_and_scope`, including its test attribute, loaded from that exact predecessor. |
| 15 | Source lifecycle/MIDI-out | Reset/restart/admission/cleanup, `prepare_silence`, `silence_outputs`, `consume_midi_outputs`, `Inputs`, and supporting buffer state loaded. This confirms reset is an enqueue-and-clear operation, not an engine tick. |
| 16 | LFO RON fixture and `S/Makefile` | Both complete. The fixture provides Clock/Turing Machine/LFO/MIDI wiring; native make targets and scope are confirmed. |
| 17 | `D/closing-report.md` | Complete Iteration02 intake/evidence/gates/attempts section loaded; this report is append-only for the new iteration. |

No truncation affected the active prompt, original regression function, or test
contracts; the relevant code was loaded in bounded contiguous sections. No
conditional unfamiliar Bevy API is needed. No dataset query, new artifact, or
new intake file is applicable.

### Contract readback

The first reset must be compared with an owned snapshot taken immediately before
that reset. A baseline captured only after reset cannot detect a first-reset
change because it treats the already-mutated state as expected. Likewise,
resetting both musical arms makes a phase or RNG rewind common to both and
therefore invisible; the control must continue uninterrupted, with only the
single fixture-derived release of the note silenced at sample 300 removed from
its expected MIDI continuation. Reset cleanup stays a separate exact vector.
The current restart test proves reconstruction from the live snapshot at sample
zero, but it does not execute save → continue → unload → drain → reload; those
operations are a distinct accepted regression and both paths must remain.

The disabled-cable restart error comparison must happen immediately after the
exact `UnsupportedSnapshot` result, before reenabling the cable or calling
another operation. It will compare the old owner’s fixed/virtual clocks,
running/driver state, fault, Scope configuration and records, module/cable
identity, installation order, and typed lifecycle messages without taking an
unsupported snapshot in the disabled state. Source changes will remain test-only
and confined to the two paths named by the prompt.

## Iteration03 evidence and proposed disposition — 2026-10-13

Source commit `ec85ebe99c09b8a2ea64d55da0346af4ae317a67`, parent
`03825e5077b7cd621185ae70b47020da321706c4`.
The exact changed paths are `crates/oxurack/src/lib.rs` and
`crates/oxurack/tests/lifecycle.rs`; all changed code is inside named `cfg(test)`
tests/helpers or integration tests. No product implementation, API, fixture,
example, manifest, dependency, lockfile, or protected-main change. Planning
return is recorded separately below.

### Corrected observations

- LF-03 now owns the complete existing `RetainedState` immediately before its
  first stopped reset and compares all state immediately afterward, changing
  only pending events `1→4`. The existing second reset still compares the full
  retained state and drains the literal note0/reset4 vector. The running reset
  after the sample-1 retrigger likewise compares its complete pre-reset state,
  allowing only `0→3`, then asserts cleanup at sample2. The actual stored
  backpressure fault is copied before first reset and compared immediately
  after it; the returned error still reports completed1 while the stored fault
  reports completed0. No tick/drain occurs between each pre/post comparison.
- LF-04 keeps the stop/start control unchanged and resets only the subject at
  sample300. The measured prefix includes `(251,[0x90,62,21])`; cleanup is the
  exact separate vector `(300,[0x80,62,0])`, `(300,[0xB0,123,0])`,
  `(300,[0xB0,120,0])`. The control continues to sample1300. Its unique
  `(376,[0x80,62,0])` is removed once from an owned expected vector; all other
  events, including both CC74/75 values at376, compare exactly. Both arms
  contain `(626,[0x90,64,44])` and compare completely from626 onward, including
  a later NoteOff. All Scope records compare, selector/capacity configuration
  matches, overwrites remain zero, and a future clock edge is observed.
- Restored verbatim with its test attribute from `1242a60` is
  `edited_saved_lifetime_replay_matches_fresh_trace_and_scope`, alongside the
  distinct current-snapshot restart proof. It saves at200, continues100 samples,
  unloads/drains/reloads the file, then compares the 10,000-sample MIDI/scalar/
  copied-MIDI replay with a fresh rack (72 MIDI events, 20,000 scalar records,
  zero Scope overwrites). The restart success test explicitly checks stopped
  behavior (`advance_samples(1)==0`), sample0, Manual driver, no fault, and
  Scope disabled before configuring Scope.
- The disabled-cable restart test captures old-owner state after disabling the
  selected cable and before calling restart. At the immediate exact
  `UnsupportedSnapshot` error it compares fixed and virtual elapsed/delta/pause,
  driver/running/sample, fault, Scope selectors/capacity/overwrite/all records,
  module identities/handle, selected cable identity/endpoints/enabled/order,
  install order, and typed `ModuleAdded`/`PatchLoaded` payloads. Only after that
  equality does the test reenable the cable and perform the existing snapshot
  equality check; no disabled-state snapshot is attempted.

### Iteration03 commands and attempts

Fresh commands from S after the final source edits:

| Command | Result |
| --- | --- |
| Focused `cargo test --locked -p oxurack --lib failed_restart_snapshot_and_unloaded_restart_preserve_admission_boundaries` | pass; 1 test |
| Focused pending-output/reset-state and unread-fault facade unit tests | pass; 1 test each |
| Focused sample300 control, restored save/unload/reload, and current restart integration tests | pass; 1 test each |
| `cargo test --locked -p oxurack-mod-midiout` | exit0; 9 unit, 0 doc passed |
| `cargo test --locked -p oxurack --all-targets` | exit0; 56 facade unit tests, 11 lifecycle integration tests, all other integration/example targets passed (save runner4, MIDI example26) |
| `cargo test --locked -p oxurack --no-default-features --all-targets` | exit0; 54 facade unit tests, 11 lifecycle integration tests, all other integration/example targets passed; existing `cables.rs:231 unused_mut` warning retained |
| Restricted `make test` | exit2; 146 passed, four known `oxurack-rt` CoreMIDI initialization failures, three existing ignored |
| Unchanged authorized host-access `make test` retry | exit0; RT150 passed/0 failed/3 ignored; full workspace and doctests passed |
| `make lint` | exit0; workspace Clippy and format check passed |
| `cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings` | exit0 |
| `git diff --check` | exit0 |

The restricted failures were exactly `test_list_input_ports`,
`test_list_output_ports`, `test_open_input_nonexistent_returns_error`, and
`test_open_nonexistent_port_returns_error`, all reporting `MIDI support could
not be initialized`. The unchanged retry does not erase the restricted result.
The initial focused compile (exit101) exposed a Rust assertion limitation because the
owner observation was one tuple wider than the supported trait arity; grouping
it into typed clock/Scope/owner/message projections fixed the test helper. The
first compile also revealed an obsolete local Scope binding, which was removed.
No product defect was observed. The existing block0.1.6 advisory remains
unsuppressed.

### Six-row disposition and inherited-owner walk

| LF row | CC proposal after iteration03 | CRC status / evidence boundary |
| --- | --- | --- |
| LF-01 | retain done | independently done by CRC at03825e5; no changed assertion area |
| LF-02 | retain done | independently done by CRC at03825e5; no changed assertion area |
| LF-03 | done | open for CRC reproduction of first stopped/running reset and actual first-fault preservation observations above |
| LF-04 | done | open for CRC reproduction of uninterrupted control, restored replay regression and immediate failed-snapshot preservation above |
| LF-05 | retain done | CRC accepted at1242a60; implementation/notification surface unchanged |
| LF-06 | retain done | CRC accepted at1242a60; fixture/example/docs/runner unchanged; affected all-target suites rerun |

The current CC proposal therefore has six rows accounted for: four CRC-accepted
rows retained and LF-03/LF-04 proposed done pending independent review. This
does not close Slice04. Arc05 A-05-01 still carries its later removal/handles/
commands/BPM work, A-05-02 remains subject to final reconciliation, Arc12
R-01–R-04 remain open, project P-01–P-06 and release gates remain open, and
Slice05 remains held.

Retention proposal: preserve CRC strict-docs and original MIDI13/LFO70 fixture
evidence at `f6fdeb5`; preserve CRC accepted LF-05/LF-06, actual example and
core/error receipts at `1242a60`; preserve CRC LF-01/LF-02 and unrelated lifecycle
observations at `03825e5`. These revisions, dates and unaffected claims are
unchanged because this correction touches only test code in the two named paths;
the historical original replay test itself is restored at its exact accepted
source. Fresh mandatory checks above cover both facade modes. No new artifact,
dataset query, checker, or external MIDI witness was needed.
