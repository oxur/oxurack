# CC assignment — Arc12 Slice04 explicit rack lifetimes

2026-10-12. Sofie/CRC → Duncan → CC. Three contributors; Expedited Mode.
Initial assignment, correction0/5. Implement the approved product increment;
all six LF rows are open. Do not close the slice, arc or inherited ledgers.

## 1. Authority, roots and preflight

The source-of-record for lifecycle policy is [CDC directive12](../../cdc-directive12.md)
D1, installation recipe, D2 and D4. D3 describes successor05; only its explicit
low-level Panic/SetBpm honesty repair is in04. Directive08's playable-first,
proportionate-proof policy remains binding. This prompt resolves the engineering
mechanics; private factoring may differ, public behavior may not.

Use these exact roots; do not implement in main or in planning:

- `R=/Users/oubiwann/lab/oxur/oxurack` — main protected at
  `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
- `S=/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
  — branch `codex/oxurack-module-behavior-installation`, pinned source predecessor
  `deafdd228fb44720878c700f8ff62b2a59ddbad6`.
- `W=/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning` — branch `planning`.
  CDC return `67e9150dff5fd45df0f3882c3439ce4ff3e5a3dc` is the authoring ancestor;
  the CRC issuance commit adds this packet. Pin actual issuance HEAD at intake.
- `P=W/project01-mvp`; `A=P/arc12-live-rack-operations`;
  `D=A/slice04-runtime-api` — the sole new planning return home.
- `K=/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`;
  `F=K/collaboration-framework/knowledge` — installed guidance roots.

Before dependent edits: collect cwd, worktree list, branches, HEADs and status at
R/S/W; check source pin and planning ancestry/current assignment. Read the manifest
completely in bounded chunks, recovering truncated text. Record intake extents,
source citations and contract readback in D/closing-report.md. If coherent, proceed
under existing authorization. A scope conflict, unexpected source delta or missing
required input stops dependent work and returns the smallest concrete issue through
Duncan. Routine safe private mechanics are yours. No merge/push/release/endpoint.

## 2. Required-reading manifest and intake

Paths below expand against the roots above, not the terminal's implicit cwd.
Text scopes are normative: read their complete contiguous contents, not search
snippets. `full` means beginning through EOF. Named sections include their body
and subordinate headings, stopping at the next peer heading. Pinned source line
ranges apply to deafdd228; resolve by symbol if lines move after your own edit.
No required structured dataset; do not substitute a query receipt for reading.

| Order | Text / revision | Required extent | Purpose / deadline |
| --- | --- | --- | --- |
| 1 | W/AGENTS.md; S/CLAUDE.md, issuance/source pin | full both | Ownership, skills and worktree rules; before edits |
| 2 | D/cc-prompt.md, D/slice-plan.md, D/ledger.md, issuance | full all three | Current executable contract; before edits |
| 3 | P/cdc-directive08.md; P/cdc-directive12.md, CDC67e9150 | full both;12 D3 read as successor boundary, not assigned implementation | Current policy/design, including exceptions; before edits |
| 4 | P/project-plan.md, issuance; A/arc-plan.md; A/ledger.md | Project top/current direction and current Design handoff history through the new CRC acknowledgement; full arc plan/ledger | Current scope/parent gates; before edits; earlier dated project history is reference-only |
| 5 | P/arc05-public-api/arc-plan.md and ledger.md; P/crc-escalation12.md | full Arc05 pair; escalation Findings and Decisions requested sections | Original criteria, superseded transport, actual leak versus new success oracle; before tests/return walk |
| 6 | K/collaboration-framework/SKILL.md,2.6.0; F/engineering-methods/guides/01-engineering-methodology.md and07-implementation-prompt-authoring.md | full entrypoint/07;01 Notes for Codex, Implementation assignment authorship, Roles and Shared Invariants, Three-Contributor Workflow, Workflow Selection and Transitions, Codex Interpretation and Authority | Roles, intake and no invented handoffs; before edits |
| 7 | F/project-management/guides/README.md;01-scales-of-work.md;02-canonical-planning-worktree.md;03-planning-top-down.md;06-confirmation-protocol.md | full README/01/02/06;03 through end of Issuing and executing an iteration | Existing layout and Expedited Mode; before planning edits; no new confirmation for this authorized home |
| 8 | F/work-verification/guides/01-ledger-discipline.md;02-evidence-strength.md;03-row-closure.md;05-independent-verification.md | full | Attestation/acceptance and return; before edits/row walk |
| 9 | K/testing/SKILL.md; F/testing/guides/01-testing-discipline.md and03-validation-gates.md | full | Behavioral oracles/native gates/failure preservation; before tests |
| 10 | K/rust-guidelines/SKILL.md,2.0.0; guides/11-anti-patterns.md | full; load anti-pattern chapter before new Rust work | Safety net and per-topic routes; before edits |
| 11 | K/rust-guidelines/guides/{02-api-design.md,03-error-handling.md,05-type-design.md} | complete API-02/API-23, EH-01, TD-02/TD-03 entries | Borrowed input/owned output, concrete errors, recoverable versus panic, domain state; before design edits |
| 12 | K/rust-guidelines/guides/14-cli-tools/{02-argument-parsing.md,03-error-handling.md,04-output-and-ux.md} | complete CLI-05, CLI-55, CLI-27 entries respectively | Existing clap, ExitCode, trace/status stream separation; before example edits |
| 13 | S/crates/oxurack/src/lib.rs, deafdd228 | production1–535; test fixtures/sample/driver tests660–1640, load tests1660–1737 and MIDI tests1850–1995 | Actual owner/base/hook/schedules/admission and preservation fixtures; before edits |
| 14 | S/crates/oxurack/src/{persistence.rs,scope.rs,cables.rs,labels.rs}; tests/{persistence.rs,midi_output.rs} | full | Snapshot, capture and synchronous edit contracts; before edits/tests; cables/labels read-only |
| 15 | S/crates/oxurack-core/src/{module.rs,error.rs,event.rs} | full | Module ownership/hooks, typed errors, messages/stubs; before edits |
| 16 | S/crates/oxurack-core/src/patch.rs | validate_patch and apply_patch_to_world complete implementations and their docs | Preserve file validation; understand partial writes; before edits/tests |
| 17 | S/crates/oxurack-mod-midiout/src/lib.rs; S/crates/oxurack-midi/src/wire.rs | full both | Buffer/reservations/actual consume/bytes and existing tests; before edits; wire.rs read-only |
| 18 | S/crates/oxurack/examples/{save-live-patch.rs,rewire-live-patch.rs}; examples/patches/{turing-machine-midi.ron,turing-machine-lfo.ron}; S/README.md; S/Makefile | full examples/fixtures/Makefile; README live-save/annotation/rewiring and MIDI/Scope example sections | Executable recipe and repository-native entrypoints; before example/gates; fixtures unchanged |
| 19 | P/arc12-live-rack-operations/slice03-graph-composition/crc-verification.md | Candidate and authority; Iteration01 independent review through Findings resolved and final row walk | Prior provenance; no permission to retain affected lifecycle integration |

