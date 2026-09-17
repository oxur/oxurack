# Bucket 4 (part 1): RackGraph foundation

## Context

Bucket 4 builds the **`oxurack-graph` crate** — a feature-gated materialized view over the rack's structural state, consuming the event substrate that shipped in 3.25 + 3.5. Part 1 establishes the foundation: new crate scaffolding, the `RackGraph` resource backed by petgraph's `StableDiGraph`, plain-struct result types with serde derives, the plugin and maintenance system, feature-gate wiring on the umbrella crate, and basic add-and-see-it tests.

Part 2 (separate spec, drafted after part 1 ships) builds out the full query surface — 18 navigation queries from the design memo (upstream/downstream, dangling, cycles, topological order, reachability, port compatibility, etc.) plus the dense per-query test suite.

**Why split.** Bucket 4 is large in absolute scope (a new crate + ~18 queries + tests for each + edge cases + feature-gate wiring + two-mode test matrix). Bucket 3.5 taught us that single-session CC has context limits when total scope crosses a threshold; pre-baking the split avoids mid-implementation pressure and produces a cleaner git history. Part 1 is foundation that compiles standalone with self-contained tests; part 2 layers the navigation surface on top.

The architecture is locked in **`workbench/rackgraph-design.md`** (refreshed 2026-05-10 against shipped 3.25/3.5 substrate). Read it before recon. The reconciliation section at the top of that memo is particularly important — several specifics evolved from the original draft.

---

## Required reading before recon

- **`workbench/rackgraph-design.md`** — design memo, refreshed 2026-05-10. The "Post-3.25/3.5 reconciliation" section at the top is the most current architectural truth.
- **`workbench/event-substrate-design.md`** — bucket 3.25 design memo. Defines the events the graph subscribes to.
- **`workbench/literate-patching-design.md`** — bucket 3.5 design memo. Defines the `RackMeta` / `DisplayName` / `Tags` shapes that part 2 will use for label-resolving result builders.
- **`workbench/deferred-tracking.md`** — central deferred-items registry. `CableKind` (Signal vs Modulation) is D-25.7; deferred for v0 graph. Edges are undifferentiated in part 1.
- **`assets/ai/ai-rust/skills/claude/SKILL.md`** — Rust skill. Particularly `12-project-structure.md` for new-crate scaffolding patterns and `15-cargo/02-cargo-build-system.md` for feature-gate conventions.

---

## Recon findings (as of 2026-05-10, verify against the current tree)

**HEAD is `ce10c39`** ("Bucket 3.5 (part 2): patch integration"). Re-verify if newer commits land before this bucket starts.

### Workspace structure

- **`Cargo.toml`** (workspace root) has `members = ["crates/*"]` (glob). Adding a new crate at `crates/oxurack-graph/` is automatic — no `members` list edit needed.
- **`resolver = "2"`** workspace-level.
- **`crates/oxurack/Cargo.toml`** (umbrella) does **not** currently have a `[features]` section. Part 1 adds one with `default = ["graph"]` and `graph = ["dep:oxurack-graph"]`.
- **`crates/oxurack-core/Cargo.toml`** uses Bevy 0.18 (`bevy_ecs`, `bevy_app`). Confirm the same for `oxurack-graph`.
- **Existing crate count:** verify via `ls crates/` — should be at least `oxurack-core`, `oxurack`, possibly `oxurack-rt`.

### Events the graph subscribes to (already shipped)

All in `oxurack-core::event`, derive `Message`, exported from crate root:

