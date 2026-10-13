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
