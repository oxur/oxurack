# Slice04 — independent CRC review

2026-10-12. Sofie/CRC. Three contributors, Expedited Mode.

Current: iteration01 reviewed at1242a60; LF-05/LF-06 done, LF-01–LF-04 open.
See the appended Iteration01 independent review. Active correction: iteration02.

## Initial candidate, authority and disposition

Reviewed source `f6fdeb513331a8d8962876bdf4766cedfce63f59`, against accepted
predecessor `deafdd228fb44720878c700f8ff62b2a59ddbad6`; CC planning return
`622e9948c443748bddbdc9c944b50aa5398fc1b1`. Both worktrees were clean at intake.
Main remains `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
The issued `cc-prompt.md` (SHA-256
`285eecac303b5aa849f49072b1c56d940af20f1bb6c8b2cb9ee0abb97d14fd60`),
directive12 D1/D2/D4 and its narrow D3 honesty exception govern acceptance.

**Return for one collected corrective iteration. Six LF rows remain open.**
Native gates and the actual musical recipe reproduce successfully. The remaining
acceptance gaps concern explicitly assigned interaction tests, not a reproduced
failure of the lifetime implementation. One minor diagnostic defect is observed.
No architecture amendment, production lifecycle repair, Slice05 opening, or
ancestor acceptance is claimed. The next assignment is
[cc-prompt-iteration01.md](cc-prompt-iteration01.md), correction1/5.

## Source and intake review

CRC inspected the entire eleven-path delta, new lifecycle implementation, MIDI
buffer/consumer/cleanup, all eight lifecycle integration tests, affected inline
fixtures and the actual example. All eleven changed paths are inside the issued
fourteen-path allowlist. `oxurack-core/src/patch.rs` changes documentation only;
its loader/validator bodies, fixtures, manifests, lockfile, graph production,
standalone engines and native endpoint code are unchanged.

The isolated candidate, ordered installation recipes, typed stopped/pending
checks, complete cleanup staging and sole caller-owned drain fit CDC12. Preparing
read-only old cleanup before candidate construction is a tactical ordering
difference from the sketch; it does not mutate the old owner and no defect is
established from that difference. Ordinary returned errors all precede App swap.
The panic/global-effect exclusions are documented rather than disguised as rollback.

CC's intake/readback is attestation in its closing report. It captures the main
design, but its return overstates test coverage for LF-02–LF-05. In particular,
the single installation test observes one successful load, and the fault test
injects a synthetic fault into an unplayed rack; neither establishes the full
reported interaction matrix. Passing gates do not fill those coverage gaps.

## Fresh independent commands

All commands ran in the assigned source worktree at f6fdeb5, without source edits.
Transient complete logs: `/tmp/oxurack-crc-slice04-18qxy8th/`; these are convenient
local receipts, not a durable dependency for acceptance. The outcomes below and
source tests are the retained review record. Gates were run sequentially.

| Command | Independent result |
| --- | --- |
| `cargo test --locked -p oxurack-core` | exit0;424 unit,5 full-cycle,9 seeded-spawn,2 doctests passed |
| `cargo test --locked -p oxurack-mod-midiout` | exit0;7 unit passed |
| `cargo test --locked -p oxurack --all-targets` | exit0;48 facade unit,8 lifecycle integration, all other integration/example targets passed |
| `cargo test --locked -p oxurack --no-default-features --all-targets` | exit0;46 facade unit,8 lifecycle integration, all other integration/example targets passed |
| `make test`, restricted | exit2; RT146 passed,4 CoreMIDI initialization failures,3 pre-existing ignores; failure retained |
| unchanged `make test`, host access | exit0; RT150 passed,0 failed,3 existing ignores; workspace suites passed |
| `make lint` | exit0 |
| `cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings` | exit0 |
| `RUSTDOCFLAGS='-D warnings' make docs` | exit0 |
| `cargo build --locked -p oxurack --example save-live-patch --example turing-machine-midi` | exit0 |
| `cargo build --locked -p oxurack --no-default-features --example save-live-patch` | exit0 |
| `git diff --check` | exit0 |

The four restricted failures are `midi_io::tests::{test_list_input_ports,
test_open_input_nonexistent_returns_error,test_list_output_ports,
test_open_nonexistent_port_returns_error}` in oxurack-rt. Their host retry is not
relabeled as a restricted pass. Existing block0.1.6 future-incompatibility and
no-default unused_mut advisory remain visible; no warnings/tests were suppressed.
The 26 example tests belong to `turing-machine-midi`, not `save-live-patch`;
the latter currently executes zero tests in both facade all-target matrices.

## Fresh musical and CLI observations

Executed the issued section7 cargo-run commands with transient saves under the
receipt directory. The default lifecycle, no-default lifecycle and a separate
fresh `turing-machine-midi --samples 10000` run of the saved patch all exited0
and matched byte-for-byte:72 rows, SHA-256
`20d86b836d7540a2ac6bf725f5045dfe6304fa7a027d703f22e1fe93f27613ba`.
Lifecycle stderr reported3 retired cleanup events,20,000 scalar Scope records,
and2 final silence events; its internal zero-overwrite check passed. Source
inspection confirms explicit copied-MIDI recording. This command does not
independently inspect every retained MIDI record or port/sample pairing.

A separate fresh saved-patch run with `--scope-csv`, both named `--scope-port`
selectors and `--scope-capacity 21000` exited0. CRC inspected its complete CSV:
20,000 signal and72 midi_generated rows, zero overwrites, write_probability0 at
sample0 then.5 at every sample1..9999. This establishes the fresh-reference
observation; it does not substitute for checking the lifecycle recipe's own
capture. The new prompt uses this source-supported publication boundary.

The ordinary save example exited0 with the frequency edit/save lines and
`completed_samples=300`. Original MIDI2000 remained13 rows with SHA-256
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`;
original LFO10000 remained70 rows with SHA-256
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Missing patch, malformed temporary RON, and nonexistent save parent each exited1
with empty stdout in ordinary and lifecycle modes (six negative invocations).
No MIDI endpoint was listed, connected or played.

