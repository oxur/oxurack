# CC assignment — Arc12 Slice03: rewire the live musical rack

Initial implementation, issued2026-10-12 by CRC Sofie. Three contributors,
Expedited Mode; Duncan relays, CC implements, CRC independently reviews. Approved
Arc12 now delivers actual cable edits, fresh graph inspection and saved replay.
All five G rows start open. This is product implementation, not another proof tool.

## Assignment and baseline

Assignment: `arc12-live-rack-operations/slice03-graph-composition/cc-prompt.md`,
relative to project01-mvp. Exact roots:

- S = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
- W = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`
- P = W/project01-mvp
- D = P/arc12-live-rack-operations/slice03-graph-composition
- K = `/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`

S clean at `20c60efa63c4fbded5c0036a3bcd10ff2c4ba4d9`, branch
`codex/oxurack-module-behavior-installation`. Protected main clean at
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Planning predecessor is
`d8761d0ee6383fe5a830585ef457f584d8ca4383`; this open set and Slice02 CRC acceptance
are subsequent planning-only changes. Record actual clean W HEAD, S HEAD and
prompt SHA-256 at intake, not an impossible self-referential planning commit.

Binding authority: approved Arc12 slice table/ownership, directive08, directive11
logical cable ordering, this current open set. CRC resolves the cable/graph
mechanics within that design. Historical Arc04/05 text supplies retained criteria,
not old execution roles or assumed APIs. No load/reset/transport/handle redesign,
async service, schema/ID system or endpoint audition. Future PortRef/typed handle
sugar and coherent facade command owner remain Slice04. Stop affected work on a
material new contract/scope conflict; report to CRC for CDC/Operator resolution.
Coherent implementation needs no additional permission round trip.

## Required reading and intake

Load each declared extent completely before dependent edits, using contiguous
bounded reads when output is truncated. Hashes/headings/file existence/summaries
are not full reading. Record each item, revision, loaded extent and recovered gaps
in an intake section of **D/closing-report.md**. Record conditional triggers as
loaded or not applicable with reason. No new intake artifact. Required-data: none.

Root-relative paths expand the exact roots above; production means everything
before cfg(test), and named functions include full bodies/necessary definitions.

| Order | Paths | Required extent and purpose |
| --- | --- | --- |
| 1 | S/CLAUDE.md; W/AGENTS.md; this prompt; D/slice-plan.md; D/ledger.md | required-full; roles, assignment, scope and criteria |
| 2 | P/arc12-live-rack-operations/arc-plan.md; ledger.md in that arc; slice02-events-and-metadata/crc-verification.md in that arc | required-full; approved architecture, accepted predecessor and limits |
| 3 | P/project-plan.md | required-section: Current direction through Design handoff history and current CRC advancement; historical appended assignments reference-only |
| 4 | P/cdc-directive08.md | required-section: Assessment and decisions, Prompt quality for all remaining work, Checks and correction policy from Slice03 onward |
| 5 | P/cdc-directive11.md | required-section: Decision2 including Discriminating ordering proof; append-after-ordered and fallback policies |
| 6 | K/collaboration-framework/SKILL.md; K/rust-guidelines/SKILL.md; K/testing/SKILL.md | required-full; installed framework2.6.0/Rust2.0.0/testing routing |
| 7 | K/collaboration-framework/knowledge/project-management/guides/README.md | required-full; PM wayfinder before slice/planning operations |
| 8 | Same PM guides/02-canonical-planning-worktree.md; 03-planning-top-down.md | required-section: Per-slice documents and artifact home, Slice iteration filenames and preservation in02; Planning authority, Per-slice open set, Issuing and executing an iteration in03 |
| 9 | K/collaboration-framework/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md | required-section: Required reading and CC intake, including CC preflight |
| 10 | K/collaboration-framework/knowledge/testing/guides/03-validation-gates.md; knowledge/project-management/guides/04-closing-slices.md under framework | required-full; real checks, proportionate proof and return |
| 11 | K/collaboration-framework/knowledge/collaboration-framework/guides/01-posture-and-ethics.md; 02-structural-pulls.md; 03-collaborative-rights.md; K/collaboration-framework/knowledge/engineering-methods/guides/01-engineering-methodology.md | required-full; governing posture and contributor method |
| 12 | K/collaboration-framework/knowledge/work-verification/guides/01-ledger-discipline.md; 03-row-closure.md | required-full before ledger work; no self-acceptance |
| 13 | K/rust-guidelines/guides/11-anti-patterns.md; 02-api-design.md; 03-error-handling.md; 05-type-design.md | required-section: AP-42/43/44 first; API-02/23; EH-01; TD-03 respectively, full rationale/examples. Apply concrete guidance below |
| 14 | S/crates/oxurack-core/src/label.rs; cable.rs; error.rs; event.rs | required-section: all production; label active/parallel selector tests; cable transform/index tests; event command dispatch tests and topology notifications |
| 15 | S/crates/oxurack-core/src/patch.rs; port.rs; tick.rs; lib.rs | required-section: Patch/ModuleConfig/CableConfig/PatchHandle, validate_patch/check_patch_cycles/full cycle helpers, full apply_patch_to_world and topology/load tests; all port production; PropagationOrder/Dirty/rebuild/propagate/merge functions and ordinal schedule test; root exports/plugin registrations |
| 16 | S/crates/oxurack/src/lib.rs; labels.rs; persistence.rs; scope.rs | required-section: exports/constructor/load/graph/driver/start/stop/advance/drain/world access; all labels production and full pending/control tests; all persistence production and duplicate-order/fallback tests; ScopeRecord/capture iteration/overwrite/selector resolution |
| 17 | S/crates/oxurack-graph/src/lib.rs | required-section: all production plus native maintenance, navigation/diagnostic and getter tests; complete source-grounded 18-query reconciliation, not just new edge counts |
| 18 | S/crates/oxurack/examples/annotate-live-patch.rs; S/crates/oxurack/tests/labels.rs; S/crates/oxurack/tests/persistence.rs; S/crates/oxurack/examples/patches/turing-machine-lfo.ron; S/crates/oxurack/Cargo.toml | required-full; actual runnable/public/feature patterns |
| 19 | S/README.md; S/Makefile | required-section: live persistence/metadata/workbench recipes in README; test/lint/docs targets and dependencies in Makefile |
| 20 | P/arc04-rackgraph/arc-plan.md; ledger.md; slice01-foundation/ledger.md; slice02-navigation/ledger.md; slice03-diagnostics/ledger.md under that arc | required-full; eight separate inherited arc/child rows |
| 21 | P/arc04-rackgraph/artifacts/0011-rackgraph-design-memo-v0.md; slice01-foundation/slice-plan.md; slice02-navigation/slice-plan.md; slice03-diagnostics/slice-plan.md under that arc | required-section: memo Post-3.25/3.5 reconciliation and Scope of v0 navigation catalog; foundation Locked decisions; navigation Locked decisions (recap); diagnostics Locked decisions (recap). Historical instructions/old paths are reference-only; current code/this prompt govern mechanics |
| 22 | P/arc05-public-api/arc-plan.md; ledger.md; artifacts/0010-oxurack-public-api-design-v0.md under that arc | required-section: current Remaining/re-entry and both A rows; memo Confirmed Naming Decisions and Full Public Surface cable-ops fragment. Preserve patch/unpatch naming; broad PortRef/handles/lifecycle owned by Slice04 |
| 23 | K/rust-guidelines/guides/14-cli-tools/02-argument-parsing.md | conditional: full CLI-05/11 before adding parser features beyond the existing two-PathBuf example; no new CLI framework/dependency |

Normative dependencies encountered later must be loaded before affected work;
reference-only historical links are not an unbounded reading chain. Before edits,
write an own-words contract readback linking G rows/source to actual changes,
atomic failures, selector/order/provenance, graph freshness versus ticking, held
inputs and replay limits. Explain why a removed input need not become zero and
why an arbitrary Some(transform) is insufficient type validation. Missing required
material or conflicting obligations block dependent work, not routine implementation.
After compaction revalidate source/assignment and reload contracts no longer
reliably available. Budget approximately50–70k tokens of reading/source plus
implementation, native checks/self-review and recovery headroom. Do not skim to fit.

## Reconnaissance and guideline application

Inspected at20c60ef: AddCable/RemoveCable dispatch returns NotImplemented; core
CableIndex already adds/removes both directions; loaded cables have CableOrder;
mark_propagation_order_dirty exists. Label selection resolves PatchHandle names
and actual ChildOf ports, sorts present ordinal then absent/Entity. Snapshot/save
already captures current Cable/labels/order. GraphMaintenance reads topology
messages synchronously on load; facade has private app/world. No new scheduler.

Two existing subtleties govern this design. Zero cable contributions retain an
input's previous CurrentValue in merge_inputs; removing a cable doesn't neutralize
its held signal. Legacy validate_patch treats any Some(transform) as enough for
kind mismatch and doesn't validate direction/numeric transform parameters. The
new runtime boundary must validate actual transform support, not reuse that shortcut.
Do not silently change legacy file-load compatibility in this slice; record the
loader limitation for final Arc12/Slice04 disposition. Runtime edits admitted by
this stricter boundary remain representable and reloadable in the existing schema.

| Guide / strength | Applied decision and reason | Implementation / proof |
| --- | --- | --- |
| Rust API-02 SHOULD | Borrow separate module/port names, return owned occurrence, not world/entity/guard | facade signatures below, public no-graph integration |
| Rust API-23 and EH-01 MUST | Expected missing endpoints, illegal connections and order exhaustion return typed CoreError/PatchError before mutation | exact variants, negative before/after tests |
| Rust TD-03 SHOULD | Reuse CableOrder for ordinal meaning and LabelTarget's existing usize occurrence for view-relative selection; no invented global persistent ID | opposed ordinal/Entity and snapshot/fresh reload |
| Rust AP-42 AVOID | No unwrap chain for external endpoint/transform input; preflight Result then infallible validated commit | failed edits leave every observed owner unchanged |
| Rust AP-43 CONSIDER | Small synchronous shared core helpers, no generic command bus/async abstraction for two operations | real direct and explicit command routes |
| Rust AP-44 AVOID | Curated facade doesn't expose Bevy Entity or mutable World for rewiring | inherent patch/unpatch methods in both feature modes |

No new trait/async/unsafe work, dependency/edition/MSRV change or hot-path allocation
is prescribed. Off-sample preflight may allocate O(modules+ports+cables), sort cable
matches O(c log c) and cycle-check O(m+c); existing sample propagation stays intact.
This bounded rack operation needs no new benchmark platform. Private helper names,
test factoring and borrow scopes are local discretion. Binding observable behavior
below must agree across the facade and existing explicit command dispatcher.

## Binding API and operational contracts

Add these inherent methods, available with and without graph:

```rust
pub fn patch(
    &mut self,
    source: (&str, &str),
    target: (&str, &str),
    transform: Option<CableTransform>,
) -> Result<usize, CoreError>;

