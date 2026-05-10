# Bucket 3.25: Complete the Notification Tier of the Event Substrate

## Context

This is bucket 3.25 of the v0 plan. Its job is to **complete the notification tier of oxurack's two-tier event architecture**. The command tier (`CoreCommand`) and several existing notifications (`PatchLoaded`, `RtWarning`, `SongPositionChanged`, `TransportChanged`, `MidiInReceived`) are already in place. This bucket adds the structural-mutation notification tier — events that fire when modules, cables, or parameter values change — completing the substrate that bucket 4 (the graph), eventually the TUI, and eventually Lykn handlers will all consume.

The architecture is CQRS-shaped, with finance/AMQP-driven heritage. Commands flow in, get processed serially, and produce notifications that flow out to arbitrary subscribers. This bucket finishes the notification side.

**This is not optional ceremony.** The graph (bucket 4) cannot maintain its adjacency lists without these events. The future Lykn `(rack:on :ParameterChanged ...)` API has nothing to listen to without these events. A subtly wrong emission discipline here cascades into every downstream consumer being subtly wrong.

---

## Required reading before recon

**`workbench/event-substrate-design.md`** is the design memo. It locks all architectural decisions: the two-tier framing, the six new notification types, the A-everywhere emission discipline, the batch semantics rule, the test discipline carried from bucket 3, and five resolved naming/shape decisions (O1–O5). Do not re-litigate those decisions in this bucket. If you find a real reason one of them is wrong, **stop and report** — that's a memo-level revision, not an implementation workaround.

The eight locked architectural decisions (D1–D8) and the five resolved open items (O1–O5) are recapped in §3 below for convenience, not for re-decision.

---

## Recon findings (as of 2026-05-10, verify against the current tree)

All claims here cite file:line as of 2026-05-10. Bucket 3 closed at commit `8e9652c`; if newer commits land before this bucket starts, re-verify each claim. Where this doc cites file:line, **check it still applies** — silent drift is the most common failure mode.

### Existing types in `oxurack-core::event` (event.rs)

- **`CoreCommand`** (`event.rs:42-78`) is the command tier — `#[non_exhaustive] #[derive(Message, Debug, Clone)]` enum with variants LoadPatch / SavePatch / SetParameter / AddCable / RemoveCable / SetBpm / Panic. The doc-comment at `event.rs:40-42` literally describes the queue-of-operations pattern: *"Sent from the REPL, UI, or scripting layer. Processed by the command handler system during the next tick."*
- **`PatchLoaded`** (`event.rs:80-85`) **already exists** as a `#[derive(Message, Debug, Clone)]` struct with one field `patch_name: String`. This bucket extends it; do not delete or rename.
- **`RtWarning`, `RtWarningCode`, `SongPositionChanged`, `TransportChanged`, `MidiInReceived`** all use `#[derive(Message, ...)]`. Use them as your model for the new types' derive shape.
- **`dispatch_core_command`** (`event.rs:153-169`) is a free fn taking `&mut World`. The `SetParameter` arm closes via `dispatch_set_parameter` (event.rs:171-211). This is the `ParameterChanged` emission site.

### Existing types in `oxurack-core::patch`

- **`apply_patch_to_world`** (referenced in `oxurack/src/lib.rs:96`) is the function that reifies a `Patch` into ECS state. **This is the emission site for the patch-load notification batch** (`ModuleAdded`, `CableConnected`, `ParameterChanged` for initial values, then `PatchLoaded` last).
- **`load_patch_into_world`** also exists per the export list (`lib.rs:80`). Find which is the right top-level boundary — `load_patch_into_world` or `apply_patch_to_world` — and emit `PatchLoaded` once at the end of whichever wraps the per-module/per-cable work.
- **`PatchHandle`** (`patch.rs:455`) is `pub modules: HashMap<String, Entity>`. Already present as a resource after load.
- **`Cable`** (`cable.rs:132-?`) is a struct with `pub source_port: Entity, pub target_port: Entity`. May also have `transform: Option<CableTransform>` — verify field shape before constructing `CableConnected`.

