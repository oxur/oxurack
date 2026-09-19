# Project 01 — oxurack MVP

| Field | Value |
| --- | --- |
| project | project01-mvp |
| status | active — arc09 slices closed; CRC composition assembly awaiting independent CDC gate |
| depends-on | No other project required for the baseline MIDI rack; local Rust/Bevy/MIDI toolchain and test hardware |
| blocks | A reproducible modular MIDI recording workflow and an accepted MVP |
| related | underack; Lykn embedding (optional, externally gated); original v1 roadmap |
| source baseline | `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` on `main`; original migration baseline retained below |
| planning home | `planning` branch, `.worktrees/planning/project01-mvp` |

## Purpose and definition of done

Deliver a small, reproducible modular MIDI rack: assemble modules, route typed
signals, save/load seeded patches, drive an external synth or DAW, and operate
the rack live. No audio synthesis. Architecture remains documented in
[0005](artifacts/0005-oxurack-system-architecture.md).

For this best-effort migration, MVP retains the existing v1 capability envelope
from [the catalog/build roadmap](artifacts/0009-module-catalog-and-build-order-v2.md):
Turing Machine, Noise, LFO, Sequencer, Euclidean, Clock, Quantizer, Range,
Sample-and-Hold, Arpeggiator, and MIDI-out; deterministic persistence; master/slave
clocking; CLI/REPL; controller bindings; documentation and operational validation.
The catalog calls these 12 modules but includes MIDI-in, explicitly deferred to
v1.1. Treat MIDI-in as deferred, leaving 11 baseline rack modules.
This is a working scope reconstruction, not a fresh release commitment.

Completion requires a reproducible saved-patch performance, live editing and
controller interaction, demonstrated master/slave synchronization, measured
jitter against the legacy P99 <500 μs target on named hardware, actionable errors,
module documentation/examples, and an Operator-accepted musical recording session.
The original 95% coverage target, 24-hour soak, and 10/50/100-module benchmarks
remain tracked validation obligations; no result is inferred from old plans.
Release/tagging is a separate explicit Operator decision, not implied by the MVP name.

## Arc roadmap

Numeric order is a stable identifier. Dependencies determine execution order;
the RT and core foundations developed partly in parallel.

| Arc | Capability | Depends on | Migration assessment |
| --- | --- | --- | --- |
| [01 — RT and MIDI](arc01-realtime-midi/arc-plan.md) | MIDI I/O, queues, master/slave timing and transport primitives | Shared MIDI types; composes with 02 | Substantial historical implementation; hardware/timing acceptance unverified |
| [02 — Core and persistence](arc02-core-and-persistence/arc-plan.md) | ECS tick cycle, module infrastructure, parameters and patch round trips | 01 for bridge integration | Substantial historical implementation; integration and persistence gaps remain |
| [03 — Events and literate patching](arc03-events-and-literate-patching/arc-plan.md) | Observable changes and patches carrying labels, tags and prose | 02 | Historical notification/label substrate implemented; runtime edits remain |
| [04 — RackGraph](arc04-rackgraph/arc-plan.md) | Fresh rack navigation, diagnostics and parameter introspection | 02, 03 | v0 catalog appears implemented, including later getter follow-up |
| [05 — Public API](arc05-public-api/arc-plan.md) | Operate a rack through a coherent runtime API | 01–04 | Partial; proposed next active implementation arc |
| [06 — Module catalog](arc06-module-catalog/arc-plan.md) | Usable standalone and rack-integrated musical modules | 01, 02, 05 as needed | Standalone Turing Machine exists; adapter and other rack modules remain |
| [07 — Live interface](arc07-live-interface/arc-plan.md) | Patch CLI, REPL, inspection and controller bindings | 04–06 | Planned; no implementation-specific legacy document |
| [08 — Validation and release](arc08-validation-and-release/arc-plan.md) | Reproducibility, performance, usability and release readiness compose | 01–07 | Planned; no current acceptance claim |

No arc is newly marked closed. Historical implementation is useful evidence,
not an independently reproduced acceptance result. [The project ledger](ledger.md)
tracks composition; [the status and backlog register](artifacts/status-and-backlog.md)
accounts for the old inventory, remaining work, and explicit deferrals.

## Superseded migration restart suggestion

