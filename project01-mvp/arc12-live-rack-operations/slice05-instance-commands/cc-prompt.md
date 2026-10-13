# CC implementation — usable instance removal, handles and commands

2026-10-13. Sofie/CRC → Duncan → CC. Three contributors, Expedited Mode.
Initial assignment: `arc12-live-rack-operations/slice05-instance-commands/cc-prompt.md`.
Implement the final approved Arc12 increment, not an investigation-only return.
All six IC rows start open. Slice04 is CRC accepted at ec85ebe; CDC12 D3/D4
bind this slice, and D1/D2 still govern lifetimes and MIDI. No new design gate.

S=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, clean source
`ec85ebe99c09b8a2ea64d55da0346af4ae317a67`.
W=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch planning;
issuance descends from CC return `972e3b7220f0e660875dcdc887763765ebd64d48`.
P=W/project01-mvp; A=P/arc12-live-rack-operations; D=A/slice05-instance-commands.
Protected R=`/Users/oubiwann/lab/oxur/oxurack`, main
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
K=`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`;
F=K/collaboration-framework/knowledge. Framework2.6.0; Rust-guidelines2.0.0.
Check R/S/W cwd/branch/HEAD/status; unexpected source drift returns through CRC.
Do not reset others' work. Complete coherent intake, then implement autonomously.

## Reading and intake before dependent edits

Load required text completely in bounded contiguous chunks, recovering truncation.
Hashes/headings/search hits/previous-context summaries do not prove receipt of
required text. Record itemized state/extents/recovery and a concise source-cited
contract readback in **D/closing-report.md**, created when work begins. Readback
must connect handle identity, cleanup-before-retirement, queue slots/cancellation,
Command provenance and global-versus-Clock BPM to their tests. Missing/conflicting
binding inputs block dependent work, not unrelated work. No extra intake artifact.

| Order | Exact input/state | Required extent | Purpose |
| --- | --- | --- | --- |
| 1 | W/AGENTS.md; S/CLAUDE.md | required-full both | Roles, source fence, conventions |
| 2 | This prompt; D/slice-plan.md; D/ledger.md | required-full all three, issuance | Active scope/API/oracles and six opening rows |
| 3 | P/cdc-directive12.md | required-section: Decisions and delivery; D1 lifecycle semantics/matrix; full D2; full D3; D4 source boundary, Discriminating proof and Guidance/readiness | Binding05 design and interacting04 contract |
| 4 | P/cdc-directive08.md | required-section: Prompt quality; Checks and correction policy | Playable-first/proportionate proof and native gates |
| 5 | P/project-plan.md; A/arc-plan.md; A/ledger.md | required-section: project metadata/Current direction; arc Capability/Five delivery slices/Binding design/Current lifetime contract; full arc ledger | Current authority and higher-scale gates |
| 6 | A/slice04-runtime-api/crc-verification.md | required-section: Iteration03 independent review and slice closure, complete | Accepted predecessor, affected/retained evidence |
| 7 | K/collaboration-framework/SKILL.md; F/project-management/guides/README.md | required-full both | Workflow and Expedited Mode |
| 8 | F/engineering-methods/guides/07-implementation-prompt-authoring.md; F/work-verification/guides/05-independent-verification.md | required-full both | Intake and honest proposed evidence/retention |
| 9 | K/testing/SKILL.md; F/testing/guides/01-testing-discipline.md;03-validation-gates.md | required-full all | Meaningful integration tests, Proportionate Proof |
| 10 | K/rust-guidelines/SKILL.md; K/rust-guidelines/guides/11-anti-patterns.md | required-full both, before Rust changes | Rust guidance actually applies to execution |
| 11 | K/rust-guidelines/guides/02-api-design.md;03-error-handling.md;05-type-design.md | required-section: API-02/07/23; EH-01/14; TD-02/03/05/06/07/17, including complete examples/rationale | Owned refs, private identities, typed failures and queue states |
| 12 | S/crates/oxurack/src/{lib.rs,lifecycle.rs,persistence.rs,cables.rs,labels.rs} at ec85ebe | required-section: production bodies through cfg(test); inline lifecycle RetainedState/first-fault helpers and disabled-restart test; cable/label provenance tests | Constructor/install/commit/admission/source sharing and private test access |
| 13 | S/crates/oxurack/src/scope.rs | required-section: ScopeSelector/ScopeCapture fields/accessors; configure/disable/install_observer/observe_scope | Selective capture invalidation and survivor observation |
| 14 | S/crates/oxurack-core/src/{module.rs,topology.rs,cable.rs,event.rs,error.rs,lib.rs} | required-section: production bodies through cfg(test); topology endpoint/ordering/preflight helpers; all CoreCommand/structural notifications/dispatch; CableIndex; CorePlugin schedule/message setup | Hook registration, owned removal preflight, cable commit reuse and exports |
| 15 | S/crates/oxurack-mod-midiout/src/lib.rs | required-section: production body through cfg(test); cleanup/channel accounting tests | Selected silence and reservation release without discarding MIDI |
| 16 | S/crates/oxurack-graph/src/lib.rs | required-section: RackGraph fields/public queries, plugin schedule, full four structural handlers; graph_test_app, test_module_removed_cleans_incident_edges | Both edge directions and guarded name mapping; direct notification proof |
| 17 | S/crates/oxurack/tests/lifecycle.rs; S/crates/oxurack/examples/save-live-patch.rs | required-section: lifecycle imports/rack, late failure/normalized structural/seeded replay helpers and sample300 control; example Args/run_with_output/run_lifecycle/drain helpers | Real module/replay and writer-based example seams |
| 18 | S/crates/oxurack/examples/patches/turing-machine-lfo.ron; turing-machine-midi.ron in same directory; S/Cargo.toml; S/crates/oxurack/Cargo.toml; S/Makefile | required-full all | Existing fixtures/dependencies/features/native gates |
| 19 | P/arc05-public-api/ledger.md; A/slice04-runtime-api/slice-plan.md | required-section: Arc05 current two A rows/final CDC disposition;04 Boundaries and inherited work/Final review and bubble-up | Explicit retained successor accounting, no false full historical-API claim |

