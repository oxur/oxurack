# CDC directive12 — usable, explicit rack lifetimes

2026-10-12. Sofie/CDC → Duncan → CRC. Three contributors; Expedited Mode.
Answers [crc-escalation12](crc-escalation12.md). Directive08's playable-first
policy and framework2.6.0 remain binding. This is a design return, not a CC
assignment, source implementation, or acceptance of Arc12.

Baseline: source `deafdd228fb44720878c700f8ff62b2a59ddbad6`, branch
`codex/oxurack-module-behavior-installation`; planning
`cd5add4b3d3a66b0cc6d1e918ea8a31218b0400a`; both clean at intake. Main remains
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Project v3.37 / Arc12 v1.6 become
v3.38 / v1.7. Slice03's five accepted G rows and correction1/5 stay closed.
R-01–R-04 remain open. No earlier prompt is amended.

## Decisions and delivery

D1–D4 are resolved below. CRC may author **Slice04**, without another exploratory
assignment. Split the unissued final increment into two software deliveries:

| Owner | Delivered behavior |
| --- | --- |
| `arc12-live-rack-operations/slice04-runtime-api` | Failure-preserving patch replacement, unload, narrow reset, explicit seeded restart, MIDI cleanup/fault recovery, and a runnable edit/save/unload/reload session |
| `arc12-live-rack-operations/slice05-instance-commands` | Instance removal and stale-safe references/handles, synchronous command execution, bounded queued load/save with results, global BPM configuration, and an interactive-style executable example |

This is a CDC structural amendment under Duncan's existing authorization to
replan for solid engineering and useful increments. It preserves all inherited
requirements; the few future interface/catalog forms are explicitly routed below.
Slice05 opens after CRC accepts04; CRC authors both within this contract. Arc12
closes only after both and the inherited/composition walk. Do not create empty
Slice05 artifacts now. The split separates owner replacement from instance edits
and command ergonomics; neither slice is a planning/tooling-only deliverable.

## D1 — lifecycle semantics (binding)

**Retain current `start`/`stop`.** They toggle execution and preserve sample time,
engine/RNG/phase, knobs, topology, Scope, pending output and output fault. Stop does
not emit MIDI or silence held notes. Driver switching keeps its accepted behavior.
No implicit transport wire messages or second transport counter are introduced.

`reset(&mut self) -> Result<(), CoreError>` is **voice silence**, not reseeding.
It may run while running, stopped, backpressured or terminally faulted. It keeps
SampleTime, running/driver state, engine state, parameters, topology, metadata,
Scope and the first fault. It emits cleanup under D2 and, once05 exists, cancels
pending commands with observable cancelled results. Failure leaves all those
states and the pending commands untouched. `CoreCommand::Panic` means this same
operation; supersede its misleading “reset all modules” documentation.

`restart(&mut self) -> Result<(), CoreError>` reconstructs the **current snapshot**
with its current seed and configuration at sample0, **remaining stopped**. It does
not reread the originally loaded file or restore a performance checkpoint.
The caller drains any cleanup, optionally configures Scope, then calls `start`.
`unload_patch(&mut self) -> Result<(), CoreError>` leaves an empty, stopped rack
with installed types available for another load. Repeated empty unload is a no-op
subject to the same admission rule; it must not erase undrained cleanup/faults.
Existing `load_patch(&Path) -> Result<(), CoreError>` becomes initial load or
replacement using the same candidate path. All three require stopped + no pending
MIDI, including initial load. They may recover a terminal fault **after its events
are drained**. Do not reuse `check_admission` for this: that method rejects faults.
Check running first, then pending count, without changing the old fault.
These admission checks precede file I/O, snapshotting and candidate construction.

| Operation | Time/engines and execution on success | Membership, graph, Scope, installation | Output/fault | Failure |
| --- | --- | --- | --- | --- |
| Load / replace | New seed-derived engines, sample0, stopped; retain selected driver, rebase live clocks/discard overstep | New App lifetime, exact new membership/metadata/index/graph; disable old Scope including records; replay installed type recipes | Carry only prepared old cleanup through the normal drain; new fault is clear | Entire old lifetime intact, including stopped continuation, Scope, output/fault, registries and messages |
| Restart | Same as replacement using live snapshot | Same configuration; fresh lifetime and invalidated old bound handles; Scope disabled | Same as replacement | Snapshot or construction/admission failure leaves old lifetime intact |
| Unload | Sample0, stopped; driver retained | Fresh empty App with type recipes; no PatchHandle/header/Scope; handles invalidated | Old cleanup remains drainable even with zero new sinks; clear old fault on successful nonempty unload | Old lifetime intact |
| Reset | Time/engines/running/driver unchanged | Configuration, bindings/Scope records and install hooks unchanged | Append silence once per dirty channel; preserve unread events and first fault | No partial silence, state clearing or cancellation |
| Instance removal (05) | Surviving engines/time/driver unchanged; requires stopped + drained | Retire incident cables/children/module and update handle/index/graph immediately; preserve rack labels/header and survivors; invalidate only removed instance's bindings; disable Scope only if a selector targets it | Prepare selected sink cleanup, then release its reservation; preserve existing fault | Validate name/ownership/admission before mutation; old state intact on recoverable errors |

