---
number: 10
title: "oxurack Public API Design (v0)"
author: "module authors"
component: All
tags: [change-me]
created: 2026-04-20
updated: 2026-04-20
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# oxurack Public API Design (v0)

**Status:** Working draft — source of truth for Bucket 3+ implementation.
**Last updated:** 2026-04-20
**Supersedes:** Bucket 2's `OxurackApp` surface (see Migration section).

---

## Positioning

> **We give you the ease of software MIDI-modular synthesis, with the feel of hardware.**

The developer experience should read as *operating a rack*, not *configuring a Bevy app*. Every public name is chosen to reinforce that frame: you build a rack, install modules, patch cables, turn knobs, power on, play, stop, panic.

Bevy is the engine underneath. It should not leak into user code except as an explicit, named escape hatch.

---

## Confirmed Naming Decisions

| Concern | Decision | Rationale |
|---|---|---|
| Root type | `Oxurack` | Reads as "the rack"; `let rack = Oxurack::new()` |
| Construction | `Oxurack::new()` | Plain constructor; no builder ceremony at v0 |
| Module registration | `rack.install::<Clock>()` | Hardware verb; "installing a module in a rack" |
| Cable ops | `patch` / `unpatch` | Hardware-authentic; software-sense collision is negligible in the target audience |
| Parameter setting | `set_knob` | Concrete, tactile; no ambiguity with MIDI CC, automation, etc. |
| Lifecycle | `power_on` / `power_off` | Hardware verb; independent of transport (a rack can be powered on but stopped) |
| Transport | `start` / `stop` / `pause` / `resume` | Four verbs mirroring MIDI START/STOP/CONTINUE + `pause`; clean reset/preserve split. `resume` instead of `continue` because the latter is a Rust keyword |
| Emergency | `reset` *(narrow scope)* | `panic` collides with Rust's catastrophic-error vocabulary; `reset` is also the name of the button on a synth |
| Patch loading | `load_patch` | Unchanged — already hardware-adjacent |
| Frame stepping | `step` | Unchanged — deterministic testing/hosting |
| Run loop | *removed at v0* | Available via `rack.into_app().run()` for the rare headless-daemon case |
| Bevy escape | `Oxurack::into_app() -> bevy_app::App` | Consuming; no reference leak; also the path to `App::run()` |
| Module trait | `oxurack::Module` | Crate namespace disambiguates; drop `Oxurack` prefix |
| Internal ECS rename | `oxurack_core::Module` component → `ModuleInstance` | Avoids collision with both the `oxurack::Module` trait *and* the pre-existing `ModuleId` identity component |
| Reference vocabulary | Per-module `<Module>Port` / `<Module>Knob` enums + shared value enums | One vocabulary, four surface styles (handle, inline, string, typed-method sugar) |
| Handle lookup | `rack.module::<M>(name) -> Result<Handle<M>>` | "Access the lfo_1 module" reads as hardware; grab-once for multi-touch flows |
| Value enums | Shared in `oxurack::values::{Waveform, ClockDivision, …}`; module-specific value enums in the module's crate | Closed-range knobs map 1:1 to hardware rotary switches |

---

## Full Public Surface (v0 sketch)

### Usage as a user would encounter it

```rust
use oxurack::{Oxurack, Module, modules::{Clock, Lfo, Scope}};

let mut rack = Oxurack::new();

rack.install::<Clock>();
rack.install::<Lfo>();
rack.install::<Scope>();

rack.load_patch(my_patch)?;

// Post-load edits:
rack.patch("clock.gate", "lfo.reset")?;
rack.set_knob("lfo", "rate", 2.0)?;

rack.power_on();
rack.start();        // begin from position 0
rack.step();         // advance one frame (or loop calling step() in a host)
rack.pause();
rack.resume();
rack.stop();         // halt and reset position
rack.power_off();

// Escape hatch: reach into the engine.
let app: bevy_app::App = rack.into_app();
// ...or, for a headless daemon that runs forever:
// rack.into_app().run();
```

### `Oxurack`

