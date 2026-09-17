# Bucket 4 (part 2): Essential navigation surface

## Context

Bucket 4 part 1 (commit `9a0dda4`) shipped the `oxurack-graph` crate foundation: `RackGraph` resource, `RackNode` / `RackEdge` types, reactive maintenance systems consuming `ModuleAdded` / `ModuleRemoved` / `CableConnected` / `CableDisconnected`, plus the `RackGraphPlugin` and feature-gate wiring. Part 2 layers the **essential navigation surface** on top: result-type builders (resolving labels via ECS reads), the rosters / lookup / ports / neighborhoods queries the REPL and TUI need first, the freshness contract via a dedicated `GraphMaintenance` schedule, and the `rack.graph()` accessor on `Oxurack`.

**Part 3 (separate spec, drafted after part 2 ships) layers the advanced/diagnostic surface** on top: dangling-module / dangling-port detection, topological order, cycle detection, reachability traversal (downstream/upstream closure), port-compatibility filtering. The split keeps both specs at a manageable size and produces clean commit boundaries: part 2 = "you can find your way around the rack"; part 3 = "you can analyze the rack's structure for problems."

The architectural foundation is locked in **`workbench/rackgraph-design.md`** (refreshed 2026-05-10). Most of the 18-query catalog from the memo's §Scope of v0 lands in this bucket. The remainder lands in part 3.

---

## Required reading before recon

- **`workbench/rackgraph-design.md`** — design memo, especially §Query surface sketch (the `impl RackGraph` block listing the 18 methods) and §Freshness.
- **`workbench/cc-bucket-04-part-1-rackgraph-foundation.md`** — part 1's spec, particularly §3 (which types part 2 builds on) and §4 (the result-type stubs that need builders).
- **`workbench/literate-patching-design.md`** — bucket 3.5 design memo. Defines `DisplayName` / `Description` / `Tags` components and the `display_name_of` fallback helper that result builders use.
- **`workbench/deferred-tracking.md`** — registry. Part 2 doesn't promote any deferred items.
- **`assets/ai/ai-rust/skills/claude/SKILL.md`** — Rust skill, particularly `04-ownership-borrowing.md` (the `GraphView` borrow design) and `15-cargo/02-cargo-build-system.md` (feature-gate hygiene).

---

## Recon findings (as of part-1 close)

**HEAD is `9a0dda4`** ("Bucket 4 (part 1): oxurack-graph crate foundation"). Re-verify if newer commits land.

### What part 1 produced

In `crates/oxurack-graph/src/lib.rs`:

- **`RackGraph` resource** with `StableDiGraph<RackNode, RackEdge>`, `node_by_entity: HashMap<Entity, NodeIndex>`, `edge_by_entity: HashMap<Entity, EdgeIndex>`, `by_instance_name: HashMap<String, Entity>`. All `pub(crate)` fields per part 1.
- **`RackNode { entity, instance_name, kind }`**.
- **`RackEdge { cable_entity, source_port, target_port, transform }`**.
- **Membership queries:** `module_count`, `cable_count`, `contains_module`, `contains_cable`, `entity_by_instance_name`.
- **Maintenance systems** in `Update` schedule via `RackGraphPlugin`.
- **Result type stubs:** `ModuleInfo`, `CableInfo`, `PortInfo`, `PortDirection` — declared with serde derives but **no builder functions exist yet**. Part 2 writes them.

In `crates/oxurack/src/lib.rs`:

- `#[cfg(feature = "graph")] app.add_plugins(oxurack_graph::RackGraphPlugin);` at line 65-66.
- **No `rack.graph()` accessor method** — part 2 adds it.

### What part 2 needs to access in ECS

For result-type builders to resolve labels:

- **`oxurack_core::DisplayName`** component on a module entity → instance-level display name.
- **`oxurack_core::Description`** component → optional instance-level description.
- **`oxurack_core::Tags`** component (wraps `BTreeSet<Tag>`) → tags.
- **`oxurack_core::DisplayNameFormatter`** resource → fallback formatter for display names.
- **`oxurack_core::display_name_of(formatter, explicit, instance_name)`** helper → resolves the fallback chain.

For port queries:

- **`oxurack_core::Port`** component on child entities of modules. Fields: `name: PortName`, `direction: PortDirection`, `value_kind: ValueKind`, `merge_policy: MergePolicy`.
- **`bevy_ecs::hierarchy::Children`** or `ChildOf` to traverse from module entity to its port children.

