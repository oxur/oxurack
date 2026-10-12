# CC iteration01 — configured readback and persistent cable order

Issued 2026-10-11 by Sofie/CRC, through Duncan. Execute this implementation
correction and return to CRC. Three contributors; Expedited Mode; refinement1/5.
Project01-mvp / Arc12 / Slice01. All LP-01–LP-05 remain open, none accepted.
CDC directive11 resolves the two design conflicts; no further design round trip
is needed to implement these decisions. Subsequent slices remain held for CRC.

## Assignment, baseline and preservation

Current assignment relative to project01-mvp:
`arc12-live-rack-operations/slice01-live-persistence/cc-prompt-iteration01.md`.
Predecessor: sibling `cc-prompt.md`, returned at source a3633bc / planning9fdb2cd.
Preserve that issued prompt and the initial CC return in Git and in the existing
closing-report.md; append a clearly identified iteration01 return there.

Use these roots throughout this prompt:

- **S**: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
  branch `codex/oxurack-module-behavior-installation`, clean source intake
  `a3633bc0d0f468e933cc0f1463fb637a623b33d1`.
- **W**: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch `planning`.
  Author intake `37d41f675236834ff093ff2b93b7c1dad09c74df`; this issuance adds
  planning only. Require the current slice-plan to identify this assignment;
  record the actual planning intake HEAD, rather than requiring the old author HEAD.
