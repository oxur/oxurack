//! Validated live cable edits against the active patch topology.

use std::collections::{HashMap, HashSet};

use bevy_ecs::hierarchy::ChildOf;
use bevy_ecs::prelude::Entity;
use bevy_ecs::world::World;

use crate::{
    Cable, CableConnected, CableDisconnected, CableIndex, CableOrder, CableTransform, ChangeSource,
    CoreError, MergePolicy, Module, ModuleConfig, ModuleRegistry, ModuleRemoved, Patch, PatchError,
    PatchHandle, Port, PortDirection, ValueKind, mark_propagation_order_dirty,
};

struct Endpoint {
    module_name: String,
    port_entity: Entity,
    port_name: String,
    direction: PortDirection,
    kind: ValueKind,
    merge_policy: MergePolicy,
}

struct PreparedCable {
    cable: Cable,
    ordinal: usize,
    occurrence: usize,
}

struct PreparedIncidentCable {
    entity: Entity,
    cable: Cable,
}

/// Fully validated state for one in-place module retirement.
///
/// Construct this with [`prepare_module_removal`] and pass it to
/// [`commit_module_removal`] only after caller-owned cleanup admission succeeds.
/// The value owns all identities required by the commit so no cable or module
/// names are resolved after mutation begins.
pub struct PreparedModuleRemoval {
    name: String,
    entity: Entity,
    kind: crate::ModuleKind,
    hook: crate::ModuleRemovalHook,
    cables: Vec<PreparedIncidentCable>,
}

impl PreparedModuleRemoval {
    /// Returns the retiring module entity for module-author cleanup preparation.
    #[must_use]
    pub fn module_entity(&self) -> Entity {
        self.entity
    }
}

/// Adds one enabled cable after validating it against the active patch.
///
/// The returned occurrence is the cable's current zero-based position among
/// cables with these exact endpoints, using the same ordering as labels.
pub fn connect_cable(
    world: &mut World,
    source: (&str, &str),
    target: (&str, &str),
    transform: Option<CableTransform>,
    source_of_change: ChangeSource,
) -> Result<usize, CoreError> {
    let prepared = prepare_cable(world, source, target, transform)?;
    let source_port = prepared.cable.source_port;
    let target_port = prepared.cable.target_port;
    let cable_entity = world
        .spawn((prepared.cable.clone(), CableOrder(prepared.ordinal)))
        .id();
    world
        .resource_mut::<CableIndex>()
        .add_cable(cable_entity, &prepared.cable);
    mark_propagation_order_dirty(world);
    world.write_message(CableConnected {
        cable_entity,
        source_port,
        target_port,
        transform,
        source: source_of_change.clone(),
    });
    if let Some(mut staging) = world.get_resource_mut::<crate::ObservationStaging>() {
        let _ = staging.allocate_cable(cable_entity);
    }
    if let Some(cable) = crate::observation::describe_cable(world, cable_entity) {
        crate::observation::stage(
            world,
            crate::OwnedChange::CableConnected {
                cable,
                source: source_of_change.clone(),
            },
        );
    }
    Ok(prepared.occurrence)
}

/// Removes one cable occurrence between exact endpoints.
pub fn disconnect_cable(
    world: &mut World,
    source: (&str, &str),
    target: (&str, &str),
    occurrence: usize,
    source_of_change: ChangeSource,
) -> Result<(), CoreError> {
    let source_endpoint = resolve_endpoint(world, source)?;
    let target_endpoint = resolve_endpoint(world, target)?;
    let mut query = world.query::<(Entity, &Cable, Option<&CableOrder>)>();
    let mut matches: Vec<_> = query
        .iter(world)
        .filter_map(|(entity, cable, order)| {
            (cable.source_port == source_endpoint.port_entity
                && cable.target_port == target_endpoint.port_entity)
                .then_some((order.is_none(), order.map_or(0, |value| value.0), entity))
        })
        .collect();
    matches.sort_unstable();
    let Some((_, _, cable_entity)) = matches.get(occurrence).copied() else {
        return Err(CoreError::UnknownCable {
            source_endpoint: (source.0.to_owned(), source.1.to_owned()),
            target_endpoint: (target.0.to_owned(), target.1.to_owned()),
            occurrence,
        });
    };
    let cable = world
        .get::<Cable>(cable_entity)
        .expect("selected cable remains present during preflight")
        .clone();

    commit_disconnect_cable(world, cable_entity, cable, source_of_change);
    Ok(())
}