Begin by selecting a bounded arc05 runtime-operation slice with the Operator,
likely cable add/remove. The old inventory's first priority, bucket-getter, is
already represented by `7a6fcdb` and `d760049`; do not re-open it by accident.
Re-establish the workspace's actual validation baseline when implementation resumes.
The first usable-rack milestone also needs the Turing Machine adapter and MIDI-out
module in arc06. Detail those slices only when selected; this migration creates
no empty future slice directories or synthetic assignments.

## Boundaries and historical interpretation

GUI/TUI product work, web UI, plugin wrappers, MIDI 2.0, feedback cycles, parallel
execution, distributed racks, and dynamic module loading are outside the inherited
v1 envelope. Chord/module-state persistence and Lykn behavior are retained as
candidate extensions; they are not silently added to the MVP. See the backlog
for owners and re-entry conditions. Historical week estimates are not schedules.

Old plans and design memos are archaeology: their original text, links, document
states and implementation instructions remain intact, except rename history notes.
Their paths and supersession state are retained in [the manifest](artifacts/migration-manifest.tsv).
Read this roadmap, the implementation repository README, and Git history first.
Existing `workbench/` material was outside the requested migration and stays put.

## Superseded migration workflow and authority

The Operator explicitly selected One-Contributor mode on 2026-09-17: Sofie handles
design, implementation, and self-review in this conversation; Duncan is Operator.
This applies to project01-mvp until explicitly changed. Scope changes and existing
approval gates stay with the Operator. Self-checks are attested; no independent
review is claimed. No `cdc-verification.md`, `crc-verification.md`, or simulated
CC handoff is created. Future concrete slices receive current plans and ledgers;
use assignment prompts only for real handoffs if the workflow changes.

Layout authorization comes from the migration request: a dedicated planning
worktree, project01-mvp, document-backed slices, and artifacts at the closest scale.
The requested history-preserving exception uses filtered document ancestry instead
of an empty orphan commit. Original implementation history remains intact.
Retrospective slice ledgers remain open as reconciliation aids; they do not require
reimplementing completed work or inventing old acceptance records.

## Version History

- v1.0 — 2026-09-17: Initial reconstructed roadmap and layout, based on README,
  legacy docs and Git through `5cad90a`. Grouped eight arcs; corrected stale
  getter and module-status assumptions; retained future inventory items explicitly.

## Current plan — v2.0, 2026-09-17

The Operator requested reconciliation/closure of open slices and arcs, explicitly
allowed carried-over closure, and selected **Three-Contributor Workflow in
Expedited Mode**. This supersedes the one-contributor and cable-first suggestions
above. The existing planning layout is confirmed by the Operator's request and
on-disk migration; no parallel planning tree is introduced.

Active work: [arc08/slice01 restart baseline](arc08-validation-and-release/slice01-restart-baseline/slice-plan.md).
This uses the validation arc early to gather evidence; final arc08 acceptance
still depends on arcs01–07 and all inherited system gates. Arcs01–06 are carryover
closure candidates, not accepted capabilities. Six historical slices have explicit
candidate reports; 24 historical ledger rows remain open pending review.
Arc07 remains planned. No new product scope is selected by this restart.
The reconstructed MIDI MVP remains the scope baseline unless the Operator changes
it. Science-rig and Chord alternatives stay candidates; CDC/Operator settle the
next feature milestone after baseline evidence, before a feature assignment.

### Roles, evidence and approvals

- CDC: Sofie, current restart-planning conversation; project/arc design,
  first-slice preparation, design escalation and independent arc composition review.
- CRC: Sofie in the Operator-selected GPT-5.6 Sol forked review context; directive02
  acknowledged at planning commit `cc41597`; independent CC review, historical
  carryover review, and routine approved slice advancement.
- CC: separate Operator-selected execution context, not yet assigned; execute the
  named prompt and return evidence through Duncan. Context identity is recorded at
  handoff; this plan does not silently launch contributor sessions.
- Operator: Duncan; relay packets, decide scope/milestone and retain release,
  musical acceptance and any existing hardware/approval gates.

CC evidence is attested until CRC reproduces it. New slice reviews use
`crc-verification.md`; arcs/project require CRC plus independent CDC records.
CDC cannot accept its own carryover assembly: CRC checks the administrative
proposal before CDC considers a separately assembled arc packet. Existing
historical evidence is preserved without upgrading its strength.

