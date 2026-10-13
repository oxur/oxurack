# CRC escalation12 — resolve the Slice04 lifecycle contract

2026-10-12, Sofie/CRC to CDC through Duncan. Three contributors, Expedited Mode.
Owner: Project01 / Arc12 live rack operations / planned slice04-runtime-api.
Predecessors: directive08 playable-first policy, directive10 Arc12 opening,
directive11 configured getter/order correction. This is a real design request,
not a CC implementation prompt or an architecture amendment already approved.

Source clean at `deafdd228fb44720878c700f8ff62b2a59ddbad6`; planning intake
`9b156a416d85b5ce8f6517cccbae0e4337b675c2`; main clean/unchanged
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. CRC closes Slice03's five G rows at
this source, with fresh affected native gates and retained independent docs/demo
proof in its crc-verification.md. Final reviewed CC assignment was
arc12-live-rack-operations/slice03-graph-composition/cc-prompt-iteration01.md;
no active CC assignment remains. Iteration01 completed, correction1/5; this
handoff is not a second Slice03 correction or a reset of its iteration count.

Routine close records advance project-plan to v3.37, Arc12 arc-plan to v1.6 and
Slice03 plan to v1.2. Arc12 R-01–R-04 stay open; original Arc05 A-05-01/02 and
Arc04 eight rows remain historically deferred with their successor requirements.
Slice04 is not yet opened: its full inherited scope is retained in the arc plan.
CDC owns consequential design; CRC can detail the resolved increment afterward.

## Findings: source and independently reproduced behavior

1. **Transport conflict.** Historical Arc05
   artifacts/0010-oxurack-public-api-design-v0.md, Semantics, specifies separate
   power and transport; start begins at0, stop resets position, pause/resume
   preserve. Its reset is narrow All-Notes-Off plus command-queue clear, preserving
   knobs/cables/transport/power. Current accepted facade lib.rs:274–296 explicitly
   preserves sample time and module state through start/stop. Core event.rs:83–85
   describes Panic as resetting all modules. These are different contracts.
   Arc09/10/11 and current native continuation tests depend on accepted preservation.
   Arc12 says reset/transport must be coherent but does not choose which semantics
   supersede which older requirements. CRC must not silently rename behavior or
   call a discarded requirement implemented.

2. **Replacement and failed candidate leak.** Facade lib.rs:204 calls
   apply_patch_to_world in the live World and swaps only PatchHandle after success.
   Core patch.rs:528 onward validates, then spawns modules/emits success events
   sequentially before the complete candidate succeeds. The registry's arbitrary
   spawner receives &mut World; the trait in module.rs:216 has no rollback contract.
   MIDI-out lib.rs:167/239 reserves channels in a shared MidiOutputBuffer; no
   release hook exists. Sink state includes active_note at111–115. Existing
   load_patch errors preserve the old handle, not necessarily the old World.

   CRC ran a bounded temporary Rust program against the current compiled facade:
   load one midi_out channel0; reload it; attempt a candidate with channels1,0;
   then try a candidate using1. Observations (compile0, run0):

   ```text
   same-channel replacement: Err(InvalidParameterValue { module: "sink_0", param: "channel", reason: "channel 0 is already assigned to another midi_out sink" })
   partial replacement: Err(InvalidParameterValue { module: "sink_1", param: "channel", reason: "channel 0 is already assigned to another midi_out sink" })
   reuse candidate channel after failure: Err(InvalidParameterValue { module: "sink_0", param: "channel", reason: "channel 1 is already assigned to another midi_out sink" })
   active snapshot modules: 1
   active handle modules: 1
   world live entities: 16
   ```

   The two live sinks each own seven child ports, but only one sink belongs to
   the active handle. Channel1 remains reserved after its candidate fails.
   Source and log are transient in
   `/var/folders/dt/kc8ndn6d3kx8c1jnxr05drsh0000gn/T/oxurack-crc-deafdd2-9aa57kcj/`
   as replacement-witness.rs/.log. This justified one small linked reproduction,
   not a new Cargo/probe crate, dependency, lockfile or permanent proof framework.
   No repository source change, tick or endpoint connection was involved.

3. **MIDI ownership needs a lifecycle rule.** MidiOutputBuffer::check_admission
   rejects any pending event and retains the first terminal fault; drain is sole
   output ownership. Existing facade stop preserves active module notes/state;
   the playback example owns endpoint cleanup separately. Unload/replacement/reset
   must specify where silence messages go, their timestamp/order, behavior with
   unread events, channel-release ordering and whether terminal faults recover.
   Neither silently discarding pending output nor retaining reservations forever
   meets coherent lifecycle. Endpoint transport is not owned by the facade.

4. **Graph retirement needs composed proof.** graph lib.rs:960's ModuleRemoved
   handler collects graph.inner.edges(idx) before removal; its current incident
   edge test removes a source with an outgoing edge. Directed incoming mappings
   need explicit coverage. Maintenance orders all added messages before all removed
   messages, and removed handling erases by_instance_name by string. A replacement
   reusing an instance name can therefore require separate old-retirement and new-
   installation maintenance stages, or identity-aware removal. This is inspected
   risk to test/resolve, not a newly reproduced Slice03 cable defect. All current
   native graph tests pass; fresh cable disconnection already retires that edge.

5. **Remaining API/command and loader contracts.** CoreCommand messages have no
   consumer. Explicit dispatch implements parameters/labels/cables, returns
   NotImplemented for LoadPatch/SavePatch, and succeeds without effect for
   Panic/SetBpm (event.rs:353–370). Arc05 A-05-01 still names queued load/save,
   handles, transport and reset. Arc12 permits synchronous ownership and requires
   an explicit queued-compatibility disposition; it does not authorize deleting
   those obligations. Typed ModuleRef/PortRef/KnobRef/Handle vocabulary in the old
   memo is also aspirational. Legacy file validation intentionally remained less
   strict than Slice03 runtime validation; the final lifecycle packet must choose
   compatibility or an explicit changed policy rather than tighten it accidentally.