Use source queries to map every relevant caller/test of load_patch, start/stop,
try_install, buffer capacity/default/admission and Panic/SetBpm, including outside
the allowlist. Record query roots/patterns, complete match set reviewed and any
required outside-path repair. Inspect pinned Bevy0.18 World::try_query and mutable
resource borrowing API in local Cargo dependency source before adopting the
immutable collector sketch. This is source investigation, not a structured-data
manifest. Do not require the entire historical project/backlog or future05 code.

Before editing, write a source-cited readback of: fresh-App isolation versus core
partial mutation; stopped-first/drained checks that ignore the old fault; once
per-App ordered hooks; reset versus restart; dirty channel/order/timestamp/capacity;
new messages/Scope/driver lifetime; exact14-path boundary and05/07/13 exclusions.
Record any justified tactical divergence beside its governing rule. Receipts alone
are not comprehension. Budget roughly80–110k tokens for packet/guidance/source,
plus implementation/recovery headroom; chunk large files and checkpoint intact
contract/source/results in the same closing record if context needs continuation.
Do not trim required readings or tests to fit a turn. A stale skill version with
materially changed rules requires reconciliation before dependent edits.

## 3. Problem and exact public contract

At deafdd228 load_patch applies directly to the owned World, then replaces its
handle. MidiOutputBuffer keeps prior channel reservations; late spawners can
write before failing. The current handle-only replacement test does not prove
retirement. Repair lifetime ownership, not the standalone engine algorithms.

Add inherent public methods returning `Result<(), CoreError>`:

```rust
pub fn reset(&mut self) -> Result<(), CoreError>;       // append voice silence
pub fn restart(&mut self) -> Result<(), CoreError>;     // snapshot, rebuild at0
pub fn unload_patch(&mut self) -> Result<(), CoreError>; // fresh empty stopped owner
// Existing signature remains:
pub fn load_patch(&mut self, path: &std::path::Path) -> Result<(), CoreError>;
```

| Call | Admission and successful state | Failure / output |
| --- | --- | --- |
| start/stop | Existing execution toggle; preserve time, phase/RNG, knobs, topology, Scope, pending events/fault | No implicit silence or MIDI transport; retain accepted driver-switch behavior |
| reset | Any running/stopped/backpressured/faulted state; preserve all above including running/driver and first fault | Append complete D2 silence batch after pending ordinary output; no reseed/tick. Error clears/appends nothing |
| load_patch | Check stopped first, then no pending MIDI before file I/O; new isolated configured App, sample0, stopped | Old lifetime unchanged on ordinary Err. Carry only old cleanup; new fault clear; preserve installed recipes/driver |
| restart | Same admission before snapshot; rebuild current snapshot/master seed at0 stopped, never reread original file | NoPatchLoaded or unsupported getter/cable snapshot leaves old owner untouched. Old cleanup drain before new notes |
| unload_patch | Same admission; loaded lifetime becomes new empty stopped App at0 with installed recipes/driver | Cleanup drainable with zero new sinks; no handle/header/Scope; old fault cleared on successful retirement |
| repeated already-unloaded unload | Same stopped/drained admission, then no-op | Preserve any existing fault. Undrained cleanup rejects before no-op. A loaded empty Patch counts as loaded |

Busy errors are concrete **CoreError::LifecycleRequiresStopped** and
**CoreError::PendingMidi { count: usize }**, not TickError or a new latched fault.
Use **CoreError::CandidateMidiNotEmpty { count: usize }** for a spawner/hook that
produces candidate performance output before commit. Reuse existing snapshot,
I/O, validation and spawn errors; fallible buffer/cleanup capacity failures may
wrap existing TickError::OutputCapacity without latching/replacing an old fault.
Reject an already-faulted candidate before commit too; do not erase a candidate
fault to hide an invalid hook. Document error precedence and concrete error traits.

