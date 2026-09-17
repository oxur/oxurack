# Bucket 4 (part 3): Diagnostic queries — closing the 18-query catalog

## Context

Bucket 4 parts 1 and 2 (commits `9a0dda4` + the uncommitted part-2 working tree) shipped the graph foundation and the essential navigation surface. Part 3 closes out the 18-query catalog from the design memo with the **diagnostic and analytic queries**: dangling modules, dangling ports, topological order, cycle detection, reachability traversal (downstream/upstream closure), and port-compatibility filtering. Plus one small follow-up from the part-2 review: sorting the `cables()` output for consistency with `modules()`.

After part 3, bucket 4 is feature-complete for v0. The only outstanding query from the original memo is `knobs_of(entity)`, which is genuinely blocked on bucket-getter (D-25.1: `ParameterRegistry` needs getter machinery before runtime knob values are introspectable).

Part 3 is smaller than part 2 because the substrate is in place. The queries are mostly thin wrappers over petgraph algorithms plus existing parts-of `GraphView`.

---

## Required reading before recon

- **`workbench/rackgraph-design.md`** — design memo, especially §Scope of v0 queries #11–#17 (the queries part 3 implements) and the reconciliation section at top.
- **`workbench/cc-bucket-04-part-1-rackgraph-foundation.md`** and **`workbench/cc-bucket-04-part-2-essential-navigation-surface.md`** — for what parts 1 and 2 produced.
- **`workbench/deferred-tracking.md`** — registry. Part 3 doesn't promote any deferred items; D-25.1 (knobs / bucket-getter) and D-25.7 (CableKind Signal/Modulation split) remain deferred.
- **`assets/ai/ai-rust/skills/claude/SKILL.md`** — Rust skill, particularly `03-error-handling.md` for `CycleError` design via `thiserror` (matches `TagError` from bucket 3.5).
- **petgraph 0.7 docs** — `petgraph::algo::toposort`, `petgraph::algo::tarjan_scc`, `petgraph::visit::Dfs`. All available without additional feature flags.

---

## Recon findings (verify against current tree)

**HEAD after part 2 commits:** whatever SHA part 2 lands at. Verify before starting.

### What parts 1 and 2 produced

In `crates/oxurack-graph/src/lib.rs`:

- `RackGraph` resource with `StableDiGraph<RackNode, RackEdge>` + lookup maps. Membership queries: `module_count`, `cable_count`, `contains_module`, `contains_cable`, `entity_by_instance_name`.
- `GraphView<'w>` wrapper with rich queries: `modules`, `cables`, `module_kinds`, `modules_of_kind`, `module_by_name`, `ports_of`, `upstream_of`, `downstream_of`, `incoming_cables`, `outgoing_cables`, `raw`, `new`.
- Result types: `ModuleInfo`, `CableInfo`, `PortInfo`, `PortDirection`, `PortSet` — all serde-enabled.
- `RackGraphPlugin` + `GraphMaintenance` schedule.
- Internal builders: `build_module_info`, `build_cable_info`, `build_port_info`, plus port-to-module resolver.

In `crates/oxurack/src/lib.rs`:

- `Oxurack::graph()` feature-gated accessor returning `GraphView<'_>`.
- `Oxurack::load_patch` calls `run_schedule(GraphMaintenance)` after the patch applies (freshness contract).

### petgraph 0.7 algorithms available

Confirmed in `crates/oxurack-graph/Cargo.toml`: petgraph 0.7 with `stable_graph` feature. All algorithms in `petgraph::algo` and visitors in `petgraph::visit` are available without additional feature flags. Specifically used by part 3:

- **`petgraph::algo::toposort(g, None) -> Result<Vec<NodeId>, Cycle<NodeId>>`** — topological order; returns `Err(Cycle)` when cycles exist.
- **`petgraph::algo::tarjan_scc(g) -> Vec<Vec<NodeId>>`** — all strongly-connected components.
- **`petgraph::visit::Dfs` + `petgraph::visit::Reversed`** — depth-first traversal; reversal for upstream-closure.