## Findings and required correction

All source anchors below are at f6fdeb5. These are serious proof gaps (F-01–F-05),
with a minor diagnostic issue in F-06; they do not establish latent production
failures. The prescribed iteration reuses current tests rather than a new verifier.

1. **F-01 / LF-01 — full failure and membership observations are incomplete.**
   `tests/lifecycle.rs:375` projects aggregate counts and handle names, not exact
   port/cable/index/graph membership. The late-failure test at:283 checks Scope
   count, then musical continuation, but never compares scalar captures. Its
   message check after1000 more samples compares only ModuleAdded lengths, not
   payload preservation or other typed queues. Observe payloads immediately after Err, complete
   normalized membership and Scope/control projections, and prove a subsequent
   channel1 load after that same late failure.
2. **F-02 / LF-02 — per-App replay is tested only across load.**
   `install_recipes_keep_order_reinstall_and_collision_contract` at:354 consumes
   the rack after one load/tick. It does not cover restart/unload/reload, live
   current-App reinstall/collision state, accessors after each lifetime, or
   execution counts after reconstruction. Reuse ordered hooks and the existing
   LiveBehaviorCounter fixture with inline private access.
3. **F-03 / LF-03 — low-level staging tests do not prove actual reset emission.**
   The peak test at MIDI-out:991 invokes `append_silence` directly, leaving dirty
   state uncleared; it checks length, not the80-event prefix/48 cleanup bytes and
   boundaries or second reset. Channel/released tests hand-set dirty state, so
   they cannot catch omission of production CC-only/NoteOff dirty marking.
   The facade test at lifecycle integration:169 tests presence of controls after
   draining all normal output, not the literal unread NoteOn→cleanup sequence.
   Exercise actual normal production and silence/reset, preserving clocks,
   Scope, fault and voice state, plus an invariant failure distinct from room.
4. **F-04 / LF-04 — admission/recovery/restart/Live matrix remains unproved.**
   At integration:144/:224 only load has running/pending rejection assertions.
   The restart check at:169 reads one parameter but does not delete the original
   file or compare fresh edited seeded replay. Inline lifecycle:180 injects a
   synthetic fault in an unplayed rack and recovers only via load. No new test
   exercises Live lifetime reconstruction, real-fault reset/recovery across all
   three operations, empty-cleanup/no-op fault preservation, or actual musical
   stop/reset continuation. Existing pre-lifecycle driver tests do not cover App
   replacement. Fill the explicit initial-prompt section5 matrix.
