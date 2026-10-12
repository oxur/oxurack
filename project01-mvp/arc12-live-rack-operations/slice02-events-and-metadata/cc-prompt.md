# CC assignment — Arc12 Slice02: annotate a live musical rack

Initial **implementation** pass, issued2026-10-12 by CRC Sofie. Execute this
assignment and return to CRC through Duncan. Three contributors, Expedited Mode.
Approved Arc12 Slice02 follows independently accepted live persistence. Deliver
public live metadata editing, correct events, immediate inspection and reusable
annotated RON, with a real runnable example. All M-01–M-05 start open.

## Assignment, baseline and authority

Assignment relative to project01-mvp:
`arc12-live-rack-operations/slice02-events-and-metadata/cc-prompt.md`.
Current assignment must match slice-plan.md; no earlier prompt is active.

Exact roots (expand these aliases for reads/commands):

- S = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
- W = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`
- P = W/project01-mvp
- D = P/arc12-live-rack-operations/slice02-events-and-metadata
- K = `/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`

S: branch `codex/oxurack-module-behavior-installation`, clean source
`fabfa1cb4beab5fd79f05b88d4bbec34208a1cac`. Main clean at
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`, protected. Planning predecessor at
CRC intake `4c680b484e4d4d8ede949fa4cdcb736a1fab13a4`; this issued open set and
Slice01 acceptance are subsequent planning-only changes. Record actual clean W
HEAD at intake, prompt SHA-256 and exact S HEAD. A newer planning commit containing
this assignment is expected; do not require W to equal its predecessor.
Source drift/unrelated dirty edits require reconciliation before affected edits.

Binding authority: current Arc12 capability/slice table/between-sample ownership,
this open set and directive08 playable-first/proportionate-proof policy. CRC
supplies concrete mechanics inside the approved metadata/event/persistence scope.
Do not redesign public semantics, schema, scheduling or acceptance. Stop and
report a consequential contradiction to CRC; CDC/Operator resolves design changes.
No extra permission round trip for coherent implementation under this assignment.

## Required reading and execution preflight

Before dependent edits, load the manifest completely into this executing context.
Required-full means entire file, code/tables/appendices included. Required-section
means complete named definitions/functions/sections and necessary dependencies,
not grep hits or headings. Recover truncated outputs with contiguous bounded reads;
exit0/hash/another context's summary does not establish full reading. Record actual
loaded extents/revisions and gaps in **D/closing-report.md**, an intake section;
this is the eventual return record, not a separate intake artifact. Do not claim
closure when opening the record. Conditional dependencies are recorded as loaded
when triggered or not applicable with reason. Required-data: none for this slice.

