---
number: 14
title: "Open Work Inventory — Post-Bucket-4 Snapshot"
author: "Duncan McGreggor"
component: All
tags: [roadmap,inventory]
created: 2026-05-13
updated: 2026-05-13
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# Open Work Inventory — Post-Bucket-4 Snapshot

## Purpose

This document consolidates all open work items across the project as of bucket 4's closure (2026-05-13, commit `5c49283`). It pulls residual items from each open design doc plus the workbench's `deferred-tracking.md` registry into a single view, so the question *"what's left?"* has one answer instead of seven scattered ones.

It is **not** a re-specification of any of the source docs. Each entry points back to its source. The value of this doc is the **consolidated view**, not redundant detail.

**Companion to:** `workbench/deferred-tracking.md` (per-bucket deferral registry; persists across bucket reviews; finer granularity).

---

## What's Shipped (context for what's open)

Buckets 3, 3.25, 3.5, 4 are all complete:

- **Bucket 3** (`8e9652c`) — `CoreCommand::SetParameter` dispatch via `PatchHandle` + `ParameterRegistry`.
- **Bucket 3.25** (`229b616`) — Event substrate completed: `ModuleAdded`/`ModuleRemoved`/`CableConnected`/`CableDisconnected`/`ParameterChanged` notification tier on top of the existing `CoreCommand`.
- **Bucket 3.5** (`a894e0a` + `ce10c39`) — Literate patching: `DisplayName`/`Description`/`Tags`/`Tag`/`TagError` components and newtype, `RackMeta` resource, `DisplayNameFormatter` resource with synth-domain acronym table, `LabelChanged` event, patch-file format extension with `RackConfig`.
- **Bucket 4** (`9a0dda4` + `fc0e000` + `5c49283`) — `oxurack-graph` crate: `RackGraph` resource, `GraphView` query surface, `GraphMaintenance` schedule + freshness contract, 17 of 18 v0 queries plus diagnostic queries (dangling, cycles, topo, closure, port-compat).

Plus the foundational work that landed before bucket 3 (oxurack-core, oxurack-rt, oxurack-mod-turingmachine, oxurack-midi).

---

## Open Items — Organized by Area

### A. Cross-cutting infrastructure (high-leverage)

**A1. Bucket-getter** *(unblocks two items)*
Add `get_parameter` to `ParameterRegistry` symmetrically with `set_parameter`. Requires extending `OxurackModule` trait (or a parallel registration path) to expose getters per parameter.
*Unblocks:*
- `ParameterChanged.previous` population (currently always `None`; `TODO(bucket-getter)` markers at `event.rs:355` and `patch.rs:506`).
- `GraphView::knobs_of(entity)` — the only deferred query from bucket 4's v0 catalog.
*Source:* `deferred-tracking.md` D-25.1.

**A2. Runtime label-mutation `CoreCommand` variants**
Add `SetDisplayName` / `SetDescription` / `SetTags` / `SetNote` (and dispatch). Bucket 3.5 shipped the substrate (components, formatter, patch-file format, patch-load emission) but no runtime mutation commands.
*Source:* `deferred-tracking.md` D-35.1.

**A3. Cable ops runtime API** *(2 closely-related items)*
Implement runtime `add_cable` / `remove_cable` dispatch (currently `NotImplemented`). Today, cables only get added during `apply_patch_to_world`. Once the dispatch exists, the existing `CableConnected` / `CableDisconnected` notification types start firing at runtime, not just during patch load.
*Source:* design doc 0010 §"Cable ops bucket"; `event.rs:296-297` `NotImplemented` arms.

**A4. Transport API: start / stop / pause / resume**
Four verbs mirroring MIDI START / STOP / CONTINUE plus `pause`. The RT thread handles transport messages already (per design doc 0006); what's missing is the `Oxurack`-level API and `TransportState` resource on the ECS side.
*Source:* design doc 0010 §Locked decisions; the original "Bucket 3.5" naming was for this work but the name was repurposed for literate patching.

**A5. Emergency `reset()` (All-Notes-Off)**
Narrow-scope reset: All-Notes-Off to every module + clear pending command queue. Does not touch knobs, cables, transport, or power.
*Source:* design doc 0010 §Locked decisions.

**A6. `PatchUnloaded` notification**
Type declared in 3.25? Actually deferred (no unload exists). Lands when an `unload_patch` API exists.
*Source:* `deferred-tracking.md` D-25.6.

---

### B. Public API — handle types and surface styles

**B1. Handle API: `rack.module::<M>(name) → Handle<M>`**
With `.port()`, `.knob()`, `.set()` methods. The v0 plan for surface Style B.
*Source:* design doc 0010 §Locked decisions; not shipped.

**B2. Per-module enum vocabularies**
Each module defines its `Port` enum and `Knob` enum. Enables Styles B/C ergonomics.
*Source:* design doc 0010 §Confirmed naming decisions; not shipped.

**B3. Value enums in `oxurack::values`**
Domain-specific value enums (`Waveform::Sine`, etc.) with `From` impls into `ParameterValue` so `set_knob` accepts both scalars and enum variants.
*Source:* design doc 0010 §Locked decisions; not shipped.

