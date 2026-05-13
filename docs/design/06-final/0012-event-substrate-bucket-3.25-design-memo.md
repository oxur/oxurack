---
number: 12
title: "Event Substrate (Bucket 3.25) — Design Memo"
author: "Duncan McGreggor"
component: All
tags: [change-me]
created: 2026-05-10
updated: 2026-05-12
state: Final
supersedes: null
superseded-by: null
version: 1.1
---


# Event Substrate (Bucket 3.25) — Design Memo

**Status:** Draft
**Date:** 2026-05-10
**Type:** Design memo (precedes CC-facing implementation spec)
**Origin:** [event-substrate-brainstorm.md](./event-substrate-brainstorm.md), six open questions converged 2026-05-10.

---

## Confirmed decisions

| # | Decision | Source |
|---|----------|--------|
| D1 | Two-tier architecture: **Commands** (existing `CoreCommand`) + **Notifications** (this bucket completes). | Q2 + queue-of-operations insight |
| D2 | Emission model: **A-everywhere**. Every mutation path explicitly emits the corresponding notification. No reactor systems based on `Changed<T>` for 3.25 events. | Q2 |
| D3 | Batch semantics: **individual notifications + summary, summary last**. Subscribers select what they care about. | Q3 |
| D4 | Six notification types added in 3.25: `ModuleAdded`, `ModuleRemoved`, `CableConnected`, `CableDisconnected`, `ParameterChanged`, plus extension to existing `PatchLoaded`. | Q4 |
| D5 | `PatchUnloaded` **deferred** to whichever bucket implements unload — no emission site exists today. Verified 2026-05-10: no `unload_patch` / `clear_patch` / `reset_patch` in codebase; `CoreCommand::RemoveCable` returns `NotImplemented`. | Q5 |
| D6 | Test discipline: positive-path + `matches!`-with-field-checks regime carries forward from Bucket 3 verbatim. No regressions. | Q6 |
| D7 | Notifications use `Message` derive, consistent with existing notifications (`CoreCommand`, `PatchLoaded`, `RtWarning`, `SongPositionChanged`, `TransportChanged`, `MidiInReceived`). Bevy 0.16+ convention. | Codebase audit |
| D8 | All new notifications live in `oxurack-core::event`, exported alongside existing types. | Architectural |

---

## Context: half the architecture already exists

oxurack already has half the architecture this memo formalizes. `CoreCommand` exists as the command tier; `PatchLoaded`, `RtWarning`, `SongPositionChanged`, `TransportChanged`, and `MidiInReceived` exist as notification-tier events. The doc-comment on `CoreCommand` literally describes the queue-of-operations pattern: *"Sent from the REPL, UI, or scripting layer. Processed by the command handler system during the next tick."*

What's missing is the **structural-mutation notification tier** — events that fire when the rack's structure (modules, cables) or knob state changes. Bucket 3.25 adds those events and completes the two-tier picture.

The architectural pattern is **CQRS-shaped** (Command Query Responsibility Segregation), with the command-and-event lineage that finance/AMQP-driven trading systems have run in production for decades. Commands describe user intent (imperatives, "do this"); notifications describe facts (declaratives, "this happened"). Producers of one are not the same as consumers of the other; each tier scales independently.

**Structural payoff**, made explicit because it's load-bearing for downstream buckets:

- **Audit log = persist the command stream.** Replay a session by re-running its commands.
- **Undo = invert each command in the stream.** Per-command, not per-notification.
- **Cross-process = serialize commands over IPC.** Future out-of-process TUI talks to headless rack via the command stream.
- **Determinism via ordering.** Commands serialize through one processor; notifications come out in deterministic causal order.
- **REPL is a command producer**, REPL display is a notification consumer.
- **Lykn handlers are notification consumers** (`on ParameterChanged`); Lykn scripts that mutate the rack are **command producers** via the host-effects API.
- **The graph (bucket 4) is just one notification consumer.** TUI, audit log, future Lykn, future cross-process bridges are all peer consumers reading the same stream.