pub fn unpatch(
    &mut self,
    source: (&str, &str),
    target: (&str, &str),
    occurrence: usize,
) -> Result<(), CoreError>;
```

These signatures resolve this slice's facade, not the future PortRef/handle sketch.
Names are case-sensitive; do not split dots or trim names. patch returns the new
cable's current zero-based occurrence among that exact endpoint pair, usable in
LabelTarget::Cable. It is view-relative, not durable identity: removals can shift
later occurrences. Every successful patch adds one enabled, initially unlabeled
cable, including a duplicate when the target merge policy permits fan-in. It is
not an idempotent ensure-connection. unpatch removes exactly one selected cable;
missing occurrence returns UnknownCable, not success or remove-all. Removal can
repair a disabled or legacy malformed-transform cable; it doesn't revalidate
that cable's direction/transform. Module/port endpoints must still resolve.

Core helpers share semantics and accept ChangeSource:

```rust
pub fn connect_cable(
    world: &mut World, source: (&str, &str), target: (&str, &str),
    transform: Option<CableTransform>, source_of_change: ChangeSource,
) -> Result<usize, CoreError>;
pub fn disconnect_cable(
    world: &mut World, source: (&str, &str), target: (&str, &str),
    occurrence: usize, source_of_change: ChangeSource,
) -> Result<(), CoreError>;
```

Facade passes DirectApi; dispatch_core_command AddCable passes Command, discards
returned occurrence and preserves Result<()>; existing RemoveCable shape stays
unchanged and removes occurrence0 (document that legacy convenience). No queue
consumer, facade dispatch service or implicit tick. Raw core owners explicitly
run GraphMaintenance when they installed the optional graph plugin; the graph
crate must not become a core dependency. Successful facade operations run that
schedule before returning under cfg(feature="graph"); failure does not run it.

Both calls work stopped or running, Manual or Live, between samples. They do not
reset engines, advance SampleTime, change driver/transport, write parameters,
clear/reconfigure Scope, change existing cable metadata/order, drain/drop pending
MIDI or flush target CurrentValue/MergeBuffers. Topology takes effect in propagation
on the next actual sample through the existing dirty flag. With no remaining
contributors, input sample-and-hold persists; adding a replacement cable before
that sample supplies the new signal. No disable/enable/transform-in-place API:
remove/add is the explicit replacement, two individually committed operations;
if add fails after successful remove, the remove stays successful. Do not promise
transactional multi-call rewiring. No enabled field/schema migration.

### Validation, order and failure boundary

Connect preflight is entirely read-only with respect to observable rack state:

1. Require PatchHandle; otherwise CoreError::NoPatchLoaded. Resolve source module
   and actual ChildOf port, then target module and port using existing active
   membership/stale checks. UnknownModule and Patch(UnknownPort) retain exact fields.
   Raw superseded patch entities are excluded from the active topology projection.
2. Require source Output and target Input. Add typed
   `CoreError::InvalidCableDirection { module: String, port: String,
   expected: PortDirection, actual: PortDirection }`; source direction checked first.
3. Validate transform parameters/support and resulting target kind, in this order:
   supplied numeric parameters finite; Clamp min<=max; support table below;
   resulting kind equals target. No transform requires source kind==target and
   otherwise returns existing Patch(KindMismatch). Invalid/unsupported transforms
   return `CoreError::InvalidCableTransform { source_kind: ValueKind,
   target_kind: ValueKind, transform: CableTransform, reason: String }`.
   Reason diagnoses the actual issue; its wording needn't be a brittle test oracle.
4. Reject a second active structural cable into MergePolicy::Reject, or illegal
   target merge kind/policy, using existing Patch(IllegalMerge) and actual target
   module/port/kind/policy. Count all selected cables including disabled cables,
   matching structural load validation. Check policy validity even on a first cable.
5. Project active modules and active endpoint connections plus candidate into a
   topology-only Patch, and call the existing cycle checker (make it pub(crate)).
   Reuse its FeedbackCycle and deterministic member list; don't clone full engine
   configuration or call facade snapshot/getters. Preserve current policy: same-
   module self-connections allowed; multi-module cycles rejected. Graph diagnostics
   still report a self-loop and topological_order returns CycleError for it.
6. Find max CableOrder among active structural cables; next checked_add(1), empty
   ordered set starts0. Max usize::MAX returns `CoreError::CableOrderExhausted`
   before spawn/index/event/dirty change. Do not wrap, saturate or renumber anything.
   New ordered cables follow existing ordered cables; within exact endpoint ties
   absent-order cables still follow ordered ones. Never claim the new cable is
   after unordered cables. Compute returned occurrence as count of present-order
   cables for this endpoint pair; the new unique maximum comes after those and
   before absent-order matches. Equal old ordinals retain Entity fallback.

All recoverable failures above leave modules, ports, Cable entities/components,
CableIndex, propagation dirty/cache, metadata, graph, SampleTime, driver/running,
Scope and pending MIDI unchanged and emit no success. Query-cache/internal scratch
allocation isn't an observable mutation. No rollback promised for OOM or a Bevy
programmer-invariant panic; no external/user hook runs during the commit phase.
Unpatch preflight uses steps1 and exact occurrence resolution only, with the same
present/absent/order/Entity key as LabelTarget. No patch or missing endpoint wins
before missing occurrence. Missing cable uses existing UnknownCable exact fields.

| Transform | Accepted source → result | Numeric policy |
| --- | --- | --- |
| None | same kind | — |
| Affine | Float→Float; Bipolar→Bipolar | factor and offset finite; negative/zero factor allowed |
| Invert | Float→Float; Bipolar→Bipolar | — |
| Clamp | Float→Float; Bipolar→Bipolar | finite min/max, min<=max, equal bounds allowed |
| Threshold | Float→Gate | finite threshold |
| GateToFloat | Gate→Float | — |
| Unipolar | Float→Bipolar | preserve current apply semantics despite the name |
| Bipolarize | Bipolar→Float | preserve current apply semantics despite the name |

A supplied transform incompatible even with otherwise equal port kinds is rejected.
Do not use apply(Value::default) as the whole validator: it misses numeric hazards
and an invalid Clamp may panic. Finite coefficients don't promise every arbitrary
runtime float stays finite; do not silently clamp all values or change CableTransform.
Legacy file validation remains unchanged except cycle helper visibility/event
provenance production. Track that compatibility limit honestly at return.

### Commit sequence, exactly-once messages and graph

Representative pseudocode, **uncompiled guidance**, binding order/atomic preflight:

```text
connect(world, endpoints, transform, provenance):
    prepared = resolve_and_validate_active_connection(world, endpoints, transform)?
    // owns endpoint IDs, cable value, ordinal and return occurrence; no world borrow
    entity = world.spawn((prepared.cable.clone(), CableOrder(prepared.ordinal))).id()
    world.resource_mut<CableIndex>().add_cable(entity, &prepared.cable)
    mark_propagation_order_dirty(world)
    world.write_message(CableConnected { entity, endpoint IDs, transform, source: provenance })
    return prepared.occurrence