```rust
impl Oxurack {
    pub fn new() -> Self;

    // Module registration
    pub fn install<M: Module>(&mut self) -> &mut Self;

    // Patch management
    pub fn load_patch(&mut self, patch: Patch) -> Result<()>;

    // Cable ops (runtime — for post-load rewiring)
    pub fn patch(
        &mut self,
        source: impl Into<PortRef>,
        target: impl Into<PortRef>,
    ) -> Result<()>;
    pub fn unpatch(
        &mut self,
        source: impl Into<PortRef>,
        target: impl Into<PortRef>,
    ) -> Result<()>;

    // Parameters
    pub fn set_knob(
        &mut self,
        module: impl Into<ModuleRef>,
        knob: impl Into<KnobRef>,
        value: impl Into<ParameterValue>,
    ) -> Result<()>;

    // Module access (handle lookup for grab-once flows)
    pub fn module<M: Module>(&self, name: &str) -> Result<Handle<M>>;

    // Lifecycle (rack power — independent of transport)
    pub fn power_on(&mut self);
    pub fn power_off(&mut self);

    // Transport — four verbs with unambiguous semantics:
    //   start  — begin from position 0         (MIDI START)
    //   stop   — halt and reset position to 0  (MIDI STOP)
    //   pause  — halt without resetting        (MIDI STOP)
    //   resume — resume from paused position   (MIDI CONTINUE)
    pub fn start(&mut self);
    pub fn stop(&mut self);
    pub fn pause(&mut self);
    pub fn resume(&mut self);

    // Emergency — narrow scope: silence + flush pending commands.
    // Leaves knobs, cables, transport, and power alone.
    pub fn reset(&mut self);

    // Frame stepping
    pub fn step(&mut self);

    // Escape hatch (consuming; also the path to App::run() for headless use)
    pub fn into_app(self) -> bevy_app::App;
}
```

### `Handle<M>`

Returned by `rack.module::<M>(name)`. Caches the entity lookup so multi-touch flows don't re-search the registry per call. The handle's methods consume the module's associated `Port` and `Knob` enums directly — type-checked, autocompletable.

```rust
impl<M: Module> Handle<M> {
    pub fn port(&self, port: M::Port) -> PortRef;
    pub fn knob(&self, knob: M::Knob) -> KnobRef;
    pub fn set(&self, knob: M::Knob, value: impl Into<ParameterValue>) -> Result<()>;
}
```

**Optional per-module sugar** (added by module authors, purely additive — the core enum path always works):

```rust
// Example: an LFO implementation's own handle sugar
impl Handle<Lfo> {
    pub fn out(&self) -> PortRef   { self.port(LfoPort::Out) }
    pub fn shape(&self) -> KnobRef { self.knob(LfoKnob::Shape) }
    // ...
}
```

This is Style A from the appendix. Ships per-module, v1+, when friction surfaces.

### `oxurack::Module` (trait)

Re-homed from `oxurack_core::OxurackModule`. Every module declares two associated types that power the enum vocabulary:

```rust
pub trait Module {
    type Port;  // per-module enum, e.g., LfoPort
    type Knob;  // per-module enum, e.g., LfoKnob
    // ... existing module-trait methods
    fn set_param(&mut self, knob: Self::Knob, value: ParameterValue) -> Result<()>;
    // exact shape finalized in Bucket 3
}
```

Bucket 3 lands `set_param` (the gap CC surfaced in the Bucket 2 completion report) and commits the associated-type shape.

### `oxurack::Result`

```rust
pub type Result<T> = std::result::Result<T, oxurack::Error>;
```

where `oxurack::Error` is `oxurack_core::CoreError` re-exported at v0 (see "Open" below).

### `oxurack::modules`

Curated re-exports for module implementations that ship with the rack (v0: `Clock`, `Lfo`, `Scope`, `TuringMachine`, …).

---

## Semantics — what the verbs do

- **`install::<M>()`** — registers a module type with the internal `ModuleRegistry`. Replaces `register_module`. Still type-only; instances are created by `load_patch`.
- **`power_on` / `power_off`** — toggles `SimulationRunning`. Module tick systems only fire when powered. Power-off **does not clear state** — the rack is paused, not reset. Independent of transport.
- **`start` / `stop` / `pause` / `resume`** — transport state, mirroring MIDI's START / STOP / CONTINUE messages plus an added `pause`. `start()` begins from position 0 (sends MIDI START). `stop()` halts and resets position to 0 (sends MIDI STOP). `pause()` halts without resetting (sends MIDI STOP). `resume()` resumes from paused position (sends MIDI CONTINUE). Clean split: `start`/`stop` is the "reset" pair, `pause`/`resume` is the "preserve" pair. At v0, drives a `TransportState` resource (running-flag + position counter) that modules can read; full sequencer integration comes later.
- **`reset`** — "get me out of a stuck state" button. **Narrow scope:** emits All-Notes-Off to every installed module and clears the pending command queue. Does **not** touch knob values, cables, transport state, or power. For heavier wipes: `stop()` for transport reset; `load_patch()` for a full reload. `reset` rather than `panic` because `panic` collides with Rust's catastrophic-error vocabulary.
- **`patch` / `unpatch`** — runtime cable edits. Mutates the same `PatchHandle` that `load_patch` populates. At v0, `source` and `target` use `"module.port"` dotted syntax.
- **`set_knob`** — synchronous single-parameter setter. Dispatches to `Module::set_param` (new, Bucket 3). Queue-based variants stay internal for now.
- **`into_app`** — consuming conversion. The user receives `bevy_app::App` and the `Oxurack` wrapper is gone. No ref-out, no lifetime smuggling. Also the path to `App::run()` for the rare headless-daemon case: `rack.into_app().run()`.

