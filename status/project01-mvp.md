# project01-mvp — status

> Current: Three-Contributor Workflow, Expedited Mode. TM-in-rack MIDI playback
> first; Clock/LFO/Scope workbench second. See the priority decision at the end
> and [current CDC directive](../project01-mvp/cdc-directive02.md). The original
> estimate and One-Contributor snapshot below are historical. Restart baseline
> evidence is CRC-accepted; six historical slices closed carried over; six arc
> assemblies await independent CDC review. Product validation remains open.

**Best estimate: approximately 45% of the reconstructed MVP implemented**
(plausible range **35–55%**). The infrastructure is much further along than the
musician-facing product: a substantial engine exists, but a usable rack with
musical modules, live controls and demonstrated recording behavior is still ahead.

Updated **2026-09-17** by Sofie, One-Contributor mode. Source inspected:
`main` at `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`; planning baseline `7f8d901`.
Last implementation change: `d760049` (2026-05-13); subsequent commits through
this source baseline concern documentation/ignores. This is the temporary
project status home requested by Duncan, pending the forthcoming status spec.

## Reading the estimate and the statuses

The denominator is the broad MIDI MVP in the [project plan](../project01-mvp/project-plan.md),
including its eleven baseline rack modules. The estimate is a judgment about
delivered capability, not lines of code, elapsed effort, test coverage, or release
readiness. Its deliberately rough weighting is:

| Capability group | Share of MVP | Estimated implemented |
| --- | ---: | ---: |
| RT/core/event/graph infrastructure | 45% | 90% |
| Public runtime API | 15% | 25% |
| Rack module catalog | 25% | 5% |
| Remaining product integration and acceptance | 15% | Approximately 0% demonstrated |

That gives 45.5%, rounded to about 45%. The weights are provisional, not a new
scope agreement. A decision to target the older science rig instead would require
a different denominator; see the restart decision below.

**Delivered** below means the historical feature scope has implementation in
source/history. **Open** means material feature work remains. These reconstructed
arc names postdate the implementation: “opened” here means work has landed in
that area, not that a historical framework opening record exists. No formal arc
closure or independent acceptance is inferred. Current builds, tests, coverage,
hardware timing and musical acceptance have **not** been rerun for this snapshot.

| Started arc | Feature status | What matters on resumption |
| --- | --- | --- |
| 01 — RT and MIDI | Foundation delivered; operational acceptance pending | Real MIDI/clock engine exists; hardware/timing and rack integration still need evidence |
| 02 — Core and persistence | Mostly delivered; persistence completion open | Patch data/file round trips and loading exist; saving the current live world does not appear implemented |
| 03 — Events and literate patching | Historical substrate delivered; runtime extensions open | Notifications and labeled patch files exist; live editing and several emission sites remain |
| 04 — RackGraph | v0 query capability delivered | All 18 catalog queries are represented after the getter follow-up; keep future mutation APIs consistent with freshness |
| 05 — Public API | Open; partial | Bootstrap/load/install/introspection exist; runtime rack operation is substantially incomplete |
| 06 — Module catalog | Open; early | One standalone musical engine exists; its rack adapter and the other baseline rack modules remain |

## Arc01 — RT and MIDI

**Delivered.** Dedicated MIDI runtime/thread lifecycle; input/output port handling;
compact queue messages and lock-free RT↔ECS queues; master clock; slave clock with
tempo estimator/PLL and dropout handling; Start/Stop/Continue and song position;
passthrough clock multiplication/division. Priority elevation, overflow/error
reporting and timing primitives are implemented. Unit/thread test code exists.

**Discoveries and scope changes.** Early review found that priority elevation
was immediately undone when its handle was dropped. Follow-ups retain the handle
for the thread lifetime and make elevation failure explicit. Queue overflow and
previously discarded RT warnings/song-position events now have reporting paths.
The passthrough mode was expanded from a stub. Shared `MidiMessage`/`MidiWire`
types were extracted into `oxurack-midi`, and MIDI field masking was hardened.
These are supporting infrastructure, not a MIDI-out rack module.

**Remaining / deferred.** Establish current hardware/DAW master/slave behavior,
jitter and coverage results, and connect the runtime to the public rack lifecycle.
Scheduled output is deferred: the ignored `SendMidi.timestamp_ns` field was
removed rather than promising timing it did not deliver. SysEx payload handling
remains a historical deferral. Audit item **F-15** leaves stopped-state resume
latency pending a real DAW requirement; do not confuse this with normal tick jitter.

