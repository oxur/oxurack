# CC iteration03 — finish reset preservation and continuation proof

2026-10-12. Sofie/CRC → Duncan → CC. Three contributors, Expedited Mode.
Correction3/5; two corrective passes executed. Active assignment:
`arc12-live-rack-operations/slice04-runtime-api/cc-prompt-iteration03.md`.
Predecessor `cc-prompt-iteration02.md` is preserved unchanged.

## Outcome, authority and baseline

Complete the remaining F-03/F-04 observations in the **Iteration02 independent
review** of crc-verification.md. LF-01/LF-02 are independently done at03825e5;
LF-05/LF-06 retain acceptance at1242a60. LF-03/LF-04 remain open. No lifecycle
product defect was reproduced. This is a small test/evidence correction under
CDC12 D1/D2/D4 and directive08's proportionate-proof policy; no API redesign.

S=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, clean source
`03825e5077b7cd621185ae70b47020da321706c4`.
W=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch planning;
issuance descends from CC return `18abe111ce065979259cfdcece3c5557d6e16fbe`.
P=W/project01-mvp; A=P/arc12-live-rack-operations; D=A/slice04-runtime-api.
Protected R=`/Users/oubiwann/lab/oxur/oxurack`, main
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
K=`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`;
F=K/collaboration-framework/knowledge. Framework2.6.0; Rust-guidelines2.0.0.

Check cwd/branch/HEAD/status at R/S/W. Unexpected source drift returns through
Duncan/CRC; do not reset another contributor's changes. After coherent intake,
execute without another permission gate. No main edits, merge, push, endpoint,
Slice05 implementation or planning directory. Preserve all four accepted rows.

## Required reading and CC intake

Full means beginning through EOF. Named sections include complete bodies and
necessary definitions. Load complete contents in bounded contiguous chunks;
recover truncated output. Search hits, hashes, another context's reading and
successful shell exits do not prove your context received the required text.
Append itemized source state/loaded extent/recovery/conditional applicability and
a concise source-cited contract readback to **D/closing-report.md before dependent
edits**. No new intake file or dataset query. Readback must explain why resetting
both controls can hide rewinding, why an after-first-reset baseline cannot prove
first-reset preservation, and why restart does not replace save/unload/reload.

| Order | Exact input / state | Required extent | Purpose |
| --- | --- | --- | --- |
| 1 | W/AGENTS.md; S/CLAUDE.md | required-full both | Roles, worktrees and Rust/testing conventions |
| 2 | This prompt; D/slice-plan.md; D/ledger.md | required-full all three, issuance | Active work, accepted versus open rows and preservation |
| 3 | D/crc-verification.md | required-section: full Iteration02 independent review; Iteration01 Delta/intake/retained evidence and Fresh independent checks | Current gaps and pinned evidence eligible for retention |
| 4 | P/cdc-directive12.md,67e9150 ancestor | required-section: D1 lifecycle semantics, full D2, D4 Discriminating proof and Guidance/readiness | Reset/phase/failure/output contract;05 proof is successor-only |
| 5 | P/cdc-directive08.md | required-section: Prompt quality for all remaining work; Checks and correction policy | Proportionate correction and native gates |
| 6 | P/project-plan.md; A/arc-plan.md; A/ledger.md | required-section: project top table/Current direction; arc Capability/Five delivery slices/Current lifetime contract; required-full arc ledger | Higher-scale gates and unchanged04→05 authority |
| 7 | K/collaboration-framework/SKILL.md; F/project-management/guides/README.md | required-full both | Operational routing/Expedited Mode |
| 8 | F/engineering-methods/guides/07-implementation-prompt-authoring.md | required-full | Complete-loading/readback and source-grounded proof |
| 9 | F/project-management/guides/02-canonical-planning-worktree.md;03-planning-top-down.md; F/work-verification/guides/05-independent-verification.md | required-section:02 Slice iteration filenames;03 Issuing/executing an iteration; required-full05 | Preserved assignment, return and retention |
| 10 | K/testing/SKILL.md; F/testing/guides/01-testing-discipline.md;03-validation-gates.md | required-full all three | Real behavioral oracles and mandatory gates |
| 11 | K/rust-guidelines/SKILL.md; K/rust-guidelines/guides/11-anti-patterns.md | required-full both; anti-patterns before Rust edits | Rust applies to execution, not just authoring |
| 12 | S/crates/oxurack/src/lib.rs,03825e5 | required-section: imports and cfg(test) imports; complete set_port_value, midi_facade_denies_unread_next_sample_before_either_clock_advances, pending_output_rejects_lifecycle_and_reset_preserves_then_cleans_exact_state (including nested RetainedState/helper), failed_restart_snapshot_and_unloaded_restart_preserve_admission_boundaries | Existing private access and exact fields; only these tests/helpers need repairs |
| 13 | S/crates/oxurack/tests/lifecycle.rs,03825e5 | required-section: imports/constants/rack; complete edited_live_snapshot_restart_matches_independent_expected_trace_and_scope, stop_start_and_reset_at_sample_300_match_continuous_controls and capture_replay/trace plus their type aliases | Real musical controls/current restart and retained helper vocabulary |
| 14 | `git show 1242a60fa3a997f95b29195de3b28f7e9a2864ff:crates/oxurack/tests/lifecycle.rs` | required-section: complete edited_saved_lifetime_replay_matches_fresh_trace_and_scope, including its test attribute | Restore the actual accepted regression, not its prose description |
| 15 | S/crates/oxurack/src/lifecycle.rs; S/crates/oxurack-mod-midiout/src/lib.rs | required-section: lifecycle reset/restart/require_stopped_and_drained; MIDI-out prepare_silence/silence_outputs and consume_midi_outputs plus supporting state/Inputs definitions | Mutation boundary and literal one-old-release exception |
| 16 | S/crates/oxurack/examples/patches/turing-machine-lfo.ron; S/Makefile | required-full both | Seeded fixture and native commands |
| 17 | D/closing-report.md | required-section: complete Iteration02 intake/evidence/gates/attempts | Append corrections; preserve previous observations and failed attempts |