| Order | Exact root-relative paths and state | Extent / purpose |
| --- | --- | --- |
| 1 | S/CLAUDE.md; W/AGENTS.md | required-full; repository/worktree roles and source boundaries |
| 2 | This prompt; D/slice-plan.md; D/ledger.md | required-full; current assignment, resolved design/readiness and five criteria |
| 3 | P/arc12-live-rack-operations/arc-plan.md; P/arc12-live-rack-operations/ledger.md | required-full; approved composition and retained lifecycle/cable obligations |
| 4 | P/project-plan.md | required-section: Current direction through Design handoff history and current2026-10-12 CRC advancement; appended historical assignments are reference-only |
| 5 | P/cdc-directive08.md | required-section: Assessment and decisions; Prompt quality for all remaining work; Checks and correction policy from Slice03 onward |
| 6 | P/cdc-directive11.md; P/arc12-live-rack-operations/slice01-live-persistence/crc-verification.md | required-full; inherited configured getter/effective cable order, acceptance and limits; no redo of predecessor assignment |
| 7 | K/collaboration-framework/SKILL.md; K/rust-guidelines/SKILL.md; K/testing/SKILL.md | required-full; current installed framework2.6 and Rust2.0 routing; apply Rust guidance, not just mention loading it |
| 8 | K/collaboration-framework/knowledge/project-management/guides/README.md | required-full wayfinder before project/slice operation |
| 9 | K/collaboration-framework/knowledge/project-management/guides/02-canonical-planning-worktree.md; guides/03-planning-top-down.md under the same directory | required-section: Per-slice documents and artifact home; Slice iteration filenames and preservation in02; Planning authority, Per-slice open set, Issuing and executing an iteration in03 |
| 10 | K/collaboration-framework/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md | required-section: Required reading and CC intake, including CC preflight; CRC applied full guide and companion template |
| 11 | K/collaboration-framework/knowledge/testing/guides/03-validation-gates.md; K/collaboration-framework/knowledge/project-management/guides/04-closing-slices.md | required-full; real checks/proportionate proof and honest row-walk/bubble-up return |
| 12 | K/rust-guidelines/guides/05-type-design.md; guides/02-api-design.md; guides/03-error-handling.md; guides/11-anti-patterns.md under rust-guidelines | required-section: TD-03,TD-06,TD-07; API-02,API-23; EH-01; AP-42,AP-43,AP-44 respectively, including rationale/examples |
| 13 | S/crates/oxurack-core/src/label.rs; src/error.rs; src/event.rs under oxurack-core | required-section: all production before cfg(test); label Tag/formatter/component tests; event dispatch tests and notification definitions; error tests for typed error conventions |
| 14 | S/crates/oxurack-core/src/lib.rs; src/patch.rs; src/port.rs; src/cable.rs; src/tick.rs | required-section: root exports/build_core_plugin message+resource registration; Patch/ModuleConfig/CableConfig/RackConfig/PatchHandle, full apply_patch_to_world and port resolution; label notification/load tests; Port/ChildOf spawning; Cable/CableOrder; effective propagation key and rebuild paths, ordinal/fallback schedule test |
| 15 | S/crates/oxurack/src/lib.rs; src/persistence.rs; src/scope.rs | required-section: facade exports/constructor/load_patch/world/world_mut/driver/start/stop/advance/drain/graph; all persistence production and loaded_rack/direct-event/pending-preservation tests; ScopeRecord/capture iterator/overwrite and public scope accessors |
| 16 | S/crates/oxurack-graph/src/lib.rs | required-section: ModuleInfo/CableInfo, build_module_info/build_cable_info/resolve_port_context and GraphView module/cable queries; labels are fresh world reads, not topology rebuild work |
| 17 | S/crates/oxurack/tests/persistence.rs; S/crates/oxurack/examples/save-live-patch.rs; S/crates/oxurack/examples/patches/turing-machine-lfo.ron; S/crates/oxurack/Cargo.toml | required-full; public/replay integration, executable install/advance/drain/stop pattern, real fixture and dependency/feature wiring |
| 18 | S/README.md | required-section: live persistence and terminal Scope/workbench recipes; extend the actual product documentation |
| 19 | P/arc03-events-and-literate-patching/ledger.md; slice01-notifications/ledger.md; slice02-literate-patching/ledger.md under that arc | required-full; six carried-over rows remain distinct obligations, not automatically accepted by this slice |
| 20 | P/arc03-events-and-literate-patching/slice01-notifications/slice-plan.md; slice02-literate-patching/slice-plan.md | required-section: Locked decisions (recap from design memo) in notifications; Locked decisions from design memo, §2 Create oxurack-core::label through RackMeta definitions (lines148–225 at planning predecessor), complete §3 Extend the LabelChanged event, complete §6 Extend apply_patch_to_world emission, and Out of scope for this bucket in literate patching. Historical role/commit instructions reference-only; current prompt governs execution |
| 21 | K/rust-guidelines/guides/14-cli-tools/02-argument-parsing.md | conditional: load full CLI-05 and CLI-11 sections before introducing parser features beyond existing two-PathBuf example; no new framework/dependency |