### `ParameterRegistry` shape (parameter.rs:96-134)

- **`setters: HashMap<(ModuleKind, ParameterName), ParameterSetter>`** is the only field. **There is no getter machinery.** This is load-bearing for Decision 1 below.
- `set_parameter` (`parameter.rs:117-134`) returns `Result<(), CoreError>` — does not return the previous value.

### Bevy version

Bevy 0.18. `Message` derive is the convention. `MessageWriter<T>` / `MessageReader<T>` are the system params. `Messages<T>` is the resource type. There is no `Event` derive or `EventWriter` in this codebase — verified by grep, 2026-05-10.

### Existing message registration

Find where existing messages (`RtWarning`, `SongPositionChanged`, etc.) are registered with the `App` — almost certainly via `add_message::<T>()` or `init_message::<T>()` calls in `CorePlugin::build()` at `lib.rs:122+`. **Follow the existing pattern verbatim** for the new types.

### Existing emission patterns

The codebase has `MessageWriter<T>` system params in `bridge.rs:108-112` for systems that emit. But this bucket emits from `dispatch_core_command` (a free fn taking `&mut World`, not a system) and from `apply_patch_to_world` (likely also taking `&mut World`). For exclusive-world contexts, the standard Bevy 0.18 idiom is:

```rust
world.write_message(MyMessage { ... });
// or, equivalently:
// world.send_message(MyMessage { ... });
```

**Verify** which method name Bevy 0.18 uses; both have appeared across versions. If neither works, fall back to `world.resource_mut::<Messages<MyMessage>>().write(MyMessage { ... })` (or its current 0.18 spelling).

### Test patterns

The codebase doesn't currently have tests that assert message emission (verified by grep for `MessageReader` and `Messages<` in test code, 2026-05-10). **You're establishing the pattern.** The standard Bevy 0.18 in-test idiom is:

```rust
let messages = world.resource::<Messages<MyMessage>>();
let collected: Vec<_> = messages.iter_current_update_messages().cloned().collect();
// assert with matches! + field checks
```

— or its current 0.18 spelling. Verify against Bevy's 0.18 docs/source if uncertain.

---

## Decision to make (resolve explicitly in your completion report)

### 1. How to populate `ParameterChanged.old`

`ParameterRegistry` has setters only — no getter machinery. The design memo's `ParameterChanged` struct declares `old: ParameterValue` as a non-Optional field, intended for audit log, undo, and diff-renderer consumers.

Three implementation paths:

**(A) Add `get_parameter` to ParameterRegistry symmetrically with `set_parameter`.** Requires a `ParameterGetter = fn(&World, Entity) -> ParameterValue` type, a `getters: HashMap<...>` field on `ParameterRegistry`, a `register` extension to take both setter and getter, and an extension to whatever trait modules use to register parameters (probably `OxurackModule`) to expose both setter and getter functions per parameter. Substantial scope addition, but completes the registry's symmetry.

**(B) Change `ParameterSetter` signature to return the previous value.** From `Fn(&mut World, Entity, ParameterValue) -> Result<(), CoreError>` to `Fn(&mut World, Entity, ParameterValue) -> Result<ParameterValue, CoreError>` returning the prior value. Setters do read-then-write internally. Smaller change than (A) if few setters exist; larger long-term churn if setters multiply. Verify: count the existing setter implementations before deciding.

**(C) Make `ParameterChanged.old: Option<ParameterValue>`, document deferral.** Smallest-scope change. Substrate is complete; capability for `old` population is deferred to a future bucket that adds getter machinery. Design memo walks back from `old: ParameterValue` to `old: Option<ParameterValue>` for v0, with explicit `// TODO(bucket-X): populate when registry getters land` at every emission site.