Conditional: before using an unfamiliar Bevy API, inspect its pinned local
source. Time<Virtual> has elapsed/delta/is_paused, **no overstep accessor**;
Time<Fixed> has elapsed/overstep. Existing APIs suffice; add no dependencies.
Reference-only: earlier prompts and historical complete plans; their current
obligations are synthesized here, not simultaneous full assignments. Budget
approximately35–50k tokens for required text/source, focused edits and recovery;
chunk rather than skim. On compaction revalidate state/reload unreliable contracts.

## Applied guidance and inspected route

CRC inspected03825e5's actual four-file delta and relevant existing tests,
reset/restart/snapshot/consume bodies and1242a60's preserved replay function.
CRC independently ran the required gates and the existing offline example with
LFO fixture plus note_cc74/velocity_cc75 for1300 samples. This grounds the literal
control notes/times below. No example or product source change was made by CRC.

| Author-applied guide / strength | Decision and reason | Proof |
| --- | --- | --- |
| guide07 §5, Corrective iterations | Keep the comparison arm unreset, observe before the first mutation and preserve the older distinct operation path | Three bounded repair groups below |
| Rust AP-12/AP-18 AVOID | Intentional owned snapshots, with borrows ending before reset; use distinct rack controls instead of cloning/mutating an observation target | Existing RetainedState and per-sample traces |
| Rust AP-09/AP-42 AVOID | Exact typed errors and useful test assertions; test-only expect/unwrap is allowed, no production panic changes | Pre-reset real fault captured separately from returned error |
| Rust AP-44 AVOID | Keep private clock/message inspection in inline tests; real musical continuation in integration tests | No public mutable World/test accessor |
| Rust AP-35 AVOID | Explain300/376/626 and62 as fixture-derived boundaries/note, not arbitrary exclusions | Literal cleanup, exactly one excluded release, full later equality |
| Testing Proportionate Proof; independent-verification retention | Fix the invalid/missing original proof using existing helpers and gates; retain unaffected evidence at its original revision | No new verifier, broad mutation campaign or hardware gate |

**Binding:** approved behavior/oracles and preservation below. **Recommended:**
extend named tests, extract a small cfg(test)-only observation helper if useful.
**Local discretion:** helper names, private grouping and diagnostics that preserve
all assertions. Any red product-behavior test requires root-cause diagnosis and
return through Duncan/CRC before out-of-scope production repair. Do not weaken
expectations to obtain green tests. Sketches are illustrative, not compiled code;
the actual uninterrupted control recipe has been exercised by CRC.

## 1. F-03 / LF-03 — compare the first reset to pre-reset state

At03825e5 lib.rs:2617 reset happens **before** the baseline at2622. The second
reset comparison cannot detect a first reset that changed retained state.
Reuse the existing complete RetainedState helper, capture immediately before
first reset, and compare immediately after it, allowing **only** pending count1→4.
No tick/drain/mode change between those observations. Keep every current field:
patch/driver/sample/running, Fixed elapsed/overstep, Virtual elapsed/delta/pause,
Scope selectors/capacity/overwrites/full records, fault, installation order and
typed ModuleAdded/PatchLoaded payloads. Retain second-reset count4/full-state
comparison, literal four-event drain and exact sample1 lone NoteOn retrigger.