/// Validates active membership, hierarchy ownership and every incident cable
/// without mutating the World or emitting a message.
///
/// Keep the World intact between preparation and commit: the returned plan
/// contains the entities and hook selected from this validated snapshot.
///
/// # Errors
///
/// Returns [`CoreError::NoPatchLoaded`], [`CoreError::UnknownModule`], or
/// [`CoreError::InvalidModuleOwnership`] when the active hierarchy, hook registry,
/// ports, incident cables, or cable index is inconsistent.
pub fn prepare_module_removal(
    world: &mut World,
    name: &str,
) -> Result<PreparedModuleRemoval, CoreError> {
    let handle = world
        .get_resource::<PatchHandle>()
        .ok_or(CoreError::NoPatchLoaded)?;
    let entity = handle
        .modules
        .get(name)
        .copied()
        .ok_or_else(|| CoreError::UnknownModule(name.to_owned()))?;
    let module = world
        .get::<Module>(entity)
        .filter(|module| module.instance_name == name)
        .ok_or_else(|| {
            invalid_ownership(name, "active PatchHandle entry has no matching Module")
        })?;
    let kind = module.kind.clone();
    let hook = world
        .resource::<ModuleRegistry>()
        .removal_hook(&kind)
        .ok_or_else(|| invalid_ownership(name, "module kind has no registered removal hook"))?;

    let active_modules = handle.modules.clone();
    let mut children_by_parent: HashMap<Entity, Vec<Entity>> = HashMap::new();
    let mut hierarchy = world.query::<(Entity, &ChildOf)>();
    for (child, parent) in hierarchy.iter(world) {
        children_by_parent
            .entry(parent.parent())
            .or_default()
            .push(child);
    }

    let mut descendants = HashSet::from([entity]);
    let mut pending = vec![entity];
    while let Some(parent) = pending.pop() {
        if let Some(children) = children_by_parent.get(&parent) {
            for child in children {
                if descendants.insert(*child) {
                    pending.push(*child);
                }
            }
        }
    }
    for (active_name, active_entity) in &active_modules {
        if *active_entity != entity && descendants.contains(active_entity) {
            return Err(invalid_ownership(
                name,
                &format!("active module '{active_name}' is nested in the retiring hierarchy"),
            ));
        }
    }

    let mut ports = HashSet::new();
    let mut port_query = world.query::<(Entity, &Port)>();
    for (port_entity, _) in port_query.iter(world) {
        if descendants.contains(&port_entity) {
            ports.insert(port_entity);
        }
    }

    let mut incident = Vec::new();
    let mut incident_endpoints = HashMap::new();
    let mut cable_query = world.query::<(Entity, &Cable, Option<&CableOrder>)>();
    for (cable_entity, cable, order) in cable_query.iter(world) {
        if !ports.contains(&cable.source_port) && !ports.contains(&cable.target_port) {
            continue;
        }
        if world.get::<Port>(cable.source_port).is_none()
            || world.get::<Port>(cable.target_port).is_none()
            || world.get::<ChildOf>(cable.source_port).is_none()
            || world.get::<ChildOf>(cable.target_port).is_none()
        {
            return Err(invalid_ownership(
                name,
                &format!("incident cable {cable_entity:?} has a missing endpoint owner"),
            ));
        }
        let index = world.resource::<CableIndex>();
        if index
            .cables_from(cable.source_port)
            .iter()
            .filter(|id| **id == cable_entity)
            .count()
            != 1
            || index
                .cables_targeting(cable.target_port)
                .iter()
                .filter(|id| **id == cable_entity)
                .count()
                != 1
        {
            return Err(invalid_ownership(
                name,
                &format!("incident cable {cable_entity:?} is inconsistent with CableIndex"),
            ));
        }
        incident.push((
            order.is_none(),
            order.map_or(0, |value| value.0),
            cable_entity,
            cable.clone(),
        ));
        incident_endpoints.insert(cable_entity, (cable.source_port, cable.target_port));
    }
    let index = world.resource::<CableIndex>();
    for port in &ports {
        for (indexed, is_source) in index
            .cables_from(*port)
            .iter()
            .map(|entity| (*entity, true))
            .chain(
                index
                    .cables_targeting(*port)
                    .iter()
                    .map(|entity| (*entity, false)),
            )
        {
            let Some((source, target)) = incident_endpoints.get(&indexed) else {
                return Err(invalid_ownership(
                    name,
                    &format!("CableIndex contains stale incident entity {indexed:?}"),
                ));
            };
            if (is_source && source != port) || (!is_source && target != port) {
                return Err(invalid_ownership(
                    name,
                    &format!("CableIndex incident entity {indexed:?} has inconsistent endpoints"),
                ));
            }
        }
    }
    incident.sort_by_key(|(absent, ordinal, entity, _)| (*absent, *ordinal, *entity));

    Ok(PreparedModuleRemoval {
        name: name.to_owned(),
        entity,
        kind,
        hook,
        cables: incident
            .into_iter()
            .map(|(_, _, entity, cable)| PreparedIncidentCable { entity, cable })
            .collect(),
    })
}

/// Applies a previously prepared module retirement without recoverable failure.
///
/// The caller must enqueue selected output cleanup before calling this method
/// and must not mutate the World between preparation and commit. Invariant
/// violations can panic in debug builds; a registered removal hook can also
/// panic, which is not rolled back.
pub fn commit_module_removal(
    world: &mut World,
    removal: PreparedModuleRemoval,
    source: ChangeSource,
) {
    for incident in removal.cables {
        commit_disconnect_cable(world, incident.entity, incident.cable, source.clone());
    }
    (removal.hook)(world, removal.entity);
    let removed = world
        .resource_mut::<PatchHandle>()
        .modules
        .remove(&removal.name);
    debug_assert_eq!(removed, Some(removal.entity));
    let despawned = world.despawn(removal.entity);
    debug_assert!(despawned);
    world.write_message(ModuleRemoved {
        entity: removal.entity,
        instance_name: removal.name.clone(),
        kind: removal.kind.clone(),
    });
    crate::observation::stage(
        world,
        crate::OwnedChange::ModuleRemoved {
            module: removal.name,
            kind: removal.kind,
        },
    );
    mark_propagation_order_dirty(world);
}

fn commit_disconnect_cable(
    world: &mut World,
    cable_entity: Entity,
    cable: Cable,
    source: ChangeSource,
) {
    let observed = crate::observation::describe_cable(world, cable_entity);
    world
        .resource_mut::<CableIndex>()
        .remove_cable(cable_entity, &cable);
    let despawned = world.despawn(cable_entity);
    debug_assert!(despawned);
    mark_propagation_order_dirty(world);
    world.write_message(CableDisconnected {
        cable_entity,
        source_port: cable.source_port,
        target_port: cable.target_port,
        source: source.clone(),
    });
    if let Some(cable) = observed {
        crate::observation::stage(
            world,
            crate::OwnedChange::CableDisconnected { cable, source },
        );
    }
    if let Some(mut staging) = world.get_resource_mut::<crate::ObservationStaging>() {
        staging.retire_cable(cable_entity);
    }
}

fn invalid_ownership(module: &str, reason: &str) -> CoreError {
    CoreError::InvalidModuleOwnership {
        module: module.to_owned(),
        reason: reason.to_owned(),
    }
}

fn prepare_cable(
    world: &mut World,
    source: (&str, &str),
    target: (&str, &str),
    transform: Option<CableTransform>,
) -> Result<PreparedCable, CoreError> {
    let source_endpoint = resolve_endpoint(world, source)?;
    let target_endpoint = resolve_endpoint(world, target)?;

    if source_endpoint.direction != PortDirection::Output {
        return Err(CoreError::InvalidCableDirection {
            module: source_endpoint.module_name,
            port: source_endpoint.port_name,
            expected: PortDirection::Output,
            actual: source_endpoint.direction,
        });
    }
    if target_endpoint.direction != PortDirection::Input {
        return Err(CoreError::InvalidCableDirection {
            module: target_endpoint.module_name,
            port: target_endpoint.port_name,
            expected: PortDirection::Input,
            actual: target_endpoint.direction,
        });
    }
    validate_transform(source_endpoint.kind, target_endpoint.kind, transform)?;

    let active = active_topology(world)?;
    let target_count = active
        .cables
        .iter()
        .filter(|(_, cable, _)| cable.target_port == target_endpoint.port_entity)
        .count();
    if !target_endpoint
        .merge_policy
        .is_valid_for(target_endpoint.kind)
        || (target_endpoint.merge_policy == MergePolicy::Reject && target_count > 0)
    {
        return Err(PatchError::IllegalMerge {
            module: target_endpoint.module_name.clone(),
            port: target_endpoint.port_name.clone(),
            kind: target_endpoint.kind,
            policy: target_endpoint.merge_policy,
        }
        .into());
    }

    let mut patch = active.patch;
    patch.cables.push(crate::CableConfig {
        source: (
            source_endpoint.module_name.clone(),
            source_endpoint.port_name.clone(),
        ),
        target: (
            target_endpoint.module_name.clone(),
            target_endpoint.port_name.clone(),
        ),
        transform,
        ..Default::default()
    });
    crate::patch::check_patch_cycles(&patch)?;

    let ordinal = active
        .cables
        .iter()
        .filter_map(|(_, _, order)| order.map(|value| value.0))
        .max()
        .map_or(Some(0), |value| value.checked_add(1))
        .ok_or(CoreError::CableOrderExhausted)?;

    let occurrence = active
        .cables
        .iter()
        .filter(|(_, cable, order)| {
            cable.source_port == source_endpoint.port_entity
                && cable.target_port == target_endpoint.port_entity
                && order.is_some()
        })
        .count();

    Ok(PreparedCable {
        cable: Cable {
            source_port: source_endpoint.port_entity,
            target_port: target_endpoint.port_entity,
            transform,
            enabled: true,
        },
        ordinal,
        occurrence,
    })
}

