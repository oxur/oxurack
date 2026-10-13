# CC iteration01 — complete lifecycle interaction evidence

2026-10-12. Sofie/CRC → Duncan → CC. Three contributors, Expedited Mode.
Correction1/5. All six LF rows remain open. This replaces the initial prompt as
the active assignment; that issued file and its binding behavior remain preserved.
Assignment path: `arc12-live-rack-operations/slice04-runtime-api/cc-prompt-iteration01.md`;
preceding prompt: `arc12-live-rack-operations/slice04-runtime-api/cc-prompt.md`.

## Authority, outcome and exact baseline

Complete F-01–F-06 in [crc-verification.md](crc-verification.md). The actual musical
delivery and native gates passed CRC reproduction at
`f6fdeb513331a8d8962876bdf4766cedfce63f59`; six interaction-proof gaps prevent
acceptance. No lifecycle implementation defect has been reproduced. This is a
bounded test/evidence correction, with narrow example testability/diagnostic work.
Directive12's approved semantics remain binding; no architecture decision is open.

Roots:

- R=`/Users/oubiwann/lab/oxur/oxurack`; protected main
  `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
- S=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
  branch `codex/oxurack-module-behavior-installation`, exact source pin f6fdeb5 above.
- W=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch planning;
  issuance descends from CC return `622e9948c443748bddbdc9c944b50aa5398fc1b1`.
- P=W/project01-mvp; A=P/arc12-live-rack-operations; D=A/slice04-runtime-api.
- K=`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`;
  F=K/collaboration-framework/knowledge. Framework2.6.0, Rust2.0.0.

Collect cwd/branches/HEAD/status/worktree list at R/S/W before edits. Unexpected
source drift or outside-scope repair goes through Duncan to CRC. Do not implement
in main/planning, merge, push, operate an endpoint or create Slice05. Proceed after
coherent preflight under existing authority; no additional confirmation gate.

## Required reading and intake

Each full item means beginning through EOF. Section items include their complete
bodies/subheadings/necessary definitions. Recover truncated reads with contiguous
bounded chunks. File hashes/search summaries are not full reading. Record actual
source state/extents, conditional triggers and a concise source-cited readback
in D/closing-report.md before dependent edits; preserve initial-pass history.
No structured dataset or separate intake artifact is assigned.

| Order | Input / state | Required scope | Purpose |
| --- | --- | --- | --- |
| 1 | W/AGENTS.md; S/CLAUDE.md, issuance/source pin | full both | Roles, allowed homes and Rust/testing routes |
| 2 | This prompt; D/slice-plan.md; D/ledger.md; D/crc-verification.md, issuance | full all four | Assignment, all six open rows and concrete findings/retention |
| 3 | D/cc-prompt.md, initial preserved issuance | sections3–5 and7–8 complete | Binding lifecycle matrix, implementation seams, every required oracle, gates and return; old initial scope is narrowed below |
| 4 | P/cdc-directive12.md,67e9150 ancestor | D1 lifecycle; D1 isolated candidate; D2; D3 Commands and global tempo paragraph beginning “Global bpm()” through “correct their tests/docs”; full D4 and Evidence and return state | Approved04 semantics; low-level honesty exception;05 only a boundary |
| 5 | P/cdc-directive08.md | Assessment and decisions; Prompt quality for all remaining work; Checks and correction policy from Slice03 onward | Proportionate proof, no dependency/hardware campaign |
| 6 | P/project-plan.md; A/arc-plan.md and ledger.md, issuance | Project top table/Current direction; arc Capability, Five delivery slices and retained ownership, Current lifetime and compatibility contract; full arc ledger | Current04/05 and parent authority; historical project timeline reference-only |
| 7 | K/collaboration-framework/SKILL.md; F/project-management/guides/README.md | full both | Skill routing before planning work |
| 8 | F/engineering-methods/guides/07-implementation-prompt-authoring.md; F/project-management/guides/03-planning-top-down.md | full07;03 Issuing and executing an iteration and necessary assignment definitions | Intake, active preserved assignment, executable correction |
| 9 | F/work-verification/guides/05-independent-verification.md; K/testing/SKILL.md; F/testing/guides/01-testing-discipline.md and03-validation-gates.md | full all four | Correction retention, discriminating real checks, no new proof platform |
| 10 | K/rust-guidelines/SKILL.md and guides/11-anti-patterns.md | full both, anti-patterns before Rust edits | Domain skill explicitly applied by author and CC |
| 11 | K/rust-guidelines/guides/14-cli-tools/README.md and06-testing.md | full README; complete CLI-34/35/37/38 entries | Actual example/parse/error paths and isolated temporary files |
| 12 | K/rust-guidelines/guides/14-cli-tools/04-output-and-ux.md | complete CLI-27 | Fallible trace stdout, diagnostic stderr |
| 13 | S/crates/oxurack/src/lifecycle.rs; S/crates/oxurack/tests/lifecycle.rs; S/crates/oxurack/examples/save-live-patch.rs; S/crates/oxurack-mod-midiout/src/lib.rs, f6fdeb5 | full four files | Actual implementation, eight tests, cleanup fixtures, recipe |
| 14 | S/crates/oxurack/src/lib.rs, f6fdeb5 | production through with_completed_samples; complete tests module fixtures LiveBehaviorCounter/live_state/live_parameter/set_port_value; complete test_reinstall_after_parameter_edit_preserves_loaded_state, test_driver_transitions_discard_overstep_without_catch_up, test_live_transitions_and_stop_start_discard_catch_up, midi_facade_denies_unread_next_sample_before_either_clock_advances and facade_reports_consume_fault_after_failing_sample_finalizes | Private owner access, real faults, existing TimeUpdateStrategy/parameter helpers |
| 15 | S/crates/oxurack/src/persistence.rs | snapshot_patch complete; failed_snapshot_preserves_destination_and_failed_load_keeps_identity; Getterless and missing_getter_fails_snapshot_and_preserves_existing_file complete | Snapshot errors and preservation helpers; no new getters |
| 16 | S/crates/oxurack-core/src/event.rs and S/crates/oxurack-core/src/patch.rs; S/crates/oxurack-core/src/cable.rs; S/crates/oxurack-graph/src/lib.rs | event types PatchLoaded through LabelValue, dispatch_core_command and Panic/SetBpm tests; apply_patch_to_world full; Cable/CableOrder/CableIndex complete; graph RackGraph/GraphView roster/ports_of definitions and methods | Actual notification names, typed queues, normalized index/graph membership |
| 17 | S/crates/oxurack/src/scope.rs; examples/patches/turing-machine-midi.ron andturing-machine-lfo.ron; S/Makefile | ScopeRecord/MidiDelivery/ScopeCapture APIs; full fixtures/Makefile | Observation, literal configuration and native gates |
| 18 | P/arc05-public-api/ledger.md; D/closing-report.md,622e9948 | full both | Preserve original rows and CC attempts; inherited walk |

If changing an API usage beyond these inspected seams, load its pinned dependency
definition before coding; do not guess signatures. Guides/whole files outside
these bounds are conditional on the affected code, not universal homework. No
new helper crate, verifier, matrix file, benchmark or coverage tool. Budget roughly
60–85k tokens for the packet/source/guidance plus implementation/recovery headroom;
checkpoint in the same closing record rather than trimming binding tests.

Readback: connect each F finding to its LF row, named test home and observable
oracle. Explain reset versus restart, pending/fault admission, fresh versus failed
App effects, old cleanup timestamps and typed notification lifetime. Explain why
post-continuation message lengths and a successful cargo command cannot prove the
missing interactions. Retention below refers to CRC receipts, not CC attestation.

## Source reconnaissance and applied guidance

CRC inspected the eleven-path f6fdeb5 delta, full lifecycle/MIDI consumer/tests,
current inline fixtures and actual examples, and independently ran every original
native gate plus happy/negative CLI cases. New lifetime behavior follows the
design; current tests observe only a subset of section5. The existing eight tests
and their helpers are the implementation spine; extend them instead of duplicating
the rack or introducing a production test API.

| Author-applied guide / strength | Decision and reason | Proof home |
| --- | --- | --- |
| guide07 §5 and Corrective iterations | Add only the missing contract observations; distinguish missing proof from a known product defect; keep existing green checks with justified retention | Six finding recipes below; CRC record |
| Rust AP-08 AVOID | Preserve128-slot storage and reuse the existing drained scratch; test capacity before/after actual silence, not allocator folklore | MIDI-out peak test |
| Rust AP-09/AP-42 AVOID | Production example writers propagate Result; test unwrap/expect is appropriate with fixture-specific diagnostics | Actual parsed recipe and writer failure tests |
| Rust AP-44 AVOID | Use inline private owner tests; do not expose World just to make tests convenient | lib.rs/lifecycle.rs tests |
| CLI-34 SHOULD,35 MUST,37 SHOULD,38 CONSIDER | Test actual mode parsing/dispatch, error inputs and saved replay. Reuse std temporary files/writers; assert_cmd/assert_fs are illustrative guidance, not authorization for dependencies | save-live-patch example tests and manual native CLI checks |
| CLI-27 MUST | Preserve existing ordinary output; lifecycle MIDI goes only to trace writer, accurate diagnostics to status writer | Example helper factoring/byte equality |
| testing validation Proportionate Proof | Real sinks, real App lifecycle and existing suites, no new checker or metric-driven test count | Complete positive/negative assertions below |

## Repair sequence and binding test oracles

Sketches below are **uncompiled test/implementation shapes** based on inspected
symbols. Private helper names and grouping are local discretion; observable
behavior, scope and expected values are binding. A few multi-contract fixtures
are preferable to a large Cartesian test matrix. Each missing initial section5
observation must have a specific assertion or a justified existing receipt.

### 1. F-01 — exact retirement and immediate failure preservation (LF-01)

Extend `repeated_real_patch_replacement...` and `late_spawner_failure...` in
tests/lifecycle.rs; use inline lifecycle tests for observations that require the
retained private owner. Do not consume `into_app` until all facade calls end.

Normalize entities **inside each World** to exact module name/kind, parent module
and port name/direction/kind/merge, and cable endpoints/transform/enabled/order.
Compare complete sorted rosters and Module/Port/Cable/ChildOf counts against a
fresh intended fixture. Resolve every PatchHandle member to its actual Module;
verify CableIndex.cables_from/cables_targeting against every cable and exact
target roster, not merely target_ports().count(). With graph enabled, compare
GraphView.modules()/cables()/ports_of rosters, resolving their own-world identities;
check contains_module/contains_cable. No cross-World Entity equality or guessed
graph “port count” method. No graph production edit.

For valid[1,0], reject[1,1] and late-spawner Err, observe old header/snapshot,
time/clocks, selected driver/running, full retained Scope values, pending output,
fault, membership/index/graph and notification payloads immediately around the
call. LateFailure must reach its resource/entity/message write **after** a real
sink is spawned. Existing custom schema/temporary RON already provide this seam.
Test an owned marker Drop counter if practical; external hook counts are expressly
not rolled back and must not race other tests.

Then resume old real LFO/TM against a matched unfailed control through a future
edge, per-sample drain and compare MIDI **and scalar Scope records**, not just
record counts. Stop/drain, load a channel1 temporary patch successfully in that
same subject: failed candidate reservation must not survive. Sample/message
maintenance may rotate queues; inspect Err messages before this continuation.

### 2. F-02 — replay across every App, actual hooks/accessors/systems (LF-02)

Keep the two ordered hooks but extend observation through load → restart →
unload → reload. Use inline lib.rs tests to retain `rack.app` between calls.
Each fresh App sees ordered `[1,2]` once; same-type reinstall and kind collision
inside a loaded/edited App leave recipes, registrations, order, state and system
counters untouched. Empty unload still replays types; already-empty unload does
not create another App. Separately count the failed candidate already in F-01.
Stop/drain before each lifetime operation and drain carried cleanup before a
new sample. A local test-only colliding type may use behavior_counter's kind;
its hook/spawn must never run when the existing type owns that kind.

Reuse `LiveBehaviorCounter` and behavior-installation.ron for nonempty accessors:
initial increments2/3; one sample totals2/3 and exactly one system execution;
set alpha increment4, reinstall/collision preflight, next totals6/6 and execution2.
Restart from current configuration: fresh totals0/0, getter4/3, next totals4/3,
one execution in this App. Unload/reload the original fixture: getter2/3, totals2/3
after one sample, one execution. All recipe/hook counts are per-App. Observe
disabled/unrelated schedule separately where the existing fixture already does
so; retain prior installation tests, never weaken them to accommodate duplication.

### 3. F-03 — actual reset, dirty production and atomic bounds (LF-03)

Inline lib.rs can reuse `set_port_value` and the raw sink setup. Prefer a
temporary one-sink loaded Patch (no cables), so Scope selector sink.note and its
snapshot are observable, then set note60/velocity100/step+valid+gate true and run
one Manual sample. Do not drain it. Capture clocks/time/running/driver/Scope/
configuration/fault before actual `reset`; the exact drained vector is:

```rust
// Contract oracle: ordinary sample0 first, silence at completed boundary1.
assert_eq!(observed, vec![
    (0, [0x90, 60, 100]),
    (1, [0x80, 60,   0]),
    (1, [0xB0,123,   0]),
    (1, [0xB0,120,   0]),
]);
```

Second reset **before drain** leaves count4; all captured state stays unchanged.
Stop/start alone adds no events. After draining, actual retrigger emits one new
NoteOn, without an obsolete active NoteOff. Exercise reset while running and
stopped; a representative retained Live driver can be selected after the first
controlled sample without wall-clock execution.

Modify the MIDI-out16-sink test to call `silence_outputs`, not merely append a
prepared batch. First normal output is64 messages; drain. Second sample holds80
normal events unread. For each numeric channel c0..15 with note60+c, the literal
ordinary order at sample1 is:

```text
80+c, note,0; 90+c,note,100; B0+c,74,23; B0+c,75,note; B0+c,76,100
```

Reset appends sample2 cleanup in channel order:
`80+c,note,0; B0+c,123,0; B0+c,120,0`. Assert the complete128-event vector,
ordinary prefix unchanged, logical capacity80, backing capacity unchanged,
no fault, SampleTime2, active/dirty cleared and second silence still128. Existing
admission denies a subsequent sample while pending; a129th append returns Err
without modifying the vector. These expected byte formulas are contract-derived,
not another invocation of prepare_silence.

Produce CC-only and already-released-note cases through actual normal inputs/run,
not by assigning dirty=true. CC-only and released channels both emit123/120 after
their ordinary events are drained; never-used emits nothing. Keep reverse name/
entity versus channel order. Existing room-error fixture remains; also corrupt
one private accounting invariant (e.g. active note with dirty=false), snapshot
pending/fault/active/dirty before prepare/silence Err, assert exact preservation,
repair injected invariant and observe original cleanup. No production test branch.

### 4. F-04 — admission, real fault, seeded restart and drivers (LF-04)

Use table-driven inline lifecycle operations to cover load/restart/unload. For
running owner with pending output all return LifecycleRequiresStopped without
hooks/messages/time/Scope/state mutation. Missing load path and an unloaded
restart discriminate I/O/snapshot precedence. For stopped unread **one NoteOn**,
all return PendingMidi{count:1}, preserving the exact pending event/fault/owner;
no hook replay. Initial unloaded load has the same admission. After cleanup carry,
repeat unload before drain returns its exact pending count, after drain empty
unload is a true no-op preserving an injected fault. A loaded empty Patch must
retire once, unlike already-unloaded; empty reset emits nothing.

Use the real MIDI fixture and `advance_samples` to create backpressure: advance/
drain251 samples, then request2 samples so NoteOn at251 is left unread and the
next admission latches OutputBackpressure. Assert actual first fault, stopped
state, sample252 and pending NoteOn. Do not guess stored-versus-returned
completed_samples: compare the captured first fault's fields at each transition.
Reset succeeds through this fault, appends cleanup at252 and preserves the first
cause/time/Scope. In separate instances for load/restart/unload, drain ordinary
output, fail a candidate and preserve the fault; successful lifetime clears it
and carries exact three-event old cleanup at252. New time0/stopped/no Scope;
drain before starting (reload after unload) and admit fresh sample0 successfully.
Replacement attempted with unread output cannot clear the fault. Candidate failure
must reach spawning when claiming candidate isolation, not only missing-file I/O.

Restart: extend the real edited fixture with LFO1, TM base.25, constant+.25 cable
and its description, retained seed/header. Save expected current configuration;
delete/alter **temporary original input**, never committed fixtures. Stop/drain,
restart, drain cleanup, assert current snapshot equals independently checked
literal configuration and not original. Time0/stopped/no Scope/fault. Reconfigure
Scope, start10000 samples and compare MIDI/scalars with a separately fresh rack
loading the checked configuration. This must reject rereading the original,
keeping performance phase/RNG, or reconstructing default configuration. Getterless
restart already has a test; add disabled-selected-cable UnsupportedSnapshot and
prove old owner intact using the existing persistence failure fixture technique.

Real stop/start and reset continuation: identical Clock/LFO/TM+Scope subject and
control, same per-sample advance/drain. At300 stop/resume subject and compare
future event/capture sequence unchanged. For reset at300, compare all scalar
captures/time/state and future NoteOn/CC production; account explicitly for its
intentional cleanup and the control's later release of the now-silenced old note.
Resume full event equality after the next shared retrigger. Do not blindly filter
every NoteOff forever or require reset and unsilenced control to be byte-identical.

Controlled Live uses existing `TimeUpdateStrategy::ManualDuration` and inline
owner access. Load/restart/nonempty-unload retain selected Live, fresh fixed/
virtual clocks0, paused/stopped, overstep0. Simulate a long stopped interval with
controlled duration (no sleeps), then start with zero duration: no sample/counter
advance; next1ms advances exactly one sample/index0. Reinstall a test time strategy
after the App swap: arbitrary old strategy resources are intentionally not cloned.
Reload after unload before testing musical execution. Verify Manual retention too.

### 5. F-05 — candidate contamination and immediate notifications (LF-05)

Use a temporary candidate with one real MIDI sink, no cables, and a valid-schema
test spawner after it. Set that sink's actual
ports to the raw note oracle, set candidate running and run its FixedUpdate solely
to violate the no-performance-output contract deliberately; return its own valid
module entity. Public load must return CandidateMidiNotEmpty{count:1}, leaving old
owner/Scope/fault/messages intact. This unsupported test spawner is not production
permission to tick during initialization. A companion spawner can latch a known
candidate fault without pending events; load returns that Tick error, without
clearing the old lifetime. No new public buffer push/test accessor.

For successful annotated candidate, immediately inspect ModuleAdded,
ParameterChanged, LabelChanged, **CableConnected** (actual type, not CableAdded),
PatchLoaded and absence of ModuleRemoved/CableDisconnected. Match payload entities
to new active membership, names/kinds, literal values, counts and PatchLoad
provenance. File load source equals File(path); restart equals InMemory. Nonempty
unload has no synthetic retired notifications. Failure comparisons use the same
typed payload projections before/after Err, before further sample execution.
Independent queues have no global ordering API: cite unchanged apply_patch_to_world
write order for PatchLoaded-last, pair with actual batch content; do not invent a
cross-type MessageId comparison. Core Panic/SetBpm return exact NotImplemented
and leave a populated World's state/time/notifications untouched (extend inline
event tests). All other command behavior remains unchanged.

### 6. F-06 — test the actual example and complete capture (LF-06)

Narrow production allowance in `examples/save-live-patch.rs`: factor existing
dispatch/recipe output through private writers so tests execute parsed Args and
the same routine as main. Keep signatures private, ExitCode and default output.
Recommended shape (uncompiled, representative):

```rust
fn run_with_output(args: Args, output: &mut impl std::io::Write,
                   status: &mut impl std::io::Write)
    -> Result<(), Box<dyn std::error::Error>>
{
    if args.lifecycle { return run_lifecycle(args, output, status); }
    // Original ordinary recipe; replace println! with fallible writeln!.
    // No alternate test-only musical recipe.
    /* ... */ Ok(())
}
// run/main obtain and lock stdout/stderr, then call this dispatcher.
// In run_lifecycle, capture before unload:
let retired_sample = rack.sample_time().completed_samples(); //300
rack.unload_patch()?;
let cleanup: Vec<_> = rack.drain_midi().collect();
writeln!(status, "lifecycle: retired at sample {retired_sample}; cleanup events={}",
         cleanup.len())?;
