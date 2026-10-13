# CC iteration02 — finish the four lifecycle interaction rows

2026-10-12. Sofie/CRC → Duncan → CC. Three contributors, Expedited Mode.
Correction2/5; one executed correction returned without full acceptance.
Active assignment: `arc12-live-rack-operations/slice04-runtime-api/cc-prompt-iteration02.md`.
Predecessor: `cc-prompt-iteration01.md`, preserved unchanged.

## Outcome, authority and baseline

Finish the remaining F-01–F-04 observations in the **Iteration01 independent
review** of `crc-verification.md`. LF-05 and LF-06 are independently **done** at
1242a60; LF-01–LF-04 remain open. No lifecycle implementation failure has been
reproduced. This assignment changes tests and their evidence, not the feature
contract. CDC directive12 D1/D2/D4 and directive08 proportionate proof govern.
Do not reopen the accepted candidate/message/example work or implement Slice05.

Exact source S=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, HEAD
`1242a60fa3a997f95b29195de3b28f7e9a2864ff`, clean at CRC intake.
Planning W=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch planning;
this issuance descends from CC return `9081268e92c70c1fc8269927399056de9cfa4af8`.
P=W/project01-mvp; A=P/arc12-live-rack-operations; D=A/slice04-runtime-api.
Protected main R=`/Users/oubiwann/lab/oxur/oxurack` stays
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
K=`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`;
F=K/collaboration-framework/knowledge. Framework2.6.0, Rust2.0.0.

Check cwd/HEAD/branch/status/worktrees at R/S/W before edits. Reconcile unexpected
source drift through Duncan/CRC. Proceed after coherent intake under existing
authority. No new permission gate, merge, push, endpoint or source edit in main.

## Required reading and preflight

Full means beginning through EOF; sections include definitions and complete
bodies. Recover every truncated read using contiguous bounded chunks. Hashes,
search hits and another context's reading do not establish your own full reading.
Before dependent edits, append an **itemized** intake/readback to the existing
D/closing-report.md: item, actual source state, loaded extent, missing/recovered
portion, conditional applicability. No new intake artifact or structured dataset.

| Order | Exact input / state | Required extent | Purpose |
| --- | --- | --- | --- |
| 1 | W/AGENTS.md; S/CLAUDE.md | full both | Roles, worktrees, Rust/testing conventions |
| 2 | This prompt; D/slice-plan.md; D/ledger.md; D/crc-verification.md, issuance | full all four | Active assignment, accepted versus remaining rows, retained receipts and exact gaps |
| 3 | D/cc-prompt-iteration01.md, preserved | Source reconnaissance and applied guidance; Repair sequence items1–4; Scope/checks/retention/return | Binding observations not yet delivered; accepted items5–6 are historical/reference-only |
| 4 | P/cdc-directive12.md,67e9150 ancestor | D1 lifecycle and isolated candidate; full D2; D3 global BPM paragraph through low-level honesty exception; full D4 and Evidence/return | Approved lifetime/output/compatibility contract;05 is successor-only |
| 5 | P/cdc-directive08.md | Assessment/decisions; Prompt quality; Checks/correction policy | Proportionate proof and correction authority |
| 6 | P/project-plan.md; A/arc-plan.md; A/ledger.md | Project top table/Current direction; arc Capability/Five delivery slices/Current lifetime contract; full arc ledger |04/05 and higher-scale gates |
| 7 | K/collaboration-framework/SKILL.md; F/project-management/guides/README.md | full both | Required operational routing |
| 8 | F/engineering-methods/guides/07-implementation-prompt-authoring.md; F/project-management/guides/02-canonical-planning-worktree.md and03-planning-top-down.md | full07;02 Slice iteration filenames;03 Issuing/executing an iteration | Preservation, author/intake contract and active assignment |
| 9 | F/work-verification/guides/05-independent-verification.md; K/testing/SKILL.md; F/testing/guides/01-testing-discipline.md and03-validation-gates.md | full all four | Independent acceptance/retention and real behavioral checks |
| 10 | K/rust-guidelines/SKILL.md; guides/11-anti-patterns.md | full both; anti-patterns first before Rust edits | Rust guidance applies to CC, not merely the prompt author |
| 11 | S/crates/oxurack/src/lifecycle.rs,1242a60 | full | Candidate commit/admission; private owner/message/graph helpers and test spawners |
| 12 | S/crates/oxurack/tests/lifecycle.rs,1242a60 | full | Late resource witness, ordered hooks, normalized membership and replay/capture helpers |
| 13 | S/crates/oxurack/src/lib.rs,1242a60 | production through with_completed_samples; complete tests fixtures LiveStepState/LiveBehaviorCounter/accessors; test_reinstall_after_parameter_edit_preserves_loaded_state; installation_recipe_replays_once_across_restart_unload_and_reload; live_driver_replacement_rebases_time_without_catch_up; set_port_value; midi_facade_denies_unread_next_sample_before_either_clock_advances; real_backpressure_fault_recovers_through_load_restart_and_unload | Retain facade across calls, real clocks/fault/ports and existing per-App counters |
| 14 | S/crates/oxurack-mod-midiout/src/lib.rs,1242a60 | full | Real dirty marking, fixed capacity, silence preparation and nine tests |
| 15 | S/crates/oxurack/src/persistence.rs | snapshot_patch; failed_snapshot_preserves_destination_and_failed_load_keeps_identity; Getterless and missing_getter_fails_snapshot_and_preserves_existing_file, complete definitions/tests | Disabled-cable and getterless restart errors; read-only implementation |
| 16 | S/crates/oxurack-core/src/{port.rs,cable.rs}; S/crates/oxurack/src/scope.rs | Port/CurrentValue; Cable/CableOrder/CableIndex including target_ports; ScopeRecord/MidiDelivery/ScopeCapture APIs, complete bodies | Complete rosters, typed scalar captures and exact target membership |
| 17 | S/crates/oxurack/examples/patches/turing-machine-lfo.ron and turing-machine-midi.ron; S/crates/oxurack/tests/fixtures/behavior-installation.ron; S/Makefile | full four | Literal configuration/oracles and repository gates |
| 18 | D/closing-report.md; P/arc05-public-api/ledger.md | full both | Preserve attempts, check overstated versus actual observations and inherited owners |