fn validate_transform(
    source_kind: ValueKind,
    target_kind: ValueKind,
    transform: Option<CableTransform>,
) -> Result<(), CoreError> {
    let Some(transform) = transform else {
        return if source_kind == target_kind {
            Ok(())
        } else {
            Err(PatchError::KindMismatch {
                source_kind,
                target_kind,
            }
            .into())
        };
    };
    let invalid = |reason: &str| CoreError::InvalidCableTransform {
        source_kind,
        target_kind,
        transform,
        reason: reason.to_owned(),
    };
    let result_kind = match transform {
        CableTransform::Affine { factor, offset } => {
            if !factor.is_finite() || !offset.is_finite() {
                return Err(invalid("Affine factor and offset must be finite"));
            }
            match source_kind {
                ValueKind::Float | ValueKind::Bipolar => source_kind,
                _ => return Err(invalid("Affine supports Float and Bipolar")),
            }
        }
        CableTransform::Invert => match source_kind {
            ValueKind::Float | ValueKind::Bipolar => source_kind,
            _ => return Err(invalid("Invert supports Float and Bipolar")),
        },
        CableTransform::Clamp { min, max } => {
            if !min.is_finite() || !max.is_finite() {
                return Err(invalid("Clamp bounds must be finite"));
            }
            if min > max {
                return Err(invalid("Clamp minimum must not exceed maximum"));
            }
            match source_kind {
                ValueKind::Float | ValueKind::Bipolar => source_kind,
                _ => return Err(invalid("Clamp supports Float and Bipolar")),
            }
        }
        CableTransform::Threshold { threshold } => {
            if !threshold.is_finite() {
                return Err(invalid("Threshold value must be finite"));
            }
            if source_kind != ValueKind::Float {
                return Err(invalid("Threshold supports Float input"));
            }
            ValueKind::Gate
        }
        CableTransform::GateToFloat => {
            if source_kind != ValueKind::Gate {
                return Err(invalid("GateToFloat supports Gate input"));
            }
            ValueKind::Float
        }
        CableTransform::Unipolar => {
            if source_kind != ValueKind::Float {
                return Err(invalid("Unipolar supports Float input"));
            }
            ValueKind::Bipolar
        }
        CableTransform::Bipolarize => {
            if source_kind != ValueKind::Bipolar {
                return Err(invalid("Bipolarize supports Bipolar input"));
            }
            ValueKind::Float
        }
    };
    if result_kind != target_kind {
        return Err(invalid("transform result kind must match target port kind"));
    }
    Ok(())
}

struct ActiveTopology {
    patch: Patch,
    cables: Vec<(Entity, Cable, Option<CableOrder>)>,
}

fn active_topology(world: &mut World) -> Result<ActiveTopology, CoreError> {
    let handle = world
        .get_resource::<PatchHandle>()
        .ok_or(CoreError::NoPatchLoaded)?;
    let module_entities: HashSet<Entity> = handle.modules.values().copied().collect();
    let mut modules = Vec::with_capacity(handle.modules.len());
    let mut names_by_entity = HashMap::with_capacity(handle.modules.len());
    for (name, entity) in &handle.modules {
        let module = world
            .get::<Module>(*entity)
            .filter(|module| module.instance_name == *name)
            .ok_or_else(|| CoreError::UnknownModule(name.clone()))?;
        names_by_entity.insert(*entity, name.clone());
        modules.push(ModuleConfig {
            kind: module.kind.to_string(),
            instance_name: name.clone(),
            ..Default::default()
        });
    }

    let mut module_for_port = HashMap::new();
    let mut port_names = HashMap::new();
    let mut ports = world.query::<(Entity, &Port, &ChildOf)>();
    for (entity, port, parent) in ports.iter(world) {
        let module_entity = parent.parent();
        if module_entities.contains(&module_entity) {
            module_for_port.insert(entity, module_entity);
            port_names.insert(entity, port.name.as_ref().to_owned());
        }
    }
    let mut cables = world.query::<(Entity, &Cable, Option<&CableOrder>)>();
    let cable_data: Vec<_> = cables
        .iter(world)
        .filter(|(_, cable, _)| {
            module_for_port.contains_key(&cable.source_port)
                && module_for_port.contains_key(&cable.target_port)
        })
        .map(|(entity, cable, order)| (entity, cable.clone(), order.copied()))
        .collect();
    let mut configs = Vec::with_capacity(cable_data.len());
    for (_, cable, _) in &cable_data {
        let source_module = module_for_port[&cable.source_port];
        let target_module = module_for_port[&cable.target_port];
        configs.push(crate::CableConfig {
            source: (
                names_by_entity[&source_module].clone(),
                port_names[&cable.source_port].clone(),
            ),
            target: (
                names_by_entity[&target_module].clone(),
                port_names[&cable.target_port].clone(),
            ),
            transform: cable.transform,
            ..Default::default()
        });
    }
    Ok(ActiveTopology {
        patch: Patch {
            version: "1.0".to_owned(),
            master_seed: 0,
            bpm: 120.0,
            modules,
            cables: configs,
            rack: None,
        },
        cables: cable_data,
    })
}