For cable lookup:

- **`oxurack_core::Cable`** component on cable entities. Carries `source_port`, `target_port`, `transform`.

### Port direction conversion

`oxurack_core::PortDirection` (input/output enum, `#[non_exhaustive]`, no serde derives) is distinct from the part-1-declared `oxurack_graph::PortDirection` (serde-enabled). Builders must convert between them. Trivial match.

### `oxurack_core::ModuleKind` is convertible to/from `String`

Confirmed in earlier buckets. `kind: String` field in `ModuleInfo` is populated via `module_kind.to_string()` or equivalent. Verify the exact conversion idiom against the existing codebase.

---

## Part-2 decisions

### Decision 1 — `GraphView` vs direct query methods on `RackGraph`

Part 1's queries (`module_count`, etc.) are on `&self` of `RackGraph`. They don't need world access; the data lives in the graph's own maps.

Part 2's result-builder queries need to read multiple ECS resources and components — `RackGraph` itself, `DisplayNameFormatter`, `DisplayName` / `Description` / `Tags` components per module entity, `Port` components per port child entity, `Cable` components per cable entity. Querying these requires `&World` access. Two API shapes:

**(A) `GraphView<'w>` wrapper.** A thin type holding `&'w World`, with all rich-query methods on it. `rack.graph()` returns `GraphView<'_>`. Membership queries (from part 1) are also exposed via `GraphView::raw() -> &RackGraph` or duplicated on `GraphView`.

**(B) Query methods take `&World` parameter directly.** `graph.modules(&world)`, `graph.module_by_name(&world, "vco_1")`, etc. More verbose at call sites; no new type.

**Recommendation: (A).** The design memo posits `rack.graph().modules()` as the call shape; that requires `graph()` to return *something* on which `.modules()` exists without needing an explicit world parameter. A `GraphView` wrapper is the natural shape, and Bevy's borrow checker handles the `&'w World` lifetime cleanly.

Implementation sketch:

```rust
pub struct GraphView<'w> {
    world: &'w World,
}

impl<'w> GraphView<'w> {
    pub fn new(world: &'w World) -> Self {
        Self { world }
    }

    pub fn raw(&self) -> &RackGraph {
        self.world.resource::<RackGraph>()
    }

    pub fn modules(&self) -> Vec<ModuleInfo> { /* … */ }
    pub fn cables(&self) -> Vec<CableInfo> { /* … */ }
    // … etc.
}
```

The membership queries from part 1 stay on `RackGraph` (`&self`); callers reach them via `view.raw().module_count()` or via convenience pass-throughs on `GraphView`. CC decides whether to add pass-throughs or rely on `raw()`.

### Decision 2 — `GraphMaintenance` schedule label vs system ordering

Part 1's systems run in `Update`. Part 2's freshness contract says: after every `Oxurack` mutation method returns, the graph must reflect the mutation. Today, the maintenance systems run only at `app.update()` time.

Three concrete approaches:

**(A) Dedicated `GraphMaintenance` schedule.** Move the four maintenance systems out of `Update` into a custom schedule label. `Oxurack::load_patch` etc. call `world.run_schedule(GraphMaintenance)` after the mutation. `app.update()` still runs the schedule normally — either via `app.add_systems(Update, ...)` calling it, or by adding `GraphMaintenance` as a startup-and-update schedule.

**(B) Keep systems in `Update`, call `app.update()` after mutation.** Simpler — no new schedule. But `app.update()` runs *all* systems, not just maintenance. Could trigger unintended side effects (timers advance, audio threads tick, etc.).

**(C) Manually execute the four maintenance systems synchronously.** No new schedule, no `app.update()` — direct system invocation. Bypasses Bevy's scheduling machinery. Brittle if system signatures change.

**Recommendation: (A).** Cleanest semantics, no side-effect concerns, Bevy-idiomatic. The implementation has CC pick a schedule label (probably `GraphMaintenance` per the memo) and configure it to run both in the `Update` chain and via explicit synchronous calls.

CC: validate the Bevy 0.18 idiom for "schedule runs in Update *and* is callable directly." A `ScheduleLabel` works for both; the question is the exact wiring in `RackGraphPlugin::build`. Recon if uncertain.

---

## Locked decisions (recap)

From design memo + part 1:

| # | Decision |
|---|----------|
| L1 | Part 2 builds the essential navigation surface; part 3 adds advanced/diagnostic queries. |
| L2 | Result-type builders read fresh from ECS at query time. Labels are not cached in `RackGraph`. |
| L3 | `GraphView<'w>` is the rich-query surface; membership queries from part 1 stay on `RackGraph` (`&self`). |
| L4 | `rack.graph()` on `Oxurack` returns `GraphView<'_>` over the umbrella's `World`. Feature-gated; absent when `--no-default-features`. |
| L5 | Freshness contract via dedicated `GraphMaintenance` schedule (Decision 2A). |
| L6 | Edges are undifferentiated for v0 (CableKind deferred per D-25.7). Queries that would otherwise filter by Signal vs. Modulation treat all edges uniformly. |
| L7 | Test discipline carries forward: `matches!` + field checks; no `.to_string().contains(...)`; positive paths assert observable state. |
| L8 | Anti-softpedal patterns: claim-without-code, silent failure paths, drift between in-memory counts and lookup maps. |