disconnect(world, endpoints, occurrence, provenance):
    entity = resolve_active_cable_in_shared_effective_order(world, endpoints, occurrence)?
    cable = clone actual Cable before despawn
    world.resource_mut<CableIndex>().remove_cable(entity, &cable)
    world.despawn(entity)
    mark_propagation_order_dirty(world)
    world.write_message(CableDisconnected { old entity, old endpoint IDs, source: provenance })
    return success

facade operation:
    result = shared helper(..., DirectApi)?
    cfg(graph): world.run_schedule(GraphMaintenance)
    return result
```

Keep world/resource borrows short. No fallible lookup after spawning/removing.
Events use existing cable_entity/source_port/target_port fields (the pseudocode
abbreviates them), exactly once with actual old/new identity. Add `source:
ChangeSource` to CableConnected and CableDisconnected; patch application emits
PatchLoad, runtime direct emits DirectApi, explicit dispatch emits Command.
This is the necessary unreleased event-surface extension for the approved arc
provenance contract; update all existing constructors/tests and rustdoc. Current
constructors are patch.rs plus four graph mock connect sites and one disconnect
site; recheck with rg. Preserve all other event fields/shape and message registration.
No ParameterChanged/LabelChanged/ModuleAdded/Removed/PatchLoaded emitted by bare
rewiring. Labeling the new cable is a separate Slice02 operation with its own event.
Clarify LabelChanged rack previous rustdoc: runtime writes have Some stored value,
PatchLoad retains None. Update ChangeSource::Command docs for actual dispatch.

Graph freshness is before facade return without app.update or sample execution.
Queries retain StableDiGraph and world-fresh label/getter builders, no graph copy
inside the facade. Connection/disconnection must update edge/entity maps exactly;
remove/add/remove without a tick leaves correct counts and no stale lookup.
The raw explicit command route owns maintenance separately, as documented above.
Do not claim graph now isolates old appended loads; Slice04 retains retirement.

## File-level implementation recipe

1. **core label.rs:** expose existing module/port resolution crate-private; extract
   a crate-private exact endpoint occurrence helper so labels/removal share the
   same key and errors. Keep label public behavior and existing tests unchanged.
2. **core topology.rs (new), error.rs, lib.rs:** implement/export shared connect/
   disconnect helpers, typed errors, preflight projection/order/validation. Reuse
   CableIndex/mark_propagation_order_dirty and CableOrder. Add inline tests for
   private world/index/cache observations; no extra public observability hooks.
3. **core patch.rs/event.rs:** cycle helper pub(crate), add source to patch-load
   CableConnected, implement two existing command arms through shared helpers;
   replace obsolete NotImplemented tests only for those arms. Update event docs/
   source variants and nonblocking predecessor rustdoc clarification. Do not
   implement queued load/save/BPM/panic lifecycle or rewrite validate_patch policy.
4. **facade cables.rs (new), lib.rs:** inherent methods delegate helper then optional
   GraphMaintenance; module registration only. No new dependency or scheduling.
   Inline tests can use private world to compare actual state/pending/Scope.
5. **graph lib.rs:** adapt all constructors for provenance, preserve maintenance
   architecture and existing query contracts. Add real command+maintenance and
   removal/re-add tests where useful. Reconcile native query/getter coverage below;
   repair deterministic neighborhood cable sorting required by the
   historical navigation contract: incoming/outgoing currently follow edge
   iteration, and their existing tests check counts only. All three cable-list
   queries (cables/incoming_cables/outgoing_cables) must sort by source/port/
   target/port, then present/ordinal/Entity within identical endpoints. Build
   keys from each RackEdge.cable_entity and its optional world CableOrder before
   building owned CableInfo; no result schema change or Entity-bits inference
   is necessary. Test opposed insertion/lexical order and opposed ordinal/Entity
   parallel ties. Missing CableOrder (including existing graph mock cables that
   have no Cable component) uses the defined absent/Entity fallback. Do not redesign unrelated queries or result structs.
6. **facade tests/cables.rs (new):** real public loading, rewiring, graph/feature
   coverage and saved configuration/fresh reference; no use of into_app followed
   by reuse of its consumed rack. Use inline tests for inaccessible index/buffer.
7. **rewire-live-patch example (new) + README:** actual musical route below, existing
   clap Parser with --patch and --save PathBuf, ordinary ExitCode/error diagnosis,
   per-sample sole drain, stop after success/error. No MIDI endpoint or fake module.

No new RON fixture is needed: use the accepted original LFO/MIDI fixtures as
immutable inputs and save the derived patch in a temporary user-chosen directory.

## Tests and discriminating oracles

| Row | Setup/action/observable oracle | Plausible mistake rejected |
| --- | --- | --- |
| G-01 | Active-handle source/target, stale/outside names, missing ports; patch duplicates on LastWins; returned occurrences0,1, then unpatch0 shifts second to0 with its literal label/transform preserved. Raw ordered/absent and opposing Entity case selects identical cable through labels and removal | global entity search, ensure/no-op add, remove-all, selector drift |
| G-02 | Each direction wrong; no-transform kind mismatch; invalid Clamp NaN/reversed, non-finite Affine/Threshold, unsupported source and wrong transformed result; legal cases from every table row incl equal Clamp, zero/negative Affine. Reject second fan-in, bad merge policy, real A→B→A cycle, allowed self-loop, usize::MAX overflow. Compare actual before/after entities/index/dirty/messages/graph and pending/time/Scope, exact error variants/fields | any-Some acceptance, panic, spawn-then-validate, wrap, partial mutation |
| G-03 | Real direct and command success/failure: one topology message with exact IDs/transform/source; load messages PatchLoad. Drain observations at call boundaries. Immediate graph nine→eight→nine in fixture without tick; add/remove/add and parallel one-only removal. Core owner explicitly maintains graph after command; facade does itself | duplicate/wrong-source messages, delayed graph update, stale edge maps, accidental sample |
| G-04 | Real Clock/LFO/TM/MIDI at200 and pending252; same-transform remove/re-add before next tick leaves original continuation/control exact, including full Scope and LFO phase, unread event251 retained once and admission unchanged. New constant transform changes next propagation in Update and FixedUpdate, then snapshot/save/fresh load reconstructs literal intended configuration/order | dirty flag missing, phase/RNG reset, tick-to-ack, drain/drop, allocator-dependent ties |
| G-05 | Runnable200→rewire→100 reaches300, literal saved constant+.25 cable/labels, repeated fresh replay versus independently constructed constant+.25 Patch, non-identical original modulation behavior, original fixture hashes/native feature gates, eight Arc04 rows individually accounted | paper-only API, stale saved cable, same-bug self-comparison, inherited rows silently discharged |

Make test setup force order alternatives to disagree: derive opposing ordinal/
Entity order, not assume allocator numbering. Load duplicate identical-endpoint
LastWins cables with constants[.75,.25], add constant.5 (new ordinal wins), remove
only its returned occurrence (winner returns.25), save/fresh reload and repeat;
assert literal vector order and exact CurrentValue each time. Include an unrelated
endpoint with larger ordinal to prove name prefix still precedes ordinals. A raw
absent/equal-order case must retain defined fallback and exclude superseded cables
from max/order/selection. Successful removal of disabled cable is supported; snapshot
of a still-selected disabled cable retains the existing unsupported error.

Use real core Produce→Propagate→MergeInputs→Consume schedule for literal input:
source Float(.75), target Float LastWins, initial transform Affine(1,0); one sample
receives .75. Remove only cable: next sample still .75 (held); add Affine(0,.25):
next sample exactly .25. Re-run same sequence through CorePlugin Update and
CorePluginFixed FixedUpdate. A fresh cache initially rebuilt then edited must
prove add/remove dirtying rather than pass because Dirty was already true.

For engine/pending preservation reuse labels.rs's actual loaded_rack and pending252
setup in new facade inline tests. Clear old messages, record time/driver/running,
all Scope records/overwrite, actual engine-visible output and admission/pending1.
A successful remove/re-add of the original Affine(.5,0) before a tick must leave
all observations unchanged; drain sample251 exactly once, continue against unedited
control per sample. Failed connect/removal separately preserves all of those values.
New constant+.25 topology test compares LFO Scope continuation to untouched control
while testing changed target signal; do not infer phase preservation from matching
two racks both subjected to the same potentially resetting edit.

Fresh replay uses a separately read original Patch with only its named LFO→TM
cable transform changed to Affine(0,.25) and intended metadata applied literally.
It must match the saved-patch rack's whole per-sample event sequence for10000
samples, with explicit saved header/base-parameter and cable membership/order
assertions. Fresh replay restarts engines; do not compare it to the live rack's
continuation after200. A native integration test should show the new effective
modulation is+.25 rather than merely detecting a changed serialized string.

### Historical graph reconciliation, not new query infrastructure

Run the existing graph package's full native suite and record the witness for
all18 existing v0 queries: module_kinds, modules, ports_of, parameters_of,
upstream_of, downstream_of, incoming_cables, outgoing_cables, cables,
dangling_modules, dangling_ports, topological_order, cycles, downstream_closure,
upstream_closure, compatible_inputs_for, module_by_name, modules_of_kind.
Preserve diagnostics' SCC/self-loop distinction, partial topological ordering,
closures including start, missing-input empties, getter missing→None and fresh
current parameter values. On the real loaded facade set tm_main.write_probability
to Float(.25), immediately query graph.parameters_of using the module Entity
resolved from module_by_name.entity_bits, and assert Some(Float(.25)) without a
tick; public parameter read stays .25 in both feature modes. Use bevy_ecs::entity::Entity::from_bits(info.entity_bits), verified against
pinned Bevy0.18.1 src/entity/mod.rs:576; only bits from that same live rack are
valid. Alternatively use an inline test with the existing private PatchHandle
entity lookup while retaining the rack; no new world accessor. Retain configured-base
versus effective-modulation semantics from Slice01. Keep graph absent in no-default facade builds; core
metadata/configuration remain available. Compatibility discovery is by value kind,
not a promise fan-in/cycle/transform checks have all passed; document that limit.
The old diagnostic same-module exclusion was explicitly amended to allow
self-connections; follow actual reconciled source, not stale old task text.

Reuse native tests for unchanged queries; add a missing test only for a named gap
or affected runtime interaction. In the return walk A-04-01/02 and each foundation/
navigation/diagnostics S-01/02 separately, identifying delivered subsets, native
checks and retained future/lifecycle/representation owners. Six child rows aren't
one generic “graph tests passed” claim. Historical rows stay deferred until final
Arc12 assembly/CDC disposition; do not relabel migration as past acceptance.
Arc05 cable ops now deliver patch/unpatch semantics; PortRef/typed handles and
queued/lifecycle/reset requirements remain Slice04, not silently dropped.

## Native validation and Duncan's executable recipe

All commands in S; final source revision recorded. Run:

```bash
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack-graph
cargo test --locked -p oxurack --lib --test cables --test labels --test persistence --test scope --test lfo --test midi_output --example rewire-live-patch --example annotate-live-patch --example save-live-patch --example turing-machine-midi
cargo test --locked -p oxurack --no-default-features --lib --test cables --test labels --test persistence --test scope --test lfo --test midi_output --example rewire-live-patch --example annotate-live-patch --example save-live-patch --example turing-machine-midi
make test
make lint
cargo clippy --locked -p oxurack-core -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
cargo build --locked -p oxurack --example rewire-live-patch --example turing-machine-midi
cargo build --locked -p oxurack --no-default-features --example rewire-live-patch
git diff --check
```

Known baseline: restricted make test can fail four CoreMIDI initialization tests;
retain failed attempt and exact unchanged host-access retry separately. Host RT
currently150 pass/three existing priority/timing ignores. block0.1.6 future advisory
remains visible. No new ignores/suppression, coverage/soak/checker or endpoint gate.
A previous slice checker isn't proof of this one. Real native tests+example and
CRC review are the selected proportionate proof; evidence beyond them needs a gap.

Example behavior (binding): install real four modules, load original LFO patch,
Manual start, advance/drain200 one sample at a time. unpatch LFO bipolar_out→TM
write_modulation occurrence0, immediately inspect graph count8 when enabled.
patch same endpoints Affine{factor:0,offset:.25}; returned occurrence0 for fixture,
immediate graph count9; set its Description to “Constant +0.25 write modulation”.
Save current patch, inspect raw description and selected cable transform via owned
snapshot, advance/drain100; stop on success/error. Print completed_samples300,
occurrence, actual transform/description, saved path, and graph counts when enabled.
No graph-specific stdout assumption in no-default mode; metadata/save still work.

Representative Rust usage, **uncompiled until CC implements**:

```rust
let source = ("lfo_main", "bipolar_out");
let target = ("tm_main", "write_modulation");
rack.unpatch(source, target, 0)?;
let occurrence = rack.patch(source, target,
    Some(CableTransform::Affine { factor: 0.0, offset: 0.25 }))?;
