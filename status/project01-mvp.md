# project01-mvp — status

> Current (2026-10-14): Arc10, Arc11 and [Arc12](../project01-mvp/arc12-live-rack-operations/cdc-verification.md)
> are CDC accepted. Arc12 source 27ca00f delivers live parameters/metadata/topology,
> current configuration saving, failure-preserving replacement, unload/reset/restart,
> instance removal/stale-safe handles and explicit bounded command results.
> Five children/27 rows and four arc rows done. CDC reproduced composed playback
> preservation and 10000-sample fresh replay in both feature modes (82 MIDI/30082
> Scope records); host workspace/lint/docs pass; restricted CoreMIDI failures retained.
> Duncan selected **live controls for the existing workbench**. Arc07/Slice01 is
> now CRC-accepted at e582aed/CC518897: five LC rows done after one correction.
> [Independent review](../project01-mvp/arc07-live-interface/slice01-live-controls/crc-verification.md)
> reproduces typed live edits/status, fair playback, default Scope/export/save,
> native trace/accounting and shutdown; all three original failing probes now pass.
> Both feature/native/host gates and actual CLI sessions pass; nonempty saved/fresh
> replay matches independent configuration and pinned MIDI/LFO hashes are intact.
> [Escalation14](../project01-mvp/crc-escalation14.md), required by directive13,
> returns02's observer/lifecycle/metadata/completion decisions to CDC through Duncan.
> No current CC assignment;02 unissued and both Arc07 rows remain open. No arc or
> project acceptance inferred. Source worktree remains e582aed; CRC changed plans
> only. Root main has separate documentation-only updates through cd3119c.
> Historical unload notification/common observer ordering re-enter Arc07/Slice02;
> metadata/config/controller/transport and catalog/typed/graph extensions retain
> explicit Arc07/13 owners. Exact checkpoints remain future work.
> P-07 stays accepted with the original software MIDI witness; P-01–P-06 and
> hardware timing/coverage/benchmarks/soak/release remain open. Three contributors,
> Expedited Mode and directive08 playable-first policy apply.
> Earlier snapshots below are history.

**Historical estimate (2026-09-17): approximately 45% of the reconstructed MVP implemented**
(plausible range **35–55%**; not recomputed for the current candidate). The infrastructure is much further along than the
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

## CDC return on escalation03 — current, 2026-09-17

Reviewed planning `390d2aa2c3449f8a6e4bbaff3df6c64d821ce4ea` and unchanged source
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Six historical slices and arcs01–06
are now **closed, carried over**, all 24 inherited rows deferred with exact
[successor routing](../project01-mvp/artifacts/cdc-successors03.tsv). No historical
capability was upgraded to delivered/accepted. Arc08 baseline is observation-complete.

Active implementation assignment: arc09/slice01 module behavior installation,
with default system hook, idempotent type/kind registration, pre-mutation conflict
error and bounded strict-doc repairs. Next arc09 slices establish sample/phase
semantics and seed delivery; arc10 TM+MIDI follows, then arc11 workbench.
Other original operations/modules remain assigned to arcs12/13 and retained07/08.

CDC independently reproduced 11 rustdoc link diagnostics (three unresolved and
eight redundant), a larger observed set than the earlier two TM links. Raw log
retained. Q-01 is in the first implementation slice; coverage provenance and
95%-target hardening have separate arc08 successors. CDC did not rerun coverage
or claim MIDI/timing/soak acceptance; CRC's qualified 93.47% report remains open.

Relay [cdc-directive03.md](../project01-mvp/cdc-directive03.md) to CRC; after
acknowledgement relay the current CC prompt relative to project01-mvp:
`arc09-executable-rack/slice01-module-behavior-installation/cc-prompt.md`.
Earlier prompt/directive/status entries above are historical. Implementation code
has not changed during this CDC planning/review pass.

## CRC prompt-authoring assignment — current, 2026-09-17

Duncan requested adoption of the newly installed authoring guide. CDC refreshed
collaboration-framework 2.3.0 and read the full guide/template.
Relay `cdc-directive04.md` (project-relative) to CRC. It supersedes the instruction
to relay the initial CC prompt unchanged: CRC must inspect source, apply Rust/test
guidance, provide implementation recipes and exact test oracles, then record all
six readiness results before issuing a preserved replacement. No replacement
prompt is issued yet. The intended next-unused filename is
`arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration01.md`.
Source and acceptance criteria are unchanged; prior handoff entries are history.

## CRC current handoff — 2026-09-17