**Where recorded.** [Historical arc plan](../project01-mvp/arc01-realtime-midi/arc-plan.md),
[RT design](../project01-mvp/arc01-realtime-midi/artifacts/0006-oxurack-rt-the-real-time-thread.md),
[foundation review](../../../workbench/core-rt-implementation-review.md), and
[audit ledger](../../../workbench/old/cc-rust-audit-ledger.md).
History anchors: `178fd9b` (original phase completion), `4f920af` (passthrough/shared
MIDI), `f109eb7` and `205b252` (review repairs). Source home: `crates/oxurack-rt/`.

## Arc02 — Core and persistence

**Delivered.** Typed values, ports, cables and transforms; merge policies;
Produce→Propagate→Consume scheduling; deterministic ordering and per-module seed
derivation; scale/quantization vocabulary; module registration and explicit spawn
contract; parameter setters **and getters**; optional RT bridge. Patch validation
checks names, kinds, ports, merges, cycles and numeric validity. RON patch data can
be serialized/deserialized and saved/loaded as files; registered module spawners
apply a patch to an ECS world. Full-cycle dummy-module integration tests exist.

**Discoveries and scope changes.** Persistence settled on **serde + RON**, and
unused core Reflect machinery was removed. A real module-spawn/patch-loader path
had to be added after the original implementation claimed completion. The RT
bridge moved to Bevy's non-send resource handling, removing its unsafe Sync
assertion. Cable propagation gained stable ordering and a dirty cache; cycle
diagnostics were consolidated; non-finite values and invalid BPM are rejected.
`Interleave` was removed because one-value-per-port-per-tick could not implement
the promised behavior. These are substantive changes to the early design.

**Remaining.** `save_patch_to_file(&Patch, ...)` saves a supplied data structure;
it is not a snapshot of a running rack. No world→Patch capture path was found in
the public persistence surface. Saving live parameter/topology/label edits and
proving fresh-process output equivalence remain open. Centralized setter schema
validation still has a TODO; individual setters currently own validation.
Arbitrary module-state persistence has a draft design, not implementation.

**Deferred / dependent work.** Audit **F-7** (empty port/kind names) needs a design
decision; **F-21** (remaining linear validation scans) waits for large-patch
evidence; **F-23** (duplicate Scale types) belongs with Turing Machine integration.
`PersistableModuleState` and the chord module are a coupled candidate extension;
their draft leaves trait/registry shape, serialized-state handling and failure
semantics unresolved. They are not counted as completed or added to the MVP.

**Where recorded.** [Historical arc plan](../project01-mvp/arc02-core-and-persistence/arc-plan.md),
[core design](../project01-mvp/arc02-core-and-persistence/artifacts/0007-oxurack-core-ecs-world-values-ports-and-cables.md),
[module-authoring design](../project01-mvp/arc02-core-and-persistence/artifacts/0008-oxurack-module-infrastructure-authoring-modules-on-bevy.md),
[foundation review](../../../workbench/core-rt-implementation-review.md),
[audit ledger](../../../workbench/old/cc-rust-audit-ledger.md), and
[state/chord draft](../../../workbench/persistable-module-state-and-chord-module-design.md).
History anchors: `4fd37bd`, `205b252`, `7a6fcdb`.
Current save boundary: [patch.rs](../../../crates/oxurack-core/src/patch.rs),
`save_patch_to_file`; setter boundary: [event.rs](../../../crates/oxurack-core/src/event.rs),
`dispatch_set_parameter`.

## Arc03 — Events and literate patching

**Delivered.** The notification substrate records module/cable creation and
parameter changes during patch load, command-driven parameter changes, and a
patch-loaded summary with provenance/counts. Removal notification types are
declared. The getter follow-up populates `ParameterChanged.previous` for runtime
commands when a getter exists; patch initialization intentionally retains `None`.
Labels, descriptions, validated tags, rack notes and the display-name formatter
are implemented, with backward-compatible patch-file fields and `LabelChanged`
emission during load. Both historical slices have landed implementations.

**Discoveries and scope changes.** “Knob” events became “Parameter” events;
`old` became `previous`. Literate patching grew the patch schema and emission
contract and was split into substrate and patch-integration passes. Rack notes
became a simple string list; the formatter became configurable data rather than
a user closure. Rich author/date/version/parent-patch metadata did not land.