---

## Migration: Bucket 2 → v1 Names

| Bucket 2 (landed in `2583d40`) | v1 |
|---|---|
| `OxurackApp` | `Oxurack` |
| `OxurackApp::new` | `Oxurack::new` |
| `register_module` | `install` |
| `bevy_app_mut` | *removed* |
| `bevy_app` | *removed* (use `into_app()`) |
| `run` | *removed* (use `rack.into_app().run()` for headless-daemon case) |
| `step` | unchanged |
| `load_patch` | unchanged |
| `pub use oxurack_core;` | explicit re-exports (already in the Bucket 2 follow-up queue) |

New surface (not present in Bucket 2): `power_on`, `power_off`, `start`, `stop`, `pause`, `resume`, `reset`, `patch`, `unpatch`, `set_knob`, `into_app`.

---

## Locked vs. Open

### Locked (Duncan +1, 2026-04-20 across two design sessions)

- All names in the "Confirmed Naming Decisions" table (including the enum-vocabulary and transport amendments).
- `into_app()` as consuming escape hatch; `bevy_app_mut()`, `bevy_app()`, and `run()` removed from v0 surface (available via `into_app().run()`).
- `Module` trait lives in `oxurack::` with `type Port` and `type Knob` **mandatory** associated types.
- `set_knob` accepts `impl Into<KnobRef>` and `impl Into<ParameterValue>` — one call handles scalars (`f32`) and value enums (`Waveform`, etc.) via `From` impls.
- Four surface styles share **one** per-module enum vocabulary. Styles B (handle + enums), C (inline tuples), and D (strings) ship at v0. Style A (typed port/knob methods) is per-module opt-in sugar deferred to v1+.
- `rack.module::<M>(name) -> Result<Handle<M>>` is the handle lookup; `Handle<M>::{port, knob, set}` are the v0 handle methods.
- **Transport:** `start` / `stop` / `pause` / `resume` — four verbs mirroring MIDI START / STOP / CONTINUE plus `pause`. `start`/`stop` reset position; `pause`/`resume` preserve. `resume` (not `continue`) because `continue` is a Rust keyword.
- **Emergency:** `reset()` with narrow scope — All-Notes-Off to every module + clear pending command queue. Does not touch knobs, cables, transport, or power.
- **ECS component rename:** `oxurack_core::Module` → `oxurack_core::ModuleInstance`. Avoids collision with both the `oxurack::Module` trait re-export *and* the pre-existing `ModuleId` identity component.
- **Error type:** direct re-export of `CoreError` as `oxurack::Error` at v0.
- **Value enums** live in `oxurack::values` at v0; promote to a dedicated crate if the set grows.
- **String-path typo errors** ship helpful "did you mean?" messages naming the available ports/knobs.

### Open — decide before Bucket 3 lands

*(empty — all previously-open questions resolved in the 2026-04-20 design sessions)*

### Out of scope for this memo

- MIDI I/O surface (separate design pass).
- UI / rendering surface (separate design pass).
- Full `Module` trait shape including the new parameter setter (that is the Bucket 3 decision doc).

---

## Impact on the Implementation Queue