**Recommendation: choose based on what recon reveals about setter count and `OxurackModule`'s shape.**

- If `OxurackModule` already exposes per-parameter type info (kinds, etc.) and adding getters is a clean extension, take **(A)**.
- If there's exactly one or two setter implementations in the codebase today (probably the test mock module), take **(B)** — it's cheaper than (A) and cleaner than (C).
- If both (A) and (B) cascade into more than this bucket can absorb, take **(C)** with explicit `TODO(bucket-X)` markers and document it loudly in the completion report. **Do not silently land (C).**

The design memo's intent was for `old` to be mandatory; (C) is a real walkback. If you take it, the design memo gets a follow-up edit to reflect the v0 reality.

---

## Locked decisions (recap from design memo — do not re-litigate)

| # | Decision |
|---|----------|
| L1 | Two-tier architecture: `CoreCommand` (existing) + new notification tier. |
| L2 | A-everywhere emission discipline. Every mutation site emits explicitly. No `Changed<T>` reactor systems for the new events. |
| L3 | Batch semantics: individual notifications first, summary last. `PatchLoaded` is the patch-load summary. |
| L4 | New types use `#[derive(Message, Clone, Debug)]`. |
| L5 | All new types live in `oxurack-core::event`, exported from the crate root. |
| L6 | `ChangeSource` and `PatchSource` use `#[non_exhaustive]`. |
| L7 | `PatchUnloaded` is **deferred** — no emission site exists. Do not add it in this bucket. |
| L8 | Test discipline: `matches!` + field checks. No `.to_string().contains(...)`. Positive-path tests must read back component state. |
| O1 | Notification name: `ParameterChanged` (not `KnobChanged`). |
| O2 | `CableConnected` carries `transform: Option<CableTransform>`. |
| O3 | Use `Message` derive, `MessageWriter<T>` / `MessageReader<T>`, `Messages<T>`. |
| O4 | Cable port fields: `source_port: Entity`, `target_port: Entity`. |
| O5 | Patch-load emits `ParameterChanged` for initial parameter values, with `source: ChangeSource::PatchLoad`. |

If implementation reveals one of these is wrong, **stop and report** rather than work around.

---

## Work mode

**Thinking work stays in your context** with the Rust skill loaded (`assets/ai/ai-rust/skills/claude/SKILL.md`). This includes: Decision 1 above; the borrow-checker pattern for emitting from inside `dispatch_set_parameter`'s `resource_scope`; whether to add the `add_message::<T>()` registrations alongside existing ones in `CorePlugin::build()` or as a separate pass; how to thread `ChangeSource` provenance through `apply_patch_to_world` so patch-load knob initialization gets `ChangeSource::PatchLoad` and command-driven changes get `ChangeSource::Command`.

**Lookup work may delegate to subagents.** This includes: grep for the exact `add_message` / `init_message` pattern in CorePlugin; finding all existing `PatchLoaded` construction sites that need updating for the new fields; counting existing `ParameterSetter` implementations for Decision 1; verifying `Cable`'s exact field shape; finding where `apply_patch_to_world` and `load_patch_into_world` actually live and which is the outer boundary.

Serial execution on the thinking path. Quality over speed. Lookup parallelism is welcome.

---

## The task

Add the notification tier to `oxurack-core::event` and wire up emission at every mutation site that exists today.

### Type additions (`oxurack-core/src/event.rs`)