- **`ModuleAdded { entity, instance_name, kind }`** — active emission site: `apply_patch_to_world` per module. Graph adds a node.
- **`ModuleRemoved { entity, instance_name, kind }`** — type declared, no active emission site today. Graph subscribes for forward compatibility; emission lands when `remove_module` API exists. (Part 1 wires the subscription; verifies it doesn't panic with zero emissions; no behavior to test beyond "subscribe doesn't break.")
- **`CableConnected { cable_entity, source_port, target_port, transform }`** — active emission site: `apply_patch_to_world` per cable. Graph adds an edge.
- **`CableDisconnected { cable_entity, source_port, target_port }`** — type declared, no active emission site. Same deal as `ModuleRemoved`.
- **`ParameterChanged`** — graph does **not** subscribe. Parameters aren't structural.
- **`PatchLoaded`** — graph does **not** subscribe for maintenance. Could be used by TUI for "batch done, redraw" signaling, but the graph itself doesn't need it for index maintenance.
- **`LabelChanged`** — graph does **not** subscribe. Labels are resolved fresh from ECS at query time (per memo §RackGraph internals — "Copies of the labels are not stored here — always fetched fresh from the ECS world via the query layer. Avoids label staleness.").

### Node identity: `Entity`, not `ModuleId`

The original memo used `ModuleId` for node identity. The shipped events carry `Entity`. The graph uses `Entity`. `ModuleId` exists in the codebase (`module.rs:60`) but the substrate doesn't use it, so the graph doesn't either. If a future use case demands deterministic-across-runs identity, `ModuleId` can become a parallel lookup; for v0, `Entity` is sufficient.

### Edge kind: undifferentiated for v0

`CableKind` (Signal vs Modulation) was deferred per D-25.7. Cables in the codebase are uniform `(source_port, target_port, transform: Option<CableTransform>)` tuples. Part 1's `RackEdge` is a single struct with no kind discriminator. When `CableKind` lands later, `RackEdge` grows a `kind` field non-breakingly via `#[non_exhaustive]`.

### Bevy 0.18 Plugin conventions

Part 1 follows the existing `CorePlugin` / `CorePluginFixed` shape in `crates/oxurack-core/src/lib.rs:122-194`:

- Resource registrations via `init_resource::<T>()`.
- Message subscriptions via `MessageReader<T>` in systems.
- Systems registered with `Update` or a custom schedule (the memo proposes a dedicated `GraphMaintenance` schedule; verify Bevy 0.18 convention for custom schedule labels).
- Plugin's `build(&self, app: &mut App)` does all the wiring.

### Existing test patterns

- Tests for events use `world.resource::<Messages<T>>().iter_current_update_messages()`.
- Tests use `pretty_assertions::assert_eq!` from dev-dependencies.
- Test helper pattern: `setup_set_parameter_world()` (`event.rs:608+`), `test_app()` / `test_registry()` / `valid_patch()` (`patch.rs` test module).

For graph tests, the natural setup is:
1. Create a `bevy_app::App`.
2. Add `CorePlugin` (or whatever brings the message resources).
3. Add `RackGraphPlugin`.
4. Apply a patch via `apply_patch_to_world` — emits `ModuleAdded` and `CableConnected`.
5. Run the schedule once to let `GraphMaintenance` systems consume the messages.
6. Read `RackGraph` resource, assert structure.

---

## Locked decisions (from design memo; do not re-litigate)

| # | Decision |
|---|----------|
| L1 | New crate `oxurack-graph` at `crates/oxurack-graph/`, depends on `oxurack-core` + `petgraph`. |
| L2 | Umbrella crate gains `[features] default = ["graph"]` block with `graph = ["dep:oxurack-graph"]`. |
| L3 | `RackGraph` resource backed by `petgraph::stable_graph::StableDiGraph<RackNode, RackEdge>`. Stable indices required because consumers hold refs across ticks. |
| L4 | Push-update reactive maintenance via `MessageReader<T>` systems. No pull-on-demand. |
| L5 | Node identity: `Entity` (per reconciliation in design memo). |
| L6 | Edge undifferentiated for v0 (`CableKind` deferred per D-25.7). |
| L7 | Graph stores structure only; labels and parameter values fetched fresh from ECS at query time. |
| L8 | Freshness contract: graph current after every mutation-API call returns. Achieved by running `GraphMaintenance` schedule synchronously. |
| L9 | Plain-struct result types (`ModuleInfo`, `CableInfo`, `PortInfo`) with `Serialize + Deserialize` derives for cross-process preparedness. |
| L10 | `oxurack-graph` is a separate crate. Part 1 = scaffolding + maintenance; part 2 = query surface. |

---

## Work mode

**Thinking work stays in your context** with the Rust skill loaded. This includes: choosing the `GraphMaintenance` schedule label vs. a system set within an existing schedule; how to handle the freshness contract concretely (run schedule from inside the umbrella's mutation methods? rely on a system ordering constraint? a sentinel `GraphDirty` resource?); the exact `RackEdge` shape (newtype around `(Entity, Entity, Option<CableTransform>)`, or a struct with named fields); how to expose graph membership without giving access to petgraph internals; whether `RackGraphPlugin` should auto-install when `Oxurack::new()` is called or whether the user opts in.

**Lookup work may delegate to subagents.** This includes: finding existing schedule-label patterns in `oxurack-core`; greping for `init_resource` / `add_message` / `add_systems` usage; identifying which Bevy 0.18 prelude items are commonly imported in plugin code; counting all `apply_patch_to_world` test sites that will need to be re-runnable under the graph feature.

Serial execution on thinking. Quality over speed.

---

## The task

### 1. Crate scaffolding

Create `crates/oxurack-graph/` with:

```
crates/oxurack-graph/
├── Cargo.toml
└── src/
    └── lib.rs
```

**`Cargo.toml`:**

```toml
[package]
name        = "oxurack-graph"
version     = "0.1.0"
edition     = "2024"
description = "Materialized graph view over oxurack rack state — navigation queries via petgraph."
license     = "MIT OR Apache-2.0"
repository  = "https://github.com/oxur/oxurack"

[dependencies]
oxurack-core = { version = "0.1.0", path = "../oxurack-core" }
bevy_app     = { version = "0.18", default-features = false, features = ["std"] }
bevy_ecs     = { version = "0.18", default-features = false, features = ["std"] }
petgraph     = { version = "0.6", default-features = false, features = ["stable_graph"] }
serde        = { version = "1", features = ["derive"] }

[dev-dependencies]
pretty_assertions = "1"
```

Verify the petgraph version against what's current; 0.6+ has `stable_graph` as a feature.

### 2. Umbrella crate `[features]` wiring

Edit `crates/oxurack/Cargo.toml`:

```toml
[features]
default = ["graph"]
graph = ["dep:oxurack-graph"]

[dependencies]
# ... existing deps ...
oxurack-graph = { version = "0.1.0", path = "../oxurack-graph", optional = true }
```

In `crates/oxurack/src/lib.rs`, gate the plugin install:

```rust
#[cfg(feature = "graph")]
use oxurack_graph::RackGraphPlugin;

impl Oxurack {
    pub fn new() -> Self {
        let mut app = bevy_app::App::new();
        app.add_plugins(oxurack_core::CorePlugin);
        #[cfg(feature = "graph")]
        app.add_plugins(RackGraphPlugin);
        Self { app }
    }
    // ...
}
```

(Exact integration point depends on the current shape of `Oxurack::new()` — verify via recon.)

### 3. Core types in `oxurack-graph::graph`

```rust
use bevy_ecs::prelude::*;
use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use std::collections::HashMap;

/// Materialized graph view over the rack's structural state.
///
/// Maintained reactively by [`RackGraphPlugin`] subscribing to
/// `ModuleAdded` / `ModuleRemoved` / `CableConnected` / `CableDisconnected`
/// notifications from `oxurack-core`. Queries are pure reads.
///
/// Node identity is `Entity` (the ECS entity for a module). Edge
/// identity is the cable's `Entity`.
///
/// Labels and parameter values are *not* stored in the graph; they're
/// fetched fresh from the ECS world at query time. This avoids
/// label-staleness bugs.
#[derive(Resource, Default, Debug)]
pub struct RackGraph {
    pub(crate) inner: StableDiGraph<RackNode, RackEdge>,
    pub(crate) node_by_entity: HashMap<Entity, NodeIndex>,
    pub(crate) edge_by_entity: HashMap<Entity, EdgeIndex>,
    pub(crate) by_instance_name: HashMap<String, Entity>,
}

/// A module node in the rack graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RackNode {
    pub entity: Entity,
    pub instance_name: String,
    pub kind: oxurack_core::ModuleKind,
}

/// A cable edge in the rack graph.
///
/// v0 is undifferentiated — Signal vs Modulation distinction is
/// deferred (D-25.7). When `CableKind` ships, this struct grows a
/// `kind` field via `#[non_exhaustive]`.
#[derive(Clone, Debug, PartialEq)]
pub struct RackEdge {
    pub cable_entity: Entity,
    pub source_port: Entity,
    pub target_port: Entity,
    pub transform: Option<oxurack_core::CableTransform>,
}
```

Part 1 exposes minimal querying — enough to test add-and-see-it without committing to the full navigation surface (which is part 2):

```rust
impl RackGraph {
    /// Number of modules in the graph.
    pub fn module_count(&self) -> usize {
        self.inner.node_count()
    }

    /// Number of cables in the graph.
    pub fn cable_count(&self) -> usize {
        self.inner.edge_count()
    }

    /// Whether the given entity is registered as a module node.
    pub fn contains_module(&self, entity: Entity) -> bool {
        self.node_by_entity.contains_key(&entity)
    }

    /// Whether the given entity is registered as a cable edge.
    pub fn contains_cable(&self, entity: Entity) -> bool {
        self.edge_by_entity.contains_key(&entity)
    }

    /// Resolve an instance name to its module entity.
    pub fn entity_by_instance_name(&self, name: &str) -> Option<Entity> {
        self.by_instance_name.get(name).copied()
    }
}
```

Everything beyond these basic membership queries is **part 2 scope** — including `modules()`, `cables()`, `upstream_of`, `downstream_of`, `dangling_modules`, `signal_order`, `compatible_inputs_for`, and the label-resolving `ModuleInfo` / `CableInfo` / `PortInfo` result builders. Part 1 declares the placeholder types for these (next section) so part 2's spec can build on them without re-declaring.

### 4. Result type declarations (full part-2 use)

Declare the result types but don't build the queries that return them. Part 1 only needs to compile-check that the types are well-formed; part 2 wires them up to actual queries.

```rust
use serde::{Deserialize, Serialize};

/// Snapshot of a module's queryable state.
///
/// Built fresh at query time by reading the ECS world. Carries
/// label information so consumers don't need to query for labels
/// separately.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub entity_index: u64,       // Entity serialized as its index — see note below
    pub instance_name: String,
    pub kind: String,             // ModuleKind stringified for cross-process portability
    pub display_name: String,     // resolved via DisplayNameFormatter fallback
    pub description: Option<String>,
    pub tags: Vec<String>,
    // Port and knob detail comes in part 2.
}

/// Snapshot of a cable's queryable state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CableInfo {
    pub entity_index: u64,
    pub source_module_instance: String,
    pub source_port_name: String,
    pub target_module_instance: String,
    pub target_port_name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

/// Snapshot of a port on a module.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PortInfo {
    pub name: String,
    pub direction: PortDirection,
    pub value_kind: String,        // ValueKind stringified
}