// Separately assert new lifetime time0; do not label that the retired boundary.
```

Tests use Args::try_parse_from including/defaulting --lifecycle, missing required
arguments, and unknown flags. Call actual dispatcher with Vec writers and unique
temporary save paths. Actual lifecycle trace equals fresh saved reference; no
ordinary summary on its stdout, diagnostics report retired300/new0 consistently.
Default mode retains frequency edit and completed_samples300. Missing/malformed
patch and nonexistent save parent return Err; manual CLI checks below establish
nonzero exit. A deliberately failing writer returns Err, no panic. Use std only;
no new test dependencies or recursive cargo invocation inside cargo tests.

Strengthen the actual capture inspection and integration capture helper: exactly
one scalar for each sample0..9999 and selector0/1, expected kinds,10000 each,
zero overwrites. Fresh configured fixture derives initial typed default probability
0 at sample0; Consume applies.25+.25 and next publication exposes.5 at sample1,
then all subsequent observations are.5. CRC independently checked the fresh
saved fixture CSV: this transition is0→.5 at1, not a guessed warm-up at250.
Compare LFO values/phase to the independently configured fresh reference. Retained
Scope MIDI records equal the actual drained event vector in order, every delivery
Generated, no cleanup included in replay; do not drop them from the test projection.
For this concrete configuration the reproduced trace contains72 events and hence
20072 mixed capture records. Keep the stronger literal/configuration oracles even
when the72-row equality passes. No synthetic hardware/delivery claim.

## Scope, checks, retention and return

Exactly these **six source paths** are allowed:

1. crates/oxurack/src/lib.rs — existing `#[cfg(test)]` module only.
2. crates/oxurack/src/lifecycle.rs — `#[cfg(test)]` module only.
3. crates/oxurack/tests/lifecycle.rs — integration tests/helpers.
4. crates/oxurack-mod-midiout/src/lib.rs — `#[cfg(test)]` module only.
5. crates/oxurack-core/src/event.rs — `#[cfg(test)]` module only.
6. crates/oxurack/examples/save-live-patch.rs — private writer/dispatch factoring,
   accurate retired-sample diagnostic, complete capture checks and actual tests.