Success replays only installed-type recipes into the new App; it does not clone
arbitrary World state. Reset keeps Scope and bindings/records. Successful new
lifetime disables capture/old records; errors preserve them. Retain selected
SimulationDriver and rebase live clocks/discard overstep so paused wall time is
not caught up. Never call app.update or run a sample to construct/acknowledge a
lifetime. SampleTime/time resources are fresh0; candidate stays stopped.

## 4. Implementation recipe and worked shapes

All sketches here are **uncompiled design examples**, not claimed drop-in code.
Names marked NEW are proposed private/low-level seams; equivalent private names
are local discretion. Existing symbols were inspected at deafdd228. Compile and
verify the final code. This is a concrete small implementation, not a generic
transaction framework, scheduler, command service or World cloning layer.

### A. Ordered erased recipes and the base App

In lib.rs replace HashMap<ModuleKind, TypeId> values with ordered recipe lookup.
An ordered Vec plus kind→index map is sufficient. Record only successful public
installs; no recording during replay. Check same-type/collision before registry,
schedule or recipe mutation. Retain setter AND getter registration.

```rust
// NEW private entry. Vec order is the original successful install order.
struct InstalledType {
    kind: oxurack_core::ModuleKind,
    type_id: std::any::TypeId,
    install_into: fn(&mut bevy_app::App),
}

// NEW private monomorphized installer, shared by initial install and replay.
fn install_type<M: oxurack_core::OxurackModule>(app: &mut bevy_app::App) {
    use bevy_ecs::schedule::ScheduleLabel;
    let kind = oxurack_core::ModuleKind::from(M::KIND);
    {
        let world = app.world_mut();
        world.resource_mut::<oxurack_core::ModuleRegistry>().register::<M>();
        let mut parameters = world.resource_mut::<oxurack_core::ParameterRegistry>();
        for &(name, setter) in M::parameter_setters() {
            parameters.register(kind.clone(), name, setter);
        }
        for &(name, getter) in M::parameter_getters() {
            parameters.register_getter(kind.clone(), name, getter);
        }
    } // every registry/resource borrow ends before the user hook
    M::register_systems(app, bevy_app::FixedUpdate.intern());
}
// try_install checks kind/index/TypeId, invokes install_type::<M>, then records.
// candidate replay invokes only entry.install_into, in Vec order.
```

Factor current new() setup into NEW private `build_base_app() -> Result<App,CoreError>`:
TimePlugin, CorePlugin::fixed(), fallibly pre-reserved MIDI buffer, Scope observer,
optional RackGraphPlugin,1ms Fixed timestep, SimulationRunning(false), Virtual
paused, fixed overstep discarded. All resources needed by hooks exist first.
Keep `Oxurack::new() -> Self` unchanged: its infallible allocation boundary is
explicit in #Panics, with an invariant-specific diagnostic on reservation failure.
Fallible candidates propagate reservation errors. Ordinary Bevy/allocator panics
are not caught or advertised as rollback. Do not build an Oxurack recursively.

Compatibility: hooks run once per **App lifetime**, including failed candidate
Apps; same-type reinstall inside one lifetime remains idempotent. User hooks and
spawners own mutable effects inside their supplied App/World. Dropping resources
releases owned native resources; global singleton changes, non-owned handles and
arbitrary hook/spawner/Drop panics have no transaction guarantee. Document this
on try_install, load/restart and OxurackModule::register_systems. No speculative
module teardown trait or per-instance removal hook in04.

The existing private `stop_probe_rack` test helper in lib.rs adds StopControl,
StopTrace and four phase systems **before** load. A fresh App intentionally retires
that setup. Move those private probe resources/systems to after successful load
(or a fixture hook with per-App initialization) and before start; retain every
literal stop/sample/phase assertion. This is fixture adaptation to the explicit
lifetime, not permission to carry arbitrary old resources or weaken stop proof.
The installed module's own hook remains the recipe source. Review analogous
pre-load private test setup found by the required caller query.

### B. Candidate apply, preflight and commit in lifecycle.rs

Declare `mod lifecycle` in lib.rs. Child-module impl can access private owner
fields; no public ECS access is needed. Lifecycle admission reads running first
and buffer.pending_count (NEW low-level getter); it must not call check_admission
because the old terminal fault is intentionally recoverable through replacement.

```rust
// NEW private helper, assuming the new ordinary CoreError variants.
fn require_stopped_and_drained(&self) -> Result<(), oxurack_core::CoreError> {
    if self.app.world().resource::<oxurack_core::SimulationRunning>().0 {
        return Err(oxurack_core::CoreError::LifecycleRequiresStopped);
    }
    let count = self.app.world().resource::<oxurack_mod_midiout::MidiOutputBuffer>()
        .pending_count();
    if count != 0 { return Err(oxurack_core::CoreError::PendingMidi { count }); }
    Ok(()) // ignores, preserves old fault
}
```

Apply the existing loader only to `next`. Its resource_scope pattern is already
used in lib.rs and avoids borrowing ModuleRegistry and &mut World simultaneously:

```rust
// NEW private candidate helper; existing core symbols and PatchSource variants.
fn apply_in_candidate(
    next: &mut bevy_app::App,
    patch: &oxurack_core::Patch,
    source: oxurack_core::PatchSource,
) -> Result<(), oxurack_core::CoreError> {
    next.world_mut().resource_scope::<oxurack_core::ModuleRegistry, _>(|world, registry| {
        let handle = oxurack_core::apply_patch_to_world(patch, &registry, world, source)?;
        world.insert_resource(handle);
        Ok(())
    })
}
```