---

## Work mode

**Thinking work stays in your context** with the Rust skill loaded. This includes: deciding the `GraphView` lifetime and borrow story; the exact `ports_of(entity)` implementation (need to query children of the module entity); whether to use `world.query_filtered::<...>()` or pre-resolved lookups; how to handle the case where a cable references a port that isn't a registered module port (shouldn't happen, but defensive handling); the freshness contract's interaction with `apply_patch_to_world`'s `resource_scope::<ModuleRegistry, _>` pattern (the integration test from part 1 wraps the patch-apply in resource_scope — `world.run_schedule(GraphMaintenance)` outside the scope is the natural placement).

**Lookup work may delegate to subagents.** This includes: greping for existing `ScheduleLabel` definitions in `oxurack-core` to model `GraphMaintenance` after them; finding the Bevy 0.18 API for "run a schedule in Update *and* synchronously" (`add_systems` vs `add_schedule` vs both); finding `Oxurack::load_patch` signature for the freshness-call wiring; verifying `display_name_of` is correctly exported.

Serial execution on thinking. Quality over speed.

---

## The task

### 1. Result-type builders

Three private builder functions in `oxurack-graph` (probably in a new `builders.rs` module, but a single-file crate is also fine):

```rust
fn build_module_info(world: &World, node: &RackNode) -> ModuleInfo {
    let formatter = world.resource::<DisplayNameFormatter>();
    let entity = node.entity;
    let explicit = world.get::<DisplayName>(entity);
    let display_name = display_name_of(formatter, explicit, &node.instance_name);

    let description = world.get::<Description>(entity).map(|d| d.0.clone());
    let tags = world
        .get::<Tags>(entity)
        .map(|t| t.0.iter().map(|tag| tag.as_str().to_string()).collect())
        .unwrap_or_default();

    ModuleInfo {
        entity_bits: entity.to_bits(),
        instance_name: node.instance_name.clone(),
        kind: node.kind.to_string(),
        display_name,
        description,
        tags,
    }
}

fn build_cable_info(world: &World, edge: &RackEdge) -> CableInfo {
    // Resolve source_port -> (source_module_entity, port_name)
    // Resolve target_port -> (target_module_entity, port_name)
    // Resolve module entities -> instance names via RackGraph or
    // the world's PatchHandle.
    // Pull DisplayName/Description/Tags from the cable_entity if present.
    /* … */
}

fn build_port_info(port: &Port) -> PortInfo {
    PortInfo {
        name: port.name.as_ref().to_string(),
        direction: convert_direction(port.direction),
        value_kind: format!("{:?}", port.value_kind), // or a proper Display
    }
}

fn convert_direction(d: oxurack_core::PortDirection) -> PortDirection {
    match d {
        oxurack_core::PortDirection::Input => PortDirection::Input,
        oxurack_core::PortDirection::Output => PortDirection::Output,
    }
}
```

**`build_cable_info` details:** the cable's `source_port` and `target_port` are port entities, not module entities. To produce `source_module_instance` / `source_port_name` / `target_module_instance` / `target_port_name` strings for `CableInfo`:

1. Walk the port entity's `ChildOf` parent to get the module entity.
2. Look up the module entity in `RackGraph::by_instance_name` (reverse-walk via `node_by_entity`) to get the instance name.
3. Read the `Port` component on the port entity for the port name.

Cleanest: a small private helper `resolve_port_to_module(world, port_entity) -> (instance_name, port_name)` that does steps 1–3. Use it twice in `build_cable_info`.