Imports needed by tests belong inside their cfg(test) module. No lifecycle/MIDI/
core production change is prescribed because no such defect is yet reproduced;
this is an evidence iteration, not a generic prohibition for feature work. If
new tests expose a real implementation defect, preserve its failing oracle and
report exact cause/path through Duncan before outside-scope repair. Do not weaken
assertions, add ignores or label the red return proposed-done. All other source
paths/fixtures/manifests/lockfile are read-only. No new command/handle/teardown API.

Planning edits only D/closing-report.md and D/ledger.md. Append an iteration01
section with itemized intake/readback and attempts; keep initial failures/history
and CC/CRC evidence strengths distinct. Do not modify prompt, plan, CRC record,
parents/status/directives or accepted ledgers. No permanent log forest expected.

Run on final source from S:

```bash
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack-mod-midiout
cargo test --locked -p oxurack --all-targets
cargo test --locked -p oxurack --no-default-features --all-targets
make test
make lint
cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings
cargo build --locked -p oxurack --example save-live-patch --example turing-machine-midi
cargo build --locked -p oxurack --no-default-features --example save-live-patch
git diff --check
```

Keep restricted make test failure if CoreMIDI cannot initialize; retry unchanged
with host access, recording exact command/revision/context/ignored denominator.
No blanket ignore or suppressed advisory. Run original section7 actual lifecycle
default/no-default/fresh trace comparisons and ordinary save; six CLI negative
cases (missing/malformed/save-parent in both modes). New automated tests run in
both feature modes. Final musical trace should retain the observed72-row hash
20d86b836d7540a2ac6bf725f5045dfe6304fa7a027d703f22e1fe93f27613ba;
if it changes, report the actual discrepancy, do not repin to silence it.

Propose retention of CRC2026-10-12 strict-docs success and original13/70-row
fixture hashes at f6fdeb5 from crc-verification.md, only if production library/
docs/fixtures/runner inputs stay unchanged. CRC will audit test-only regions.
No full docs rerun or original trace campaign required under those conditions;
uncertain/changed inputs must be rerun. Native test/lint/Clippy and changed actual
example demonstrations are affected and cannot be replaced by prior receipts.

Report every LF row with test name/path and decisive assertions, every F finding's
resolution/remaining gap, actual commands/exits/failures/retries, file-region audit,
scope/unchanged-main/worktree checks, prompt SHA and full commits. Walk Arc05
A-05-01/02 individually:01–03 accepted configuration/metadata/cables;04 lifecycle
still proposed here;05 removal/handles/commands/global BPM,07 transport,13 typed
catalog forms remain explicit owners. R-01–R-04 and project/release stay open.

Commit only named allowed changed paths on source, then separately the two return
files in planning using `git commit --only`. Preserve required repository trailers
if applicable; no blanket staging/amending historical records. Return through
Duncan; all LF statuses stay open with CC proposed-done evidence until CRC accepts.
Slice05 and Arc12 acceptance remain held. No new approval role or checker.