Full replacement flow, including every fallible seam:

```rust
// NEW private impl helper; helper names below are not pre-existing APIs.
fn replace_configuration(&mut self, patch: &Patch, source: oxurack_core::PatchSource)
    -> Result<(), CoreError>
{
    self.require_stopped_and_drained()?;
    let identity = persistence::LoadedIdentity::from_patch(patch);
    let mut next = build_base_app()?;
    for recipe in &self.install_order { (recipe.install_into)(&mut next); }
    apply_in_candidate(&mut next, patch, source)?;
    // Check next.pending_count()==0 and next.fault()==None; otherwise return Err.
    // GraphMaintenance runs here, only when graph enabled; never a sample.
    // Rebase live clocks/discard overstep, stopped; no capture is configured.
    let cleanup = oxurack_mod_midiout::prepare_silence(self.app.world())?; // NEW/read-only
    next.world_mut().resource_mut::<MidiOutputBuffer>().append_silence(&cleanup)?; // NEW
    // All ordinary failure is now behind us. No fallible call after swapping.
    let old = std::mem::replace(&mut self.app, next);
    self.loaded_identity = Some(identity);
    drop(old); // old reservations/resources disappear; do not clear old state earlier
    Ok(())
}
```

The complete implementation fills the comments with real checks/clock maintenance;
use CoreError's existing From<TickError>. `load_patch` calls admission **before**
load_patch_from_file and the helper. `restart` checks before snapshot_patch and
uses PatchSource::InMemory, never a fake File path. `unload_patch` checks before
no-op/build; for a loaded lifetime builds the same base+ordered recipes without
apply/handle/header, verifies empty candidate output/fault, maintains optional
graph, prepares/attaches old cleanup, then commits loaded_identity=None. Do not
swap/drop old before any buffer reservation, snapshot, spawn or cleanup error.

Core apply_patch_to_world stays explicitly nontransactional; change its docs,
not its production body or validation rules. Fresh candidate load notifications
remain in the new World, PatchLoaded last. Failed candidate messages never enter
the old World. Do not invent old ModuleRemoved/CableRemoved notifications in the
new World: Entity values can recur across Worlds. Document reader/GraphView
reacquisition and lifetime invalidation. Granular instance notifications belong05.

### C. Whole MIDI silence batch, room check, then commit

In oxurack-mod-midiout add dirty state to each sink. Set dirty only when normal
messages actually commit; any NoteOff or CC makes dirty, even without a held note.
Active note implies dirty. Keep existing strict Value parsing, five-message worst
case and sample admission/fault semantics; no new per-sample drain/allocation.

Keep normal logical_capacity = sink_count*5, with at most16 unique channels.
Separately pre-reserve **128 event slots** at construction, even without sinks:
80 normal plus48 cleanup. `try_new() -> Result<MidiOutputBuffer, TickError>` (NEW)
may use Vec::try_reserve_exact. Manual Default supports existing raw-world
init_resource callers with documented infallible allocation behavior. Reserve_sink
checks count/channel before mutation and updates accounting without shrinking
storage. Do not introduce unused instance release hooks; App drop retires04 sinks,
05 will release selected sink reservations after cleanup. Enforce logical bounds
independently of allocator capacity (which may exceed128).

Recommend an opaque owned low-level `SilenceBatch` with private fixed staging
slots `[Option<MidiOutputEvent>;48]`, length, and selected old sink identities for
in-place reset. Public Debug plus read-only events accessor is sufficient if
facade/tests need it; do not let callers forge entries or targets. Its producer
uses `&World`, returns owned data, no retained borrow. Cross-crate helpers live in
MIDI-out, without new umbrella reexports/World-dependent facade methods.

```rust
// NEW low-level outline inside MIDI-out, where MidiOutState is private.
// Pinned Bevy0.18 provides World::try_query(&self); verify its exact signature.
fn collect_dirty(world: &bevy_ecs::world::World) -> Result<SilenceBatch, TickError> {
    let boundary = world.resource::<oxurack_core::SampleTime>().completed_samples();
    let mut by_channel = [None; 16]; // owned (Entity, active_note), not State borrows
    if let Some(mut sinks) = world.try_query::<(bevy_ecs::entity::Entity, &MidiOutState)>() {
        for (entity, state) in sinks.iter(world) {
            // Validate channel uniqueness/accounting and active=>dirty first.
            // Store dirty entries by config.channel, independent of name/Entity.
            if state.dirty { by_channel[state.config.channel as usize] = Some((entity, state.active_note)); }
        }
    } // no registered sink component is legitimate only when buffer accounting is empty
    let mut batch = SilenceBatch::empty(); // NEW, fixed storage; checked append helper
    for (channel, entry) in by_channel.into_iter().enumerate() {
        if let Some((entity, active)) = entry {
            if let Some(note) = active {
                batch.push(boundary, oxurack_midi::MidiWire::note_off(channel as u8, note, 0))?;
            }
            batch.push(boundary, oxurack_midi::MidiWire::cc(channel as u8, 123, 0))?;
            batch.push(boundary, oxurack_midi::MidiWire::cc(channel as u8, 120, 0))?;
            batch.remember_sink(entity)?;
        }
    }
    Ok(batch)
}
```