```rust
// Illustrative: existing local RetainedState has an owned pending_events field.
let mut expected = retained_state(&mut rack); // BEFORE the first reset
assert_eq!(expected.pending_events, 1);
expected.pending_events = 4; // the only permitted observed-field change
rack.reset().expect("first stopped reset");
assert_eq!(retained_state(&mut rack), expected);
// Keep the existing second reset and exact event vector assertions.
```

Also compare the existing running reset (after sample1 retrigger, sample time2)
to its pre-reset typed state, allowing pending0→3 only; keep exact cleanup2.
This rejects stopping or changing clocks/Scope/metadata on the running path.
The raw-input note state is intentionally silenced; it is not a retained field.

In existing midi_facade_denies_unread_next_sample_before_either_clock_advances,
capture **the actual first fault before first reset**. It is the real generated
backpressure case: returned completed_samples1, stored completed_samples0 with
pending_messages1. Compare the saved stored fault immediately after first reset,
and keep later repeat/drain assertions. Capturing a new expected fault after the
operation can mask clearing it. Preserve sample1/Fixed1ms/Virtual1ms and stopped
state across that first reset as already required. The existing helper uses a
stable Option<String> fault representation; typed variant/fields may be used
instead. Do not compare HashMap Debug output or add an error PartialEq API.

LF-03's accepted observations at03825e5 — dirty/unused/CC/released channels,
all128 pending values,129th append rejection, capacity/fault/state preservation,
second actual silence and exact reset4/retrigger — remain untouched.

## 2. F-04 / LF-04 — reset one rack, retain an uninterrupted control

Keep the existing stop/start half. In the separate reset-at300 half of
stop_start_and_reset_at_sample_300_match_continuous_controls, **do not call reset,
stop, restart or rebuild on reset_control**. Both racks run the same fixture/
seed and CC74/75 configuration, draining one admitted sample at a time.
At300 only reset_subject resets. Keep immediate subject time/Scope/config
preservation and both prefix/scalar comparisons. Continue to1300 through a
future clock edge, with zero Scope overwrites and matching selector configuration.

For this actual fixture, CRC's uninterrupted offline run observes:

- `(251,[0x90,62,21])` is the active old note at300.
- Subject reset cleanup is **exactly**
  `[(300,[0x80,62,0]),(300,[0xB0,123,0]),(300,[0xB0,120,0])]`.
- Control's natural `(376,[0x80,62,0])` is absent from subject continuation,
  because reset already released62. Every CC74/75 at376 still matches.
- Next shared NoteOn is `(626,[0x90,64,44])`. All events from626 through1300
  compare exactly, including later NoteOffs and the1251 Off/On sequence.

Keep cleanup separate from continuation traces. Compare **all** scalar records
for clock pulse, LFO bipolar, TM note/gate. For continuation MIDI the sole allowed
exception is one exact control tuple `(376,[0x80,62,0])`; assert it occurs exactly
once and remove only that occurrence from an owned expected control vector.
Then assert complete subject continuation equality. Do not filter all NoteOffs,
use status-only comparisons, truncate at the next NoteOn or reset both controls.
Explicit full suffix equality at626 plus presence of a later release makes the
reconvergence assertion readable. Require the literal626 NoteOn in both arms.

```rust
// Illustrative, using owned Vec<(u64,[u8;3])> observations, cleanup separate.
let old_release = (376, [0x80, 62, 0]);
assert_eq!(control_future.iter().filter(|e| **e == old_release).count(), 1);
let mut expected_future = control_future.clone(); // intentional independent oracle
let release_index = expected_future.iter().position(|e| *e == old_release)
    .expect("control naturally releases the note silenced at300");
expected_future.remove(release_index);
assert_eq!(subject_future, expected_future);
// Also compare all scalar records, literal cleanup and full suffix >=626.
```

This distinguishes narrow voice silence from phase/RNG rewinding that two equally
reset racks could hide. It does not claim endpoint or hardware silence.

## 3. F-04 — preserve the distinct replay path and immediate error state