This memo's scope is *closing out the notification tier so the substrate is complete*. Consumers come in later buckets.

---

## The two tiers, formalized

### Command tier (existing — recap)

```rust
#[non_exhaustive]
#[derive(Message, Debug, Clone)]
pub enum CoreCommand {
    LoadPatch(PathBuf),
    SavePatch(PathBuf),
    SetParameter { module: String, param: String, value: ParameterValue },
    AddCable { source: (String, String), target: (String, String), transform: Option<CableTransform> },
    RemoveCable { source: (String, String), target: (String, String) },
    SetBpm(f32),
    Panic,
}
```

- **Producers:** REPL, umbrella crate command handler, future scripting layer.
- **Consumer:** `dispatch_core_command` in `event.rs`. One consumer; serial processing.
- **Ordering:** Insertion order through `Messages<CoreCommand>` is preserved per buffer drain.
- **Status:** Implemented for `SetParameter`, `Panic`, `SetBpm`. `LoadPatch` / `SavePatch` / `AddCable` / `RemoveCable` return `NotImplemented`.

### Notification tier (this bucket completes)

**Existing notifications** (do not modify the type, but do extend `PatchLoaded`):

- `PatchLoaded` — emitted on patch load. Currently carries only `patch_name: String`. Extended in this bucket; see below.
- `MidiInReceived` — RT-side notification. No changes.
- `RtWarning { code: RtWarningCode }` — RT-side warning. No changes.
- `SongPositionChanged { position: u16 }` — MIDI clock notification. No changes.
- `TransportChanged { state: TransportState }` — transport state change. No changes.

**New notifications added in 3.25:**

```rust
/// Emitted when a module entity is added to the rack.
///
/// Producers: `apply_patch_to_world` (during patch load); future
/// `install_module`-style API surfaces.
#[derive(Message, Clone, Debug)]
pub struct ModuleAdded {
    /// The ECS entity for the new module.
    pub entity: Entity,
    /// User-facing instance name (key in `PatchHandle::modules`).
    pub instance_name: String,
    /// Module kind (matches the `Module` component's `kind` field).
    pub kind: ModuleKind,
}

/// Emitted when a module entity is removed from the rack.
///
/// Producers: future `remove_module` API (does not exist today).
/// 3.25 declares the type but no emission site fires until the API
/// lands. Consumers can subscribe now without breakage.
#[derive(Message, Clone, Debug)]
pub struct ModuleRemoved {
    pub entity: Entity,
    pub instance_name: String,
    pub kind: ModuleKind,
}

/// Emitted when a cable entity is added to the rack.
///
/// Producers: `apply_patch_to_world` (during patch load); future
/// `dispatch_add_cable` (currently `NotImplemented`).
#[derive(Message, Clone, Debug)]
pub struct CableConnected {
    /// The ECS entity for the new cable.
    pub cable_entity: Entity,
    /// Source port entity (from `Cable::source_port`).
    pub source_port: Entity,
    /// Target port entity (from `Cable::target_port`).
    pub target_port: Entity,
    /// Optional inline transform (from `Cable::transform` if present).
    pub transform: Option<CableTransform>,
}

/// Emitted when a cable entity is removed from the rack.
///
/// Producers: future `dispatch_remove_cable` (currently
/// `NotImplemented`). Same reasoning as `ModuleRemoved` re. emission
/// site availability.
#[derive(Message, Clone, Debug)]
pub struct CableDisconnected {
    pub cable_entity: Entity,
    pub source_port: Entity,
    pub target_port: Entity,
}

/// Emitted after a parameter is successfully written via the registry.
///
/// Producers: `dispatch_set_parameter` after the registry call
/// succeeds. Future direct-mutation paths must also emit (the
/// "every mutation site" obligation).
///
/// `old` and `new` are both carried so consumers (audit log, undo,
/// diff renderers) get the full delta in one event.
///
/// Naming note: this is `ParameterChanged`, not `ParameterChanged`,
/// because parameters have multiple form factors — knobs, switches,
/// sliders, MIDI-CC inputs, modulation destinations are all
/// parameters. The notification name describes the substrate, not
/// the UI affordance.
#[derive(Message, Clone, Debug)]
pub struct ParameterChanged {
    /// Module entity whose parameter changed.
    pub entity: Entity,
    /// User-facing instance name.
    pub instance_name: String,
    /// Parameter name (registry key).
    pub param: ParameterName,
    /// Value before the change.
    pub old: ParameterValue,
    /// Value after the change.
    pub new: ParameterValue,
    /// What caused the change (provenance).
    pub source: ChangeSource,
}

/// Provenance for a `ParameterChanged` notification.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum ChangeSource {
    /// Direct user/script command via `CoreCommand::SetParameter`.
    Command,
    /// Direct API mutation (future API surfaces).
    DirectApi,
    /// Patch-load initialization (one `ParameterChanged` per knob set during load).
    PatchLoad,
}
```