Provide NEW low-level `prepare_silence(&World) -> Result<SilenceBatch,TickError>`
using that collection/validation; NEW buffer `append_silence(&SilenceBatch)` checks
full batch fits total128 before append, ignoring old fault but not capacity.
NEW `silence_outputs(&mut World) -> Result<(),TickError>` implements reset:
prepare/read-only → validate all targets and total room → append complete batch →
clear selected active notes/dirty flags. No returned recoverable error after the
first append or clear. Separate World/resource borrows into short blocks; use
safe queries, not unsafe aliases or allocation to escape borrowing.

For replacement/unload, prepare against **old immutable World**, attach events to
candidate buffer only; old target Entity values must never be used to clear new
sink state. The old owner is dropped only on success. For reset, append to current
buffer, then clear its selected state. A prepare/room/invariant error returns
without latching over the first fault or changing pending events/voice state.
No test-only branch in production. Test capacity errors by corrupting private
state/limits inline; test normal supported capacity with real sink output.

Boundary sample is completed_samples=n, no tick/increment. Pending normaln-1
comes first, cleanupn next. After replacement cleanup carries old n even though
new SampleTime=0; it must be drained before new execution. Document this in
MidiOutputEvent and facade drain/lifecycle docs; timestamps are lifetime-local.
A second reset before new output adds nothing. Empty reset succeeds with no MIDI.
Normal next-sample admission still requires an empty buffer;128 storage is not
permission to accumulate multiple samples. Keep existing sender fallback unchanged.

### D. Honest stubs and guidance interaction

Core dispatch's current `Panic | SetBpm(_) => Ok(())` is a misleading success.
Return existing `CoreError::NotImplemented("Panic")` and `NotImplemented("SetBpm")`
respectively; update its two no-op tests/docs. CoreCommand::Panic documentation
means narrow voice silence; facade command execution/cancellation is05. Do not
add an execute/enqueue/queue/bpm/handle/removal API or transport wire support here.

| Guidance actually applied by CRC | Strength / concrete consequence |
| --- | --- |
| Rust API-02 | SHOULD: borrow Path/Patch/World for reads; return owned snapshot/batch with no World borrow |
| API-23, EH-01 | MUST: concrete CoreError busy/candidate errors, ordinary capacity results before mutation; unsupported commands do not claim success. Arbitrary user panic is not a recoverable transaction |
| TD-02/03, AP-06 | SHOULD / AVOID: named recipe/batch state with private invariants; distinguish ordinary capacity from cleanup storage |
| AP-08/09/42 | AVOID: reserve storage before music, short borrowing, no cleanup hot-path allocation/chained unwrap; infallible new allocation boundary explicitly authorized by CDC |
| AP-43/55 | CONSIDER: one erased fn installer and fixed bounded staging, no generic rollback framework |
| AP-44 | AVOID: simple inherent facade/CoreError signatures, no extra public World/Entity escape for tests. Existing module-author low-level World seam remains confined to MIDI-out |
| CLI-05/27/55 | MUST/MUST/SHOULD: extend clap derive, MIDI trace stdout/status stderr for lifecycle mode, return ExitCode with normal drops |

Additional relevant Rust routes from the entrypoint apply as needed (new helper
idioms, ownership, docs). Do not blindly create a crate/lints/dependency because a
skill's generic “new project” example suggests it. Binding project/CDC scope wins.

## 5. Discriminating tests: required observations, not names alone

Put owner inspection tests inline in lib.rs or lifecycle.rs; those can access
private app/world. Low-level dirty/room/buffer tests go inline in MIDI-out. Public
integration tests use inherent APIs/Scope/snapshots and may call into_app **only
after the last facade use**. No public test-only World getter. Build expected
values from literal contract/independent fresh configuration, not the same helper
being tested. Give each LF row specific test/path/assertion receipts.

### LF-01 / LF-02: real retirement and late failures

- Load the real Clock/LFO/TM/MIDI fixture stopped, advance/drain and stop; replace
  with the same channel patch at least twice, draining cleanup between lifetimes.
  Count **all** core Module/Port/Cable/ChildOf components, exact names/kinds,
  PatchHandle membership, CableIndex and optional graph module/port/edge membership
  against a separately fresh rack with the intended patch. Old entity components
  and channel reservations may not accumulate. Do not compare Entity numbers
  across Worlds as an identity oracle; use structural projections and counts.
- Use two-sink valid channel[1,0] candidate: it must now succeed; prior live-owner
  failure was the leak, not desired validation. Reject[1,1] in a fresh candidate
  and prove previous snapshot/time/Scope/values/output/fault/messages unchanged.
- Add a small valid-schema custom module with a late fallible spawner: after a
  real MIDI sink spawns, write a marker resource, marker entity and existing core
  message, then return a deliberate CoreError. Ensure candidate passes file
  validation and reaches this write/error. Inspect no marker or message delta in
  old World, unchanged old PatchHandle/header/registries/index/graph/Scope/time.
  Continue old real LFO/TM with a matched unfailed control and compare musical
  events and Scope through a future edge. Then successfully load channel1: no
  failed candidate reservation survived. Temporary RON only, no fixture edits.
- Two distinct hook fixtures append ordered values to a per-App resource; call
  same-type reinstall and kind collision and assert no current-App hook/schedule/
  registry/recipe mutation. Successful load/restart/unload/reload each see original
  successful order exactly once per App; actual systems run once per tick and
  setters/getters work. A failing candidate still runs its installation hooks;
  count it separately with test-only serialized observation if needed. Do not
  claim external observation rolls back. Keep ordinary provided module hooks
  confined to supplied-world owned state, and verify abandoned owned marker drop
  where practical. Old scheduled system counters continue against control on Err.