5. **F-05 / LF-05 — candidate output and lifetime message boundary are untested.**
   `validate_candidate` at lifecycle:140 has no rejecting-output/fault witness
   through a facade load. A post-continuation ModuleAdded length comparison
   does not establish fresh message payloads/provenance or absence of retired
   removal messages. Inspect all existing notification types immediately before
   and after success/failure. Check PatchLoaded-last by the unchanged loader's
   write ordering; separate Bevy typed queues are not a global event sequence.
6. **F-06 / LF-06 — executable recipe tests and complete capture checks missing.**
   `save-live-patch.rs` has no tests; `edited_saved_lifetime_replay...` at:439 is
   a parallel recipe. Its capture helper discards MIDI records and accepts any
   one effective-probability observation. Test actual parsed mode/recipe and
   sample/selection coverage, copied MIDI and settled probability. The actual
   CLI says `retired at sample 0` (example:116) by reading the new lifetime after
   unload; the retired rack completed300. Preserve that boundary before unload
   and report300 accurately, separately checking fresh sample0.

## Row walk, inventory and bubble-up

| Row | Independently reproduced part | Remaining owner / disposition |
| --- | --- | --- |
| LF-01 | Same-channel replacement, valid[1,0], rejected[1,1], late spawner Err and continued MIDI; source isolation | F-01 complete owner projections/control observations; open |
| LF-02 | One-load hook order, same-type/collision initial checks, two systems once in one tick | F-02 multi-lifetime/current-state/accessor proof; open |
| LF-03 | Reservation128, channel ordering, room-failure retry, cleanup lengths/idempotence in partial fixtures | F-03 real reset/dirty/maximal bytes/timestamps/phase/fault/atomic invariant proof; open |
| LF-04 | Load-only busy precedence, current parameter after restart, getterless Err, basic unload/reload, synthetic fault recovery | F-04 complete operations/fresh reconstruction/real fault/Live/continuation; open |
| LF-05 | Typed variants, exact core unsupported errors, documented hook/loader boundary; unchanged loader body | F-05 candidate rejection and complete immediate notifications; open |
| LF-06 | Both actual CLI modes/fresh replay, ordinary mode, six failures, original pins and every native gate | F-06 executable automated/capture checks and diagnostic; open |

Artifact inventory: original prompt preserved; slice-plan/ledger remain canonical;
CC closing report retained unchanged by CRC; new CRC record and iteration01 are
the only new slice artifacts. No permanent receipt forest/new checker. No silent
drop: all six rows and original section5 obligations map to this correction.
Arc05 A-05-01/02 remain deferred: configuration/metadata/cables are accepted
in01–03; lifecycle is awaiting this review; removal/handles/commands/global BPM
remain05, separate transport07 and catalog forms13. R-01–R-04 and project/release
gates stay open. Findings remain within the approved design, so no new CDC design
return is needed. Slice05 remains unopened until Slice04 is independently accepted.


## Iteration01 independent review — 2026-10-12

Current verdict supersedes the initial review above **only for this candidate**.
Source `1242a60fa3a997f95b29195de3b28f7e9a2864ff`; planning return
`9081268e92c70c1fc8269927399056de9cfa4af8`; active reviewed assignment
`cc-prompt-iteration01.md`, SHA256
`bf2e933c80c5274acc98cea7c3f1f9767200c106ad115207f750b51f98da2c0e`.
Clean R/S/W and protected main a7ce6e61536ca96b6aeb2227ee06e430ed8df162 checked.

**LF-05 and LF-06 independently done; LF-01–LF-04 remain open.** Return for one
collected test/evidence iteration02, correction2/5 after one executed correction.
Slice04 is not closed and Slice05 remains unopened. No new product defect or
architecture amendment is claimed. The actual example diagnostic is repaired.

### Delta, intake and retained evidence

Read the actual six-path correction and affected tests/example; inspected current
owner/admission/commit/cleanup code, private test helpers, read-only snapshot
failures, typed notifications and source-supported fixtures. All six changed paths
match iteration01's allowlist. Byte comparison of each changed library file's
prefix before cfg(test) against f6fdeb5 is equal: core event, MIDI-out, facade lib
and lifecycle. Remaining paths are allowed integration tests and private example
factoring/capture/diagnostic checks. No protected source, fixture, lockfile,
dependency, standalone engine, core loader body, graph or transport change.