**Extension to existing `PatchLoaded`:**

```rust
/// Emitted *last* in a patch-load batch, after all individual
/// `ModuleAdded` and `CableConnected` notifications.
///
/// Semantically: "the patch is now in place, redraw if you redraw."
/// Subscribers who don't care about granular changes can subscribe
/// only to this event.
#[derive(Message, Debug, Clone)]
pub struct PatchLoaded {
    /// Existing field — preserved for backward compatibility.
    pub patch_name: String,
    /// Where the patch came from.
    pub source: PatchSource,
    /// How many modules were instantiated.
    pub module_count: usize,
    /// How many cables were instantiated.
    pub cable_count: usize,
}

/// Provenance for a `PatchLoaded` notification.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum PatchSource {
    /// Loaded from a file path.
    File(PathBuf),
    /// Built in memory via direct API or test fixture.
    InMemory,
    /// Other / future-proofing.
    Other(String),
}
```

The extension preserves the existing `patch_name` field; new fields are additions. No existing consumer of `PatchLoaded` breaks (modulo construction-site updates).

---

## Emission discipline (D2 — A-everywhere)

**Every mutation site emits.** No reactor systems based on `Changed<T>` or `Added<T>` for 3.25 notifications.

Rationale:

- **Strict ordering preserved.** Notifications appear in exact mutation order. Reactor systems batch in `PostUpdate` and lose the ordering.
- **`ChangeSource` is load-bearing.** Only the emission site knows the source — a reactor sees that `KnobState` changed but not whether a user moved the knob, a patch loaded, or a future LFO modulated it. Adding source provenance later is a breaking change for every consumer; adding it now costs one enum variant per emission site.
- **One discipline rule across all notifications.** No special-casing for ParameterChanged. Easier to teach, easier to audit, easier to enforce in reviews.

The cost: every future mutation path has to remember to emit. This is convention-driven, not type-system-enforced. The mitigation is review discipline plus the test rule below — every emission site has a test that asserts emission, so a missing emission shows up as a failing test rather than a silent regression.

### Emission sites map

| Notification | Emission site | Status |
|---|---|---|
| `ModuleAdded` | `apply_patch_to_world` (per-module loop) | Site exists; needs emission added |
| `ModuleAdded` | Future `install_module`-style API | Future bucket |
| `ModuleRemoved` | Future `remove_module` API | Future bucket — type declared now |
| `CableConnected` | `apply_patch_to_world` (per-cable loop) | Site exists; needs emission added |
| `CableConnected` | Future `dispatch_add_cable` (currently NotImplemented) | Future bucket |
| `CableDisconnected` | Future `dispatch_remove_cable` (currently NotImplemented) | Future bucket — type declared now |
| `ParameterChanged` | `dispatch_set_parameter`, after the registry call succeeds | Active emission site for 3.25 |
| `PatchLoaded` (extended) | `load_patch_into_world`, end of successful load | Site exists; needs field population update |