### LF-03: exact silence and capacity

One real raw MIDI sink can use the existing inline fixture in lib.rs: note60,
velocity100, step=true, gate=true, sample0. After one completed sample unread
normal is `0|90 3C 64`. `reset()` appends exactly:

```text
0|90 3C 64
1|80 3C 00
1|B0 7B 00
1|B0 78 00
```

Assert time still1; selected driver/running/Scope/snapshot unchanged; reset
again before drain adds nothing; old first fault unchanged if latched. Draining
gets the whole vector; stop/start does not itself produce cleanup. Later actual
output can retrigger without an obsolete active-note duplicate. Test these cases:

- Dirty channels chosen in a different name/Entity order emit cleanup in numeric
  channel order; NoteOff then123 then120 per dirty channel, velocity/value0.
- Never-used sink: none. CC-only sink:123/120 even with no active note. A note
  released earlier remains dirty:123/120 still emitted to silence sustained tails.
- 16 real sinks, unique channels0–15, three configured CC outputs: first produce
  held notes and drain. Next retrigger with all three CCs: **80 normal** messages
  (old off/new on/three CC per sink). With that output unread, reset appends
  **48 cleanup**, total**128**, boundary2 versus normal1. Assert entire ordinary
  prefix intact, ordered cleanup bytes, no fault, time2, backing capacity unchanged
  during append, second reset still128. Reused storage does not allow a129th slot
  or the next sample with pending128. Keep5-per-sink normal admission/consume tests.
- Fault then reset: preserve first cause/time/stopped state and earlier messages;
  clean batch still visible. Controlled private prepare/invariant/room failure
  keeps dirty flags/active notes and pending vector byte-for-byte; retry after
  fixing the injected defect emits original batch. Do not fabricate a production
  recovery branch solely for the fixture.
- unload after ordinary drain copies old cleanup into an empty zero-sink buffer;
  normal logical capacity0 yet cleanup drain succeeds. Repeated unload before
  draining returns PendingMidi; after drain a true empty no-op preserves an
  injected preexisting fault. Empty reset appends nothing.

### LF-04 / LF-05: admission, continuation, restart and events

- For all three lifetime operations, running rejection comes before missing file,
  NoPatchLoaded/snapshot or pending-output checks. Stopped unread output returns
  PendingMidi with exact count; errors mutate no clock/Scope/voice/header/fault/
  messages/recipes. Busy calls do not run hooks or perform file reads. Test
  initial load too; reset bypasses these busy/fault checks as specified.
- Start/stop continuation: real Clock/LFO/TM+Scope subject stops and resumes while
  control remains logically continuous; per-sample drained trace and capture
  equal. Reset has different MIDI silence but identical engine phase/time/Scope
  continuation; compare scalar Scope values/later musical production appropriately,
  excluding the deliberate cleanup/active-note suppression difference.
- Edit current LFO frequency1, configured TM write base.25, cable constant+.25 and
  its description, plus current seed/header retained; restart must use that
  snapshot. Alter/delete the original file after load to discriminate from
  rereading. Drain old cleanup, assert stopped/time0/no Scope/fault, configure
  Scope and start; compare10000 per-sample MIDI events and scalar capture with a
  fresh rack built from independently checked current configuration. Do not
  compare to the pre-restart continuation or snapshot engine state.
- Missing registered getter/disabled selected cable causes UnsupportedSnapshot on
  restart. Inject only inline private state needed by existing snapshot rules;
  assert old owner preserved. No schema or getter default workaround.
- Latch a real existing backpressure/value terminal fault. Stop/drain, fail a
  candidate and observe same cause/state; then successfully load/restart/nonempty
  unload and observe clear fault, preserved old cleanup drain and subsequent valid
  fresh sample admission (load again after empty unload). Replacement with unread
  output must not clear old fault. No sample executed while acknowledging any call.
- Selected Manual and Live drivers survive success. With controlled existing
  TimeUpdateStrategy/manual-duration fixtures, a long stopped interval must not
  create a catch-up burst after restart/load/start. Verify sample0/fresh one-tick
  grouping and overstep/clock behavior, not sleep/wall-clock timing assertions.
- Candidate with actual normal MIDI pending from a test hook/spawner rejects
  CandidateMidiNotEmpty before commit. Candidate failure does not clear old Scope;
  successful lifetime disables it and its records. Fresh candidate ModuleAdded/
  ParameterChanged/LabelChanged/CableAdded/PatchLoaded contain actual new members
  and source; PatchLoaded last. Failed candidate causes no live message delta.
  Whole-App replacement/unload does not synthesize retired Entity removal messages.
- Core dispatcher returns exact NotImplemented variants for Panic/SetBpm, no
  tick/state/success-message mutation. Document narrow Panic/reset without claiming
  05 facade command implementation. Keep all other command behavior unchanged.

### LF-06: executable musical use and inherited walk

Extend existing `examples/save-live-patch.rs` with a clap-derived `lifecycle: bool` field
exposed by `#[arg(long)]` as `--lifecycle`. Default/no-flag keeps existing200→edit LFO1
→save→100 behavior and300-sample summary. Lifecycle flag follows this fixed recipe:

1. Install real Clock/LFO/TM/MIDI, load supplied LFO fixture, Manual, start and
   advance200 with the existing per-sample caller drain.
2. Set LFO frequency1 and TM write_probability.25. Unpatch occurrence0 of
   lfo_main.bipolar_out→tm_main.write_modulation; add Affine{factor:0,offset:.25}.
   Label its returned occurrence description “Constant +0.25 write modulation”.
   Save to --save, assert current snapshot includes these literal values/label.
3. Continue100, stop/drain ordinary output, unload, drain **all** cleanup through
   the existing owner (report it, not a hidden delivery service). Assert empty/
   stopped0/no patch/no Scope; reload --save, verify same saved configuration at0.
4. Configure Scope for lfo_main.bipolar_out and tm_main.write_probability,
   capacity21000 (20000 scalar records plus room for this fixture’s copied MIDI;
   fail visibly if overwrites occur). Start/replay10000 samples, advance1/drain each
   time. After each drain borrow ends, copy its events with record_scope_midi and
   MidiDelivery::Generated; this is observation, never a second drain/send owner.
   Print only replay MIDI `sample|XX XX XX` rows to stdout exactly like the existing
   turing-machine-midi formatter. Put stage/save/sample/cleanup/capture diagnostics
   on stderr in lifecycle mode; no default summary mixed into this trace.
5. At end stop, inspect Scope records0..9999:10000 observations per selected port,
   correct literal .5 effective probability after the delayed publication settles,
   LFO values/phase equal fresh configured reference in tests, zero overwrites.
   Stop alone deliberately does not silence; finish with reset and drain/report
   final silence to stderr, excluded from replay stdout. No endpoint operation.

The actual TM port is named `write_probability`, not `write_probability_out`.
The explicit Scope copy seam is already public; a reusable example-only scratch
Vec can end the drain borrow before calls back into rack:

```rust
// Uncompiled example fragment; initialize scratch once outside the loop.
let mut drained = Vec::with_capacity(128);
for _ in 0..10000 {
    rack.advance_samples(1)?;
    drained.clear();
    drained.extend(rack.drain_midi());
    for &event in &drained {
        let [status, data1, data2] = event.message.to_bytes();
        // Use a fallible Write sink in final code for redirectable trace output.
        println!("{}|{status:02X} {data1:02X} {data2:02X}", event.sample);
        rack.record_scope_midi(event, oxurack::MidiDelivery::Generated);
    }
}
```

Use the current rewire example's actual label/patch signatures and existing
Scope record/selector APIs. Keep helper functions testable; use std::io::Write
if convenient for fallible trace output, existing ExitCode on errors. No dependency
needed for this example. Tests execute the actual recipe/argument mode, not a
parallel imitation that could diverge; retain default mode and missing/malformed
patch/save-path failure cases. Public integration independently compares actual
edited values, topology/label, capture and seeded reference. Clean final output
makes CLI replay directly comparable with a fresh existing runner on the saved
patch. Do not pin a guessed new musical hash; compute/record fresh equality only
after literal configuration and independent module/control assertions.

Return walks every LF row and maps Arc05 A-05-01/02: cable/parameter/metadata work
accepted in01–03; lifecycle supported here only if observed; handles/removal/
sync+queued load/save/results/globalBPM remain05, transport separate07 and catalog
vocabulary13. Both historical rows remain deferred, parent R-01–R-04 open. No
inherited child/ancestor is marked done merely because this software works.

## 6. Exact source and planning scope

Source edits are allowed only in these **14 paths**, with roles:

1. `README.md` — lifecycle recipe/API/compatibility guidance.
2. `crates/oxurack/src/lib.rs` — recipes/base factor/export wiring/docs/inline tests.
3. `crates/oxurack/src/lifecycle.rs` — NEW lifecycle impl/private tests.
4. `crates/oxurack/src/persistence.rs` — necessary snapshot/lifetime integration only;
   preserve configured getters/cable order/file schema.
5. `crates/oxurack/src/scope.rs` — necessary base/capture lifetime integration only;
   preserve bounded observer/CSV behavior.
6. `crates/oxurack/tests/lifecycle.rs` — NEW public real-module composition tests.
7. `crates/oxurack/tests/persistence.rs` — affected save/restart/replay callers/tests.
8. `crates/oxurack/tests/midi_output.rs` — affected lifetime/admission composition.
9. `crates/oxurack/examples/save-live-patch.rs` — existing executable plus --lifecycle.
10. `crates/oxurack-core/src/module.rs` — supplied-world/per-App hook contract docs.
11. `crates/oxurack-core/src/event.rs` — Panic/SetBpm honest errors/docs/tests only.
12. `crates/oxurack-core/src/error.rs` — named lifecycle errors/traits/tests.
13. `crates/oxurack-core/src/patch.rs` — **loader documentation only**; its production
    implementations and validation policy are protected, no transaction rewrite.
14. `crates/oxurack-mod-midiout/src/lib.rs` — buffer construction/dirty/cleanup/helpers
    and inline boundary/room/peak tests.

Only edit a listed path when needed; do not touch every path for compliance.
All other source paths are read-only, including graph production, labels/cables
helpers, standalone engines, native RT/MIDI wire, manifests/lockfile, original
fixtures, runner endpoint fallback and accepted packets. If an outside-path caller
actually requires repair, return exact path/reason before that edit; no inferred
allowlist. No new crate/dependency/schema/benchmark/verifier or future05 files.

