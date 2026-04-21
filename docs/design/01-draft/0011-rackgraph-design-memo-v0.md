---
number: 11
title: "RackGraph — Design Memo (v0)"
author: "** Duncan + Claude (Cowork), April 2026."
component: All
tags: [change-me]
created: 2026-04-21
updated: 2026-04-21
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# RackGraph — Design Memo (v0)

**Status:** Draft, under review with Duncan. Not yet handed to CC.
**Companion to:** `oxurack-public-api-design.md` (public API memo). This memo focuses on the introspection/navigation layer that sits on top of the public API.
**Author:** Duncan + Claude (Cowork), April 2026.

---

## Positioning

A rack is a directed graph. Modules are nodes. Cables are edges. Ports and knobs are typed attachment points. Once that framing is accepted, everything the REPL and TUI want to do — "what's in my rack?", "what's connected to what?", "what feeds this filter?", "where does this LFO go?" — is a query over a graph.

This memo specifies:

- The **event substrate** in `oxurack-core` that emits mutation notifications whenever rack structure changes. Always on, data-structure-agnostic, consumable by any subsystem.
- The **RackGraph** — a materialized view (petgraph-backed) that consumes the event substrate and answers navigation queries in `O(1)` / `O(deg)`. Lives in `oxurack-graph`, a separate feature-gated crate.
- The **literate patching layer** — optional labels, descriptions, authorship, notes, and tags attached to the rack and every component inside it. Lives in `oxurack-core`. Travels with the patch file. Turns a patch from a wiring diagram into a documented wiring diagram.

Three distinct concerns, three distinct locations. Events and labels ship with every rack ("dumb rack"). The graph is opt-in ("smart rack"). Orthogonality is a design constraint, not a happy accident.

---

## Confirmed Naming Decisions

| Concept | Name | Rationale |
|---|---|---|
| Crate | `oxurack-graph` | Feature-gated; pulls in petgraph; not part of "dumb rack" baseline |
| Resource | `RackGraph` | Scope is rack, not patch — outlives any single `load_patch` call |
| Accessor | `rack.graph()` | Returns `&RackGraph`; all navigation queries chain from here |
| Node identity | `ModuleId` (existing) | Already the deterministic module identity in `oxurack-core` |
| Edge kinds | `Signal`, `Modulation` | Two edge types; see "Two edge kinds" below |
| Label component | `DisplayName(String)` | Optional ECS component; attaches to any labelable entity |
| Description component | `Description(String)` | Optional ECS component; attaches to any labelable entity |
| Tag component | `Tags(Vec<String>)` | Optional; multi-tag per entity; powers filter views |
| Free-floating note | `Note { text, created_at }` | Owned by the rack, not a component |
| Cargo feature | `graph` (on umbrella `oxurack`) | Users opt in via Cargo.toml; method `rack.graph()` appears conditionally |

## Locked Decisions