No implicit Scope archive: export or otherwise inspect capture before a successful
new lifetime. That follows existing load's disable behavior; errors now preserve it.
Reset and stop/start preserve it. Scope never becomes another MIDI drain owner.
Reset on an empty rack succeeds without MIDI (and cancels pending commands in05).
Restart without a loaded patch returns existing NoPatchLoaded without mutation.
An empty patch still counts as a loaded lifetime for unload/restart purposes.

The historical Arc05 memo's start/stop-reset-position contract is **superseded**
for these facade verbs. Its separate power versus MIDI transport and START/STOP/
CONTINUE wire support remains **deferred to Arc07**, re-entering before a transport
UI or external master/slave controller is advertised. There use a separately
named transport API; do not retroactively change start/stop. Pause/resume and
power aliases are not added here merely to imitate that old vocabulary. Narrow
reset is retained with fallible, observable delivery semantics. No module phase
reset hooks or fake MIDI transport implementation are required in04.

## D1 — isolated candidate and installation recipe (binding shape)

Build candidates in a **new App**, not the live World and not a cloned arbitrary
World. Entity-only rollback cannot undo custom spawners' resource/message writes.
Retain the existing core loader as a low-level, explicitly nontransactional helper;
the facade provides the stronger failure guarantee by isolation.

Replace the install map's bare TypeId value with an entry containing kind, TypeId
and an erased `fn(&mut App)` installer, plus an ordered Vec of successful entries
(or an equivalent ordered store). Preserve original successful installation order;
never replay by HashMap order or TypeId sort. The monomorphized installer wires
ModuleRegistry, setters/getters and `M::register_systems` using FixedUpdate. It
must not recursively record another recipe while replaying. Same-type reinstall
in the **current App** remains idempotent; conflicting kind still fails before
mutation. New public installs affect the current lifetime and future candidates.

**Explicit compatibility amendment:** registration hooks run once per App
lifetime, including failed candidate lifetimes, rather than once per facade
object forever. No old schedule is rerun or mutated during candidate construction.
Document this on `try_install`, `OxurackModule::register_systems`, and load/restart.
Hooks/spawners must confine owned mutable effects to their supplied App/World;
owned resource Drop implementations release native resources on candidate drop.
External singleton changes, global side effects, arbitrary hook/spawner/Drop
panics and non-owned handles are not transactionally rolled back. Do not catch
panics and present them as recoverable load errors. The provided module types
must satisfy the supported boundary. `into_app` consumes the facade, so no promise
to reconstruct arbitrary post-conversion user plugins/resources is needed.

Construction and commit sketch (design pseudocode, **not compiled code**):

```rust
// Registry access borrows end before hooks; installation order is stable.
struct InstalledType {
    type_id: TypeId,
    install_into: fn(&mut App),
    // kind retained in ordered record / lookup map
}

fn replace_configuration(&mut self, patch: &Patch, source: PatchSource)
    -> Result<(), CoreError>
{
    self.require_stopped_and_drained()?; // does not reject or clear old fault
    let mut next = build_base_app()?;    // reserves bounded MIDI storage
    for recipe in &self.install_order {
        (recipe.install_into)(&mut next);
    }
    apply_in_candidate(&mut next, patch, source)?; // existing validation policy
    maintain_candidate_graph(&mut next);          // no tick / app.update()
    let cleanup = prepare_old_silence(self.app.world())?; // read-only
    admit_cleanup(&mut next, &cleanup)?;          // all fallible work before commit
    // fresh lifetime token; new header; selected driver retained; next stays stopped
    self.commit_candidate(next, patch);
    Ok(())
}
```