For 3.25's actually-active emission sites: `apply_patch_to_world` (module + cable loops + summary), `dispatch_set_parameter` (`ParameterChanged` after success). The other sites are type-declared but emission lands when the corresponding `NotImplemented` arms get implemented.

---

## Batch semantics (D3 — individual + summary, summary last)

When `LoadPatch` processes a patch with N modules and M cables:

1. For each module in declaration order: emit `ModuleAdded` with `source: ChangeSource::PatchLoad` for any knob initialization side-effects.
2. For each cable in declaration order: emit `CableConnected`.
3. Finally: emit `PatchLoaded { patch_name, source, module_count: N, cable_count: M }`.

Within a single command's notification batch, individual events come first; the summary event is the *last* notification, semantically meaning *"everything is now in place."* Document this in the `PatchLoaded` doc-comment so consumers don't accidentally treat it as "patch starting to load."

Cross-command ordering follows command-enqueue order: notifications from command N+1 do not interleave with notifications from command N.

The test for ordering is non-negotiable (see test discipline below).

---

## Test discipline (D6)

Carried forward from Bucket 3 without modification.

**Per-emission-site tests:**

1. Set up the rack in pre-emission state.
2. Perform the action (call the command-dispatch function or the API method).
3. Drain the message queue: `world.resource_mut::<Messages<T>>().drain()` (Bevy 0.16+ idiom).
4. Assert each notification with `matches!` + explicit field checks.

**No** `.to_string().contains(...)` patterns. **No** "assert at least one event" — assert exact set + exact field values.

**Integration test for `LoadPatch` ordering:** load a multi-module multi-cable patch; drain all notification queues; assert the per-tier ordering (`ModuleAdded`s in declaration order, then `CableConnected`s in declaration order, then exactly one `PatchLoaded`).

**Extension to Bucket-3 positive-path test (`test_dispatch_set_parameter_positive_path`):** the existing test reads back the component state and asserts `state.gain == 0.75`. Extend it to also assert a `ParameterChanged` message was emitted with `old: ParameterValue::Float(0.0)`, `new: ParameterValue::Float(0.75)`, `source: ChangeSource::Command`. Same `matches!` + field-check pattern.

**No new `NotImplemented` arms.** All notification types declared in 3.25 must compile and be writable to `Messages<T>`; the absence of an emission site (for `ModuleRemoved`, `CableDisconnected`) is documented in the type's doc-comment, not enforced via panic.

---

## Locked decisions

(Re-stating from the table at top, with brief rationale where helpful.)

- **L1.** Two-tier architecture, with `CoreCommand` as the command tier (existing) and the six 3.25 events plus extended `PatchLoaded` as the structural-mutation notification tier.
- **L2.** A-everywhere emission. Mutation sites emit; no reactor systems for the 3.25 events.
- **L3.** Individual + summary batch semantics, summary last. `PatchLoaded` is a sync point.
- **L4.** All notification types use `Message` derive (Bevy 0.16+).
- **L5.** All new types live in `oxurack-core::event` and are exported from the crate root.
- **L6.** `ChangeSource` and `PatchSource` use `#[non_exhaustive]` to preserve future variant additions without breaking changes.
- **L7.** `PatchUnloaded` deferred — verified no emission site exists today.
- **L8.** Test discipline carried from Bucket 3 verbatim.

---

## Deferred items