Conditional: before using unfamiliar dependency methods inspect the **locked**
local source (Bevy ECS0.18.1, petgraph0.7.1); do not use an unrelated installed
version. CRC checked ChildOf/Children linked-spawn cascade and World::despawn,
StableDiGraph::edges_directed with Direction::{Incoming,Outgoing}. No new crate,
runtime or dependency upgrade. Old directives/prompts are reference-only unless
named above; do not inherit their obsolete transport semantics. No structured
dataset query applies. Budget approximately65–90k tokens for source/required
text/design/implementation and recovery; chunk rather than skim. On compaction,
revalidate assignment/state and reload unavailable contracts before dependent work.

## Source-grounded implementation route and guidance

CRC inspected ec85ebe's production seams and tests. Removal/handles/queue do not
exist. Oxurack owns App, ordered InstalledType records, driver and LoadedIdentity;
commit swaps only App/identity. Core ModuleRegistry already keeps a side map for
context spawners, allowing an analogous removal-hook map without breaking public
ModuleRegistration literals. CableIndex owns source/target maps; topology.rs
already validates names and commits disconnects. Graph removal uses edges(idx),
which inspects outgoing edges only, and unconditionally deletes a name mapping.
MidiOutputBuffer has fixed128 storage and reserves channels but no release method.
CoreCommand has eight existing variants and low-level facade-only NotImplemented.

| Applied guidance / strength | Decision and rationale | Implementation/proof |
| --- | --- | --- |
| API-02 SHOULD; API-07 MUST | Owned name refs/results, borrowed rack arguments; private Arc tokens, no public Entity or Arc parameter | handles.rs identity checks, cross-rack/stale tests |
| API-23 MUST; EH-01 MUST | Named recoverable errors; unsupported input/admission/queue failures before mutation; user hook panic outside rollback | error.rs; removal and queue error snapshots |
| TD-02/03 SHOULD; TD-05 MUST | Opaque CommandId and Handle, private bindings; exact names remain configuration references | New public types/constructor tests |
| TD-06/07 SHOULD | Pending/Completed slot enum, non-exhaustive outcome status; no parallel unbounded results store | cancellation/capacity/result-collection tests |
| TD-17 SHOULD | Handle<M> uses invariant PhantomData<fn(M)->M>, owns no M, manual Clone avoids requiring M: Clone | Compile/use real Clock/LFO/TM handles |
| EH-14 MUST | Document admission, invalidation, dropped drain/into_app queue behavior, hooks/panic boundary and global BPM | strict docs and real example |
| Testing Proportionate Proof | Extend native/private/public tests and existing musical fixture; no new verifier framework or hardware gate | Six IC rows below |

