# CC assignment — edit the live rack, save it, replay the configuration

Execute Arc12/Slice01 and return to CRC through Duncan. Three contributors,
Expedited Mode; framework2.6.0 / prompt-authoring guide07. This is implementation
work. All five LP rows are open. Do not stop after producing another design plan.

## Identity, scope and required reading

Planning root P:
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp`
Source root S:
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
Source branch `codex/oxurack-module-behavior-installation`, inspected clean at
`72fe6b52197b7a138afc3d5d7ac970ddfdf66981`. Main is not the source worktree.
Assignment: `arc12-live-rack-operations/slice01-live-persistence/cc-prompt.md`.
Current assignment must match sibling slice-plan.md. Recheck worktrees/status;
report consequential source drift before dependent changes, preserving others' work.

Read in this order, before dependent edits. Required sections mean the complete
named section/function and definitions it depends on, not search hits. At the
pinned revision source symbols are the boundary where line numbers are omitted.

| Kind | Path / scope | Purpose |
| --- | --- | --- |
| Required-full | This prompt; sibling slice-plan.md and ledger.md | Implementation and acceptance contract |
| Required-full | P/cdc-directive10.md; P/arc12-live-rack-operations/arc-plan.md | Current authority, cross-slice lifecycle and scope |
| Required-section | P/project-plan.md: Current direction, through the policy paragraphs before Design handoff history | Current product sequence; historical sections are not simultaneous assignments |
| Required-section | P/cdc-directive08.md: “Assessment and decisions”, “Prompt quality for all remaining work”, and “Checks and correction policy from Slice03 onward” | Playable-first/proportionate proof; its historical Arc10 assignment is reference-only |
| Required-section | S/crates/oxurack/src/lib.rs: production code before cfg(test); existing inline live parameter fixture/getter tests and load replacement test | Facade construction/install/load, sample ownership, private test access |
| Required-section | S/crates/oxurack-core/src/patch.rs: Patch/RackConfig/ModuleConfig/CableConfig, validate_patch with helpers, serialize/deserialize/save/load and apply_patch_to_world with helpers | Existing schema, current load behavior and file error types |
| Required-section | S/crates/oxurack-core/src/module.rs: Module, OxurackModule and ModuleRegistration/ModuleRegistry production definitions; parameter.rs production definitions; event.rs dispatch_core_command/dispatch_set_parameter and ParameterChanged/ChangeSource | Live instance identity, complete getter enumeration, setter event reuse |
| Required-section | S/crates/oxurack-core/src/label.rs: label/Tag/RackMeta types; cable.rs: Cable; tick.rs: rebuild_propagation_order and apply_merge; error.rs: CoreError/PatchError; lib.rs export/plugin registration sections | Metadata, cable tie order, errors and exports |
| Required-full | S/crates/oxurack/examples/patches/turing-machine-lfo.ron; S/crates/oxurack/Cargo.toml | Real musical fixture and existing example dependencies |
| Required-section | S/crates/oxurack/tests/lfo.rs: loaded modulation/replay tests; S/crates/oxurack/examples/turing-machine-midi.rs: prepare_rack/run_trace and clear_and_continue_recipe test; README: current LFO/Scope usage | Reuse runnable patterns and per-sample drain |
| Required-section | Rust skill paths below: API-02/API-23, EH-01, AP-42/43/44 | Applied ownership/error/API guidance |
| Conditional | Each module adapter setter/getter implementation touched or relied on by a newly failing test | Inspect before changing behavior; module algorithms are outside this slice's default scope |
| Reference-only | Historical Arc02 phases5.3–5.5, carried-over Arc03–05 packets, Arc11 reviews | Provenance; current clarified contract is here |

Rust skill root is
`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/rust-guidelines`.
Guides: `guides/02-api-design.md`, `03-error-handling.md`, `11-anti-patterns.md`.
Use existing framework/domain session instructions too; these scoped references
bound the assignment's additional reading rather than waive governing instructions.
No required structured-data queries. Recover truncated text with contiguous reads.
Record loaded extents/source states and conditional applicability concisely in
closing-report.md, then a brief source-cited contract readback in your own words.
Identify the key trap: saving initial overrides or comparing saved configuration
against midstream engine continuation would implement the wrong feature. Resolve
material contradictions before affected edits; coherent preflight authorizes work.

## Product outcome and source findings

Duncan can advance the installed Clock/LFO/TM/MIDI patch, turn the LFO knob, save
what is currently configured, and use the saved file with the existing runner.
No device is needed to exercise this increment. A saved patch restarts from its
seed; it is not a checkpoint of the running engine's phase/register/RNG state.

The existing `save_patch_to_file(&Patch, &Path)` serializes supplied data only.
Oxurack does not retain seed/version/BPM and has no public parameter/snapshot/save
facade. `try_install` already registers schemas, setters and getters. Core command
dispatch already handles instance resolution, previous values, setter validation,
error name normalization and ParameterChanged — reuse this implementation rather
than create a second subtly different path. Current load appends entities and
replaces only PatchHandle; snapshot must use the active handle. Actual replacement
repair belongs to Slice04, explicitly recorded in the arc plan.

## Binding API and behavior

Add these inherent methods on Oxurack (names/signatures are binding):

```rust
pub fn parameter(&self, module: &str, param: &str)
    -> Result<ParameterValue, CoreError>;
