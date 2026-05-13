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
//! fetched fresh from the ECS world at query time. This avoids
//! label-staleness bugs.
//!
//! # Feature gate
//!
//! This crate is pulled into the umbrella `oxurack` crate behind the
//! `graph` Cargo feature (enabled by default).

use std::collections::HashMap;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

use oxurack_core::{
    CableConnected, CableDisconnected, CableTransform, ModuleAdded, ModuleKind, ModuleRemoved,
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
/// fresh from the ECS world via the query layer. Avoids label-staleness.
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

// ── Result types (declared for part 2; serde-enabled) ──────────────────

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
/// Systems run in the [`Update`] schedule, chained so that module
/// additions are processed before cable connections within the same
/// tick.
pub struct RackGraphPlugin;

impl Plugin for RackGraphPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RackGraph>().add_systems(
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
            .map(|c| c.0)
            .expect("source port must have a ChildOf parent");
        let target_module = children
            .get(msg.target_port)
            .map(|c| c.0)
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
    use pretty_assertions::assert_eq;

    fn graph_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(oxurack_core::CorePlugin);
        app.add_plugins(RackGraphPlugin);
        app
    }

    // ── Test #1: empty on init ─────────────────────────────────────

    #[test]
    fn test_rack_graph_empty_on_init() {
        let graph = RackGraph::default();
        assert_eq!(graph.module_count(), 0);
        assert_eq!(graph.cable_count(), 0);
    }

    // ── Test #2: module added adds node ────────────────────────────

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

    // ── Test #3: two modules added ─────────────────────────────────

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
        assert_eq!(graph.entity_by_instance_name("vco_1"), Some(e1));
        assert_eq!(graph.entity_by_instance_name("filter_1"), Some(e2));
    }

    // ── Test #4: cable connected adds edge ─────────────────────────

    #[test]
    fn test_cable_connected_adds_edge() {
        let mut app = graph_test_app();
        app.update();

        // Spawn two module entities.
        let mod_a = app.world_mut().spawn_empty().id();
        let mod_b = app.world_mut().spawn_empty().id();

        // Spawn port entities as children of their modules.
        let port_out = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(mod_a).add_child(port_out);

        let port_in = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(mod_b).add_child(port_in);

        // Emit ModuleAdded for both.
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

        // Process module additions.
        app.update();

        // Now emit CableConnected.
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

    // ── Test #5: module removed removes node ───────────────────────

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

        assert_eq!(app.world().resource::<RackGraph>().module_count(), 1);

        app.world_mut().write_message(ModuleRemoved {
            entity,
            instance_name: "vco_1".to_string(),
            kind: ModuleKind::from("vco"),
        });
        app.update();

        let graph = app.world().resource::<RackGraph>();
        assert_eq!(graph.module_count(), 0);
        assert!(!graph.contains_module(entity));
        assert_eq!(graph.entity_by_instance_name("vco_1"), None);
    }

    // ── Test #6: cable disconnected removes edge ───────────────────

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

        assert_eq!(app.world().resource::<RackGraph>().cable_count(), 1);

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

    // ── Anti-softpedal: module removed cleans incident edges ───────

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

        assert_eq!(app.world().resource::<RackGraph>().cable_count(), 1);
        assert!(app.world().resource::<RackGraph>().contains_cable(cable));

        // Remove mod_a WITHOUT emitting CableDisconnected.
        // The graph must clean up the incident edge.
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

    // ── Test #7: ModuleInfo serde roundtrip ────────────────────────

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
        let deserialized: ModuleInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(info, deserialized);
    }

    // ── Test #8: CableInfo serde roundtrip ─────────────────────────

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
        let deserialized: CableInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(info, deserialized);
    }

    // ── Test #9: PortInfo serde roundtrip ──────────────────────────

    #[test]
    fn test_port_info_serde_roundtrip() {
        let info = PortInfo {
            name: "cutoff".to_string(),
            direction: PortDirection::Input,
            value_kind: "Float".to_string(),
        };
        let json = serde_json::to_string(&info).expect("serialize");
        let deserialized: PortInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(info, deserialized);
    }
}