Binding: CDC12 behavior and exact public forms below. Recommended: private helper
shapes/code sketches; they are **illustrative and uncompiled**, not tested drop-in
code. Local discretion: helper names/grouping, ordinary private allocation outside
the sample hot path, test organization inside the named fence. No concurrency,
background queue consumer, serialized token/state, schema migration or new scheduler.
Any consequential contradiction returns through Duncan/CRC to CDC before affected
implementation; do not soften an oracle to fit a mistaken design.

## 1. Core removal and selected MIDI retirement

Add default infallible `OxurackModule::on_remove(&mut World, Entity)` and erased
hook storage in ModuleRegistry. register::<M> records M::on_remove; manually
inserted legacy registrations use no-op. Copy the hook pointer out before calling
it, so no registry borrow survives the mutable World call. Exactly once for a
successful **in-place** instance removal; never per-instance during whole-App
retirement. Hook may release that instance's shared registrations, not fail,
emit MIDI or modify unrelated instances. Panic stays outside recoverable guarantees.

In core topology.rs factor the existing entity-level disconnect commit so removal
can use it after complete preflight, without re-resolving names or adding a later
recoverable error. Recommended cross-crate seam, documented as module-author/internal
rather than musician facade:

```rust
// Illustrative proposed types/seams, not current or compiled APIs.
pub struct PreparedModuleRemoval { /* owned identity, ordered cables, hook */ }
pub fn prepare_module_removal(world: &mut World, name: &str)
    -> Result<PreparedModuleRemoval, CoreError>;
impl PreparedModuleRemoval { pub fn module_entity(&self) -> Entity; }
pub fn commit_module_removal(world: &mut World, removal: PreparedModuleRemoval,
    source: ChangeSource); // all recoverable checks have already passed
```

Preflight resolves **active PatchHandle membership**, confirms Module name/kind,
registry hook and owned hierarchy (reject a different active module nested in
the retiring subtree), gathers descendant port identities, then all
incoming/outgoing/self cables touching those ports. Deduplicate self-edges; order
by existing effective `(order absent, CableOrder ordinal, Entity)` key. Retain
owned cable/endpoints for later notifications/index removal. Existing NoPatchLoaded/
UnknownModule/UnknownModuleKind cover absent names/kinds; add
`InvalidModuleOwnership { module: String, reason: String }` for inconsistent active
membership/hierarchy. No mutation/messages/output/Scope change during preflight.

Commit ordered cable disconnects (CableIndex removal, cable despawn,
CableDisconnected DirectApi), then the copied on_remove hook once, remove the
PatchHandle entry and recursively despawn the module/owned children, emit one
ModuleRemoved and mark propagation order dirty. ModuleRemoved's existing payload
has no provenance field; keep it, document its actual producer. Preserve unrelated
children/cables/registries, RackMeta/header and all survivor engines. Bevy0.18.1
ChildOf/Children linked-spawn already cascades descendants on World::despawn;
prove custom grandchild retirement rather than introducing a guessed despawn API.
Do not rebuild the App for in-place removal.

Midi-out: factor prepare_silence to retain whole-world accounting checks while
selecting only the requested sink's dirty channel, e.g. new
`prepare_silence_for(world: &World, module: Entity) -> Result<SilenceBatch, TickError>`.
A non-MIDI module yields an empty batch; never silence unrelated dirty sinks.
Use append_silence's all-room check **before** any teardown. MidiOutModule::on_remove
reads its config and infallibly releases that channel, decrementing sink_count and
normal logical capacity5 per sink; fixed128 storage and pending cleanup remain.
No additional allocation in cleanup/sample execution. Removed sink state dies
with its entity; reservation release cannot clear unread cleanup or other fault.

Facade `remove_module(&mut self, name: &str) -> Result<(), CoreError>`:

```text
require_stopped_and_drained()   // running first, pending second; faults allowed
prepare_module_removal()        // all identity/ownership checks before changes
prepare_silence_for()           // read-only selected sink / full accounting
append_silence()                // all-room checked before appending; no later Err
if Scope selects removed name: disable Scope
commit_module_removal(DirectApi)
remove only its private instance token
run GraphMaintenance if graph enabled; return Ok without ticking
```

Leave SampleTime/Fixed/Virtual/driver/running/fault and facade queue unchanged.
Scope targeting only survivors retains configuration, records and overwrite count;
any selector targeting the removed name disables the entire capture. Caller drains
selected cleanup before another sample. Repeated missing name returns UnknownModule,
not a successful no-op. Last module removal leaves a **loaded empty patch**, not
unload: snapshot/header/BPM remain available and installed recipes still exist.