**B4. Style C / D ergonomic surfaces**
`rack.patch((module, port), (module, port))`, `rack.set_knob(module, knob, value)`, string-path variants for loaded-patch and REPL use.
*Source:* design doc 0010 §Appendix A.

---

### C. Module catalog (10 of 12 modules unshipped)

Existing crates: `oxurack-mod-turingmachine`, `oxurack-midi`.

Catalog from design doc 0009 §3, status by module:

| # | Module | Status |
|---|--------|--------|
| 3.1 | turingmachine | shipped |
| 3.2 | noise | not shipped |
| 3.3 | lfo | not shipped |
| 3.4 | sequencer | not shipped |
| 3.5 | euclidean | not shipped |
| 3.6 | clock | not shipped (RT-side master-clock exists in oxurack-rt; the *module-tier* clock module that exposes it as a normal rack module hasn't shipped) |
| 3.7 | quantizer | not shipped |
| 3.8 | range | not shipped |
| 3.9 | sample-hold | not shipped |
| 3.10 | arpeggiator | not shipped |
| 3.11 | midi-out | shipped via `oxurack-midi` |
| 3.12 | midi-in (v1.1) | partial — depends on what's in `oxurack-midi` |

Plus the proposed **chord module** (Duncan, 2026-04 conversation) — not in catalog 0009 because it postdates it; the use case that drove the Lykn integration design. Specifies user-input state (key, mode, progression, voicing) beyond what knobs can express; first concrete consumer of a `PersistableModuleState` trait if/when that lands.

*Source:* design doc 0009 §3; chord module from 2026-04 conversation logs.

---

### D. Graph (post-bucket-4 follow-ups)

**D1. `knobs_of(entity)` query**
The only deferred query from bucket 4's v0 catalog. Gated on bucket-getter (A1).
*Source:* `rackgraph-design.md` reconciliation; `deferred-tracking.md` D-25.1.

**D2. `CableKind` (Signal vs Modulation) enum**
Cables are currently undifferentiated. When modulation cables (port-to-knob) become a thing, `RackEdge` grows a `kind` field; graph queries split into kind-aware variants.
*Source:* `deferred-tracking.md` D-25.7; `rackgraph-design.md` reconciliation.

**D3. Cross-type emission ordering test (stronger)**
Bucket 3.25's `test_patch_load_notification_ordering` relies on a "structural guarantee" (sequential `write_message` calls). A monotonic-counter test would catch refactor regressions silently dropped today. Possibly subsumed by bucket-4 end-to-end graph tests.
*Source:* `deferred-tracking.md` D-25.2.

---

### E. Substrate event-tier extensions

**E1. `ParameterSetFailed` failure event**
Notification emitted on `SetParameter` `Err`. Useful for audit log and UI feedback. Non-breaking add later.
*Source:* `deferred-tracking.md` D-25.3.

**E2. Audit-log subscriber**
Demonstrator of the notification substrate's value. Records every notification with timestamp for replay/forensics.
*Source:* `deferred-tracking.md` D-25.4.

**E3. `serde::Serialize` on notifications**
For future cross-process bridge (out-of-process TUI via IPC). Cheap to add when needed; notifications use only owned data.
*Source:* `deferred-tracking.md` D-25.5.

**E4. Dependency-graph command ordering**
Replace flat ordered command queue with DAG-based scheduling. Earned-its-keep when concurrency forces it (script batches, cross-process producers, replay determinism). Adding `depends_on` metadata to `CoreCommand` later is a non-breaking enum extension.
*Source:* `event-substrate-design.md` Appendix C; `deferred-tracking.md` D-25.9.

---

### F. Literate-patching extensions

**F1. Port-level instance labels**
`DisplayName` / `Description` on port entities. Schema-level port labels (kind-level) exist; per-instance overrides are speculative.
*Source:* `deferred-tracking.md` D-35.2.

**F2. Tag validation extensions**
Length limits, internal-whitespace rules, case normalization. v0 validates non-empty + no leading/trailing whitespace. Newtype is the seam for additional rules.
*Source:* `deferred-tracking.md` D-35.3.

**F3. Hierarchical tags** (`audio/lead/voice`)
Speculative. Flat tags are sufficient for v0.
*Source:* `deferred-tracking.md` D-35.4.

**F4. Note IDs + per-note CRUD events**
Per-note add/edit/remove granularity. v0 emits a single `LabelChanged` with full notes list at patch load.
*Source:* `deferred-tracking.md` D-35.5.

**F5. Tag search / filter machinery**
Query API: "find modules tagged X." Tags stored today; consumer queries come when REPL or TUI bucket lands.
*Source:* `deferred-tracking.md` D-35.6.

**F6. Acronym table persistence**
`DisplayNameFormatter` extensions survive sessions. Lands alongside other user-config work.
*Source:* `deferred-tracking.md` D-35.7.

---

### G. Module-state persistence (the chord-module driver)

**G1. `PersistableModuleState` trait**
General mechanism for modules to expose non-knob state that round-trips through patch save/load. Chord module's progression/voicing/key is the first motivating example. The trait sits in `oxurack-core` alongside `Module`/`ModuleKind`.
*Source:* 2026-04 conversation logs; bucket 3.5 design memo's "post-3.25/3.5 reconciliation" section notes the two layers (persistable state + Lykn-as-behavior).

**G2. Patch-file format extension for arbitrary module state**
Whatever serialized blob `PersistableModuleState::serialize` produces gets carried in the patch file alongside knob values. Format design TBD.
*Source:* same.

---

### H. Lykn integration (gated on external project)

**H1. Lykn Rust embedding API**
Gated on Liaozhai MUX (per the 2026-04 conversation) and the Lykn project shipping a Rust-embeddable interpreter. oxurack would consume the same interpreter Liaozhai uses.

**H2. Lykn-as-module-behavior**
Once H1 ships, modules can carry user-authored Lykn forms as part of their `PersistableModuleState` (G1). The chord-module's "transformation function" is the canonical example.

*Source:* `lykn-embedding-requirements-from-oxurack.md` in workbench.

---

### I. Far-future architecture (deferred past v1)

From design doc 0005 §"Open Questions":

- **BatchGenerator pattern** — decide placement (core vs. per-module utility) when first generator needs it.
- **Hot-reload of patches** — delta-reload diffing for live performance (v2).
- **DebugInspector** — live port-value observation (TUI/egui/REPL — application-layer concern).
- **Port aliases** — flat names suffice for v1; aliases v2.
- **`Raw(u16)` usage audit** — drop before 1.0 if no module uses it.
- **VST/AU/CLAP plugin hosting** — architecturally permitted, deferred past v1.
- **MIDI 2.0** — deferred past v1.
- **Distributed / networked racks** — deferred past v1.

---

## Source Doc Status

Audit of each Draft/Active design doc and recommended next state.

| Doc | Title | Current | Recommendation |
|-----|-------|---------|----------------|
| 0005 | Oxurack System Architecture | Draft | **Light refresh → Final.** Mostly descriptive of shipped state; Open Questions captured above. |
| 0007 | oxurack-core: ECS World, Values, Ports, Cables | Draft | **Light refresh → Final.** Core is built; Open Questions captured above. |
| 0008 | Module Infrastructure: Authoring Modules on Bevy | Draft | **Light refresh → Final.** Infrastructure is built; conventions documented. |
| 0009 | Module Catalog and Build Order (v2) | Draft | **Keep as Draft.** 10 of 12 modules unshipped; doc is still the right spec for them. Possibly refresh as "v3" with current status. |
| 0010 | oxurack Public API Design (v0) | Draft | **Keep as Draft.** Locked decisions partially shipped (renames, set_knob, into_app); handle API + transport + reset + cable runtime ops + value enums all unshipped. *Watch:* the doc's "Bucket 3.5: lifecycle+transport" plan was renumbered when "3.5" got used for literate patching — implementation queue needs renumbering. |
| 0011 | RackGraph — Design Memo (v0) | Active | **Light refresh (lines 428, 439) → Final.** Architecture is realized; line 439 still says "Up next" for Bucket 4 which has shipped. |

**No docs recommended for Superseded.** All Drafts are coherent specs against which future work can be implemented; none have drifted enough to require rewriting. The two with substantial unshipped scope (0009 catalog, 0010 API) need *implementation*, not *re-specification*.

---

## Suggested Priority Sketch

If you wanted to convert this inventory into an ordered roadmap, the highest-leverage moves first:

1. **Bucket-getter (A1)** — small, focused, unblocks `ParameterChanged.previous` *and* `knobs_of` (D1). Closes two long-running TODOs in one move.
2. **Chord module + `PersistableModuleState` (G1, G2)** — the first concrete use case for state-beyond-knobs. Drives the persistence trait into existence; tests it against a real module.
3. **Cable ops runtime API (A3)** — closes two `NotImplemented` arms in `dispatch_core_command`; the existing `CableConnected`/`CableDisconnected` notifications start firing at runtime, not just during patch load.
4. **Handle API + per-module enums (B1, B2)** — Style B/C ergonomics. Makes the REPL substantially nicer.
5. **Transport + reset (A4, A5)** — finishing the `Oxurack` lifecycle API. Modest scope, completes a coherent surface.
6. **Module catalog expansion (C)** — start working through 0009's 10 unshipped modules. LFO and clock-module first (per the catalog phase ordering).
7. **Audit-log subscriber (E2)** — small demonstrator that validates the notification substrate. Useful for development workflow even before any production use case.

Lower-leverage but still tracked: everything in §E (event extensions beyond audit log), §F (literate-patching extensions), §I (far-future architecture).

Gated on external work: §H (Lykn integration) waits on the Lykn project's Rust-embeddable interpreter.

---

## Lineage and Cross-References

- Source design docs: 0005, 0007, 0008, 0009, 0010, 0011 in `docs/design/` (numbering and state per `index.md`).
- Per-bucket deferral registry: `workbench/deferred-tracking.md` (D-25.x and D-35.x items).
- Bucket plans: `docs/dev/0001`–`0007`.
- Lykn embedding requirements: `workbench/lykn-embedding-requirements-from-oxurack.md`.
