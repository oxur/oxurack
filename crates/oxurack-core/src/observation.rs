//! Owned logical descriptions of mutations emitted by the core.
//!
//! The optional staging resource is installed by the rack facade. Core-only
//! worlds therefore retain their existing setup requirements.

use std::collections::{HashMap, VecDeque};

use bevy_ecs::prelude::{Entity, Resource};

use crate::{ChangeSource, LabelKind, LabelValue, ModuleKind, ParameterName, ParameterValue};

/// Stable identity for a cable within one rack lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObservationCableId(u64);

impl ObservationCableId {
    /// Returns the numeric identity assigned at successful creation.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// An owned logical cable description, independent of ECS entity identity.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservedCable {
    /// Stable cable identity.
    pub id: ObservationCableId,
    /// Source module and port.
    pub source: (String, String),
    /// Target module and port.
    pub target: (String, String),
    /// Current zero-based occurrence among parallel exact-endpoint cables.
    pub occurrence: usize,
    /// Optional signal transform.
    pub transform: Option<crate::CableTransform>,
}

/// A stable logical target for a label mutation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalTarget {
    /// Rack metadata.
    Rack,
    /// Named module instance.
    Module(String),
    /// Cable descriptor captured while the cable is active.
    Cable(ObservedCable),
}

/// One actual core mutation, captured at its producer.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum OwnedChange {
    /// A module entered the active patch.
    ModuleAdded { module: String, kind: ModuleKind },
    /// A module left the active patch.
    ModuleRemoved { module: String, kind: ModuleKind },
    /// A cable was connected.
    CableConnected {
        cable: ObservedCable,
        source: ChangeSource,
    },
    /// A cable was disconnected.
    CableDisconnected {
        cable: ObservedCable,
        source: ChangeSource,
    },
    /// A parameter accepted a write.
    ParameterChanged {
        module: String,
        param: ParameterName,
        previous: Option<ParameterValue>,
        new: ParameterValue,
        source: ChangeSource,
    },
    /// A label accepted a write.
    LabelChanged {
        target: LogicalTarget,
        kind: LabelKind,
        previous: Option<LabelValue>,
        new: LabelValue,
        source: ChangeSource,
    },
    /// A patch was loaded into a candidate world.
    PatchLoaded {
        patch_name: String,
        source: crate::PatchSource,
        module_count: usize,
        cable_count: usize,
        label_count: usize,
    },
}

/// A staged change and its one-based position in the producer stream.
#[derive(Debug, Clone, PartialEq)]
pub struct StagedChange {
    /// Position counts all emissions, including entries later evicted.
    pub position: u64,
    /// Owned event payload.
    pub change: OwnedChange,
}

/// Optional bounded core-side producer staging installed by the facade.
#[derive(Resource, Debug)]
pub struct ObservationStaging {
    records: VecDeque<StagedChange>,
    emitted: u64,
    next_cable_id: u64,
    active_cables: HashMap<Entity, ObservationCableId>,
    unavailable: bool,
}

impl Default for ObservationStaging {
    fn default() -> Self {
        Self {
            records: VecDeque::new(),
            emitted: 0,
            next_cable_id: 1,
            active_cables: HashMap::new(),
            unavailable: false,
        }
    }
}

impl ObservationStaging {
    /// Reserves the next stable cable identity, if observation remains available.
    pub fn allocate_cable(&mut self, entity: Entity) -> Option<ObservationCableId> {
        if self.unavailable {
            return None;
        }
        let id = ObservationCableId(self.next_cable_id);
        let Some(next) = self.next_cable_id.checked_add(1) else {
            self.unavailable = true;
            return None;
        };
        self.next_cable_id = next;
        self.active_cables.insert(entity, id);
        Some(id)
    }

    /// Looks up the stable identity for an active cable.
    pub fn cable_id(&self, entity: Entity) -> Option<ObservationCableId> {
        self.active_cables.get(&entity).copied()
    }

