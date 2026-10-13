# Arc12 — live rack operations

Version1.8 — 2026-10-12. Status: active; Slice01–03 CRC accepted atdeafdd228 or their named predecessors. CRC acknowledges CDC directive12 at67e9150 and issues [Slice04 lifecycle](slice04-runtime-api/cc-prompt.md). Six LF rows open; Slice05 stays unopened until04 acceptance. Four arc rows remain open.
Depends on accepted Arc10/Arc11; source predecessor72fe6b5. Blocks saved/live
musical workflow and Arc07 controls. Related: carried-over Arcs02–05; Arc13 catalog.
Authority: approved successor roadmap and Duncan's explicit “Continue with live
persistence” after Arc11 review. Three contributors, Expedited Mode. CDC owns
architecture/first slice, CRC reviews and details subsequent approved slices.

## Capability

Edit the running instrument, preserve its current configuration as a reusable RON
patch, and make subsequent metadata/topology/lifecycle edits coherent with events,
graph views and persistence. Each slice delivers an executable use of the rack.
Reuse the current synchronous owner and core registries; introduce neither an
asynchronous service nor a second scheduler as a prerequisite.

## Slice01 design correction — directive11

CC's a3633bc return exposed two assumptions in CDC's initial packet. Was: the
TM registered getter returned effective probability under the historical Arc10
contract, while saving expected the configured base; Entity ties were assumed to
survive reload. Now: [directive11](../cdc-directive11.md) authorizes the registered
parameter getter to expose existing write_base, retaining effective engine/output
semantics; loaded cables gain CableOrder from Patch.cables vector position, used
by propagation and snapshot with the exact fallback policy in the directive.
No RON schema or standalone engine change. The .25 base and .25 LastWins replay
oracles remain binding. These repairs are part of Slice01, not additional slices.
CRC issued [iteration01](slice01-live-persistence/cc-prompt-iteration01.md) at
source a3633bc after reproducing the failed replay and inspecting the diagnostic
tie assertion. Was: all LP rows open pending review. Now: CRC accepted all five at fabfa1cb;
[Slice01 verification](slice01-live-persistence/crc-verification.md) retains
independent evidence. Directive11 is fulfilled, with no additional design change.

## Five delivery slices and retained ownership

| Slice | Product result | Dependency / inherited owner |
| --- | --- | --- |
| slice01-live-persistence | Public parameter read/write, snapshot current loaded configuration, save RON and replay in a fresh rack; runnable edit/save example | Arc11; Arc02 A-02-01/02 live snapshot/seeded round-trip obligation |
| slice02-events-and-metadata | Public live label/note/tag edits with correct notifications, persistence and useful inspection | Slice01; Arc03 A-03-01/02 and both historical child S rows each |
| slice03-graph-composition | Runtime cable edits with validation, event emission, immediate graph freshness and save/reload consistency | Slice02; Arc04 A-04-01/02 and all six child S rows; cable part of Arc05 |
| slice04-runtime-api | Failure-preserving replacement/unload, narrow silence/reset, explicit seeded restart, MIDI cleanup/recovery and edit/save/unload/reload | Prior slices; Arc05 lifecycle and reproduced leak; directive12 D1/D2/D4 |
| slice05-instance-commands | In-place instance removal, stale-safe handles/references, synchronous and bounded queued commands, global BPM configuration and runnable use | Slice04; remaining Arc05 handle/queued-operation requirements; directive12 D3/D4 |

Was: Slice02's open set issued. Now: Slice01 and Slice02 independently closed;
[Slice02 verification](slice02-events-and-metadata/crc-verification.md) retains
actual source/native/replay evidence. CRC issued
[Slice03 graph composition](slice03-graph-composition/slice-plan.md).
Was: its981964b return passed native/runnable checks but left required interaction
proof open. Now: CRC accepted [iteration01](slice03-graph-composition/crc-verification.md)
atdeafdd228 after fresh affected checks and bounded inherited reconciliation;
five G rows done, correction1/5 completed. No Slice03 scope amendment. Next-slice
investigation reproduced MIDI channel reservation and partial-candidate leaks and
found historical/current transport conflicts. [Escalation12](../crc-escalation12.md)
was answered by [directive12](../cdc-directive12.md). CDC selects isolated App
candidates, stopped/drained replacement, narrow reset through the sole MIDI drain,
current start/stop preservation and explicit restart. The previously unissued
Slice04 is split into lifecycle04 and instance-command05; no requirement disappears.
Separate MIDI transport re-enters Arc07; catalog-specific typed forms re-enter
Arc13. The directive is authoritative for the complete matrix, failure boundary,
installation replay, source scope, compatibility and discriminating tests.
The original successor IDs remain intact.
Graph composition is a product increment (actual cable operations plus their graph
behavior), not a test-only arc inside the arc. CRC supplies concrete contracts and
examples when each later slice opens, escalating any consequential choice not
covered here. Carryover criteria are reconciled against actual historical source
specifications, not discharged simply by creating these directories.