### Port enumeration for compatibility filter

The cleanest port-compatibility implementation walks the graph's modules (via `RackGraph::node_weights`), then for each module walks its `Children` to enumerate `Port` components. This reuses the existing `Children` → `Port` traversal pattern already in `ports_of` (`lib.rs:394+`). Avoids a global `world.query::<...>()` (which requires mutable world access).

### `ValueKind` equality

Verify in `crates/oxurack-core/src/value.rs` that `ValueKind` derives `PartialEq` so port-compatibility can compare directly without stringification. If it doesn't, either add the derive (small Bucket 4 follow-up to `oxurack-core`) or compare via `format!("{:?}", ...)` (less clean; document).

---

## Part-3 decisions

### Decision 1 — `CycleError` shape

When `topological_order` finds a cycle, what does it return?

**(A)** `CycleError::Cycles(Vec<Vec<String>>)` — instance names of nodes in each cycle. Simple, serializable, debuggable.
**(B)** `CycleError::Cycles(Vec<Vec<ModuleInfo>>)` — full module info for each cycle member. More expensive to build; rarely needed.
**(C)** `CycleError::HasCycles` — single sentinel; caller calls `cycles()` separately for detail. Two-step API.

**Recommendation: (A).** Instance names are the load-bearing identifier; full `ModuleInfo` adds cost for a path that's primarily diagnostic. If a consumer needs more detail, they call `cycles()` and resolve names to entities themselves.

```rust
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum CycleError {
    /// The graph contains one or more cycles. Each inner `Vec`
    /// lists the instance names of modules in one cycle (self-loops
    /// are single-element vectors).
    #[error("graph contains {} cycle(s)", .0.len())]
    Cycles(Vec<Vec<String>>),
}
```

### Decision 2 — `cycles()` semantics: SCCs and self-loops

petgraph's `tarjan_scc` returns *all* strongly-connected components, including singleton SCCs (a node connected to itself only via traversal). A genuine cycle is:

- **An SCC with `len() > 1`** — two or more mutually reachable nodes.
- **A singleton SCC where the node has a self-loop** — a node with an edge to itself.

**`cycles()` must return both.** A singleton SCC *without* a self-loop is not a cycle and must be filtered out.

```rust
pub fn cycles(&self) -> Vec<Vec<ModuleInfo>> {
    let graph = self.raw();
    let sccs = petgraph::algo::tarjan_scc(&graph.inner);
    sccs.into_iter()
        .filter(|component| {
            // Multi-node SCC is always a cycle.
            if component.len() > 1 { return true; }
            // Single-node SCC: cycle only if it has a self-loop.
            let node = component[0];
            graph.inner.edges_connecting(node, node).next().is_some()
        })
        .map(|component| {
            let mut modules: Vec<ModuleInfo> = component
                .iter()
                .filter_map(|&idx| graph.inner.node_weight(idx))
                .map(|node| build_module_info(self.world, node))
                .collect();
            modules.sort_by(|a, b| a.instance_name.cmp(&b.instance_name));
            modules
        })
        .collect()
}
```

Sort each cycle's members and the outer list deterministically (by first member's instance name) so test assertions are stable.

### Decision 3 — `compatible_inputs_for` same-module exclusion

Given a source output port on module A, should the compatible-inputs list include input ports on module A itself?

**(A)** Include same-module inputs. Self-connection is a valid pattern (rare, but legal) — let the user decide.
**(B)** Exclude same-module inputs. Most output→input connections cross modules; same-module is usually a mistake.

**Recommendation: (A).** Include. v0 doesn't prevent the user from making self-connections via direct API; the compatibility filter shouldn't be more restrictive than what's actually wireable. If user-experience research later shows self-connections are usually bugs, add an exclusion flag non-breakingly.

### Decision 4 — Cables sort key (part-2 follow-up)

Part-2 review flagged that `cables()` doesn't sort while `modules()` does. Resolve in part 3 for consistency. Sort key:

**(A)** `(source_module_instance, source_port_name, target_module_instance, target_port_name)` — full tuple.
**(B)** `(source_module_instance, target_module_instance)` — module-pair only.