- **Push-updated index, not pull-on-demand.** The graph is a materialized view. A Bevy system in `PreUpdate` (or a dedicated `GraphMaintenance` schedule) reacts to events emitted by core and updates the index in place. Queries are pure reads. *Why:* ergonomic uniformity (REPL/TUI call a method, get a shaped answer) + predictable cost (TUI at 60 Hz doesn't re-traverse). One escape hatch — `rack.live_query()` — exists for queries the cached index doesn't serve.
- **Events live in `oxurack-core`, always emitted.** The dumb rack emits; the smart rack consumes. Events are a cross-cutting substrate; the graph is one consumer among several (TUI, logging, telemetry, future patch-recorder).
- **`oxurack-graph` is a separate feature-gated crate.** Integration test matrix exercises both default build and `--features graph` build. A rack shipped without the graph feature still works, still emits events, still loads patches. Adding the feature layers on navigation without altering runtime behavior.
- **`StableDiGraph` from petgraph, not `DiGraph`.** Node indices must survive module removal; users of the graph (REPL, TUI) hold references across ticks. The memory cost is negligible at rack scale.
- **Two edge kinds, one graph.** `Signal` and `Modulation` coexist in one `StableDiGraph<RackNode, RackEdge>`. Algorithms that want one kind filter at query time.
- **Modulation edges do not constrain topological order.** Topo-sort considers only `Signal` edges. Modulation is allowed to form cycles (intentional feedback is legitimate); those are reported separately.
- **Freshness contract: the graph is current after every `rack.mutate(...)` call returns.** `load_patch`, `install`, `add_cable`, `remove_cable`, `set_knob` — all run the graph-maintenance schedule synchronously before returning. No "call `.step()` to see your changes" surprises.
- **Query results are serializable plain structs.** `ModuleInfo`, `CableInfo`, `PortInfo`, `SubgraphInfo`. In-process callers can additionally resolve these to typed `Handle<M>` via `rack.handle_of(module_info.id)`. Out-of-process and cross-network consumers (future) get the same shapes over the wire.
- **Display names are not required unique.** Instance names remain the unique key. Display names are presentational; two modules can both be "LFO" without error.
- **Labels serialize into patch files.** A patch file is not just wiring + parameter values; it carries display names, descriptions, tags, and notes. Literate patching is a file-format property, not a runtime-only affordance.

## Open

None at time of writing. If something surfaces during implementation, it gets logged here as a question before CC makes a judgment call.

---

## Scope of v0

Duncan's directive: *everything*. The full query catalog and the full literate-patching layer ship in v0 so rough edges surface early. Specifically:

**Navigation queries (from RackGraph):**

1. Unique module kinds present (multiset: `HashMap<ModuleKind, usize>`).
2. Full module roster (`Vec<ModuleInfo>`).
3. Ports per module (split: inputs, outputs).
4. Knobs per module.
5. Upstream neighbors of a module (node-level: `Vec<ModuleInfo>`).
6. Downstream neighbors of a module (node-level).
7. Upstream edges of a module (edge-level: `Vec<CableInfo>`).
8. Downstream edges of a module (edge-level).
9. Cables in the rack (full edge list).
10. Dangling modules (zero cables touching).
11. Dangling ports (output with no outgoing cable; input with no incoming cable).
12. Topological order of modules (signal edges only).
13. Feedback cycles (signal: returned as "cycles exist but patch is intentional or broken — you decide"; modulation: reported separately, always allowed).
14. Reachability: subgraph downstream of a node (`Dfs` traversal).
15. Reachability: subgraph upstream of a node (`Dfs` on reversed edges).
16. Port-compatibility filter: given an output port, list all input ports in the rack it could legally connect to (by `ValueKind`).
17. Name lookup: `rack.module("vco_1") → Handle<?>`.
18. Kind lookup: `rack.modules_of_kind::<LFO>() → Vec<Handle<LFO>>`.

**Literate-patching surface (from core, available regardless of graph feature):**

1. `DisplayName` on rack, modules, ports, knobs, cables.
2. `Description` on rack, modules, ports, knobs, cables.
3. `Tags` on modules (modules only for v0; tag-on-ports is v1 if needed).
4. `RackMetadata` — authorship, created_at, modified_at, version, parent_patch (optional provenance link).
5. `Notes` — free-floating commentary owned by the rack, not attached to any single component. `Vec<Note>` on the rack. Ordered by creation time.
6. Default display-name formatter — configurable on the rack. Defaults to `snake_case → Title Case` (`vco_1` → "VCO 1"). Users can override the formatter closure for custom rules.
7. Full serialization round-trip: load a patch with labels, save it, every label preserved.

**Events (from core, always emitted):**

1. `ModuleAdded { id, kind, instance_name }`
2. `ModuleRemoved { id }`
3. `CableConnected { id, kind: Signal|Modulation, src, dst }`
4. `CableDisconnected { id }`
5. `KnobChanged { module, knob, old, new }` (for the existing parameter-change flow)
6. `LabelChanged { target, field: DisplayName|Description|Tags }`
7. `NoteAdded { note_id }` / `NoteRemoved { note_id }`
8. `PatchLoaded { path, module_count, cable_count }`

v1 (explicitly deferred): tag-on-ports, hierarchical groupings beyond tags, visual hints/color, version history beyond single `version` field, cross-process IPC transport, and any graph query that benefits from having real-world usage data before being locked in.

---

## Architecture

### Crate layout

```
oxurack-core/           ← always shipped
  ├─ events.rs          ← the event enum + emission machinery
  ├─ labels.rs          ← DisplayName, Description, Tags, Note, RackMetadata
  ├─ formatter.rs       ← default + configurable display-name formatter
  └─ (existing)         ← Module trait, ModuleRegistry, PatchHandle, etc.

oxurack-graph/          ← feature-gated, depends on oxurack-core + petgraph
  ├─ graph.rs           ← RackGraph resource, petgraph storage
  ├─ maintenance.rs     ← systems reading events, updating graph
  ├─ queries.rs         ← the 18 queries listed above
  └─ types.rs           ← ModuleInfo, CableInfo, PortInfo, SubgraphInfo

oxurack/                ← umbrella
  Cargo.toml:
    [features]
    default = ["graph"]
    graph = ["dep:oxurack-graph"]
  src/lib.rs:
    #[cfg(feature = "graph")]
    impl Oxurack { pub fn graph(&self) -> &RackGraph { ... } }
```

Integration-test matrix: default build (includes `graph`), and a `--no-default-features` build (dumb rack). Both run the full test suite; the graph crate has its own test suite on top.

### Event substrate

Events are plain structs emitted via Bevy's `EventWriter<E>` from whatever system performs the mutation. In `oxurack-core`:

```rust
#[derive(Event)]
pub struct ModuleAdded {
    pub id: ModuleId,
    pub kind: ModuleKind,
    pub instance_name: String,
}
// ...and siblings
```

Emission points:

- `install<M>` emits `ModuleAdded` (if the module is also spawned as an entity at install time — clarify during implementation).
- `load_patch` emits `PatchLoaded`, then iterates the patch and emits `ModuleAdded` / `CableConnected` for each row. All synchronously before `load_patch` returns.
- `add_cable` / `remove_cable` (v0 API: see Public Surface below) emit `CableConnected` / `CableDisconnected`.
- Parameter-change systems emit `KnobChanged`.
- Label setters emit `LabelChanged`.

The graph's maintenance system reads these events in `PreUpdate` and updates the index. Any other consumer (TUI, logger) registers its own `EventReader` and reads the same stream. No consumer blocks any other.

### RackGraph internals

```rust
pub struct RackGraph {
    inner: StableDiGraph<RackNode, RackEdge>,
    by_module_id: HashMap<ModuleId, NodeIndex>,
    by_instance_name: HashMap<String, ModuleId>,
    by_kind: HashMap<ModuleKind, Vec<ModuleId>>,
    cable_index: HashMap<CableId, EdgeIndex>,
    // ...small caches for the hottest lookups
}

pub struct RackNode {
    pub id: ModuleId,
    pub kind: ModuleKind,
    // Copies of the labels are *not* stored here — always fetched
    // fresh from the ECS world via the query layer. Avoids label
    // staleness.
}

pub enum RackEdge {
    Signal {
        cable_id: CableId,
        src_port: PortId,
        dst_port: PortId,
    },
    Modulation {
        cable_id: CableId,
        src_port: PortId,
        dst_knob: KnobId,
    },
}
```

The graph stores **structural** information (who connects to whom). Labels and current values are kept in ECS; query-result builders read from both sources when assembling `ModuleInfo` / `CableInfo` structs.

### Query surface sketch

```rust
impl RackGraph {
    // Rosters
    pub fn modules(&self) -> Vec<ModuleInfo>;
    pub fn module_kinds(&self) -> HashMap<ModuleKind, usize>;
    pub fn modules_of_kind(&self, kind: ModuleKind) -> Vec<ModuleInfo>;
    pub fn cables(&self) -> Vec<CableInfo>;

    // Neighborhoods
    pub fn upstream_of(&self, module: ModuleId) -> Vec<ModuleInfo>;
    pub fn downstream_of(&self, module: ModuleId) -> Vec<ModuleInfo>;
    pub fn incoming_cables(&self, module: ModuleId) -> Vec<CableInfo>;
    pub fn outgoing_cables(&self, module: ModuleId) -> Vec<CableInfo>;

    // Diagnostics
    pub fn dangling_modules(&self) -> Vec<ModuleInfo>;
    pub fn dangling_ports(&self) -> Vec<(ModuleInfo, PortInfo)>;
    pub fn signal_cycles(&self) -> Vec<Vec<ModuleId>>;
    pub fn modulation_cycles(&self) -> Vec<Vec<ModuleId>>;  // reported, not errors

    // Ordering
    pub fn signal_order(&self) -> Result<Vec<ModuleInfo>, CycleError>;

    // Reachability
    pub fn downstream_closure(&self, module: ModuleId) -> Vec<ModuleInfo>;
    pub fn upstream_closure(&self, module: ModuleId) -> Vec<ModuleInfo>;

    // Connection discovery
    pub fn compatible_inputs_for(&self, src_port: PortId) -> Vec<(ModuleInfo, PortInfo)>;

    // Lookup
    pub fn module_by_name(&self, name: &str) -> Option<ModuleInfo>;
}
```

Result structs resolve labels at query time:

```rust
pub struct ModuleInfo {
    pub id: ModuleId,
    pub kind: ModuleKind,
    pub instance_name: String,
    pub display_name: String,    // falls back to formatter if no DisplayName component
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub ports: PortSet,          // inputs + outputs split
    pub knobs: Vec<KnobInfo>,
}
```

### Freshness

Every `Oxurack` method that mutates rack state synchronously runs the graph maintenance schedule before returning. Concretely: `load_patch`, `install`, `add_cable`, `remove_cable`, `set_knob` (and equivalents) end with `self.app.world_mut().run_schedule(GraphMaintenance)` when the `graph` feature is enabled. This is a no-op when the feature is disabled.

The call `rack.graph().modules()` immediately after `rack.load_patch(...)` sees every just-loaded module. No polling, no sleep, no "did I forget to step?".

---

## Literate Patching Layer

This is the part that turns a patch from a wiring diagram into a narrative.

### Labels on every component

`DisplayName(String)` and `Description(String)` are optional Bevy ECS components that can attach to any entity the user might want to label: the rack itself, modules, ports, knobs, cables. Absence means "fall back to the formatter." Presence overrides.

Setter surface (sketch):

```rust
rack.set_name("Duncan's Berlin School Rack");
rack.set_description("April 2026 ambient sketches.");
rack.set_tags(["berlin-school", "ambient", "wip"]);

rack.module("lfo_1")
    .set_display_name("Bass Wobble")
    .set_description("The slow sweep driving the bass filter.");

rack.module("lfo_1").port("out").set_display_name("wobble out");
rack.cable("lfo_1.out → filter_1.cutoff").set_description("this is where the character comes from");
```

### Rack-level metadata

```rust
pub struct RackMetadata {
    pub author: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub version: u32,
    pub parent_patch: Option<String>,  // provenance: "this was derived from X"
}
```

Stored as a resource. Serializes into the patch file as a metadata block. `modified_at` is auto-updated on save.

### Notes — free-floating commentary

```rust
pub struct Note {
    pub id: NoteId,
    pub text: String,
    pub created_at: DateTime<Utc>,
}
```

The rack holds `Vec<Note>`. Notes are not attached to any single component — they're commentary on the rack as a whole, or on design decisions, or TODO items. The user writes them; the patch file carries them; a future TUI displays them in a side panel.

```rust
rack.add_note("The filter cutoff is intentionally modulated hard — don't touch.");
rack.add_note("TODO: try the delay on send 2 instead of send 1.");
```

### Tags

`Tags(Vec<String>)` on modules. Powers filter views: "show me everything tagged `bass`", "hide everything tagged `wip`". In v0 tags are module-only; port-tagging is v1 if we ever need it.

### Display-name formatter

A configurable closure stored on the rack:

```rust
pub type NameFormatter = Arc<dyn Fn(&str) -> String + Send + Sync>;

rack.set_name_formatter(|instance_name| {
    // custom rule: "vco_1" → "Oscillator #1"
    ...
});
```

Default: `snake_case_with_trailing_number` → `Title Case With Space Number` (`vco_1` → "VCO 1", `low_pass_filter_2` → "Low Pass Filter 2"). Acronyms (VCO, LFO, ADSR) are recognized via a small table; users can extend.

### Serialization

Patch files (existing `.oxurack` format or whatever the final shape is) gain a `labels` section and a `metadata` section:

```yaml
metadata:
  author: "Duncan"
  created_at: "2026-04-19T12:34:56Z"
  version: 3
  parent_patch: "berlin-school-wip-v2"
name: "Duncan's Berlin School Rack"
description: "April 2026 ambient sketches."
notes:
  - text: "The filter cutoff is intentionally modulated hard — don't touch."
    created_at: "2026-04-19T13:00:00Z"
labels:
  modules:
    lfo_1:
      display_name: "Bass Wobble"
      description: "The slow sweep driving the bass filter."
      tags: ["bass"]
    filter_1:
      display_name: "Main Filter"
  ports:
    lfo_1.out:
      display_name: "wobble out"
  cables:
    lfo_1.out-filter_1.cutoff:
      description: "this is where the character comes from"
modules: [...]  # existing wiring data
cables: [...]
```

All labels are optional. A label-free patch file loads as a normal rack where `module.display_name()` falls back to the formatter. A labeled patch loads as a documented rack where every label is available in REPL, TUI, and via the graph query surface.

---

## Cross-process preparedness

Every query result type (`ModuleInfo`, `CableInfo`, `PortInfo`, `SubgraphInfo`, `Note`, `RackMetadata`) is `Serialize + Deserialize` via serde. No `Entity` handles, no lifetime-tied references leak into result types.

An in-process caller who wants richer access can resolve a `ModuleInfo.id` back to a `Handle<M>` via `rack.handle_of(id)`. An out-of-process caller gets the serializable snapshot and stops there.

Concretely: a future `oxurack-server` crate could expose the full query surface over any transport (gRPC, WebSocket, Unix socket, in-memory channel) without restructuring `oxurack-graph`. That's the test.

---

## Migration / Interaction with existing work

**Bucket 2 follow-ups (already landed):** no conflict. `Oxurack`, `install`, `into_app`, curated re-exports, deterministic test — all compatible. The graph crate will add `oxurack_graph::RackGraphPlugin` that `Oxurack::new()` installs when the `graph` feature is enabled. Otherwise the umbrella is unchanged.

**Bucket 3 (upcoming):** the graph work belongs in a new bucket — call it **Bucket 4** — after Bucket 3's core parameter-setting work lands. Rationale: label-setting methods want the same ergonomic shape as knob-setting methods, so doing Bucket 3 first gives us a reference surface to mirror. Also: labels land in core as optional components, which is small; the full graph crate is the heavier lift and benefits from a clean starting point.

**Labels in core are a Bucket 3.5 candidate.** Small enough to bundle adjacent to parameter-setting work; independent enough to stand on its own. Duncan's call whether to split.

**Events are a prerequisite.** The graph depends on the event substrate existing. Events should land in core as their own bucket — call it **Bucket 3.25** — before the graph crate starts. They're useful to other subscribers (TUI, logging) even before the graph exists.

Proposed bucket ordering:
1. **Bucket 3** — parameter-setting (already designed in the API memo).
2. **Bucket 3.25** — event substrate in `oxurack-core`. Emission points, `#[derive(Event)]` structs, basic test that events fire.
3. **Bucket 3.5** — labels in `oxurack-core`. Optional components, formatter, setters, serialization round-trip.
4. **Bucket 4** — `oxurack-graph` crate. RackGraph resource, maintenance system, 18-query surface, full test suite, feature-flag integration.

---

## Appendix A — Example REPL session

```rust
let mut rack = Oxurack::new();
rack.install::<Lfo>().install::<Filter>().install::<Mixer>();
rack.load_patch(Path::new("berlin-school.oxurack"))?;

// What's in the rack?
for m in rack.graph().modules() {
    println!("{} ({})", m.display_name, m.kind);
}
// Bass Wobble (Lfo)
// Main Filter (Filter)
// Final Mix (Mixer)

// What feeds the filter?
for m in rack.graph().upstream_of(rack.graph().module_by_name("filter_1").unwrap().id) {
    println!("  ← {}", m.display_name);
}
// ← Bass Wobble

// Anything dangling?
for m in rack.graph().dangling_modules() {
    println!("unconnected: {}", m.display_name);
}

// What notes did I leave myself?
for note in rack.notes() {
    println!("// {}", note.text);
}
// // The filter cutoff is intentionally modulated hard — don't touch.
// // TODO: try the delay on send 2 instead of send 1.

// Signal flow order
for (i, m) in rack.graph().signal_order()?.iter().enumerate() {
    println!("{}. {}", i + 1, m.display_name);
}
```

## Appendix B — Why this composition is load-bearing

Three things coexist in v0 that are often collapsed into one in other frameworks:

- **Events** are the substrate. Always on. Data-structure-agnostic. Emitted by core, consumed by many.
- **Labels** are the literate layer. Optional, pervasive, purely human-facing, serialize with patches.
- **Graph** is the materialized view. Opt-in, algorithmically rich, consumes events.

Keeping them separate means:
- A headless audio server can run the dumb rack with events for logging, no graph, no labels — tiny footprint.
- A REPL with labels but no graph is useful on its own (names, descriptions, notes, serialization).
- A TUI with graph but minimal labels is useful on its own (navigation, diagnostics).
- A full experience combines all three and is more than the sum of its parts.

The orthogonality is the design. If any pair becomes entangled during implementation — e.g., "the graph needs labels inline in the node" — that's a smell; back out and re-separate.