Fresh base App includes existing time/core/Scope observer/optional graph setup.
Factor `new` privately instead of building an Oxurack inside every helper.
Allocation failures under explicitly fallible buffer reservation return typed
errors; ordinary Bevy/allocator panics retain normal Rust behavior. At commit
there is no expected recoverable failure. Drop the old App after its cleanup is
copied; do not transfer its reservations, fault, entities, time or arbitrary
resources. Retain facade-owned recipes, driver selection and (in05) command queue.
Install new patch identity only on success. Candidate MIDI output must be empty
before attaching cleanup; spawners may initialize state, not produce performance
output. Reject a candidate violating that supported contract before commit.

Candidate load messages belong to the candidate lifetime and remain available
there, with PatchLoaded last as today. Failed candidates expose no messages to
the live owner. Whole-App retirement does not synthesize old Entity removal
messages in the new World: Entity values can repeat across Worlds. Lifetime
replacement invalidates all old readers/references; GraphView is borrowed and
reacquired. Update this boundary explicitly in docs/tests. Granular remove
notifications remain required for in-place instance removal in05. No global
cross-lifetime event bus is a prerequisite.

## D2 — MIDI cleanup through one owner

The existing caller remains the only drain/send owner. Lifecycle never opens an
endpoint, sends behind its back, drains on its behalf, or silently discards MIDI.
Stopping alone does not silence. Caller recipe:

```rust
rack.stop();
for event in rack.drain_midi() { /* send/record each event */ }
rack.load_patch(next_path)?; // or unload_patch / restart
for event in rack.drain_midi() { /* old lifetime's cleanup, before any new note */ }
rack.start();                // omit after unload
```

Track dirty channels: a channel becomes dirty whenever its sink produces normal
MIDI output. Clear that channel's dirty flag only after cleanup has been enqueued;
clear the sink's active note at the same commit. For each selected dirty channel,
in ascending channel order, emit its active NoteOff (velocity0) if present, then
CC123=0 (All Notes Off), then CC120=0 (All Sound Off). Active note implies dirty.
A second reset before new musical output emits nothing; an unused sink emits
nothing. This silences held voices and released/sustained tails without re-running
TM/Clock/LFO. A later musical event may legitimately retrigger a silenced sink.

Cleanup uses sample=`SampleTime::completed_samples()` at the between-samples
boundary, **without incrementing time**. Ordinary output already queued at n-1
precedes cleanup at n. After replacement that cleanup still has its **old** sample
stamp; it must be drained before the new lifetime can emit sample0. Timestamps
are lifetime-local, not a monotonically increasing clock across replacement.
`MidiOutputEvent` need not gain a schema/generation field for this contract; document
boundary events in its formerly sample-only docs. No drain means next-sample
admission still fails exactly as currently specified.

**Capacity:** at most16 sink channels; existing normal worst case5 messages per
sink =80. Reserve storage for128 messages (80 + 16*3) at base-buffer construction,
including empty racks, and enforce those logical bounds. No cleanup allocation
in the sample hot path. Buffer construction uses fallible reservation for the
fallible candidate path; preserve the existing infallible `Oxurack::new` surface
with documented ordinary allocation behavior. Keep the normal logical limit of5 messages per registered sink separate from
the128-message total cleanup storage limit; update reserve/release arithmetic
without shrinking storage under pending cleanup. Keep normal per-sample
capacity/admission tests; fixed storage is not permission to accumulate samples.
Prepare the complete cleanup batch and validate room before changing active notes
or dirty flags. Repeated reset cannot fill the buffer: it has no newly dirty
channels until another sample, and samples require an empty drain first. A fixed
small staging array or bounded preallocated vector is fine; no new queue service.

Reset ignores a latched terminal fault for silence admission and preserves it.
A capacity/invariant error is returned without hiding the original latched fault.
Non-emergency lifecycle admission errors are ordinary typed CoreError variants
(e.g. `LifecycleRequiresStopped`, `PendingMidi { count }`), **not new terminal tick
faults**. A faulted rack can stop, drain, successfully replace/restart/unload,
drain cleanup, then start cleanly. A failed candidate preserves the old fault.
Release old channel reservations with old owner drop; for in-place sink removal,
release after cleanup is safely enqueued. Do not carry old reservations into the
candidate. Keep cleanup separate from candidate reservation accounting.

This is software emission, not evidence that a hardware receiver is silent.
The native example's sender still owns delivery-error cleanup; do not remove its
fallback or add an endpoint acceptance gate to these slices.

## D3 — next instance and command increment