/// Serializable port direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortDirection {
    Input,
    Output,
}
```

**Entity serialization note:** Bevy's `Entity` doesn't implement `Serialize` natively. The convention is to extract `Entity::to_bits()` → `u64` for serialization. The exact field name (`entity_index`, `entity_bits`, etc.) is a style choice — pick one and use it consistently. Document the rationale in a doc-comment.

### 5. `RackGraphPlugin` and maintenance system

```rust
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use oxurack_core::{CableConnected, CableDisconnected, ModuleAdded, ModuleRemoved};

pub struct RackGraphPlugin;

impl Plugin for RackGraphPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RackGraph>()
            .add_systems(
                Update,
                (
                    handle_module_added,
                    handle_module_removed,
                    handle_cable_connected,
                    handle_cable_disconnected,
                )
                    .chain(),
            );
    }
}

fn handle_module_added(
    mut graph: ResMut<RackGraph>,
    mut reader: MessageReader<ModuleAdded>,
) {
    for msg in reader.read() {
        let node = RackNode {
            entity: msg.entity,
            instance_name: msg.instance_name.clone(),
            kind: msg.kind.clone(),
        };
        let idx = graph.inner.add_node(node);
        graph.node_by_entity.insert(msg.entity, idx);
        graph.by_instance_name.insert(msg.instance_name.clone(), msg.entity);
    }
}