| Item | Bucket / when | Reason |
|------|---------------|--------|
| `LabelChanged` notification | 3.5 | Lands alongside `DisplayName` / `Description` / `Tags` components. Same emission discipline. |
| `PatchUnloaded` notification | Whichever bucket adds unload | No emission site exists today. Adding now would be defining a notification for an action that can't fire. |
| `CableKind` / `CableTarget` enums | 4 (graph) or earlier | Currently no Signal/Modulation distinction in the type system; cables are just `(source_port, target_port)` pairs. When modulation cables become a thing, `CableKind` lands and `CableConnected` / `CableDisconnected` get a `kind` field. Non-breaking add via `#[non_exhaustive]`-style extension. |
| `ParameterSetFailed` (failure event) | Future | Failures return `Result`; failure notifications are non-breaking adds later if a consumer needs them. |
| Audit-log subscriber | Future | Validates the substrate; not load-bearing for 3.25. |
| Cross-process bridge over notifications | Future | Will validate `Clone + Debug + Serialize` is sufficient for serialization. Not adding `serde::Serialize` derives in 3.25 — they're cheap to add when we need them. |
| ModulationRouted, TagsChanged, ParameterSchemaLoaded, … | Future | Same pattern: declare when emission site exists. |
| Dependency-graph command ordering | Future (script-driven batches, cross-process IPC) | See Appendix C. Flat ordered queue is correct for v0; DAG-shaped command graphs become load-bearing when concurrent producers arrive. Adding `depends_on` metadata to `CoreCommand` later is a non-breaking extension. |

---

## Open items — resolved 2026-05-10

All five open items decided. Decisions baked into the body of the memo above; this section is the audit trail.

**O1 — naming: `ParameterChanged`** (not `KnobChanged`). Parameters have multiple form factors — knobs, switches, sliders, MIDI-CC inputs, modulation destinations are all parameters. The notification names the substrate, not the UI affordance. **Locked.**

**O2 — `CableConnected` carries `transform: Option<CableTransform>`.** Required for audit-log replay; cheap to carry; already present on `Cable` and `CoreCommand::AddCable`. **Locked.**

**O3 — Bevy 0.18 resource type verified.** Existing notifications use `#[derive(Message)]` with `MessageWriter<T>` / `MessageReader<T>` system params. The resource type is `Messages<T>`. There is no `Event` derive or `EventWriter` lurking anywhere in the codebase (verified by grep, 2026-05-10). Bevy 0.18's design follows the principle "an event is a special case of a message" — `Message` is the general type, with entity-targeted observation handled by separate `Trigger`/`Observer` machinery. **Locked**: use `Message` derive throughout, consistent with existing `RtWarning`, `SongPositionChanged`, `TransportChanged`, etc.

**O4 — port-field naming: `source_port` / `target_port`.** Matches `Cable` struct exactly. If a future refactor unifies cable terminology, the notification fields refactor in lockstep. **Locked.**

**O5 — patch-load emits `ParameterChanged` for initial parameter values** with `source: ChangeSource::PatchLoad` provenance. Consumers who don't care about load-time noise filter on the provenance field. **Locked.**

The `O5` decision raised a forward-looking question worth recording: as concurrent command producers (Lykn batches, cross-process IPC) enter the system, the flat ordered queue may want to evolve into a dependency-graph ordering model. See Appendix C for the future-work note. Out of scope for 3.25.

---

## Migration notes

1. **`PatchLoaded` extension.** Existing field `patch_name` preserved. Three new fields added. Search the codebase for `PatchLoaded {` constructions; update each to populate new fields. (As of 2026-05-10: emission at `load_patch_into_world` is the one production site; tests construct it as well.)

2. **Bucket-3 positive-path test extension.** The test currently reads back component state. Add assertions for `ParameterChanged` emission. Same test, more assertions.

3. **`CoreCommand::AddCable` and `RemoveCable`** are still `NotImplemented` after this bucket. The notification types `CableConnected` and `CableDisconnected` are declared and `world.write_message`-able; the dispatch arms remain `NotImplemented` until a future bucket implements cable mutation through the command path. (Patch-load emits `CableConnected` directly, not via the command path.)

4. **No new dependencies** introduced. All work is in `oxurack-core` against existing `bevy_ecs` types.

---

## Appendix A — Why this is the right architecture

The structural argument is in the Context section above. A few further notes worth keeping for future-self.

**The two-tier separation is what makes the substrate orthogonal.** If commands and notifications were the same enum, every consumer would have to filter "is this an imperative or a fact?" at the type level. Separating them makes consumer code declarative: subscribe to the notifications you care about, ignore the rest, never see commands at all.