**Remaining.** Runtime label/note edits; removal/unload emission sites tied to
actual lifecycle APIs; and stronger cross-type notification-order verification.
Declared `ModuleRemoved`/`CableDisconnected` types are not proof that public
remove/disconnect operations exist. File-data round trips do not establish live
world capture of edited metadata.

**Deferred.** Failure notifications, audit-log consumer, notification serialization
for IPC, specialized future events (**D-25.8**), DAG command ordering, modulation
cable distinctions; port-instance labels, richer tag rules/hierarchy, note IDs and
per-note events, tag search, formatter configuration persistence, and rich rack
provenance. Revisit each with its concrete consumer. The old getter deferral
**D-25.1 is implemented**, despite still appearing open in older registries.

**Where recorded.** [Arc plan and both slices](../project01-mvp/arc03-events-and-literate-patching/arc-plan.md),
[event memo](../project01-mvp/arc03-events-and-literate-patching/slice01-notifications/artifacts/0012-event-substrate-bucket-3.25-design-memo.md),
[literate memo](../project01-mvp/arc03-events-and-literate-patching/slice02-literate-patching/artifacts/0013-literate-patching-bucket-3.5-design-memo.md),
[graph reconciliation](../project01-mvp/arc04-rackgraph/artifacts/0011-rackgraph-design-memo-v0.md),
and [D-25/D-35 registry](../../../workbench/deferred-tracking.md).
History anchors: `229b616`, `ce10c39`, `7a6fcdb`, `d760049`.

## Arc04 — RackGraph

**Delivered.** Separate, feature-gated graph crate; reactive topology maintenance;
module/cable rosters, kind/name lookup, ports, neighbors and incident cables;
dangling-node/port diagnostics; cycles, topological order, transitive reachability
and port compatibility. `parameters_of` adds schema/current-value introspection,
completing the previously deferred eighteenth catalog query. Result structs are
serializable; labels are resolved from the current ECS world. A dedicated
maintenance schedule makes the graph fresh immediately after umbrella patch load.
All three historical slices plus the getter follow-up have implementation evidence.

**Discoveries and scope changes.** The original broad graph bucket became three
parts to keep implementation/review manageable. Identity follows ECS `Entity`,
not the original memo's `ModuleId`. The final query is named `parameters_of`, not
`knobs_of`; missing getters produce absent values rather than blocking inspection.
The getter change also closed an event-substrate deferral, so graph completion
depends on work that landed after the original part-three report.

**Follow-ups / deferrals.** Future cable/remove/unload APIs must honor the same
freshness contract; fresh-after-load does not prove those unimplemented APIs.
Signal-versus-modulation edges, cable-to-knob routing, tag-based queries and IPC
transport remain future work. Serializable query results do not implement IPC.
No additional missing v0 catalog query was found; current feature-matrix and
composition acceptance still need evidence.

**Where recorded.** [Arc plan](../project01-mvp/arc04-rackgraph/arc-plan.md),
[design and reconciliation](../project01-mvp/arc04-rackgraph/artifacts/0011-rackgraph-design-memo-v0.md),
[getter specification](../../../workbench/cc-bucket-getter-parameter-introspection.md),
and [current graph source](../../../crates/oxurack-graph/src/lib.rs).
History anchors: `9a0dda4`, `fc0e000`, `5c49283`, `7a6fcdb`, `d760049`.

## Arc05 — Public API

**Delivered.** `Oxurack` bootstraps a Bevy app with 1000 Hz fixed-time configuration,
module installation registers spawners/setters/getters, direct file-based patch
loading works through core, and graph access is feature-gated. `step()` updates
the app; `into_app()` is the Bevy escape hatch. Re-exports were deliberately
narrowed to the user-facing vocabulary. Core SetParameter dispatch resolves a
loaded instance and calls its registered setter.

**Remaining, explicitly separated by feature.**

| Feature | Present state / missing behavior |
| --- | --- |
| Runtime cable edits | AddCable/RemoveCable return NotImplemented; cables are wired directly during patch load |
| Runtime persistence/lifecycle | Queued LoadPatch/SavePatch return NotImplemented; direct load is separate; live save and unload remain |
| Transport and power | No complete public play/start/stop/pause/resume/power surface; SimulationRunning exists but does not itself implement or gate a complete lifecycle |
| Panic/reset and tempo | Core Panic and SetBpm return Ok without action; do not count these as reset or tempo propagation |
| Ergonomic API | Typed module/port/knob handles, module enums, domain value enums, tuple/string operations and public set/get convenience remain |
| Validation | Setter-local validation exists; central schema validation remains unresolved |