CC's intake/readback attests the expected boundary and skill use, but is grouped
rather than itemized by loaded extent. Its LF-01–LF-04 completion prose exceeds
observable tests as detailed below. CRC does not infer comprehension from the
reported reading or green gates. Iteration02 asks concise itemized extents and
explicit unresolved observations in the same existing record, no new checker.

Retain CRC2026-10-12 f6fdeb5 strict-docs success and original MIDI13/LFO70 hashes
from this record's initial reproduction. Their inputs, library production/docs,
fixtures, runner and dependencies are unchanged and the correction cannot affect
those properties. They are retained at that historical revision/date, not called
fresh runs. Current example demonstrations and affected native gates were rerun.

### Fresh independent checks at1242a60

| Command from S | Result |
| --- | --- |
| cargo test --locked -p oxurack-core | exit0;425 unit,5 full-cycle,9 seeded,2 doc passed |
| cargo test --locked -p oxurack-mod-midiout | exit0;9 unit passed |
| cargo test --locked -p oxurack --all-targets | exit0;52 facade unit,9 lifecycle integration,4 save-live-patch,26 runner and other targets passed |
| cargo test --locked -p oxurack --no-default-features --all-targets | exit0;50 facade unit and corresponding integration/example suites passed |
| make test, restricted | exit2; RT146 passed,4 known CoreMIDI initialization failures,3 existing ignores |
| unchanged make test, host access | exit0; RT150 passed,0 failed,3 existing ignores; workspace passed |
| make lint | exit0 |
| cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings | exit0 |
| cargo build --locked -p oxurack --example save-live-patch --example turing-machine-midi | exit0 |
| cargo build --locked -p oxurack --no-default-features --example save-live-patch | exit0 |
| git diff --check | exit0 |

Transient receipts: `/tmp/oxurack-crc-slice04-iteration01-sk21njoa` gate00–09,
separate host-make-test.log and CLI stdout/stderr. Canonical results are here;
no durable supporting artifact/checker added. Known block0.1.6 advisory and
existing no-graph unused_mut remain visible; no warning suppression/new ignores.

Actual cargo-run save-live-patch commands used original LFO fixture and unique
transient saves: default --lifecycle; --no-default-features --lifecycle; separate
fresh turing-machine-midi using default saved patch --samples10000. Each exit0,
72 rows, identical bytes/SHA256
`20d86b836d7540a2ac6bf725f5045dfe6304fa7a027d703f22e1fe93f27613ba`.
Both lifecycle diagnostics report retired300/cleanup3, new0, replay10000,
Scope20000/final silence2. Ordinary mode exit0/frequency.5→1/completed_samples300.
Direct final executable missing patch, malformed temporary RON and missing save
parent each exit1/empty stdout in both modes (six cases). No endpoint operation.

### Remaining findings — existing iteration01 obligations only

All anchors below qualify source1242a60. These are incomplete required proof,
not newly invented hardening or evidence of a production failure.

- **F-01 / LF-01:** integration `late_spawner_failure_isolated_and_old_engine_continues`
  at tests/lifecycle.rs:364 compares MIDI but Scope count, not scalar control
  records. CandidateMarker absence is observed after later successful replacement,
  so a failed candidate resource leaked into the old owner could pass. Inline
  CandidateLateFailure:440 writes an entity/message but no resource. New inline
  immediate owner/index/graph/messages/Scope comparisons are useful and retained.
  Cross-World structure at integration:481 includes raw module Entity bits and
  omits merge/enabled/complete target roster: normalize same-World identities and
  complete the prescribed membership observations, keeping current graph checks.
- **F-02 / LF-02:** integration ordered hooks:461 still ends after one App.
  lib.rs:877 multi-lifetime test has one counter type and alpha-only assertions;
  it does not observe ordered two-hook replay in every new App, both configured
  instances or same-type reinstall inside that edited lifecycle sequence. Current
  once-per-App execution/getter/collision and Live timing checks pass and stay.
- **F-03 / LF-03:** MIDI-out maximal test:1083 now exercises actual silence and
  full literal128 events, plus real CC-only/released and invariant cases pass.
  However its full-buffer overflow assertion was removed and pending no-fault/
  SampleTime/capacity observations are after drain. The facade unread-note reset
  at lib.rs:1992 establishes first-fault retention and literal four events, but
  not repeated actual reset before drain, full preserved state or raw retrigger.
  Complete those small boundary observations; retain the valid new low-level tests.