fn resolve_endpoint(world: &mut World, endpoint: (&str, &str)) -> Result<Endpoint, CoreError> {
    let handle = world
        .get_resource::<PatchHandle>()
        .ok_or(CoreError::NoPatchLoaded)?;
    let module_entity = handle
        .modules
        .get(endpoint.0)
        .copied()
        .ok_or_else(|| CoreError::UnknownModule(endpoint.0.to_owned()))?;
    let module = world
        .get::<Module>(module_entity)
        .filter(|module| module.instance_name == endpoint.0)
        .ok_or_else(|| CoreError::UnknownModule(endpoint.0.to_owned()))?;
    let module_name = module.instance_name.clone();
    let mut query = world.query::<(Entity, &Port, &ChildOf)>();
    let (port_entity, port_name, direction, kind, merge_policy) = query
        .iter(world)
        .find(|(_, port, child)| {
            child.parent() == module_entity && port.name.as_ref() == endpoint.1
        })
        .map(|(entity, port, _)| {
            (
                entity,
                port.name.as_ref().to_owned(),
                port.direction,
                port.value_kind,
                port.merge_policy,
            )
        })
        .ok_or_else(|| {
            CoreError::Patch(PatchError::UnknownPort {
                module: endpoint.0.to_owned(),
                port: endpoint.1.to_owned(),
            })
        })?;
    Ok(Endpoint {
        module_name,
        port_entity,
        port_name,
        direction,
        kind,
        merge_policy,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::prelude::{App, FixedUpdate, Update};
    use bevy_ecs::prelude::Messages;
    use pretty_assertions::assert_eq;

    #[derive(bevy_ecs::prelude::Resource, Default)]
    struct RemovalHookLog(Vec<(usize, usize)>);

    struct RemovalProbe;

    impl crate::OxurackModule for RemovalProbe {
        const KIND: &'static str = "removal_probe";
        const DISPLAY_NAME: &'static str = "Removal Probe";

        fn port_schema() -> &'static [crate::PortSchema] {
            &[]
        }
        fn parameter_schema() -> &'static [crate::ParameterSchema] {
            &[]
        }
        fn spawn(
            world: &mut World,
            name: &str,
            _parameters: &HashMap<String, crate::ParameterValue>,
        ) -> Result<Entity, CoreError> {
            Ok(crate::spawn_module_entity(world, Self::KIND, name))
        }
        fn on_remove(world: &mut World, _entity: Entity) {
            let disconnected = world.resource::<Messages<CableDisconnected>>().len();
            let removed = world.resource::<Messages<ModuleRemoved>>().len();
            world
                .resource_mut::<RemovalHookLog>()
                .0
                .push((disconnected, removed));
        }
    }

    struct TestWorld {
        app: App,
        a_out: Entity,
        a_in: Entity,
        b_in: Entity,
        gate_out: Entity,
        gate_in: Entity,
        bipolar_out: Entity,
        bipolar_in: Entity,
        reject_in: Entity,
    }

    fn setup() -> TestWorld {
        setup_with_schedule(false)
    }

    fn setup_with_schedule(fixed: bool) -> TestWorld {
        let mut app = App::new();
        if fixed {
            app.add_plugins(crate::CorePlugin::fixed());
        } else {
            app.add_plugins(crate::CorePlugin);
        }
        let world = app.world_mut();
        let a = crate::spawn_module_entity(world, "test", "a");
        let b = crate::spawn_module_entity(world, "test", "b");
        let a_out = crate::spawn_port_on_module(
            world,
            a,
            "out",
            PortDirection::Output,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        let a_in = crate::spawn_port_on_module(
            world,
            a,
            "in",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::LastWins,
        );
        crate::spawn_port_on_module(
            world,
            b,
            "out",
            PortDirection::Output,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        let b_in = crate::spawn_port_on_module(
            world,
            b,
            "in",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::LastWins,
        );
        let gate_out = crate::spawn_port_on_module(
            world,
            a,
            "gate_out",
            PortDirection::Output,
            ValueKind::Gate,
            MergePolicy::Reject,
        );
        let gate_in = crate::spawn_port_on_module(
            world,
            b,
            "gate_in",
            PortDirection::Input,
            ValueKind::Gate,
            MergePolicy::Average,
        );
        let bipolar_out = crate::spawn_port_on_module(
            world,
            a,
            "bipolar_out",
            PortDirection::Output,
            ValueKind::Bipolar,
            MergePolicy::Reject,
        );
        let bipolar_in = crate::spawn_port_on_module(
            world,
            b,
            "bipolar_in",
            PortDirection::Input,
            ValueKind::Bipolar,
            MergePolicy::LastWins,
        );
        let reject_in = crate::spawn_port_on_module(
            world,
            b,
            "reject_in",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        world.insert_resource(PatchHandle {
            modules: [("a".to_owned(), a), ("b".to_owned(), b)].into(),
        });
        TestWorld {
            app,
            a_out,
            a_in,
            b_in,
            gate_out,
            gate_in,
            bipolar_out,
            bipolar_in,
            reject_in,
        }
    }

    #[test]
    fn module_removal_orders_all_incident_cables_before_hook_and_module_event() {
        let mut app = App::new();
        app.add_plugins(crate::CorePlugin);
        app.world_mut()
            .resource_mut::<ModuleRegistry>()
            .register::<RemovalProbe>();
        app.world_mut().insert_resource(RemovalHookLog::default());
        let a = crate::spawn_module_entity(app.world_mut(), "removal_probe", "a");
        let b = crate::spawn_module_entity(app.world_mut(), "removal_probe", "b");
        let c = crate::spawn_module_entity(app.world_mut(), "removal_probe", "c");
        let a_out = crate::spawn_port_on_module(
            app.world_mut(),
            a,
            "out",
            PortDirection::Output,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        let b_in = crate::spawn_port_on_module(
            app.world_mut(),
            b,
            "in",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::LastWins,
        );
        let b_out = crate::spawn_port_on_module(
            app.world_mut(),
            b,
            "out",
            PortDirection::Output,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        let c_in = crate::spawn_port_on_module(
            app.world_mut(),
            c,
            "in",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::LastWins,
        );
        app.world_mut().insert_resource(PatchHandle {
            modules: [("a".into(), a), ("b".into(), b), ("c".into(), c)].into(),
        });

        let specs = [
            (a_out, b_in, 3),
            (b_out, c_in, 1),
            (b_out, b_in, 2),
            (b_out, c_in, 0),
        ];
        let cables: Vec<_> = specs
            .into_iter()
            .map(|(source_port, target_port, order)| {
                let cable = Cable {
                    source_port,
                    target_port,
                    transform: None,
                    enabled: true,
                };
                let entity = app
                    .world_mut()
                    .spawn((cable.clone(), CableOrder(order)))
                    .id();
                app.world_mut()
                    .resource_mut::<CableIndex>()
                    .add_cable(entity, &cable);
                (entity, order)
            })
            .collect();
        let removal = prepare_module_removal(app.world_mut(), "b").unwrap();
        commit_module_removal(app.world_mut(), removal, ChangeSource::DirectApi);

        let disconnected: Vec<_> = app
            .world()
            .resource::<Messages<CableDisconnected>>()
            .iter_current_update_messages()
            .map(|event| event.cable_entity)
            .collect();
        let expected: Vec<_> = cables
            .iter()
            .filter(|(_, order)| matches!(order, 0..=3))
            .map(|(entity, order)| (*order, *entity))
            .collect();
        let mut expected = expected;
        expected.sort_by_key(|(order, _)| *order);
        assert_eq!(
            disconnected,
            expected
                .into_iter()
                .map(|(_, entity)| entity)
                .collect::<Vec<_>>()
        );
        assert!(
            app.world()
                .resource::<Messages<CableDisconnected>>()
                .iter_current_update_messages()
                .all(|event| event.source == ChangeSource::DirectApi)
        );
        assert_eq!(app.world().resource::<RemovalHookLog>().0, vec![(4, 0)]);
        assert_eq!(app.world().resource::<Messages<ModuleRemoved>>().len(), 1);
        assert_eq!(app.world().resource::<PatchHandle>().modules.len(), 2);
        assert!(app.world().get::<Module>(a).is_some());
        assert!(app.world().get::<Module>(c).is_some());
        assert!(app.world().get::<Module>(b).is_none());
        assert!(
            app.world()
                .resource::<CableIndex>()
                .cables_targeting(b_in)
                .is_empty()
        );
        assert!(
            app.world()
                .resource::<CableIndex>()
                .cables_from(b_out)
                .is_empty()
        );
    }

    #[test]
    fn transforms_validate_all_supported_kind_pairs_and_numeric_edges() {
        let legal = [
            (ValueKind::Float, ValueKind::Float, None),
            (ValueKind::Gate, ValueKind::Gate, None),
            (ValueKind::Bipolar, ValueKind::Bipolar, None),
            (
                ValueKind::Float,
                ValueKind::Float,
                Some(CableTransform::Affine {
                    factor: 0.0,
                    offset: 0.0,
                }),
            ),
            (
                ValueKind::Float,
                ValueKind::Float,
                Some(CableTransform::Affine {
                    factor: -2.0,
                    offset: 0.0,
                }),
            ),
            (
                ValueKind::Float,
                ValueKind::Float,
                Some(CableTransform::Invert),
            ),
            (
                ValueKind::Float,
                ValueKind::Float,
                Some(CableTransform::Clamp { min: 1.0, max: 1.0 }),
            ),
            (
                ValueKind::Float,
                ValueKind::Gate,
                Some(CableTransform::Threshold { threshold: 0.0 }),
            ),
            (
                ValueKind::Gate,
                ValueKind::Float,
                Some(CableTransform::GateToFloat),
            ),
            (
                ValueKind::Float,
                ValueKind::Bipolar,
                Some(CableTransform::Unipolar),
            ),
            (
                ValueKind::Bipolar,
                ValueKind::Float,
                Some(CableTransform::Bipolarize),
            ),
            (
                ValueKind::Bipolar,
                ValueKind::Bipolar,
                Some(CableTransform::Affine {
                    factor: 1.0,
                    offset: 0.0,
                }),
            ),
            (
                ValueKind::Bipolar,
                ValueKind::Bipolar,
                Some(CableTransform::Invert),
            ),
            (
                ValueKind::Bipolar,
                ValueKind::Bipolar,
                Some(CableTransform::Clamp {
                    min: -1.0,
                    max: 1.0,
                }),
            ),
        ];
        for (source, target, transform) in legal {
            assert!(validate_transform(source, target, transform).is_ok());
        }
        for transform in [
            CableTransform::Affine {
                factor: f32::NAN,
                offset: 0.0,
            },
            CableTransform::Affine {
                factor: 1.0,
                offset: f32::INFINITY,
            },
            CableTransform::Clamp {
                min: f32::NEG_INFINITY,
                max: 1.0,
            },
            CableTransform::Clamp { min: 2.0, max: 1.0 },
            CableTransform::Threshold {
                threshold: f32::NAN,
            },
        ] {
            assert!(matches!(
                validate_transform(ValueKind::Float, ValueKind::Float, Some(transform)),
                Err(CoreError::InvalidCableTransform { .. })
            ));
        }
        assert!(matches!(
            validate_transform(
                ValueKind::Gate,
                ValueKind::Gate,
                Some(CableTransform::Affine {
                    factor: 1.0,
                    offset: 0.0
                })
            ),
            Err(CoreError::InvalidCableTransform { .. })
        ));
        assert!(matches!(
            validate_transform(
                ValueKind::Float,
                ValueKind::Gate,
                Some(CableTransform::Invert)
            ),
            Err(CoreError::InvalidCableTransform { .. })
        ));
        assert!(matches!(
            validate_transform(
                ValueKind::Float,
                ValueKind::Bipolar,
                Some(CableTransform::Threshold { threshold: 0.5 })
            ),
            Err(CoreError::InvalidCableTransform { .. })
        ));
        assert!(matches!(
            validate_transform(ValueKind::Float, ValueKind::Bipolar, None),
            Err(CoreError::Patch(PatchError::KindMismatch { .. }))
        ));
    }

    #[test]
    fn public_connect_errors_preserve_rebuilt_topology_and_emit_nothing() {
        #[derive(Debug, PartialEq)]
        struct CableObservation {
            entity: Entity,
            source: Entity,
            target: Entity,
            transform: Option<CableTransform>,
            enabled: bool,
            order: Option<usize>,
        }
        #[derive(Debug, PartialEq)]
        struct Observation {
            cables: Vec<CableObservation>,
            from: Vec<Entity>,
            to: Vec<Entity>,
            dirty: bool,
            order: Vec<Entity>,
            values: Vec<crate::Value>,
            contributions: Vec<Vec<crate::Value>>,
            connected: usize,
            disconnected: usize,
        }
        fn observe(world: &mut World, ports: &[Entity]) -> Observation {
            let mut query = world.query::<(Entity, &Cable, Option<&CableOrder>)>();
            let mut cables = query
                .iter(world)
                .map(|(entity, cable, order)| CableObservation {
                    entity,
                    source: cable.source_port,
                    target: cable.target_port,
                    transform: cable.transform,
                    enabled: cable.enabled,
                    order: order.map(|value| value.0),
                })
                .collect::<Vec<_>>();
            cables.sort_by_key(|item| item.entity);
            let index = world.resource::<CableIndex>();
            let mut from = ports
                .iter()
                .flat_map(|port| index.cables_from(*port).iter().copied())
                .collect::<Vec<_>>();
            let mut to = ports
                .iter()
                .flat_map(|port| index.cables_targeting(*port).iter().copied())
                .collect::<Vec<_>>();
            from.sort();
            to.sort();
            let dirty = world.resource::<crate::PropagationOrderDirty>().0;
            let order = world.resource::<crate::PropagationOrder>().cables.clone();
            let values = ports
                .iter()
                .map(|port| world.get::<crate::CurrentValue>(*port).unwrap().0)
                .collect();
            let contributions = ports
                .iter()
                .map(|port| {
                    world
                        .resource::<crate::MergeBuffers>()
                        .contributions(*port)
                        .to_vec()
                })
                .collect();
            let connected = world
                .resource::<Messages<CableConnected>>()
                .iter_current_update_messages()
                .count();
            let disconnected = world
                .resource::<Messages<CableDisconnected>>()
                .iter_current_update_messages()
                .count();
            Observation {
                cables,
                from,
                to,
                order,
                dirty,
                values,
                contributions,
                connected,
                disconnected,
            }
        }

        let mut test = setup();
        let world = test.app.world_mut();
        crate::connect_cable(
            world,
            ("a", "out"),
            ("b", "in"),
            None,
            ChangeSource::DirectApi,
        )
        .unwrap();
        world.run_schedule(Update);
        assert!(!world.resource::<crate::PropagationOrderDirty>().0);
        world.resource_mut::<Messages<CableConnected>>().clear();
        world.resource_mut::<Messages<CableDisconnected>>().clear();
        let ports = [
            test.a_out,
            test.a_in,
            test.b_in,
            test.gate_out,
            test.gate_in,
            test.bipolar_out,
            test.bipolar_in,
            test.reject_in,
        ];
        macro_rules! unchanged_error {
            ($call:expr, $pattern:pat $(if $guard:expr)?) => {{
                let before = observe(world, &ports);
                let result = $call;
                let description = format!("{result:?}");
                assert!(matches!(result, $pattern $(if $guard)?), "unexpected result: {description}");
                assert_eq!(observe(world, &ports), before);
            }};
        }
        unchanged_error!(crate::connect_cable(world, ("missing", "out"), ("b", "in"), None, ChangeSource::DirectApi), Err(CoreError::UnknownModule(name)) if name == "missing");
        unchanged_error!(
            crate::connect_cable(
                world,
                ("a", "missing"),
                ("b", "in"),
                None,
                ChangeSource::DirectApi
            ),
            Err(CoreError::Patch(PatchError::UnknownPort { .. }))
        );
        unchanged_error!(
            crate::connect_cable(
                world,
                ("a", "in"),
                ("b", "in"),
                None,
                ChangeSource::DirectApi
            ),
            Err(CoreError::InvalidCableDirection {
                expected: PortDirection::Output,
                actual: PortDirection::Input,
                ..
            })
        );
        unchanged_error!(
            crate::connect_cable(
                world,
                ("a", "out"),
                ("b", "out"),
                None,
                ChangeSource::DirectApi
            ),
            Err(CoreError::InvalidCableDirection {
                expected: PortDirection::Input,
                actual: PortDirection::Output,
                ..
            })
        );
        unchanged_error!(
            crate::connect_cable(
                world,
                ("a", "gate_out"),
                ("b", "bipolar_in"),
                None,
                ChangeSource::DirectApi
            ),
            Err(CoreError::Patch(PatchError::KindMismatch {
                source_kind: ValueKind::Gate,
                target_kind: ValueKind::Bipolar
            }))
        );
        unchanged_error!(
            crate::connect_cable(
                world,
                ("a", "out"),
                ("b", "in"),
                Some(CableTransform::Affine {
                    factor: f32::NAN,
                    offset: 0.0
                }),
                ChangeSource::DirectApi
            ),
            Err(CoreError::InvalidCableTransform { .. })
        );
        unchanged_error!(
            crate::connect_cable(
                world,
                ("a", "out"),
                ("b", "in"),
                Some(CableTransform::Clamp { min: 2.0, max: 1.0 }),
                ChangeSource::DirectApi
            ),
            Err(CoreError::InvalidCableTransform { .. })
        );
        unchanged_error!(
            crate::connect_cable(
                world,
                ("a", "gate_out"),
                ("b", "gate_in"),
                Some(CableTransform::Affine {
                    factor: 1.0,
                    offset: 0.0
                }),
                ChangeSource::DirectApi
            ),
            Err(CoreError::InvalidCableTransform { .. })
        );
        unchanged_error!(
            crate::connect_cable(
                world,
                ("a", "out"),
                ("b", "in"),
                Some(CableTransform::Threshold { threshold: 0.5 }),
                ChangeSource::DirectApi
            ),
            Err(CoreError::InvalidCableTransform {
                source_kind: ValueKind::Float,
                target_kind: ValueKind::Float,
                reason,
                ..
            }) if reason == "transform result kind must match target port kind"
        );
        unchanged_error!(
            crate::connect_cable(
                world,
                ("a", "gate_out"),
                ("b", "gate_in"),
                None,
                ChangeSource::DirectApi
            ),
            Err(CoreError::Patch(PatchError::IllegalMerge { .. }))
        );
        unchanged_error!(
            crate::connect_cable(
                world,
                ("b", "out"),
                ("a", "in"),
                None,
                ChangeSource::DirectApi
            ),
            Err(CoreError::Patch(PatchError::FeedbackCycle(_)))
        );
        let stale_entity = world.spawn_empty().id();
        world.despawn(stale_entity);
        world
            .resource_mut::<PatchHandle>()
            .modules
            .insert("stale".to_owned(), stale_entity);
        unchanged_error!(
            crate::connect_cable(
                world,
                ("stale", "out"),
                ("b", "in"),
                None,
                ChangeSource::DirectApi
            ),
            Err(CoreError::UnknownModule(name)) if name == "stale"
        );
    }

    #[test]
    fn command_edits_have_provenance_and_remove_only_occurrence_zero() {
        let mut test = setup();
        let world = test.app.world_mut();
        world.resource_mut::<Messages<CableConnected>>().clear();
        world.resource_mut::<Messages<CableDisconnected>>().clear();
        let cmd = crate::CoreCommand::AddCable {
            source: ("a".into(), "out".into()),
            target: ("b".into(), "in".into()),
            transform: Some(CableTransform::Affine {
                factor: 0.0,
                offset: 0.75,
            }),
        };
        crate::dispatch_core_command(world, &cmd).unwrap();
        let added: Vec<_> = world
            .resource::<Messages<CableConnected>>()
            .iter_current_update_messages()
            .collect();
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].source, ChangeSource::Command);
        assert_eq!(added[0].source_port, test.a_out);
        assert_eq!(added[0].target_port, test.b_in);
        assert_eq!(
            added[0].transform,
            Some(CableTransform::Affine {
                factor: 0.0,
                offset: 0.75
            })
        );
        let first_entity = added[0].cable_entity;
        assert!(world.get::<Cable>(first_entity).is_some());
        assert!(
            world
                .resource::<CableIndex>()
                .cables_from(test.a_out)
                .contains(&first_entity)
        );
        crate::dispatch_core_command(world, &cmd).unwrap();
        assert_eq!(
            world
                .resource::<CableIndex>()
                .cables_targeting(test.b_in)
                .len(),
            2
        );
        let remove = crate::CoreCommand::RemoveCable {
            source: ("a".into(), "out".into()),
            target: ("b".into(), "in".into()),
        };
        crate::dispatch_core_command(world, &remove).unwrap();
        let removed: Vec<_> = world
            .resource::<Messages<CableDisconnected>>()
            .iter_current_update_messages()
            .collect();
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0].source, ChangeSource::Command);
        assert_eq!(removed[0].cable_entity, first_entity);
        assert_eq!(
            (removed[0].source_port, removed[0].target_port),
            (test.a_out, test.b_in)
        );
        assert!(world.get_entity(first_entity).is_err());
        let failed = crate::CoreCommand::AddCable {
            source: ("missing".into(), "out".into()),
            target: ("b".into(), "in".into()),
            transform: None,
        };
        assert!(
            matches!(crate::dispatch_core_command(world, &failed), Err(CoreError::UnknownModule(name)) if name == "missing")
        );
        assert_eq!(
            world
                .resource::<Messages<CableConnected>>()
                .iter_current_update_messages()
                .count(),
            2
        );
        assert_eq!(
            world
                .resource::<CableIndex>()
                .cables_targeting(test.b_in)
                .len(),
            1
        );
    }

    #[test]
    fn cycle_and_merge_failures_and_exhaustion_are_preflight_only() {
        let mut test = setup();
        let world = test.app.world_mut();
        crate::connect_cable(
            world,
            ("a", "out"),
            ("b", "in"),
            None,
            ChangeSource::DirectApi,
        )
        .unwrap();
        let before_index = world
            .resource::<CableIndex>()
            .cables_targeting(test.b_in)
            .to_vec();
        world.resource_mut::<crate::PropagationOrderDirty>().0 = false;
        let dirty_before = false;
        let invalid_first_merge = crate::connect_cable(
            world,
            ("a", "gate_out"),
            ("b", "gate_in"),
            None,
            ChangeSource::DirectApi,
        );
        assert!(matches!(
            invalid_first_merge,
            Err(CoreError::Patch(PatchError::IllegalMerge { .. }))
        ));
        assert!(!world.resource::<crate::PropagationOrderDirty>().0);
        assert!(
            world
                .resource::<CableIndex>()
                .cables_targeting(test.gate_in)
                .is_empty()
        );
        let cycle = crate::connect_cable(
            world,
            ("b", "out"),
            ("a", "in"),
            None,
            ChangeSource::DirectApi,
        );
        assert!(matches!(
            cycle,
            Err(CoreError::Patch(PatchError::FeedbackCycle(_)))
        ));
        assert_eq!(
            world.resource::<CableIndex>().cables_targeting(test.b_in),
            before_index
        );
        assert_eq!(
            world.resource::<crate::PropagationOrderDirty>().0,
            dirty_before
        );
        assert_eq!(
            world
                .resource::<Messages<CableConnected>>()
                .iter_current_update_messages()
                .count(),
            1
        );

        crate::connect_cable(
            world,
            ("a", "out"),
            ("b", "reject_in"),
            None,
            ChangeSource::DirectApi,
        )
        .unwrap();
        let reject_existing = world
            .resource::<CableIndex>()
            .cables_targeting(test.reject_in)[0];
        world.get_mut::<Cable>(reject_existing).unwrap().enabled = false;
        world.resource_mut::<crate::PropagationOrderDirty>().0 = false;
        let before_reject_ids: Vec<_> = {
            let mut query = world.query::<(Entity, &Cable)>();
            query.iter(world).map(|(entity, _)| entity).collect()
        };
        let before_reject_index = world
            .resource::<CableIndex>()
            .cables_targeting(test.reject_in)
            .to_vec();
        let fan_in = crate::connect_cable(
            world,
            ("b", "out"),
            ("b", "reject_in"),
            None,
            ChangeSource::DirectApi,
        );
        assert!(matches!(
            fan_in,
            Err(CoreError::Patch(PatchError::IllegalMerge { .. }))
        ));
        let after_reject_ids: Vec<_> = {
            let mut query = world.query::<(Entity, &Cable)>();
            query.iter(world).map(|(entity, _)| entity).collect()
        };
        assert_eq!(after_reject_ids, before_reject_ids);
        assert_eq!(
            world
                .resource::<CableIndex>()
                .cables_targeting(test.reject_in),
            before_reject_index
        );
        assert!(!world.resource::<crate::PropagationOrderDirty>().0);
        assert_eq!(
            world.resource::<CableIndex>().cables_targeting(test.b_in),
            before_index
        );
        assert_eq!(
            world
                .resource::<Messages<CableConnected>>()
                .iter_current_update_messages()
                .count(),
            2
        );

        let max_cable = Cable {
            source_port: test.a_out,
            target_port: test.b_in,
            transform: None,
            enabled: false,
        };
        let max_entity = world
            .spawn((max_cable.clone(), CableOrder(usize::MAX)))
            .id();
        world
            .resource_mut::<CableIndex>()
            .add_cable(max_entity, &max_cable);
        let dirty_at_max = world.resource::<crate::PropagationOrderDirty>().0;
        let overflow = crate::connect_cable(
            world,
            ("a", "out"),
            ("a", "in"),
            None,
            ChangeSource::DirectApi,
        );
        assert!(matches!(overflow, Err(CoreError::CableOrderExhausted)));
        assert_eq!(
            world.resource::<crate::PropagationOrderDirty>().0,
            dirty_at_max
        );
        assert_eq!(
            world
                .resource::<CableIndex>()
                .cables_targeting(test.a_in)
                .len(),
            0
        );
    }

    #[test]
    fn same_module_self_connection_is_allowed_and_disabled_cables_count_for_fanin() {
        let mut test = setup();
        let world = test.app.world_mut();
        let occurrence = crate::connect_cable(
            world,
            ("a", "out"),
            ("a", "in"),
            None,
            ChangeSource::DirectApi,
        )
        .unwrap();
        assert_eq!(occurrence, 0);
        let first = world.resource::<CableIndex>().cables_targeting(test.a_in)[0];
        let mut cable = world.get_mut::<Cable>(first).unwrap();
        cable.enabled = false;
        cable.transform = Some(CableTransform::Clamp { min: 2.0, max: 1.0 });
        let second = crate::connect_cable(
            world,
            ("b", "out"),
            ("a", "in"),
            None,
            ChangeSource::DirectApi,
        )
        .unwrap();
        assert_eq!(second, 0);
        // The target's LastWins policy admits fan-in even when one structural cable is disabled.
        assert_eq!(
            world
                .resource::<CableIndex>()
                .cables_targeting(test.a_in)
                .len(),
            2
        );
        crate::disconnect_cable(world, ("a", "out"), ("a", "in"), 0, ChangeSource::DirectApi)
            .unwrap();
        assert_eq!(
            world
                .resource::<CableIndex>()
                .cables_targeting(test.a_in)
                .len(),
            1
        );
        let _ = (
            test.gate_out,
            test.gate_in,
            test.bipolar_out,
            test.bipolar_in,
            test.reject_in,
        );
    }

    #[test]
    fn superseded_topology_is_excluded_and_unpatch_uses_shared_entity_fallback() {
        let mut test = setup();
        let world = test.app.world_mut();
        let old_a = crate::spawn_module_entity(world, "test", "a");
        let old_out = crate::spawn_port_on_module(
            world,
            old_a,
            "out",
            PortDirection::Output,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        let stale = Cable {
            source_port: old_out,
            target_port: test.b_in,
            transform: None,
            enabled: true,
        };
        let stale_entity = world.spawn((stale.clone(), CableOrder(usize::MAX))).id();
        world
            .resource_mut::<CableIndex>()
            .add_cable(stale_entity, &stale);

        crate::connect_cable(
            world,
            ("a", "out"),
            ("b", "in"),
            None,
            ChangeSource::DirectApi,
        )
        .unwrap();
        let active_added = world
            .resource::<CableIndex>()
            .cables_targeting(test.b_in)
            .iter()
            .copied()
            .find(|entity| *entity != stale_entity)
            .unwrap();
        assert_eq!(world.get::<CableOrder>(active_added), Some(&CableOrder(0)));

        let first = Cable {
            source_port: test.a_out,
            target_port: test.a_in,
            transform: None,
            enabled: true,
        };
        let first_id = world.spawn((first.clone(), CableOrder(7))).id();
        world
            .resource_mut::<CableIndex>()
            .add_cable(first_id, &first);
        let second_id = world.spawn((first.clone(), CableOrder(7))).id();
        world
            .resource_mut::<CableIndex>()
            .add_cable(second_id, &first);
        let absent_id = world.spawn(first.clone()).id();
        world
            .resource_mut::<CableIndex>()
            .add_cable(absent_id, &first);
        let (entity_first, entity_second) = if first_id < second_id {
            (first_id, second_id)
        } else {
            (second_id, first_id)
        };
        world.entity_mut(entity_first).insert(CableOrder(10));
        world.entity_mut(entity_second).insert(CableOrder(0));
        let expected = [entity_second, entity_first, absent_id];
        let endpoint_label = |occurrence| crate::LabelTarget::Cable {
            source: ("a".to_owned(), "out".to_owned()),
            target: ("a".to_owned(), "in".to_owned()),
            occurrence,
        };
        for (occurrence, name) in [(0, "ordinal-zero"), (1, "ordinal-ten")] {
            crate::set_label_value(
                world,
                &endpoint_label(occurrence),
                &crate::LabelKind::Description,
                &crate::LabelValue::Text(name.to_owned()),
                ChangeSource::DirectApi,
            )
            .unwrap();
        }
        assert_eq!(
            crate::label_value(world, &endpoint_label(0), &crate::LabelKind::Description).unwrap(),
            Some(crate::LabelValue::Text("ordinal-zero".to_owned()))
        );
        assert_eq!(
            crate::label_value(world, &endpoint_label(1), &crate::LabelKind::Description).unwrap(),
            Some(crate::LabelValue::Text("ordinal-ten".to_owned()))
        );
        crate::disconnect_cable(world, ("a", "out"), ("a", "in"), 1, ChangeSource::DirectApi)
            .unwrap();
        assert!(world.get_entity(expected[0]).is_ok());
        assert!(world.get_entity(expected[1]).is_err());
        assert!(world.get_entity(expected[2]).is_ok());
        assert_eq!(
            crate::label_value(world, &endpoint_label(0), &crate::LabelKind::Description).unwrap(),
            Some(crate::LabelValue::Text("ordinal-zero".to_owned()))
        );
        let appended = crate::connect_cable(
            world,
            ("a", "out"),
            ("a", "in"),
            None,
            ChangeSource::DirectApi,
        )
        .unwrap();
        assert_eq!(appended, 1, "unordered legacy cable is not an occurrence");
        let new_entity = world
            .resource::<CableIndex>()
            .cables_from(test.a_out)
            .iter()
            .copied()
            .find(|entity| {
                *entity != expected[0]
                    && world
                        .get::<Cable>(*entity)
                        .is_some_and(|cable| cable.target_port == test.a_in)
            })
            .unwrap();
        crate::set_label_value(
            world,
            &endpoint_label(appended),
            &crate::LabelKind::Description,
            &crate::LabelValue::Text("new-cable".to_owned()),
            ChangeSource::DirectApi,
        )
        .unwrap();
        assert!(world.get_entity(new_entity).is_ok());
        assert_eq!(
            crate::label_value(
                world,
                &endpoint_label(appended),
                &crate::LabelKind::Description
            )
            .unwrap(),
            Some(crate::LabelValue::Text("new-cable".to_owned()))
        );
    }

    #[test]
    fn rewiring_changes_the_next_sample_and_removed_input_holds_in_update_and_fixed() {
        for fixed in [false, true] {
            let mut test = setup_with_schedule(fixed);
            test.app
                .world_mut()
                .get_mut::<crate::CurrentValue>(test.a_out)
                .unwrap()
                .0 = crate::Value::Float(0.75);
            crate::connect_cable(
                test.app.world_mut(),
                ("a", "out"),
                ("b", "in"),
                Some(CableTransform::Affine {
                    factor: 1.0,
                    offset: 0.0,
                }),
                ChangeSource::DirectApi,
            )
            .unwrap();
            let run = |app: &mut App| {
                if fixed {
                    app.world_mut().run_schedule(FixedUpdate);
                } else {
                    app.world_mut().run_schedule(Update);
                }
            };
            run(&mut test.app);
            assert_eq!(
                test.app
                    .world()
                    .get::<crate::CurrentValue>(test.b_in)
                    .unwrap()
                    .0,
                crate::Value::Float(0.75)
            );
            crate::disconnect_cable(
                test.app.world_mut(),
                ("a", "out"),
                ("b", "in"),
                0,
                ChangeSource::DirectApi,
            )
            .unwrap();
            run(&mut test.app);
            assert_eq!(
                test.app
                    .world()
                    .get::<crate::CurrentValue>(test.b_in)
                    .unwrap()
                    .0,
                crate::Value::Float(0.75)
            );
            crate::connect_cable(
                test.app.world_mut(),
                ("a", "out"),
                ("b", "in"),
                Some(CableTransform::Affine {
                    factor: 0.0,
                    offset: 0.25,
                }),
                ChangeSource::DirectApi,
            )
            .unwrap();
            run(&mut test.app);
            assert_eq!(
                test.app
                    .world()
                    .get::<crate::CurrentValue>(test.b_in)
                    .unwrap()
                    .0,
                crate::Value::Float(0.25)
            );
        }
    }
}
