# Arc12 — live rack operations

Version1.0 — 2026-10-11. Status: active; Slice01 issued, other slices planned.
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

## Four delivery slices and retained ownership

| Slice | Product result | Dependency / inherited owner |
| --- | --- | --- |
| slice01-live-persistence | Public parameter read/write, snapshot current loaded configuration, save RON and replay in a fresh rack; runnable edit/save example | Arc11; Arc02 A-02-01/02 live snapshot/seeded round-trip obligation |
| slice02-events-and-metadata | Public live label/note/tag edits with correct notifications, persistence and useful inspection | Slice01; Arc03 A-03-01/02 and both historical child S rows each |
| slice03-graph-composition | Runtime cable edits with validation, event emission, immediate graph freshness and save/reload consistency | Slice02; Arc04 A-04-01/02 and all six child S rows; cable part of Arc05 |
| slice04-runtime-api | Coherent unload/replacement/reset/transport and ergonomic instance operations, composing edit/save with lifecycle; address retained command-routing/API gaps | Prior slices; Arc05 A-05-01/02 and original lifecycle/handle/queued-operation requirements |

Only Slice01's open set is authored now. The original successor IDs remain intact.
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

Module instance names are persistent identities. Bevy entities are process-local.
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

Queued operation compatibility remains owned by Slice04. Concurrent output/reconnect
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

CRC assembles four child outcomes, inherited-owner walk and runnable composed
edit/save/reload evidence; CDC independently gates arc closure. Broader MIDI
hardware timing, 95%/coverage provenance, benchmarks and soak remain Arc08;
remaining modules/Chord Arc13, full CLI/REPL/controllers Arc07. ProjectP-01–P-06
remain open. P-07's delivered instrument/workbench stays accepted.

## Version history

- v1.0 — 2026-10-11: Arc11 delivered workbench and Operator selected live persistence.
  Opened existing four-slice successor with configuration/runtime-state distinction,
  getter-driven save, direct ownership and a concrete owner for append-only loading.
