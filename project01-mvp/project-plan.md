# Project 01 — oxurack MVP

| Field | Value |
| --- | --- |
| project | project01-mvp |
| status | active — arc08/slice01 restart baseline; historical closures pending independent review |
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