fn handle_module_removed(
    mut graph: ResMut<RackGraph>,
    mut reader: MessageReader<ModuleRemoved>,
) {
    for msg in reader.read() {
        if let Some(idx) = graph.node_by_entity.remove(&msg.entity) {
            graph.inner.remove_node(idx); // StableDiGraph: index of other nodes preserved
            graph.by_instance_name.remove(&msg.instance_name);
        }
    }
}

fn handle_cable_connected(
    mut graph: ResMut<RackGraph>,
    mut reader: MessageReader<CableConnected>,
) {
    for msg in reader.read() {
        // Cables connect ports, not modules directly. To add a graph edge,
        // we need to resolve port_entity -> parent_module_entity. This
        // requires a world query, which is *not* available in a system
        // taking only Res/ResMut params. Two options:
        //
        // (A) Take a Query<&ChildOf> in the system and resolve here.
        // (B) Defer resolution: store unresolved edges and resolve them
        //     in a follow-up system that has world access.
        //
        // Option A is cleaner — Bevy's ChildOf component links ports to
        // their parent modules. Recon to verify.
        //
        // [CC: pick a path during recon; document in completion report.]
    }
}

fn handle_cable_disconnected(
    mut graph: ResMut<RackGraph>,
    mut reader: MessageReader<CableDisconnected>,
) {
    for msg in reader.read() {
        if let Some(idx) = graph.edge_by_entity.remove(&msg.cable_entity) {
            graph.inner.remove_edge(idx);
        }
    }
}
```

**Implementation note on `handle_cable_connected`:** the `CableConnected` notification carries `source_port: Entity` and `target_port: Entity` — port entities, not module entities. To build a graph edge between *modules*, the system needs to resolve port → parent module. Two paths:

- **(A)** Take a `Query<&bevy_ecs::hierarchy::ChildOf>` parameter in the system, resolve `port_entity → module_entity` via the parent relationship. Cleaner, requires verifying the existing port-spawn machinery actually inserts `ChildOf`.
- **(B)** Defer edge insertion to a follow-up system that has world query access; store unresolved edges in a buffer on `RackGraph`. More indirection, but works even if the port hierarchy isn't reliable.

CC: pick during recon, document in the completion report. **Recon check:** verify that port entities have a `ChildOf` component pointing to their parent module. Reference: `port.rs:spawn_port_on_module` — see how ports are spawned and whether `ChildOf` is attached.

### 6. Schedule choice

The original memo proposed a dedicated `GraphMaintenance` schedule label. For part 1, the simpler `Update` schedule with `.chain()` is sufficient: events from the previous tick (or same tick if before the maintenance systems run) get consumed.

**However:** the freshness contract (memo §Freshness) says the graph must be current after every mutation-API call returns. This implies the maintenance systems run synchronously *during* the API call, not on the next tick. Two options:

- **(A) Defer the freshness contract to part 2.** Part 1 lands maintenance in `Update`; the graph is current after `app.update()` runs once. Tests explicitly call `app.update()` between patch-load and graph-query. Part 2 introduces the `GraphMaintenance` schedule and the synchronous-run mechanism.
- **(B) Run the freshness contract in part 1.** Add `GraphMaintenance` schedule now; update the umbrella's mutation APIs to call `world.run_schedule(GraphMaintenance)` after the mutation. More invasive, but part 1 is feature-complete with respect to freshness.

**Recommendation: (A).** Part 1 ships the foundation that's testable in isolation; part 2 layers the freshness contract on top with proper schedule plumbing. Document this in the completion report.

### 7. Plugin registration in umbrella

Edit `crates/oxurack/src/lib.rs` to install `RackGraphPlugin` when the `graph` feature is enabled. The exact integration point depends on the current shape of `Oxurack::new()` (verify in recon).

### 8. Crate-root re-exports

In `crates/oxurack-graph/src/lib.rs`:

```rust
pub mod graph;
pub mod plugin;
pub mod types;