let selector = LabelTarget::Cable {
    source: (source.0.into(), source.1.into()),
    target: (target.0.into(), target.1.into()), occurrence,
};
rack.set_label(&selector, LabelKind::Description,
    LabelValue::Text("Constant +0.25 write modulation".into()))?;
rack.save_patch(&args.save)?;
```

Document and actually execute the following; temporary files aren't new artifacts:

```bash
OXURACK_REWIRE_DIR=$(mktemp -d /tmp/oxurack-rewire.XXXXXX)
cargo run --locked -p oxurack --example rewire-live-patch -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --save "$OXURACK_REWIRE_DIR/rewired.ron"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_REWIRE_DIR/rewired.ron" --samples 10000 \
  > "$OXURACK_REWIRE_DIR/rewired-1.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_REWIRE_DIR/rewired.ron" --samples 10000 \
  > "$OXURACK_REWIRE_DIR/rewired-2.trace"
cmp "$OXURACK_REWIRE_DIR/rewired-1.trace" "$OXURACK_REWIRE_DIR/rewired-2.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_REWIRE_DIR/rewired.ron" --samples 10000 \
  --scope-view --scope-port tm_main.write_modulation \
  > "$OXURACK_REWIRE_DIR/view.trace" 2> "$OXURACK_REWIRE_DIR/view.txt"