These contracts bind05; CRC details concrete symbols/tests after04 lands. Do not
implement them speculatively in04 except the minimal ownership seams above.

### Instance removal and references

Provide `remove_module(&mut self, name: &str) -> Result<(), CoreError>` under the
matrix above. Use existing cable mutation helpers with DirectApi provenance,
retire incident incoming **and** outgoing cables in effective cable order before
the module, update PatchHandle/CableIndex/propagation dirty state, despawn its
owned children, emit ModuleRemoved once, run graph maintenance before returning.
Hooks for per-instance shared resource release are needed: add an erased,
**infallible** `on_remove(&mut World, Entity)` trait hook defaulting to no-op,
registered with ModuleRegistry. Invoke once before recursive despawn after all
recoverable validation and cleanup admission. MidiOutModule releases its sink
reservation here; the facade prepares MIDI silence first. Custom modules own
instance components/children and must release shared per-instance registrations
in this hook. Hooks may not fail, produce MIDI, or alter unrelated instances;
panic is outside recoverable guarantees. Whole-App drop does not call this hook
per instance; World resources own whole-lifetime cleanup.

Graph fixes: collect incoming + outgoing incident edge mappings (deduplicate
self-edges); remove a name mapping only when it still points to the removed
Entity. Test direct core notification maintenance as well as facade removal:
disconnecting cables first must not conceal the graph handler defect. New-App
replacement itself requires no old/new message interleaving.

Persistent names are lookup references; bound handles are lifetime identities.
Deliver owned `ModuleRef`, `PortRef`, `KnobRef` names with constructors/accessors,
and `module::<M>(&str) -> Result<Handle<M>, CoreError>` validating actual installed
Rust type and instance kind. A handle stores name + opaque rack/lifetime identity
and instance identity, never a public Entity. Use Arc identity tokens (rack lifetime
and per-instance) or an equally collision-free private scheme; Arc pointer equality
avoids counters/global identity services. New lifetimes receive fresh tokens;
removed instances lose their token; failed loads keep tokens. Cross-rack use,
removed instance use and same-name replacement return a typed stale-handle error.
Surviving handles remain valid through removal of a different instance.

Add handle `parameter(&Oxurack, &str)`, `set_parameter(&mut Oxurack, &str,
ParameterValue)` and `port(&str)`/`knob(&str)` producing **bound** references.
Names stay exact/case-sensitive; bound port/knob references retain the handle's
validation token. Add `patch_ports(&PortRef, &PortRef, Option<CableTransform>)`
and `unpatch_ports(&PortRef, &PortRef, usize)` counterparts to existing tuple
APIs; add `set_knob(&KnobRef, ParameterValue)` and parameter read counterpart.
All resolve against active membership before mutation. Do not allow conversion
to an unbound name to bypass stale checking internally. String tuple constructors
are unambiguous. Dotted string parsing uses one separator and rejects ambiguous
names; tuple constructors retain support for names containing dots. Existing
parameter/patch/unpatch signatures remain available and delegate shared helpers.

Historical module-specific Port/Knob enums, `Handle<Lfo>::out/shape` convenience
methods, typed value enums/coercions and broader historical construction styles
remain **Arc13 catalog/API reconciliation**, re-entering with public module
vocabulary for the remaining catalog. The generic handles/string forms above are
actual05 delivery; do not call the entire historical four-style API implemented.
No new public mutable World or duplicate parameter registry.

### Commands and global tempo

Provide `execute_command(&mut self, &CoreCommand) -> Result<(), CoreError>` on
the facade, routing all existing variants through the same helpers as direct APIs.
Load/Save call the real lifecycle/persistence functions; Panic calls reset;
SetBpm changes saved global BPM configuration only. Parameter/label/cable changes
use Command provenance and graph freshness without a tick. Loader initialization
retains PatchLoad provenance; load origin can use `PatchSource::Other` to describe
a command plus path (do not falsify a parameter initialization as a live edit).
Save has no mutation notification. No successful no-op stubs.

Global `bpm() -> Result<f32, CoreError>` and `set_bpm(f32) -> Result<(), CoreError>`
read/write LoadedIdentity. Require a loaded patch; finite and >0 follows existing
`PatchError::InvalidBpm`. It survives snapshot/save/restart. It does **not** change
individual Clock BPM parameters or RT clock selection. Name/document it as the
patch's global BPM field; changing audible tempo uses the Clock parameter today.
No new BPM event type is needed. Low-level `dispatch_core_command(&mut World,...)`
continues handling world-owned operations; facade-only Load/Save/Panic/SetBpm must
return explicit NotImplemented there. Remove Panic/SetBpm successful no-ops in04
and correct their tests/docs; facade delivery follows in05.