    /// Retires an active cable identity after its descriptor is captured.
    pub fn retire_cable(&mut self, entity: Entity) {
        self.active_cables.remove(&entity);
    }

    /// Adds an emitted event without making the caller's mutation fallible.
    pub fn record(&mut self, change: OwnedChange) {
        if self.unavailable {
            return;
        }
        let Some(position) = self.emitted.checked_add(1) else {
            self.unavailable = true;
            return;
        };
        self.emitted = position;
        self.records.push_back(StagedChange { position, change });
        if self.records.len() > 1024 {
            self.records.pop_front();
        }
    }

    /// Drains the current operation packet without resetting cable identities.
    pub fn take_packet(&mut self) -> (u64, Vec<StagedChange>, bool) {
        let emitted = self.emitted;
        let records = self.records.drain(..).collect();
        (emitted, records, self.unavailable)
    }
}

/// Appends one logical change when optional staging is present.
pub fn stage(world: &mut bevy_ecs::world::World, change: OwnedChange) {
    if let Some(mut staging) = world.get_resource_mut::<ObservationStaging>() {
        staging.record(change);
    }
}

/// Stages a successful label event using an owned target description.
pub fn stage_label(
    world: &mut bevy_ecs::world::World,
    target: LogicalTarget,
    kind: LabelKind,
    new: LabelValue,
) {
    stage(
        world,
        OwnedChange::LabelChanged {
            target,
            kind,
            previous: None,
            new,
            source: ChangeSource::PatchLoad,
        },
    );
}

/// Captures an active cable using stable identity and its current occurrence.
pub fn describe_cable(world: &mut bevy_ecs::world::World, entity: Entity) -> Option<ObservedCable> {
    use bevy_ecs::hierarchy::ChildOf;
    let cable = world.get::<crate::Cable>(entity)?.clone();
    let source_port = world.get::<crate::Port>(cable.source_port)?;
    let target_port = world.get::<crate::Port>(cable.target_port)?;
    let source_owner = world.get::<ChildOf>(cable.source_port)?.parent();
    let target_owner = world.get::<ChildOf>(cable.target_port)?.parent();
    let source_module = world.get::<crate::Module>(source_owner)?;
    let target_module = world.get::<crate::Module>(target_owner)?;
    let source = (
        source_module.instance_name.clone(),
        source_port.name.as_ref().to_owned(),
    );
    let target = (
        target_module.instance_name.clone(),
        target_port.name.as_ref().to_owned(),
    );
    let mut matches = Vec::new();
    let mut query = world.query::<(Entity, &crate::Cable, Option<&crate::CableOrder>)>();
    for (candidate, other, order) in query.iter(world) {
        if other.source_port == cable.source_port && other.target_port == cable.target_port {
            matches.push((order.is_none(), order.map_or(0, |value| value.0), candidate));
        }
    }
    matches.sort_unstable();
    let occurrence = matches
        .iter()
        .position(|(_, _, candidate)| *candidate == entity)?;
    let id = world
        .get_resource::<ObservationStaging>()?
        .cable_id(entity)?;
    Some(ObservedCable {
        id,
        source,
        target,
        occurrence,
        transform: cable.transform,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staging_counter_exhaustion_does_not_wrap() {
        let mut staging = ObservationStaging {
            next_cable_id: u64::MAX,
            ..ObservationStaging::default()
        };
        assert_eq!(staging.allocate_cable(Entity::PLACEHOLDER), None);
        assert_eq!(staging.next_cable_id, u64::MAX);
        assert!(staging.take_packet().2);

        let mut staging = ObservationStaging {
            emitted: u64::MAX,
            ..ObservationStaging::default()
        };
        staging.record(OwnedChange::ModuleAdded {
            module: "x".to_owned(),
            kind: "test".into(),
        });
        assert_eq!(staging.emitted, u64::MAX);
        assert!(staging.take_packet().2);
    }
}