Expedited Mode: each contributor commits its own explicit file list; never use
blanket staging. CC commits before review; CRC commits review/planning changes.
Preserve issued prompts and use numbered refinements, at most five executed
refinement passes. Close/advance when evidence permits; stop affected work at
independent review, unresolved design changes or Operator gates. No push, merge,
release, dependency modernization, hardware interaction or scope reduction is
implied by this documentation assignment.

### Transition state and design handoff history

Source `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`; planning predecessor `f0cf3a5172e02c717c7f924c84156f5fe0405733`. No current behavioral
acceptance or live CC assignment existed in the reconstructed tree. All original
project/arc/slice rows and findings remain tracked; no prior iteration budget is
invented. Initial active prompt is `arc08-validation-and-release/slice01-restart-baseline/cc-prompt.md`.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [cdc-directive01.md](cdc-directive01.md) | none; Operator restart request | superseded before CRC acknowledgement | R-01–R-06 and 24 inherited rows; carryover permitted, evidence gates retained | See directive02; no execution occurred under this packet |

### Version History — restart amendment

- v2.0 — 2026-09-17: Arc08 restart planning receives missing evidence and unresolved
  scope sequencing from arcs01–06/status; enables three contributors and Expedited
  Mode, prepares carryover candidates and first baseline slice. No MVP acceptance,
  timeline, scope reduction or release authority is inferred.

## Operator priority amendment — v2.1, 2026-09-17

The Operator selected **Turing Machine inside the Bevy rack producing MIDI first**,
then **Clock, LFO and Scope as a testing workbench**. This supersedes the unresolved
milestone choice above. Retain all other MVP capabilities as later work. Scope is
an explicitly selected diagnostic addition; a full TUI/GUI, Chord/state extension
or general cable-editing API is not automatically required for the first milestone.

The [dependency and execution map](arc08-validation-and-release/slice01-restart-baseline/artifacts/turing-machine-dependency-map.md) records source-supported hidden
prerequisites: module system installation, tick/edge/phase ownership, patch seed
handoff, ordered MIDI events and note cleanup, RT lifetime/backpressure, then bounded
Scope capture and a minimal observation interface. The next issued slice remains
arc08/slice01 baseline; it now validates these TM-first dependencies. Following
baseline, CDC details the smallest prerequisite implementation slice(s), then the
adapter and MIDI-out path. CRC may not invent the cross-arc API contracts.

Historical carryover candidates remain pending independent review. Capability
ownership remains with the named domains until a recorded successor assignment;
reopening the same arc with a new slice is allowed and does not resurrect an old
prompt. No original requirement is removed by focusing this sequence.

### Version History — priority amendment

- v2.1 — 2026-09-17: Operator response during arc08/slice01 preparation selected
  TM-to-MIDI, then Clock/LFO/Scope; dependency inspection showed execution/seed/event
  contracts must precede a useful adapter. Original broader backlog retained.

### Design handoff history — priority update

Current directive: `cdc-directive02.md`; current CC assignment:
`arc08-validation-and-release/slice01-restart-baseline/cc-prompt-iteration01.md`.
The preceding directive01/initial-prompt entries are preserved and superseded
before execution; neither was acknowledged or executed in this planning context.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [cdc-directive02.md](cdc-directive02.md) | cdc-directive01.md | acknowledged by CRC against source `a7ce6e6` and planning `cc41597` | R-03 focused; R-01–R-06 still open; Operator selected TM-first/Clock-LFO-Scope-second | Duncan relays iteration01 to CC; CRC reviews the returned commit and evidence |

CRC acknowledgement: the directive, dependency map, active assignment, six-row
ledger, and source/planning revisions agree. The active CC prompt remains
`arc08-validation-and-release/slice01-restart-baseline/cc-prompt-iteration01.md`.
No ledger row, historical carryover, implementation behavior, or hardware result
is accepted by this acknowledgement.

### CRC return — 2026-09-17

CRC accepted the restart observation slice, closed all six historical slices as
carried over with twelve deferred S rows, and assembled six arc packets with
twelve A rows still open pending independent CDC review. The new design and
quality findings are in [crc-escalation03.md](crc-escalation03.md). This return
does not create the next implementation slice or accept an arc capability.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [crc-escalation03.md](crc-escalation03.md) | cdc-directive02.md and CC `df6ec0a` | sent for Operator relay to CDC | R-01–R-06 done as evidence; 12 S rows deferred; 12 A rows open | CDC independently reviews six arc assemblies and designs TM prerequisite slice(s) |

## Current roadmap — maintained through v3.7, 2026-09-18