Conditional: before introducing a different dependency API, read its pinned
local definition; do not guess signatures or add dependencies. Current Bevy0.18
APIs and installed test helpers suffice. Reference-only: initial prompt/history,
accepted example source and hardware receipts; not additional assignments.
Budget about55–75k tokens for bounded packet/source/guidance plus implementation
and recovery. If context is compacted, revalidate state and reload binding inputs
not reliably retained. Record continuation in the same closing report.

Readback must connect each remaining finding below to its LF row, test location
and discriminating observation. Explain why unload/reload does not prove restart,
why one ordered App does not prove ordered replay in every App, and why a marker
checked only after a successful replacement cannot prove failed-owner isolation.
Do not report omitted checks as completed merely because native suites pass.

## Applied guidance and source route

CRC inspected the six-path1242a60 correction, production/test-region boundaries,
current actual example, and reproduced affected native/musical checks. Library
production, fixtures, runner, docs and dependencies are unchanged from f6fdeb5.
The six new groups improve proof; four still omit assigned interactions.

| Author-applied source / strength | Decision and reason | Concrete proof |
| --- | --- | --- |
| guide07 §5 and Corrective iterations | Extend existing real tests only for remaining claims; don't create a proof platform or rerun unaffected demonstrations | Four grouped recipes below; retention table |
| Rust AP-08 AVOID | Preserve fixed128 storage; observe capacity during full-buffer silence/error, reuse caller drain scratch | Existing peak test, no allocator instrumentation |
| Rust AP-09/AP-42 AVOID | Fixture-specific test expect/assert diagnostics; no new production unwrap or suppressed error | Exact typed errors and literal event vectors |
| Rust AP-12 AVOID | Owned before/after snapshots and intentional separate controls; short World/resource borrows | Observe failure before continuing or replacing the owner |
| Rust AP-44 AVOID | Private inline tests for live owner inspection; keep public integration tests for actual musical APIs | No public World/test accessor |
| testing Proportionate Proof | One fixture can cover several operation boundaries; preserve accepted tests and native gates | No Cartesian test expansion, coverage/dependency campaign or new checker |

