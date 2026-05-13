//! Materialized graph view over the oxurack rack's structural state.
//!
//! [`RackGraph`] is a Bevy resource backed by petgraph's `StableDiGraph`
//! that tracks modules (nodes) and cables (edges). It is maintained
//! reactively by [`RackGraphPlugin`], which subscribes to
//! [`ModuleAdded`](oxurack_core::ModuleAdded),
//! [`ModuleRemoved`](oxurack_core::ModuleRemoved),
//! [`CableConnected`](oxurack_core::CableConnected), and
//! [`CableDisconnected`](oxurack_core::CableDisconnected) notifications
//! emitted by `oxurack-core`.
//!
//! Labels and parameter values are *not* stored in the graph — they are
//! fetched fresh from the ECS world at query time via [`GraphView`].
//! This avoids label-staleness bugs.
//!
//! # Feature gate
//!
//! This crate is pulled into the umbrella `oxurack` crate behind the
//! `graph` Cargo feature (enabled by default).

use std::collections::HashMap;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

use oxurack_core::{
    CableConnected, CableDisconnected, CableTransform, Description, DisplayName,
    DisplayNameFormatter, ModuleAdded, ModuleKind, ModuleRemoved, Port, Tags, display_name_of,
};

// ── Core graph types ───────────────────────────────────────────────────

/// Materialized graph view over the rack's structural state.
///
/// Maintained reactively by [`RackGraphPlugin`] subscribing to
/// structural-mutation notifications from `oxurack-core`. Queries are
/// pure reads against the internal petgraph storage.
///
/// Node identity is [`Entity`] (the ECS entity for a module). Edge
/// identity is the cable's [`Entity`].
///
/// Labels and parameter values are *not* stored here — always fetched
/// fresh from the ECS world via [`GraphView`]. Avoids label-staleness.
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
    /// The ECS entity for this module.
    pub entity: Entity,
    /// User-facing instance name (e.g. `"vco_1"`).
    pub instance_name: String,
    /// Module kind (e.g. `"vco"`, `"lfo"`).
    pub kind: ModuleKind,
}

/// A cable edge in the rack graph.
///
/// v0 edges are undifferentiated — Signal vs Modulation distinction is
/// deferred (D-25.7). When `CableKind` ships, this struct grows a
/// `kind` field non-breakingly.
#[derive(Clone, Debug, PartialEq)]
pub struct RackEdge {
    /// The ECS entity for this cable.
    pub cable_entity: Entity,
    /// The output port entity this cable reads from.
    pub source_port: Entity,
    /// The input port entity this cable writes to.
    pub target_port: Entity,
    /// Optional inline signal transform.
    pub transform: Option<CableTransform>,
}

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

// ── Result types ───────────────────────────────────────────────────────

/// Snapshot of a module's queryable state.
///
/// Built fresh at query time by reading the ECS world. The
/// `entity_bits` field carries `Entity::to_bits()` — a `u64` that
/// preserves both the entity index and generation counter for
/// cross-process serialization.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Entity serialized via `Entity::to_bits()`.
    pub entity_bits: u64,
    /// User-facing instance name.
    pub instance_name: String,
    /// Module kind, stringified for cross-process portability.
    pub kind: String,
    /// Resolved display name (via `DisplayNameFormatter` fallback).
    pub display_name: String,
    /// Optional description label.
    pub description: Option<String>,
    /// Tags attached to this module.
    pub tags: Vec<String>,
}

/// Snapshot of a cable's queryable state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CableInfo {
    /// Cable entity serialized via `Entity::to_bits()`.
    pub entity_bits: u64,
    /// Instance name of the source module.
    pub source_module_instance: String,
    /// Name of the source port.
    pub source_port_name: String,
    /// Instance name of the target module.
    pub target_module_instance: String,
    /// Name of the target port.
    pub target_port_name: String,
    /// Optional display name.
    pub display_name: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Tags attached to this cable.
    pub tags: Vec<String>,
}

/// Snapshot of a port on a module.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PortInfo {
    /// Port name.
    pub name: String,
    /// Port direction (input or output).
    pub direction: PortDirection,
    /// Value kind, stringified for cross-process portability.
    pub value_kind: String,
}

/// Serializable port direction.
///
/// A serde-enabled mirror of `oxurack_core::PortDirection` for
/// cross-process result types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortDirection {
    /// The port receives signals.
    Input,
    /// The port emits signals.
    Output,
}

/// All ports on a single module, separated by direction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PortSet {
    /// Input ports on this module.
    pub inputs: Vec<PortInfo>,
    /// Output ports on this module.
    pub outputs: Vec<PortInfo>,
}

// ── Result-type builders ───────────────────────────────────────────────

fn convert_direction(d: oxurack_core::PortDirection) -> PortDirection {
    match d {
        oxurack_core::PortDirection::Input => PortDirection::Input,
        oxurack_core::PortDirection::Output => PortDirection::Output,
        // Non-exhaustive: future variants fall through to Input as a
        // safe default. This will surface as a test failure if new
        // variants are added.
        _ => PortDirection::Input,
    }
}

fn build_port_info(port: &Port) -> PortInfo {
    PortInfo {
        name: port.name.as_ref().to_string(),
        direction: convert_direction(port.direction),
        value_kind: format!("{:?}", port.value_kind),
    }
}