CDC reviewed CRC's actual `390d2aa` packet and resolved escalation03 through
[cdc-directive03.md](cdc-directive03.md). All six historical slices and arcs01–06
are now **closed, carried over**. Their 24 rows are deferred, not delivered;
[exact successor routing](artifacts/cdc-successors03.tsv) preserves every original
criterion and concrete re-entry. The earlier active/pending statements are history.
P-01–P-06 remain open. Arc08/slice01 is accepted as observation evidence only.

| Current owner | Scope / concrete next identifiers | State and dependency |
| --- | --- | --- |
| [Arc09 executable rack](arc09-executable-rack/arc-plan.md) | slice01-module-behavior-installation; slice02-simulation-samples; slice03-seeded-spawn-context | All slices CRC-closed; arc composition assembled, independent CDC gate pending |
| arc10-turing-machine-midi | slice01-musical-clock; slice02-turing-machine-adapter; slice03-midi-output; slice04-output-runtime; slice05-endpoint-acceptance | Planned after arc09; internal Clock is pulled forward because M1 needs a real pulse source |
| arc11-observation-workbench | slice01-lfo; slice02-scope-capture; slice03-workbench-integration | Planned after M1; enrich existing Clock, bounded scalar/event observation and minimal inspection |
| arc12-live-rack-operations | slice01-live-persistence; slice02-events-and-metadata; slice03-graph-composition; slice04-runtime-api | Retains remaining original arcs02–05 obligations after M1/M2; original requirements remain the input when detailed |
| arc13-remaining-module-catalog | slice01-catalog-reentry, then per-module slices selected from the retained catalog | Planned after M2; preserves the original eleven-module denominator, standalone compatibility, MIDI-in and extension deferrals |
| Arc07 live interface | Minimal workbench presentation, then full approved CLI/REPL/controller needs | Existing planned domain; no mandatory four-pane UI or broader feature expansion |
| Arc08 validation | slice02-coverage-provenance; slice03-coverage-hardening; slice04-hardware-timing; later system/soak/resource validation | Existing active validation domain; no global quality target is lowered |

Only arc09's issued slice directories/open sets have been created. Arc10–13 and unissued slice IDs
above are reserved roadmap destinations, not empty plans or implementation prompts.
Dependencies, not arc numbers, set execution order. Feature completion of old arcs
is not a prerequisite fiction: known source foundations can be used with exact
current validation, while missing acceptance remains explicitly tracked.

CDC chose the contracts in the [execution design](arc09-executable-rack/slice01-module-behavior-installation/artifacts/execution-contract.md).
Slice01 was bounded to behavior installation plus its strict documentation gate;
seed/time/MIDI are separate reviewable changes. The P99 target, 95% target,
benchmarks, soak and real musical acceptance remain project gates. M1 audible
sample-quantized playback does not claim final low-jitter release readiness.

### Quality dispositions

- Q-01: arc09/slice01 owns 11 observed rustdoc diagnostics (3 unresolved, 8
  redundant) and its new API docs. CDC reproduced the failure; preserve earlier
  two-link reports as observations, not the complete error inventory.
- Q-02: arc08/slice02-coverage-provenance owns the nine profile mismatches, precise
  tool/source/denominator accounting and trusted reproducibility. Start before
  using coverage to select hardening work; no exclusions or warning suppression.
- Q-03: arc08/slice03-coverage-hardening owns behavior-based coverage remediation
  against the unchanged 95% floor after Q-02. CRC's 93.47% remains a failing,
  qualified observation. New code must have meaningful tests immediately.

### Design handoff history — current

Current directive: `cdc-directive03.md`. Current CC assignment:
`arc09-executable-rack/slice01-module-behavior-installation/cc-prompt.md`.
Baseline iteration01 is accepted/complete as observation work, not a live assignment.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [cdc-directive03.md](cdc-directive03.md) | crc-escalation03.md / directive02 | issued; awaiting CRC acknowledgement | 12 A rows now deferred; 24 exact successors; E-01–04 and I-01–06 newly open | Duncan relays to CRC; CRC acknowledges then relays the initial implementation prompt to CC |

### Version History — CDC design return

- v3.0 — 2026-09-17: CRC escalation03 and accepted baseline led to independent
  carried-over arc closure, concrete successor arcs/slices, executable-rack
  contracts and first implementation assignment. CDC rustdoc reproduction enlarged
  Q-01's known failure set; original quality/capability obligations retained.