The skill entrypoints also require governing posture/method/work-verification
reading when applicable; their named routes remain binding. For this substantial
session explicitly load K/collaboration-framework/knowledge/collaboration-framework/
guides/01-posture-and-ethics.md, 02-structural-pulls.md and
03-collaborative-rights.md, and K/collaboration-framework/knowledge/engineering-methods/
guides/01-engineering-methodology.md (required-full), plus
K/collaboration-framework/knowledge/work-verification/guides/01-ledger-discipline.md
and 03-row-closure.md (required-full before ledger work). These supplement the
manifest rather than being optional history.

Any additional governing dependency encountered must be loaded before affected
work; list exact extent rather than chasing every historical reference. The packet
is bounded (~30–45k tokens reading/source plus execution/recovery headroom). If
required text is unavailable, report the exact gap; do not silently skim/drop it.
On recovery revalidate assignment/source and reload binding contracts no longer
reliably available. Before source edits write a concise **own-words contract
readback**, linked to source/rows: raw/fallback values, target membership/parallel
selection, provenance/clear/equal/failure policy, between-sample preservation,
feature gates and exclusions. Explain the easily missed cable occurrence and
empty-clear contracts. Complete coherent preflight authorizes execution.

## Source reconnaissance and applied guidance

At fabfa1cb:

| File/symbol | Actual reusable behavior | Delta |
| --- | --- | --- |
| core label.rs | Typed DisplayName/Description/Tags components, private validated Tag, RackMeta full-list notes, display-name fallback | Add target enum, shared read/validate/resolve/mutate helpers; reuse existing data |
| core event.rs | LabelKind/LabelValue/LabelChanged already registered; explicit dispatch_core_command and shared set_module_parameter | Add SetLabel and delegate with Command; actual facade delegates with DirectApi |
| core patch.rs/port.rs/cable.rs | PatchHandle maps active names; ports ChildOf module; cables reference port entities; CableOrder optional logical precedence | Resolve only active handle endpoints; choose one parallel occurrence without mutation |
| facade persistence.rs | Extracts current RackMeta/module/cable components into existing RON; only active membership | Reuse unchanged; no new save representation |
| graph build_module_info/build_cable_info | Fresh component reads when querying existing topology | No graph maintenance/source change needed for labels |
| facade lib.rs/save-live-patch.rs | Private world owner, real module install/manual advance/drain, stop on result | Add private labels module/public exports; runnable annotation example follows owner pattern |

CoreCommand's header promises next-tick processing, but inspection shows no
registered consumer dispatching these commands. This slice wires **explicit
synchronous dispatch**, correcting that misleading documentation locally. It
neither implements nor claims queue processing. Existing Load/Save/Add/Remove
NotImplemented arms and Panic/SetBpm behavior remain; Slice04 owns their disposition.

| Applied guide/pattern and strength | Concrete choice / proof |
| --- | --- |
| Rust TD-03 SHOULD | Keep Tag private/validated; no plain-string bypass. Typed BTreeSet eliminates duplicates and sorts persisted tags |
| TD-06/TD-07 SHOULD | LabelTarget enum distinguishes rack/module/cable data, non_exhaustive for future variants; no Entity-as-persistent-ID facade |
| API-02 SHOULD | Borrow target/payload in shared core; return owned optional values. Facade owns write payload like set_parameter; ownership of this small value is deliberate for ergonomic call sites |
| API-23/EH-01 MUST | Concrete CoreError with exact selector/type failures; no recoverable user-input panic, boxed library error or success event on failure |
| AP-42/44 AVOID; AP-43 CONSIDER | Small concrete synchronous helpers, no speculative metadata service/traits/queues; preserve domain distinctions/raw versus formatted values |
| Guide07 + proportionate proof | File/symbol recipe, uncompiled worked sketches and actual mutation/continuation oracles; no new validator, probe crate or coverage campaign |

## Binding API and behavior

All policies here are binding details of approved live metadata/events. Sketches
are **uncompiled illustrative guidance**, not tested drop-in code. Private helper
names/organization inside the listed files are local discretion; alternatives
must preserve these signatures and observable policies.

Add in core label.rs, export from core root and facade:

