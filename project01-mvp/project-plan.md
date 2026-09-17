# Project 01 — oxurack MVP

| Field | Value |
| --- | --- |
| project | project01-mvp |
| status | active — resuming; historical implementation requires re-baselining |
| depends-on | No other project required for the baseline MIDI rack; local Rust/Bevy/MIDI toolchain and test hardware |
| blocks | A reproducible modular MIDI recording workflow and an accepted MVP |
| related | underack; Lykn embedding (optional, externally gated); original v1 roadmap |
| source baseline | `5cad90a6b8904a3ae615bcbd9ed3e277bd94d95a` on `main` |
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

## Recommended restart

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

## Workflow and authority

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