CRC has issued `arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration01.md`
relative to project01-mvp for Duncan to relay to CC. The earlier `cc-prompt.md`
remains preserved and unexecuted; executed refinement count is 0 of 5. The slice
plan records six author-readiness results. I-01–I-06 remain open, source main is
still pinned at `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`, and no new
implementation or acceptance evidence is claimed by this planning handoff.

## CRC partial review — 2026-09-17

At source `d43078ea9c71bba10035699c9914bef3891f1d83`, CRC independently
reproduced I-01, I-03 and I-04. I-02 needs an after-edit reinstall test, I-05
needs the unchanged all-targets Clippy and structural planning checks resolved,
and I-06 needs evidence-provenance repair. Slice01 remains open; arc09/slice02
is held. Project-level `crc-escalation05.md` is the current Operator relay to
CDC; earlier status sections above describe historical handoffs.

## CDC return05 — current, 2026-09-17

CDC independently reproduced both Clippy diagnostics and the stale-assignment
verifier failure. Directive05 approves the exact test-only graph/bridge edits,
metadata-driven verifier repair, after-edit reinstall test and honest provenance
corrections in one CRC-authored iteration02. The consuming into_app API requires
a test arrangement retaining the rack; an existing private unit-test context is
recommended, with no public API addition.

Relay `cdc-directive05.md` to CRC. Iteration02 is authorized for authoring, not
yet issued. Required gates remain unchanged, slice acceptance and arc09/slice02
remain held, and no source implementation was changed during CDC's return.

## CRC iteration02 handoff — 2026-09-17

CRC's `arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration02.md`
is now the current assignment for Duncan to relay to CC. It covers the exact
test and planning-checker scope approved by directive05, plus the after-edit
reinstall oracle and provenance repair. Source remains clean at
`d43078ea9c71bba10035699c9914bef3891f1d83`; I-02/I-05/I-06 are open,
and arc09/slice02 is held pending independent acceptance. Earlier return05
wording describes CDC's state before CRC authored the prompt.

## CRC slice01 close and slice02 handoff — current, 2026-09-17

CC's iteration02 source `6a31c0f99241eccb0dcf0f23e9f64ac9dec2f94a` and
planning evidence `0f4561b`/`4fb9f2f` were independently reviewed. Arc09/slice01
is closed with I-01–I-06 done. After-edit reinstall, both feature variants,
full Clippy, strict docs, format, verifier and host-access workspace/make
gates pass. Restricted MIDI/timing/DNS failures and three ignored RT tests
remain visible; this is not physical MIDI or global coverage acceptance.
The first 20 historical validation rows retain their results, with exactly ten
cwd corrections and explicit dirty-tree provenance.

CRC opened arc09/slice02 under the existing CDC execution contract: indexed
1 ms samples, five named phases, stopped/start/stop behavior, exclusive
manual/live drivers and stable named ordering. Six S rows are open. Relay the
current CC prompt relative to project01-mvp:
`arc09-executable-rack/slice02-simulation-samples/cc-prompt.md`.
Arc09/slice03 seed handoff remains held until slice02 review; arc10 TM/MIDI,
arc11 Clock/LFO/Scope, Q-02/Q-03 coverage and hardware gates remain assigned.

## CRC slice02 partial review and iteration01 — current, 2026-09-17

CC source `1a22066`/`8a73d33` and planning evidence `80ddf0e`/`85ea14d`
provide passing sample traces, principal Rust gates, host retries and preserved
restricted failures. CRC independently reran focused sample/core tests,
all-targets Clippy, formatting and diff check, and inspected source/planning
diffs. S-01/S-02 behavior reproduces. S-04's transition test does not run the
next live update, S-05's ordering fixture does not force numeric ID and name
order to disagree, and S-06's close report is under `artifacts/` while the
green verifier checks slice01 only. S-03 needs fixed-core false and stopped-live
negatives. All six ledger rows remain open pending a clean-candidate review.

CRC's [partial verification](../project01-mvp/arc09-executable-rack/slice02-simulation-samples/crc-verification.md)
records the findings. Relay the current CC assignment relative to project01-mvp:
`arc09-executable-rack/slice02-simulation-samples/cc-prompt-iteration01.md`.
No arc-plan amendment or slice03 opening is authorized by this partial review.

## CRC slice02 close and slice03 handoff — current, 2026-09-18

