//! Frame-tick scheduling phases, merge buffers, and ordering.
//!
//! [`TickPhase`] defines the three-phase ordering for each tick of
//! the modular rack:
//!
//! 1. **Produce** -- modules generate output values.
//! 2. **Propagate** -- cables carry values from outputs to inputs.
//! 3. **Consume** -- modules read their input ports.
//!
//! This module also provides:
//!
//! - [`MergeBuffers`] -- per-tick accumulator for values arriving at
//!   input ports via cables.
//! - [`TickOrder`] -- cached topological ordering of module entities
//!   for the Produce phase.
//! - [`TickNow`] -- event signalling that a tick should execute.
//! - `propagate_cables_system` -- propagates values through cables.
//! - `consume_ports_system` -- merges buffered values into input ports.
//! - [`compute_tick_order`] -- topological sort of the module graph.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use bevy_ecs::prelude::{Entity, Message, Query, Res, ResMut, Resource};
use bevy_ecs::schedule::SystemSet;
use smallvec::SmallVec;

/// System set labels for the three phases of a rack tick.
///
/// Configured as a strict chain (`Produce -> Propagate -> Consume`)
/// inside the [`Update`](bevy_app::Update) schedule by [`CorePlugin`](crate::CorePlugin).
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TickPhase {
    /// Modules write to their output ports.
    Produce,
    /// Cables propagate values from output ports to input ports.
    Propagate,
    /// Modules read from their input ports.
    Consume,
}

// ── MergeBuffers ──────────────────────────────────────────────────

/// Per-tick accumulator for values arriving at input ports via cables.
///
/// During the Propagate phase, each cable contributes a transformed
/// value to its target port's buffer. During the Consume phase, these
/// contributions are merged according to the port's [`MergePolicy`](crate::MergePolicy).
#[derive(Resource, Default, Debug)]
pub struct MergeBuffers {
    buffers: HashMap<Entity, SmallVec<[crate::Value; 4]>>,
}

impl MergeBuffers {
    /// Returns the contributions accumulated for the given port entity.
    ///
    /// Returns an empty slice if no values have been contributed.
    pub fn contributions(&self, port: Entity) -> &[crate::Value] {
        self.buffers.get(&port).map_or(&[], |v| v.as_slice())
    }

    /// Adds a value contribution to the given port entity's buffer.
    pub fn contribute(&mut self, port: Entity, value: crate::Value) {
        self.buffers.entry(port).or_default().push(value);
    }

    /// Clears all contribution buffers, retaining allocated memory.
    pub fn clear(&mut self) {
        for buffer in self.buffers.values_mut() {
            buffer.clear();
        }
    }
}

// ── TickOrder ─────────────────────────────────────────────────────

/// Cached topological ordering of module entities for the Produce phase.
///
/// Updated whenever the patch graph changes. Modules are processed in
/// this order so that upstream modules produce their outputs before
/// downstream modules read their inputs.
#[derive(Resource, Default, Debug)]
pub struct TickOrder {
    /// The ordered list of module entities to process.
    pub order: Vec<Entity>,
}

// ── TickNow ───────────────────────────────────────────────────────

/// Message signalling that a tick should execute.
///
/// Sent each frame by the scheduling layer. The `frame` counter
/// provides a monotonically increasing tick identifier.
#[derive(Message, Debug, Clone, Copy)]
pub struct TickNow {
    /// The frame number for this tick.
    pub frame: u64,
}

// ── PropagationOrder ─────────────────────────────────────────────

/// Cached ordering of cable entities for deterministic propagation.
///
/// Cables are ordered by (target_module_id, target_port_name,
/// source_module_id, source_port_name) so that propagation is
/// deterministic across runs even if entity IDs change.
#[derive(Resource, Default, Debug)]
pub struct PropagationOrder {
    /// Ordered list of cable entities to process during propagation.
    pub cables: Vec<Entity>,
}

/// Rebuilds the propagation order from the current cable graph.
///
/// Orders cables by (target_module_id, target_port_name,
/// source_module_id, source_port_name) for cross-run determinism.
fn rebuild_propagation_order(
    cable_q: &Query<(Entity, &crate::Cable)>,
    port_q: &Query<(&crate::Port, &bevy_ecs::hierarchy::ChildOf)>,
    module_q: &Query<&crate::ModuleId>,
) -> Vec<Entity> {
    let mut entries: Vec<(Entity, crate::ModuleId, String, crate::ModuleId, String)> = cable_q
        .iter()
        .filter_map(|(cable_entity, cable)| {
            let (target_port, target_child_of) = port_q.get(cable.target_port).ok()?;
            let target_module_id = *module_q.get(target_child_of.0).ok()?;
            let target_port_name = target_port.name.as_ref().to_string();

            let (source_port, source_child_of) = port_q.get(cable.source_port).ok()?;
            let source_module_id = *module_q.get(source_child_of.0).ok()?;
            let source_port_name = source_port.name.as_ref().to_string();

            Some((
                cable_entity,
                target_module_id,
                target_port_name,
                source_module_id,
                source_port_name,
            ))
        })
        .collect();

    entries.sort_by(|a, b| {
        a.1.cmp(&b.1)
            .then_with(|| a.2.cmp(&b.2))
            .then_with(|| a.3.cmp(&b.3))
            .then_with(|| a.4.cmp(&b.4))
    });

    entries.into_iter().map(|(e, ..)| e).collect()
}