```rust
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelTarget {
    Rack,
    Module(String),
    Cable {
        source: (String, String),
        target: (String, String),
        occurrence: usize,
    },
}

// Shared core APIs, exported from core root:
pub fn label_value(
    world: &mut World, target: &LabelTarget, kind: &LabelKind,
) -> Result<Option<LabelValue>, CoreError>;
pub fn set_label_value(
    world: &mut World, target: &LabelTarget, kind: &LabelKind,
    value: &LabelValue, source: ChangeSource,
) -> Result<(), CoreError>;

// Facade inherent methods in new labels.rs:
pub fn label(&mut self, target: &LabelTarget, kind: LabelKind)
    -> Result<Option<LabelValue>, CoreError>;
pub fn set_label(&mut self, target: &LabelTarget, kind: LabelKind, value: LabelValue)
    -> Result<(), CoreError>;
```

Re-export LabelTarget, LabelKind, LabelValue, Tag, TagError at facade root; use
existing core ChangeSource. Do not expose a facade world accessor. Mutable borrow
for getter permits World query construction; it is observational, with no tick,
message or mutation of musical state. Returned values are owned **raw metadata**,
not fallback display names or registration descriptions.

Add CoreCommand::SetLabel `{ target: LabelTarget, kind: LabelKind, value: LabelValue }`.
Explicit dispatch delegates to shared helper with Command; facade with DirectApi.
No Changed<T> reactor, duplicate emission or new scheduled processor.

### Target and payload resolution

| Kind | Accepted payload | Targets / readback |
| --- | --- | --- |
| DisplayName | Text(String) | Rack/Module/Cable |
| Description | Text(String) | Rack/Module/Cable |
| Tags | Tags(BTreeSet<Tag>) | Rack/Module/Cable |
| Notes | Notes(Vec<String>) | Rack only |

Rack metadata is its existing resource. A loaded empty patch still permits Rack;
no loaded PatchHandle fails NoPatchLoaded for every target. Module/Cable membership
is restricted to **active PatchHandle**, even if old entities survive a prior
append-only load. Module entity must still have Module with the selected instance
name; stale/missing returns UnknownModule. Do not rename persistent instances when
changing DisplayName. Find port by Port.name and ChildOf(active_module), not a
world-global same-name port or graph display name. Missing endpoint module gives
UnknownModule, missing endpoint port gives CoreError::Patch(PatchError::UnknownPort).
Resolve source before target for deterministic failure precedence.

Cable occurrence is **zero-based among cables with exactly those resolved source
and target port entities**, ordered by `(order_absent, ordinal_or_zero, Entity)`.
All endpoint-prefix fields are identical within this candidate set; this is the
Slice01 effective order restricted to that pair. Present before absent, smaller
ordinal first, equal/absent Entity ascending. Do not choose by transform, graph
enumeration, full-world cable rank or first query item. An existing disabled cable
can be annotated (save still follows Slice01 unsupported-disabled policy). No
matching occurrence returns UnknownCable. Selector is view-relative; reselect
after cable membership/order changes, not a durable ID across arbitrary edits.

Add these concrete CoreError variants with documented fields:

```rust
InvalidLabelValue { kind: LabelKind, reason: String },
UnknownCable {
    source_endpoint: (String, String),
    target_endpoint: (String, String),
    occurrence: usize,
},
```

Use source_endpoint (not a field named source: thiserror treats that as an error
source). Display lowercase/contextual; tests match variants/fields rather than
substring errors. Failure precedence: NoPatchLoaded first; then target/kind support
and payload validation; then active target/port/cable resolution. Unsupported Notes
on module/cable and wrong payload return InvalidLabelValue; reads also reject
unsupported Notes. Validated Tag cannot be forged; caller Tag::new failure occurs
before a facade write, propagates TagError and changes nothing. No new string
restriction/normalization: whitespace/empty text allowed; Tag preserves its current
nonempty/no-leading-or-trailing-whitespace rule including allowed internal spaces.

### Raw values, clearing and notification

- Entity absent component reads None. Nonempty text/set inserts or replaces the
  existing component; previous is actual raw Some if present, None otherwise.
  Empty Text/empty Tags **remove** the component; raw getter then None, graph display
  falls back for DisplayName. Existing manually inserted empty component, if any,
  reads Some(empty) before clearing. Empty raw RON fields remain absent on reload.