- **P**: W/project01-mvp. **D**: P/arc12-live-rack-operations/slice01-live-persistence.
- **K**: `/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
  Installed collaboration-framework2.6.0 and rust-guidelines2.0.0 were inspected.

Check worktree/branch/HEAD/dirty state before editing. Do not reset unrelated work.
Source drift that changes these contracts returns to CRC with exact evidence;
routine local compiler/borrow/helper adjustments within the nine-path scope below
are authorized. Main stays clean at a7ce6e61536ca96b6aeb2227ee06e430ed8df162.

**Binding outcome:** live parameter edits save configured values, and a fresh rack
replays those values and the effective cable order. Saving does not checkpoint
engine/RNG/phase state. Directive11 supersedes only the initial prompt's effective
registered-getter assumption, Entity-only tie policy and source allowlist.
Its remaining API, extraction, failure, preservation and replay requirements apply.

CRC reproduced at a3633bc:

- `cargo test --locked -p oxurack --test persistence live_configuration_saves_and_replays_in_fresh_racks -- --exact`:
  exit101; 0 passed/1 failed/1 filtered. Saved trace differs from the independently
  edited .25-base reference. Transient log `/tmp/oxurack-crc-a3633bc-replay.log`.
- `cargo test --locked -p oxurack --lib persistence::tests::parallel_cable_tie_order_survives_snapshot_and_fresh_load -- --exact`:
  exit0; 1 passed/39 filtered. This test currently asserts first=.25, replay=.75,
  so it is a diagnostic, not acceptance. Log `/tmp/oxurack-crc-a3633bc-tie.log`.
- Existing offline runner on the unmodified LFO fixture at 1000 samples exits0,
  with first MIDI row `251|90 3E 15`. The initial sample252 pending-event oracle is
  feasible. These author checks are a bounded subset, not full LP acceptance.

## Required reading and execution preflight

Read in order. Required-full means the whole named file; required-section means
the complete named definitions/functions and their dependencies, not grep snippets.
If output truncates, recover the omitted ranges. Confirm no required material is
missing before source edits. Resolve the local installed skill root if K differs.
An unavailable governing skill is a disclosed intake gap, not silently skipped.

| Order | Path relative to indicated root | Extent and reason |
| --- | --- | --- |
| 1 | S/CLAUDE.md; W/AGENTS.md | required-full; repository, role and worktree instructions |
| 2 | This prompt; D/slice-plan.md; D/ledger.md | required-full; current execution identity, readiness, five criteria |
| 3 | P/cdc-directive11.md; P/arc12-live-rack-operations/arc-plan.md | required-full; resolved getter/order policy, boundaries and retained ownership |
| 4 | P/project-plan.md | required-section: Current direction through Design handoff history and directive11 acknowledgement; older appended current assignments are historical |
| 5 | P/cdc-directive08.md | required-section: Assessment and decisions; Prompt quality for all remaining work; Checks and correction policy from Slice03 onward |
| 6 | D/cc-prompt.md | required-section: Product outcome and source findings; Binding API and behavior including Configuration extraction; Tests and exact oracles. Apply directive11's explicit replacements; the old source fence is superseded |
| 7 | D/closing-report.md | required-full at intake; preserves the actual failed/partial initial return, not accepted proof |
| 8 | K/collaboration-framework/SKILL.md; K/rust-guidelines/SKILL.md | required-full; apply installed skill routing and Rust execution guidance |
| 9 | K/collaboration-framework/knowledge/project-management/guides/README.md; K/collaboration-framework/knowledge/project-management/guides/02-canonical-planning-worktree.md; K/collaboration-framework/knowledge/project-management/guides/03-planning-top-down.md | required-full README; required-section Slice iteration filenames and preservation in02, Issuing and executing an iteration in03 |
| 10 | K/collaboration-framework/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md | required-section: Required reading and CC intake (within Assemble a usable assignment), including CC preflight; author has applied the entire guide and companion template |
| 11 | K/collaboration-framework/knowledge/testing/guides/03-validation-gates.md | required-full; native gates and proportionate proof |
| 12 | K/rust-guidelines/guides/05-type-design.md; K/rust-guidelines/guides/03-error-handling.md; K/rust-guidelines/guides/02-api-design.md; K/rust-guidelines/guides/11-anti-patterns.md | required-section: TD-02, TD-03; EH-01; API-02, API-23; AP-42, AP-43, AP-44 respectively, including rationale/examples |
| 13 | S/crates/oxurack-mod-turingmachine/src/rack.rs | required-section: TuringMachineState, ConfigCandidate::from_state, apply_candidate, consume/produce/reconcile functions, set_parameter and write-probability getter, existing held_modulation test, registered get/set, output_value and deep_observation test helpers. Inspect rejection-preservation tests reused below |
| 14 | S/crates/oxurack-mod-turingmachine/README.md; S/README.md | required-section: rack adapter introduction/parameter and output semantics; live-configuration persistence and its runnable recipes |
| 15 | S/crates/oxurack-core/src/cable.rs; S/crates/oxurack-core/src/lib.rs | required-section: Cable and its imports; cable re-exports, CorePlugin/CorePluginFixed and build_core_plugin schedule wiring |
| 16 | S/crates/oxurack-core/src/patch.rs | required-section: Patch/ModuleConfig/CableConfig; validate_patch and fan-in validation; PatchHandle and full apply_patch_to_world/port-resolution helpers; test_apply_patch_to_world_spawns_cable and its existing fixtures |
| 17 | S/crates/oxurack-core/src/tick.rs | required-section: PropagationOrder/dirty flag, rebuild function and system, propagation/merge systems and LastWins apply_merge; spawn_test_module/port, test_app and cached ordering integration tests |
| 18 | S/crates/oxurack/src/persistence.rs | required-section: all production; loaded_rack, direct event, pending/Scope/continuation and failure tests; TieSource/TieSink and complete parallel tie test. Reuse active-handle/header/getterless tests without changing their contracts |
| 19 | S/crates/oxurack-core/src/event.rs; S/crates/oxurack/src/lib.rs; S/crates/oxurack/src/scope.rs | required-section: ParameterChanged/ChangeSource and shared set_module_parameter; facade world/world_mut, driver/time/advance/drain accessors; ScopeRecord/capture iterator and overwrite accounting. Observation context only; no edits outside allowlist |
| 20 | S/crates/oxurack/tests/persistence.rs; S/crates/oxurack/examples/save-live-patch.rs; S/crates/oxurack/examples/patches/turing-machine-lfo.ron; S/crates/oxurack/Cargo.toml | required-full; independent reference, unchanged runnable example, exact fixture and feature/dependency wiring |

Expected bounded intake is roughly 35–50k tokens, leaving implementation headroom;
do not load legacy histories, whole unrelated guides or prior raw validation logs.
Other directives, older accepted adapter packets and unrelated source tests are
reference-only unless a concrete dependency appears. No required-data-only files.
Before editing, add concise intake/readback to the existing closing-report.md:
actual assignment/hash/roots/HEADs, completed read extents and resolved skill paths,
approved replacements, unchanged obligations, planned tests, and any real blocker.
Do not create an intake validator or treat this readback as acceptance.

## Binding design and recommended implementation

The following Rust is illustrative and uncompiled. Symbols/owners were checked
at a3633bc; adapt local imports/borrows/helper names while retaining the behavior.

### 1. Read the configured TM knob; retain effective engine/output

In `crates/oxurack-mod-turingmachine/src/rack.rs`, change only the existing
registered getter's source of truth:

```rust
fn get_write_probability(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Float(state_ref(world, entity)?.write_base))
}
```

Existing set_parameter constructs/decodes/validates a candidate before mutation,
stores write_base, and reconciles clamp(base + held CV). Keep that sequence.
Do not change standalone engine.write_probability(), effective StepOutputs,
pending snapshot publication, RNG, seed or action priority. There is already
one correct base field; no cache, alternate registry or snapshot-only TM path.

Facade parameter readback, shared DirectApi event.previous and snapshot now all
use the configured getter. `event.rs` and facade `lib.rs` need no correction.
Document the distinction in the adapter README and top-level persistence docs:
registered write_probability is the knob; engine and same-named output are
effective/clamped, with output published from a step one sample later. Historical
Arc10 effective-getter acceptance remains recorded at its original revision.

### 2. Attach logical order when loading a patch

Add beside Cable in `crates/oxurack-core/src/cable.rs`, and re-export in lib.rs:

```rust
/// Position in the loaded cable list; ordering metadata, not an entity identity.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CableOrder(pub usize);
```

In apply_patch_to_world, enumerate the existing Patch.cables loop and spawn
`(cable.clone(), CableOrder(ordinal))` together. Keep endpoint resolution,
CableIndex insertion, CableConnected, labels and final dirty marking as today.
Extend the current patch-load test to assert metadata matches vector positions,
including two cables if needed to discriminate ordinal assignment.
No Cable/CableConfig field, RON version change or separately serialized ordinal.

### 3. Use one policy in core propagation and snapshot extraction

Both tick.rs rebuild_propagation_order **and its system query** must read
`(Entity, &Cable, Option<&CableOrder>)`. Match that optional query in persistence.rs.
After resolving the existing names/ports, the sort key in both consumers is:

```rust
let (absent_order, ordinal) = match order {
    Some(order) => (false, order.0),
    None => (true, 0),
};
let key = (
    target_name, target_port_name, source_name, source_port_name,
    absent_order, ordinal, cable_entity,
);
// Sort by key; retain entity/CableConfig separately as current code does.
```

The four names/ports remain primary. Within an identical endpoint tuple,
ordered cables precede absent metadata; explicit ordinals ascend; equal or absent
ordinals fall back to ascending Entity in this world. Snapshot writes this actual
effective order to Patch.cables; fresh load assigns distinct vector ordinals,
preserving even a resolved raw-world fallback order. Missing/duplicate ordinals
alone have no cross-world identity promise. Do not reverse spawning, sort by
transform/label/value or introduce a global identity service.

Core rebuild stays dirty-triggered O(C log C); normal propagation keeps its
cached cable vector and current Cable query. Snapshot remains synchronous
configuration extraction. No extra scheduler, per-sample sorting or I/O.
Correct stale comments about module IDs/invalid ties; duplicate endpoints with
LastWins are valid. Document raw-world CableOrder edits requiring
mark_propagation_order_dirty. The future Slice03 runtime API must append order
after existing ordered cables and mark dirty; its overflow/mutation policy is
not new work for this iteration.

### Applied Rust guidance

The author read these actual sections and used them in the recipe. CC applies
the same separately installed skill during implementation; record it on return.

| Source / strength | Applied decision and fit | Target and observable proof |
| --- | --- | --- |
| rust-guidelines guides/05-type-design.md TD-02/TD-03, SHOULD | Small public semantic component distinguishes ordinal from Entity; derive ordinary ordering traits, no ID service | CableOrder attached at load; explicit ordinal defeats opposite Entity order |
| guides/02-api-design.md API-02, SHOULD | Preserve borrowed names and owned ParameterValue/Patch; no public world/state accessor to rescue tests | existing facade APIs; inline private observations and public replay |
| guides/02-api-design.md API-23, MUST; guides/03-error-handling.md EH-01, MUST | Keep existing concrete CoreError/Result and extraction-before-write; errors remain recoverable and contextual | wrong-type/NaN rejection, getterless/unsupported snapshot and I/O tests |
| guides/11-anti-patterns.md AP-42, AVOID | Fallible production lookup uses existing state_ref/Result and contextual snapshot errors rather than chained unwraps | missing state/getter error behavior; no new panic in extraction |
| guides/11-anti-patterns.md AP-43, CONSIDER; AP-44, AVOID | Keep concrete query keys and owned facade types; a new generic reflection/ID service or exposed Bevy test accessor adds coupling without a caller need. CableOrder belongs to the already ECS-facing core | nine-file repair, private inline observations, public facade replay; no new dependency/cache/harness |

## Behavioral proof and remaining five-row walk

Extend existing tests in the allowed files; no separate harness/package or public
test accessor. The original slice's complete contracts still govern. The following
cases are repairs/completions of those contracts, not discretionary new gates.

| Rows / home | Setup and action | Required observation and credible mistake rejected |
| --- | --- | --- |
| LP-01/04; TM rack inline | Existing held_modulation_reconciles_targets_and_registered_base_edits: base.5 with CV [-1,-.25,0,.25,1], repeated holds; then CV+.25 with old pending.75, set base.25 | Getter stays.5 across CV first loop, then.25; engine and new step.5; old pending.75 unchanged and published next sample; following pending/output.5 one sample later. Keep engine and actual CurrentValue output assertions using output_value, not just getter assertions |
| LP-01; facade persistence inline | Reuse loaded_rack; in this test only, make existing LFO→TM modulation cable Affine(factor0,offset.25), identify it by endpoints; advance/drain200; clear load/tick notifications, set TM base.5→.25 through facade | one DirectApi event previousFloat(.5), newFloat(.25); parameter reads.25, no time/running change. Reject Int and Float(NaN) before more ticks: no additional success event/base change. Continue through next real step and assert effective output.5. Compare continuation with identically configured control; private TM inline rejection/deep_observation also proves engine/pending/ports unchanged immediately on rejection. Facade cannot access private TuringMachineState |
| LP-02/04; existing facade parallel tie test | Load unchanged duplicate-endpoint LastWins patch vector with offsets [.75,.25]. Remove diagnostic transform reassignment by sorted Entity. Tick, snapshot/save, fresh load, then save/reload a second fresh rack | literal sink.25 in all three racks; snapshot and deserialized saved vectors retain [.75,.25] at both rounds. No .75 replay expectation. This catches loader/propagation/snapshot disagreement and transform-based ordering |
| LP-02; core tick inline | Reuse raw-world module/port/cable helpers and CableIndex. For one identical-endpoint pair, deliberately oppose ordinal and Entity order: Entity-first gets ordinal1/contribution.25, Entity-second ordinal0/contribution.75. An unrelated target named a_sink with ordinal99 precedes z_sink's ordinal0 | actual cached order is lexical unrelated pair first, then ordinal0/ordinal1; z_sink receives literal.25. Derive Entity ordering from entities in that world, not allocator numeric assumptions. This raw fixture may assign transforms after identifying entities; the loaded-patch tie test above must retain its original vector/transforms |
| LP-02; same core fixture | Exercise equal explicit ordinals, both absent, and mixed present/absent; after raw edits call mark_propagation_order_dirty and tick again | equal/both absent use ascending Entity, yielding.75 with preceding setup; mixed present first then absent yields the absent cable's contribution. Assert exact cached order and merged values. Run these ordering cases through real CorePlugin Update and CorePlugin::fixed FixedUpdate; reuse existing schedule patterns, not direct calls to the sort helper |
| LP-02/04; facade tie fixtures | Compact additional fallback case with equal ordinals or absent metadata, determine actual within-world Entity fallback, then snapshot/save/fresh load | serialized offsets equal that known fallback order; loader creates distinct vector ordinals; sink winner and saved vector survive reload. May consolidate both fallback modes in one test using existing TieSource/TieSink |
| LP-03; existing pending/Scope/continuation inline | Two identical advancing racks; only subject calls both snapshot AND save at completed_samples200 and252; at252 leave first event sample251 unread. Keep existing control and caller drain | time, driver/running, collected Scope records/overwrite count unchanged by each operation; at252 check same OutputBackpressure count before/after without draining to observe it, then caller drains once and compares exact event251 plus continuation through1000. Existing test only snapshots at pending point; complete its original save/200 obligations |
| LP-04; tests/persistence.rs | Keep live_configuration_saves_and_replays_in_fresh_racks: advance200 with real sine CV, set LFO1 and TMbase.25, save, compare two fresh saved racks to independently edited fixture reference10000 | explicit saved RON TM Float(.25) and LFO Float(1), original header; all three nonempty MIDI traces equal and differ from original. Fresh LFO observation0; saving rack continues its phase. Do not substitute effective .5426204, drop modulation or construct reference from saved snapshot |
| LP-01–LP-05; retained existing tests | Existing LFO live edit/default getter extraction/metadata/header/active handle/empty patch/getterless/disabled/cross-boundary/error and example behavior | retain original exact error/provenance, no success event on error, sentinel destination untouched, owned snapshot and failed-load identity. Rerun; prior CC passes are attestation, not independent acceptance |

In facade inline tests use the rack's existing private world/world_mut, Messages,
port/ChildOf queries, Scope iterator, MidiOutputBuffer::check_admission and public
drain/parameter APIs. Do not consume the rack with into_app before snapshot/save.
Compare rejection state before any later success can hide it. TM output publication
and configured readback are separate assertions. Local test consolidation is fine.
Demonstrate corrected base-readback and loaded-vector acceptance assertions fail
on a3633bc where feasible; do not revert production or replace required final gates
with this regression demonstration. The reproduced initial failures above remain
valid history even if no additional baseline run is needed.

## Native checks and executable replay

Run from S on final source; record every command, revision, exit and useful summary,
including failed, blocked, ignored and unrun attempts. Complete the native gates
that previously stopped at the genuine replay failure; do not relabel them passed.

```bash
cargo test --locked -p oxurack-mod-turingmachine
cargo test --locked -p oxurack-mod-turingmachine --features rack
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack --lib --test persistence --test scope --test lfo --test midi_output --example save-live-patch --example turing-machine-midi
cargo test --locked -p oxurack --no-default-features --lib --test persistence --test scope --test lfo --test midi_output --example save-live-patch --example turing-machine-midi
make test
make lint
cargo clippy --locked -p oxurack-core -p oxurack --all-features --all-targets -- -D warnings
cargo clippy --locked -p oxurack-mod-turingmachine --features rack --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
git diff --check
```

Run both existing examples and fresh processes. A feasible copy/paste recipe
below edits only temporary files; Python here is a one-off text copy, no new
product or committed validator. The independently constructed reference changes
only the fixture's single LFO frequency entry, matching save-live-patch's edit;
the native integration test above independently covers both LFO and TMbase edits.

```bash
OXURACK_PERSIST_DIR=$(mktemp -d /tmp/oxurack-persistence.XXXXXX)
cargo run --locked -p oxurack --example save-live-patch -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --save "$OXURACK_PERSIST_DIR/edited.ron"
# Expected: Float(0.5) -> 1.0, destination, completed_samples=300; exit0.
python3 - "$OXURACK_PERSIST_DIR/reference.ron" <<'PY'
import pathlib, sys
text = pathlib.Path('crates/oxurack/examples/patches/turing-machine-lfo.ron').read_text()
old = '"frequency_hz": Float(0.5)'
assert text.count(old) == 1
pathlib.Path(sys.argv[1]).write_text(text.replace(old, '"frequency_hz": Float(1.0)'))
PY
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_PERSIST_DIR/edited.ron" --samples 10000 \
  --scope-view --scope-port lfo_main.bipolar_out > "$OXURACK_PERSIST_DIR/view.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_PERSIST_DIR/edited.ron" --samples 10000 > "$OXURACK_PERSIST_DIR/saved-1.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_PERSIST_DIR/edited.ron" --samples 10000 > "$OXURACK_PERSIST_DIR/saved-2.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_PERSIST_DIR/reference.ron" --samples 10000 > "$OXURACK_PERSIST_DIR/reference.trace"