pub fn set_parameter(&mut self, module: &str, param: &str, value: ParameterValue)
    -> Result<(), CoreError>;
pub fn snapshot_patch(&mut self) -> Result<Patch, CoreError>;
pub fn save_patch(&mut self, path: &Path) -> Result<(), CoreError>;
```

The mutable snapshot receiver permits Bevy query-state construction; it does not
permit semantic mutation. Return an owned Patch so callers can choose when/where
to serialize it. Calls are synchronous between sample executions, valid stopped
or running, Manual or Live. They do not start/stop/tick, drain/discard MIDI, clear
Scope, reset modules, adjust elapsed time or emit fake lifecycle notifications.
No threads, queues, async framework or allocation in a scheduled tick is added.
File I/O may take time; the caller chooses the save point. This is not hard-RT save.

Parameter resolution requires a loaded PatchHandle and exact instance name, then
Module.kind and ParameterRegistry. Use NoPatchLoaded, UnknownModule(name), and
existing UnknownParameter/InvalidParameterValue normalized to instance names.
Propagate getter errors; do not return cached/schema defaults on getter failure.
Setters retain each module's existing validation, phase/state and equal-value
semantics. Successful facade set emits exactly one ParameterChanged with
DirectApi and available previous value; failure emits none. Equal successful set
still emits one notification, matching existing command semantics. Command
SetParameter continues to emit Command; getter absence remains previous=None for
setter notification, but missing getters cannot be accepted by a snapshot.

### Configuration extraction

Retain a small private loaded identity (version:String, master_seed:u64, bpm:f32)
on Oxurack, initialized None. Commit it only after successful load/application,
alongside the new handle. Preserve the prior identity on failure. Do not cache
module parameters or infer the seed from engine state. Snapshot without identity
or handle returns NoPatchLoaded. It does not invent defaults for a rack never loaded.
An empty successfully loaded patch is valid and can be saved.

Extract current ECS data belonging to the active PatchHandle:

1. Resolve every named module entity; collect current kind and instance identity.
   Sort output modules by instance name. Missing named Module => UnknownModule;
   mismatched handle-name/Module.instance_name => UnsupportedSnapshot below.
2. Enumerate the installed kind's complete parameter_schemas (including parameters
   omitted from original RON). Call each registered getter and store returned
   ParameterValue. Missing getter/error fails the snapshot, naming instance/param;
   never substitute defaults or loaded overrides. Empty schema needs no getter.
3. Read current DisplayName/Description/Tags on those entities (absent = empty),
   not formatter-derived presentation names. Tags are emitted in BTreeSet order.
4. Resolve Port + ChildOf identities for cable endpoints. Include all cables with
   both endpoint parents in the active set, using their current transform/labels.
   Ignore cables wholly outside that set; fail for a cable touching the active
   set whose other endpoint cannot be resolved into it. Corrupt selected endpoint
   state is an error, never a silently missing connection. Bevy entities are not
   serialized as identity. Use actual Port names and module instance names.
5. Preserve runtime propagation order when constructing cable vector: sort by
   `(target module name, target port name, source module name, source port name,
   current cable Entity)` exactly as current rebuild_propagation_order does.
   Entity is only a within-world tie-break for parallel identical endpoint pairs;
   save the resulting vector order, not entity IDs. This preserves transformed
   duplicate contributions through fresh sequential cable spawn. Do not sort ties
   by transform/value/label: LastWins could change. Provide a real tie-order test.
6. Current CableConfig cannot represent disabled cables. If a selected cable has
   enabled=false, fail with UnsupportedSnapshot; neither omit nor enable it.
   This is a bounded explicit schema limitation, not authority to migrate schema.
7. Read current RackMeta into RackConfig; normalize all-empty metadata to None,
   otherwise Some. Preserve notes order and tag-set ordering. Copy the loaded
   version/seed/global BPM unchanged. A Clock module's current bpm parameter is
   independently saved; changing that knob must not silently rewrite global BPM
   or synchronize other clocks. Public global tempo control is later work.
8. Validate the completed Patch with existing validate_patch/registry before
   returning. This catches nonfinite getter values and invalid supported topology;
   getters remain responsible for their module-specific accepted configuration.

Use existing errors where precise. Add exactly this public core error variant
for unrepresentable extraction conditions not covered by existing variants:

```rust
// In PatchError, using the existing thiserror derive:
#[error("cannot snapshot current patch: {reason}")]
UnsupportedSnapshot { reason: String },
```

Its reason must name affected module/port or disabled cable; tests match variant
and relevant context, not every punctuation byte. Missing registered module kind
uses PatchError::UnknownModuleKind. No new generalized fault/status framework.

Saving calls snapshot first, then existing save_patch_to_file. Extraction or
serialization failure must not touch the destination. I/O failures propagate via
existing PatchError::Io. Preserve the existing create/overwrite file semantics:
no crash-safe atomic-write/fsync guarantee and no mandatory new dependency.
No PatchSaved event is added (there is no such existing event). No silent error,
no partial Patch returned, no new fallback after failure. Snapshot/save leave the
rack intact even if disk writing fails; disk content after an I/O failure has the
existing std::fs::write limitations, documented accurately.

Stable musical reconstruction and semantic Patch equality are required. Byte-
canonical RON is not: parameters use the existing HashMap schema. Do not replace
all maps or change file versions just to make textual diffs deterministic.
CurrentValue, SampleTime, engine registers/phase/RNG cursor, pending MIDI, Scope
records and device handles are not patch configuration and are never serialized.

## Recommended implementation sequence and worked shapes

These examples are source-grounded guidance, **not compiled drop-in code**.
The behavior and signatures above bind; private helper/layout details are local
CC discretion. Explain equivalent local adaptations on return. A change to public
API, file semantics, timing, acceptance or scope returns to CRC/CDC before coding.

1. In core/event.rs refactor the existing private dispatch_set_parameter into a
   reusable public helper `set_module_parameter(world: &mut World,
   instance_name: &str, param: &str, value: &ParameterValue,
   source: ChangeSource) -> Result<(), CoreError>`, re-exported by core/lib.rs.
   Preserve the exact existing previous-value/error-normalization path; replace
   only the hardcoded message source. Existing dispatch passes Command; facade
   passes DirectApi. Existing tests should continue to pass, with one new direct
   provenance/failure test in the facade.
2. In facade lib.rs add private loaded identity, initialize in new(), and assign
   after apply succeeds. Add `mod persistence;` and put new inherent methods /
   extraction helper in src/persistence.rs. A child module can access the parent
   type's private fields; no public world accessor or consuming into_app workaround.
   Keep modules/queries owned or in separate borrow scopes before mutable access.
3. Implement parameter read with immutable World and existing registries. Implement
   setter by calling the shared helper. Snapshot uses short-lived World queries
   to gather module/port/cable identities, then read-only registry/getter access.
   Do not hold a mutable query construction borrow while borrowing another resource.
   Header clone first; finish all extraction before serializing/writing.

```rust
// Sketch: facade mutation routes through the one core notification owner.
oxurack_core::set_module_parameter(
    self.app.world_mut(), module, param, &value,
    oxurack_core::ChangeSource::DirectApi,
)?;

