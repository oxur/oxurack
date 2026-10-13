//! Whole-App lifetime operations for [`super::Oxurack`].

use std::path::Path;
use std::time::Instant;

use bevy_time::prelude::{Fixed, Real, Time, Virtual};
use oxurack_core::{CoreError, ModuleRegistry, Patch, PatchSource, SimulationRunning};
use oxurack_mod_midiout::{
    MidiOutputBuffer, prepare_silence, prepare_silence_for, silence_outputs,
};

use crate::{Oxurack, build_base_app};

impl Oxurack {
    /// Removes one active module and its owned descendants while preserving
    /// the current rack lifetime and all surviving engines.
    pub fn remove_module(&mut self, name: &str) -> Result<(), CoreError> {
        self.require_stopped_and_drained()?;
        let removal = oxurack_core::prepare_module_removal(self.app.world_mut(), name)?;
        let module_entity = removal.module_entity();
        let cleanup = prepare_silence_for(self.app.world(), module_entity)?;
        self.app
            .world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .append_silence(&cleanup)
            .map_err(CoreError::Tick)?;
        if self
            .app
            .world()
            .get_resource::<crate::ScopeCapture>()
            .is_some_and(|scope| {
                scope
                    .selectors()
                    .iter()
                    .any(|selector| selector.module == name)
            })
        {
            crate::scope::disable(self.app.world_mut());
        }
        oxurack_core::commit_module_removal(
            self.app.world_mut(),
            removal,
            oxurack_core::ChangeSource::DirectApi,
        );
        self.instance_tokens.remove(name);
        #[cfg(feature = "graph")]
        self.app
            .world_mut()
            .run_schedule(oxurack_graph::GraphMaintenance);
        Ok(())
    }

    /// Appends complete MIDI voice silence while preserving this rack lifetime.
    ///
    /// This does not tick, reseed, stop, or clear a previously latched fault.
    /// The output must still be drained before another sample can be admitted.
    ///
    /// # Errors
    ///
    /// Returns an output capacity/invariant error before changing events or
    /// voice state when the complete silence batch cannot be prepared.
    pub fn reset(&mut self) -> Result<(), CoreError> {
        silence_outputs(self.app.world_mut()).map_err(CoreError::Tick)?;
        self.command_queue.cancel_pending();
        Ok(())
    }

    /// Rebuilds the loaded patch from its current live snapshot at sample zero.
    ///
    /// The current rack must be stopped and drained. Installed module hooks run
    /// once in their original successful install order in the new App. Arbitrary
    /// World state is not cloned. User hook/spawner panic and external side
    /// effects are outside the rollback guarantee.
    ///
    /// # Errors
    ///
    /// Returns stopped/pending-output errors before snapshot errors, then any
    /// snapshot, candidate, or capacity error without replacing the old App.
    pub fn restart(&mut self) -> Result<(), CoreError> {
        self.require_stopped_and_drained()?;
        let patch = self.snapshot_patch()?;
        self.replace_configuration(&patch, PatchSource::InMemory)
    }

    /// Replaces the current patch with an empty, stopped rack lifetime.
    ///
    /// Cleanup is carried into the new zero-sink buffer and remains drainable.
    /// Repeated unload of an already empty rack is a no-op after admission.
    ///
    /// # Errors
    ///
    /// Returns stopped/pending-output errors before candidate construction.
    pub fn unload_patch(&mut self) -> Result<(), CoreError> {
        self.require_stopped_and_drained()?;
        if self.loaded_identity.is_none() {
            return Ok(());
        }
        let cleanup = prepare_silence(self.app.world())?;
        let mut next = build_base_app()?;
        self.replay_installations(&mut next);
        validate_candidate(&next)?;
        maintain_candidate(&mut next);
        next.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .append_silence(&cleanup)?;
        self.commit(next, None);
        Ok(())
    }

    fn require_stopped_and_drained(&self) -> Result<(), CoreError> {
        if self.app.world().resource::<SimulationRunning>().0 {
            return Err(CoreError::LifecycleRequiresStopped);
        }
        let count = self
            .app
            .world()
            .resource::<MidiOutputBuffer>()
            .pending_count();
        if count != 0 {
            return Err(CoreError::PendingMidi { count });
        }
        Ok(())
    }

    fn replay_installations(&self, next: &mut bevy_app::App) {
        for (index, recipe) in self.install_order.iter().enumerate() {
            debug_assert_eq!(self.installed_modules.get(&recipe.kind), Some(&index));
            (recipe.install_into)(next);
        }
    }