**Binding:** all missing behavior is already in iteration01/directive12, not new
hardening. **Recommended:** extend the named tests/helpers below, using inline
owner access when needed. **Local discretion:** helper names and test grouping.
Any newly red test requires root-cause diagnosis and a return through Duncan
before production repair beyond this test-only assignment. Preserve the failing
oracle; do not change expectations, add ignores or call a red return proposed-done.
Sketches below are uncompiled shapes based on inspected APIs, not drop-in code.

## 1. F-01 / LF-01 — observe the failed lifetime before it retires

At1242a60, `late_spawner_failure_isolated_and_old_engine_continues` compares MIDI,
but only Scope count; CandidateMarker is checked **after** successful replacement.
The inline CandidateLateFailure writes an entity/message, not a resource. A leak
of that marker or a reset LFO phase can therefore pass these witnesses.

Extend the inline `CandidateLateFailure` to insert a local test resource before
its existing entity/message/Err. Assert resource absence in the retained old World
immediately after Err, before ticks or replacement. Keep the existing exact
membership/index/graph/message/Scope assertions. It must fail after a real sink
has been spawned. Add the same immediate Scope-value comparison in the public
late-spawner control test, then continue subject/control through at least1000
samples with per-sample drain; compare full scalar records and overwrites as well
as MIDI. Scope capacity can remain bounded256 if both complete retained windows
are compared; use a sufficient window if comparing all historical observations.
Stop/drain and preserve the already passing same-subject channel1 recovery.

Fix cross-World structure projection: the current modules tuple includes
`Entity::to_bits()` and is compared against a different fresh World. Resolve
identities to module/port names **within each World**; keep raw IDs only for
same-owner before/after comparisons. Include port merge_policy, cable enabled
and explicit ChildOf roster/count. For CableIndex verify each cable endpoint
projection and normalize the complete `target_ports()` roster, not only queries
for known ports. Compare with the fresh intended fixture; no graph source edit.
This rejects stale target entries/orphans and avoids relying on repeated Entity
allocation. Existing exact graph membership/ports checks remain.

## 2. F-02 / LF-02 — ordered hooks and edited state in every App

The integration HookOrder `[1,2]` test ends after one load; the new multi-lifetime
counter uses one type and checks only alpha. Combine their observations in the
inline lib.rs test so the facade survives all operations. Two local ordered hook
probe types may reuse the integration macro shape; each register_systems appends
its literal1 or2 and installs one counter system into the supplied schedule.
Keep LiveBehaviorCounter/getters alongside them. Inspect per-App resources after
load → restart → nonempty unload → original reload; HookOrder is exactly `[1,2]`
with no9 collision entry, each system runs once per admitted sample.

Use behavior-installation.ron alpha2/beta3. First sample totals2/3, one live system
run and two hook-probe runs. Set alpha4; same-type reinstall and kind collision
inside that edited loaded App preserve recipes, order, setters/getters, totals
and counters. Next sample totals6/6, live runs2, probes4. Restart snapshots4/3:
fresh totals0/0/getters4/3/counters0; next sample totals4/3/live1/probes2.
Unload's empty App still has `[1,2]`/counters0; repeated already-unloaded operation
is a true no-op. Original reload getters2/3 and one sample totals2/3/live1/probes2.
Observe both instances and the original successful install order each time.
Stop/drain before every lifetime operation; don't consume into_app mid-sequence.
Existing unrelated-schedule/reinstall tests and failed-candidate hook counts stay.

```rust
// Illustrative observations after a restart, using existing test helpers:
let handle = rack.world().resource::<PatchHandle>().clone();
assert_eq!(live_state(&rack, &handle, "alpha").total, 0);
assert_eq!(live_parameter(&mut rack, handle.modules["alpha"]), ParameterValue::Int(4));
assert_eq!(live_parameter(&mut rack, handle.modules["beta"]), ParameterValue::Int(3));
// Also inspect the new local HookOrder/counters before and after one admitted sample.
```

## 3. F-03 / LF-03 — actual facade reset and the full-buffer error boundary

Keep passing real CC-only/released-note, room/invariant and complete80+48 tests.
Add a one-sink loaded raw-port facade case using existing set_port_value: channel0,
step/valid/gate true, note60, velocity100, no CC mappings/cables. Configure Scope
on midi_main.note, Manual advance1, **do not drain**. Before reset, running load,
restart and unload must each return LifecycleRequiresStopped even with pending
output; after stop they must each return PendingMidi{count:1}. Capture time,
Fixed/Virtual clock elapsed/overstep, configuration, driver, Scope selectors/
capacity/overwrites/full values, fault and per-App hook/message state around
these calls and reset. This consolidates the admission interaction with LF-04.