- Rack getters always Some of the actual RackMeta field (including empty defaults).
  Clear assigns empty field; Notes replace entire vector, preserve order, duplicates
  and empty strings. Notes do not become entity components or per-note events.
- Every successful write, **including equal/clear-already-empty**, emits exactly one
  LabelChanged. Previous is actual raw value just before mutation; new is the
  accepted explicit payload even when clear removes the component. Read emits none.
- Event entity None + instance_name empty for Rack; actual active module entity +
  persistent name for Module; actual chosen cable entity + existing loader format
  `source.port -> target.port` for Cable. Kind/new/previous/source exact. Do not
  encode DisplayName as instance identity or emit ParameterChanged/topology events.
- Complete all fallible validation/resolution before mutation; collect owned previous
  and event identity, release resource/query/entity borrows, then mutate one field,
  then emit once. Failed edit/read leaves metadata, order, time, engines, transport,
  Scope and queued MIDI unchanged and emits no successful event. No external
  callback/transaction/rollback service is needed; no guarantee of arbitrary
  programmer-panic recovery is added.
- Between-sample call only; no schedule/tick/stop/start/advance/drain/reset/reload.
  Label edit doesn't dirty/rebuild topology or change CableOrder/transform. Existing
  graph queries read labels fresh; snapshot/save serialize existing components.
  Feature absence must not remove public raw read/edit APIs.

## Recommended file-level implementation sequence

1. **core label.rs/error.rs/lib.rs:** define/export LabelTarget/errors and shared
   helpers. Private resolved target can be enum Rack/Entity{entity,instance_name};
   do not borrow world data across mutation or message write. Reuse Tag/RackMeta
   and DisplayName/Description/Tags; no duplicate representation.
2. **label.rs resolution:** read/copy PatchHandle membership first, verify active
   modules, find endpoint entities with `(Entity,&Port,&ChildOf)`, query candidates
   `(Entity,&Cable,Option<&CableOrder>)`, sort and nth. Linear O(world ports+cables)
   is acceptable at user edit frequency, outside the 1ms path; no cache/index
   service. Missing target errors are precise; no success event before resolution.
3. **label.rs read/mutate:** validate target/kind/payload, snapshot raw previous and
   owned identity, mutate exact component/resource field, end borrow and write
   LabelChanged via existing World message API. No default formatting in raw read.

```text
set_label_value(world,target,kind,value,source):
    require loaded PatchHandle
    validate support + payload
    resolved = resolve active target (including exact ordered cable occurrence)
    previous = clone raw field (entity optional / rack explicit)
    identity = owned notification entity/name
    assign rack field OR insert/remove selected entity component
    end entity/resource/query borrow
    world.write_message(LabelChanged { identity, kind.clone(), previous,
                                       new: value.clone(), source })
    return Ok(())
```

4. **event.rs:** SetLabel delegates to helper with Command. Correct CoreCommand
   header to distinguish explicit dispatch from future queue routing; retain all
   other dispatch behavior. **facade lib.rs/labels.rs:** mod/reexports and inherent
   forwarding, source DirectApi. Do not copy mutate+emit into a second path.
5. **tests:** core inline tests can observe Messages and raw entities; facade
   labels.rs inline tests can use the existing private world methods while keeping
   Oxurack alive. Public tests/labels.rs exercises actual modules/public methods,
   graph default feature only, snapshot/save/fresh load in both modes. Do not use
   consuming into_app then call facade again or add public test-only world access.
6. **new annotate-live-patch.rs and README:** follow real save-live-patch pattern:
   --patch/--save PathBuf, install Clock/LFO/TM/MidiOut, Manual/start, advance+drain
   200, annotate, save, advance+drain100, stop on success/error. Metadata-only,
   no parameter edit/no endpoint. Print saved path, completed_samples300 and owned
   raw readbacks. Document name/tag clear, notes full-list read/append/set, error
   and cable-occurrence semantics, feature availability and concrete run commands.