// Sketch: parameter projection for a resolved active module.
let registration = modules.get(&kind)
    .ok_or_else(|| oxurack_core::PatchError::UnknownModuleKind(kind.to_string()))?;
let mut parameters = std::collections::HashMap::new();
for schema in &registration.parameter_schemas {
    let value = getters.get_parameter(world, entity, &kind, schema.name)
        .map_err(|error| normalize_instance_error(error, instance_name))?;
    parameters.insert(schema.name.to_owned(), value);
}
// Missing getter is not a cue to copy schema.default.

// Sketch: save has no mutation or output-drain step.
pub fn save_patch(&mut self, path: &Path) -> Result<(), CoreError> {
    let patch = self.snapshot_patch()?;
    oxurack_core::save_patch_to_file(&patch, path)
}
```

4. Add `examples/save-live-patch.rs`, a small real facade-use example with existing
   clap (no Cargo/dependency changes required; examples are auto-discovered).
   Required flags `--patch PATH --save PATH`; no MIDI device selection. Install
   ClockModule, LfoModule, TuringMachineModule and MidiOutModule; load the patch
   and select Manual. The recipe intentionally expects `lfo_main` from our fixture;
   other patches fail descriptively through the parameter API.
   Advance200 samples with per-sample MIDI drain, set frequency_hz=Float(1.0),
   save while still running, advance100 more with per-sample drain, then stop.
   Print a short summary of changed value, save path and completed_samples=300.
   Return failure exit status on any error and ensure rack.stop on error as well.
   No output endpoint is opened. Saved patch is reusable with the existing runner.

```rust
// Sketch of the recipe's core. Use the same fallible stop/finalize structure
// as run_trace so an early error still stops. No held Drain across rack access.
for _ in 0..200 {
    rack.advance_samples(1)?;
    for _event in rack.drain_midi() { /* offline demo consumes events */ }
}
rack.set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(1.0))?;
rack.save_patch(&output_path)?; // completed samples remains200
for _ in 0..100 {
    rack.advance_samples(1)?;
    for _event in rack.drain_midi() { /* consume this sample */ }
}
rack.stop();
```

5. Document public methods (Errors and mutation/timing/state boundary), add README
   usage and a configuration-versus-checkpoint explanation. Existing file helpers
   and standalone modules keep their semantics. Example commands:

```bash
cargo run --locked -p oxurack --example save-live-patch -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --save /tmp/oxurack-edited.ron
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch /tmp/oxurack-edited.ron --samples 10000 \
  --scope-view --scope-port lfo_main.bipolar_out
