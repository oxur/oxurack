# CRC escalation14 — resolve session observation and metadata before Slice02

2026-10-14, Sofie/CRC → Duncan → CDC. Three contributors, Expedited Mode.
Owner: Project01/Arc07 live interface, planned slice02-session-observation.
Predecessor: [directive13](cdc-directive13.md), which expressly requires this
return after Slice01 closure; it is not an unresolved defect or another Slice01
correction. Directive12's accepted lifecycle contract remains binding.

## State, authority and what is held

CRC independently accepts Slice01 iteration01 at source
`e582aed6e87c234799921cc66a5dd55214f76afd`, CC planning
`518897ed5978e9e6b3fece776a1fe8bb1ca63537`. Both worktrees clean at intake.
The [CRC record](arc07-live-interface/slice01-live-controls/crc-verification.md)
contains fresh mode/native/host/actual CLI and interaction proof; all five LC rows
done, closed after one corrective pass1/5. Initial failed evidence is preserved.
No current CC assignment. Root main has separate docs-only cd3119c; CRC does not
alter it or implementation source.

Close/return records advance project v3.48, Arc07 v2.2 and Slice01 v1.2. Arc07
A-07-01/A-07-02 remain open; P-01–P-06 remain open and P-07 keeps prior acceptance.
Arc12 and its historical residual disposition are not reopened. Current dependency
order is accepted01 → CDC design return → detailed02 → subsequent controller/
config and external transport. No Slice02 directory/prompt is created merely to
hold this request, and no proposed design below is already authorized.

**Held:** authoring/implementation of02's observer/lifecycle/metadata and dependent
watch/completion promises. **Available:** the accepted runnable live controls,
Scope/save and retained software MIDI workflow. No merge/push, hardware campaign,
new event service or concurrent output owner is requested.

## Source findings, not assumptions

Inspected current implementation at e582aed:

- `crates/oxurack/src/lib.rs:78–87`: facade owns App, install recipes, loaded
  identity, lifetime/instance Arc tokens and bounded command queue. There is no
  facade-lifetime public notification stream. `into_app` consumes this owner.
- `src/lifecycle.rs:106–187`: unload/replacement constructs an isolated candidate,
  preserves cleanup in the new buffer, then commit replaces App and drops the old
  World. New lifetime/instance tokens replace old ones. An observer stored only in
  the old World cannot span that transition; old Entity payloads cannot be treated
  as identities in the new World. Failed candidates must remain unpublished.
- `src/persistence.rs:14–31`: LoadedIdentity stores version/seed/BPM, not the
  original input path or a persistent logical patch/lifetime identifier. The live
  example passes its startup patch path to Scope; interactive replacement needs
  an explicit current-display/provenance policy.
- `crates/oxurack-core/src/event.rs:90–276` and core plugin registration:
  separate typed Bevy Messages for structural/parameter/label notifications and
  PatchLoaded, with summary-last producer intent. No PatchUnloaded. Several
  payloads contain Entity. Per-type message buffers and counts do not by themselves
  establish a common runtime-observed cross-type chronology. Existing graph
  maintenance consumes its own message cursors; a new watcher must not steal them.
- Facade `labels.rs:1–34` exposes full-value labels; `cables.rs::patch/unpatch`
  provides synchronous live topology edits; lifecycle removal/replacement requires
  stopped and drained. The accepted rack-live input thread only frames lines;
  the main owner executes commands and delivers MIDI. It has no terminal editing/
  completion library or current session-replacement command yet.
- Core `label.rs:89–97`, `event.rs::LabelValue::Notes` and `patch.rs:50–62` store
  notes as Vec<String>, tags as validated flat case-preserving strings/sets.
  LabelTarget covers Rack, Module(name), and Cable(exact endpoints, occurrence).
  Notes have no per-note identifier/authorship/revision model. Duplicate equal
  notes and cable occurrence shifts make inferred content/index IDs insufficient
  as a persistent edit identity without a deliberate rule.

These are expected successor design seams. Current tests reproduce accepted
lifecycle/metadata behavior, not a public watch capability. The source investigation
above is not claimed as runtime proof of an observer not yet implemented.
Directive13 and Arc12's [historical decisions](arc12-live-rack-operations/cdc-verification.md#historical-decisions-and-remaining-owners)
retain actual unload receipt, measured common ordering, per-note/authorship/history/
tag queries in02; tag/formatter configuration in03; transport in04; richer graph/
port/typed vocabulary inArc13. Nothing here silently deletes those obligations.

## Decisions requested and CRC recommendation

### D1 — observation owner, identity and delivery contract

Choose the actual public observation surface and its relationship to the existing
Bevy message types: facade-owned ordered receipts, World-local observers bridged
into a facade owner, or another concrete mechanism. Resolve what survives successful
load/replacement/restart/unload, whether watchers are independent cursors or one
explicit drain owner, and exactly which direct/command mutations are observed.
Does the promise include lower-level World APIs after into_app, or end at that
ownership transfer? Decide capacity/overflow, subscriber start position and error
policy; a dropped record must be distinguishable from complete history.

**Recommendation:** the smallest bounded in-process, pull-based facade/session
observation mechanism with owned logical targets, a sequence/lifetime identity
and an explicit gap/reset policy. Keep existing typed messages for Bevy consumers;
do not require callbacks, threads, IPC, durable audit storage or a generic event
bus. Capture common order at actual mutation/emission points, not by concatenating
per-type buffers afterward. Whether the public API is multi-cursor or destructive
must be decided by CDC, not left as an accidental implementation choice. Explain
how graph consumers retain their own evidence and failed candidates stay private.
A World-local-only watcher is smaller but cannot fulfill the named cross-lifetime
requirement; it would require explicit re-scoping, not a completion claim.