1. **Bucket 2 follow-ups** (already queued — deterministic schedule test, explicit re-exports, `load_patch` refactor) **should absorb the `OxurackApp → Oxurack` and `register_module → install` renames** as part of the explicit-re-exports pass. One breaking change, one bucket.
2. **Bucket 3** (`SetParameter` dispatch) becomes *implement `set_knob`*. Same plumbing, hardware name.
3. **Bucket 3.5** *(new, low-complexity)*: lifecycle + transport + reset — `power_on`/`power_off`, `start`/`stop`/`pause`/`resume`, `reset`, plus the `TransportState` resource (running-flag + position counter). Likely a single bucket; the Semantics section above is the spec.
4. **Cable ops bucket** (new): `patch` / `unpatch` runtime edits. Separate from Bucket 3 because it needs cable-add/remove semantics in `PatchHandle`, which Bucket 2 did not build.

---

## Style Notes

- No `_mut` in public method names. Mutability is obvious from `&mut self`. `bevy_app_mut` read like it was embarrassed to exist; the new design owns the escape.
- No `Builder` or `With*` at v0. The positioning is "pick it up and play." Ceremony can be added later if the surface demands it.
- Docstrings on every public method should open with the hardware analogue, not the Bevy implementation. Example: `/// Flip the power switch on. Modules begin processing their tick systems on the next step.` — not `/// Set SimulationRunning to true.`

---

## Appendix A: Surface Examples

Same scenario across all four styles: an LFO modulates a filter cutoff; a scope watches the LFO; the LFO is set to a slow sine at 0.5 Hz.

### Style A — full typed handles with port/knob methods *(v1+ sugar, per-module opt-in)*

```rust
let lfo    = rack.module::<Lfo>("lfo_1")?;
let filter = rack.module::<Filter>("filter_1")?;
let scope  = rack.module::<Scope>("scope_1")?;

rack.patch(lfo.out(), filter.cutoff_cv())?;
rack.patch(lfo.out(), scope.input())?;

lfo.shape().set(Waveform::Sine)?;
lfo.rate().set(0.5)?;
```

### Style B — handle + per-module enums *(v0)*

```rust
let lfo    = rack.module::<Lfo>("lfo_1")?;
let filter = rack.module::<Filter>("filter_1")?;
let scope  = rack.module::<Scope>("scope_1")?;

rack.patch(lfo.port(LfoPort::Out), filter.port(FilterPort::CutoffCv))?;
rack.patch(lfo.port(LfoPort::Out), scope.port(ScopePort::In))?;

lfo.set(LfoKnob::Shape, Waveform::Sine)?;
lfo.set(LfoKnob::Rate, 0.5)?;
```

### Style C — inline tuples, no handles *(v0)*

```rust
rack.patch(("lfo_1", LfoPort::Out), ("filter_1", FilterPort::CutoffCv))?;
rack.patch(("lfo_1", LfoPort::Out), ("scope_1", ScopePort::In))?;

rack.set_knob("lfo_1", LfoKnob::Shape, Waveform::Sine)?;
rack.set_knob("lfo_1", LfoKnob::Rate, 0.5)?;
```

### Style D — pure strings *(v0; required for loaded patches / CLI / MIDI-learn)*

```rust
rack.patch("lfo_1.out", "filter_1.cutoff_cv")?;
rack.patch("lfo_1.out", "scope_1.in")?;

rack.set_knob("lfo_1", "shape", "sine")?;
rack.set_knob("lfo_1", "rate", 0.5)?;
```

### Where each style wins

- **Style B** — multi-touch on one module; three knob edits on `lfo` don't repeat the module name.
- **Style C** — quick one-shots; no binding ceremony.
- **Style D** — loaded patches, CLI, MIDI-learn callbacks, anywhere module/port identity comes from data rather than code.
- **Style A** *(v1+)* — maximum polish for modules whose author has chosen to ship the sugar.

All four styles share the same per-module enum vocabulary. `PortRef` / `KnobRef` / `ModuleRef` are internal coercion targets; users never name them directly.

---

## Appendix B: Why the Overlapping Maxima Is Cheap

Committing to B + C + D together does not cost three implementations. The underlying machinery is the same:

- One `PortRef` type. `impl From<(S, M::Port)> for PortRef where S: Into<ModuleRef>, M: Module` gives Style C. `Handle<M>::port(p: M::Port) -> PortRef` gives Style B. `impl From<&str> for PortRef` gives Style D. One set of storage; three constructors.
- One `KnobRef` type, same pattern.
- One `set_param` trait method on `Module`. All three styles lower to it.
- `Handle<M>` is a thin wrapper around `(Entity, PhantomData<M>)`. The generic param is just a token that constrains which enum variants are legal on the handle's methods — zero runtime cost.

So the "both feel best" decision is not a compromise — it's strictly additive over any single-style commitment.