## Prompt-authoring amendment — v3.1, 2026-09-17

Current directive: [cdc-directive04.md](cdc-directive04.md), initiated by Duncan's
request to apply the newly installed implementation-prompt-authoring guide. CRC
now prepares a detailed preserved replacement for arc09/slice01's initial prompt
before CC relay/execution. Prior current-assignment statements describe historical
handoffs; the initial prompt is held and no replacement is issued yet.
Directive03's architecture, successor ownership, criteria and validation still govern.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | cdc-directive04.md | cdc-directive03.md | acknowledged by Sofie/CRC at planning `de337fa4ff0833af689bce6dd90efc4aaeabf3f1` and clean source `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` | I-01–I-06 remain open; Operator requested improved authoring | CRC authors preserved iteration01, records six readiness results, then Duncan relays it to CC |

CRC acknowledgement: directive03's architecture and successor routing remain
binding; directive04 replaces only its instruction to relay the held initial CC
prompt unchanged. The installed collaboration-framework is version 2.4.0 (newer
than directive04's 2.3.0 reference); the required authoring guide/template and
project-management routes are present. No CC execution packet or source change
exists at this baseline. Current source and planning worktrees are clean before
CRC's prompt-authoring edits.

- v3.1 — 2026-09-17: Added guide-based CRC authoring assignment and held the earlier
  CC handoff. No behavioral contract, ledger status or acceptance gate changed.

## CRC prompt-authoring return — v3.2, 2026-09-17

Sofie/CRC issued the preserved [arc09/slice01 iteration01 replacement](arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration01.md)
for Duncan to relay to CC. The original `cc-prompt.md` was never executed and stays
held; no executed correction pass is consumed (0 of 5). The [slice-plan author
readiness record](arc09-executable-rack/slice01-module-behavior-installation/slice-plan.md#authoring-amendment--2026-09-17)
covers source, design, guide application, execution route, test oracles and
portability. I-01–I-06 remain open for implementation and later independent CRC
review. Directive04 governs this relay; directive03 and the execution contract
still govern behavior. No source, acceptance or quality gate changed.

- v3.2 — 2026-09-17: Issued detailed first-execution CC prompt after the
  guide-based authoring check; retained all six open implementation rows.

## CRC slice01 review handoff — v3.3, 2026-09-17

Current exchange: [crc-escalation05.md](crc-escalation05.md), awaiting CDC through
Duncan. CRC independently reviewed source `d43078ea9c71bba10035699c9914bef3891f1d83`
and planning evidence through `2da485ebad1c685b847bcebf954921dd0aecedfa`.
The [slice verification](arc09-executable-rack/slice01-module-behavior-installation/crc-verification.md)
reproduced I-01/I-03/I-04; I-02/I-05/I-06 remain open. The unchanged
all-targets Clippy gate exposes two baseline test-code warnings beyond the
current source allowlist, while the historical planning verifier rejects the
authorized current assignment. CDC is asked to authorize exact repairs, not
to lower either gate. No arc09/slice02 assignment is issued pending the response.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [crc-escalation05.md](crc-escalation05.md) | cdc-directive04.md / executed iteration01 | awaiting CDC through Duncan | I-02/I-05/I-06 open; source/checker scope decision | CDC answers with directive05; CRC acknowledges and prepares bounded correction |

- v3.3 — 2026-09-17: Recorded CRC partial verification and escalation for
  exact gate-repair authority; preserved arc09 and project quality contracts.

## CDC scope return — v3.4, 2026-09-17

Current directive: [cdc-directive05.md](cdc-directive05.md), answering
[escalation05](crc-escalation05.md). D1 approves two exact test-only Clippy
repairs; D2 approves the current-assignment verifier correction without dropping
any structural checks; D3 authorizes CRC's detailed iteration02 combining those
with I-02's after-edit reinstall oracle and I-06's provenance/bubble-up corrections.
CDC reproduced both failing gates; no additional implementation acceptance.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | cdc-directive05.md | crc-escalation05.md / directive04 | acknowledged by Sofie/CRC against clean planning `dcab7931f8379380e0054c9b44b7b7d82b9be597` and source `d43078ea9c71bba10035699c9914bef3891f1d83` | I-02/I-05/I-06 open; exact scope amendment, unchanged gates | CRC authors iteration02, CC repairs, CRC independently verifies before slice02 |

- v3.4 — 2026-09-17: Bounded corrective scope approved; no architectural or
  quality-target change, no slice/arc close. Previous pending-CDC status is history.

CRC acknowledgment: D1 authorizes only the named graph/bridge test-code
normalizations; D2 authorizes CC to repair the current-assignment checker while
retaining every existing structural check; D3 authorizes one combined corrective
assignment after guide-based authoring. The source and planning states above
match directive05's observed candidate. No CC iteration02 work is present yet;
CRC's prior I-01/I-03/I-04 verdict remains candidate-specific, and no gate or
target has been waived.

## CRC corrective handoff — v3.5, 2026-09-17

Sofie/CRC has issued [arc09/slice01 iteration02](arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration02.md)
under directive05 for Duncan to relay to CC. The [slice-plan author readiness](arc09-executable-rack/slice01-module-behavior-installation/slice-plan.md#crc-corrective-authoring-and-readiness--v15-2026-09-17)
records six checks and a single canonical current-assignment field. Source
remains clean at `d43078ea9c71bba10035699c9914bef3891f1d83`; I-02/I-05/I-06
remain open, and the three earlier done rows are tied to that prior candidate.
The next CC execution will be the first corrective pass (1 of 5 when run), not
an acceptance or arc09/slice02 advancement.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [cc-prompt-iteration02.md](arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration02.md) | cdc-directive05.md / iteration01 | issued for relay, not executed | I-02/I-05/I-06 correction within approved scope | Duncan relays to CC; CC returns committed source/planning evidence for CRC review |

## CRC slice01 close and slice02 issue — v3.6, 2026-09-17

Sofie/CRC independently reviewed CC's clean `6a31c0f` corrective source and
planning evidence `0f4561b`/`4fb9f2f`. [Slice01 verification](arc09-executable-rack/slice01-module-behavior-installation/crc-verification.md#iteration02-independent-close)
closes I-01–I-06. The after-edit reinstall oracle reaches `(4,6)` before
reinstall and `(8,9)` after one further fixed run. Full all-targets Clippy,
strict docs, format, planning verification and host-access workspace/make
checks pass. Restricted MIDI/timing and DNS failures remain recorded alongside
successful host retries; three RT tests remain ignored. The inherited 95%
coverage, physical MIDI and hardware timing gates remain with their existing
owners. No arc09 sequence or scope amendment was required.

CRC opened [arc09/slice02](arc09-executable-rack/slice02-simulation-samples/slice-plan.md)
within the existing CDC execution-contract §2. The [initial prompt](arc09-executable-rack/slice02-simulation-samples/cc-prompt.md)
defines the exact 1 ms sample API, five-phase pipeline, manual/live transitions,
raw-core compatibility and sample-indexed traces. Its six S rows are open;
arc09/slice03 and arc close remain future work. Current CC assignment relative
to project01-mvp:
`arc09-executable-rack/slice02-simulation-samples/cc-prompt.md`.

- v3.6 — 2026-09-17: Independently closed the first executable-rack slice and
  issued the next approved sample/phase slice with guide-authored prompt.

- v3.5 — 2026-09-17: Issued the bounded corrective assignment after directive05
  acknowledgment and six-part author readiness; no gate or source behavior changed.

## CRC slice03 close and arc09 composition handoff — v3.7, 2026-09-18

Sofie/CRC independently reviewed clean source `642313a` and CC planning evidence
`698d2d0`/`cc35c4b`. All six slice03 rows are done: the fixed FNV-1a byte vectors,
ModuleId migration, pre-construction context, legacy registration compatibility,
real core/umbrella patch paths and two-child-process replay reproduce. Package,
both host workspace feature matrices, Clippy, format, strict docs and host
`make check` pass. Three pre-existing RT tests remain ignored; physical MIDI,
timing, coverage and musical acceptance are not claimed.

All three arc09 slices are now CRC-accepted. CRC assembled the formal
[arc closing report](arc09-executable-rack/closing-report.md),
[operational verification](arc09-executable-rack/crc-verification.md), and
[composition ledger](arc09-executable-rack/ledger.md). E-01–E-04 remain open
until CDC independently verifies this assembly. No structural project-plan
change was discovered. Arc10 and its first slice remain CDC-owned and unopened
until that gate passes.

- v3.7 — 2026-09-18: Closed slice03 after independent CRC reproduction and
  assembled arc09 composition for CDC review; retained all successor and
  project acceptance gates.