Restore **alongside** the passing current-restart test the entire accepted
`edited_saved_lifetime_replay_matches_fresh_trace_and_scope` function from1242a60.
It edits/saves at200, continues100 more samples, unloads/drains/reloads the saved
file, then compares10000-sample MIDI/scalar/copied-MIDI captures with a fresh rack,
72 events and zero overwrites. Restore its test attribute. Copy only this function;
do not replace the whole file, erase new tests or undo the restart witness.
Private helper reuse is fine; save→unload→drain→load remains the observed path.
This was an explicit preservation requirement in iteration02, not extra feature
scope. LF-06's unchanged actual example receipt remains accepted independently.

In inline failed_restart_snapshot_and_unloaded_restart_preserve_admission_boundaries,
complete the same disabled-cable error observation already assigned. After setting
Cable.enabled=false and **before restart**, capture the old owner's Fixed elapsed/
overstep, Virtual elapsed/delta/pause, SimulationRunning, first fault, Scope
selectors/capacity/overwrites/all records and typed ModuleAdded/PatchLoaded payloads.
Include existing module identities/handle, selected disabled cable and install
order. Compare immediately after the exact UnsupportedSnapshot disabled-cable Err,
before restoring cable.enabled=true or calling another operation. No snapshot
unwrap inside the disabled-state helper: disabled snapshots intentionally fail.
Keep the earlier valid configuration snapshot, then reenable only after error
observations and retain its existing equality check. A small private record or
explicit field assertions are sufficient; no general observation framework.

Also make the current-restart success test's already required stopped/fresh0/
Manual/no-fault/**ScopeNone** assertions explicit before configure_scope. The
existing replay then starts and checks all future values. Keep all accepted
admission, empty-rack, real fault→failed candidate→fresh note0, install and Live
no-catch-up assertions. No broader operation Cartesian matrix is requested.

## Exact scope, fresh gates and retained evidence

Only source paths, **test regions only**:

1. `crates/oxurack/src/lib.rs` — cfg(test), named assertions/private helpers.
2. `crates/oxurack/tests/lifecycle.rs` — named continuation/error/success tests and restored replay function.

No production prefixes, MIDI-out/lifecycle production or tests, examples, docs,
fixtures, manifests, lockfile, graph/native RT/standalone engine changes.
Planning edits only D/closing-report.md and D/ledger.md; append iteration03
intake/attempts/observations/row walk/retention proposal. Do not edit prompts,
CRC review, plans, parent ledgers or status. No durable supporting artifacts
expected; use transient /tmp output. If a durable failure receipt is necessary,
return its reason/placement through CRC rather than inventing another tree.

Run from S after final edits, preserving every failed attempt/retry separately:

```bash
cargo test --locked -p oxurack-mod-midiout
cargo test --locked -p oxurack --all-targets
cargo test --locked -p oxurack --no-default-features --all-targets
make test
make lint
cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings
git diff --check
```

Both facade modes must exercise the repaired assertions, not only compile them.
Known restricted make test fails four CoreMIDI initialization cases; preserve
that run and retry **unchanged** with authorized host access, record both exits/
RT denominators/3 existing ignores. No suppression/new ignore. Preserve existing
no-default cables.rs unused_mut and block advisory; unrelated repair not assigned.
No docs/build/fixture demo repetition is required for this test-only change:
propose retention of CRCf6fdeb5 docs/original pins and CRC1242a60 actual example/
core/error receipts, plus03825e5 LF-01/LF-02/peak/real recovery evidence. State
original dates/revisions and unchanged inputs/code rationale. Fresh commands above
are mandatory even where they also rerun previously accepted assertions.

Separate source and planning commits, exact paths only; inspect status/staging
before committing. From S:

```bash
git diff -- crates/oxurack/src/lib.rs crates/oxurack/tests/lifecycle.rs
git diff --check
git commit --only -m "test(oxurack): finish reset continuation and preservation evidence" -- crates/oxurack/src/lib.rs crates/oxurack/tests/lifecycle.rs
```

From W, commit only
`project01-mvp/arc12-live-rack-operations/slice04-runtime-api/closing-report.md`
and `project01-mvp/arc12-live-rack-operations/slice04-runtime-api/ledger.md` using
`git commit --only`. Preserve earlier evidence and prompts; source/main unchanged
except the two authorized test regions. Do not blanket-stage shared files.

Return this prompt's identity/hash, source/planning commits and changed paths,
intake extents/readback, actual oracles including cleanup/control exception,
all command attempts/results, all six row dispositions and inherited-owner walk.
Keep LF-01/LF-02/LF-05/LF-06 done with CRC provenance; LF-03/LF-04 remain **open**
with CC proposed evidence until CRC accepts. Do not claim Slice04/Arc12 closure,
release acceptance or Slice05 advancement. A red behavior test stays unresolved;
diagnose/report the actual failure instead of labeling proposed-done.