fn build_module_info(world: &World, node: &RackNode) -> ModuleInfo {
    let formatter = world.resource::<DisplayNameFormatter>();
    let explicit = world.get::<DisplayName>(node.entity);
    let display_name = display_name_of(formatter, explicit, &node.instance_name);

    let description = world.get::<Description>(node.entity).map(|d| d.0.clone());
    let tags = world
        .get::<Tags>(node.entity)
        .map(|t| t.0.iter().map(|tag| tag.as_str().to_string()).collect())
        .unwrap_or_default();

    ModuleInfo {
        entity_bits: node.entity.to_bits(),
        instance_name: node.instance_name.clone(),
        kind: node.kind.to_string(),
        display_name,
        description,
        tags,
    }
}

/// Resolve a port entity to its parent module's instance name and the
/// port's own name.
fn resolve_port_context(world: &World, port_entity: Entity) -> (String, String) {
    let port = world
        .get::<Port>(port_entity)
        .expect("port entity must have a Port component");
    let port_name = port.name.as_ref().to_string();

    let child_of = world
        .get::<ChildOf>(port_entity)
        .expect("port entity must have a ChildOf parent");
    let module_entity = child_of.parent();
    let module = world
        .get::<oxurack_core::Module>(module_entity)
        .expect("port parent must have a Module component");

    (module.instance_name.clone(), port_name)
}

fn build_cable_info(world: &World, edge: &RackEdge) -> CableInfo {
    let (source_module_instance, source_port_name) = resolve_port_context(world, edge.source_port);
    let (target_module_instance, target_port_name) = resolve_port_context(world, edge.target_port);

    let display_name = world
        .get::<DisplayName>(edge.cable_entity)
        .map(|d| d.0.clone());
    let description = world
        .get::<Description>(edge.cable_entity)
        .map(|d| d.0.clone());
    let tags = world
        .get::<Tags>(edge.cable_entity)
        .map(|t| t.0.iter().map(|tag| tag.as_str().to_string()).collect())
        .unwrap_or_default();

    CableInfo {
        entity_bits: edge.cable_entity.to_bits(),
        source_module_instance,
        source_port_name,
        target_module_instance,
        target_port_name,
        display_name,
        description,
        tags,
    }
}

// ── GraphView ──────────────────────────────────────────────────────────

/// Read-only view over the rack graph with label-resolving queries.
///
/// Created via [`GraphView::new`] or [`Oxurack::graph`]. All query
/// methods resolve labels (display names, descriptions, tags) fresh
/// from the ECS world at call time — no caching, no staleness.
pub struct GraphView<'w> {
    world: &'w World,
}

impl<'w> GraphView<'w> {
    /// Create a new graph view over the given world.
    pub fn new(world: &'w World) -> Self {
        Self { world }
    }

    /// Borrow the raw [`RackGraph`] for part-1 membership queries.
    pub fn raw(&self) -> &RackGraph {
        self.world.resource::<RackGraph>()
    }

    // ── Pass-through membership queries ────────────────────────────

    /// Number of modules in the graph. `O(1)`.
    pub fn module_count(&self) -> usize {
        self.raw().module_count()
    }

    /// Number of cables in the graph. `O(1)`.
    pub fn cable_count(&self) -> usize {
        self.raw().cable_count()
    }

    /// Whether the given entity is registered as a module node. `O(1)`.
    pub fn contains_module(&self, entity: Entity) -> bool {
        self.raw().contains_module(entity)
    }

    /// Whether the given entity is registered as a cable edge. `O(1)`.
    pub fn contains_cable(&self, entity: Entity) -> bool {
        self.raw().contains_cable(entity)
    }

    /// Resolve an instance name to its module entity. `O(1)`.
    pub fn entity_by_instance_name(&self, name: &str) -> Option<Entity> {
        self.raw().entity_by_instance_name(name)
    }

    // ── Rosters ────────────────────────────────────────────────────

    /// All modules in the graph, with labels resolved.
    ///
    /// Results are sorted by `instance_name` for deterministic output
    /// (petgraph's internal iteration order is not guaranteed). `O(n)`.
    pub fn modules(&self) -> Vec<ModuleInfo> {
        let graph = self.raw();
        let mut modules: Vec<ModuleInfo> = graph
            .inner
            .node_weights()
            .map(|node| build_module_info(self.world, node))
            .collect();
        modules.sort_by(|a, b| a.instance_name.cmp(&b.instance_name));
        modules
    }

    /// All cables in the graph, with labels resolved. `O(m)`.
    pub fn cables(&self) -> Vec<CableInfo> {
        let graph = self.raw();
        graph
            .inner
            .edge_weights()
            .map(|edge| build_cable_info(self.world, edge))
            .collect()
    }