Graph handler: collect incident cable entities from BOTH edges_directed directions
into a HashSet before remove_node; clear each edge_by_entity mapping once. Remove
by_instance_name only when its current Entity equals the removed Entity. Preserve
another same-name mapping already pointing at a newer node. Direct ModuleRemoved
notification tests must not first disconnect edges, which would hide the handler
bug. Facade tests also prove immediate graph freshness and actual World retirement.

## 2. Owned references and stale-safe typed handles

Add handles.rs and facade exports. Binding public signatures:

```rust
// Signature sketches, not executable bodies.
impl ModuleRef {
    pub fn new(module: impl Into<String>) -> Self;
    pub fn module(&self) -> &str;
    pub fn port(&self, port: impl Into<String>) -> PortRef;
    pub fn knob(&self, parameter: impl Into<String>) -> KnobRef;
}
impl PortRef {
    pub fn new(module: impl Into<String>, port: impl Into<String>) -> Self;
    pub fn from_dotted(value: &str) -> Result<Self, CoreError>;
    pub fn module(&self) -> &str;
    pub fn port(&self) -> &str;
}
impl KnobRef {
    pub fn new(module: impl Into<String>, parameter: impl Into<String>) -> Self;
    pub fn from_dotted(value: &str) -> Result<Self, CoreError>;
    pub fn module(&self) -> &str;
    pub fn parameter(&self) -> &str;
}
impl<M: oxurack_core::OxurackModule> Handle<M> {
    pub fn name(&self) -> &str;
    pub fn parameter(&self, rack: &Oxurack, param: &str) -> Result<ParameterValue, CoreError>;
    pub fn set_parameter(&self, rack: &mut Oxurack, param: &str, value: ParameterValue)
        -> Result<(), CoreError>;
    pub fn port(&self, name: &str) -> PortRef;
    pub fn knob(&self, name: &str) -> KnobRef;
}
impl Oxurack {
    pub fn module<M: oxurack_core::OxurackModule>(&self, name: &str) -> Result<Handle<M>, CoreError>;
    pub fn patch_ports(&mut self, source: &PortRef, target: &PortRef,
        transform: Option<CableTransform>) -> Result<usize, CoreError>;
    pub fn unpatch_ports(&mut self, source: &PortRef, target: &PortRef, occurrence: usize)
        -> Result<(), CoreError>;
    pub fn knob(&self, reference: &KnobRef) -> Result<ParameterValue, CoreError>;
    pub fn set_knob(&mut self, reference: &KnobRef, value: ParameterValue) -> Result<(), CoreError>;
}
```

ModuleRef is an unbound lookup name; its builders are unbound. PortRef/KnobRef
carry owned names plus **optional private binding**. Handle builders copy the
binding; they need not resolve/validate the specific port/knob until use. Exact
case-sensitive tuple names support dots. from_dotted requires exactly one literal '.' separator, with nonempty sides
and no other dots; reject ambiguous/empty forms with `InvalidReference { value: String }`.
Do not escape/split-at-last or silently guess dotted names.

Recommended private `Binding { lifetime: Arc<()>, instance: Arc<()> }`;
facade holds one lifetime token plus name→instance token map. Constructor gets
fresh lifetime even empty. Every successful candidate commit prepares/replaces
both token collections from the candidate's active PatchHandle; unload creates
fresh empty tokens. Failed admission/snapshot/candidate leaves tokens untouched.
In-place removal deletes only that name's token; reset/stop/edits do not rotate any.
Do not serialize tokens or use Entity/ModuleId alone, a wrapping counter or a
global service: retained Arc allocations distinguish same-name/entity reuse.

module::<M> resolves current active membership, Module kind and **installed Rust
TypeId**, not just M::KIND; validate both. Reuse InstalledType.type_id and retain
private type_name::<M>() for useful mismatch diagnostics. Return
`ModuleTypeMismatch { module: String, expected: String, actual: String }` on wrong
kind/type, including a distinct Rust type claiming the same kind. Existing missing
patch/name errors apply to unbound resolution. Bound validation first checks
Arc::ptr_eq lifetime and map instance token, then confirms active membership.
Cross-rack, removed name, unload/restart/replacement return
`StaleHandle { module: String }`, even if a current same-name instance exists.
Survivor bindings remain valid when a different instance is removed. End token/
World borrows before mutation. Clone/Debug refs and handles without M: Clone/Debug
bounds or exposing pointers; small manual impls are fine. No public mutable World.