cmp "$OXURACK_PERSIST_DIR/saved-1.trace" "$OXURACK_PERSIST_DIR/saved-2.trace"
cmp "$OXURACK_PERSIST_DIR/saved-1.trace" "$OXURACK_PERSIST_DIR/reference.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --samples 10000 > "$OXURACK_PERSIST_DIR/lfo-original.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron --samples 2000 > "$OXURACK_PERSIST_DIR/midi-original.trace"
wc -l "$OXURACK_PERSIST_DIR/"*.trace
shasum -a 256 "$OXURACK_PERSIST_DIR/lfo-original.trace" "$OXURACK_PERSIST_DIR/midi-original.trace"
```

All example runs and equalities must succeed; saved traces must be nonempty and
different from lfo-original.trace (a difference comparison should exit1).
Original LFO10000 remains70 rows, SHA-256
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`;
original MIDI2000 remains13 rows,
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
View stays usable with the expected 10000 signal observations; no endpoint opens.
The example edits only LFO, so its saved event count is not the TMbase.25 test's
oracle. Inspect native saved RON assertions as well as equality of traces.

Retain block0.1.6 advisory and the three pre-existing RT ignores accurately.
If restricted CoreMIDI blocks make test, preserve its failure and retry the exact
unchanged command with host access; unavailable host leaves the gate unproved.
No warning suppression, added ignore, dependency freshness/coverage/soak campaign
or endpoint witness is authorized/required here. No current LP row has prior CRC
acceptance to retain in place of these affected checks.