Representative annotation payloads (binding example behavior; sketch uncompiled):

```rust
let rack_target = LabelTarget::Rack;
let tm = LabelTarget::Module("tm_main".into());
let clock_cable = LabelTarget::Cable {
    source: ("clock_main".into(), "pulse".into()),
    target: ("tm_main".into(), "clock".into()),
    occurrence: 0,
};
let tags = [Tag::new("workbench")?, Tag::new("generative")?]
    .into_iter().collect();
rack.set_label(&rack_target, LabelKind::DisplayName,
    LabelValue::Text("Turing workbench".into()))?;
rack.set_label(&rack_target, LabelKind::Notes,
    LabelValue::Notes(vec!["Seeded LFO modulation".into(),
                          "Configuration replay; engines restart".into()]))?;
rack.set_label(&rack_target, LabelKind::Tags, LabelValue::Tags(tags))?;
rack.set_label(&tm, LabelKind::DisplayName, LabelValue::Text("Melody generator".into()))?;
rack.set_label(&clock_cable, LabelKind::Description,
    LabelValue::Text("Clock pulse to Turing Machine".into()))?;
rack.save_patch(&args.save)?;
```

Read full-list Notes, append a string and set Notes to demonstrate callers can
extend notes without a new note-ID API. Example fixed payloads above remain exact;
README append recipe may be separate. No new Cargo dependency or fixture needed.

## Behavior tests and exact oracles

Use real current fixture unless a core inline malformed/parallel world is needed.
Install real modules; clear patch-load messages only for runtime-event observations.
Do not calculate expected metadata by asking the same new helper twice. Derive
literal values, errors, event identities/counts from this contract.

| Row / setup and action | Exact observation / wrong behavior rejected |
| --- | --- |
| M-01; raw public reads/writes for all supported target-kind pairs | Loaded unlabeled module/cable read None; rack empty defaults read Some(empty). Write literal name/description/tag/notes; correct raw value. Entity clear reads None; graph module fallback `Tm Main` (existing formatter), not empty/stale label. Rack clear Some(empty). Notes on entity fails exact InvalidLabelValue |
| M-01/02; tags and notes | Tags from [zeta,alpha,zeta] serialize/read in [alpha,zeta]; Tag::new empty/leading/trailing whitespace fails before edit; internal-space tag accepted. Notes ["second","first","second",""] remain exact; clear [] retained as empty rack field. Wrong Text/Tags/Notes combinations reject before any mutation/event |
| M-01; inline real core active ports with parallel cables | Oppose Entity order: Entity-first ordinal1, Entity-second ordinal0. occurrence0 selects ordinal0 only; occurrence1 selects ordinal1; unrelated lower-ordinal cable excluded. Equal/missing order follows Entity, mixed present first. Exact raw chosen labels/event entity; transforms/CableOrder untouched. Missing occurrence→UnknownCable with exact endpoints/index; source then target port errors typed |
| M-01; no patch/unknown/stale membership | All targets without PatchHandle→NoPatchLoaded. Missing/stale module→UnknownModule. Old entities outside active PatchHandle never receive label changes even with same names; valid active labels/snapshot correct. Loaded empty patch permits Rack. No in-place replacement acceptance claim |
| M-02; direct and explicit command mutations | Actual LabelChanged previous None→Text("Lead"), then Some("Lead")→Text("Lead") equal write, then Some("Lead")→Text("") clear; three exact successful events, then raw None. Second clear previous None/new empty still one. Rack previous Some(empty), cable formatted persistent identity exact. Facade DirectApi, command Command; no ParameterChanged/cable events |
| M-02; mixed consumer at call boundaries | Direct label then explicit command parameter then explicit command label; drain/inspect each relevant message stream after each completed operation and append owned observations to one consumer list; exact order/provenance and fields at those boundaries. No tick required. Do not compare MessageId numbers from different types to invent a global ordering clock; no queue execution claim |
| M-03; default graph + persistence | Set rack all four fields, module/cable all three; graph query immediately reflects labels/description/tags without tick. Snapshot/save include raw labels (not fallback), deterministic tags, exact notes, unchanged headers/parameters/endpoints/transforms/order; new fresh installed rack loads and raw reads equal. Both feature modes run public read/save/reload; graph tests cfg(feature="graph") |
| M-04; actual running fixture subject/control | Both advance/drain200 with Scope configured identically. On subject read/edit/equal/clear/reject/save; sample/driver/running/Scope full record vectors+overwrite unchanged from before calls. Compare musical continuation against untouched control; advance both without draining event251 until pending252, perform more metadata edits/rejections/read/save; admission still pending1, drain actual event251 once on each, second drain empty; subsequent per-sample events/time through1000 identical. Metadata differs intentionally; engine state/output must not |
| M-05; executable recipe below and old native load tests | Metadata-only saved trace identical to original70 rows/hash, repeated fresh process equal; original MIDI13/hash unchanged. Existing patch label/load/default/schema checks pass. Reconcile each legacy row and identify retained omissions rather than claiming complete original arc acceptance |