**Discoveries / scope boundary.** Naming moved from `OxurackApp`/`register_module`
to `Oxurack`/`install`; redundant Bevy accessors and `run()` were removed. Earlier
silent command stubs were made explicit NotImplemented errors, but Panic/SetBpm
are still successful no-ops. `step()` is an app update, not a promise of exactly
one deterministic musical tick. The science-rig plan deliberately deferred live
cable editing and saving; their absence is expected historical scope, not proof
that the broader MIDI MVP is complete.

**Where recorded.** [Arc plan](../project01-mvp/arc05-public-api/arc-plan.md),
[public API design](../project01-mvp/arc05-public-api/artifacts/0010-oxurack-public-api-design-v0.md),
[umbrella source](../../../crates/oxurack/src/lib.rs), and
[command dispatch](../../../crates/oxurack-core/src/event.rs).
History anchors: `2583d40`, `c3756a7`, `17eb3d3`, `8e9652c`.

## Arc06 — Module catalog

**Delivered.** Standalone Turing Machine Mk2 engine: shift-register looping and
randomization, length/write controls, quantized musical outputs, clock/step
behavior, and optional direct MIDI I/O. It was renamed to the `oxurack-mod-*`
convention. Audit follow-ups hardened feedback-length invariants and MIDI errors.

**Remaining.** The Turing Machine crate still lacks its core/Bevy module adapter,
port/parameter registration and rack-driven tick integration. Its Scale remains
separate from core's. No baseline rack-module implementation was found for
**Clock, Noise, LFO, Sequencer, Euclidean, Quantizer, Range, Sample-and-Hold,
Arpeggiator or MIDI-out**. Shared `oxurack-midi` types and standalone TM MIDI I/O
do not fill that MIDI-out module slot. The catalog is therefore not “two modules
done”; even its first engine still needs integration.