### D2 — lifetime notification and failure/ordering matrix

Define PatchUnloaded/replace/restart observations: old logical patch identity,
source/path availability, sample/lifetime stamps, reason, new identity and batch
boundary. Is replacement an unload followed by ordered new granular events and
PatchLoaded, or a distinct replacement receipt? Which receipt covers explicit
unload, restart, initial load, repeated empty unload and failed load? Does reset
emit an observation distinct from lifecycle retirement? State publication only
at commit and the failure acknowledgment policy separately from successful changes.

**Recommendation:** preserve existing lifetime/reset/restart admission semantics;
only successful commit publishes retirement/new-lifetime success. A failed
candidate publishes no successful structural changes and leaves observers bound
to the old lifetime. Failed commands may have explicit rejection replies without
inventing an unapproved public ParameterSetFailed contract. Carry old cleanup
with its old sample/lifetime rather than attaching it to sample0 of the new rack.
Use owned logical names/endpoints plus a lifetime discriminator so same-name reuse
is unambiguous; do not send old Entity removals into the new World.

CDC should specify a runtime-observed literal order for: initial load; label →
parameter → cable edit; remove with incident cables; same-name replacement; restart;
unload twice; and failed candidate. A common collector must observe that actual
order during mutation/commit, not infer it from final counts. This is the explicit
missing stronger observer proof, not a reason to reopen Arc12's direct-owner API.

### D3 — session command, MIDI and Scope behavior

Choose02's complete command set/grammar for load/unload/restart/remove, patch/
unpatch, label/notes/tags/find/watch, including parallel cable selection and
error/help behavior. Does lifecycle editing require the user to pause/drain first,
or may the frontend implicitly interrupt playback? Define whether the existing
native connection and Session persist across patch changes, how cleanup reaches
that owner once, and how current patch display/provenance is updated. A new patch
may invalidate selected Scope ports: reject the operation before commit, clear/
reconfigure capture with an explicit reply, or another deliberate policy?

**Recommendation:** retain one rack/output owner and explicit stopped/drained
lifecycle admission; no hidden auto-pause/restart on rejected edits. Live-compatible
cable/label mutations still run synchronously between samples. Keep the selected
MIDI connection across successful rack replacement and deliver old cleanup once
before new playback. Define Scope selector validation and current patch label as
part of that transition; do not silently keep a stale startup path or claim
continuous capture when replacement intentionally creates a new lifetime. Preserve
successful save's configuration-versus-engine-state distinction.

### D4 — per-note model, persistence and tag-query semantics

Resolve the smallest actual persistent per-note identity/CRUD/authorship/history
model promised by directive13. Who supplies IDs and authorship, what revision or
history is retained, and which events carry previous/new values? What happens to
legacy notes strings, equal-text duplicates and mixed/new file representations?
Choose one authoritative representation and explicit migration/compatibility
behavior; a second unsynchronized notes store or list index is not a substitute.
Clarify whether this changes public RackMeta/LabelValue/RackConfig types/schema
and which existing callers/fixtures must be migrated or kept compatible.

**Recommendation:** preserve old patch loading, deterministic save/fresh replay and
explicit user-authored metadata; avoid implicit current time/OS author that makes
otherwise identical files differ without a stated need. CDC must choose the exact
serialized identity/history shape before CC sees a prompt. Do not defer the
already retained capability merely by naming fields as future work.
For tag queries recommend exact current Tag membership, deterministic logical
result ordering and explicit Rack/Module/Cable target coverage; no case folding,
hierarchical tag meaning, query-language engine or formatter/config persistence
is inferred. Decide match-any/all and target default rather than letting parser
convenience set the API. Rich tag/config policy remains03 unless CDC amends it.

### D5 — terminal completion and achievable increment

The current bounded stdin-only helper cannot perform normal interactive Tab/history
editing as implemented. Choose a supported line-editing integration/dependency and
its effect on signal/output/thread ownership, or explicitly specify another
completion UX and reconcile the historical requirement. An output-only `complete`
command is not silently equivalent to interactive completion. Watch output also
needs a selected stderr/output cadence so background redraw does not corrupt
line editing or stdout MIDI traces. Avoid a scheduler/output service unless a
measured need earns it.

Please assess whether the combined observer/persistence/command/completion work
fits one coherent02 packet. CRC can detail an approved bounded increment, but
cannot independently move note/history/completion obligations to unnamed later
work. If it needs more than one slice, CDC should amend the plan and give each
retained feature an explicit owner/re-entry before prompt authoring. This is a
sizing/design decision, not permission to reduce scope. Controllers/config and
external transport remain subsequent capabilities, not hidden prerequisites.

## Required CDC return / next owner

Duncan: please relay this file to CDC. CDC should answer via matching
`cdc-directive14.md` at this project level, resolving D1–D5 with behavior/ownership/
compatibility choices and any authorized roadmap amendment; state which metadata/
command features belong in the next executable increment and which are held with
concrete re-entry. A resolved design needs a source-grounded API/producer/commit/
consumer spine and discriminating observer/legacy replay oracles, not only “add
watch.” CRC will then author the next implementation prompt using guide07 and
Rust guidance, preserving accepted Slice01/Arc12 behavior and scoped Expedited
commits. No pending response file is pre-created. Until that return,02 remains
unissued; Arc07's two composition rows and project gates remain open.