// ── propagate_cables_system ───────────────────────────────────────

/// Propagates values through all enabled cables.
///
/// Runs in [`TickPhase::Propagate`]. For each cable (in stable order
/// via [`PropagationOrder`]):
///
/// 1. Read the source port's [`CurrentValue`](crate::CurrentValue).
/// 2. Apply the cable's [`CableTransform`](crate::CableTransform) (if any).
/// 3. Contribute the result to the target port's [`MergeBuffers`] entry.
///
/// The propagation order is rebuilt each tick from the cable graph for
/// cross-run determinism. The cost is O(C log C) where C is the cable
/// count, which is fast for small C.
pub(crate) fn propagate_cables_system(
    cable_q: Query<(Entity, &crate::Cable)>,
    port_q: Query<(&crate::Port, &bevy_ecs::hierarchy::ChildOf)>,
    module_q: Query<&crate::ModuleId>,
    port_values: Query<&crate::CurrentValue>,
    mut merge_buffers: ResMut<MergeBuffers>,
) {
    // Clear buffers from previous tick.
    merge_buffers.clear();

    // Rebuild propagation order each tick for determinism.
    let ordered_cables = rebuild_propagation_order(&cable_q, &port_q, &module_q);

    for cable_entity in ordered_cables {
        let Ok((_, cable)) = cable_q.get(cable_entity) else {
            continue;
        };
        if !cable.enabled {
            continue;
        }

        let Ok(source_value) = port_values.get(cable.source_port) else {
            continue;
        };

        // Apply transform if present.
        let value = if let Some(ref transform) = cable.transform {
            match transform.apply(source_value.0) {
                Some(v) => v,
                None => continue, // incompatible transform, skip silently
            }
        } else {
            source_value.0
        };

        merge_buffers.contribute(cable.target_port, value);
    }
}

// ── consume_ports_system ──────────────────────────────────────────

/// Consumes merge buffers and writes merged values to input ports.
///
/// Runs in [`TickPhase::Consume`]. For each input port with one or
/// more contributions: applies the port's [`MergePolicy`](crate::MergePolicy)
/// across the contributions and writes the result to
/// [`CurrentValue`](crate::CurrentValue). Ports with zero
/// contributions retain their previous value (sample-and-hold).
pub(crate) fn consume_ports_system(
    mut port_q: Query<(Entity, &crate::Port, &mut crate::CurrentValue)>,
    merge_buffers: Res<MergeBuffers>,
) {
    for (entity, port, mut current) in port_q.iter_mut() {
        if port.direction != crate::PortDirection::Input {
            continue;
        }

        let contributions = merge_buffers.contributions(entity);
        if contributions.is_empty() {
            continue; // sample-and-hold: keep previous value
        }

        current.0 = apply_merge(port.merge_policy, contributions);
    }
}

/// Applies a merge policy to a non-empty slice of values.
///
/// # Panics
///
/// Panics if `values` is empty.
pub(crate) fn apply_merge(policy: crate::MergePolicy, values: &[crate::Value]) -> crate::Value {
    debug_assert!(!values.is_empty(), "apply_merge called with empty slice");

    match policy {
        crate::MergePolicy::Reject => {
            // Validation ensures at most one cable at load time.
            values[0]
        }

        crate::MergePolicy::LastWins => *values.last().expect("non-empty"),

        crate::MergePolicy::Average => match values[0] {
            crate::Value::Float(_) => {
                let sum: f32 = values
                    .iter()
                    .map(|v| match v {
                        crate::Value::Float(f) => *f,
                        _ => 0.0,
                    })
                    .sum();
                crate::Value::Float(sum / values.len() as f32)
            }
            crate::Value::Bipolar(_) => {
                let sum: f32 = values
                    .iter()
                    .map(|v| match v {
                        crate::Value::Bipolar(f) => *f,
                        _ => 0.0,
                    })
                    .sum();
                crate::Value::Bipolar(sum / values.len() as f32)
            }
            other => other, // shouldn't happen if validated at load time
        },

        crate::MergePolicy::Sum => match values[0] {
            crate::Value::Float(_) => {
                let sum: f32 = values
                    .iter()
                    .map(|v| match v {
                        crate::Value::Float(f) => *f,
                        _ => 0.0,
                    })
                    .sum();
                crate::Value::Float(sum.clamp(0.0, 1.0))
            }
            crate::Value::Bipolar(_) => {
                let sum: f32 = values
                    .iter()
                    .map(|v| match v {
                        crate::Value::Bipolar(f) => *f,
                        _ => 0.0,
                    })
                    .sum();
                crate::Value::Bipolar(sum.clamp(-1.0, 1.0))
            }
            crate::Value::Gate(_) => {
                // OR semantics: true if any contribution is true.
                let any_true = values.iter().any(|v| matches!(v, crate::Value::Gate(true)));
                crate::Value::Gate(any_true)
            }
            other => other,
        },

        crate::MergePolicy::Max => match values[0] {
            crate::Value::Float(_) => {
                let max = values
                    .iter()
                    .filter_map(|v| match v {
                        crate::Value::Float(f) => Some(*f),
                        _ => None,
                    })
                    .fold(f32::NEG_INFINITY, f32::max);
                crate::Value::Float(max)
            }
            crate::Value::Bipolar(_) => {
                let max = values
                    .iter()
                    .filter_map(|v| match v {
                        crate::Value::Bipolar(f) => Some(*f),
                        _ => None,
                    })
                    .fold(f32::NEG_INFINITY, f32::max);
                crate::Value::Bipolar(max)
            }
            crate::Value::Gate(_) => {
                let any_true = values.iter().any(|v| matches!(v, crate::Value::Gate(true)));
                crate::Value::Gate(any_true)
            }
            other => other,
        },
    }
}