Worked private identity check (illustrative proposed helper, not compiled):

```rust
impl Binding {
    fn matches(&self, lifetime: &Arc<()>, instance: Option<&Arc<()>>) -> bool {
        Arc::ptr_eq(&self.lifetime, lifetime)
            && instance.is_some_and(|token| Arc::ptr_eq(&self.instance, token))
    }
}
```

Call this with the facade's current lifetime and the named instance token. False
returns StaleHandle before any getter/setter/cable helper; true still requires
active PatchHandle membership/type validation. Retaining the binding's Arc prevents
allocation reuse from reviving a retired handle. Do not compare names alone.

Every bound handle/port/knob operation validates **before** shared tuple/name helpers.
Validate both cable refs before mutation; no internal conversion to an unbound
name can bypass it. Existing string APIs remain available; factor shared behavior
in persistence/cables/labels for provenance rather than duplicate registries or
parameter logic. Caller explicitly constructing a new unbound name reference
performs a current lookup; it is not the old binding becoming valid again.

## 3. Synchronous commands and global BPM

Export existing CoreCommand from oxurack plus new command result types.
`execute_command(&mut self, &CoreCommand) -> Result<(), CoreError>` handles all
**eight** current variants: LoadPatch, SavePatch, SetParameter, SetLabel, AddCable,
RemoveCable, SetBpm, Panic. No success stubs. Load uses the actual admission/read/
candidate path with `PatchSource::Other(format!("command load: {}", path.display()))`;
refactor lifecycle load helper to accept source rather than mutate event provenance
post-commit. File validation/setter initialization still use PatchLoad. Save uses
save_patch and emits no mutation event. Panic calls reset. SetParameter/SetLabel/
AddCable/RemoveCable share the existing implementations with ChangeSource::Command;
RemoveCable keeps occurrence0. Maintain graph before successful cable commands
return. Direct/bound facade calls stay DirectApi. Unknown future non-exhaustive
variants return explicit NotImplemented rather than Ok. Low-level core dispatcher
retains NotImplemented for Load/Save/Panic/SetBpm and no automatic message consumer.

Add `bpm(&self) -> Result<f32, CoreError>` / `set_bpm(&mut self, f32) -> Result<(), CoreError>`
in persistence.rs. They access LoadedIdentity.bpm; NoPatchLoaded without identity,
finite and >0 else existing PatchError::InvalidBpm. No engine/time/Scope/output or
notification mutation; snapshots/save/restart retain the global field. **Clock's
own bpm parameter remains unchanged**. The legacy global header is configuration;
audible tempo still uses Clock's parameter. No RT synchronization/transport feature.

## 4. Bounded explicit queue and reset cancellation

Add commands.rs; facade owns CommandQueue outside the App and keeps it through
successful load/restart/unload. Public forms:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandId(/* private u64 */);
#[non_exhaustive]
#[derive(Debug)]
pub enum CommandResult { Completed(Result<(), CoreError>), CancelledByReset }
#[derive(Debug)]
pub struct CommandOutcome { pub id: CommandId, pub result: CommandResult }
// enqueue_command(&mut self, CoreCommand) -> Result<CommandId, CoreError>
// process_commands(&mut self) -> Vec<CommandOutcome>
```

CommandId Display may show its numeric value; no public construction. No Clone
requirement on CoreError/outcomes. Private slot enum is Pending{id,command} or
Completed(outcome), held by a VecDeque reserved for64. queue.len counts **all
outstanding**, including uncollected cancellations. next_id starts0 and uses
checked_add before accepting; on full return `CommandQueueFull { capacity: 64 }`,
on exhaustion `CommandIdExhausted`, with counter/slots unchanged. No recycling,
retry or background acceptance. process_commands explicitly drains every slot in
submission order, executes Pending against the then-current rack, continues after
Err, and returns Completed or cancelled outcomes exactly once. Paths/names/values
resolve at **execution**, not enqueue. No sample/tick is an acknowledgment.

```text
process_commands:
    while pop_front slot exists:
        Completed(outcome): append outcome to this call's returned vector
        Pending(id, cmd): result = execute_command(&cmd)
                         append Outcome(id, Completed(result))
    return outcomes