### 2. `GraphView<'w>` wrapper and rich queries

```rust
pub struct GraphView<'w> {
    world: &'w World,
}

impl<'w> GraphView<'w> {
    pub fn new(world: &'w World) -> Self {
        Self { world }
    }

    /// Borrow the raw graph for part-1 membership queries.
    pub fn raw(&self) -> &RackGraph {
        self.world.resource::<RackGraph>()
    }

    // ── Rosters ────────────────────────────────────────────────────

    /// All modules in the graph, with labels resolved.
    pub fn modules(&self) -> Vec<ModuleInfo> {
        let graph = self.raw();
        graph
            .inner
            .node_weights()
            .map(|node| build_module_info(self.world, node))
            .collect()
    }

    /// All cables in the graph, with labels resolved.
    pub fn cables(&self) -> Vec<CableInfo> {
        let graph = self.raw();
        graph
            .inner
            .edge_weights()
            .map(|edge| build_cable_info(self.world, edge))
            .collect()
    }

    /// Histogram of module kinds.
    pub fn module_kinds(&self) -> HashMap<ModuleKind, usize> { /* … */ }

    /// All modules of a given kind.
    pub fn modules_of_kind(&self, kind: &ModuleKind) -> Vec<ModuleInfo> { /* … */ }

    // ── Lookup ─────────────────────────────────────────────────────

    /// Look up a module by its instance name.
    pub fn module_by_name(&self, name: &str) -> Option<ModuleInfo> {
        let graph = self.raw();
        let entity = *graph.by_instance_name.get(name)?;
        let idx = *graph.node_by_entity.get(&entity)?;
        let node = graph.inner.node_weight(idx)?;
        Some(build_module_info(self.world, node))
    }

    // ── Ports ──────────────────────────────────────────────────────

    /// All ports on a given module, split into inputs and outputs.
    pub fn ports_of(&self, module_entity: Entity) -> PortSet { /* … */ }

    // ── Neighborhoods ──────────────────────────────────────────────

    /// Modules upstream of the given module (have a cable feeding it).
    pub fn upstream_of(&self, module_entity: Entity) -> Vec<ModuleInfo> { /* … */ }

    /// Modules downstream of the given module (receive a cable from it).
    pub fn downstream_of(&self, module_entity: Entity) -> Vec<ModuleInfo> { /* … */ }

    /// Cables coming into the given module.
    pub fn incoming_cables(&self, module_entity: Entity) -> Vec<CableInfo> { /* … */ }

    /// Cables going out from the given module.
    pub fn outgoing_cables(&self, module_entity: Entity) -> Vec<CableInfo> { /* … */ }
}
```

### 3. `PortSet` type

Helper return type for `ports_of`:

```rust
/// All ports on a single module, separated by direction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PortSet {
    pub inputs: Vec<PortInfo>,
    pub outputs: Vec<PortInfo>,
}
```

### 4. `GraphMaintenance` schedule

```rust
use bevy_ecs::schedule::ScheduleLabel;

/// Schedule label for the four graph-maintenance systems.
///
/// Runs as part of `Update` for normal app ticks, and can be
/// invoked synchronously via `world.run_schedule(GraphMaintenance)`
/// to honor the freshness contract from `Oxurack` mutation methods.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphMaintenance;
```

In `RackGraphPlugin::build`, configure the schedule:

```rust
impl Plugin for RackGraphPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RackGraph>()
            // Maintenance systems live in the GraphMaintenance schedule.
            .add_systems(
                GraphMaintenance,
                (
                    handle_module_added,
                    handle_module_removed,
                    handle_cable_connected,
                    handle_cable_disconnected,
                )
                    .chain(),
            )
            // GraphMaintenance runs as part of every Update tick.
            .add_systems(Update, run_graph_maintenance_in_update);
    }
}

fn run_graph_maintenance_in_update(world: &mut World) {
    world.run_schedule(GraphMaintenance);
}
```

**Validate this idiom against Bevy 0.18** — there may be a cleaner way to make a schedule run-as-part-of-another-schedule. The wrapper-system approach above works but may not be the most idiomatic. Recon and adjust.

### 5. `rack.graph()` accessor on `Oxurack`

In `crates/oxurack/src/lib.rs`:

```rust
impl Oxurack {
    /// Access the rack's graph view for navigation queries.
    ///
    /// Available only with the `graph` feature.
    #[cfg(feature = "graph")]
    pub fn graph(&self) -> oxurack_graph::GraphView<'_> {
        oxurack_graph::GraphView::new(self.app.world())
    }
}
```

### 6. Freshness contract — update `Oxurack` mutation methods

For every `Oxurack` method that triggers a structural mutation, run `GraphMaintenance` synchronously before returning. Concretely:

- **`Oxurack::load_patch`** — after `apply_patch_to_world` returns successfully, call `self.app.world_mut().run_schedule(GraphMaintenance)`. Only when the graph feature is enabled.
- Future mutation APIs (`install_module`, `add_cable`, etc.) get the same treatment when they land. Not in scope for part 2.

The post-mutation call must happen *outside* the `resource_scope::<ModuleRegistry, _>` block (the resource is taken out of the world during the scope; `run_schedule` needs the full world).

Cleanest pattern:

```rust
#[cfg(feature = "graph")]
pub fn load_patch(&mut self, path: &Path) -> Result<PatchHandle, CoreError> {
    let patch = oxurack_core::load_patch_from_file(path)?;
    let handle = self.app.world_mut().resource_scope::<ModuleRegistry, _>(|world, registry| {
        oxurack_core::apply_patch_to_world(&patch, &registry, world, oxurack_core::PatchSource::File(path.to_path_buf()))
    })?;
    self.app.world_mut().run_schedule(oxurack_graph::GraphMaintenance);
    Ok(handle)
}

#[cfg(not(feature = "graph"))]
pub fn load_patch(&mut self, path: &Path) -> Result<PatchHandle, CoreError> { /* unchanged */ }
```

(Verify the actual `load_patch` signature in the current `Oxurack` — adjust as needed.)

### 7. Exports

In `crates/oxurack-graph/src/lib.rs` (or `mod.rs`), update the re-exports:

```rust
pub use graph::{RackGraph, RackNode, RackEdge};
pub use view::{GraphView, PortSet};     // new in part 2
pub use plugin::{RackGraphPlugin, GraphMaintenance};  // GraphMaintenance new
pub use types::{CableInfo, ModuleInfo, PortInfo, PortDirection};
```

(Module organization is illustrative; pick what makes sense given the file size.)

---

## Tests

Test discipline carries forward from buckets 3 / 3.25 / 3.5 / 4-part-1: `matches!` + field checks; no `.to_string().contains(...)`; positive paths assert observable state.

### Result-builder tests (unit, in `oxurack-graph::tests`)

1. `test_module_info_resolves_display_name_from_component` — module with `DisplayName("Lead VCO")` component; `build_module_info` returns `display_name: "Lead VCO"`.
2. `test_module_info_falls_back_to_formatter_when_no_component` — module with no `DisplayName` component, instance_name `"vco_1"`; formatter has VCO acronym; result has `display_name: "VCO 1"`.
3. `test_module_info_resolves_description` — module with `Description("the lead voice")` → `description: Some("the lead voice")`. No component → `description: None`.
4. `test_module_info_resolves_tags` — module with `Tags(BTreeSet)` → `tags` contains all entries as strings, sorted.
5. `test_module_info_no_tags_when_component_absent` — module with no `Tags` component → `tags: vec![]`.
6. `test_cable_info_resolves_ports_and_modules` — set up two modules with cabled ports; `build_cable_info` returns correct `source_module_instance` / `source_port_name` / `target_module_instance` / `target_port_name`.

### Query tests (integration-ish, in unit-test module with `App` setup)

7. `test_modules_returns_all` — patch with 3 modules; `view.modules().len() == 3`; instance names match.
8. `test_modules_empty_for_empty_graph` — fresh app, no patches loaded; `view.modules().is_empty()`.
9. `test_cables_returns_all` — patch with 2 cables; `view.cables().len() == 2`.
10. `test_module_kinds_histogram` — patch with 2 VCOs and 1 filter; `module_kinds()` returns `{vco: 2, filter: 1}` (or whatever the test kinds are).
11. `test_modules_of_kind_filters` — same patch; `modules_of_kind(&ModuleKind::from("vco")).len() == 2`.
12. `test_module_by_name_resolves` — patch with `vco_1`; `module_by_name("vco_1")` returns `Some(ModuleInfo { instance_name: "vco_1", ... })`.
13. `test_module_by_name_returns_none_for_missing` — `module_by_name("nonexistent")` returns `None`.