// ── compute_tick_order ────────────────────────────────────────────

/// Computes a topological ordering of modules based on cable connections.
///
/// Returns `Err(PatchError::FeedbackCycle)` if a cycle is detected.
/// Tie-breaks within the same topological level by
/// [`ModuleId`](crate::ModuleId) (ascending).
///
/// # Arguments
///
/// * `modules` -- `(Entity, &Module, &ModuleId)` tuples for all modules.
/// * `cables` -- `(Entity, &Cable)` tuples for all cables.
/// * `ports` -- `(Entity, &Port, Entity)` tuples mapping port entities to
///   their parent module entities.
pub fn compute_tick_order(
    modules: &[(Entity, &crate::Module, &crate::ModuleId)],
    cables: &[(Entity, &crate::Cable)],
    ports: &[(Entity, &crate::Port, Entity)], // (port_entity, port, parent_module_entity)
) -> Result<Vec<Entity>, crate::PatchError> {
    // Build port-to-module mapping.
    let port_to_module: HashMap<Entity, Entity> = ports
        .iter()
        .map(|(port_entity, _, module_entity)| (*port_entity, *module_entity))
        .collect();

    // Build adjacency list: source_module -> [target_modules]
    let module_set: HashSet<Entity> = modules.iter().map(|(e, _, _)| *e).collect();
    let mut in_degree: HashMap<Entity, usize> = module_set.iter().map(|&e| (e, 0)).collect();
    let mut dependents: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (_, cable) in cables {
        let Some(&source_module) = port_to_module.get(&cable.source_port) else {
            continue;
        };
        let Some(&target_module) = port_to_module.get(&cable.target_port) else {
            continue;
        };
        if source_module == target_module {
            continue; // self-loop within same module, skip
        }

        dependents
            .entry(source_module)
            .or_default()
            .push(target_module);
        *in_degree.entry(target_module).or_insert(0) += 1;
    }

    // Map entity -> ModuleId for tie-breaking.
    let module_ids: HashMap<Entity, crate::ModuleId> = modules
        .iter()
        .map(|(e, _, id)| (*e, **id))
        .collect();

    // Kahn's algorithm with ModuleId tie-breaking via BinaryHeap<Reverse<...>>.
    let mut queue: BinaryHeap<Reverse<(crate::ModuleId, Entity)>> = in_degree
        .iter()
        .filter(|(_, deg)| **deg == 0)
        .map(|(entity, _)| Reverse((module_ids[entity], *entity)))
        .collect();

    let mut result = Vec::with_capacity(modules.len());

    while let Some(Reverse((_, entity))) = queue.pop() {
        result.push(entity);
        if let Some(deps) = dependents.get(&entity) {
            for &dep in deps {
                let deg = in_degree.get_mut(&dep).expect("known module");
                *deg -= 1;
                if *deg == 0 {
                    queue.push(Reverse((module_ids[&dep], dep)));
                }
            }
        }
    }

    if result.len() != modules.len() {
        // Cycle detected -- use Tarjan's SCC to find modules that are
        // actually in cycles (SCCs of size > 1), not just downstream.
        let resolved: HashSet<Entity> = result.iter().copied().collect();
        let unresolved: HashSet<Entity> = module_set
            .iter()
            .filter(|e| !resolved.contains(e))
            .copied()
            .collect();

        let module_names: HashMap<Entity, String> = modules
            .iter()
            .map(|(e, m, _)| (*e, m.instance_name.clone()))
            .collect();

        let cycle_members =
            find_cycle_members(&unresolved, &dependents, &module_names);
        return Err(crate::PatchError::FeedbackCycle(cycle_members));
    }

    Ok(result)
}