Implement retained queued load/save in05 with a small facade-owned queue, not a
Bevy message consumer or worker. `enqueue_command(CoreCommand) -> Result<CommandId,
CoreError>`; `process_commands(&mut self) -> Vec<CommandOutcome>` explicitly applies
pending commands FIFO and returns one typed result per ID in submission order.
A command's inputs (including path/name) are resolved at execution time. Failure
returns its error and processing continues with the next command against the
then-current rack; no retries or acknowledgments by ticking. Successful replacement
keeps the queue outside the swapped App. No background processing.

Bound64 **total outstanding** slots, including cancelled results not yet collected.
An opaque monotonic CommandId uses checked increment; full queue and ID exhaustion
are typed errors before acceptance. Reset success cancels every still-pending
command in submission order, preserving its ID as `CancelledByReset`; completed
results are not overwritten. A queued Panic succeeds and cancels its tail;
commands before it retain their results. A direct reset leaves cancellation
results available to the next `process_commands` call. A simple slot enum
`Pending { id, command } | Completed(CommandOutcome)` in the existing bounded
VecDeque supports this without a second unbounded result store. Cancelled slots
continue consuming capacity until results are collected; this is documented,
not silently dropped output. `process_commands` consumes each result exactly once.
Reset failure cancels nothing. New command submissions after reset are permitted
if slots remain. There is no concurrent producer API in this slice.

Bevy CoreCommand messages remain explicitly unconsumed; do not advertise them as
this queue. `into_app` docs must warn that consuming the facade abandons its pending
facade commands/results; callers process them first. Existing consuming escape-hatch
behavior does not authorize a hidden scheduler/receiver.

## D4 — source boundary, proof and authoring return

**Loader compatibility:** keep current `validate_patch` file rules, including
historical differences from live cable admission. Candidate isolation is not a
schema migration. Keep cable vector order/getter semantics from directive11.
Missing getters/unsupported snapshots still fail restart without touching the old
lifetime. Do not add a serialized runtime phase, output, Scope or handle token.

Permitted04 source homes (CRC enumerates the actual commit paths in its prompt):
`crates/oxurack/src/{lib.rs,persistence.rs,scope.rs,lifecycle.rs}`;
`crates/oxurack-core/src/{module.rs,event.rs,error.rs}` and loader documentation
in `patch.rs`; `crates/oxurack-mod-midiout/src/lib.rs`; affected inline/integration
tests under those three crates; existing facade examples/README and root README.
`lifecycle.rs` is an authorized new module. No dependency or lockfile change is
expected. Do not prohibit necessary cross-crate lifecycle repairs as “production
code”; CRC must list these seams explicitly, with no inferred scope escape.

05 additionally owns facade `commands.rs`/`handles.rs` (authorized new modules),
core module/cable/index/propagation/event/export wiring, graph removal maintenance
in `crates/oxurack-graph/src/lib.rs`, and affected tests/docs/examples. CRC inspects
that fresh baseline and names exact files; changes to standalone musical engine
algorithms, native RT transport or RON schema require another design return.
Private helper organization is local discretion; public behavior above is binding.

### Discriminating proof

Use real modules and existing tests/examples. CRC converts these to slice rows
and exact commands; no new verifier crate, coverage campaign, soak or hardware gate.

- **04 replacement/failure:** repeated same-channel replacement succeeds; exact
  World membership counts and active snapshots/index/graph match a fresh candidate,
  not just its handle. The CRC `[1,0]` candidate is now **valid** in a fresh owner;
  do not preserve its old failure as an oracle. Use `[1,1]` or a deliberate *late*
  fallible custom spawner to exercise candidate rejection. The latter writes a
  resource/entity/message before Err; verify none appear in the old owner, its
  Scope/time/seeded continuation match a control, and a subsequent channel1 load
  succeeds. The late failure fixture must pass file validation to reach spawning.
- **04 hook replay:** source-grounded fixture proves once per App, ordered replay,
  same-type idempotence, collision rejection before mutation, and actual systems
  advance once. Count failed candidate hooks separately; don't assert a global
  once-per-facade count that contradicts this amendment.