```

### Applied Rust guidance

| Guide/strength | Applied decision and reason | Proof |
| --- | --- | --- |
| API-02 SHOULD: borrowed inputs/owned outputs | Names/Path borrowed; snapshot returns owned Patch so file placement and lifetime stay with caller | Use returned Patch after further rack advancement; source snapshot stays unchanged |
| API-23 MUST; EH-01 MUST: concrete recoverable errors | CoreError/PatchError for missing instance/getter, unsupported file state and I/O; no library String/Box error | Match variants and preserve instance context; no success message on failure |
| AP-42 AVOID chained unwraps | Resolve user names through typed errors; no indexing into unvalidated handle/ports | Missing module/port/getter negatives |
| AP-43 CONSIDER concrete functions; AP-44 AVOID leaking implementation types | Four concrete facade methods, ordinary owned data; ECS query details private; no async/storage trait platform | External integration test uses Oxurack APIs without Bevy world |

The source already uses thiserror and existing Result propagation. No new error
crate, framework defaults migration or Tokio dependency is justified.

## Tests and exact oracles

Use `tests/persistence.rs` for public composition; inline tests in persistence.rs
for deliberate ECS/registry edits and private-state observations. Preserve the
rack wrapper in inline tests; `into_app` consumes it and cannot precede calls to
snapshot/save. Reuse current fixture; build small test-only modules only for
missing-getter and cable-order gaps the four musical modules cannot expose.
No new validator crate/CLI or generalized fault injection infrastructure.

| Rows | Setup/action | Required observation / plausible wrong implementation rejected |
| --- | --- | --- |
| LP-01 | Load fixture, advance/drain200, read .5 Hz, set1 Hz while running | Getter returns1; SampleTime stays200; next LFO sample evaluates phase.1 (sin(.2*pi), tolerance1e-6), rather than phase0; one DirectApi event with previous.5/new1 |
| LP-01 | Unknown instance/param, no patch, wrong variant and NaN for LFO frequency; equal valid set | Exact existing error classes, previous parameter unchanged and no success event for failures; equal set emits once; command path still Command |
| LP-02 | Input omits selected default parameters; snapshot after live setter and an inline direct registered-setter edit bypassing facade | ALL declared parameters present at current getter values. Detect caching initial patch or facade writes instead of reading engines |
| LP-02 | Inline mutate current module/cable labels and transform plus RackMeta notes/tags, retaining valid topology; snapshot | Those exact current values, original seed42/version1.0/global BPM120, including labels/notes/order; Clock knob can change independently without global BPM rewrite |
| LP-02 | Load A then B with distinct instance names; snapshot without ticking the accumulated world | Only active B modules/internal cables, B header; no stale A entities. This tests snapshot membership, not replacement correctness |
| LP-02 | Two parallel same-endpoint cables into a test LastWins input with transforms producing .75 then .25, followed by fresh saved load | Literal input .25 before/after (opposes ascending transform-value order); preserve actual runtime tie ordering, not transform sort. Include a second unrelated endpoint pair to exercise lexical ordering |
| LP-02/03 | Module schema declares a parameter with no getter; pre-existing destination sentinel | UnknownParameter naming instance/param; no returned patch; save leaves sentinel untouched. No default fallback |
| LP-02 | Disabled selected cable; cross-boundary selected endpoint; no patch; empty loaded patch | UnsupportedSnapshot with useful context for first two; NoPatchLoaded for third; empty successful Patch for fourth |
| LP-03 | Two identical advancing racks; snapshot/save only one at completed_samples200 and again at completed_samples252 with unread event sample251 | Time, driver, running state, Scope records/overwrites and unread event251 unchanged; after caller drains, continuation through1000 equals unsaved control. No duplicate/missing event, no phase/seed reset |
| LP-03 | Save to path inside nonexistent parent; failed pre-validation load after a good load | I/O error with live configuration/time unchanged; invalid load leaves saved identity from good load. No claim of arbitrary spawner rollback |
| LP-04 | Independently construct reference Patch from fixture with LFO frequency1 and TM write_probability.25; live rack sets same values after advancing, saves; load saved file into two fresh racks and reference into a third | 10000-sample MIDI from all three equal and nonempty; edited reference differs from original fixture trace. All use per-sample drain. Fresh LFO first observation=0; do NOT compare it to saving rack's phase. Inspect serialized getter values as well, avoiding a vacuous compare of identical wrong snapshots |
| LP-05 | Run new example then existing runner on saved file; default and no-default feature modes | Completed300, saved LFO1, inspectable nonempty trace/capture. Existing Arc11 fixture trace remains unchanged |

For event tests, create/read Bevy Messages using the existing inline event test
pattern and isolate mutation events from patch-load notifications. Keep failure
checks before subsequent actions so later success cannot hide a partial mutation.
For pending-MIDI/Scope checks use inline access and the public capture iterator;
never drain solely to assert that snapshot did not drain, then reuse a consumed
value as though still pending. Use MidiOutputBuffer::check_admission() before and after snapshot/save and
match TickError::OutputBackpressure with the same pending_messages count; its
private Vec is not accessible from the facade crate. Then perform the normal
caller drain once and compare exact events and continuation with the control.
Do not add a public buffer accessor merely for this assertion.

Run from S (each outcome, including failures, recorded concisely):

```bash
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack --lib --test persistence --test scope --test lfo --test midi_output --example save-live-patch --example turing-machine-midi
cargo test --locked -p oxurack --no-default-features --lib --test persistence --test scope --test lfo --test midi_output --example save-live-patch --example turing-machine-midi
make test
make lint
cargo clippy --locked -p oxurack-core -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
git diff --check
```

Execute the two example commands above and saved/reference fresh-process replay.
Retain existing block0.1.6 advisory and three RT ignored tests accurately; do not
suppress warnings or add ignores. If restricted CoreMIDI blocks workspace tests,
record that result and rerun the same command with host access; an unavailable
host leaves that check explicitly unproved for CRC. No coverage/benchmark/soak
campaign needed for this slice; release owners unchanged. CRC chooses justified
retention for unaffected prior evidence; doer attestation is never acceptance.

## Authorized files, commits and return

Source edits permitted, explicitly:

- `crates/oxurack/src/lib.rs`
- `crates/oxurack/src/persistence.rs` (new)
- `crates/oxurack/tests/persistence.rs` (new)
- `crates/oxurack/examples/save-live-patch.rs` (new)
- `crates/oxurack-core/src/event.rs`
- `crates/oxurack-core/src/error.rs`
- `crates/oxurack-core/src/lib.rs`
- `README.md`

Existing Patch/CableConfig schema, module algorithms, dependencies/lockfile,
fixtures, playback runtime and issued prompts are unchanged. If a source-backed
problem genuinely needs another file, explain it to CRC before expanding scope;
local helper/test names and equivalent borrow mechanics are yours to choose.
Do not work around a consequential contract mistake by silently implementing a
new one. Report it with the smallest source-backed correction proposal.

After checks, commit source using explicit named paths and `git commit --only --
<the changed paths above>`; stage new files explicitly first. No `git add .` or
unrelated staged content. Then update only this slice's closing-report.md and
ledger.md with proposed-done evidence, and commit those exact planning paths
separately. Never merge planning into source; no push/merge/release is assigned.
Use actual repository commit conventions; do not invent a required trailer.

Return current assignment and source commit, changed files, actual command exits
and results, intake/readback, five-row evidence walk, self-review and any departures,
artifact inventory (normally none beyond source and canonical records), and
bubble-up. CRC independently verifies, closes when justified, and details Slice02
under the arc contract. No ritual extra iteration or new report format is needed.