Planning edits only D/closing-report.md (NEW), D/ledger.md (CC proposed-done notes,
Status remains open). Supporting artifacts none expected; explain named necessary
exceptions below D/artifacts/, no automatic intake/matrix/log file forest.
Do not modify issued prompt/slice-plan, parents/status, other reports/directives,
AGENTS or ancestor ledgers. Preserve original failed attempts and retries.

## 7. Required fresh gates and runnable commands

Run from S, on final source. Focused failures first: repair the cause, not the
assertion/ignore/lint. Existing three RT ignores remain reported, not expanded.
The two full facade package modes include all existing clock, sample-pipeline,
seeded-spawn and installation integration tests as well as the new lifecycle and
example tests; this App boundary affects their setup too. No new coverage/outdated/
dependency campaign just for this slice.

```bash
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack-mod-midiout
cargo test --locked -p oxurack --all-targets
cargo test --locked -p oxurack --no-default-features --all-targets
make test
make lint
cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
cargo build --locked -p oxurack --example save-live-patch --example turing-machine-midi
cargo build --locked -p oxurack --no-default-features --example save-live-patch
git diff --check
```

If restricted make test hits known CoreMIDI initialization, retain the failed
command/exit/tests and retry **unchanged** with host access, recording context and
revision. Do not suppress failures/advisories or claim a restricted pass from a
host retry. Keep block0.1.6 future-incompatibility advisory visible. Lifecycle
changes affect facade integration: rerun required suites and demonstrations;
prior CRC receipts are context only, not substitutes for these commands. No graph
production changes expected; graph feature behavior is exercised in facade tests
and full workspace make test. If your change reveals a graph defect, report it
rather than edit outside scope or weaken exact membership proof.

Run these real CLI demonstrations and add the same usable recipe to README:

```bash
OXURACK_LIFECYCLE_DIR=$(mktemp -d /tmp/oxurack-lifecycle.XXXXXX)
cargo run --locked -p oxurack --example save-live-patch -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --save "$OXURACK_LIFECYCLE_DIR/saved-default.ron" --lifecycle \
  > "$OXURACK_LIFECYCLE_DIR/lifecycle-default.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch "$OXURACK_LIFECYCLE_DIR/saved-default.ron" --samples 10000 \
  > "$OXURACK_LIFECYCLE_DIR/fresh-reference.trace"
cmp "$OXURACK_LIFECYCLE_DIR/lifecycle-default.trace" "$OXURACK_LIFECYCLE_DIR/fresh-reference.trace"
cargo run --locked -p oxurack --no-default-features --example save-live-patch -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --save "$OXURACK_LIFECYCLE_DIR/saved-no-graph.ron" --lifecycle \
  > "$OXURACK_LIFECYCLE_DIR/lifecycle-no-graph.trace"
cmp "$OXURACK_LIFECYCLE_DIR/lifecycle-default.trace" "$OXURACK_LIFECYCLE_DIR/lifecycle-no-graph.trace"
cargo run --locked -p oxurack --example save-live-patch -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --save "$OXURACK_LIFECYCLE_DIR/saved-ordinary.ron"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron --samples 2000 \
  > "$OXURACK_LIFECYCLE_DIR/original-midi.trace"
cargo run --locked -p oxurack --example turing-machine-midi -- \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --samples 10000 \
  > "$OXURACK_LIFECYCLE_DIR/original-lfo.trace"
shasum -a 256 "$OXURACK_LIFECYCLE_DIR/original-midi.trace" "$OXURACK_LIFECYCLE_DIR/original-lfo.trace"
```

Expected unchanged originals: MIDI13 rows
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`;
LFO70 rows `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Observe row counts, compare outputs and literal edited configuration/Scope; a
hash alone does not prove lifecycle. Missing/malformed patch or unusable save
path exits nonzero with useful stderr; include actual failures/CLI tests.
No --output-name/index or connection/listing for this software-only assignment.

## 8. Return, commits and stop conditions

Self-review every LF row against the complete contract and AP chapter; inspect
all diff paths, typed failure behavior, capacity/order and owned-drop boundary.
Do not change an oracle to accept a reproduced leak, assert only handle counts,
claim rollback of globals/panics or discard unread cleanup. A consequential
contradiction goes through Duncan to CRC/CDC before dependent implementation.

Closing-report.md contains: intake/full read extents/query coverage/readback;
actual source decisions and Rust rule applications; each LF row's concrete
paths/tests/assertions and proposed disposition; all required command exits,
feature mode/revision, failures/retries/ignored denominator, fresh musical and
Scope evidence; file-boundary review, inherited-owner walk, limits/bubble-up.
Report any native failure accurately. Include full source commit(s), prompt hash,
source/plan HEAD and clean status. CC proposed-done is attested, never CRC done.

Commit source only named changed allowlist paths on assigned source branch,
then separately commit D/closing-report.md and D/ledger.md in planning. Prefer
`git commit --only` with explicit paths, never indiscriminate staging. Preserve
required repository commit conventions/trailers, if present. No recursive staging
or amendment of issued historical packets. Update planning provenance in a
separate named-path follow-up only if needed; never invent its own hash.

Return to Duncan the source/planning commits, canonical report path, concrete
outcome, required gate results/limits, all six open proposed-done LF rows and
both worktree statuses. CRC independently reproduces before closure or Slice05.
Arc12 R-01–R-04, historical A-05-01/02 and project/release gates remain as assigned.