/// Uses Tarjan's SCC algorithm on the unresolved subgraph to find
/// entities that are actually in cycles (SCCs of size > 1).
/// Returns sorted instance names of those modules.
fn find_cycle_members(
    unresolved: &HashSet<Entity>,
    dependents: &HashMap<Entity, Vec<Entity>>,
    module_names: &HashMap<Entity, String>,
) -> Vec<String> {
    struct TarjanState<'a> {
        unresolved: &'a HashSet<Entity>,
        dependents: &'a HashMap<Entity, Vec<Entity>>,
        index_counter: usize,
        stack: Vec<Entity>,
        on_stack: HashSet<Entity>,
        indices: HashMap<Entity, usize>,
        lowlinks: HashMap<Entity, usize>,
        sccs: Vec<Vec<Entity>>,
    }

    impl TarjanState<'_> {
        fn visit(&mut self, node: Entity) {
            self.indices.insert(node, self.index_counter);
            self.lowlinks.insert(node, self.index_counter);
            self.index_counter += 1;
            self.stack.push(node);
            self.on_stack.insert(node);

            if let Some(deps) = self.dependents.get(&node) {
                for &dep in deps {
                    if !self.unresolved.contains(&dep) {
                        continue;
                    }
                    if !self.indices.contains_key(&dep) {
                        self.visit(dep);
                        let dep_low = self.lowlinks[&dep];
                        let node_low = self.lowlinks.get_mut(&node).expect("just inserted");
                        *node_low = (*node_low).min(dep_low);
                    } else if self.on_stack.contains(&dep) {
                        let dep_idx = self.indices[&dep];
                        let node_low = self.lowlinks.get_mut(&node).expect("just inserted");
                        *node_low = (*node_low).min(dep_idx);
                    }
                }
            }

            if self.lowlinks[&node] == self.indices[&node] {
                let mut scc = Vec::new();
                loop {
                    let w = self.stack.pop().expect("stack non-empty");
                    self.on_stack.remove(&w);
                    scc.push(w);
                    if w == node {
                        break;
                    }
                }
                self.sccs.push(scc);
            }
        }
    }

    let mut state = TarjanState {
        unresolved,
        dependents,
        index_counter: 0,
        stack: Vec::new(),
        on_stack: HashSet::new(),
        indices: HashMap::new(),
        lowlinks: HashMap::new(),
        sccs: Vec::new(),
    };

    let mut sorted_unresolved: Vec<Entity> = unresolved.iter().copied().collect();
    sorted_unresolved.sort();
    for node in sorted_unresolved {
        if !state.indices.contains_key(&node) {
            state.visit(node);
        }
    }

    let mut result: Vec<String> = state
        .sccs
        .iter()
        .filter(|scc| scc.len() > 1)
        .flatten()
        .filter_map(|e| module_names.get(e).cloned())
        .collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Cable, CurrentValue, MergePolicy, Module, ModuleId, Port, PortDirection, Value, ValueKind,
    };
    use bevy_ecs::world::World;
    use pretty_assertions::assert_eq;

    // ── MergeBuffers unit tests ───────────────────────────────────

    #[test]
    fn test_merge_buffers_contribute_and_read() {
        let mut world = World::new();
        let port = world.spawn_empty().id();

        let mut bufs = MergeBuffers::default();
        bufs.contribute(port, Value::Float(0.25));
        bufs.contribute(port, Value::Float(0.75));

        let contributions = bufs.contributions(port);
        assert_eq!(contributions.len(), 2);
        assert_eq!(contributions[0], Value::Float(0.25));
        assert_eq!(contributions[1], Value::Float(0.75));
    }

    #[test]
    fn test_merge_buffers_clear() {
        let mut world = World::new();
        let port = world.spawn_empty().id();

        let mut bufs = MergeBuffers::default();
        bufs.contribute(port, Value::Float(0.5));
        assert_eq!(bufs.contributions(port).len(), 1);

        bufs.clear();
        assert!(bufs.contributions(port).is_empty());
    }

    #[test]
    fn test_merge_buffers_empty_port() {
        let mut world = World::new();
        let unknown = world.spawn_empty().id();

        let bufs = MergeBuffers::default();
        assert!(bufs.contributions(unknown).is_empty());
    }

    // ── apply_merge unit tests ────────────────────────────────────

    #[test]
    fn test_apply_merge_reject_single() {
        let result = apply_merge(MergePolicy::Reject, &[Value::Float(0.5)]);
        assert_eq!(result, Value::Float(0.5));
    }

    #[test]
    fn test_apply_merge_last_wins() {
        let result = apply_merge(
            MergePolicy::LastWins,
            &[Value::Float(0.1), Value::Float(0.2), Value::Float(0.3)],
        );
        assert_eq!(result, Value::Float(0.3));
    }

    #[test]
    fn test_apply_merge_average_float() {
        let result = apply_merge(
            MergePolicy::Average,
            &[Value::Float(0.25), Value::Float(0.75)],
        );
        assert_eq!(result, Value::Float(0.5));
    }

    #[test]
    fn test_apply_merge_average_bipolar() {
        let result = apply_merge(
            MergePolicy::Average,
            &[Value::Bipolar(-0.5), Value::Bipolar(0.5)],
        );
        assert_eq!(result, Value::Bipolar(0.0));
    }

    #[test]
    fn test_apply_merge_sum_float_clamped() {
        let result = apply_merge(
            MergePolicy::Sum,
            &[Value::Float(0.75), Value::Float(0.75)],
        );
        assert_eq!(result, Value::Float(1.0));
    }

    #[test]
    fn test_apply_merge_sum_bipolar_clamped() {
        let result = apply_merge(
            MergePolicy::Sum,
            &[Value::Bipolar(0.75), Value::Bipolar(0.75)],
        );
        assert_eq!(result, Value::Bipolar(1.0));
    }

    #[test]
    fn test_apply_merge_sum_gate_or() {
        let result = apply_merge(
            MergePolicy::Sum,
            &[Value::Gate(false), Value::Gate(true)],
        );
        assert_eq!(result, Value::Gate(true));
    }

    #[test]
    fn test_apply_merge_max_float() {
        let result = apply_merge(
            MergePolicy::Max,
            &[Value::Float(0.25), Value::Float(0.75)],
        );
        assert_eq!(result, Value::Float(0.75));
    }

    #[test]
    fn test_apply_merge_max_gate() {
        let result = apply_merge(
            MergePolicy::Max,
            &[Value::Gate(false), Value::Gate(false)],
        );
        assert_eq!(result, Value::Gate(false));
    }

    // ── TickPhase tests (preserved from Phase 2) ──────────────────

    #[test]
    fn test_tick_phase_eq() {
        assert_eq!(TickPhase::Produce, TickPhase::Produce);
        assert_ne!(TickPhase::Produce, TickPhase::Propagate);
        assert_ne!(TickPhase::Propagate, TickPhase::Consume);
    }

    #[test]
    fn test_tick_phase_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(TickPhase::Produce);
        set.insert(TickPhase::Propagate);
        set.insert(TickPhase::Consume);
        set.insert(TickPhase::Produce); // duplicate
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_tick_phase_clone() {
        let phase = TickPhase::Propagate;
        let cloned = phase.clone();
        assert_eq!(phase, cloned);
    }

    // ── TickNow tests ─────────────────────────────────────────────

    #[test]
    fn test_tick_now_clone_copy() {
        let tick = TickNow { frame: 100 };
        let copied = tick;
        assert_eq!(tick.frame, copied.frame);
    }

    // ── compute_tick_order tests ──────────────────────────────────

    /// Helper: spawn a module entity directly in a World and return
    /// `(Entity, Module ref data, ModuleId)` for use with `compute_tick_order`.
    fn spawn_test_module(world: &mut World, kind: &str, name: &str) -> Entity {
        crate::spawn_module_entity(world, kind, name)
    }

    /// Helper: spawn a port entity as a child of a module.
    fn spawn_test_port(
        world: &mut World,
        module: Entity,
        name: &str,
        direction: PortDirection,
    ) -> Entity {
        crate::spawn_port_on_module(
            world,
            module,
            name,
            direction,
            ValueKind::Float,
            MergePolicy::LastWins,
        )
    }

    /// Helper: collect modules data from the world into the tuple format
    /// required by `compute_tick_order`.
    fn collect_modules(world: &World, entities: &[Entity]) -> Vec<(Entity, Module, ModuleId)> {
        entities
            .iter()
            .map(|&e| {
                let module = world.entity(e).get::<Module>().unwrap().clone();
                let id = *world.entity(e).get::<ModuleId>().unwrap();
                (e, module, id)
            })
            .collect()
    }

    #[test]
    fn test_topo_linear_chain() {
        let mut world = World::new();

        let mod_a = spawn_test_module(&mut world, "gen", "a");
        let mod_b = spawn_test_module(&mut world, "gen", "b");
        let mod_c = spawn_test_module(&mut world, "gen", "c");

        let out_a = spawn_test_port(&mut world, mod_a, "out", PortDirection::Output);
        let in_b = spawn_test_port(&mut world, mod_b, "in", PortDirection::Input);
        let out_b = spawn_test_port(&mut world, mod_b, "out", PortDirection::Output);
        let in_c = spawn_test_port(&mut world, mod_c, "in", PortDirection::Input);

        let cable_ab = world
            .spawn(Cable {
                source_port: out_a,
                target_port: in_b,
                transform: None,
                enabled: true,
            })
            .id();
        let cable_bc = world
            .spawn(Cable {
                source_port: out_b,
                target_port: in_c,
                transform: None,
                enabled: true,
            })
            .id();

        let modules_data = collect_modules(&world, &[mod_a, mod_b, mod_c]);
        let modules_ref: Vec<(Entity, &Module, &ModuleId)> = modules_data
            .iter()
            .map(|(e, m, id)| (*e, m, id))
            .collect();

        let cable_ab_comp = world.entity(cable_ab).get::<Cable>().unwrap().clone();
        let cable_bc_comp = world.entity(cable_bc).get::<Cable>().unwrap().clone();
        let cables: Vec<(Entity, Cable)> =
            vec![(cable_ab, cable_ab_comp), (cable_bc, cable_bc_comp)];
        let cables_ref: Vec<(Entity, &Cable)> =
            cables.iter().map(|(e, c)| (*e, c)).collect();

        let ports: Vec<(Entity, Port, Entity)> = vec![
            (
                out_a,
                world.entity(out_a).get::<Port>().unwrap().clone(),
                mod_a,
            ),
            (
                in_b,
                world.entity(in_b).get::<Port>().unwrap().clone(),
                mod_b,
            ),
            (
                out_b,
                world.entity(out_b).get::<Port>().unwrap().clone(),
                mod_b,
            ),
            (
                in_c,
                world.entity(in_c).get::<Port>().unwrap().clone(),
                mod_c,
            ),
        ];
        let ports_ref: Vec<(Entity, &Port, Entity)> =
            ports.iter().map(|(e, p, m)| (*e, p, *m)).collect();

        let order = compute_tick_order(&modules_ref, &cables_ref, &ports_ref).unwrap();

        // A must come before B, B must come before C.
        let pos_a = order.iter().position(|&e| e == mod_a).unwrap();
        let pos_b = order.iter().position(|&e| e == mod_b).unwrap();
        let pos_c = order.iter().position(|&e| e == mod_c).unwrap();
        assert!(pos_a < pos_b, "A should come before B");
        assert!(pos_b < pos_c, "B should come before C");
    }

    #[test]
    fn test_topo_diamond() {
        let mut world = World::new();

        let mod_a = spawn_test_module(&mut world, "gen", "a");
        let mod_b = spawn_test_module(&mut world, "gen", "b");
        let mod_c = spawn_test_module(&mut world, "gen", "c");
        let mod_d = spawn_test_module(&mut world, "gen", "d");

        let out_a = spawn_test_port(&mut world, mod_a, "out", PortDirection::Output);
        let in_b = spawn_test_port(&mut world, mod_b, "in", PortDirection::Input);
        let out_b = spawn_test_port(&mut world, mod_b, "out", PortDirection::Output);
        let in_c = spawn_test_port(&mut world, mod_c, "in", PortDirection::Input);
        let out_c = spawn_test_port(&mut world, mod_c, "out", PortDirection::Output);
        let in_d1 = spawn_test_port(&mut world, mod_d, "in1", PortDirection::Input);
        let in_d2 = spawn_test_port(&mut world, mod_d, "in2", PortDirection::Input);

        // A -> B, A -> C, B -> D, C -> D
        let cables_raw: Vec<(Entity, Cable)> = vec![
            (
                world
                    .spawn(Cable {
                        source_port: out_a,
                        target_port: in_b,
                        transform: None,
                        enabled: true,
                    })
                    .id(),
                Cable {
                    source_port: out_a,
                    target_port: in_b,
                    transform: None,
                    enabled: true,
                },
            ),
            (
                world
                    .spawn(Cable {
                        source_port: out_a,
                        target_port: in_c,
                        transform: None,
                        enabled: true,
                    })
                    .id(),
                Cable {
                    source_port: out_a,
                    target_port: in_c,
                    transform: None,
                    enabled: true,
                },
            ),
            (
                world
                    .spawn(Cable {
                        source_port: out_b,
                        target_port: in_d1,
                        transform: None,
                        enabled: true,
                    })
                    .id(),
                Cable {
                    source_port: out_b,
                    target_port: in_d1,
                    transform: None,
                    enabled: true,
                },
            ),
            (
                world
                    .spawn(Cable {
                        source_port: out_c,
                        target_port: in_d2,
                        transform: None,
                        enabled: true,
                    })
                    .id(),
                Cable {
                    source_port: out_c,
                    target_port: in_d2,
                    transform: None,
                    enabled: true,
                },
            ),
        ];
        let cables_ref: Vec<(Entity, &Cable)> =
            cables_raw.iter().map(|(e, c)| (*e, c)).collect();

        let modules_data = collect_modules(&world, &[mod_a, mod_b, mod_c, mod_d]);
        let modules_ref: Vec<(Entity, &Module, &ModuleId)> = modules_data
            .iter()
            .map(|(e, m, id)| (*e, m, id))
            .collect();

        let ports: Vec<(Entity, Port, Entity)> = [
            (out_a, mod_a),
            (in_b, mod_b),
            (out_b, mod_b),
            (in_c, mod_c),
            (out_c, mod_c),
            (in_d1, mod_d),
            (in_d2, mod_d),
        ]
        .iter()
        .map(|&(pe, me)| (pe, world.entity(pe).get::<Port>().unwrap().clone(), me))
        .collect();
        let ports_ref: Vec<(Entity, &Port, Entity)> =
            ports.iter().map(|(e, p, m)| (*e, p, *m)).collect();

        let order = compute_tick_order(&modules_ref, &cables_ref, &ports_ref).unwrap();

        let pos_a = order.iter().position(|&e| e == mod_a).unwrap();
        let pos_b = order.iter().position(|&e| e == mod_b).unwrap();
        let pos_c = order.iter().position(|&e| e == mod_c).unwrap();
        let pos_d = order.iter().position(|&e| e == mod_d).unwrap();

        // A must be first, D must be last.
        assert_eq!(pos_a, 0, "A should be first");
        assert_eq!(pos_d, 3, "D should be last");
        assert!(pos_b < pos_d, "B before D");
        assert!(pos_c < pos_d, "C before D");
    }

    #[test]
    fn test_topo_no_cables() {
        let mut world = World::new();

        let mod_a = spawn_test_module(&mut world, "gen", "a");
        let mod_b = spawn_test_module(&mut world, "gen", "b");
        let mod_c = spawn_test_module(&mut world, "gen", "c");

        let modules_data = collect_modules(&world, &[mod_a, mod_b, mod_c]);
        let modules_ref: Vec<(Entity, &Module, &ModuleId)> = modules_data
            .iter()
            .map(|(e, m, id)| (*e, m, id))
            .collect();

        let cables_ref: Vec<(Entity, &Cable)> = vec![];
        let ports_ref: Vec<(Entity, &Port, Entity)> = vec![];

        let order = compute_tick_order(&modules_ref, &cables_ref, &ports_ref).unwrap();

        // All modules should be present, sorted by ModuleId.
        assert_eq!(order.len(), 3);

        // Verify sorted by ModuleId.
        let ids: Vec<ModuleId> = order
            .iter()
            .map(|&e| *world.entity(e).get::<ModuleId>().unwrap())
            .collect();
        let mut sorted_ids = ids.clone();
        sorted_ids.sort();
        assert_eq!(ids, sorted_ids, "modules should be in ModuleId order");
    }

    #[test]
    fn test_topo_cycle_detected() {
        let mut world = World::new();

        let mod_a = spawn_test_module(&mut world, "gen", "a");
        let mod_b = spawn_test_module(&mut world, "gen", "b");

        let out_a = spawn_test_port(&mut world, mod_a, "out", PortDirection::Output);
        let in_a = spawn_test_port(&mut world, mod_a, "in", PortDirection::Input);
        let out_b = spawn_test_port(&mut world, mod_b, "out", PortDirection::Output);
        let in_b = spawn_test_port(&mut world, mod_b, "in", PortDirection::Input);

        // A -> B -> A (cycle)
        let cables_raw: Vec<(Entity, Cable)> = vec![
            (
                world
                    .spawn(Cable {
                        source_port: out_a,
                        target_port: in_b,
                        transform: None,
                        enabled: true,
                    })
                    .id(),
                Cable {
                    source_port: out_a,
                    target_port: in_b,
                    transform: None,
                    enabled: true,
                },
            ),
            (
                world
                    .spawn(Cable {
                        source_port: out_b,
                        target_port: in_a,
                        transform: None,
                        enabled: true,
                    })
                    .id(),
                Cable {
                    source_port: out_b,
                    target_port: in_a,
                    transform: None,
                    enabled: true,
                },
            ),
        ];
        let cables_ref: Vec<(Entity, &Cable)> =
            cables_raw.iter().map(|(e, c)| (*e, c)).collect();

        let modules_data = collect_modules(&world, &[mod_a, mod_b]);
        let modules_ref: Vec<(Entity, &Module, &ModuleId)> = modules_data
            .iter()
            .map(|(e, m, id)| (*e, m, id))
            .collect();

        let ports: Vec<(Entity, Port, Entity)> = [
            (out_a, mod_a),
            (in_a, mod_a),
            (out_b, mod_b),
            (in_b, mod_b),
        ]
        .iter()
        .map(|&(pe, me)| (pe, world.entity(pe).get::<Port>().unwrap().clone(), me))
        .collect();
        let ports_ref: Vec<(Entity, &Port, Entity)> =
            ports.iter().map(|(e, p, m)| (*e, p, *m)).collect();

        let result = compute_tick_order(&modules_ref, &cables_ref, &ports_ref);
        assert!(result.is_err(), "should detect feedback cycle");

        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::PatchError::FeedbackCycle(_)),
            "expected FeedbackCycle, got: {err:?}"
        );
    }

    #[test]
    fn test_topo_deterministic() {
        let mut world = World::new();

        let mod_a = spawn_test_module(&mut world, "gen", "a");
        let mod_b = spawn_test_module(&mut world, "gen", "b");
        let mod_c = spawn_test_module(&mut world, "gen", "c");

        let out_a = spawn_test_port(&mut world, mod_a, "out", PortDirection::Output);
        let in_b = spawn_test_port(&mut world, mod_b, "in", PortDirection::Input);
        let out_b = spawn_test_port(&mut world, mod_b, "out", PortDirection::Output);
        let in_c = spawn_test_port(&mut world, mod_c, "in", PortDirection::Input);

        let cables_raw: Vec<(Entity, Cable)> = vec![
            (
                world
                    .spawn(Cable {
                        source_port: out_a,
                        target_port: in_b,
                        transform: None,
                        enabled: true,
                    })
                    .id(),
                Cable {
                    source_port: out_a,
                    target_port: in_b,
                    transform: None,
                    enabled: true,
                },
            ),
            (
                world
                    .spawn(Cable {
                        source_port: out_b,
                        target_port: in_c,
                        transform: None,
                        enabled: true,
                    })
                    .id(),
                Cable {
                    source_port: out_b,
                    target_port: in_c,
                    transform: None,
                    enabled: true,
                },
            ),
        ];
        let cables_ref: Vec<(Entity, &Cable)> =
            cables_raw.iter().map(|(e, c)| (*e, c)).collect();

        let modules_data = collect_modules(&world, &[mod_a, mod_b, mod_c]);
        let modules_ref: Vec<(Entity, &Module, &ModuleId)> = modules_data
            .iter()
            .map(|(e, m, id)| (*e, m, id))
            .collect();

        let ports: Vec<(Entity, Port, Entity)> = [
            (out_a, mod_a),
            (in_b, mod_b),
            (out_b, mod_b),
            (in_c, mod_c),
        ]
        .iter()
        .map(|&(pe, me)| (pe, world.entity(pe).get::<Port>().unwrap().clone(), me))
        .collect();
        let ports_ref: Vec<(Entity, &Port, Entity)> =
            ports.iter().map(|(e, p, m)| (*e, p, *m)).collect();

        let order1 = compute_tick_order(&modules_ref, &cables_ref, &ports_ref).unwrap();
        let order2 = compute_tick_order(&modules_ref, &cables_ref, &ports_ref).unwrap();
        assert_eq!(order1, order2, "topo sort should be deterministic");
    }

    // ── Integration tests (using App with CorePlugin) ─────────────

    use bevy_app::App;
    use crate::{CableIndex, CableTransform, CorePlugin};

    /// Helper: set up a minimal App with CorePlugin and return it.
    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        app
    }

    #[test]
    fn test_propagate_single_cable() {
        let mut app = test_app();

        // Spawn two modules and their ports.
        let world = app.world_mut();
        let mod_a = spawn_test_module(world, "gen", "a");
        let out_a = spawn_test_port(world, mod_a, "out", PortDirection::Output);

        let mod_b = spawn_test_module(world, "gen", "b");
        let in_b = spawn_test_port(world, mod_b, "in", PortDirection::Input);

        // Set output value.
        world
            .entity_mut(out_a)
            .get_mut::<CurrentValue>()
            .unwrap()
            .0 = Value::Float(0.5);

        // Spawn cable.
        let cable = Cable {
            source_port: out_a,
            target_port: in_b,
            transform: None,
            enabled: true,
        };
        let cable_entity = world.spawn(cable.clone()).id();

        // Manually update CableIndex.
        world
            .resource_mut::<CableIndex>()
            .add_cable(cable_entity, &cable);

        // Run one update.
        app.update();

        // Check that in_b now has Float(0.5).
        let world = app.world();
        let cv = world.entity(in_b).get::<CurrentValue>().unwrap();
        assert_eq!(cv.0, Value::Float(0.5));
    }

    #[test]
    fn test_propagate_with_transform() {
        let mut app = test_app();

        let world = app.world_mut();
        let mod_a = spawn_test_module(world, "gen", "a");
        let out_a = spawn_test_port(world, mod_a, "out", PortDirection::Output);

        let mod_b = spawn_test_module(world, "gen", "b");
        let in_b = spawn_test_port(world, mod_b, "in", PortDirection::Input);

        // Source: Float(0.25).
        world
            .entity_mut(out_a)
            .get_mut::<CurrentValue>()
            .unwrap()
            .0 = Value::Float(0.25);

        // Cable with Affine(2.0, 0.0): 0.25 * 2.0 + 0.0 = 0.5.
        let cable = Cable {
            source_port: out_a,
            target_port: in_b,
            transform: Some(CableTransform::Affine {
                factor: 2.0,
                offset: 0.0,
            }),
            enabled: true,
        };
        let cable_entity = world.spawn(cable.clone()).id();
        world
            .resource_mut::<CableIndex>()
            .add_cable(cable_entity, &cable);

        app.update();

        let world = app.world();
        let cv = world.entity(in_b).get::<CurrentValue>().unwrap();
        assert_eq!(cv.0, Value::Float(0.5));
    }

    #[test]
    fn test_propagate_disabled_cable() {
        let mut app = test_app();

        let world = app.world_mut();
        let mod_a = spawn_test_module(world, "gen", "a");
        let out_a = spawn_test_port(world, mod_a, "out", PortDirection::Output);

        let mod_b = spawn_test_module(world, "gen", "b");
        let in_b = spawn_test_port(world, mod_b, "in", PortDirection::Input);

        // Set source to Float(0.99).
        world
            .entity_mut(out_a)
            .get_mut::<CurrentValue>()
            .unwrap()
            .0 = Value::Float(0.99);

        // Set target to a known initial value.
        world
            .entity_mut(in_b)
            .get_mut::<CurrentValue>()
            .unwrap()
            .0 = Value::Float(0.1);

        // Disabled cable.
        let cable = Cable {
            source_port: out_a,
            target_port: in_b,
            transform: None,
            enabled: false,
        };
        let cable_entity = world.spawn(cable.clone()).id();
        world
            .resource_mut::<CableIndex>()
            .add_cable(cable_entity, &cable);

        app.update();

        // Target should retain previous value (sample-and-hold).
        let world = app.world();
        let cv = world.entity(in_b).get::<CurrentValue>().unwrap();
        assert_eq!(cv.0, Value::Float(0.1));
    }

    #[test]
    fn test_merge_average_integration() {
        let mut app = test_app();

        let world = app.world_mut();
        let mod_a = spawn_test_module(world, "gen", "a");
        let out_a = spawn_test_port(world, mod_a, "out", PortDirection::Output);

        let mod_b = spawn_test_module(world, "gen", "b");
        let out_b = spawn_test_port(world, mod_b, "out", PortDirection::Output);

        let mod_c = spawn_test_module(world, "gen", "c");
        // Use Average merge policy.
        let in_c = crate::spawn_port_on_module(
            world,
            mod_c,
            "in",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::Average,
        );

        // Set source values.
        world
            .entity_mut(out_a)
            .get_mut::<CurrentValue>()
            .unwrap()
            .0 = Value::Float(0.25);
        world
            .entity_mut(out_b)
            .get_mut::<CurrentValue>()
            .unwrap()
            .0 = Value::Float(0.75);

        // Two cables targeting the same input.
        let cable1 = Cable {
            source_port: out_a,
            target_port: in_c,
            transform: None,
            enabled: true,
        };
        let cable2 = Cable {
            source_port: out_b,
            target_port: in_c,
            transform: None,
            enabled: true,
        };
        let c1 = world.spawn(cable1.clone()).id();
        let c2 = world.spawn(cable2.clone()).id();
        {
            let mut index = world.resource_mut::<CableIndex>();
            index.add_cable(c1, &cable1);
            index.add_cable(c2, &cable2);
        }

        app.update();

        let world = app.world();
        let cv = world.entity(in_c).get::<CurrentValue>().unwrap();
        // Average of 0.25 and 0.75 = 0.5.
        assert_eq!(cv.0, Value::Float(0.5));
    }

    #[test]
    fn test_sample_and_hold() {
        let mut app = test_app();

        let world = app.world_mut();
        let mod_a = spawn_test_module(world, "gen", "a");
        let in_a = spawn_test_port(world, mod_a, "in", PortDirection::Input);

        // Set an initial value.
        world
            .entity_mut(in_a)
            .get_mut::<CurrentValue>()
            .unwrap()
            .0 = Value::Float(0.42);

        // No cables to in_a. Run two updates.
        app.update();
        app.update();

        // Should retain the value.
        let world = app.world();
        let cv = world.entity(in_a).get::<CurrentValue>().unwrap();
        assert_eq!(cv.0, Value::Float(0.42));
    }
}