**Discoveries / candidate scope.** There are two later directions beyond the old
catalog: the Clock/LFO/**Scope** science rig and the **Chord** module paired with
module-owned persistence. Draft designs exist, but no corresponding crates were
found. Neither silently expands the current eleven-module denominator. MIDI-in
was explicitly deferred to v1.1. Lykn behavior remains an externally dependent
proposal; current embedding readiness was not investigated here.

**Where recorded.** [Arc plan](../project01-mvp/arc06-module-catalog/arc-plan.md),
[Turing Machine historical slice](../project01-mvp/arc06-module-catalog/slice01-turing-machine/slice-plan.md),
[catalog](../project01-mvp/artifacts/0009-module-catalog-and-build-order-v2.md),
[state/chord draft](../../../workbench/persistable-module-state-and-chord-module-design.md),
and [Lykn requirements](../../../workbench/lykn-embedding-requirements-from-oxurack.md).
History anchors: `c0155c3`, `355962f`, `7158cb4`; source:
[Turing Machine crate](../../../crates/oxurack-mod-turingmachine/Cargo.toml).

## Restart decision and checks that should not be lost

**Choose the next product milestone before treating the migration's suggested
cable-operations slice as settled.** The [older science-rig memo](../../../workbench/v0-science-rig-plan.md)
describes Clock + LFO + an in-patch Scope, a four-pane TUI and a small REPL,
with the RT thread dormant. It labels some decisions ratified, while the overall
memo remains draft. The migration's project plan instead reconstructs the broader
MIDI v1 roadmap and excludes TUI product work. This is a real scope/sequencing
tension, not something this status snapshot resolves by choosing a winner.
The state/chord draft is another candidate next step. Record the Operator's choice
in the project plan when made; no implementation assignment is issued here.

Whichever milestone is selected, establish the current build/test/feature baseline,
check its real module integration, and keep live-save and no-op command gaps visible.
For the broader MVP, coverage, fresh-process reproducibility, hardware/DAW timing,
resource measurements and real musical-use acceptance remain outstanding evidence.
Historical test counts/pass claims are not current results.

Planning migration itself is complete: 23 documents moved with preserved history;
eight arc homes and six document-backed slice homes exist. Main subsequently
removed the excluded generated index in `a7ce6e6`; do not restore it from the
migration report's historical description. The
[migration report](../project01-mvp/artifacts/migration-report.md) and
[path map](../project01-mvp/artifacts/migration-manifest.tsv) explain provenance.

## Maintenance and evidence boundaries

Read this file first for **current status**, the project plan for **scope**, and
the [migration backlog](../project01-mvp/artifacts/status-and-backlog.md) for the
full inventory A–I mapping. That backlog is a dated baseline; this snapshot adds
live-world saving, no-op commands, old audit deferrals, richer metadata deferrals,
and the science-rig scope tension. Update this file when work lands or decisions
change; include the inspected source revision and revise the estimate's basis if
scope changes. Retain completed-feature summaries and explicit deferral owners.

Workbenches linked above are local, Git-ignored archaeology outside the planning
branch. Their important findings are summarized here so this tracked status does
not depend on those files surviving. Implementation links assume the established
`.worktrees/planning` location; commit anchors can be inspected with `git show`.
Evidence is source inspection plus historical records, self-checked by Sofie;
ledger acceptance states remain unchanged. This temporary layout is not the
forthcoming status specification.

## Version History

- v0.1 — 2026-09-17: Initial Operator-requested project status snapshot; summarized
  six started areas, estimated broad-MVP progress, reconciled stale completion
  claims and retained newly recovered gaps, deferrals and scope questions.

## Current restart update — 2026-09-17

This section supersedes the snapshot's One-Contributor mode and unassigned restart.
Three contributors and Expedited Mode are selected. CDC/Sofie authored the first
baseline slice; CRC/CC contexts await Operator relay/acknowledgement. Active prompt:
`arc08-validation-and-release/slice01-restart-baseline/cc-prompt.md` (relative to project01-mvp).

Six historical slices have carryover closure candidates; arcs01–06 have carryover
proposals awaiting child review and CRC/CDC arc gates. They are **not formally
closed or behaviorally accepted**. All 24 historical rows remain open and appear
in the receiving slice's register. Arc07 stays planned; arc08 is active for early
baseline collection, while final system acceptance still waits for the product.

No Rust tests were run during this planning pass. The 45% estimate and its
uncertainty remain the earlier source-based estimate, not refreshed acceptance.
The broader MIDI scope remains unchanged; alternate milestone selection remains
with Duncan/CDC. Next: relay `project01-mvp/cdc-directive01.md` to CRC, execute
the named CC baseline assignment, then review carryovers and select the first
feature slice. The Makefile dependency gate's suppressed error path is a baseline
reliability finding to document, not silently repair in a documentation slice.

### Priority decision received — 2026-09-17

Duncan selected **TM loaded in the Bevy framework and producing MIDI**, then
**Clock/LFO/Scope as a testing workbench**, retaining other features. This resolves
the pending milestone choice above. Current relay: `project01-mvp/cdc-directive02.md`;
current CC prompt relative to project01-mvp:
`arc08-validation-and-release/slice01-restart-baseline/cc-prompt-iteration01.md`.
The initial packet/prompt are preserved, superseded before execution.

[The dependency map](../project01-mvp/arc08-validation-and-release/slice01-restart-baseline/artifacts/turing-machine-dependency-map.md)
records hidden execution registration, seed handoff, musical versus simulation
clocking, phase/edge latency, ordered MIDI/note cleanup and bounded observation
needs. Source findings are not reproduced behavior. Full live editing, Chord/state,
other modules and final quality/hardware obligations stay tracked later. Scope is
an explicitly requested diagnostic addition; the old 45% estimate is historical,
not a completion metric for these focused milestones.

### CRC baseline and carryover update — 2026-09-17

CC's evidence-only packet at `df6ec0a6906c2b16c049db8f267145861b623c41`
was independently reviewed. Arc08/slice01 is closed with R-01–R-06 done as
**baseline observations**, not as product acceptance. Build, lint, format and
workspace/feature tests were reproduced with normal MIDI access. Rustdoc still
fails two links. Clean direct coverage was 93.47% line coverage (886 missed of
13,560), below the inherited 95% target, with nine mismatched-data-function
warnings. Three RT tests remain ignored; physical MIDI, timing, benchmarks,
soak and Operator music session are unrun. Keep project P-03/P-05 open.

The six historical slices are closed carried over with all twelve S rows
deferred, original criteria and named domain owners retained. Six arc closing
packets account for actual children 0/0/2/3/0/1 and propose deferring twelve
A rows; **all six arcs remain open** until CDC independently verifies the CRC
assemblies. The exact review paths and cross-arc TM-first design boundary are
in [crc-escalation03.md](../project01-mvp/crc-escalation03.md). The next
implementation slice awaits CDC design/issuance; this CRC pass made no Rust,
dependency or acceptance-target change.