For M-04 use inline labels.rs ownership (private rack driver/world, MidiOutputBuffer
admission) and public ScopeRecord iterator; clone/collect complete records before
each call group. This is feasible without exposing engine internals. Stop/resume
not a new API here; existing sample/persistence tests retained. Failed call checks
compare all relevant metadata/snapshot/event count before-after too, with meaningful
preexisting nondefault labels and pending MIDI. Legitimate save filesystem failure
remains Slice01 behavior; don't rewrite that feature to make tests pass.

Patch-load cross-type emission order remains the existing literal sequence:
per-module ModuleAdded→its labels→its parameters; per-cable CableConnected→its
labels; rack labels DisplayName/Description/Notes/Tags; PatchLoaded summary last.
Inspect unchanged load emission sites and rerun native tests. New runtime mixed
consumer proves chronological operation-boundary observations; per-type counts
alone do not prove a global patch-load timeline. Preserve that stronger legacy
load-order proof limitation explicitly for Arc12 final assembly; no new cross-type
message bus/instrumentation product here. Removal/unload sites stay Slice04.

## Native checks and real executable recipe

Run from S on final source, record actual commands/native exits/counts, including
failed/retried/unrun attempts. Focused checks must pass, not only wrapper scripts.

```sh
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack --lib --test labels --test persistence --test scope --test lfo --test midi_output --example save-live-patch --example annotate-live-patch --example turing-machine-midi
cargo test --locked -p oxurack --no-default-features --lib --test labels --test persistence --test scope --test lfo --test midi_output --example save-live-patch --example annotate-live-patch --example turing-machine-midi
make test
make lint
cargo clippy --locked -p oxurack-core -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
cargo build --locked -p oxurack --example annotate-live-patch --example turing-machine-midi
git diff --check
```

Baseline restricted make test has four CoreMIDI initialization failures, exit2
(Cargo101); exact host-access retry passed at fabfa1cb, RT150passed/3existingignores.
Preserve failure and unchanged successful retry as separate results if environment
recurs; do not skip/ignore/edit MIDI tests. Existing block0.1.6 future-incompatibility
advisory stays visible. New unexpected failures remain failures until resolved.
No endpoint gate, dependency audit/coverage expansion or predecessor checker is
invented. Prior Slice01 acceptance covers unchanged standalone TM/CableOrder
contracts at fabfa1cb; changed shared paths still get native workspace/regressions.

Execute the actual example, not a test wrapper that manufactures saved output:

```sh
OXURACK_LABEL_DIR=$(mktemp -d /tmp/oxurack-labels.XXXXXX)
cargo run --locked -p oxurack --example annotate-live-patch -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --save "$OXURACK_LABEL_DIR/annotated.ron"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_LABEL_DIR/annotated.ron" --samples 10000 \
  > "$OXURACK_LABEL_DIR/annotated-1.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_LABEL_DIR/annotated.ron" --samples 10000 \
  > "$OXURACK_LABEL_DIR/annotated-2.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --samples 10000 \
  > "$OXURACK_LABEL_DIR/original.trace"
cmp "$OXURACK_LABEL_DIR/annotated-1.trace" "$OXURACK_LABEL_DIR/annotated-2.trace"
cmp "$OXURACK_LABEL_DIR/annotated-1.trace" "$OXURACK_LABEL_DIR/original.trace"
wc -l "$OXURACK_LABEL_DIR/annotated-1.trace"
shasum -a 256 "$OXURACK_LABEL_DIR/annotated-1.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_LABEL_DIR/annotated.ron" --samples 10000 \
  --scope-view --scope-port lfo_main.bipolar_out \
  > "$OXURACK_LABEL_DIR/view.trace" 2> "$OXURACK_LABEL_DIR/view.txt"
cmp "$OXURACK_LABEL_DIR/annotated-1.trace" "$OXURACK_LABEL_DIR/view.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron --samples 2000 \
  > "$OXURACK_LABEL_DIR/baseline-midi.trace"
shasum -a 256 "$OXURACK_LABEL_DIR/baseline-midi.trace"
```

All exits0; actual saved readback contains fixed annotations; completed_samples300.
Annotated/original LFO traces70 rows SHA
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Original MIDI13 rows SHA
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
Scope view remains usable, no sent events/endpoint opened, stdout equal. Read the
saved RON and inspect actual label/default/header contents; fresh public tests
establish raw field semantics. Example missing/malformed patch and unwritable save
must exit nonzero with diagnosis; no endpoint cleanup needed. Temp files aren't
new durable deliverables. Shell recipe sketches have not been run before the new
example exists; hashes are independently reproduced baseline oracles.

## Scope, commits, stop conditions and return

Complete source allowlist relative to S; commit only changed paths:

```text
crates/oxurack-core/src/label.rs
crates/oxurack-core/src/event.rs
crates/oxurack-core/src/error.rs
crates/oxurack-core/src/lib.rs
crates/oxurack/src/lib.rs
crates/oxurack/src/labels.rs
crates/oxurack/tests/labels.rs
crates/oxurack/examples/annotate-live-patch.rs
README.md
```

Nine paths cover implementation, inline/public tests, example and docs. No Cargo
files/dependencies, existing RON fixtures, engine/adapter, persistence, graph,
propagation/order or runtime/lifecycle source changes. If inspection exposes a
real necessary change outside this fence or a binding mismatch, stop dependent
work, report exact source/criterion and proposed resolution to CRC; don't silently
widen or weaken. Local private helper mechanics within these paths are permitted.
Do not modify main, other worktrees or historical assignments/evidence.

After actual self-review/checks, commit explicit changed source paths using
`git commit --only -- <exact changed paths>`; for new files explicitly stage only
those paths first. Separate planning return is limited to **D/closing-report.md
and D/ledger.md**, also explicit-path scoped commit. No blanket git add/stash/
reset/cleanup, merge/push/release or direct CDC/CC messaging. Do not close ledger
rows or advance Slice03; statuses stay open with CC proposed-done evidence.

Return assignment path/hash, S/W commits/clean state, changed paths, complete
intake/readback extent record, per-M-row behavioral evidence and command results
including failed/retried/unrun attempts, deviations and remaining blockers. Self-
review actual signatures/borrow/failure/event/cable/persistence interactions, not
just green gates. Include **Bubble-up to the arc**: whether approved design changed,
remaining lifecycle/order-proof/carryover owners and six separate Arc03 row
accountings (A-03-01/02; each historical child's S-01/02). Old criteria remain;
mark delivered sub-obligations versus deferred omission with precise source/check
and receiving owner, no false legacy acceptance. Artifacts none expected; explain
any needed supporting artifact before creating it in D/artifacts/.

CC attestation remains proposed done until CRC independently reviews. Preserve
original/pinned failure history; future corrections get new root iteration prompts,
not edits to this issued assignment. No preset iteration count or fresh hardware
witness is required for a metadata-only increment.