cmp "$OXURACK_REWIRE_DIR/rewired-1.trace" "$OXURACK_REWIRE_DIR/view.trace"
```

Inspect actual RON and Scope: target modulation held+.25 after propagation,
overwrite0 at default capacity100000 for this single selector; generated/sent
counts distinguish offline behavior. Native integration provides independent
literal-reference replay; don't add a duplicate script validator. Reproduce
original LFO10000 hash988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c
and MIDI2000 hash98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b.
Require rewired replay differ from original LFO sequence, with target+.25 native
oracle showing why; report its actual count/hash rather than invent a pinned value.
Missing/malformed patch and save-to-directory exit nonzero with diagnosis. Run
example in no-default mode too; no endpoint open or hardware result is claimed.

## Edit fence, commits, stops and return

Complete source allowlist relative to S; only changed paths committed:

```text
crates/oxurack-core/src/topology.rs
crates/oxurack-core/src/label.rs
crates/oxurack-core/src/event.rs
crates/oxurack-core/src/error.rs
crates/oxurack-core/src/lib.rs
crates/oxurack-core/src/patch.rs
crates/oxurack-graph/src/lib.rs
crates/oxurack/src/cables.rs
crates/oxurack/src/lib.rs
crates/oxurack/tests/cables.rs
crates/oxurack/examples/rewire-live-patch.rs
README.md
```

This fence includes necessary production/integration/tests/docs, not blanket
source protection. No Cargo/lockfile, accepted fixture, engine/adapter, tick,
persistence/Scope production or lifecycle change. New public observability/accessor
isn't needed for private inline tests. On a real necessary outside-path change,
stop affected work and report exact criterion/source/proposed resolution to CRC;
do not silently widen or weaken. Routine local helpers inside the contract proceed.

Explicitly stage only changed/new allowed paths; `git commit --only -- <exact
changed paths>` for source. Separate planning return limited to D/closing-report.md
and D/ledger.md, explicit paths. No blanket add/stash/reset/cleanup, main edit,
merge/push/release or direct CDC/CC messages. Artifacts none expected; explain a
real durable supporting-output need before creating it under D/artifacts/.
No new checker or evidence tree by default.

Return source/planning commits, clean states, assignment path/hash, changed paths,
itemized intake extents/conditional status and readback; actual per-G-row results,
command/exits/denominators, preserved failures/retries and unrun gates; self-review
of atomic preflight, ordering, index/dirty, provenance, immediate graph, held-input
and persistence interactions. Bubble-up: separately walk eight Arc04 rows and
Arc05 cable/remaining API obligations; retain file-validator compatibility limit
and Slice04 old-entity/lifecycle owners. Proposed done remains open pending CRC;
CC doesn't close slice/arc or advance Slice04. Preserve this prompt and prior
reports. Corrections, if justified, use a new root iteration prompt.