## Binding design across the arc

One rack owns its mutable Bevy world and sample clock. Synchronous calls execute
between samples; do not run a tick to acknowledge a command. Pending MIDI belongs
to the existing sole drain owner. Configuration reads/saves neither drain it nor
reset engines, SampleTime, Scope or transport. No auto-save in the 1 ms schedule.

A patch stores configuration for seeded reconstruction: module parameters,
connections/transforms and metadata, master seed, schema version and legacy global
BPM field. It does not store the TM register/RNG cursor, oscillator phase, current
notes, transport position, capture window or hardware connection. Exact performance
checkpointing remains the retained module-state feature with the catalog/backlog;
this arc must not imply it has been implemented. This clarifies the historical
“tick A, save, load B, tick B” proof: compare B with a fresh rack given the saved
configuration, not the continuation of A's internal state.

Module instance names are persistent configuration lookup identities. Bound handles
add rack-lifetime/instance validation under directive12; names do not revive stale
handles. Bevy entities are process-local.
Read registered getters at snapshot time; missing getters fail rather than silently
saving defaults or initial overrides. Public facades use owned Patch/ParameterValue
and CoreError, without requiring consumers to obtain the Bevy world.

Loaded patch membership uses PatchHandle. A source inspection found that current
load_patch/apply_patch_to_world appends entities and replaces only the handle; its
existing replacement test checks the handle alone. Slice04 must repair and prove
actual old-module/cable retirement, reset ownership and failure behavior before
claiming in-place replacement. Slice01 reloads into a fresh rack and snapshots
only the active handle; it must not accidentally persist stale prior entities.
This discovered gap is owned here, not an Arc11 defect or a new prerequisite arc.

The existing RON CableConfig has no enabled field. Slice01 reports an explicit
unsupported-snapshot error for disabled selected cables, rather than silently
turning them on or dropping them. If Slice03 exposes cable enable/disable, its
packet must specify compatible file representation first. No schema migration or
global BPM-to-Clock synchronization is implicit in Slice01.

Metadata/event provenance must reflect the actual initiating API; direct setters
use DirectApi, existing command dispatch retains Command. Successful mutation
emits its notification once; failed mutation does not emit success. Metadata and
graph slice details must preserve this discipline and existing getter feature modes.

Queued operation compatibility is now owned by Slice05 under directive12. Concurrent output/reconnect
re-enters only if demonstrated musical operation requires it (directive08); the
old batch/ack/join design is not mandatory. Do not turn every stub or aspirational
API in historical docs into infrastructure needed before the first saved patch.
Explicitly disposition original requirements at the arc close; no silent dropping.

## Proof, closure and boundaries

Five first-slice rows and four arc composition rows are the present contracts.
Use real modules, loaded RON, per-sample MIDI drain and fresh replay. Preserve
unaffected independent evidence with revision/rationale; rerun affected native
checks. No new validator, probe crate, coverage campaign or endpoint audition is
required just to save a configuration. Native failures stay visible.

CRC assembles five child outcomes, inherited-owner walk and runnable composed
edit/save/reload evidence; CDC independently gates arc closure. Broader MIDI
hardware timing, 95%/coverage provenance, benchmarks and soak remain Arc08;
remaining modules/Chord Arc13, full CLI/REPL/controllers Arc07. ProjectP-01–P-06
remain open. P-07's delivered instrument/workbench stays accepted.

## Design handoff — 2026-10-12