    fn replace_configuration(
        &mut self,
        patch: &Patch,
        source: PatchSource,
    ) -> Result<(), CoreError> {
        self.require_stopped_and_drained()?;
        let identity = super::persistence::LoadedIdentity::from_patch(patch);
        let cleanup = prepare_silence(self.app.world())?;
        let mut next = build_base_app()?;
        self.replay_installations(&mut next);
        apply_in_candidate(&mut next, patch, source)?;
        validate_candidate(&next)?;
        maintain_candidate(&mut next);
        next.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .append_silence(&cleanup)?;
        self.commit(next, Some(identity));
        Ok(())
    }

    fn commit(
        &mut self,
        next: bevy_app::App,
        identity: Option<super::persistence::LoadedIdentity>,
    ) {
        let instance_tokens = next
            .world()
            .get_resource::<oxurack_core::PatchHandle>()
            .map(|handle| {
                handle
                    .modules
                    .keys()
                    .map(|name| (name.clone(), std::sync::Arc::new(())))
                    .collect()
            })
            .unwrap_or_default();
        let lifetime_token = std::sync::Arc::new(());
        let old = std::mem::replace(&mut self.app, next);
        self.loaded_identity = identity;
        self.lifetime_token = lifetime_token;
        self.instance_tokens = instance_tokens;
        drop(old);
    }
}

pub(super) fn load_patch(rack: &mut Oxurack, path: &Path) -> Result<(), CoreError> {
    load_patch_with_source(rack, path, PatchSource::File(path.to_path_buf()))
}

pub(super) fn load_patch_with_source(
    rack: &mut Oxurack,
    path: &Path,
    source: PatchSource,
) -> Result<(), CoreError> {
    rack.require_stopped_and_drained()?;
    let patch = oxurack_core::load_patch_from_file(path)?;
    rack.replace_configuration(&patch, source)
}

fn apply_in_candidate(
    app: &mut bevy_app::App,
    patch: &Patch,
    source: PatchSource,
) -> Result<(), CoreError> {
    app.world_mut()
        .resource_scope::<ModuleRegistry, _>(|world, registry| {
            let handle = oxurack_core::apply_patch_to_world(patch, &registry, world, source)?;
            world.insert_resource(handle);
            Ok(())
        })
}

fn validate_candidate(app: &bevy_app::App) -> Result<(), CoreError> {
    let buffer = app.world().resource::<MidiOutputBuffer>();
    let count = buffer.pending_count();
    if count != 0 {
        return Err(CoreError::CandidateMidiNotEmpty { count });
    }
    if let Some(fault) = buffer.fault() {
        return Err(CoreError::Tick(fault.clone()));
    }
    Ok(())
}