CC's repaired source `5c1ce2c` and planning packet `733327f`/`e82637d`/
`f2897af` were independently reviewed. CRC reran core/package/no-default
tests, full Clippy, format, strict docs, host `make check`, no-default workspace
tests, diff check and the slice02 checker with missing-close/stale-assignment
negative controls. Source/principal planning diffs were inspected. S-01–S-06
are done; the reported source has no production changes after the first
sample implementation. Restricted MIDI/DNS attempts and three ignored timing
tests remain documented. Physical MIDI, global 95% coverage and arc
composition are not accepted by this slice close.

CRC opened arc09/slice03 within CDC execution-contract §3: pre-construction
seed context, fixed FNV-1a vectors, legacy spawner compatibility and separate
process replay. Relay the current CC prompt relative to project01-mvp:
`arc09-executable-rack/slice03-seeded-spawn-context/cc-prompt.md`.
Arc10 TM/MIDI remains CDC-owned and held for arc09 composition review.

## CRC slice03 close and arc09 composition handoff — current, 2026-09-18

CC source `642313a` and planning evidence `698d2d0`/`cc35c4b` were independently
reviewed. CRC reproduced all six fixed seed vectors, both fresh child-process
traces, changed seed/name and reordered controls, construction-time context,
legacy registration fallback, failure isolation and core/umbrella patch paths.
Core/package/no-default tests, both host workspace matrices, Clippy, format,
strict docs, host `make check`, diff and planning checks pass. T-01–T-06 are
done; no corrective iteration was needed.

CRC assembled arc09's three slices and 18 accepted child rows against E-01–E-04.
The composition packet is operationally ready, but the arc is not closed:
CDC must independently review `arc09-executable-rack/closing-report.md`,
`arc09-executable-rack/crc-verification.md`, and
`arc09-executable-rack/ledger.md`. Arc10 remains unopened and CDC-owned.
Physical MIDI/timing, the three ignored RT tests, global 95% coverage,
benchmarks, soak and musical acceptance retain their existing gates.

## CDC Arc09 review06 — historical candidate, 2026-09-18

All standard package/host gates reproduce and the connected seeded rack agrees
across fresh processes, driver groupings and spawn reorderings. A new stop-during-
batch probe fails in both graph modes: advance_samples(3) reports 3 and advances
clocks 3 ms although only one sample completes. CDC rejected arc close and issued
`cdc-directive06.md` for CRC to author `slice04-sample-stop-accounting`.
The repair must finish the current latched sample, return actual partial progress
and advance clocks only for completed samples. Source was not modified by CDC.
Arc10/Clock opening waits for repaired composition and independent review.

## CRC Slice04 issue — historical handoff, 2026-09-18

CRC acknowledged directive06 and opened the six-row stop-accounting slice at
clean source `642313a` and planning `6a1bc13`. All six author-readiness checks
pass. The current assignment is
`arc09-executable-rack/slice04-sample-stop-accounting/cc-prompt.md`; it permits
only `crates/oxurack/src/lib.rs` plus the named Slice04 evidence outputs. M-01–M-06
remain open, no corrective pass has run, and Arc10 remains held.

## CRC Slice04 close and Arc09 reassembly — current, 2026-09-18

CRC independently reproduced source `37deeace` and closed M-01–M-06 without a
correction. Stops in Produce, Consume and Observe finish one five-phase sample
and return/count/advance 1 ms; index-one stop yields two; stopped requests are
stable; start resumes owned seeded state against an uninterrupted reference;
false-then-true continues. Both feature matrices and strict repository gates pass
with the retained three ignored RT tests.

Arc09's four slices/24 rows are CRC-accepted and reassembled in
`arc09-executable-rack/closing-report.md` and `crc-verification.md`. E-01–E-04
remain open for CDC's new-candidate review. Arc09 is not closed and Arc10 remains
held; the next project exchange is CDC's independent verdict.

## CDC Arc09 close / Clock handoff — 2026-09-18

The stop-during-batch defect is independently reproduced as fixed: request 3
returns/counts 1 with clocks 1 ms. Connected seeded composition and full gates pass
on `37deeace`, closing Arc09 with four slices/24 child rows. Product source remains
unchanged by CDC. Arc10's first assignment is:
`arc10-turing-machine-midi/slice01-musical-clock/cc-prompt.md`.
Clock is planned and issued, not implemented. TM adapter/MIDI output still follow;
no device playback, final timing, coverage or musical acceptance is claimed.

## CRC Clock close / Turing Machine adapter handoff — 2026-09-19

CRC independently accepted corrected Clock source `a19cfd1` with C-01–C-06
done after one corrective pass. Exact pulse/edit behavior, diagnostics, full
Manual/Live traces, stop/resume/reinstall and host quality gates reproduce.