reset:
    silence_outputs(world)?     // all fallible checks before voice-state commit
    for remaining queue slots in place, submission order:
        Pending(id, ..) => Completed(Outcome(id, CancelledByReset))
        Completed(..) => preserve
    return Ok
```

Worked queue execution shape (illustrative proposed private field/type names,
not compiled; adapt names to commands.rs without changing the pop-before-execute
boundary):

```rust
pub fn process_commands(&mut self) -> Vec<CommandOutcome> {
    let mut outcomes = Vec::with_capacity(self.commands.slots.len());
    while let Some(slot) = self.commands.slots.pop_front() {
        let outcome = match slot {
            Slot::Completed(outcome) => outcome,
            Slot::Pending { id, command } => CommandOutcome {
                id,
                result: CommandResult::Completed(self.execute_command(&command)),
            },
        };
        outcomes.push(outcome);
    }
    outcomes
}

// Called only after successful silence_outputs, never before a fallible check.
fn cancel_pending(&mut self) {
    for slot in &mut self.slots {
        if let Slot::Pending { id, .. } = slot {
            let id = *id;
            *slot = Slot::Completed(CommandOutcome {
                id,
                result: CommandResult::CancelledByReset,
            });
        }
    }
}
```

The cancellation helper belongs to CommandQueue; process_commands belongs to
Oxurack. Its local results survive a queued Panic while the still-owned queue tail
changes in place. Taking the entire queue into a local variable before executing
would hide the tail from reset and violate the contract.

Popping the queued Panic **before executing** means its successful reset cancels
only its still-pending tail. Earlier returned results survive; Panic gets its own
Completed(Ok). Direct reset leaves cancellation slots until process_commands collects
them. Completed cancellation is not overwritten by later reset. New submissions
fit only the remaining64 capacity. Failure of reset cancels nothing. Reset's existing
MIDI pending/fault/state guarantees remain; empty-rack reset cancels commands without
MIDI. process_commands owns a local result vector; no second persistent/unbounded
result store or Bevy receiver. into_app documentation warns consumption abandons
unprocessed facade slots/results; callers must collect first. Queue functions run
between samples with exclusive &mut self, not concurrent producers.

## 5. Real tests and musical example

Map these checks to IC-01–IC-06. Use inline tests for private owner/queue state;
public integration tests for actual paths, handles and musical replay. A consuming
into_app must be last; do not reuse its owner. Keep all accepted lifecycle tests.

| Row | Concrete check/oracle | Incorrect implementation rejected |
| --- | --- | --- |
| IC-01 | Loaded custom A→B→C plus B→B and parallel ordered incident cables; owned port/grandchild retirement, full CableIndex source/target roster and immediate graph, exactly ordered disconnects then ModuleRemoved once; survivor labels/header/time/clocks/engine/getters/Scope/queue unchanged; missing/running/unread/invalid ownership errors compare state before/after | Outgoing-only cleanup, dangling index/children, rebuild instead of removal, success notifications on error |
| IC-02 | Real handles and bound port/knob APIs; wrong Clock-as-LFO and different Rust type with same kind; cross-rack, removed name, same-name replacement, restart/unload, failed replacement retains handles; survivor handle remains valid; dotted rejection and exact tuple names containing dots | Kind-only check, Entity/token reuse, stale-to-unbound bypass, token rotation on failed load or unrelated remove |
| IC-03 | Each of eight facade commands reaches real implementation; Command versus DirectApi event fields, graph fresh without tick; global133 saves/restarts while Clock parameter120 remains120; NaN/±infinity/zero/negative errors leave configuration unchanged; core four unsupported variants remain errors | Stubs, global BPM changes audible Clock, false load provenance, hidden tick |
| IC-04 | IDs/FIFO load→edit→save; file created/modified after enqueue used at execution; failed load→save preserves old owner and later success;64th accepted/65th full; private MAX next-id exhaustion; direct reset cancellation collected once; queued [edit,Panic,edit,save] produces [Ok,Ok,Cancelled,Cancelled]; cancelled slots fill capacity; failure of reset leaves exact Pending slots/state | Queue lost in App swap, cancel all including prior/Panic results, silent cancelled-result loss, wrapping IDs, swallowed errors |
| IC-05 | Actual MIDI fixture played/drained300, stopped; removing midi_main yields exactly [(300,[0x80,62,0]),(300,[0xB0,123,0]),(300,[0xB0,120,0])], stamp/time unchanged and cleanup still drainable with zero sinks; fault retained; a remaining different dirty sink is not silenced; internal accounting proves removed channel immediately reusable and fixed storage unchanged | Clearing events with reservations, all-channel panic for one removal, wrong sample, fault clearing |
| IC-06 | Runnable session default/no-graph, real handles/removal/FIFO persistence, final saved snapshot/fresh seeded MIDI and Scope equality; six-row/inherited walk; required native/docs gates | Bookkeeping-only example, fabricated musical/release acceptance |

IC-01's B→B is permitted by current check_patch_cycles, which skips same-module
edges; use matching Float output/input and a non-Reject merge where parallel
cables need it. Direct graph notification regression must independently include
incoming/outgoing/self-edge mappings and same-name newer entity; no CableDisconnected
before ModuleRemoved. Assert maps/contains_cable as well as petgraph counts.

For survivor musical continuity remove an otherwise unconnected real LFO instance
from one of two equal loaded fixtures after300 samples, with Scope selecting only
survivors. Compare all subsequent selected scalar/MIDI values with uninterrupted
control, clocks/running/fault and records; no reset on either. In a separate test
capture selects removed LFO and must disable. For faulted removal stop/drain first;
removal preserves the first fault, unlike a successful new lifetime. Channel-reuse
proof may spawn a fresh raw sink in a **private** inline test after draining cleanup;
it is not a new public add-instance feature. Dirty second-sink test uses direct
private inputs with two unique channels; removing channel0 must not clear channel1.

Queue reset failure can use an inline deliberately inconsistent raw MIDI world:
spawn a sink then despawn it without releasing the reservation, so existing
prepare_silence reports accounting OutputCapacity. Capture pending IDs/commands,
fault/events/time before reset; Err leaves them identical and subsequent processing
returns ordinary command results, not CancelledByReset. Do not call this injected
invariant a defect in normal configured playback or add a public fault injector.
Keep current128 peak/reset/fault tests, seeded replay and no-catch-up checks intact.

New `crates/oxurack/examples/rack-session.rs`, using existing clap dev dependency:
`--patch PATH --save PATH --samples N` (samples default2000), no endpoint option.
Implement writer-based run function plus CLI/main as in save-live-patch, with
in-process executable tests for its real sequence and failures. Sequence:

1. Install Clock/LFO/TM/MIDI; load LFO fixture; Manual; retain handles. Configure
   LFO/TM Scope, run300 admitting/draining per sample; record every drained event
   to phase-labelled status output. Stop; demonstrate handle parameter edit .25.
2. Remove lfo_main; show its bound knob is stale, TM handle survives, Scope is
   disabled, graph is fresh when enabled. Do not remove MIDI here. Scope/cleanup
   semantics are covered separately above; this is actual musical session use.
3. Enqueue SavePatch(save), LoadPatch(a guaranteed missing transient path),
   LoadPatch(save), SetParameter(tm_main.write_probability,.5), SetBpm(133),
   SavePatch(save). process once; verify ordered typed outcomes, the one expected
   failure, and final actual saved values. Successful replacement must not lose
   its queued tail. Record/drain carried cleanup to status before starting.
4. Configure clock/TM Scope; run N per-sample/drain. Write **only this final
   lifetime's MIDI trace** as `sample|HEX HEX HEX` to stdout. Compare the same N
   trace/scalars/copied MIDI with an independently loaded final saved patch inside
   the example; report comparison/sample/Scope counts and unchanged Clock BPM on
   stderr. Record any final voice cleanup to status; no generated event discarded.
   Allocation and finite vectors here are demonstration code outside the sample
   hot path. Keep zero-sample/invalid args/I/O errors honest and nonzero.

Use immutable source fixtures; saves/logs under a fresh temporary directory.
CLI supports this documented reproducible recipe (adapt only private details):

```bash
OXURACK_SESSION_DIR=$(mktemp -d)
cargo run --locked -p oxurack --example rack-session -- --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --save "$OXURACK_SESSION_DIR/session.ron" --samples 2000 > "$OXURACK_SESSION_DIR/session.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- --patch "$OXURACK_SESSION_DIR/session.ron" --samples 2000 > "$OXURACK_SESSION_DIR/fresh.trace"
cmp "$OXURACK_SESSION_DIR/session.trace" "$OXURACK_SESSION_DIR/fresh.trace"
cargo run --locked -p oxurack --no-default-features --example rack-session -- --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --save "$OXURACK_SESSION_DIR/no-graph.ron" --samples 2000 > "$OXURACK_SESSION_DIR/no-graph.trace"
cmp "$OXURACK_SESSION_DIR/session.trace" "$OXURACK_SESSION_DIR/no-graph.trace"
```

Examples are prospective, not claimed executed by CRC. README documents global
BPM versus Clock, current-lifetime handles, stopped/drained removal and exactly-once
queue/reset results. No hardware/software endpoint is opened by this assignment.

## Gates, exact scope and return

Fresh final-source gates from S; preserve every failure, correction and retry:

```bash
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack-mod-midiout
cargo test --locked -p oxurack-graph
cargo test --locked -p oxurack --all-targets
cargo test --locked -p oxurack --no-default-features --all-targets
make test
make lint
cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
cargo build --locked -p oxurack --examples
cargo build --locked -p oxurack --no-default-features --examples
git diff --check
```

Run the recipe above plus original MIDI2000 (13 rows, SHA256
98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b) and LFO10000
(70 rows, SHA256 988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c).
Current scope changes integration, so facade/graph/core/MIDI behavior must be fresh.
Known restricted make test has four CoreMIDI initialization failures (RT146pass,
4fail,3existing ignores); preserve and retry **unchanged** with host access. No
warning suppression/new ignores. Keep existing block advisory/unused_mut visible.
No make check/check-deps/outdated, new coverage campaign, verifier or audit tool.
Retained ancestor/hardware receipts retain dates/revisions and prove only their
named claims; no endpoint/release acceptance here.

Authorized source files (necessary production **and** tests inside these paths):

- crates/oxurack/src/lib.rs
- crates/oxurack/src/lifecycle.rs
- crates/oxurack/src/persistence.rs
- crates/oxurack/src/cables.rs
- crates/oxurack/src/labels.rs
- crates/oxurack/src/scope.rs
- crates/oxurack/src/handles.rs (new)
- crates/oxurack/src/commands.rs (new)
- crates/oxurack-core/src/module.rs
- crates/oxurack-core/src/topology.rs
- crates/oxurack-core/src/cable.rs
- crates/oxurack-core/src/tick.rs (dirty/order seams only)
- crates/oxurack-core/src/event.rs
- crates/oxurack-core/src/error.rs
- crates/oxurack-core/src/lib.rs
- crates/oxurack-mod-midiout/src/lib.rs
- crates/oxurack-graph/src/lib.rs
- crates/oxurack/tests/lifecycle.rs
- crates/oxurack/tests/instance_commands.rs (new)
- crates/oxurack/examples/rack-session.rs (new)
- README.md

Saves/temp fixtures stay transient; source RON, existing examples, Cargo manifests/
lockfile, standalone engine algorithms, native RT/transport, main are outside scope.
New dependencies/schema/architecture or additional source paths require CRC/CDC
resolution before affected edits. Fix adjacent bugs within the approved behavior
and named files; private organization is discretionary. No merge/push/native endpoint.

Planning edits only D/closing-report.md and D/ledger.md, current assignment read-only.
No routine durable supporting artifacts expected; /tmp for transient logs. If a
specific durable failure receipt is valuable, place it in D/artifacts/ and name
reason/inventory (not another tree); no passing-log archive by default.

Before each commit inspect diff/status/staging. Stage **only named new files** and
use `git commit --only -- <explicit actual changed authorized paths>` for source;
list every path literally in the command, no git add . or directory-wide stage.
Separate planning commit explicitly names
`project01-mvp/arc12-live-rack-operations/slice05-instance-commands/closing-report.md`
and `project01-mvp/arc12-live-rack-operations/slice05-instance-commands/ledger.md`
(plus an explicitly produced D/artifacts receipt if necessary). Do not commit
other contributors' staged files. Expedited Mode authorizes these scoped commits.

Return prompt identity/SHA, source/planning commits and complete changed paths;
itemized intake/readback, applied Rust guidance, exact behavioral observations,
all attempted/unrun commands and counts, self-review/deviations, six-row walk,
artifact inventory and bubble-up. All IC rows remain **open**, with CC attested
proposed-done evidence until CRC independently accepts. Account separately for
Arc05 A-05-01 runtime/removal/handles/queued operations (and Arc07 transport/Arc13
typed vocabulary still retained) and A-05-02 inherited reconciliation. Arc12
R-01–R-04, projectP-01–P-06/release gates remain open. Do not claim Slice05, Arc12
or MVP acceptance. Preserve all issued prompts and prior CRC acceptance.