fn maintain_candidate(app: &mut bevy_app::App) {
    let world = app.world_mut();
    world.resource_mut::<SimulationRunning>().0 = false;
    world.resource_mut::<Time<Virtual>>().pause();
    world
        .resource_mut::<Time<Fixed>>()
        .discard_overstep(std::time::Duration::MAX);
    world
        .resource_mut::<Time<Real>>()
        .update_with_instant(Instant::now());
    #[cfg(feature = "graph")]
    world.run_schedule(oxurack_graph::GraphMaintenance);
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxurack_core::{
        CoreError, OxurackModule, ParameterSchema, ParameterValue, Port, TickError, Value,
    };
    use oxurack_mod_clock::ClockModule;
    use oxurack_mod_lfo::LfoModule;
    use oxurack_mod_turingmachine::TuringMachineModule;
    use std::collections::HashMap;

    trait MessageValuesExt<M: bevy_ecs::message::Message + Clone> {
        fn iter(&self) -> std::vec::IntoIter<M>;
    }

    impl<M: bevy_ecs::message::Message + Clone> MessageValuesExt<M> for bevy_ecs::message::Messages<M> {
        fn iter(&self) -> std::vec::IntoIter<M> {
            let mut cursor = self.get_cursor();
            cursor.read(self).cloned().collect::<Vec<_>>().into_iter()
        }
    }

    type OwnerMembership = (
        Vec<(u64, String, String)>,
        Vec<(u64, u64, String, String, String)>,
        Vec<(
            u64,
            u64,
            u64,
            Option<oxurack_core::CableTransform>,
            bool,
            usize,
        )>,
        Vec<(String, u64)>,
        Vec<(u64, Vec<u64>)>,
        Vec<(u64, Vec<u64>)>,
    );

    fn owner_membership(world: &bevy_ecs::world::World) -> OwnerMembership {
        let mut query = world
            .try_query::<(bevy_ecs::entity::Entity, &oxurack_core::Module)>()
            .unwrap();
        let mut modules: Vec<_> = query
            .iter(world)
            .map(|(entity, module)| {
                (
                    entity.to_bits(),
                    module.instance_name.clone(),
                    module.kind.to_string(),
                )
            })
            .collect();
        modules.sort();
        let mut query = world
            .try_query::<(
                bevy_ecs::entity::Entity,
                &Port,
                &bevy_ecs::hierarchy::ChildOf,
            )>()
            .unwrap();
        let mut ports: Vec<_> = query
            .iter(world)
            .map(|(entity, port, parent)| {
                (
                    entity.to_bits(),
                    parent.parent().to_bits(),
                    port.name.as_ref().to_owned(),
                    format!("{:?}", port.direction),
                    format!("{:?}", port.value_kind),
                )
            })
            .collect();
        ports.sort();
        let mut query = world
            .try_query::<(
                bevy_ecs::entity::Entity,
                &oxurack_core::Cable,
                &oxurack_core::CableOrder,
            )>()
            .unwrap();
        let mut cables: Vec<_> = query
            .iter(world)
            .map(|(entity, cable, order)| {
                (
                    entity.to_bits(),
                    cable.source_port.to_bits(),
                    cable.target_port.to_bits(),
                    cable.transform,
                    cable.enabled,
                    order.0,
                )
            })
            .collect();
        cables.sort_by_key(|row| row.0);
        let mut handles: Vec<_> = world
            .resource::<oxurack_core::PatchHandle>()
            .modules
            .iter()
            .map(|(name, entity)| (name.clone(), entity.to_bits()))
            .collect();
        handles.sort();
        let index = world.resource::<oxurack_core::CableIndex>();
        let mut from: Vec<_> = ports
            .iter()
            .map(|(entity, ..)| {
                (
                    *entity,
                    index
                        .cables_from(bevy_ecs::entity::Entity::from_bits(*entity))
                        .iter()
                        .map(|cable| cable.to_bits())
                        .collect(),
                )
            })
            .collect();
        let mut to: Vec<_> = ports
            .iter()
            .map(|(entity, ..)| {
                (
                    *entity,
                    index
                        .cables_targeting(bevy_ecs::entity::Entity::from_bits(*entity))
                        .iter()
                        .map(|cable| cable.to_bits())
                        .collect(),
                )
            })
            .collect();
        from.sort();
        to.sort();
        (modules, ports, cables, handles, from, to)
    }

    #[cfg(feature = "graph")]
    type GraphMembership = (
        Vec<(String, String)>,
        Vec<(String, String, String, String, bool)>,
        Vec<(String, bool)>,
        Vec<(String, Vec<String>, Vec<String>)>,
    );

    #[cfg(feature = "graph")]
    fn graph_membership(world: &bevy_ecs::world::World) -> GraphMembership {
        let view = oxurack_graph::GraphView::new(world);
        let modules = view
            .modules()
            .into_iter()
            .map(|module| (module.instance_name, module.kind))
            .collect();
        let cables = view
            .cables()
            .into_iter()
            .map(|cable| {
                let entity = bevy_ecs::entity::Entity::from_bits(cable.entity_bits);
                (
                    cable.source_module_instance,
                    cable.source_port_name,
                    cable.target_module_instance,
                    cable.target_port_name,
                    view.contains_cable(entity),
                )
            })
            .collect();
        let mut memberships: Vec<_> = world
            .resource::<oxurack_core::PatchHandle>()
            .modules
            .iter()
            .map(|(name, entity)| (name.clone(), view.contains_module(*entity)))
            .collect();
        memberships.sort();
        let mut ports: Vec<_> = world
            .resource::<oxurack_core::PatchHandle>()
            .modules
            .iter()
            .map(|(name, entity)| {
                let port_set = view.ports_of(*entity);
                (
                    name.clone(),
                    port_set
                        .inputs
                        .into_iter()
                        .map(|port| {
                            format!("{}:{:?}:{}", port.name, port.direction, port.value_kind)
                        })
                        .collect(),
                    port_set
                        .outputs
                        .into_iter()
                        .map(|port| {
                            format!("{}:{:?}:{}", port.name, port.direction, port.value_kind)
                        })
                        .collect(),
                )
            })
            .collect();
        ports.sort();
        (modules, cables, memberships, ports)
    }

    struct CandidateOutput;
    struct CandidateFault;
    struct CandidateLateFailure;

    #[derive(bevy_ecs::prelude::Resource)]
    struct CandidateMarker;

    impl OxurackModule for CandidateOutput {
        const KIND: &'static str = "candidate_output";
        const DISPLAY_NAME: &'static str = "Candidate output probe";
        fn port_schema() -> &'static [oxurack_core::PortSchema] {
            &[]
        }
        fn parameter_schema() -> &'static [ParameterSchema] {
            &[]
        }
        fn spawn(
            world: &mut bevy_ecs::world::World,
            _name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<bevy_ecs::entity::Entity, CoreError> {
            let ports: Vec<_> = {
                let mut query = world.query::<(
                    bevy_ecs::entity::Entity,
                    &Port,
                    &bevy_ecs::hierarchy::ChildOf,
                )>();
                query
                    .iter(world)
                    .filter_map(|(entity, port, parent)| {
                        let module = world.get::<oxurack_core::Module>(parent.parent())?;
                        (module.instance_name == "candidate_sink")
                            .then(|| (entity, port.name.as_ref().to_owned()))
                    })
                    .collect()
            };
            for (entity, name) in ports {
                if let Some(mut value) = world.get_mut::<oxurack_core::CurrentValue>(entity) {
                    value.0 = match name.as_str() {
                        "step" | "note_valid" | "velocity_valid" | "gate" => Value::Gate(true),
                        "note" => Value::Raw(60),
                        "velocity" => Value::Raw(100),
                        _ => value.0,
                    };
                }
            }
            world.resource_mut::<SimulationRunning>().0 = true;
            world.run_schedule(bevy_app::prelude::FixedUpdate);
            Ok(world.spawn_empty().id())
        }
    }

    impl OxurackModule for CandidateFault {
        const KIND: &'static str = "candidate_fault";
        const DISPLAY_NAME: &'static str = "Candidate fault probe";
        fn port_schema() -> &'static [oxurack_core::PortSchema] {
            &[]
        }
        fn parameter_schema() -> &'static [ParameterSchema] {
            &[]
        }
        fn spawn(
            world: &mut bevy_ecs::world::World,
            _name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<bevy_ecs::entity::Entity, CoreError> {
            world
                .resource_mut::<MidiOutputBuffer>()
                .latch_fault(TickError::OutputCapacity {
                    reason: "candidate fault witness".into(),
                    completed_samples: 7,
                });
            Ok(world.spawn_empty().id())
        }
    }

    impl OxurackModule for CandidateLateFailure {
        const KIND: &'static str = "candidate_late_failure";
        const DISPLAY_NAME: &'static str = "Candidate late failure probe";
        fn port_schema() -> &'static [oxurack_core::PortSchema] {
            &[]
        }
        fn parameter_schema() -> &'static [ParameterSchema] {
            &[]
        }
        fn spawn(
            world: &mut bevy_ecs::world::World,
            name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<bevy_ecs::entity::Entity, CoreError> {
            world.insert_resource(CandidateMarker);
            let entity = world.spawn_empty().id();
            world.write_message(oxurack_core::ModuleAdded {
                entity,
                instance_name: name.into(),
                kind: Self::KIND.into(),
            });
            Err(CoreError::NotImplemented("candidate late failure witness"))
        }
    }

    const PATCH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/patches/turing-machine-midi.ron"
    );
    const LFO_PATCH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/patches/turing-machine-lfo.ron"
    );

    fn removal_test_rack() -> Oxurack {
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<LfoModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<oxurack_mod_midiout::MidiOutModule>()
            .unwrap();
        rack.load_patch(Path::new(LFO_PATCH)).unwrap();
        rack
    }

    #[test]
    fn removal_despawns_owned_grandchildren() {
        let mut rack = removal_test_rack();
        let module = rack
            .app
            .world()
            .resource::<oxurack_core::PatchHandle>()
            .modules["lfo_main"];
        let child = rack.app.world_mut().spawn_empty().id();
        let grandchild = rack.app.world_mut().spawn_empty().id();
        rack.app.world_mut().entity_mut(module).add_child(child);
        rack.app.world_mut().entity_mut(child).add_child(grandchild);
        rack.remove_module("lfo_main").unwrap();
        assert!(rack.app.world().get_entity(child).is_err());
        assert!(rack.app.world().get_entity(grandchild).is_err());
    }

    #[test]
    fn removal_preflight_rejects_nested_active_module_without_messages() {
        let mut rack = removal_test_rack();
        let lfo = rack
            .app
            .world()
            .resource::<oxurack_core::PatchHandle>()
            .modules["lfo_main"];
        let tm = rack
            .app
            .world()
            .resource::<oxurack_core::PatchHandle>()
            .modules["tm_main"];
        rack.app.world_mut().entity_mut(lfo).add_child(tm);
        let cable_events = rack
            .app
            .world()
            .resource::<bevy_ecs::message::Messages<oxurack_core::CableDisconnected>>()
            .len();
        let module_events = rack
            .app
            .world()
            .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleRemoved>>()
            .len();
        assert!(matches!(
            rack.remove_module("lfo_main"),
            Err(CoreError::InvalidModuleOwnership { .. })
        ));
        assert!(rack.app.world().get_entity(lfo).is_ok());
        assert!(rack.app.world().get_entity(tm).is_ok());
        assert_eq!(
            rack.app
                .world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::CableDisconnected>>()
                .len(),
            cable_events
        );
        assert_eq!(
            rack.app
                .world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleRemoved>>()
                .len(),
            module_events
        );
    }

    #[test]
    fn reset_preserves_first_fault_and_replacement_clears_it() {
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<LfoModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<oxurack_mod_midiout::MidiOutModule>()
            .unwrap();
        rack.load_patch(Path::new(PATCH)).unwrap();
        let original = TickError::OutputCapacity {
            reason: "controlled lifecycle fault fixture".into(),
            completed_samples: 1,
        };
        rack.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .latch_fault(original.clone());
        rack.reset().unwrap();
        assert!(matches!(
            rack.midi_output_error(),
            Some(TickError::OutputCapacity { reason, completed_samples: 1 })
                if reason == "controlled lifecycle fault fixture"
        ));
        assert!(
            rack.load_patch(Path::new("/tmp/oxurack-lifecycle-missing-patch.ron"))
                .is_err()
        );
        assert!(matches!(
            rack.midi_output_error(),
            Some(TickError::OutputCapacity { reason, completed_samples: 1 })
                if reason == "controlled lifecycle fault fixture"
        ));
        rack.load_patch(Path::new(PATCH)).unwrap();
        assert!(rack.midi_output_error().is_none());
        assert_eq!(rack.sample_time().completed_samples(), 0);
    }

    #[test]
    fn in_place_removal_preserves_the_first_output_fault() {
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<LfoModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<oxurack_mod_midiout::MidiOutModule>()
            .unwrap();
        rack.load_patch(Path::new(LFO_PATCH)).unwrap();
        rack.set_driver_mode(crate::SimulationDriver::Manual)
            .start();
        for _ in 0..300 {
            rack.advance_samples(1).unwrap();
            drop(rack.drain_midi());
        }
        rack.stop();
        let first = TickError::OutputCapacity {
            reason: "retirement fault witness".into(),
            completed_samples: 300,
        };
        rack.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .latch_fault(first.clone());
        rack.remove_module("midi_main").unwrap();
        let first_debug = format!("{first:?}");
        assert_eq!(
            rack.midi_output_error().map(|fault| format!("{fault:?}")),
            Some(first_debug.clone())
        );
        let cleanup: Vec<_> = rack.drain_midi().collect();
        assert_eq!(cleanup.len(), 3);
        assert_eq!(
            rack.midi_output_error().map(|fault| format!("{fault:?}")),
            Some(first_debug)
        );
    }

    #[test]
    fn candidate_output_and_fault_rejection_preserve_old_owner_and_success_messages_are_typed() {
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<LfoModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<oxurack_mod_midiout::MidiOutModule>()
            .unwrap();
        rack.try_install::<CandidateOutput>().unwrap();
        rack.try_install::<CandidateFault>().unwrap();
        rack.try_install::<CandidateLateFailure>().unwrap();
        rack.set_driver_mode(crate::SimulationDriver::Manual);
        rack.load_patch(Path::new(LFO_PATCH)).unwrap();
        rack.configure_scope(&[crate::ScopeSelector::new("lfo_main", "bipolar_out")], 128)
            .unwrap();
        rack.start();
        for _ in 0..3 {
            rack.advance_samples(1).unwrap();
            for _event in rack.drain_midi() {}
        }
        rack.stop();
        let before_snapshot = rack.snapshot_patch().unwrap();
        let before_membership = owner_membership(rack.world());
        #[cfg(feature = "graph")]
        let before_graph_membership = graph_membership(rack.world());
        let before_sample = rack.sample_time();
        let before_driver = rack.driver_mode();
        let before_pending = rack
            .app
            .world()
            .resource::<MidiOutputBuffer>()
            .pending_count();
        let before_install_count = rack.install_order.len();
        let before_scope = {
            let scope = rack.scope().unwrap();
            (
                scope.capacity(),
                scope.overwritten(),
                scope.selectors().to_vec(),
                scope.records().copied().collect::<Vec<_>>(),
            )
        };
        let before_module_messages: Vec<_> = rack
            .world()
            .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleAdded>>()
            .iter()
            .map(|message| (message.instance_name.clone(), message.kind.to_string()))
            .collect();
        let before_cable_messages: Vec<_> = rack
            .world()
            .resource::<bevy_ecs::message::Messages<oxurack_core::CableConnected>>()
            .iter()
            .map(|message| {
                (
                    message.source_port,
                    message.target_port,
                    message.source.clone(),
                )
            })
            .collect();
        let before_parameter_messages: Vec<_> = rack
            .world()
            .resource::<bevy_ecs::message::Messages<oxurack_core::ParameterChanged>>()
            .iter()
            .map(|message| {
                (
                    message.instance_name.clone(),
                    message.param.to_string(),
                    message.new.clone(),
                    message.source.clone(),
                )
            })
            .collect();
        let before_label_messages: Vec<_> = rack
            .world()
            .resource::<bevy_ecs::message::Messages<oxurack_core::LabelChanged>>()
            .iter()
            .map(|message| {
                (
                    message.instance_name.clone(),
                    message.kind.clone(),
                    message.new.clone(),
                    message.source.clone(),
                )
            })
            .collect();
        let before_loaded_messages: Vec<_> = rack
            .world()
            .resource::<bevy_ecs::message::Messages<oxurack_core::PatchLoaded>>()
            .iter()
            .map(|message| {
                (
                    message.patch_name.clone(),
                    message.source.clone(),
                    message.module_count,
                    message.cable_count,
                    message.label_count,
                )
            })
            .collect();
        let mut late_candidate = oxurack_core::load_patch_from_file(Path::new(LFO_PATCH)).unwrap();
        late_candidate.modules.push(crate::ModuleConfig {
            kind: CandidateLateFailure::KIND.into(),
            instance_name: "late_failure".into(),
            ..Default::default()
        });
        let late_path =
            std::env::temp_dir().join(format!("oxurack-candidate-late-{}.ron", std::process::id()));
        oxurack_core::save_patch_to_file(&late_candidate, &late_path).unwrap();
        assert!(matches!(
            rack.load_patch(&late_path),
            Err(CoreError::NotImplemented("candidate late failure witness"))
        ));
        assert!(rack.world().get_resource::<CandidateMarker>().is_none());
        assert_eq!(owner_membership(rack.world()), before_membership);
        #[cfg(feature = "graph")]
        assert_eq!(graph_membership(rack.world()), before_graph_membership);
        assert_eq!(rack.driver_mode(), before_driver);
        assert_eq!(
            rack.app
                .world()
                .resource::<MidiOutputBuffer>()
                .pending_count(),
            before_pending
        );
        assert_eq!(rack.install_order.len(), before_install_count);
        let scope = rack.scope().unwrap();
        assert_eq!(
            (
                scope.capacity(),
                scope.overwritten(),
                scope.selectors().to_vec(),
                scope.records().copied().collect::<Vec<_>>()
            ),
            before_scope
        );
        assert_eq!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleAdded>>()
                .iter()
                .map(|message| (message.instance_name.clone(), message.kind.to_string()))
                .collect::<Vec<_>>(),
            before_module_messages
        );
        assert_eq!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::CableConnected>>()
                .iter()
                .map(|message| (
                    message.source_port,
                    message.target_port,
                    message.source.clone()
                ))
                .collect::<Vec<_>>(),
            before_cable_messages
        );
        assert_eq!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::ParameterChanged>>()
                .iter()
                .map(|message| (
                    message.instance_name.clone(),
                    message.param.to_string(),
                    message.new.clone(),
                    message.source.clone()
                ))
                .collect::<Vec<_>>(),
            before_parameter_messages
        );
        assert_eq!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::LabelChanged>>()
                .iter()
                .map(|message| (
                    message.instance_name.clone(),
                    message.kind.clone(),
                    message.new.clone(),
                    message.source.clone()
                ))
                .collect::<Vec<_>>(),
            before_label_messages
        );
        assert_eq!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::PatchLoaded>>()
                .iter()
                .map(|message| (
                    message.patch_name.clone(),
                    message.source.clone(),
                    message.module_count,
                    message.cable_count,
                    message.label_count
                ))
                .collect::<Vec<_>>(),
            before_loaded_messages
        );
        let _ = std::fs::remove_file(late_path);
        let mut candidate = oxurack_core::load_patch_from_file(Path::new(LFO_PATCH)).unwrap();
        let mut candidate_sink = crate::ModuleConfig {
            kind: oxurack_mod_midiout::MidiOutModule::KIND.into(),
            instance_name: "candidate_sink".into(),
            ..Default::default()
        };
        candidate_sink
            .parameters
            .insert("channel".into(), ParameterValue::Int(1));
        candidate.modules.push(candidate_sink);
        candidate.modules.push(crate::ModuleConfig {
            kind: CandidateOutput::KIND.into(),
            instance_name: "candidate_output".into(),
            ..Default::default()
        });
        let output_path = std::env::temp_dir().join(format!(
            "oxurack-candidate-output-{}.ron",
            std::process::id()
        ));
        oxurack_core::save_patch_to_file(&candidate, &output_path).unwrap();
        let output_result = rack.load_patch(&output_path);
        assert!(
            matches!(
                &output_result,
                Err(CoreError::CandidateMidiNotEmpty { count: 1 })
            ),
            "unexpected output candidate result: {output_result:?}"
        );
        assert_eq!(rack.snapshot_patch().unwrap(), before_snapshot);
        assert_eq!(owner_membership(rack.world()), before_membership);
        assert_eq!(rack.sample_time(), before_sample);
        assert_eq!(
            {
                let scope = rack.scope().unwrap();
                (
                    scope.capacity(),
                    scope.overwritten(),
                    scope.selectors().to_vec(),
                    scope.records().copied().collect::<Vec<_>>(),
                )
            },
            before_scope
        );
        assert_eq!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleAdded>>()
                .iter()
                .map(|message| (message.instance_name.clone(), message.kind.to_string()))
                .collect::<Vec<_>>(),
            before_module_messages
        );
        assert_eq!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::CableConnected>>()
                .iter()
                .map(|message| (
                    message.source_port,
                    message.target_port,
                    message.source.clone()
                ))
                .collect::<Vec<_>>(),
            before_cable_messages
        );
        assert_eq!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::ParameterChanged>>()
                .iter()
                .map(|message| (
                    message.instance_name.clone(),
                    message.param.to_string(),
                    message.new.clone(),
                    message.source.clone()
                ))
                .collect::<Vec<_>>(),
            before_parameter_messages
        );
        assert_eq!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::LabelChanged>>()
                .iter()
                .map(|message| (
                    message.instance_name.clone(),
                    message.kind.clone(),
                    message.new.clone(),
                    message.source.clone()
                ))
                .collect::<Vec<_>>(),
            before_label_messages
        );
        assert_eq!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::PatchLoaded>>()
                .iter()
                .map(|message| (
                    message.patch_name.clone(),
                    message.source.clone(),
                    message.module_count,
                    message.cable_count,
                    message.label_count
                ))
                .collect::<Vec<_>>(),
            before_loaded_messages
        );
        assert!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleRemoved>>()
                .is_empty()
        );
        assert!(
            rack.world()
                .resource::<bevy_ecs::message::Messages<oxurack_core::CableDisconnected>>()
                .is_empty()
        );

        let mut fault_candidate = oxurack_core::load_patch_from_file(Path::new(LFO_PATCH)).unwrap();
        fault_candidate.modules.push(crate::ModuleConfig {
            kind: CandidateFault::KIND.into(),
            instance_name: "candidate_fault".into(),
            ..Default::default()
        });
        let fault_path = std::env::temp_dir().join(format!(
            "oxurack-candidate-fault-{}.ron",
            std::process::id()
        ));
        oxurack_core::save_patch_to_file(&fault_candidate, &fault_path).unwrap();
        assert!(
            matches!(rack.load_patch(&fault_path), Err(CoreError::Tick(TickError::OutputCapacity { reason, completed_samples: 7 })) if reason == "candidate fault witness")
        );
        assert_eq!(rack.snapshot_patch().unwrap(), before_snapshot);
        assert_eq!(owner_membership(rack.world()), before_membership);
        assert_eq!(rack.sample_time(), before_sample);
        let _ = std::fs::remove_file(output_path);
        let _ = std::fs::remove_file(fault_path);

        let mut labeled = oxurack_core::load_patch_from_file(Path::new(LFO_PATCH)).unwrap();
        let tm = labeled
            .modules
            .iter_mut()
            .find(|module| module.instance_name == "tm_main")
            .unwrap();
        tm.display_name = "Sequence".into();
        tm.description = "Seeded sequence source".into();
        tm.tags = vec!["sequence".into()];
        let success_path = std::env::temp_dir().join(format!(
            "oxurack-candidate-success-{}.ron",
            std::process::id()
        ));
        oxurack_core::save_patch_to_file(&labeled, &success_path).unwrap();
        rack.load_patch(&success_path).unwrap();
        let world = rack.world();
        let modules: Vec<_> = world
            .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleAdded>>()
            .iter()
            .collect();
        assert_eq!(modules.len(), labeled.modules.len());
        let active_handle = world.resource::<oxurack_core::PatchHandle>();
        for message in &modules {
            assert_eq!(
                active_handle.modules.get(&message.instance_name),
                Some(&message.entity)
            );
            let module = world.get::<oxurack_core::Module>(message.entity).unwrap();
            assert_eq!(message.kind, module.kind);
        }
        let cables: Vec<_> = world
            .resource::<bevy_ecs::message::Messages<oxurack_core::CableConnected>>()
            .iter()
            .collect();
        assert_eq!(cables.len(), labeled.cables.len());
        for message in &cables {
            assert_eq!(message.source, oxurack_core::ChangeSource::PatchLoad);
            let cable = world
                .get::<oxurack_core::Cable>(message.cable_entity)
                .unwrap();
            assert_eq!(cable.source_port, message.source_port);
            assert_eq!(cable.target_port, message.target_port);
        }
        let parameters: Vec<_> = world
            .resource::<bevy_ecs::message::Messages<oxurack_core::ParameterChanged>>()
            .iter()
            .collect();
        let expected_parameter_count: usize = labeled
            .modules
            .iter()
            .map(|module| module.parameters.len())
            .sum();
        assert_eq!(parameters.len(), expected_parameter_count);
        for message in &parameters {
            assert_eq!(message.source, oxurack_core::ChangeSource::PatchLoad);
            assert!(message.previous.is_none());
            assert_eq!(
                active_handle.modules.get(&message.instance_name),
                Some(&message.entity)
            );
            let module = labeled
                .modules
                .iter()
                .find(|module| module.instance_name == message.instance_name)
                .unwrap();
            assert_eq!(
                module.parameters.get(&message.param.to_string()),
                Some(&message.new)
            );
        }
        let labels: Vec<_> = world
            .resource::<bevy_ecs::message::Messages<oxurack_core::LabelChanged>>()
            .iter()
            .collect();
        let label_count = labels.len();
        assert_eq!(label_count, 3);
        assert!(
            labels
                .iter()
                .all(|message| message.source == oxurack_core::ChangeSource::PatchLoad)
        );
        assert!(
            labels
                .iter()
                .any(|message| message.instance_name == "tm_main"
                    && message.kind == oxurack_core::LabelKind::Description
                    && message.new
                        == oxurack_core::LabelValue::Text("Seeded sequence source".into()))
        );
        assert!(labels.iter().any(|message| message.instance_name == "tm_main" && message.kind == oxurack_core::LabelKind::Tags && matches!(&message.new, oxurack_core::LabelValue::Tags(tags) if tags.iter().any(|tag| tag.as_str() == "sequence"))));
        assert!(
            labels
                .iter()
                .any(|message| message.instance_name == "tm_main"
                    && message.kind == oxurack_core::LabelKind::DisplayName
                    && message.new == oxurack_core::LabelValue::Text("Sequence".into()))
        );
        let loaded: Vec<_> = world
            .resource::<bevy_ecs::message::Messages<oxurack_core::PatchLoaded>>()
            .iter()
            .collect();
        assert_eq!(loaded.len(), 1);
        assert_eq!(
            loaded[0].patch_name,
            success_path.file_stem().unwrap().to_str().unwrap()
        );
        assert_eq!(
            loaded[0].source,
            oxurack_core::PatchSource::File(success_path.clone())
        );
        assert_eq!(
            (
                loaded[0].module_count,
                loaded[0].cable_count,
                loaded[0].label_count
            ),
            (labeled.modules.len(), labeled.cables.len(), label_count)
        );
        assert!(
            world
                .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleRemoved>>()
                .is_empty()
        );
        assert!(
            world
                .resource::<bevy_ecs::message::Messages<oxurack_core::CableDisconnected>>()
                .is_empty()
        );
        for _event in rack.drain_midi() {}
        rack.restart().unwrap();
        let world = rack.world();
        let restarted: Vec<_> = world
            .resource::<bevy_ecs::message::Messages<oxurack_core::PatchLoaded>>()
            .iter()
            .collect();
        assert_eq!(restarted.len(), 1);
        assert_eq!(restarted[0].source, oxurack_core::PatchSource::InMemory);
        assert_eq!(restarted[0].patch_name, "");
        assert_eq!(
            (
                restarted[0].module_count,
                restarted[0].cable_count,
                restarted[0].label_count
            ),
            (labeled.modules.len(), labeled.cables.len(), label_count)
        );
        assert!(
            world
                .resource::<bevy_ecs::message::Messages<oxurack_core::ModuleRemoved>>()
                .is_empty()
        );
        assert!(
            world
                .resource::<bevy_ecs::message::Messages<oxurack_core::CableDisconnected>>()
                .is_empty()
        );
        let _ = std::fs::remove_file(success_path);
    }
}