pub use graph::{RackGraph, RackNode, RackEdge};
pub use plugin::RackGraphPlugin;
pub use types::{CableInfo, ModuleInfo, PortInfo, PortDirection};
```

(Module organization is illustrative — single file vs. split is CC's call based on file size and conceptual cohesion.)

---

## Tests

Part 1's test surface is intentionally minimal. Focus is on substrate correctness, not query coverage. Part 2 adds the dense query tests.

**Unit tests in `crates/oxurack-graph/src/lib.rs` test module:**

1. `test_rack_graph_empty_on_init` — `RackGraph::default()` has zero modules and zero cables.
2. `test_module_added_message_adds_node` — emit a `ModuleAdded` message into a world that has `RackGraphPlugin` installed; run the schedule; assert `RackGraph::module_count() == 1`, `RackGraph::contains_module(entity)` is true, `RackGraph::entity_by_instance_name("vco_1") == Some(entity)`.
3. `test_two_module_added_messages_add_two_nodes` — emit two `ModuleAdded` with different entities; assert `module_count == 2`, both `contains_module` checks pass.
4. `test_cable_connected_message_adds_edge` — set up two modules with port entities (use `spawn_module_entity` or manual `World::spawn` + `ChildOf` component); emit `ModuleAdded` for both; emit `CableConnected` with port entities; assert `RackGraph::cable_count() == 1`, `contains_cable(cable_entity)` is true.
5. `test_module_removed_message_removes_node` — declare-only test that emits `ModuleRemoved`; in part 1 the active emission site doesn't exist, but we can directly emit via `MessageWriter` in the test. Assert node removal.
6. `test_cable_disconnected_message_removes_edge` — same pattern for cables.
7. `test_serialization_module_info_roundtrip` — construct a `ModuleInfo`, serialize via serde (use `serde_json` from dev-deps if not already pulled in, or a simpler hand-rolled assertion), deserialize, assert structural equality.
8. `test_serialization_cable_info_roundtrip` — same for `CableInfo`.
9. `test_serialization_port_info_roundtrip` — same for `PortInfo`.

**Integration test in `crates/oxurack-graph/tests/patch_load_populates_graph.rs`:**

10. `test_patch_load_populates_graph` — set up a full `App` with `CorePlugin` + `RackGraphPlugin`; apply a valid patch via `apply_patch_to_world`; run `app.update()` once; assert `RackGraph` reflects every module and cable in the patch. Cross-check: `module_count` matches `patch.modules.len()`, `cable_count` matches `patch.cables.len()`.

**Test matrix on the umbrella crate:**

11. Verify `cargo build --no-default-features` (without `graph`) succeeds and `cargo build` (with `graph`) also succeeds. Both should be green; the umbrella crate should compile in both modes. Add a test in `crates/oxurack/tests/feature_gate.rs` or document the verification in the completion report (the actual test matrix is run via `make check` against both feature configurations — verify the Makefile / CI runs both, or add it).

**Test discipline carries forward from buckets 3/3.25/3.5:** `matches!` + field checks for assertions; no `.to_string().contains(...)`; positive paths assert observable state.

---

## Out of scope for part 1

Everything not listed above. Specifically:

- The 18 navigation queries (upstream, downstream, dangling, cycles, topological order, reachability, port compatibility, etc.). Part 2.
- `ModuleInfo` / `CableInfo` / `PortInfo` *builders* — part 1 declares the types but doesn't populate them. Part 2 writes the builders that read from ECS and produce these structs.
- The `GraphMaintenance` schedule and synchronous-freshness mechanism. Part 2.
- Label-resolving in result types (calling `display_name_of`, looking up `Description` and `Tags` components). Part 2.
- Tests that exercise the 18 queries. Part 2.
- Subscribing to `LabelChanged` / `ParameterChanged` / `PatchLoaded` — the graph doesn't need them per the design memo.
- Adding `CableKind` (Signal vs Modulation) — deferred per D-25.7.
- Closing any `NotImplemented` arms in `dispatch_core_command`.

---

## Suggested steps

1. **Recon.** Verify `Oxurack::new()` shape; verify whether port entities have `ChildOf` parent links; check the petgraph version in any existing workspace usage; confirm the test test_app pattern; locate any place that needs the umbrella's `[features]` block updated.
2. **Crate scaffolding.** `cargo new --lib crates/oxurack-graph`, fill in Cargo.toml, write minimal `lib.rs`. Verify it compiles with `cargo check -p oxurack-graph`.
3. **Type declarations.** `RackGraph` resource, `RackNode`, `RackEdge`, basic membership query methods. Verify compiles.
4. **Result type declarations.** `ModuleInfo`, `CableInfo`, `PortInfo`, `PortDirection`. Serde derives. Verify with serialize-roundtrip unit tests (tests #7–#9).
5. **Maintenance system handlers.** `handle_module_added`, `handle_module_removed`, `handle_cable_connected`, `handle_cable_disconnected`. Verify with unit tests #2–#6.
6. **Plugin.** `RackGraphPlugin` registering systems and the resource.
7. **Umbrella crate wiring.** `[features]` block, optional dependency, conditional plugin install.
8. **Integration test.** `tests/patch_load_populates_graph.rs` — verifies the substrate works end-to-end.
9. **Test matrix.** Build both `--no-default-features` and default; both green.
10. **Run `make format`, `make lint`, `make test`** (or whatever the project's full-check incantation is — verify in recon). No `#[ignore]`.