### Ports tests

14. `test_ports_of_returns_inputs_and_outputs` — module with one input port and one output port; `ports_of(entity).inputs.len() == 1`, `ports_of(entity).outputs.len() == 1`.
15. `test_ports_of_returns_empty_for_unknown_module` — entity not in graph; `ports_of(entity)` returns `PortSet { inputs: vec![], outputs: vec![] }`. (Defensive — don't panic.)

### Neighborhood tests

16. `test_upstream_of_returns_source_modules` — three-module chain `vco -> filter -> out`; `upstream_of(filter)` returns one module (vco); `upstream_of(out)` returns one module (filter); `upstream_of(vco)` is empty.
17. `test_downstream_of_returns_target_modules` — same chain; `downstream_of(vco)` returns one module (filter); `downstream_of(out)` is empty.
18. `test_incoming_cables` — module with 2 cables coming in; `incoming_cables(entity).len() == 2`.
19. `test_outgoing_cables` — module with 2 cables going out; `outgoing_cables(entity).len() == 2`.
20. `test_neighborhoods_empty_for_isolated_module` — module with no cables; both `upstream_of` and `downstream_of` return empty.

### Freshness contract test (integration)

21. `test_load_patch_makes_graph_current_synchronously` — `Oxurack::new()`, register module kinds, call `rack.load_patch(...)`, *immediately* (no explicit `app.update()`) call `rack.graph().modules()`, assert all modules present. This is the load-bearing test for the freshness contract.

### Accessor test

22. `test_graph_accessor_returns_view` — `Oxurack::new()`, register modules, load patch, call `rack.graph()`, verify it's a usable `GraphView` (e.g., `.modules().len() > 0`).

### Feature-gate verification

23. Build verification: `cargo build --no-default-features -p oxurack` still compiles (the `rack.graph()` method must be `#[cfg(feature = "graph")]` gated; absent under no-default-features).
24. Build verification: `cargo build` (default) compiles.

### Cross-type ordering (carried forward from earlier buckets)

25. `test_query_ordering_preserves_declaration_order` — patch with modules `vco_1`, `vco_2`, `filter_1`; `modules()` returns them in declaration order. (Note: this depends on petgraph's `node_weights()` iteration order. If unstable, sort explicitly in the builder and document.)

### Total: 25 tests across builder unit tests, query tests, freshness test, and accessor test.

If `module_kinds` / `modules_of_kind` use `String` rather than `ModuleKind` for the key (e.g., for serde portability of the histogram), document the choice; otherwise use `ModuleKind` directly.

---

## Out of scope for part 2

Everything not listed above. Specifically:

- **Dangling modules / dangling ports** — part 3.
- **Topological order** — part 3.
- **Cycle detection** — part 3.
- **Reachability** (`downstream_closure`, `upstream_closure`) — part 3.
- **Port compatibility filter** (`compatible_inputs_for`) — part 3.
- **Knob queries** — deferred until `bucket-getter` (D-25.1) lands the parameter-introspection mechanism.
- **Subgraph selection / extraction** — future.
- **Cable kinds (Signal vs Modulation)** — deferred per D-25.7.
- **Closing any `NotImplemented` arms** in `dispatch_core_command`.

---

## Suggested steps

1. **Recon.** Verify part-1 file shape; verify `display_name_of` export; verify `Oxurack::load_patch` signature and `resource_scope` pattern; verify Bevy 0.18 idiom for "schedule run as part of another schedule."
2. **`GraphMaintenance` schedule.** Define the label, move maintenance systems off `Update` and into `GraphMaintenance`, wire the in-Update execution.
3. **Result-type builders.** `build_module_info`, `build_cable_info`, `build_port_info`, `convert_direction`. Unit tests #1–#6 verify these in isolation (construct entities, attach components, build the info, assert fields).
4. **`GraphView` struct.** With `new`, `raw`, plus the rich-query methods listed in §2.
5. **`PortSet` type.** Declared alongside `GraphView`.
6. **Rosters.** `modules`, `cables`, `module_kinds`, `modules_of_kind`. Tests #7–#11.
7. **Lookup.** `module_by_name`. Tests #12–#13.
8. **Ports.** `ports_of`. Tests #14–#15.
9. **Neighborhoods.** `upstream_of`, `downstream_of`, `incoming_cables`, `outgoing_cables`. Tests #16–#20.
10. **`Oxurack::graph()` accessor.** Feature-gated. Test #22.
11. **Freshness contract.** Update `Oxurack::load_patch` to call `world.run_schedule(GraphMaintenance)` after the patch applies. Test #21 verifies.
12. **Build verification.** `cargo build` (default), `cargo build --no-default-features -p oxurack`. Both green.
13. **Run `make format`, `make lint`, `make test`.** All green; no `#[ignore]`; flaky `oxurack-rt` timing test (pre-existing, noted in part-1 review) remains the only known instability.

---

## DoD (every criterion is a required pass condition)

1. `cargo clean && make check` passes from a fresh state (default features).
2. `cargo build --no-default-features -p oxurack` succeeds.
3. `GraphMaintenance` schedule label defined and exported. Maintenance systems run in `GraphMaintenance`, not directly in `Update`.
4. `GraphMaintenance` runs as part of every `Update` tick (via wrapper system or equivalent Bevy 0.18 idiom).
5. `GraphView<'w>` struct exists with `new`, `raw`, and the 12 rich-query methods listed in §2 (`modules`, `cables`, `module_kinds`, `modules_of_kind`, `module_by_name`, `ports_of`, `upstream_of`, `downstream_of`, `incoming_cables`, `outgoing_cables`, plus `raw` and `new`).
6. `PortSet` type exists with `inputs: Vec<PortInfo>`, `outputs: Vec<PortInfo>`, serde derives.
7. Result-builder functions (`build_module_info`, `build_cable_info`, `build_port_info`, `convert_direction`) implemented per §1.
8. `ModuleInfo` builder resolves `display_name` via `display_name_of` fallback chain.
9. `ModuleInfo` builder resolves `description` from `Description` component (or `None` if absent).
10. `ModuleInfo` builder resolves `tags` from `Tags` component (or empty vec if absent).
11. `CableInfo` builder resolves port-to-module via `ChildOf` traversal and produces module-instance + port-name strings.
12. `Oxurack::graph()` accessor exists, feature-gated, returns `GraphView<'_>`.
13. `Oxurack::load_patch` calls `world.run_schedule(GraphMaintenance)` after the patch applies (outside the `resource_scope` block).
14. Tests #1–#22 (and #25 if applicable) pass.
15. Build verification tests #23 + #24 pass.
16. No `.to_string().contains(...)` patterns in any new test.
17. Four `NotImplemented` arms in `dispatch_core_command` unchanged.
18. Doc-comments on every public type and method (`GraphView`, `PortSet`, `GraphMaintenance`, each query method on `GraphView`, `Oxurack::graph`). Documentation explains: the query's contract; whether it's `O(1)` / `O(deg)` / `O(n)`; what kind of `Entity` argument is expected (module entity vs. port entity).
19. Crate-root re-exports updated to include `GraphView`, `PortSet`, `GraphMaintenance` (and confirm the part-1 exports are still present).
20. The bucket-4-part-2 commit lands as a single atomic commit (e.g., `Bucket 4 (part 2): essential navigation surface — GraphView, builders, freshness contract`).

---

## Completion report format

```
Bucket 4 (part 2) complete at <commit sha>.

Recon findings:
- Bevy 0.18 idiom for "schedule run as part of another schedule": <approach used + justification>
- Oxurack::load_patch signature: <as expected / drifted — describe>
- display_name_of export confirmed: <yes / no>
- <any other recon-revealed shape difference>

Implementation choices:
- Decision 1 (GraphView vs direct queries): <A — GraphView wrapper / B — direct queries>
- Decision 2 (GraphMaintenance schedule): <A — dedicated schedule / B — Update-only / C — manual>
- Convenience pass-throughs on GraphView for part-1 membership queries: <added / use .raw()>

New types added (N):
- <list>

Query methods added on GraphView (N):
- <list>

Freshness contract sites updated:
- Oxurack::load_patch: <updated / not applicable>
- <future sites listed but not updated>

Code files changed (N):
- <list>

Tests added (N):
- <list of names with one-line assertion summaries>

Verification: cargo clean && make check — <passed / failures>
Verification: cargo build --no-default-features -p oxurack — <passed / failures>
Verification: rack.graph().modules() returns expected after load_patch (freshness test #21) — <yes / no>
Verification: 4 NotImplemented arms unchanged — <yes / no>
Verification: display-name fallback chain (component → formatter) verified — <yes / no>

Cross-bucket dependencies surfaced for part 3:
- <list, or "none">

Design memo follow-up edits required:
- <list, or "none">

Deferred items registry updates required:
- <which D-XX.x entries; none expected since part 2 doesn't promote any>

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

- **`GraphView::modules()` returns empty when `RackGraph` resource isn't installed.** If the plugin install path drifts (regression to the 3.5 part 1 pattern), `world.resource::<RackGraph>()` will panic — which is loud, good. But the `Oxurack::graph()` accessor must be feature-gated; calling it under `--no-default-features` should be a compile error, not a runtime panic.
- **`build_cable_info`'s port-to-module resolution can silently produce wrong instance names.** If `ChildOf` traversal misfires (port's parent isn't a registered module), the function might return placeholder strings or panic. The behavior must be loud (panic or returning a clearly-invalid sentinel like `"<unresolved>"`); silent wrong-data is the worst outcome. Test #6 verifies the happy path; pair with a defensive log/panic for the failure path.
- **The freshness test (#21) silently passes by virtue of `update` running anyway.** If the test does `rack.load_patch()` then `rack.graph().modules()` — but in between, an `app.update()` got called somewhere (test harness, plugin init, etc.) — then the freshness contract isn't actually being tested; the modules show up because of `update`, not because of `run_schedule(GraphMaintenance)`. To make the test load-bearing: temporarily disable the in-Update wrapper (or write the test in a way that proves the synchronous path is what's making it green — e.g., immediately check `RackGraph.module_count()` raw before calling `app.update()`).
- **Cross-type ordering** (test #25) depends on petgraph's `node_weights()` iteration order. petgraph doesn't guarantee insertion order on `StableDiGraph::node_weights()`. If the test passes today but a petgraph minor-version update changes iteration order, the test breaks silently. Either: explicitly sort modules by `instance_name` (or `entity_bits`) in the builder; or: drop the ordering assertion and document. Don't rely on undocumented iteration order.
- **`GraphMaintenance` runs twice per Update tick.** If both `add_systems(Update, run_graph_maintenance_in_update)` AND the schedule is auto-run by Bevy somehow, the systems execute twice. Costless when the message queue is empty (no-op iterations), but worth verifying via a single-step test that observes message count being drained exactly once.
- **`ports_of(entity)` returns empty silently for invalid entities.** Distinguish "module has no ports" (rare but valid) from "entity isn't a module at all" (programmer error). Document the contract; consider returning `Option<PortSet>` if the distinction matters.

If recon reveals that `Oxurack::load_patch` has a different signature or wraps `apply_patch_to_world` differently than the spec assumes, **stop and report** rather than guessing.

---

## After you report

I'll do trust-but-verify: confirm `GraphView::modules()` returns labeled results (not placeholder strings or empty `display_name`); verify the freshness test actually proves the synchronous schedule run is what makes it pass (not incidental `update()` calls); check that `--no-default-features` build still compiles after the new `Oxurack::graph()` is added (must be `#[cfg]` gated); verify the in-Update wrapper for `GraphMaintenance` runs the schedule exactly once per tick; check that builder logic handles missing `DisplayName` / `Description` / `Tags` components correctly (fallback to formatter / `None` / empty vec respectively). Then we sign off or loop back; part 3 spec follows.


## Version History

- 2026-09-17 — Planning migration: renamed `0006-bucket-4-part-2-essential-navigation-surface.md` (formerly `docs/dev/0006-bucket-4-part-2-essential-navigation-surface.md`) to `slice-plan.md` in `arc04-rackgraph/slice02-navigation`. Original content retained as historical planning; this rename does not establish current implementation or acceptance status. See the project plan and migration report for the current baseline.

## Current restart disposition — v2.0, 2026-09-17

The preceding implementation instructions are preserved historical text, not a
current CC assignment. Proposed close: carried over, pending CRC verification.
See [closing-report.md](closing-report.md) for both row dispositions. Remaining
reconciliation is assigned to [arc08-validation-and-release/slice01-restart-baseline](../../arc08-validation-and-release/slice01-restart-baseline/slice-plan.md), R-02;
this is a transfer of unresolved work, not behavioral acceptance or reimplementation.
No historical prompt has been reissued. No iteration count is inferred or reset.

### Version History — restart amendment

- v2.0 — 2026-09-17: Operator-authorized carryover candidate; arc08 restart
  baseline receives reconciliation because migration supplied no current review.