    /// Histogram of module kinds present in the graph. `O(n)`.
    pub fn module_kinds(&self) -> HashMap<ModuleKind, usize> {
        let graph = self.raw();
        let mut counts = HashMap::new();
        for node in graph.inner.node_weights() {
            *counts.entry(node.kind.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// All modules of a given kind, with labels resolved. `O(n)`.
    pub fn modules_of_kind(&self, kind: &ModuleKind) -> Vec<ModuleInfo> {
        let graph = self.raw();
        let mut modules: Vec<ModuleInfo> = graph
            .inner
            .node_weights()
            .filter(|node| &node.kind == kind)
            .map(|node| build_module_info(self.world, node))
            .collect();
        modules.sort_by(|a, b| a.instance_name.cmp(&b.instance_name));
        modules
    }

    // ── Lookup ─────────────────────────────────────────────────────

    /// Look up a module by its instance name. `O(1)`.
    pub fn module_by_name(&self, name: &str) -> Option<ModuleInfo> {
        let graph = self.raw();
        let entity = *graph.by_instance_name.get(name)?;
        let idx = *graph.node_by_entity.get(&entity)?;
        let node = graph.inner.node_weight(idx)?;
        Some(build_module_info(self.world, node))
    }

    // ── Ports ──────────────────────────────────────────────────────

    /// All ports on a given module entity, split into inputs and
    /// outputs.
    ///
    /// Returns an empty [`PortSet`] if the entity is not a registered
    /// module or has no port children. `O(k)` where `k` is the number
    /// of children. Expects a **module entity**, not a port entity.
    pub fn ports_of(&self, module_entity: Entity) -> PortSet {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        if let Some(children) = self
            .world
            .get::<bevy_ecs::hierarchy::Children>(module_entity)
        {
            for child in children.iter() {
                if let Some(port) = self.world.get::<Port>(child) {
                    let info = build_port_info(port);
                    match info.direction {
                        PortDirection::Input => inputs.push(info),
                        PortDirection::Output => outputs.push(info),
                    }
                }
            }
        }

        inputs.sort_by(|a, b| a.name.cmp(&b.name));
        outputs.sort_by(|a, b| a.name.cmp(&b.name));

        PortSet { inputs, outputs }
    }

    // ── Neighborhoods ──────────────────────────────────────────────

    /// Modules upstream of the given module (have a cable feeding it).
    ///
    /// Returns modules that have an outgoing edge targeting this
    /// module. `O(deg)` where `deg` is the in-degree. Expects a
    /// **module entity**.
    pub fn upstream_of(&self, module_entity: Entity) -> Vec<ModuleInfo> {
        let graph = self.raw();
        let Some(&idx) = graph.node_by_entity.get(&module_entity) else {
            return Vec::new();
        };
        let mut seen = HashMap::new();
        for edge in graph
            .inner
            .edges_directed(idx, petgraph::Direction::Incoming)
        {
            let source_idx = edge.source();
            if let Some(node) = graph.inner.node_weight(source_idx) {
                seen.entry(node.entity)
                    .or_insert_with(|| build_module_info(self.world, node));
            }
        }
        let mut result: Vec<ModuleInfo> = seen.into_values().collect();
        result.sort_by(|a, b| a.instance_name.cmp(&b.instance_name));
        result
    }

    /// Modules downstream of the given module (receive a cable from
    /// it).
    ///
    /// Returns modules that have an incoming edge from this module.
    /// `O(deg)` where `deg` is the out-degree. Expects a **module
    /// entity**.
    pub fn downstream_of(&self, module_entity: Entity) -> Vec<ModuleInfo> {
        let graph = self.raw();
        let Some(&idx) = graph.node_by_entity.get(&module_entity) else {
            return Vec::new();
        };
        let mut seen = HashMap::new();
        for edge in graph
            .inner
            .edges_directed(idx, petgraph::Direction::Outgoing)
        {
            let target_idx = edge.target();
            if let Some(node) = graph.inner.node_weight(target_idx) {
                seen.entry(node.entity)
                    .or_insert_with(|| build_module_info(self.world, node));
            }
        }
        let mut result: Vec<ModuleInfo> = seen.into_values().collect();
        result.sort_by(|a, b| a.instance_name.cmp(&b.instance_name));
        result
    }

    /// Cables coming into the given module. `O(deg)`.
    ///
    /// Expects a **module entity**.
    pub fn incoming_cables(&self, module_entity: Entity) -> Vec<CableInfo> {
        let graph = self.raw();
        let Some(&idx) = graph.node_by_entity.get(&module_entity) else {
            return Vec::new();
        };
        graph
            .inner
            .edges_directed(idx, petgraph::Direction::Incoming)
            .map(|e| build_cable_info(self.world, e.weight()))
            .collect()
    }

    /// Cables going out from the given module. `O(deg)`.
    ///
    /// Expects a **module entity**.
    pub fn outgoing_cables(&self, module_entity: Entity) -> Vec<CableInfo> {
        let graph = self.raw();
        let Some(&idx) = graph.node_by_entity.get(&module_entity) else {
            return Vec::new();
        };
        graph
            .inner
            .edges_directed(idx, petgraph::Direction::Outgoing)
            .map(|e| build_cable_info(self.world, e.weight()))
            .collect()
    }
}

// ── GraphMaintenance schedule ──────────────────────────────────────────

/// Schedule label for the four graph-maintenance systems.
///
/// Runs as part of [`Update`] for normal app ticks, and can be
/// invoked synchronously via `world.run_schedule(GraphMaintenance)`
/// to honor the freshness contract from `Oxurack` mutation methods.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphMaintenance;

// ── Plugin ─────────────────────────────────────────────────────────────

/// Bevy plugin that maintains the [`RackGraph`] resource.
///
/// Registers the `RackGraph` resource and four maintenance systems
/// that subscribe to structural-mutation notifications:
///
/// - [`ModuleAdded`](oxurack_core::ModuleAdded) → add a node
/// - [`ModuleRemoved`](oxurack_core::ModuleRemoved) → remove a node
///   (plus incident edges)
/// - [`CableConnected`](oxurack_core::CableConnected) → add an edge
/// - [`CableDisconnected`](oxurack_core::CableDisconnected) → remove
///   an edge
///
/// Maintenance systems run in the [`GraphMaintenance`] schedule,
/// which is executed as part of every [`Update`] tick and can also
/// be triggered synchronously for the freshness contract.
pub struct RackGraphPlugin;

impl Plugin for RackGraphPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RackGraph>()
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
            .add_systems(Update, run_graph_maintenance);
    }
}

fn run_graph_maintenance(world: &mut World) {
    world.run_schedule(GraphMaintenance);
}

// ── Maintenance systems ────────────────────────────────────────────────

fn handle_module_added(mut graph: ResMut<RackGraph>, mut reader: MessageReader<ModuleAdded>) {
    for msg in reader.read() {
        let node = RackNode {
            entity: msg.entity,
            instance_name: msg.instance_name.clone(),
            kind: msg.kind.clone(),
        };
        let idx = graph.inner.add_node(node);
        graph.node_by_entity.insert(msg.entity, idx);
        graph
            .by_instance_name
            .insert(msg.instance_name.clone(), msg.entity);
    }
}

fn handle_module_removed(mut graph: ResMut<RackGraph>, mut reader: MessageReader<ModuleRemoved>) {
    for msg in reader.read() {
        if let Some(idx) = graph.node_by_entity.remove(&msg.entity) {
            // StableDiGraph::remove_node also removes incident edges.
            // Collect cable entities from incident edges first so we
            // can prune edge_by_entity before the edges disappear.
            let cable_entities: Vec<Entity> = graph
                .inner
                .edges(idx)
                .filter_map(|e| graph.inner.edge_weight(e.id()).map(|w| w.cable_entity))
                .collect();
            for cable_entity in cable_entities {
                graph.edge_by_entity.remove(&cable_entity);
            }
            graph.inner.remove_node(idx);
            graph.by_instance_name.remove(&msg.instance_name);
        }
    }
}

fn handle_cable_connected(
    mut graph: ResMut<RackGraph>,
    mut reader: MessageReader<CableConnected>,
    children: Query<&ChildOf>,
) {
    for msg in reader.read() {
        // Resolve port entities → parent module entities via ChildOf.
        let source_module = children
            .get(msg.source_port)
            .map(|c| c.parent())
            .expect("source port must have a ChildOf parent");
        let target_module = children
            .get(msg.target_port)
            .map(|c| c.parent())
            .expect("target port must have a ChildOf parent");

        let source_idx = graph
            .node_by_entity
            .get(&source_module)
            .copied()
            .expect("source module must be in the graph");
        let target_idx = graph
            .node_by_entity
            .get(&target_module)
            .copied()
            .expect("target module must be in the graph");

        let edge = RackEdge {
            cable_entity: msg.cable_entity,
            source_port: msg.source_port,
            target_port: msg.target_port,
            transform: msg.transform,
        };
        let edge_idx = graph.inner.add_edge(source_idx, target_idx, edge);
        graph.edge_by_entity.insert(msg.cable_entity, edge_idx);
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

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::App;
    use oxurack_core::{Tag, Tags};
    use pretty_assertions::assert_eq;
    use std::collections::BTreeSet;

    fn graph_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(oxurack_core::CorePlugin);
        app.add_plugins(RackGraphPlugin);
        app
    }

    /// Spawn a minimal module entity with the `Module` component and
    /// optionally port children, then emit `ModuleAdded`.
    fn spawn_module(
        app: &mut App,
        instance_name: &str,
        kind: &str,
        ports: &[(&str, oxurack_core::PortDirection)],
    ) -> Entity {
        let entity = oxurack_core::spawn_module_entity(app.world_mut(), kind, instance_name);
        for &(port_name, direction) in ports {
            oxurack_core::spawn_port_on_module(
                app.world_mut(),
                entity,
                port_name,
                direction,
                oxurack_core::ValueKind::Float,
                oxurack_core::MergePolicy::LastWins,
            );
        }
        app.world_mut().write_message(ModuleAdded {
            entity,
            instance_name: instance_name.to_string(),
            kind: ModuleKind::from(kind),
        });
        entity
    }

    fn connect_cable(app: &mut App, source_port: Entity, target_port: Entity) -> Entity {
        let cable = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(CableConnected {
            cable_entity: cable,
            source_port,
            target_port,
            transform: None,
        });
        cable
    }

    /// Find a port entity on a module by name.
    fn find_port(app: &mut App, module_entity: Entity, name: &str) -> Entity {
        let world = app.world_mut();
        let children = world
            .get::<bevy_ecs::hierarchy::Children>(module_entity)
            .expect("module should have children");
        for child in children.iter() {
            if let Some(port) = world.get::<Port>(child) {
                if port.name.as_ref() == name {
                    return child;
                }
            }
        }
        panic!("port '{name}' not found on entity {module_entity:?}");
    }

    // ── Part 1 tests (preserved) ──────────────────────────────────

    #[test]
    fn test_rack_graph_empty_on_init() {
        let graph = RackGraph::default();
        assert_eq!(graph.module_count(), 0);
        assert_eq!(graph.cable_count(), 0);
    }

    #[test]
    fn test_module_added_adds_node() {
        let mut app = graph_test_app();
        app.update();

        let entity = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(ModuleAdded {
            entity,
            instance_name: "vco_1".to_string(),
            kind: ModuleKind::from("vco"),
        });
        app.update();

        let graph = app.world().resource::<RackGraph>();
        assert_eq!(graph.module_count(), 1);
        assert!(graph.contains_module(entity));
        assert_eq!(graph.entity_by_instance_name("vco_1"), Some(entity));
    }

    #[test]
    fn test_two_modules_added() {
        let mut app = graph_test_app();
        app.update();

        let e1 = app.world_mut().spawn_empty().id();
        let e2 = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(ModuleAdded {
            entity: e1,
            instance_name: "vco_1".to_string(),
            kind: ModuleKind::from("vco"),
        });
        app.world_mut().write_message(ModuleAdded {
            entity: e2,
            instance_name: "filter_1".to_string(),
            kind: ModuleKind::from("filter"),
        });
        app.update();

        let graph = app.world().resource::<RackGraph>();
        assert_eq!(graph.module_count(), 2);
        assert!(graph.contains_module(e1));
        assert!(graph.contains_module(e2));
    }

    #[test]
    fn test_cable_connected_adds_edge() {
        let mut app = graph_test_app();
        app.update();

        let mod_a = app.world_mut().spawn_empty().id();
        let mod_b = app.world_mut().spawn_empty().id();
        let port_out = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(mod_a).add_child(port_out);
        let port_in = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(mod_b).add_child(port_in);

        app.world_mut().write_message(ModuleAdded {
            entity: mod_a,
            instance_name: "gen_1".to_string(),
            kind: ModuleKind::from("gen"),
        });
        app.world_mut().write_message(ModuleAdded {
            entity: mod_b,
            instance_name: "sink_1".to_string(),
            kind: ModuleKind::from("sink"),
        });
        app.update();

        let cable = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(CableConnected {
            cable_entity: cable,
            source_port: port_out,
            target_port: port_in,
            transform: None,
        });
        app.update();

        let graph = app.world().resource::<RackGraph>();
        assert_eq!(graph.cable_count(), 1);
        assert!(graph.contains_cable(cable));
    }

    #[test]
    fn test_module_removed_removes_node() {
        let mut app = graph_test_app();
        app.update();

        let entity = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(ModuleAdded {
            entity,
            instance_name: "vco_1".to_string(),
            kind: ModuleKind::from("vco"),
        });
        app.update();

        app.world_mut().write_message(ModuleRemoved {
            entity,
            instance_name: "vco_1".to_string(),
            kind: ModuleKind::from("vco"),
        });
        app.update();

        let graph = app.world().resource::<RackGraph>();
        assert_eq!(graph.module_count(), 0);
        assert!(!graph.contains_module(entity));
    }

    #[test]
    fn test_cable_disconnected_removes_edge() {
        let mut app = graph_test_app();
        app.update();

        let mod_a = app.world_mut().spawn_empty().id();
        let mod_b = app.world_mut().spawn_empty().id();
        let port_out = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(mod_a).add_child(port_out);
        let port_in = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(mod_b).add_child(port_in);

        app.world_mut().write_message(ModuleAdded {
            entity: mod_a,
            instance_name: "gen_1".to_string(),
            kind: ModuleKind::from("gen"),
        });
        app.world_mut().write_message(ModuleAdded {
            entity: mod_b,
            instance_name: "sink_1".to_string(),
            kind: ModuleKind::from("sink"),
        });
        app.update();

        let cable = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(CableConnected {
            cable_entity: cable,
            source_port: port_out,
            target_port: port_in,
            transform: None,
        });
        app.update();

        app.world_mut().write_message(CableDisconnected {
            cable_entity: cable,
            source_port: port_out,
            target_port: port_in,
        });
        app.update();

        let graph = app.world().resource::<RackGraph>();
        assert_eq!(graph.cable_count(), 0);
        assert!(!graph.contains_cable(cable));
    }

    #[test]
    fn test_module_removed_cleans_incident_edges() {
        let mut app = graph_test_app();
        app.update();

        let mod_a = app.world_mut().spawn_empty().id();
        let mod_b = app.world_mut().spawn_empty().id();
        let port_out = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(mod_a).add_child(port_out);
        let port_in = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(mod_b).add_child(port_in);

        app.world_mut().write_message(ModuleAdded {
            entity: mod_a,
            instance_name: "gen_1".to_string(),
            kind: ModuleKind::from("gen"),
        });
        app.world_mut().write_message(ModuleAdded {
            entity: mod_b,
            instance_name: "sink_1".to_string(),
            kind: ModuleKind::from("sink"),
        });
        app.update();

        let cable = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(CableConnected {
            cable_entity: cable,
            source_port: port_out,
            target_port: port_in,
            transform: None,
        });
        app.update();

        app.world_mut().write_message(ModuleRemoved {
            entity: mod_a,
            instance_name: "gen_1".to_string(),
            kind: ModuleKind::from("gen"),
        });
        app.update();

        let graph = app.world().resource::<RackGraph>();
        assert_eq!(graph.module_count(), 1);
        assert_eq!(graph.cable_count(), 0);
        assert!(!graph.contains_cable(cable));
    }

    // ── Serde roundtrip tests ─────────────────────────────────────

    #[test]
    fn test_module_info_serde_roundtrip() {
        let info = ModuleInfo {
            entity_bits: 42,
            instance_name: "vco_1".to_string(),
            kind: "vco".to_string(),
            display_name: "VCO 1".to_string(),
            description: Some("Main oscillator".to_string()),
            tags: vec!["audio".to_string(), "lead".to_string()],
        };
        let json = serde_json::to_string(&info).expect("serialize");
        let de: ModuleInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(info, de);
    }

    #[test]
    fn test_cable_info_serde_roundtrip() {
        let info = CableInfo {
            entity_bits: 99,
            source_module_instance: "vco_1".to_string(),
            source_port_name: "out".to_string(),
            target_module_instance: "filter_1".to_string(),
            target_port_name: "in".to_string(),
            display_name: None,
            description: Some("Audio path".to_string()),
            tags: vec![],
        };
        let json = serde_json::to_string(&info).expect("serialize");
        let de: CableInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(info, de);
    }

    #[test]
    fn test_port_info_serde_roundtrip() {
        let info = PortInfo {
            name: "cutoff".to_string(),
            direction: PortDirection::Input,
            value_kind: "Float".to_string(),
        };
        let json = serde_json::to_string(&info).expect("serialize");
        let de: PortInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(info, de);
    }

    // ── Builder tests (#1–#6) ─────────────────────────────────────

    #[test]
    fn test_module_info_resolves_display_name_from_component() {
        let mut app = graph_test_app();
        app.update();
        let e = spawn_module(&mut app, "vco_1", "vco", &[]);
        app.world_mut()
            .entity_mut(e)
            .insert(DisplayName("Lead VCO".to_string()));
        app.update();

        let view = GraphView::new(app.world());
        let info = view.module_by_name("vco_1").expect("should exist");
        assert_eq!(info.display_name, "Lead VCO");
    }

    #[test]
    fn test_module_info_falls_back_to_formatter() {
        let mut app = graph_test_app();
        app.update();
        spawn_module(&mut app, "vco_1", "vco", &[]);
        app.update();

        let view = GraphView::new(app.world());
        let info = view.module_by_name("vco_1").expect("should exist");
        assert_eq!(info.display_name, "VCO 1");
    }

    #[test]
    fn test_module_info_resolves_description() {
        let mut app = graph_test_app();
        app.update();
        let e = spawn_module(&mut app, "vco_1", "vco", &[]);
        app.world_mut()
            .entity_mut(e)
            .insert(Description("the lead voice".to_string()));
        app.update();

        let view = GraphView::new(app.world());
        let with_desc = view.module_by_name("vco_1").unwrap();
        assert_eq!(with_desc.description, Some("the lead voice".to_string()));
    }

    #[test]
    fn test_module_info_description_none_when_absent() {
        let mut app = graph_test_app();
        app.update();
        spawn_module(&mut app, "vco_1", "vco", &[]);
        app.update();

        let view = GraphView::new(app.world());
        let info = view.module_by_name("vco_1").unwrap();
        assert_eq!(info.description, None);
    }

    #[test]
    fn test_module_info_resolves_tags() {
        let mut app = graph_test_app();
        app.update();
        let e = spawn_module(&mut app, "vco_1", "vco", &[]);

        let mut tag_set = BTreeSet::new();
        tag_set.insert(Tag::new("lead").unwrap());
        tag_set.insert(Tag::new("bass").unwrap());
        app.world_mut().entity_mut(e).insert(Tags(tag_set));
        app.update();

        let view = GraphView::new(app.world());
        let info = view.module_by_name("vco_1").unwrap();
        assert_eq!(info.tags, vec!["bass", "lead"]);
    }

    #[test]
    fn test_module_info_no_tags_when_absent() {
        let mut app = graph_test_app();
        app.update();
        spawn_module(&mut app, "vco_1", "vco", &[]);
        app.update();

        let view = GraphView::new(app.world());
        let info = view.module_by_name("vco_1").unwrap();
        assert!(info.tags.is_empty());
    }

    #[test]
    fn test_cable_info_resolves_ports_and_modules() {
        let mut app = graph_test_app();
        app.update();

        let src = spawn_module(
            &mut app,
            "gen_1",
            "gen",
            &[("out", oxurack_core::PortDirection::Output)],
        );
        let sink = spawn_module(
            &mut app,
            "sink_1",
            "sink",
            &[("in", oxurack_core::PortDirection::Input)],
        );
        app.update();

        let port_out = find_port(&mut app, src, "out");
        let port_in = find_port(&mut app, sink, "in");
        let _cable = connect_cable(&mut app, port_out, port_in);
        app.update();

        let view = GraphView::new(app.world());
        let cables = view.cables();
        assert_eq!(cables.len(), 1);
        assert_eq!(cables[0].source_module_instance, "gen_1");
        assert_eq!(cables[0].source_port_name, "out");
        assert_eq!(cables[0].target_module_instance, "sink_1");
        assert_eq!(cables[0].target_port_name, "in");
    }

    // ── Query tests (#7–#13) ──────────────────────────────────────

    #[test]
    fn test_modules_returns_all() {
        let mut app = graph_test_app();
        app.update();

        spawn_module(&mut app, "vco_1", "vco", &[]);
        spawn_module(&mut app, "vco_2", "vco", &[]);
        spawn_module(&mut app, "filter_1", "filter", &[]);
        app.update();

        let view = GraphView::new(app.world());
        let modules = view.modules();
        assert_eq!(modules.len(), 3);
        let names: Vec<&str> = modules.iter().map(|m| m.instance_name.as_str()).collect();
        assert_eq!(names, vec!["filter_1", "vco_1", "vco_2"]);
    }

    #[test]
    fn test_modules_empty_for_empty_graph() {
        let mut app = graph_test_app();
        app.update();

        let view = GraphView::new(app.world());
        assert!(view.modules().is_empty());
    }

    #[test]
    fn test_cables_returns_all() {
        let mut app = graph_test_app();
        app.update();

        let src = spawn_module(
            &mut app,
            "gen_1",
            "gen",
            &[
                ("out_a", oxurack_core::PortDirection::Output),
                ("out_b", oxurack_core::PortDirection::Output),
            ],
        );
        let sink = spawn_module(
            &mut app,
            "sink_1",
            "sink",
            &[
                ("in_a", oxurack_core::PortDirection::Input),
                ("in_b", oxurack_core::PortDirection::Input),
            ],
        );
        app.update();

        let port_out_a = find_port(&mut app, src, "out_a");
        let port_out_b = find_port(&mut app, src, "out_b");
        let port_in_a = find_port(&mut app, sink, "in_a");
        let port_in_b = find_port(&mut app, sink, "in_b");
        connect_cable(&mut app, port_out_a, port_in_a);
        connect_cable(&mut app, port_out_b, port_in_b);
        app.update();

        let view = GraphView::new(app.world());
        assert_eq!(view.cables().len(), 2);
    }

    #[test]
    fn test_module_kinds_histogram() {
        let mut app = graph_test_app();
        app.update();

        spawn_module(&mut app, "vco_1", "vco", &[]);
        spawn_module(&mut app, "vco_2", "vco", &[]);
        spawn_module(&mut app, "filter_1", "filter", &[]);
        app.update();

        let view = GraphView::new(app.world());
        let kinds = view.module_kinds();
        assert_eq!(kinds[&ModuleKind::from("vco")], 2);
        assert_eq!(kinds[&ModuleKind::from("filter")], 1);
    }

    #[test]
    fn test_modules_of_kind_filters() {
        let mut app = graph_test_app();
        app.update();

        spawn_module(&mut app, "vco_1", "vco", &[]);
        spawn_module(&mut app, "vco_2", "vco", &[]);
        spawn_module(&mut app, "filter_1", "filter", &[]);
        app.update();

        let view = GraphView::new(app.world());
        let vcos = view.modules_of_kind(&ModuleKind::from("vco"));
        assert_eq!(vcos.len(), 2);
        assert!(vcos.iter().all(|m| m.kind == "vco"));
    }

    #[test]
    fn test_module_by_name_resolves() {
        let mut app = graph_test_app();
        app.update();

        spawn_module(&mut app, "vco_1", "vco", &[]);
        app.update();

        let view = GraphView::new(app.world());
        let info = view.module_by_name("vco_1").expect("should resolve");
        assert_eq!(info.instance_name, "vco_1");
        assert_eq!(info.kind, "vco");
    }

    #[test]
    fn test_module_by_name_returns_none_for_missing() {
        let mut app = graph_test_app();
        app.update();

        let view = GraphView::new(app.world());
        assert!(view.module_by_name("nonexistent").is_none());
    }

    // ── Ports tests (#14–#15) ─────────────────────────────────────

    #[test]
    fn test_ports_of_returns_inputs_and_outputs() {
        let mut app = graph_test_app();
        app.update();

        let e = spawn_module(
            &mut app,
            "vco_1",
            "vco",
            &[
                ("audio_out", oxurack_core::PortDirection::Output),
                ("pitch_in", oxurack_core::PortDirection::Input),
            ],
        );
        app.update();

        let view = GraphView::new(app.world());
        let ports = view.ports_of(e);
        assert_eq!(ports.inputs.len(), 1);
        assert_eq!(ports.outputs.len(), 1);
        assert_eq!(ports.inputs[0].name, "pitch_in");
        assert_eq!(ports.outputs[0].name, "audio_out");
        assert_eq!(ports.inputs[0].direction, PortDirection::Input);
        assert_eq!(ports.outputs[0].direction, PortDirection::Output);
    }

    #[test]
    fn test_ports_of_returns_empty_for_unknown_module() {
        let mut app = graph_test_app();
        app.update();

        let unknown = app.world_mut().spawn_empty().id();

        let view = GraphView::new(app.world());
        let ports = view.ports_of(unknown);
        assert!(ports.inputs.is_empty());
        assert!(ports.outputs.is_empty());
    }

    // ── Neighborhood tests (#16–#20) ──────────────────────────────

    #[test]
    fn test_upstream_of_returns_source_modules() {
        let mut app = graph_test_app();
        app.update();

        let vco = spawn_module(
            &mut app,
            "vco_1",
            "vco",
            &[("out", oxurack_core::PortDirection::Output)],
        );
        let filter = spawn_module(
            &mut app,
            "filter_1",
            "filter",
            &[
                ("in", oxurack_core::PortDirection::Input),
                ("out", oxurack_core::PortDirection::Output),
            ],
        );
        let output = spawn_module(
            &mut app,
            "out_1",
            "output",
            &[("in", oxurack_core::PortDirection::Input)],
        );
        app.update();

        let vco_out = find_port(&mut app, vco, "out");
        let filter_in = find_port(&mut app, filter, "in");
        let filter_out = find_port(&mut app, filter, "out");
        let output_in = find_port(&mut app, output, "in");
        connect_cable(&mut app, vco_out, filter_in);
        connect_cable(&mut app, filter_out, output_in);
        app.update();

        let view = GraphView::new(app.world());

        let ups_filter = view.upstream_of(filter);
        assert_eq!(ups_filter.len(), 1);
        assert_eq!(ups_filter[0].instance_name, "vco_1");

        let ups_output = view.upstream_of(output);
        assert_eq!(ups_output.len(), 1);
        assert_eq!(ups_output[0].instance_name, "filter_1");

        assert!(view.upstream_of(vco).is_empty());
    }

    #[test]
    fn test_downstream_of_returns_target_modules() {
        let mut app = graph_test_app();
        app.update();

        let vco = spawn_module(
            &mut app,
            "vco_1",
            "vco",
            &[("out", oxurack_core::PortDirection::Output)],
        );
        let filter = spawn_module(
            &mut app,
            "filter_1",
            "filter",
            &[
                ("in", oxurack_core::PortDirection::Input),
                ("out", oxurack_core::PortDirection::Output),
            ],
        );
        let output = spawn_module(
            &mut app,
            "out_1",
            "output",
            &[("in", oxurack_core::PortDirection::Input)],
        );
        app.update();

        let vco_out = find_port(&mut app, vco, "out");
        let filter_in = find_port(&mut app, filter, "in");
        let filter_out = find_port(&mut app, filter, "out");
        let output_in = find_port(&mut app, output, "in");
        connect_cable(&mut app, vco_out, filter_in);
        connect_cable(&mut app, filter_out, output_in);
        app.update();

        let view = GraphView::new(app.world());

        let down_vco = view.downstream_of(vco);
        assert_eq!(down_vco.len(), 1);
        assert_eq!(down_vco[0].instance_name, "filter_1");

        assert!(view.downstream_of(output).is_empty());
    }

    #[test]
    fn test_incoming_cables() {
        let mut app = graph_test_app();
        app.update();

        let gen_a = spawn_module(
            &mut app,
            "gen_a",
            "gen",
            &[("out", oxurack_core::PortDirection::Output)],
        );
        let gen_b = spawn_module(
            &mut app,
            "gen_b",
            "gen",
            &[("out", oxurack_core::PortDirection::Output)],
        );
        let sink = spawn_module(
            &mut app,
            "sink_1",
            "sink",
            &[
                ("in_a", oxurack_core::PortDirection::Input),
                ("in_b", oxurack_core::PortDirection::Input),
            ],
        );
        app.update();

        let ga_out = find_port(&mut app, gen_a, "out");
        let gb_out = find_port(&mut app, gen_b, "out");
        let s_in_a = find_port(&mut app, sink, "in_a");
        let s_in_b = find_port(&mut app, sink, "in_b");
        connect_cable(&mut app, ga_out, s_in_a);
        connect_cable(&mut app, gb_out, s_in_b);
        app.update();

        let view = GraphView::new(app.world());
        assert_eq!(view.incoming_cables(sink).len(), 2);
    }

    #[test]
    fn test_outgoing_cables() {
        let mut app = graph_test_app();
        app.update();

        let src = spawn_module(
            &mut app,
            "gen_1",
            "gen",
            &[
                ("out_a", oxurack_core::PortDirection::Output),
                ("out_b", oxurack_core::PortDirection::Output),
            ],
        );
        let sink_a = spawn_module(
            &mut app,
            "sink_a",
            "sink",
            &[("in", oxurack_core::PortDirection::Input)],
        );
        let sink_b = spawn_module(
            &mut app,
            "sink_b",
            "sink",
            &[("in", oxurack_core::PortDirection::Input)],
        );
        app.update();

        let g_out_a = find_port(&mut app, src, "out_a");
        let g_out_b = find_port(&mut app, src, "out_b");
        let sa_in = find_port(&mut app, sink_a, "in");
        let sb_in = find_port(&mut app, sink_b, "in");
        connect_cable(&mut app, g_out_a, sa_in);
        connect_cable(&mut app, g_out_b, sb_in);
        app.update();

        let view = GraphView::new(app.world());
        assert_eq!(view.outgoing_cables(src).len(), 2);
    }

    #[test]
    fn test_neighborhoods_empty_for_isolated_module() {
        let mut app = graph_test_app();
        app.update();

        let e = spawn_module(&mut app, "lonely_1", "lonely", &[]);
        app.update();

        let view = GraphView::new(app.world());
        assert!(view.upstream_of(e).is_empty());
        assert!(view.downstream_of(e).is_empty());
        assert!(view.incoming_cables(e).is_empty());
        assert!(view.outgoing_cables(e).is_empty());
    }

    // ── Ordering test (#25) ───────────────────────────────────────

    #[test]
    fn test_modules_sorted_by_instance_name() {
        let mut app = graph_test_app();
        app.update();

        spawn_module(&mut app, "zebra_1", "zebra", &[]);
        spawn_module(&mut app, "alpha_1", "alpha", &[]);
        spawn_module(&mut app, "middle_1", "middle", &[]);
        app.update();

        let view = GraphView::new(app.world());
        let modules = view.modules();
        let names: Vec<&str> = modules.iter().map(|m| m.instance_name.as_str()).collect();
        assert_eq!(names, vec!["alpha_1", "middle_1", "zebra_1"]);
    }
}