Actual reset appends cleanup; second reset **before drain** retains count4, all
preserved state unchanged. Exact drained oracle:

```rust
assert_eq!(events, vec![
    (0, [0x90, 60, 100]),
    (1, [0x80, 60, 0]),
    (1, [0xB0, 123, 0]),
    (1, [0xB0, 120, 0]),
]);
```

Stop/start adds no MIDI. After drain/start a raw retrigger produces exactly one
new NoteOn at sample1, without obsolete active NoteOff. Exercise reset running
and stopped using the same controlled setup; selected Live retention may be
checked without wall-clock execution after a controlled Manual sample. Preserve
the independently passing real-first-fault reset test; don't replace it with a
synthetic fault witness.

At MIDI-out `sixteen_sinks_fit_eighty_normal_plus_forty_eight_cleanup_events`,
keep the full literal128 vector pending **before drain**. Assert backing capacity,
logical80, SampleTime2 and no fault at that boundary. Prepare a private one-event
SilenceBatch via its existing empty/push helpers and attempt append_silence; the
129th event returns OutputCapacity and leaves all128 events, capacity, fault and
cleared active/dirty state unchanged. A second actual silence remains128. Then
drain and retain current full bytes/timestamp assertions. No production branches,
allocator hook or new queue. Existing room-error and invariant-error retries stay.

## 4. F-04 / LF-04 — current restart, faulted candidate and continuation

Four bounded interaction groups are still absent; use existing fixtures rather
than an exhaustive matrix.

**Current configuration restart:** extend the real edited-fixture proof without
removing accepted save/unload/reload coverage. Load a unique temporary copy of
LFO_PATCH, run300 samples, edit LFO frequency1, TM base.25 and the existing
lfo_main.bipolar_out → tm_main.write_modulation cable to constant+.25, description
`Constant +0.25 write modulation`. Independently form expected configuration from
the original fixture plus those literal edits; preserve seed42/header/global120,
all other metadata/parameters and cable order. Delete the temporary original
input, stop/drain, call **restart**, drain cleanup, assert fresh0/stopped/Manual,
no fault/Scope and current snapshot equals the independently checked configuration.
A separately fresh rack loaded from expected config supplies the reference.
Reconfigure the two existing Scope selectors and compare10000-sample MIDI/scalars
with zero overwrites, including probability0 at0 then.5 from1. This rejects reading
the original path, default reconstruction and retained phase/RNG. The accepted
unload/reload72-row recipe is not this restart witness.

**Failed snapshot / empty lifetime:** inline lifecycle tests can reproduce the
read-only persistence fixture technique: disable a selected cable, call restart,
expect PatchError::UnsupportedSnapshot with disabled-cable reason and unchanged
old owner/time/Scope/driver/fault/messages. Getterless test already passes and is
retained. For unloaded admission, running restart returns LifecycleRequiresStopped
before NoPatchLoaded; stopped/drained restart returns NoPatchLoaded. Initial
unloaded load with unread output rejects before missing-path I/O (use an actual
raw sink spawned in its World, then one sample, before invoking load).
Load an empty Patch with retained seed/header, unload once and prove old resource/
loaded identity retires and recipes replay. On already-unloaded rack add a local
resource marker and known fault; empty reset emits nothing and preserves it;
repeat unload after drain is a no-op preserving marker/fault/hooks/time. In a
nonempty unload carrying cleanup, second unload before drain returns exact count,
then after drain is the same no-op. No arbitrary production World accessor.

**Real fault → failed candidate → fresh sample:** extend existing
real_backpressure_fault_recovers_through_load_restart_and_unload. Its raw sink
retrigger emits two events at251; CRC accepts this useful alternative to the old
fixture's one-event setup: returned completed_samples1, stored first-fault fields
captured independently, stopped sample252 and PendingMidi{count:2}, exact
`[(251,80 3C 00),(251,90 3C 64)]` are already reproduced. Drain ordinary output;
now fail an actual late-spawner candidate and compare the retained old fault,
owner, Scope/time/driver/messages immediately. A missing file alone cannot prove
candidate preservation. Then perform each successful load/restart/unload as today,
retaining exact cleanup252. Drain cleanup, reload after unload, set fresh raw
inputs, start and admit sample0 with a literal new NoteOn, no stale NoteOff/fault.
Reset-through-first-fault evidence is retained from the existing actual facade
fault test. Do not alter the old first cause to match the returned per-call count.