Add the following new types. Doc-comments are required and should follow the example below (keep them informative; they're the consumer-facing API surface).

```rust
/// Emitted when a module entity is added to the rack.
///
/// Producers: `apply_patch_to_world` (during patch load); future
/// `install_module`-style API surfaces. Consumers can subscribe
/// without breakage even if no emission site is currently active.
#[derive(Message, Clone, Debug)]
pub struct ModuleAdded {
    pub entity: Entity,
    pub instance_name: String,
    pub kind: ModuleKind,
}

/// Emitted when a module entity is removed from the rack.
///
/// Producers: future `remove_module` API (no emission site today).
#[derive(Message, Clone, Debug)]
pub struct ModuleRemoved {
    pub entity: Entity,
    pub instance_name: String,
    pub kind: ModuleKind,
}

/// Emitted when a cable entity is added to the rack.
///
/// Producers: `apply_patch_to_world` (during patch load); future
/// `dispatch_add_cable` (currently NotImplemented).
#[derive(Message, Clone, Debug)]
pub struct CableConnected {
    pub cable_entity: Entity,
    pub source_port: Entity,
    pub target_port: Entity,
    pub transform: Option<CableTransform>,
}

/// Emitted when a cable entity is removed from the rack.
///
/// Producers: future `dispatch_remove_cable` (currently NotImplemented).
#[derive(Message, Clone, Debug)]
pub struct CableDisconnected {
    pub cable_entity: Entity,
    pub source_port: Entity,
    pub target_port: Entity,
}

/// Emitted after a parameter is successfully written.
///
/// Carries `old` and `new` so consumers (audit log, undo, diff
/// renderers) get the full delta in one notification. The
/// `source` field carries provenance — was this command-driven,
/// patch-load-driven, or direct-API-driven.
///
/// Naming note: `ParameterChanged`, not `KnobChanged`. Parameters
/// have multiple form factors — knobs, switches, sliders, MIDI-CC,
/// modulation destinations — so the notification names the
/// substrate, not the UI affordance.
#[derive(Message, Clone, Debug)]
pub struct ParameterChanged {
    pub entity: Entity,
    pub instance_name: String,
    pub param: ParameterName,
    pub old: ParameterValue,        // or Option<ParameterValue> if Decision 1 = C
    pub new: ParameterValue,
    pub source: ChangeSource,
}

/// Provenance for a [`ParameterChanged`] notification.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum ChangeSource {
    /// Direct user/script command via [`CoreCommand::SetParameter`].
    Command,
    /// Direct API mutation (future API surfaces).
    DirectApi,
    /// Patch-load initialization. One `ParameterChanged` per
    /// parameter set during patch load.
    PatchLoad,
}

/// Provenance for a [`PatchLoaded`] notification.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum PatchSource {
    File(std::path::PathBuf),
    InMemory,
    Other(String),
}
```

### Extend `PatchLoaded`

```rust
/// Emitted *last* in a patch-load batch, after all individual
/// `ModuleAdded`, `CableConnected`, and `ParameterChanged`
/// notifications. Semantically: "the patch is now in place;
/// redraw if you redraw."
///
/// Subscribers who don't care about granular changes can subscribe
/// only to this event and treat it as a batch sync point.
#[derive(Message, Debug, Clone)]
pub struct PatchLoaded {
    pub patch_name: String,         // existing, preserved
    pub source: PatchSource,         // new
    pub module_count: usize,         // new
    pub cable_count: usize,          // new
}
```

Update every existing construction site of `PatchLoaded {` to populate the new fields. Search via `rg 'PatchLoaded \{' crates/` to enumerate.

### Exports (`oxurack-core/src/lib.rs`)

Add the new types to the existing `pub use event::{...}` re-export at lib.rs:72-75. Maintain alphabetical or logical grouping consistent with what's there.

### Plugin registration (`oxurack-core/src/lib.rs`)

Find where existing messages are registered (almost certainly in `CorePlugin::build()` at `lib.rs:122+` — `app.add_message::<RtWarning>()` or similar). Add `add_message` calls for every new type:

- `ModuleAdded`
- `ModuleRemoved`
- `CableConnected`
- `CableDisconnected`
- `ParameterChanged`

(`PatchLoaded` is already registered; no change needed there.)

If `CorePluginFixed` (lib.rs:147) also registers messages, mirror the pattern there.

### Emission sites

#### A. `dispatch_set_parameter` (event.rs:171+) — emit `ParameterChanged`

After the registry call succeeds, emit:

```rust
ParameterChanged {
    entity,
    instance_name: instance_name.to_string(),
    param: ParameterName::from(param),
    old: <previous value, per Decision 1>,
    new: value.clone(),
    source: ChangeSource::Command,
}
```

The emission must happen **only on success** — failed parameter sets do not emit `ParameterChanged`. Place the emission after the `resource_scope` block, after the `.map_err(...)` rebuild, only on the `Ok` path.

The `entity` is already in scope. The `instance_name` is the function arg. `param` and `value` are also args. `source: ChangeSource::Command` is fixed at this site.

For `old`: depends on Decision 1.

#### B. `apply_patch_to_world` — emit `ModuleAdded` / `CableConnected` / `ParameterChanged` / `PatchLoaded`

This is the patch-load batch emission site. Order:

1. **For each module** in the patch (in declaration order): after spawning the module entity and inserting components, emit `ModuleAdded { entity, instance_name, kind }`.
2. **For each parameter** set during module initialization: emit `ParameterChanged` with `source: ChangeSource::PatchLoad`. (Recon detail: find where parameter initialization happens — may be inside the module's spawn function, may be a separate pass. The `ChangeSource::PatchLoad` provenance is the discriminator that lets future audit-log filters distinguish load-time noise from runtime activity.)
3. **For each cable** in the patch (in declaration order): after spawning the cable entity, emit `CableConnected { cable_entity, source_port, target_port, transform }`.
4. **Once at the end**: emit `PatchLoaded { patch_name, source, module_count, cable_count }` — `module_count` and `cable_count` are the actual counts; `source` is `PatchSource::File(path)` if loaded from file, `PatchSource::InMemory` otherwise.

The summary `PatchLoaded` is the **last** notification. Document this contract in the type's doc-comment.

If `apply_patch_to_world` doesn't have access to the patch source (file path vs. in-memory), thread it through from the caller (`load_patch_from_file` is presumably one path; tests may construct `Patch` in memory). If threading is invasive, consider a wrapper that emits `PatchLoaded` outside `apply_patch_to_world` itself — but keep the per-module/per-cable emissions inside.

#### Out-of-scope emission sites (do NOT add)

- `ModuleRemoved`, `CableDisconnected`: types declared, emission sites do not exist (`remove_module` is not a thing; `dispatch_remove_cable` returns `NotImplemented`). Add the types; do not add stub emission.
- `LoadPatch` / `SavePatch` / `AddCable` / `RemoveCable` arms in `dispatch_core_command`: still `NotImplemented`. Do not close them.

### Tests

#### Test discipline (carry forward from Bucket 3)

Every test that asserts a notification was emitted uses:

```rust
let messages = world.resource::<Messages<ModuleAdded>>();
let collected: Vec<_> = messages.iter_current_update_messages().cloned().collect();
assert_eq!(collected.len(), 1);
assert!(matches!(
    &collected[0],
    ModuleAdded { instance_name, kind, .. }
        if instance_name == "vco_1" && matches!(kind, ModuleKind::Vco)
));
// or explicit field assertions
```

— or the equivalent Bevy 0.18 idiom. Verify the exact method name (`iter_current_update_messages`, `read`, `drain`, etc.) against Bevy 0.18's actual API; the name has shifted across versions.

**No `.to_string().contains(...)` patterns.** **No "assert at least one event" patterns.** Assert exact set + exact field values.

#### Required tests

For each new notification type, at least one test:

1. `test_module_added_emitted_on_patch_load` — patch with one module, load it, drain `Messages<ModuleAdded>`, assert exactly one event with correct fields.
2. `test_cable_connected_emitted_on_patch_load` — patch with one cable, load it, drain `Messages<CableConnected>`, assert exactly one event with correct fields including `transform`.
3. `test_parameter_changed_emitted_on_set_parameter` — extend the existing `test_dispatch_set_parameter_positive_path` to also drain `Messages<ParameterChanged>` and assert one event with `old`, `new`, `source: ChangeSource::Command`. The existing positive-path component-readback assertion stays.
4. `test_parameter_changed_emitted_on_patch_load_initial_values` — patch with module that has non-default parameter values, load it, drain `Messages<ParameterChanged>`, assert events with `source: ChangeSource::PatchLoad`.
5. `test_patch_loaded_extended_fields` — load patch, drain `Messages<PatchLoaded>`, assert `module_count`, `cable_count`, `source` populated correctly.

Plus the integration test:

6. `test_patch_load_notification_ordering` — load a multi-module multi-cable patch. Drain all notification queues. Assert ordering: all `ModuleAdded` events fire first (in declaration order), then `CableConnected` events (in declaration order), then exactly one `PatchLoaded`. The assertion must check **cross-type ordering**, not just per-type counts. Suggested: drain into a single `Vec<NotificationKind>` (test-only enum) tagged by emission order, assert the sequence.

If Bevy 0.18 doesn't preserve cross-type emission order in a test-friendly way, find the closest equivalent (e.g., timestamp each notification on emission, sort by timestamp in the assertion). Document the approach taken.

#### Negative tests

7. `test_parameter_changed_not_emitted_on_set_parameter_failure` — call `SetParameter` with an unknown module / unknown parameter. Drain `Messages<ParameterChanged>`. Assert empty.

#### Failure-mode tests not required by this bucket

- Tests for `ModuleRemoved` / `CableDisconnected` emission. No emission site exists; nothing to test. The types must compile and be writable to `Messages<T>`, but emission tests come with the bucket that adds the emission sites.

### Doc-comment updates

- `event.rs:124-150` (or wherever the `dispatch_core_command` doc-comment lives now): add a "currently emits" or "emits" subsection naming the notifications dispatched. Stale docs are silent lies.
- The new `PatchLoaded` doc-comment must explain that it's a *batch sync point*, fired *last* in a patch-load batch, not "patch starting to load."
- The `ParameterChanged` doc-comment must include the parameters-have-multiple-form-factors naming explanation.

---

## Out of scope for this bucket

- `LoadPatch` / `SavePatch` / `AddCable` / `RemoveCable` arms of `dispatch_core_command`: stay `NotImplemented`. Do not close them.
- `ModuleRemoved` / `CableDisconnected` emission sites: types declared, emission deferred until `remove_module` / `dispatch_remove_cable` exist.
- `LabelChanged` notification: deferred to bucket 3.5.
- `PatchUnloaded` notification: deferred until unload exists.
- Audit-log subscriber, cross-process bridge, `serde::Serialize` derives on notifications: deferred to future buckets that need them.
- Adding `CableKind` (Signal vs Modulation) field on cable notifications: deferred until the type system distinguishes them.
- `get_parameter` API on `ParameterRegistry` *unless* you take Decision 1A. (1B and 1C don't add a getter.)

---

## Suggested steps (diverge if you find reason)

1. **Recon against the current tree.** Verify each cited file:line. Read `apply_patch_to_world`, `load_patch_into_world`, and the existing `PatchLoaded` construction sites. Count existing `ParameterSetter` implementations for Decision 1.

2. **Resolve Decision 1.** Document in your completion report which option (A, B, or C) and why.

3. **Add the new types** (event.rs). Add `ChangeSource` and `PatchSource` enums. Extend `PatchLoaded`. Update existing `PatchLoaded` construction sites for the new fields.

4. **Wire `ChangeSource::Command` emission into `dispatch_set_parameter`.** This is the cleanest emission site to land first because it has all the info in scope.

5. **Wire patch-load emissions into `apply_patch_to_world`.** This is the bigger emission piece. Order: module loop → cable loop → final `PatchLoaded`.

6. **Register all new types** with `CorePlugin::build()`.

7. **Update exports in `lib.rs`.**

8. **Tests.** Per the list above. Establish the message-assertion idiom; document any deviation from the suggested pattern.

9. **Update doc-comments.** No stale narratives.

10. **Run `make format`, `make lint`, `make test`.** No `#[ignore]`. No silent failures.

---

## DoD (every criterion is a required pass condition)

1. `cargo clean && make check` passes from a fresh state.
2. The five new notification types (`ModuleAdded`, `ModuleRemoved`, `CableConnected`, `CableDisconnected`, `ParameterChanged`) and two new enums (`ChangeSource`, `PatchSource`) compile, derive `Message` (where applicable) + `Clone + Debug`, and are exported from the crate root.
3. `PatchLoaded` extended with `source`, `module_count`, `cable_count` fields. Existing `patch_name` field preserved. Every existing construction site updated.
4. All five new notification types registered with `CorePlugin::build()` (and `CorePluginFixed` if applicable).
5. `dispatch_set_parameter` emits `ParameterChanged` with `source: ChangeSource::Command` after a successful parameter set. Failed parameter sets do not emit.
6. `apply_patch_to_world` emits, in order: `ModuleAdded` per module → `ParameterChanged { source: PatchLoad }` per initial parameter (per Decision 1) → `CableConnected` per cable → exactly one `PatchLoaded` at the end.
7. `PatchLoaded`'s doc-comment explains the batch-sync-point semantics.
8. `ParameterChanged`'s doc-comment includes the parameters-have-multiple-form-factors naming explanation.
9. `ParameterChanged.old` populated per Decision 1. If Decision 1 = C, every emission site has a `// TODO(bucket-X): ...` comment with the bucket explicitly named, and the design memo gets a follow-up edit (raise this in your completion report).
10. Test `test_module_added_emitted_on_patch_load` passes.
11. Test `test_cable_connected_emitted_on_patch_load` passes — including `transform` field assertion.
12. Test `test_parameter_changed_emitted_on_set_parameter` passes — `matches!` + field checks for `old` (per Decision 1), `new`, `source: ChangeSource::Command`. The existing component-readback assertion in `test_dispatch_set_parameter_positive_path` is preserved.
13. Test `test_parameter_changed_emitted_on_patch_load_initial_values` passes — events have `source: ChangeSource::PatchLoad`.
14. Test `test_patch_loaded_extended_fields` passes — assertions on `module_count`, `cable_count`, `source`.
15. Test `test_patch_load_notification_ordering` passes — cross-type ordering verified.
16. Test `test_parameter_changed_not_emitted_on_set_parameter_failure` passes — empty drain on failure paths.
17. No `.to_string().contains(...)` patterns in any new test. All emission assertions use `matches!` with field checks or explicit field equality.
18. Grep: `rg -n 'NotImplemented\("(LoadPatch|SavePatch|AddCable|RemoveCable)"\)' crates/` returns **exactly four hits** (one per arm). The other four `NotImplemented` arms are unchanged.
19. No `KnobChanged` in the codebase. (Sanity: `rg -n 'KnobChanged' crates/` returns zero hits.)
20. The work lands as a single atomic commit with a clear message (e.g. `Complete notification tier of event substrate (bucket 3.25)`).

---

## Completion report format

When done, reply with exactly this structure (no prose preamble, no softening):

```
Bucket 3.25 complete at <commit sha>.

Decision 1 (ParameterChanged.old population): <A / B / C> — <justification>

Recon findings:
- ParameterRegistry getter machinery present: <yes / no>
- Existing ParameterSetter implementation count: <N>
- apply_patch_to_world boundary location: <file:line>
- Patch source threading required: <yes / no — describe>
- <any other recon-revealed shape difference from this doc's claims>

New types added (N):
- <list with one-line description>

Types extended (N):
- <list>

Plugin registrations added (N):
- <list>

Emission sites wired (N):
- <list with one-line description per site>

Code files changed (N):
- <list>

Tests added (N):
- <list of test names with what each asserts>

Verification: cargo clean && make check — <passed / failures>
Verification: 5 new types registered with CorePlugin — <yes / no>
Verification: 4 NotImplemented arms unchanged — <yes / no>
Verification: zero KnobChanged occurrences — <yes / no>
Verification: PatchLoaded doc-comment explains batch-sync semantics — <yes / no>
Verification: ParameterChanged doc-comment explains naming — <yes / no>
Verification: cross-type emission ordering test passes — <yes / no>

Cross-bucket dependencies surfaced:
- <anything bucket 3.5/4+ will need to handle, or "none">

Design memo follow-up edits required:
- <list, or "none">

Intentional exceptions or deferrals:
- <list with explicit justification, or "none">

Spec-adherence walk:
- DoD #1: <met / not met — explanation if not>
- DoD #2: <met / not met>
- ... (all 20)

Nothing silently skipped.
```

That last line matters. If recon revealed shapes different from this doc's assumptions, the patch source threading was uglier than expected, the message-assertion idiom in Bevy 0.18 turned out to need a workaround, or anything else cascaded — all of that goes in the appropriate section. No silent absorption.

---

## Anti-softpedal reminder

Watch-items particular to this bucket:

- **A test that drains an empty queue and passes.** If `test_module_added_emitted_on_patch_load` calls `messages.iter_current_update_messages().count()` and gets 0 because the message-assertion idiom is wrong, the test "passes" (vacuously) and emission is silently broken. **Each emission test must assert exactly the expected count, with field checks, not just "at least one event" or "no panic."**

- **Cross-type ordering treated as out of scope.** It's not. The integration test in DoD #15 is load-bearing — the graph (bucket 4) depends on causal ordering, and downstream consumers will too. If preserving cross-type ordering in a test is hard in Bevy 0.18, find the workaround and document it; do not skip the test.

- **Decision 1 = C lands silently.** If you take (C), the design memo's `old: ParameterValue` becomes `old: Option<ParameterValue>` for v0. That's a real walkback; flag it loudly in the completion report and in the design memo follow-up edits section. Do not let it slide as "implementation detail."

- **Patch-load `ParameterChanged` emissions get accidentally tagged with `ChangeSource::Command`.** The `source` field is the entire reason `ChangeSource` exists. If patch-load emissions all carry `Command` provenance, future audit-log consumers can't distinguish initial state from runtime activity, and the field is degraded to vestigial. Verify the `PatchLoad` provenance is actually populated at patch-load emission sites.

- **`PatchLoaded` doc-comment left ambiguous.** "Emitted on patch load" doesn't say *when* in the load process. The doc-comment must say "fired *last* in a patch-load batch" so consumers don't subscribe expecting it as a "loading started" signal.

- **`add_message` registrations missed.** If you forget to register a notification type in `CorePlugin::build()`, emission may panic at runtime (`Messages<T>` resource not present). Tests that exercise emission will catch this; emission paths that aren't test-covered won't. Do the plugin registration and the type definition in the same change-set, not staggered.

If Decision 1 cascades into more than this bucket can absorb — e.g. (A) requires extending `OxurackModule` in ways that break the existing test mock module — **stop and raise it** rather than silently rewriting. A clean "stopped because Decision 1A cascades into trait extension that needs its own bucket" is more useful than a dirty "finished, kind of."

---

## After you report

I'll do trust-but-verify: confirm Decision 1's choice is internally consistent with the test patterns; grep for `KnobChanged` to confirm zero hits; check the four preserved `NotImplemented` arms; read the cross-type ordering test and confirm it actually checks ordering (not just counts); read each emission site and confirm the provenance field is populated correctly; verify `add_message` registrations match the new types added. Then we sign off or loop back.