The first replacement gap was anticipated in Arc12. The reproduced channel
reservation/failure case and conflicting semantics make its required repair
concrete; they do not invalidate earlier single-load or fresh-rack acceptance.
No new safety platform, concurrent output service or prerequisite arc is proposed.

## Decisions requested, options and CRC recommendation

**D1 — exact lifecycle and recovery matrix.** Resolve load-new/load-replace,
unload, narrow silence/reset, transport restart and instance removal: prerequisites,
running flag/driver/SampleTime, engine seed/phase, Scope, metadata/membership,
pending output, fault, registries/hooks and event/graph state on success/error.

Recommend retaining the accepted start/stop preservation contract. Add explicit
restart/transport behavior under distinct names if the MIDI START/STOP distinction
is required; preserve a narrow silence/reset operation instead of reseeding all
engines implicitly. Separate generation/reconstruction from voice silencing.
An alternative is deliberate API migration to historical power+four transport
verbs, with all affected old acceptance/tests identified and approved. Do not
silently alter start/stop tests to match a newly inferred interpretation.

For replacement, recommend constructing and validating a candidate in an isolated
owner before commit, with deterministic installed-type replay and a clear new
patch lifetime. This avoids mutating an old performance during arbitrary spawner
failure and allows a reused MIDI channel. It needs an explicit erased installer/
resource boundary and must preserve owners that are promised to survive; merely
swapping App would lose pending MIDI/Scope and could repeat installation hooks.
Define those effects rather than pretending App swapping is automatically safe.
A same-World prepare/rollback approach preserves the existing App but needs a
new supported-spawner side-effect contract: entity cleanup alone cannot undo
arbitrary resource/message writes. CDC should select one concrete approach,
its supported custom-module guarantees and failure limits, rather than make CC
solve transactional architecture. Do not destroy the old patch first and then
claim a failed load preserved it.

**D2 — MIDI silence and reservation/fault semantics.** Recommend explicit fallible
lifecycle admission, preserving unread musical events and producing any cleanup
through the existing drain. Decide whether non-emergency replacement/unload
requires stopped+drained state or may queue cleanup while running; whether reset
is allowed during backpressure/fault; how active notes are silenced once; when
channel reservations release; and whether a successful fresh lifetime clears
terminal fault. Capacity must account for cleanup without unbounded hot-path
allocation. An alternative deliberate discard/panic policy needs explicit API
and caller acknowledgment, not undocumented loss. Do not automatically connect
or send to an endpoint, add another drain owner or equate engine emission with
hardware silence.

**D3 — queued compatibility, tempo and ergonomic references.** Recommend one
explicit facade command owner using the synchronous helpers, with honest typed
results and Command provenance; no acknowledgment tick, new scheduler or async
service. Decide whether bounded queued load/save compatibility is implemented in
this slice or explicitly retained for Arc07 with a re-entry condition and approved
Arc05 reconciliation. If queued, resolve ordering, drain/results, failure and
reset-clear behavior. A successful no-op Panic/SetBpm is not an implemented command.
Choose what global BPM does versus individual Clock parameters/RT bridge; legacy
bpm does not currently synchronize Clock by implication.

For ergonomic instances, recommend owned persistent instance-name references
with explicit lifetime/generation validation for bound handles; no raw Entity
escape or borrowed &mut World for a user. Decide which typed/string coercion
forms are delivered now and how same-name replacement invalidates an old handle.
Any deferral of original forms stays named in the inherited walk. Do not add a
second parameter/cache registry or pretend a stale bound handle is live merely
because its name exists again.

**D4 — loader compatibility and source boundary.** Retain legacy file validation
unless CDC deliberately chooses strict runtime admission on load and identifies
which accepted file behavior changes. Specify the exact implementation boundary
needed for D1–D3 (facade/core module lifecycle, installed modules/MIDI buffer,
events/graph, tests/example/docs) and whether the planned single Slice04 remains
coherent in context. The original full Slice04 outcome remains binding; CRC does
not propose silently reducing it to unload-only. If these decisions need a split,
CDC owns that structural amendment and must retain all original requirements.

These recommendations are unapproved design options. CDC can resolve them
without another exploratory CC iteration; CRC will inspect the selected shape,
apply guide07/Rust guidance and author a concrete implementation spine and
behavioral oracles. Ask Duncan only for product choices genuinely requiring his
judgment; local implementation details can be resolved under existing authority.

## Expected return and re-entry

Duncan: pass this packet to CDC. CDC: issue project-root **cdc-directive12.md**
answering D1–D4, with approved semantics/source scope, required plan amendments,
affected acceptance, proof and explicit CRC permission to issue Slice04. Preserve
prior directives; do not consume response slot12 for an unrelated assignment.

Expected discrimination in that future packet: repeated same-channel musical
replacement; later-spawner failure with old performance/owner continuation;
exact old module/child/cable/index/name-map retirement; incoming and outgoing
edge reuse; pending note-on plus exactly ordered cleanup; fault/recovery and
stop/resume/restart distinctions; stale handles across same-name replacement;
queued load/save results if chosen; real edit/save/unload/reload musical recipe
in both graph modes. Existing fixture/native evidence stays preserved; no new
coverage/soak/hardware campaign is imposed by these findings.

Only affected advancement is held: Slice03 is closed and its acceptance can be
committed now. Arc12 composition stays open, Slice04 implementation prompt is not
issued, and historical API/graph/project/release gates stay with their existing
owners. CDC owns the next design decision; Duncan relays the return to CRC.