**Stop/start and reset preserve phase:** actual Clock/LFO/TM/Scope subject/control,
same fixture/seed, per-sample advance/drain. At300 stop/start subject and compare
future scalar and MIDI sequence against uninterrupted control. For a separate
reset-at300 case, assert time/Scope/config unchanged and account explicitly for
cleanup at300 and the control's release of the silenced old note. Compare **all**
scalar values and future NoteOn/CC; resume full event equality after next shared
retrigger. Bound the one intentional release difference by its active note and
sample, not a permanent filter of all NoteOff events. Both runs must continue
through a future clock edge. Keep the passing controlled Live no-catch-up test;
add fresh Fixed/Virtual elapsed0/overstep0/stopped checks at replacement boundaries
where not yet asserted. Controlled durations0/1ms already prove execution timing.

## Scope, fresh gates and retained checks

Exactly four source paths, **test regions only**:

1. crates/oxurack/src/lib.rs — existing cfg(test) module, including local fixtures.
2. crates/oxurack/src/lifecycle.rs — cfg(test) module.
3. crates/oxurack/tests/lifecycle.rs — test/helpers.
4. crates/oxurack-mod-midiout/src/lib.rs — cfg(test) module.

No other source paths, production prefixes, examples/docs/fixtures, manifest or
lockfile edits. Production defect discovery follows the earlier stop/diagnose/
return condition. No new teardown/handle/command API or Slice05 directory.
Planning edits only D/closing-report.md and D/ledger.md; append iteration02 intake,
attempts, actual assertions, per-row proposed evidence and retention proposal.
Keep LF-05/LF-06 **done** with their CRC provenance; keep LF-01–LF-04 **open**
with CC proposed-done until independent acceptance. Don't edit prompts/review/
plans/parents/status/directives. Preserve initial/iteration01 failures and results.

Run fresh from S on final source:

```bash
cargo test --locked -p oxurack-mod-midiout
cargo test --locked -p oxurack --all-targets
cargo test --locked -p oxurack --no-default-features --all-targets
make test
make lint
cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings
git diff --check
```

Preserve restricted make test failures and retry **unchanged** with authorized host
access if CoreMIDI cannot initialize. Record RT denominator/three existing ignores;
no new ignore/warning suppression. No coverage, deps/outdated or new checker.

Retain CRC2026-10-12 at1242a60: candidate/error/message LF-05, actual example LF-06,
425core unit+5full-cycle+9seeded+2doc checks, actual default/no-graph/fresh72-row hash
`20d86b836d7540a2ac6bf725f5045dfe6304fa7a027d703f22e1fe93f27613ba`, ordinary save300,
six negative CLI exits and builds. They remain applicable if only the allowed
new tests change and their accepted assertions are preserved. Core is still
covered by fresh make test/Clippy. Retain strict-docs/original13/70 fixture evidence
at f6fdeb5 with its original date, as audited in CRC iteration01 record. New tests
cannot replace fresh affected test/native checks. If inputs/assertions change or
validity is uncertain, obtain fresh corresponding proof and report it.

Return exact prompt SHA/source/planning commits, changed paths and production-region
audit, itemized intake/readback, all commands/exits/attempts/retentions, every
remaining finding's assertion or explicit gap and six-row walk. Do not say a
parallel unload/reload test proves restart or count-only Scope proves phase.
A-05-01 retains01–03 accepted children,04 pending lifecycle,05 handles/removal/
commands/BPM,07 transport and13 catalog forms. A-05-02 retains final composition
reconciliation. R-01–R-04/project/release remain open. No durable supporting
artifacts expected; transient receipts in /tmp, canonical records in D.

Commit only named allowed changed source paths via git commit --only, then
separately D/closing-report.md and D/ledger.md in planning, preserving applicable
repository trailers. Both worktrees clean on return; main unchanged. Report via
Duncan; no automatic acceptance or Slice05 advancement by CC.