**`CoreCommand` already has the right shape.** It's already a `Message` enum with non-exhaustive variants and per-variant data. The only thing missing was the corresponding *fact* tier. This memo doesn't reshape the architecture — it completes a pattern half-built into the codebase.

**The provenance discipline (`ChangeSource`, `PatchSource`) is non-trivial leverage.** Without it, a future audit-log can't distinguish "user moved this knob" from "patch load set this knob's initial value" from "future LFO modulated the knob." With it, every consumer that wants to filter by source can. Adding source fields after the fact is a breaking change for every downstream consumer; adding them now costs one enum variant per emission site.

---

## Appendix B — Relationship to existing notifications

The existing notifications (`PatchLoaded`, `RtWarning`, `SongPositionChanged`, `TransportChanged`, `MidiInReceived`) are all **system / RT-tier notifications** rather than **structural-mutation notifications**. They report on:

- Patch lifecycle events (load).
- RT-thread warnings (priority elevation, clock dropouts, queue overflow).
- MIDI clock state (song position, transport).
- MIDI input events (note on/off, CC).

Bucket 3.25 adds structural-mutation notifications: changes to *what the rack is and how it's wired*. The two categories share the `Message` derive and the `oxurack-core::event` module, but conceptually they answer different questions:

- System / RT notifications: "what state has the runtime entered?"
- Structural-mutation notifications: "what user-meaningful change just happened to the patch?"

Both tiers feed downstream consumers. Both use the same Bevy-`Messages` substrate. The categorical distinction is documentary, not enforced in the type system.

---

## Appendix C — Dependency-graph command ordering as a future architectural extension

Raised in the design discussion (Duncan, 2026-05-10) as a forward-looking thought on event queuing: *"makes me think we should have metadata or some other means of calculating a 'dependency graph' of actions to determine best-guess/most-proper ordering of actions."*

**The thought.** Command processing today (and in 3.25) is a flat, totally-ordered queue. For human-driven, single-producer, in-process work, this is sufficient and correct. As the system grows, three forces will push toward a richer ordering model:

1. **Script-driven batch emissions.** When a Lykn script fires a sequence of commands, some have causal dependencies (install module before connecting to it) and some are independent (set parameter on module A vs. set parameter on module B). A flat queue forces a serial order; a DAG-shaped command graph would expose the legitimate concurrency.

2. **Cross-process producers.** When commands arrive from out-of-process (future TUI-as-separate-binary, future IPC), reorderings are inevitable due to network and queue jitter. A causality-aware ordering model — Lamport timestamps, vector clocks, or explicit `depends_on` declarations — becomes load-bearing for determinism.

3. **Replay determinism under concurrency.** When replaying a recorded command stream, the system must produce the same notification stream regardless of any incidental reorderings. Today this works because the order is fixed and reordering doesn't happen. As concurrency enters, declared dependencies preserve replay determinism without forcing serial execution.

**Sketch of mechanics.** Each command carries optional `depends_on: SmallVec<[CommandId; 1]>` metadata. The processor builds a DAG and runs commands in any valid topological order. Independent commands may run in parallel. Notification emission still serializes per-command, but cross-command notification ordering follows the DAG's topological order, not insertion order.

**Status: deferred.** 3.25 ships the flat ordered queue. The dependency-graph extension lands when a real use case forces it — first candidate is Lykn-script-driven command batches once scripting begins. The substrate added in 3.25 (commands tier, notifications tier, deterministic emission discipline) is forward-compatible: adding `depends_on` metadata to `CoreCommand` later is a non-breaking enum-extension, and the notification tier needs no changes.

This appendix exists because the thought is correct and substantive; capturing it now keeps it from being rediscovered painfully later, while keeping it explicitly out of 3.25 scope.

---

## Suggested next step

This memo is the design. The natural next move is the CC-facing implementation spec, which follows our six-element structure (work-mode block, citations, hunt list, anti-softpedal, did-not-find section, no pre-listed decisions) and includes explicit DoD criteria for each emission site plus the integration test for batch ordering.

All open items resolved as of 2026-05-10. No blockers remain.