The current assignment is now
`arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt.md`.
Its plan inventories all engine settings/actions and all `StepOutputs` fields,
with nine parameters and 32 ports, explicit Scale conversion and seeded
standalone/fresh-process oracles. A-01–A-06 are open. MIDI output/runtime,
physical playback, Arc11 LFO/Scope and retained project quality gates remain
future work.

## CRC Slice02 review / iteration01 handoff — prior state, 2026-10-10

Source is clean at `9ba55dd`; CC planning packet `516afcd`/`2eb5ac1` is the
correct latest return. CRC independently reran standalone112/rack120/TM
integration5 tests, normal dependency tree, full all-target Clippy, format,
diff and planning checks. These pass, but the full required behavioral matrix
is not established: registered parameters/deep failure preservation, None/held
publication, conflicts/modulation, continuing pause/reinstall and meaningful
name/unrelated-allocation controls need repair. The name assertion accepts an
identity-only rename with identical values; this is a test-oracle defect, not
a claimed production seed defect.

Relay to CC relative to project01-mvp:
`arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt-iteration01.md`.
All A rows remain open. Corrective pass1/5 is authorized but not executed.
The prompt follows framework2.6.0/authoring guide and Rust 2.0.0, with exact
test recipes, state/ownership shapes and expected observations. Production
source is protected. Initial evidence is preserved, with the independent
review in Slice02 `crc-verification.md`. Full workspace/docs/Make/freshness
were not rerun by CRC during this partial review; CC's prior attempts stay
attested and the corrected candidate must return all gates.

Arc10 remains open; Slice03 MIDI output stays held. TM MIDI then LFO/Scope
remain the product priorities; physical endpoint, release timing, coverage,
broader catalog/persistence and other retained project gates remain assigned.
Earlier handoff sections above are history; the opening estimate is explicitly
the unrecomputed 2026-09-17 snapshot. No new percentage is asserted.

## Directive08 acknowledgement / Slice02 return — prior state, 2026-10-10

CRC refreshed project-plan v3.16, Arc10 arc-plan v1.3/design v1.2, the amended
ledgers and directive08 against clean planning `61671a3` and implementation
`b2ffb16d342d9a6cc800bab2713813d9646fb505`. The directive is acknowledged in
the project plan's current Design handoff history. Three contributors and
Expedited Mode remain active.

CC has returned the test-only Slice02 iteration01 correction, with its canonical
closing report and validation evidence in planning `61671a3`. This supersedes
the preceding "not executed" status, not the issued assignment or prior CRC
findings. CC reports 112 standalone, 129 rack-feature and five integration tests
passing, with host retries retained; these results are attested until CRC
reproduces the affected proof. A-01–A-06 remain open. Slice03 remains held until
Slice02 acceptance and source repinning.

Subsequent delivery is installed MIDI-out plus a runnable seeded sample/byte
trace, then direct endpoint playback and stop/error cleanup in the same example,
then Arc11's LFO/Scope workbench. Software or hardware is sufficient for the first
real endpoint witness; an unavailable endpoint remains an explicit CDC-owned
disposition. Async output/reconnect belongs to Arc12 when needed, and release
coverage/timing/benchmark/soak obligations remain with Arc08. No ledger status
or acceptance result changed during this refresh.

## CRC Slice02 accepted — prior handoff state, 2026-10-10

Source `b2ffb16` is independently accepted: six A rows done after corrected
behavioral tests, source-boundary inspection, host workspace matrices, strict
docs, Clippy/format, Make and dependency observation. Full receipts and retained
limitations are in Slice02 `crc-verification.md` and `artifacts/crc-review02/`.
No production edit or design escalation was needed. Slice03 is now ready for
CRC authoring under directive08 at this accepted predecessor. Arc10/project
rows remain open; MIDI playback and the endpoint witness remain ahead.

## Slice03 MIDI-out/trace issued — current, 2026-10-10

CRC closed Slice02 and committed independent acceptance as `95bf441`. Source
remains clean at `b2ffb16d342d9a6cc800bab2713813d9646fb505`. Current CC prompt:
`arc10-turing-machine-midi/slice03-midi-output/cc-prompt.md`. Five M rows open.
The packet supplies concrete Rust data/API shapes, note/CC transitions, strict
validation, bounded output and pre-latch admission/partial completion, real RON
wiring and focused tests. Its runnable headless example is the next deliverable;
Slice04 adds actual endpoint playback. No additional design gate is needed
within directive08; CC implementation and independent CRC review are next.