[Escalation12](../crc-escalation12.md) after Slice03 acceptance atdeafdd228 is
resolved by [directive12](../cdc-directive12.md), issued for Duncan's relay to CRC.
D1–D4 approved with explicit historical transport/typed-form deferrals; R-03/R-04
and the unissued Slice04 split are affected. Authority: Duncan's replanning and
continue-live-persistence instructions. CRC acknowledged the disk packet at67e9150 and inspected clean sourcedeafdd228.
[Slice04 open set](slice04-runtime-api/slice-plan.md) now issues the approved lifetime
repair with worked installation/candidate/cleanup shapes, six LF rows, exact source
allowlist, guide07 readiness and Rust2.0 guidance. It delivers the existing save
example’s --lifecycle edit/save/unload/reload/Scope musical session. Slice05 remains
unopened until04 independent acceptance; arc gates remain.

### Current lifetime and compatibility contract

A new patch lifetime is a new isolated App with deterministic installed-type
replay; supported module hooks own effects inside that App/World. Fallible candidate
failure leaves the old owner intact. Hooks run once per App, not once forever per
facade. Successful replacement/restart/unload resets sample time, leaves execution
stopped, retains driver and types, disables Scope, and preserves prepared old
cleanup for the existing caller to drain. Stopped + drained is required; a fault
can recover through a successful new lifetime. Reset is narrow silence, can append
to unread output even during a fault, and preserves engines/time/Scope/fault.
Start/stop retain accepted continuation. Loader validation/schema stay unchanged.

Slice05 adds in-place instance teardown and identity-validated handles, retaining
survivor state. One synchronous facade owns commands; a bounded64-slot explicit
queue supplies FIFO results and observable reset cancellation, without a scheduler.
Global BPM remains configuration independent of module Clock parameters.
Historical separate MIDI transport is Arc07; module-specific typed vocabulary is
Arc13. See directive12 for exact semantics and re-entry; historical documents and
successor register03 remain provenance, with this amendment governing current routing.

## Version history

- v1.8 — 2026-10-12: CRC acknowledges directive12 and opens approved Slice04 with six rows, concrete lifecycle/cleanup/hook recipes, bounded intake and fresh native/runnable gates. No new design amendment or acceptance; source/main unchanged and Slice05 held.

- v1.7 — 2026-10-12: CDC directive12 resolves lifecycle/output ownership; independently reproduces replacement leak. Split unissued04 into lifecycle04 and instance-command05 with full inherited scope accounted for. Preserve start/stop; explicit transport/catalog deferrals. CRC authors04 next; no acceptance added.

- v1.6 — 2026-10-12: Slice03 correction independently accepted atdeafdd228,
  five G rows done; test-only evidence repaired, prior independent musical/docs
  proof retained at981964b. Slice04 source investigation makes the already-owned
  replacement failure concrete; consequential lifecycle choices escalated to CDC.
  No approved structural amendment or parent acceptance.

- v1.5 — 2026-10-12: Slice03 native/demo checks reproduced at981964b;
  four required-proof gaps kept five G rows open. Test/evidence iteration01
  issued without changing the four-slice roadmap; parent gates remain open.

- v1.4 — 2026-10-12: CRC accepted all five metadata rows at20c60ef, with host retry
  and unchanged fresh annotated musical replay. No architecture/scope amendment.
  Issued detailed runtime cable composition with provenance/order/atomic validation
  and runnable rewire/replay; inherited rows and final composition stay open.

- v1.3 — 2026-10-12: Slice01 correction independently accepted; no structural
  bubble-up. Detailed approved Slice02 with shared synchronous metadata helpers,
  occurrence-based cable selection, typed failures/events and runnable annotation.
  No global ID/schema/queue change; lifecycle and remaining composition stay open.

- v1.2 — 2026-10-11: CRC acknowledged directive11 and issued Slice01 correction1/5
  with source-grounded getter/order recipes and original preservation/replay gates.
  No arc scope/architecture amendment or child acceptance; later slices held.

- v1.1 — 2026-10-11: Slice01 CC return exposed configured/effective getter conflict and allocator-dependent duplicate-cable order. CDC reproduced both and resolved semantics/source scope in directive11; CRC issues a bounded correction. Initial acceptance goals remain intact.

- v1.0 — 2026-10-11: Arc11 delivered workbench and Operator selected live persistence.
  Opened existing four-slice successor with configuration/runtime-state distinction,
  getter-driven save, direct ownership and a concrete owner for append-only loading.