---

## DoD (every criterion is a required pass condition)

1. `cargo clean && make check` passes from a fresh state in the default configuration (with `graph` feature).
2. `cargo build --no-default-features -p oxurack` succeeds. The umbrella crate compiles without the graph feature.
3. New crate at `crates/oxurack-graph/` exists with proper Cargo.toml (license, edition 2024, dependencies per §1).
4. `[features]` block in `crates/oxurack/Cargo.toml` with `default = ["graph"]` and `graph = ["dep:oxurack-graph"]`. `oxurack-graph` listed as `optional = true` in dependencies.
5. `RackGraph` resource defined with `StableDiGraph<RackNode, RackEdge>` storage plus `node_by_entity` / `edge_by_entity` / `by_instance_name` lookup maps.
6. `RackNode` and `RackEdge` types defined per §3.
7. Membership-query methods (`module_count`, `cable_count`, `contains_module`, `contains_cable`, `entity_by_instance_name`) implemented and tested.
8. `ModuleInfo`, `CableInfo`, `PortInfo`, `PortDirection` declared with `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]` per §4. Serialize round-trip tests pass.
9. `RackGraphPlugin` registers the `RackGraph` resource and the four maintenance systems.
10. `handle_module_added` adds nodes when `ModuleAdded` fires.
11. `handle_module_removed` removes nodes when `ModuleRemoved` fires (even though no production emission site exists yet; test emits directly via `MessageWriter`).
12. `handle_cable_connected` adds edges when `CableConnected` fires. The chosen approach (Option A or B from §5) is documented in the completion report.
13. `handle_cable_disconnected` removes edges when `CableDisconnected` fires (same direct-emission test pattern as #11).
14. Tests #1–#9 (unit) and #10 (integration) all pass.
15. Conditional plugin install in `Oxurack::new()` (or wherever the equivalent integration point is). Verified by building both feature configurations.
16. Crate-root re-exports include `RackGraph`, `RackNode`, `RackEdge`, `RackGraphPlugin`, `ModuleInfo`, `CableInfo`, `PortInfo`, `PortDirection`.
17. No `.to_string().contains(...)` patterns in any test.
18. Four `NotImplemented` arms in `dispatch_core_command` unchanged (`LoadPatch`, `SavePatch`, `AddCable`, `RemoveCable`).
19. Doc-comments on every public type and method, following the existing pattern in `oxurack-core`. Specifically: `RackGraph` explains the materialized-view semantics; `RackNode` / `RackEdge` doc-comment notes that labels are fetched fresh from ECS at query time; result types document the Entity serialization choice; `RackGraphPlugin` documents the systems it installs and which events it subscribes to.
20. The bucket-4-part-1 commit lands as a single atomic commit with a clear message (e.g., `Bucket 4 (part 1): oxurack-graph crate foundation`).

---

## Completion report format

```
Bucket 4 (part 1) complete at <commit sha>.

Recon findings:
- petgraph version available / chosen: <version>
- Port entities have ChildOf parent component: <yes / no / partial>
- Oxurack::new() integration point: <file:line>
- Existing schedule labels for plugin systems: <list>
- <any other recon-revealed shape difference from this doc's claims>

Implementation choices:
- handle_cable_connected approach: <A (Query<&ChildOf>) / B (deferred resolution)> — <justification>
- Schedule used for maintenance systems: <Update / custom>
- Entity serialization field name in *Info types: <entity_index / entity_bits / other> — <rationale>

New types added in oxurack-graph (N):
- <list>

Tests added (N):
- <names + one-line summaries>

Build verification:
- cargo build (default features): <passed / failures>
- cargo build --no-default-features -p oxurack: <passed / failures>
- cargo clean && make check: <passed / failures>

Verification: 4 NotImplemented arms unchanged — <yes / no>
Verification: serde roundtrip for all three *Info types — <yes / no>
Verification: feature-gate test matrix exercised both modes — <yes / no>

Cross-bucket dependencies surfaced for part 2:
- <list, or "none">

Design memo follow-up edits required:
- <list, or "none">

Deferred items registry updates required:
- <which D-XX.x entries should be updated; none expected since part 1 doesn't promote any>

Intentional exceptions or deferrals:
- <list, or "none">

Spec-adherence walk:
- DoD #1: met / not met
- DoD #2: met / not met
... (all 20)

Nothing silently skipped.
```

---

## Anti-softpedal reminder

Watch-items particular to this bucket:

- **Plugin install gets claimed but isn't actually wired.** Same pattern that bit us in 3.5 part 1 (commit message claimed `LabelChanged` was registered but the registration code wasn't there). Verify the `Oxurack::new()` code path actually installs `RackGraphPlugin` when the feature is enabled — not just that the docs say it should.
- **`handle_cable_connected` silently no-ops.** If the port → module resolution fails (port doesn't have `ChildOf`, or the parent isn't a registered module node), the system can early-return without inserting an edge. Test #4 must observe the cable is in the graph; failure must be loud (panic in test, log in production), not silent skip.
- **`StableDiGraph::remove_node` invalidates outgoing/incoming edges.** Per petgraph docs, removing a node also removes its incident edges. The `edge_by_entity` map will have stale entries after `handle_module_removed` if we don't also remove those edges from the map. Cleanest: enumerate the node's incident edges *before* removing the node, prune `edge_by_entity` entries, then remove the node. Verify in test — emit `ModuleAdded` + `CableConnected`, then `ModuleRemoved` (without `CableDisconnected`), assert `edge_by_entity` no longer contains the cable.
- **Feature-gate testing skipped.** If `cargo build --no-default-features` isn't run, a regression that breaks the umbrella crate's gracefully-degraded path won't surface until much later. Add it to the explicit verification list in the completion report.
- **Serde Entity serialization picks an unstable wire format.** `Entity::to_bits() -> u64` is the convention; using `Entity::index()` alone loses the generation counter; using `Debug` format is fragile across Bevy versions. Pick `to_bits()` and document.
- **`module_count` / `cable_count` drift from the actual node/edge count.** If the lookup maps are updated but the petgraph storage isn't (or vice versa), counts drift silently. Cleanest invariant: every update writes both the petgraph storage AND the lookup map, or neither. Tests #2–#6 must exercise both paths.

If recon reveals that `Oxurack::new()` has shape different from this doc's assumptions, **stop and report**. If the port → module resolution can't be done via `ChildOf` (port spawning doesn't insert it), the spec's Option A breaks; either Option B or a new approach needs evaluation in scope — report rather than work around.

---

## After you report

I'll do trust-but-verify: confirm the plugin install actually lands in `Oxurack::new()`; verify feature-gate test matrix exercises both modes (build a no-default-features compilation); check that `module_count` and `cable_count` agree with petgraph's own counts after a series of add-remove operations; verify the chosen port → module resolution approach is internally consistent; check that the `LookupMaps invariant` (lookup maps and petgraph storage stay in sync) is held under module-removal-with-incident-edges. Then we sign off or loop back; part 2 spec follows.


## Version History

- 2026-09-17 — Planning migration: renamed `0005-bucket-4-part-1-rackgraph-foundation.md` (formerly `docs/dev/0005-bucket-4-part-1-rackgraph-foundation.md`) to `slice-plan.md` in `arc04-rackgraph/slice01-foundation`. Original content retained as historical planning; this rename does not establish current implementation or acceptance status. See the project plan and migration report for the current baseline.