**Recommendation: (A).** Full tuple gives stable ordering even when two modules have multiple cables between them. Costs nothing.

---

## Locked decisions (recap)

| # | Decision |
|---|----------|
| L1 | Part 3 closes out the 18-query catalog. After part 3, bucket 4 is feature-complete for v0. |
| L2 | Edges are undifferentiated (CableKind deferred). Topological order treats all edges; cycles report combined Signal/Modulation cycles. |
| L3 | Knob queries deferred to bucket-getter (D-25.1). |
| L4 | All query results are sorted deterministically (instance names; lexicographic tuples for cables). |
| L5 | Test discipline carries forward: `matches!` + field checks; no `.to_string().contains(...)`. |
| L6 | Anti-softpedal patterns: claim-without-code; silent failure modes; cross-type ordering drift. |

---

## Work mode

**Thinking work stays in your context** with the Rust skill loaded. This includes: deciding the exact `CycleError` doc-comment phrasing (the error must be self-explanatory in REPL/log output); handling the petgraph `Cycle<NodeIndex>` → `Vec<Vec<String>>` conversion (toposort's `Err` contains one node from one cycle, not all cycles — `cycles()` call is needed for the full set); thinking through whether `compatible_inputs_for(invalid_port_entity)` should panic, return empty, or return `None`; verifying that self-loops are correctly detected via `edges_connecting(n, n)` and not missed.

**Lookup work may delegate to subagents.** This includes: verifying `ValueKind`'s `PartialEq` derive in `value.rs`; confirming petgraph 0.7's exact API for `toposort` (signature stability across patch versions); finding the existing `sort_by` patterns in part 2 for `cables()` follow-up consistency; verifying `Dfs::next(&graph)` returns `NodeIndex` so the closure traversals compose cleanly.

Serial execution on thinking. Quality over speed.

---

## The task

### 1. `CycleError` type (`crates/oxurack-graph/src/lib.rs`)

Per Decision 1:

```rust
/// Error returned by [`GraphView::topological_order`] when the graph
/// contains cycles.
///
/// Each inner `Vec<String>` is one cycle, listing the instance names
/// of the modules involved. Self-loops appear as single-element vectors.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum CycleError {
    #[error("graph contains {} cycle(s)", .0.len())]
    Cycles(Vec<Vec<String>>),
}
```

Note: `thiserror` is already a transitive dependency via `oxurack-core::TagError`. Verify it's available; if not, add to `Cargo.toml`.

### 2. Diagnostic queries on `GraphView`

Add the following methods to the existing `impl<'w> GraphView<'w>` block. Doc-comments must explain the contract, complexity, and any edge cases (empty graph, isolated nodes, etc.) — match the style of part-2 methods.

```rust
// ── Diagnostics ────────────────────────────────────────────────

/// Modules with no incident cables (neither incoming nor outgoing).
///
/// `O(n)` — iterates all nodes once.
pub fn dangling_modules(&self) -> Vec<ModuleInfo> { /* … */ }

/// Output ports with no outgoing cable, and input ports with no
/// incoming cable, grouped by their parent module.
///
/// Returns `Vec<(ModuleInfo, PortInfo)>` so callers know which
/// module each dangling port belongs to. Sorted by
/// `(instance_name, port_name)`.
///
/// `O(n * k + m)` where `n` is modules, `k` is ports per module,
/// `m` is cables.
pub fn dangling_ports(&self) -> Vec<(ModuleInfo, PortInfo)> { /* … */ }

// ── Ordering ───────────────────────────────────────────────────

/// Topological order of modules.
///
/// Treats all cables as flow edges (CableKind distinction deferred).
/// Returns `Err(CycleError)` if the graph contains cycles.
///
/// `O(n + m)`.
pub fn topological_order(&self) -> Result<Vec<ModuleInfo>, CycleError> {
    let graph = self.raw();
    match petgraph::algo::toposort(&graph.inner, None) {
        Ok(order) => Ok(order
            .into_iter()
            .filter_map(|idx| graph.inner.node_weight(idx))
            .map(|node| build_module_info(self.world, node))
            .collect()),
        Err(_) => {
            // toposort's error only includes one cycle node, not all
            // cycles. For a complete picture, run tarjan_scc.
            let cycle_names: Vec<Vec<String>> = self
                .cycles()
                .into_iter()
                .map(|cycle| cycle.into_iter().map(|m| m.instance_name).collect())
                .collect();
            Err(CycleError::Cycles(cycle_names))
        }
    }
}

// ── Cycles ─────────────────────────────────────────────────────

/// All cycles in the graph.
///
/// Each inner `Vec<ModuleInfo>` is one cycle. Includes self-loops
/// as single-element vectors. Empty when the graph is acyclic.
///
/// `O(n + m)` via Tarjan's SCC algorithm.
pub fn cycles(&self) -> Vec<Vec<ModuleInfo>> { /* per Decision 2 */ }

// ── Reachability ───────────────────────────────────────────────

/// All modules reachable downstream from the given module
/// (following outgoing edges). Includes the starting module.
///
/// Returns empty if the entity is not a registered module. `O(n + m)`.
pub fn downstream_closure(&self, module_entity: Entity) -> Vec<ModuleInfo> { /* … */ }

/// All modules reachable upstream from the given module
/// (following incoming edges). Includes the starting module.
///
/// Returns empty if the entity is not a registered module. `O(n + m)`.
pub fn upstream_closure(&self, module_entity: Entity) -> Vec<ModuleInfo> { /* … */ }

// ── Connection discovery ───────────────────────────────────────

/// Input ports in the rack that could legally accept a cable from
/// the given output port (by matching `ValueKind`).
///
/// Includes same-module inputs (self-connection is allowed per
/// part-3 Decision 3). Returns empty if the entity is not a
/// registered port. Sorted by `(instance_name, port_name)`.
///
/// `O(n * k)` where `n` is modules and `k` is ports per module.
pub fn compatible_inputs_for(
    &self,
    output_port: Entity,
) -> Vec<(ModuleInfo, PortInfo)> { /* … */ }
```

**Implementation notes:**

- `dangling_modules`: iterate `graph.inner.node_indices()`, for each call `graph.inner.edges(idx).count() == 0 && graph.inner.edges_directed(idx, Incoming).count() == 0`. Or simpler: `graph.inner.neighbors_undirected(idx).count() == 0`.

- `dangling_ports`: for each node, walk its port children via `Children`; for each port, count cables that reference it as source (outputs) or target (inputs); empty count = dangling. The cable-side check requires iterating `graph.inner.edge_weights()` and matching `source_port` / `target_port` — O(m) per port lookup; for the full query, build a `HashSet<Entity>` of all cable source/target ports once at the top of the function, then membership-check each port in O(1).

- `downstream_closure` / `upstream_closure`: use `petgraph::visit::Dfs::new(&graph.inner, start_idx)` for downstream; for upstream, wrap in `petgraph::visit::Reversed(&graph.inner)`. Iterate `dfs.next(&graph)` until `None`.

- `compatible_inputs_for`: read source port's `ValueKind`; iterate all modules' inputs via the same `Children` → `Port` traversal pattern as `ports_of`; filter by `direction == Input && value_kind == source_value_kind`; build result tuples; sort.

### 3. Cables sort (part-2 follow-up)

Update `GraphView::cables()` to sort by `(source_module_instance, source_port_name, target_module_instance, target_port_name)`. Mirror the existing `.sort_by(...)` pattern in `modules()`.

### 4. Exports

Re-export `CycleError` from `oxurack-graph::lib.rs` alongside the existing exports.

---

## Tests

Test discipline carries forward: `matches!` + field checks; no `.to_string().contains(...)`; positive paths assert observable state.

### Dangling-modules tests

1. `test_dangling_modules_empty_graph` — fresh graph returns empty.
2. `test_dangling_modules_all_connected` — three-module chain `vco → filter → out`; `dangling_modules()` returns empty.
3. `test_dangling_modules_finds_isolated` — graph with one isolated module (no cables) and a connected pair; `dangling_modules()` returns just the isolated module.

### Dangling-ports tests

4. `test_dangling_ports_empty_graph` — fresh graph returns empty.
5. `test_dangling_ports_all_wired` — module pair fully wired (every input has a cable, every output has a cable); `dangling_ports()` returns empty.
6. `test_dangling_ports_finds_unwired_output` — module with output port not connected to anything; result includes that port.
7. `test_dangling_ports_finds_unwired_input` — module with input port not connected to anything; result includes that port.

### Topological-order tests

8. `test_topological_order_linear_chain` — three modules `a → b → c`; `topological_order()?` returns `[a, b, c]` in that order.
9. `test_topological_order_diamond` — `a → b → d`, `a → c → d`; order has `a` first and `d` last (intermediate order may vary; assert via index relationships, not equality of full vector).
10. `test_topological_order_returns_cycle_error_on_cycle` — two modules `a → b → a`; `topological_order()` returns `Err(CycleError::Cycles(_))`. Assert via `matches!` + len check.
11. `test_topological_order_empty_graph` — fresh graph returns `Ok(vec![])`.

### Cycle-detection tests

12. `test_cycles_empty_graph_returns_empty` — `cycles().is_empty()`.
13. `test_cycles_acyclic_graph_returns_empty` — three-module chain `a → b → c`; `cycles().is_empty()`.
14. `test_cycles_finds_two_node_cycle` — `a → b → a`; `cycles().len() == 1`, the cycle contains `[a, b]` (sorted alphabetically per Decision 2 sort).
15. `test_cycles_finds_self_loop` — module `a` with a cable from its output back to its own input; `cycles().len() == 1`, cycle contains `[a]` (singleton).
16. `test_cycles_multiple_disjoint_cycles` — two separate cycles `a → b → a` and `c → d → c`; `cycles().len() == 2`.

### Reachability tests

17. `test_downstream_closure_linear_chain` — `a → b → c`; `downstream_closure(a)` returns `[a, b, c]`. Order: traversal order or sorted; assert content as a set.
18. `test_downstream_closure_diamond` — `a → b → d`, `a → c → d`; `downstream_closure(a)` returns all four; `downstream_closure(b)` returns `[b, d]`.
19. `test_downstream_closure_isolated_module` — module with no cables; `downstream_closure(entity)` returns just `[that_module]`.
20. `test_upstream_closure_linear_chain` — `a → b → c`; `upstream_closure(c)` returns `[a, b, c]`.
21. `test_upstream_closure_unknown_entity_returns_empty` — entity not in graph; result is empty.

### Compatibility-filter tests

22. `test_compatible_inputs_for_finds_matching_kind` — two modules each with one Float input; `compatible_inputs_for(some_float_output_port)` returns both inputs.
23. `test_compatible_inputs_for_excludes_mismatched_kind` — module with one Float input, one MidiNote input; source is a Float output; result includes only the Float input.
24. `test_compatible_inputs_for_includes_same_module` — module with both an output and a matching input (self-connection candidate); result includes the same module's input (per Decision 3).
25. `test_compatible_inputs_for_unknown_port_returns_empty` — entity isn't a port at all; result is empty.

### Cables sort (part-2 follow-up)

26. `test_cables_sorted_by_endpoints` — patch with cables `vco_1.out → filter_1.in`, `lfo_1.out → vca_1.cv`; `cables()` returns them sorted alphabetically by source module.

### Total: 26 tests.

---

## Out of scope for part 3

- **Knob queries** (`knobs_of(entity)`) — deferred to bucket-getter (D-25.1).
- **Signal-vs-Modulation cycle reporting** — deferred until `CableKind` ships (D-25.7). v0 reports combined cycles.
- **Subgraph extraction beyond closures** — not in the original 18-query catalog.
- **Cycle counting / weight** — `cycles().len()` is sufficient; no specialized "is_cyclic" boolean fast-path.
- **Closing any `NotImplemented` arms** in `dispatch_core_command`.

---

## Suggested steps

1. **Recon.** Verify part-2 commit landed and the tree compiles. Verify `ValueKind` derives `PartialEq`. Verify `thiserror` is available transitively or add it.
2. **`CycleError` type.** Declare + export.
3. **Cables sort** (part-2 follow-up). Smallest change; lands first. Test #26 verifies.
4. **`dangling_modules` and `dangling_ports`.** Tests #1–#7.
5. **`topological_order` + `cycles`.** They interlock — `topological_order`'s `Err` path calls `cycles()`. Implement `cycles()` first, then `topological_order`. Tests #8–#16.
6. **Reachability: `downstream_closure` and `upstream_closure`.** Tests #17–#21.
7. **`compatible_inputs_for`.** Tests #22–#25.
8. **Run `make format`, `make lint`, `make test`.** All green.
9. **Commit** as `Bucket 4 (part 3): diagnostic queries — closing the 18-query catalog`.

---

## DoD (every criterion is a required pass condition)

1. `cargo clean && make check` passes from a fresh state.
2. `cargo build --no-default-features -p oxurack` succeeds.
3. `CycleError` type defined per §1, exported from `oxurack-graph` crate root.
4. `GraphView::dangling_modules` implemented and tested (tests #1–#3).
5. `GraphView::dangling_ports` implemented and tested (tests #4–#7).
6. `GraphView::topological_order` implemented; returns `Err(CycleError::Cycles)` for cyclic graphs; tests #8–#11.
7. `GraphView::cycles` implemented per Decision 2 (includes self-loops; sorted); tests #12–#16.
8. `GraphView::downstream_closure` implemented via `petgraph::visit::Dfs`; tests #17–#19.
9. `GraphView::upstream_closure` implemented via `petgraph::visit::Reversed + Dfs`; tests #20–#21.
10. `GraphView::compatible_inputs_for` implemented per Decision 3 (includes same-module inputs); tests #22–#25.
11. `GraphView::cables` sorted by `(source_module_instance, source_port_name, target_module_instance, target_port_name)`; test #26.
12. Doc-comments on every new method follow the part-2 style: complexity annotation (`O(n)`, `O(n + m)`, etc.); explicit edge-case behavior; entity-type expectation (module vs. port).
13. All 26 new tests pass.
14. No `.to_string().contains(...)` patterns in any new test.
15. Four `NotImplemented` arms in `dispatch_core_command` unchanged.
16. `CycleError`'s `Display` impl produces a meaningful message (e.g., `"graph contains 2 cycle(s)"`); tested via a dedicated Display test.
17. Self-loop detection in `cycles()` verified by test #15 (a node with a self-loop is reported as a singleton cycle).
18. `topological_order` error path actually constructs the full cycle set via `cycles()`, not just the one cycle node petgraph's `Err` reports. Test #10 asserts the structure of the returned `CycleError`.
19. The bucket-4-part-3 commit lands as a single atomic commit with a clear message.
20. After part 3 commits, run grep `rg "TODO\(bucket-4\)|TODO\(part-[23]\)" crates/oxurack-graph/` and verify zero hits (or document any explicit deferrals).

---

## Completion report format

```
Bucket 4 (part 3) complete at <commit sha>.

Recon findings:
- Part 2 commit verified: <SHA>
- ValueKind derives PartialEq: <yes / no — describe if missing>
- thiserror available transitively: <yes / no>
- <any other recon-revealed shape difference>

Implementation choices:
- Decision 1 (CycleError shape): <A — Vec<Vec<String>> / B / C>
- Decision 2 (cycles self-loop handling): <as specified / variant>
- Decision 3 (compatible_inputs same-module): <A — include / B — exclude>
- Decision 4 (cables sort key): <A — full tuple / B — module pair>

New types added (N):
- CycleError — error from topological_order

New methods on GraphView (N):
- <list with one-line each>

Methods modified:
- GraphView::cables — sort key added

Code files changed (N):
- <list>

Tests added (N):
- <list with one-line summaries>

Verification: cargo clean && make check — <passed / failures>
Verification: cargo build --no-default-features -p oxurack — <passed / failures>
Verification: self-loop detected as cycle (test #15) — <yes / no>
Verification: topological_order error path returns full cycle set, not just one — <yes / no>
Verification: compatible_inputs_for ValueKind comparison direct (not stringified) — <yes / no>
Verification: 4 NotImplemented arms unchanged — <yes / no>
Verification: cables() sorted (test #26) — <yes / no>

Cross-bucket dependencies surfaced:
- <list, or "none">

Design memo follow-up edits required:
- <list, or "none">

Deferred items registry updates required:
- D-XX.x entries: <status per item, if any change>

Bucket 4 v0 query catalog status:
- 17 of 18 queries shipped across parts 1–3.
- 1 deferred: knobs_of (blocked on bucket-getter / D-25.1).

Spec-adherence walk:
- DoD #1: met / not met
- ... (all 20)

Nothing silently skipped.
```

---

## Anti-softpedal reminder

Watch-items particular to this bucket:

- **Self-loop missed by `cycles()`.** petgraph's `tarjan_scc` returns singleton SCCs for *every* node, including ones with no self-loop. If the filter logic checks only `component.len() > 1`, self-loops are silently dropped. Test #15 specifically catches this; verify the implementation uses `edges_connecting(n, n)` (or equivalent) to distinguish.
- **`topological_order` error path silently returns partial information.** petgraph's `toposort` `Err` carries one node from one cycle. If the wrapper returns `CycleError::Cycles(vec![vec![that_one_name]])`, consumers see only one cycle even when many exist. The implementation must call `cycles()` to get the full set. Test #10 should assert that the returned cycle list has the right shape.
- **`compatible_inputs_for` stringifies `ValueKind` for comparison.** If the implementation does `port_a.value_kind.to_string() == port_b.value_kind.to_string()`, it works today but breaks when a new `ValueKind` variant has Debug output matching another. Compare `ValueKind` values directly via `PartialEq`. If `ValueKind` doesn't derive `PartialEq`, that's a small `oxurack-core` follow-up — flag rather than work around.
- **`downstream_closure(invalid_entity)` panics instead of returning empty.** The contract says "returns empty if entity isn't a registered module." If the implementation looks up via `node_by_entity.get(&entity).unwrap()`, it panics on missing — silent contract violation in the inverse direction. Use `?` or `match` with a `return Vec::new()` for the missing case.
- **`cycles()` and `topological_order` disagree.** If `cycles()` reports zero cycles but `topological_order()` returns `Err`, something is wrong with one of them. Worth a sanity check in a test: for an acyclic graph, both return their "no cycles" form (`vec![]` and `Ok(_)` respectively).
- **Cables sort drift.** When you add the `.sort_by(...)` to `cables()`, the existing tests in part 2 that assert `cables().len() == N` won't break, but tests that assert specific cable order (if any) might. Verify by reading the part-2 cable tests before changing the sort.

If recon reveals that `ValueKind` doesn't derive `PartialEq` or that any other part-3 dependency is missing, **stop and report**.

---

## After you report

I'll do trust-but-verify: confirm `cycles()` reports self-loops via test #15 inspection; verify `topological_order`'s error path uses `cycles()` (read the implementation, not just trust the report); check `compatible_inputs_for` does direct `ValueKind` comparison; verify `cables()` is sorted via the test #26 assertion + reading the implementation; spot-check that no `.unwrap()` lurks in closure-traversal lookups (returning empty is the spec'd behavior for missing entities). Then we sign off and bucket 4 is closed.


## Version History

- 2026-09-17 — Planning migration: renamed `0007-bucket-4-part-3-diagnostic-queries.md` (formerly `docs/dev/0007-bucket-4-part-3-diagnostic-queries.md`) to `slice-plan.md` in `arc04-rackgraph/slice03-diagnostics`. Original content retained as historical planning; this rename does not establish current implementation or acceptance status. See the project plan and migration report for the current baseline.

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