## Source scope, commits and return

Complete authorized source path set (relative to S; commit only changed paths):

```text
crates/oxurack-mod-turingmachine/src/rack.rs
crates/oxurack-mod-turingmachine/README.md
crates/oxurack-core/src/cable.rs
crates/oxurack-core/src/lib.rs
crates/oxurack-core/src/patch.rs
crates/oxurack-core/src/tick.rs
crates/oxurack/src/persistence.rs
crates/oxurack/tests/persistence.rs
README.md
```

No standalone engine, dependency/lockfile, schema, musical fixture, facade lib,
event/error module, save example or playback runtime edits. Those existing paths
may be read/run. New consequential conflicts or a genuinely necessary out-of-scope
change return to CRC with exact evidence; no weakening of approved oracles.
This packet resolves the two known conflicts, not an open architecture choice.

After self-review/checks commit the changed source paths explicitly with
`git commit --only -- <explicit changed paths above>`; no blanket staging.
Then update only D/closing-report.md and D/ledger.md with this iteration's intake,
row walk and evidence, and commit those two exact planning paths separately.
Preserve the initial return as a dated section; do not edit issued prompts or
claim any LP acceptance, slice/arc closure or advancement. No push/merge/release.

Return assignment path/hash, source and planning commits/dirty state, changed
paths, applied Rust guides, actual commands/results, LP-01–LP-05 proposed status,
self-review, deviations and remaining blockers. Record causal distinctions:
configured knob versus effective output; logical order versus Entity fallback;
configuration reconstruction versus continuing internal state. Concise evidence
belongs in the two existing records; no durable support artifact is expected.
If a useful failure artifact must be retained, name it first and use D/artifacts/,
the existing slice home, without inventing another evidence format.

CRC independently reproduces all five rows on return and closes only when justified.
The author readiness record is in slice-plan.md; it is not acceptance evidence.