- **F-04 / LF-04:** integration current restart:157 deletes a temporary file but
  checks one parameter; the complete seeded edited replay:784 uses unload/reload,
  not restart. No actual stop/start/reset scalar-and-musical control continuation
  or disabled-selected-cable restart failure/empty-loaded versus empty-unloaded
  no-op fault witness is present. Running-all-operations at integration:236 occurs
  with no pending output. Real raw-fault test lib.rs:2134 correctly proves exact
  two-event backpressure/recovery for all three operations; it skips failed actual
  candidate after draining the real fault and fresh sample admission after cleanup.
  Accept the two-event fixture as a valid stronger observable alternative to the
  former one-event setup; retain typed count2 and literal ordinary/cleanup bytes.

### Row dispositions and accepted observations

| Row | Disposition | Evidence and remaining bound |
| --- | --- | --- |
| LF-01 | open | Exact immediate old-owner/message/Scope/index/graph snapshots and later channel1 success pass; F-01 resource/control/complete normalization missing |
| LF-02 | open | Current actual per-App system/getter/collision and controlled Live no-burst pass; F-02 ordered multi-App/both-instance interaction missing |
| LF-03 | open | Real dirty generation, literal80+48 silence, room/invariant error and actual faulted reset pass; F-03 full-buffer/repeat-before-drain/state/retrigger missing |
| LF-04 | open | All-operation stopped-pending checks, real count2 recovery and Live retention pass; F-04 current restart/continuation/empty/error interactions missing |
| LF-05 | done | reproduced: inline candidate_output_and_fault_rejection_preserve_old_owner_and_success_messages_are_typed; typed CandidateMidiNotEmpty1/known candidate Tick; immediate actual ModuleAdded/CableConnected/ParameterChanged/LabelChanged/PatchLoaded File/InMemory payloads and no retired removals; core panic_and_set_bpm_errors_emit_no_mutation_notifications populated World retains state and exact NotImplemented. Unchanged apply_patch_to_world order establishes PatchLoaded-last; separate typed queues do not prove a global event sequence. Whole-unload new base App emits no old removal messages by inspected commit/base path. Other preservation claims remainLF-01/04 |
| LF-06 | done | reproduced: four parsed actual-dispatch tests in both feature modes, integration full20000 signal/copy-MIDI/control capture, internal20072 record/no-overwrite/0→.5 checks; fresh default/no-graph/reference72 traces, corrected300/0 diagnostic, ordinary mode/six CLI failures/native gates; original fixtures/docs retained with provenance |

All six opening rows accounted for:2 done,4 open; no deferrals/no-ops added.
Accepted assertions and earlier attempts remain preserved. CC closing report is
unchanged by CRC. No supporting artifact outside allowed home. No silent drop:
remaining original iteration01 obligations are assigned together in iteration02.

### Bubble-up, retention and author readiness

The runnable04 capability/native checks reproduce; complete lifecycle interaction
proof remains open. No design contradiction or structural arc change is found.
Routine parent/status updates record two accepted rows and active correction;
CDC12 still governs04 then05. Arc05 A-05-01 retains accepted01–03, pending04,
05 removal/handles/commands/BPM and separate07 transport/13 typed forms.
A-05-02 retains final reconciliation. R-01–R-04/P-01–P-06/release remain open.

Iteration02 readiness: (1) source-grounded current delta/helpers/fixtures and exact
missing assertions; (2) design-complete CDC12 unchanged, four test-only paths;
(3) guide07/template, testing/retention and Rust2.0 actual AP-08/09/12/42/44 applied
with source-specific decisions; (4) viable retained-owner inline/public routes,
existing setters/World/query/Scope/buffer methods and illustrative code;
(5) resource checked before retirement, ordered hooks everyApp, unread note/reset
literal4, 129th append error, current restart after file deletion, real-fault failed
candidate and phase-control oracles discriminate the missing properties;
(6) bounded ordered read manifest and itemized intake, accepted versus unresolved
rows/retained native/example receipts,55–75k packet/recovery allowance and no new
proof tooling. Original prompts preserved; next assignment is iteration02.