- **04 output:** unread NoteOn makes replacement/unload/restart reject without
  state change; reset instead appends exact NoteOff/CC123/CC120, ordered/timestamped,
  preserves the earlier event and emits no duplicates on a second reset. Test16
  dirty sinks at maximal normal output and empty-destination unload; preserve
  existing sample admission/fault tests. Prepare-only failure changes nothing.
- **04 recovery/time:** successful new lifetime clears a preexisting fault only
  after pending output is drained; failed candidate retains it. Stop/start
  continues identical engine/Scope trace, reset keeps phase/time but silences,
  restart reconstructs current configured seed at0 and stays stopped. Drain old
  cleanup before comparing new sample0 trace to a fresh reference. Test driver
  retention and no live catch-up burst. Scope disabled only on successful lifetime.
- **04 demo:** edit current parameters/labels/cables, save, stop/drain, unload,
  drain cleanup, reload, configure Scope/start and replay a musical trace in both
  graph modes. Use existing example surface; deliver a runnable command in README.
- **05 removal/handles:** incoming + outgoing + self-edge cleanup; exact module,
  child, CableIndex and graph maps; sink channel release and cleanup; unaffected
  engine/Scope/handle preservation; name/Entity reuse cannot revive stale handles.
  Include cross-rack, wrong type and failed replacement preserving bound handles.
- **05 command composition:** FIFO load/edit/save replays actual saved values;
  failed load then successful save still saves old patch; full queue rejection,
  direct/queued reset cancellation results, result collection once, and no tick.
  Distinguish Command versus DirectApi events. SetBpm saves global value while
  Clock parameter stays unchanged; reject NaN/nonpositive. Real example exercises
  handles/removal and queued persistence, not only queue bookkeeping.

Affected native gates include core, MIDI-out, graph where changed, facade default
and `--no-default-features`, repository lint/strict Clippy/docs and `make test`.
Retain prior independent evidence only with pinned revision and reason it is
unaffected; lifecycle changes affect facade integration and must be rerun. Preserve
restricted/native failures and host retries honestly. Do not replay Arc10 hardware
witness or reopen Slice03 merely because a successor edits nearby code.

### Guidance applied and readiness responsibility

Installed skills root for this author:
`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
Framework2.6.0; engineering-methods guide07 sections1–7 govern CRC's authoring.
Relevant Rust guidance read by CDC: `rust-guidelines/guides/02-api-design.md`
API-02 (SHOULD borrowed inputs/owned output) and API-23 (MUST concrete errors);
`03-error-handling.md` EH-01 (MUST recoverable errors versus programmer panics);
`05-type-design.md` TD-02/03 (SHOULD domain/newtypes).

Applied: owned refs/results without World borrows; named busy/stale/cancelled errors;
fallible admission before mutation; panic exclusion for arbitrary user hooks;
opaque handle/command identity rather than raw Entity/u64 in user code. The tests
above exercise these choices. The isolated lifetime is a design response to the
actual spawner boundary, not a generic claim that Rust offers rollback.

CRC must recheck the source, turn04 into a concrete prompt with implementation
examples, bounded required readings and six guide07 readiness checks in its slice
plan. Link this directive's04 sections as binding and05 as successor-only; do not
make CC implement the entire directive in04. CRC may adapt private mechanics,
not choose a different lifetime/failure/output/transport policy. If inspection
reveals a consequential contradiction, return it with the smallest concrete
alternative; routine mechanical choices do not require another escalation.

## Evidence and return state

CDC read CRC's witness source/log and independently rebuilt/relinked it against
`deafdd228` on2026-10-12. First rustc attempt used a stale top-level rlib and failed
with missing MidiOutModule/snapshot_patch; it is not runtime evidence. Then
`cargo build -p oxurack --lib` exit0, rustc edition2024 linking
`target/debug/liboxurack.rlib` exit0, temporary witness run exit0. It reproduced
all three errors and exactly one active module versus16 World entities. No source
changes or endpoint operations. Current source inspection confirms live loader
mutation, reservation ownership and graph risks; graph incoming-edge failure is
not claimed reproduced here. No proposed design code has been executed yet.

CRC acknowledges this return in the arc handoff history, authors/opens04, and
reports its project-relative prompt path through Duncan. CRC then reviews04 and
advances05 within this design; assemble Arc12 only after both. Include the legacy
Arc05 transport and typed-vocabulary dispositions in the inherited walk and retain
A-05-01/02 as deferred until appropriate successor evidence exists. R-01–R-04,
P-01–P-06 and release gates remain open. No merge, push, endpoint approval or
broader acceptance is implied by this directive.
