//! Read and persist the configuration currently represented by the live rack.

use std::collections::HashMap;
use std::path::Path;

use bevy_ecs::hierarchy::ChildOf;
use bevy_ecs::prelude::Entity;
use oxurack_core::{
    Cable, CableConfig, CableOrder, CoreError, Description, DisplayName, Module, ModuleConfig,
    ParameterValue, Patch, PatchError, PatchHandle, RackConfig, RackMeta, Tags,
};

pub(super) struct LoadedIdentity {
    version: String,
    master_seed: u64,
    bpm: f32,
}

impl LoadedIdentity {
    pub(super) fn from_patch(patch: &Patch) -> Self {
        Self {
            version: patch.version.clone(),
            master_seed: patch.master_seed,
            bpm: patch.bpm,
        }
    }

    pub(super) fn bpm(&self) -> f32 {
        self.bpm
    }
    pub(super) fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
    }
}

impl super::Oxurack {
    /// Reads a parameter from the named live module instance.
    ///
    /// The value comes from its registered getter, including while the rack is
    /// running. This call does not advance or otherwise mutate the rack.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::NoPatchLoaded`] or [`CoreError::UnknownModule`] if
    /// the active patch or instance is absent, and propagates getter errors.
    pub fn parameter(&self, module: &str, param: &str) -> Result<ParameterValue, CoreError> {
        let world = self.app.world();
        let entity = active_module(world, module)?;
        let component = world
            .get::<Module>(entity)
            .ok_or_else(|| CoreError::UnknownModule(module.to_owned()))?;
        world
            .resource::<oxurack_core::ParameterRegistry>()
            .get_parameter(world, entity, &component.kind, param)
            .map_err(|error| normalize_parameter_error(error, module))
    }

    /// Sets a parameter on the named live module instance.
    ///
    /// A successful write emits one [`oxurack_core::ParameterChanged`] message
    /// with [`oxurack_core::ChangeSource::DirectApi`].
    ///
    /// This call is synchronous and does not tick the rack. The module's
    /// existing setter controls validation and state-change semantics.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::NoPatchLoaded`], [`CoreError::UnknownModule`],
    /// [`CoreError::UnknownParameter`], or the module's validation error.
    pub fn set_parameter(
        &mut self,
        module: &str,
        param: &str,
        value: ParameterValue,
    ) -> Result<(), CoreError> {
        self.set_parameter_with_source(module, param, value, oxurack_core::ChangeSource::DirectApi)
    }

    pub(super) fn set_parameter_with_source(
        &mut self,
        module: &str,
        param: &str,
        value: ParameterValue,
        source: oxurack_core::ChangeSource,
    ) -> Result<(), CoreError> {
        active_module(self.app.world(), module)?;
        oxurack_core::set_module_parameter(self.app.world_mut(), module, param, &value, source)
    }

    /// Returns the current loaded patch configuration, including live parameter
    /// values and current labels and cables. Runtime phase is not captured.
    ///
    /// This call does not advance time, drain MIDI, clear Scope, or reset
    /// modules. Its returned patch is owned by the caller.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::NoPatchLoaded`] without an active patch, getter
    /// errors for incomplete parameter registrations, or
    /// [`oxurack_core::PatchError::UnsupportedSnapshot`] for selected live state that the
    /// patch format cannot represent. The completed patch is validated before
    /// it is returned.
    pub fn snapshot_patch(&mut self) -> Result<Patch, CoreError> {
        let identity = self
            .loaded_identity
            .as_ref()
            .ok_or(CoreError::NoPatchLoaded)?;
        let handle = self
            .app
            .world()
            .get_resource::<PatchHandle>()
            .ok_or(CoreError::NoPatchLoaded)?
            .clone();
        let world = self.app.world_mut();
        let mut modules = Vec::with_capacity(handle.modules.len());
        let mut active_entities = HashMap::with_capacity(handle.modules.len());

        let mut named_entities: Vec<_> = handle.modules.iter().collect();
        named_entities.sort_by_key(|(name, _)| (*name).clone());
        for (name, entity) in named_entities {
            let module = world
                .get::<Module>(*entity)
                .ok_or_else(|| CoreError::UnknownModule(name.clone()))?;
            if module.instance_name != *name {
                return Err(unsupported(format!(
                    "PatchHandle name '{name}' points to Module instance '{}'",
                    module.instance_name
                )));
            }
            let kind_text = module.kind.as_ref().to_owned();
            let kind = module.kind.clone();
            active_entities.insert(*entity, name.clone());

            let schemas = world
                .resource::<oxurack_core::ModuleRegistry>()
                .get(&kind)
                .ok_or_else(|| {
                    CoreError::Patch(PatchError::UnknownModuleKind(kind.as_ref().to_owned()))
                })?
                .parameter_schemas
                .iter()
                .map(|schema| schema.name.to_owned())
                .collect::<Vec<_>>();
            let mut parameters = HashMap::with_capacity(schemas.len());
            for parameter in schemas {
                let value =
                    world.resource_scope(
                        |world,
                         registry: bevy_ecs::change_detection::Mut<
                            oxurack_core::ParameterRegistry,
                        >| {
                            registry.get_parameter(world, *entity, &kind, &parameter)
                        },
                    );
                let value = value.map_err(|error| normalize_parameter_error(error, name))?;
                parameters.insert(parameter, value);
            }

            modules.push(ModuleConfig {
                kind: kind_text,
                instance_name: name.clone(),
                parameters,
                display_name: world
                    .get::<DisplayName>(*entity)
                    .map_or_else(String::new, |label| label.0.clone()),
                description: world
                    .get::<Description>(*entity)
                    .map_or_else(String::new, |label| label.0.clone()),
                tags: world.get::<Tags>(*entity).map_or_else(Vec::new, |tags| {
                    tags.0.iter().map(|tag| tag.as_str().to_owned()).collect()
                }),
            });
        }

        let mut port_owners = HashMap::new();
        let mut ports = world.query::<(Entity, &oxurack_core::Port, &ChildOf)>();
        for (port_entity, port, child) in ports.iter(world) {
            port_owners.insert(
                port_entity,
                (
                    child.0,
                    active_entities.get(&child.0).cloned(),
                    port.name.as_ref().to_owned(),
                ),
            );
        }

        let mut cables = Vec::new();
        let mut cables_query = world.query::<(Entity, &Cable, Option<&CableOrder>)>();
        for (cable_entity, cable, order) in cables_query.iter(world) {
            let source = port_owners.get(&cable.source_port).ok_or_else(|| {
                unsupported(format!(
                    "cable {:?} has a missing source port",
                    cable_entity
                ))
            })?;
            let target = port_owners.get(&cable.target_port).ok_or_else(|| {
                unsupported(format!(
                    "cable {:?} has a missing target port",
                    cable_entity
                ))
            })?;
            match (&source.1, &target.1) {
                (Some(source_name), Some(target_name)) => {
                    if !cable.enabled {
                        return Err(unsupported(format!(
                            "cable {:?} between '{}.{}' and '{}.{}' is disabled",
                            cable_entity, source_name, source.2, target_name, target.2
                        )));
                    }
                    let source = (source_name.clone(), source.2.clone());
                    let target = (target_name.clone(), target.2.clone());
                    let (absent_order, ordinal) = match order {
                        Some(order) => (false, order.0),
                        None => (true, 0),
                    };
                    cables.push((
                        (
                            target.0.clone(),
                            target.1.clone(),
                            source.0.clone(),
                            source.1.clone(),
                            absent_order,
                            ordinal,
                            cable_entity,
                        ),
                        CableConfig {
                            source: source.clone(),
                            target: target.clone(),
                            transform: cable.transform,
                            display_name: world
                                .get::<DisplayName>(cable_entity)
                                .map_or_else(String::new, |label| label.0.clone()),
                            description: world
                                .get::<Description>(cable_entity)
                                .map_or_else(String::new, |label| label.0.clone()),
                            tags: world
                                .get::<Tags>(cable_entity)
                                .map_or_else(Vec::new, |tags| {
                                    tags.0.iter().map(|tag| tag.as_str().to_owned()).collect()
                                }),
                        },
                    ));
                }
                (None, None) => {
                    // Cables belonging to superseded patch entities are outside
                    // the active PatchHandle and are intentionally ignored.
                }
                _ => {
                    let (active_endpoint, external_endpoint) = if source.1.is_some() {
                        (source, target)
                    } else {
                        (target, source)
                    };
                    let Some(active_name) = active_endpoint.1.as_deref() else {
                        return Err(unsupported(format!(
                            "cable {:?} has unresolved active endpoint identity",
                            cable_entity
                        )));
                    };
                    let external_name = world.get::<Module>(external_endpoint.0).map_or_else(
                        || "<missing module>".to_owned(),
                        |module| module.instance_name.clone(),
                    );
                    return Err(unsupported(format!(
                        "cable {:?} crosses active PatchHandle from '{}.{}' to '{}.{}'",
                        cable_entity,
                        active_name,
                        active_endpoint.2,
                        external_name,
                        external_endpoint.2
                    )));
                }
            }
        }
        cables.sort_by(|left, right| left.0.cmp(&right.0));

        let rack = world.get_resource::<RackMeta>().and_then(rack_config);
        let patch = Patch {
            version: identity.version.clone(),
            master_seed: identity.master_seed,
            bpm: identity.bpm,
            modules,
            cables: cables.into_iter().map(|(_, cable)| cable).collect(),
            rack,
        };
        oxurack_core::validate_patch(&patch, world.resource::<oxurack_core::ModuleRegistry>())?;
        Ok(patch)
    }

    /// Saves the current configuration to a RON patch file.
    ///
    /// The patch is fully extracted before the destination is opened. File
    /// creation and overwrite behavior follows [`oxurack_core::save_patch_to_file`].
    ///
    /// # Errors
    ///
    /// Propagates snapshot, serialization, and I/O errors. Existing file-write
    /// semantics do not promise atomic replacement or crash-safe syncing.
    pub fn save_patch(&mut self, path: &Path) -> Result<(), CoreError> {
        let patch = self.snapshot_patch()?;
        oxurack_core::save_patch_to_file(&patch, path)
    }
}

fn active_module(world: &bevy_ecs::world::World, name: &str) -> Result<Entity, CoreError> {
    world
        .get_resource::<PatchHandle>()
        .ok_or(CoreError::NoPatchLoaded)?
        .modules
        .get(name)
        .copied()
        .ok_or_else(|| CoreError::UnknownModule(name.to_owned()))
}

fn normalize_parameter_error(error: CoreError, instance_name: &str) -> CoreError {
    match error {
        CoreError::UnknownParameter { param, .. } => CoreError::UnknownParameter {
            module: instance_name.to_owned(),
            param,
        },
        CoreError::InvalidParameterValue { param, reason, .. } => {
            CoreError::InvalidParameterValue {
                module: instance_name.to_owned(),
                param,
                reason,
            }
        }
        other => other,
    }
}

fn rack_config(meta: &RackMeta) -> Option<RackConfig> {
    let config = RackConfig {
        display_name: meta.display_name.clone(),
        description: meta.description.clone(),
        notes: meta.notes.clone(),
        tags: meta
            .tags
            .iter()
            .map(|tag| tag.as_str().to_owned())
            .collect(),
    };
    (!config.display_name.is_empty()
        || !config.description.is_empty()
        || !config.notes.is_empty()
        || !config.tags.is_empty())
    .then_some(config)
}

fn unsupported(reason: String) -> CoreError {
    CoreError::Patch(PatchError::UnsupportedSnapshot { reason })
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::App;
    use bevy_ecs::prelude::{Entity, Messages, World};
    use bevy_ecs::schedule::InternedScheduleLabel;
    use oxurack_core::{
        CableConfig, CableTransform, ChangeSource, CoreError, CurrentValue, MergePolicy,
        ModuleConfig, OxurackModule, ParameterChanged, ParameterSchema, PortDirection, PortSchema,
        SimulationRunning, TickError, Value, ValueKind,
    };
    use std::collections::HashMap;

    struct Getterless;

    struct TieSource;

    struct TieSink;

    impl OxurackModule for TieSource {
        const KIND: &'static str = "tie_source";
        const DISPLAY_NAME: &'static str = "Tie Source";

        fn port_schema() -> &'static [PortSchema] {
            &[PortSchema {
                name: "out",
                direction: PortDirection::Output,
                value_kind: ValueKind::Bipolar,
                merge_policy: MergePolicy::Reject,
                description: "Test signal source.",
            }]
        }

        fn parameter_schema() -> &'static [ParameterSchema] {
            &[]
        }

        fn spawn(
            world: &mut World,
            instance_name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<Entity, CoreError> {
            let entity = oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);
            oxurack_core::spawn_port_on_module(
                world,
                entity,
                "out",
                PortDirection::Output,
                ValueKind::Bipolar,
                MergePolicy::Reject,
            );
            Ok(entity)
        }

        fn register_systems(_app: &mut App, _schedule: InternedScheduleLabel) {}
    }

    impl OxurackModule for TieSink {
        const KIND: &'static str = "tie_sink";
        const DISPLAY_NAME: &'static str = "Tie Sink";

        fn port_schema() -> &'static [PortSchema] {
            &[PortSchema {
                name: "input",
                direction: PortDirection::Input,
                value_kind: ValueKind::Bipolar,
                merge_policy: MergePolicy::LastWins,
                description: "Test merge input.",
            }]
        }

        fn parameter_schema() -> &'static [ParameterSchema] {
            &[]
        }

        fn spawn(
            world: &mut World,
            instance_name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<Entity, CoreError> {
            let entity = oxurack_core::spawn_module_entity(world, Self::KIND, instance_name);
            oxurack_core::spawn_port_on_module(
                world,
                entity,
                "input",
                PortDirection::Input,
                ValueKind::Bipolar,
                MergePolicy::LastWins,
            );
            Ok(entity)
        }

        fn register_systems(_app: &mut App, _schedule: InternedScheduleLabel) {}
    }

    impl OxurackModule for Getterless {
        const KIND: &'static str = "getterless";
        const DISPLAY_NAME: &'static str = "Getterless";

        fn port_schema() -> &'static [oxurack_core::PortSchema] {
            &[]
        }

        fn parameter_schema() -> &'static [ParameterSchema] {
            &[ParameterSchema {
                name: "hidden",
                description: "A parameter intentionally lacking a getter.",
                default: ParameterValue::Float(0.0),
            }]
        }

        fn spawn(
            world: &mut World,
            instance_name: &str,
            _parameters: &HashMap<String, ParameterValue>,
        ) -> Result<Entity, CoreError> {
            Ok(oxurack_core::spawn_module_entity(
                world,
                Self::KIND,
                instance_name,
            ))
        }

        fn register_systems(_app: &mut App, _schedule: InternedScheduleLabel) {}
    }

    const FIXTURE: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/patches/turing-machine-lfo.ron"
    );

    fn loaded_rack() -> super::super::Oxurack {
        let mut rack = super::super::Oxurack::new();
        rack.try_install::<oxurack_mod_clock::ClockModule>()
            .unwrap();
        rack.try_install::<oxurack_mod_lfo::LfoModule>().unwrap();
        rack.try_install::<oxurack_mod_turingmachine::TuringMachineModule>()
            .unwrap();
        rack.try_install::<super::super::MidiOutModule>().unwrap();
        rack.load_patch(Path::new(FIXTURE)).unwrap();
        rack.set_driver_mode(super::super::SimulationDriver::Manual);
        rack.world_mut()
            .resource_mut::<Messages<ParameterChanged>>()
            .clear();
        rack
    }

    #[test]
    fn direct_set_emits_once_with_previous_value_and_failures_emit_nothing() {
        let mut rack = loaded_rack();
        rack.start();
        assert!(matches!(
            rack.set_parameter("lfo_main", "frequency_hz", ParameterValue::Int(1)),
            Err(CoreError::InvalidParameterValue { .. })
        ));
        assert!(
            rack.world()
                .resource::<Messages<ParameterChanged>>()
                .iter_current_update_messages()
                .all(|event| !matches!(event.source, ChangeSource::DirectApi))
        );

        rack.set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(0.5))
            .unwrap();
        let direct: Vec<_> = rack
            .world()
            .resource::<Messages<ParameterChanged>>()
            .iter_current_update_messages()
            .filter(|event| event.source == ChangeSource::DirectApi)
            .collect();
        assert_eq!(direct.len(), 1);
        assert_eq!(direct[0].previous, Some(ParameterValue::Float(0.5)));
        assert_eq!(direct[0].new, ParameterValue::Float(0.5));
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert!(rack.world().resource::<SimulationRunning>().0);
    }

    #[test]
    fn configured_tm_readback_and_direct_event_ignore_held_cv() {
        let mut rack = loaded_rack();
        let handle = rack.world().resource::<PatchHandle>().clone();
        let lfo = handle.modules["lfo_main"];
        let tm = handle.modules["tm_main"];
        let (source_port, target_port) = {
            let world = rack.world_mut();
            let mut ports = world.query::<(Entity, &oxurack_core::Port, &ChildOf)>();
            let mut source = None;
            let mut target = None;
            for (entity, port, parent) in ports.iter(world) {
                if parent.0 == lfo && port.name.as_ref() == "bipolar_out" {
                    source = Some(entity);
                }
                if parent.0 == tm && port.name.as_ref() == "write_modulation" {
                    target = Some(entity);
                }
            }
            (source.unwrap(), target.unwrap())
        };
        let cable_entity = {
            let world = rack.world_mut();
            let mut cables = world.query::<(Entity, &Cable)>();
            cables
                .iter(world)
                .find_map(|(entity, cable)| {
                    (cable.source_port == source_port && cable.target_port == target_port)
                        .then_some(entity)
                })
                .unwrap()
        };
        rack.world_mut()
            .get_mut::<Cable>(cable_entity)
            .unwrap()
            .transform = Some(CableTransform::Affine {
            factor: 0.0,
            offset: 0.25,
        });

        rack.start();
        for _ in 0..200 {
            rack.advance_samples(1).unwrap();
            let _ = rack.drain_midi().count();
        }
        rack.world_mut()
            .resource_mut::<Messages<ParameterChanged>>()
            .clear();
        assert_eq!(
            rack.parameter("tm_main", "write_probability").unwrap(),
            ParameterValue::Float(0.5)
        );
        rack.set_parameter("tm_main", "write_probability", ParameterValue::Float(0.25))
            .unwrap();
        assert_eq!(
            rack.parameter("tm_main", "write_probability").unwrap(),
            ParameterValue::Float(0.25)
        );
        let direct: Vec<_> = rack
            .world()
            .resource::<Messages<ParameterChanged>>()
            .iter_current_update_messages()
            .filter(|event| event.source == ChangeSource::DirectApi)
            .collect();
        assert_eq!(direct.len(), 1);
        assert_eq!(direct[0].previous, Some(ParameterValue::Float(0.5)));
        assert_eq!(direct[0].new, ParameterValue::Float(0.25));

        for invalid in [ParameterValue::Int(1), ParameterValue::Float(f32::NAN)] {
            assert!(matches!(
                rack.set_parameter("tm_main", "write_probability", invalid),
                Err(CoreError::InvalidParameterValue { .. })
            ));
        }
        let direct_after_rejections: Vec<_> = rack
            .world()
            .resource::<Messages<ParameterChanged>>()
            .iter_current_update_messages()
            .filter(|event| event.source == ChangeSource::DirectApi)
            .collect();
        assert_eq!(direct_after_rejections.len(), 1);
        assert_eq!(
            rack.parameter("tm_main", "write_probability").unwrap(),
            ParameterValue::Float(0.25)
        );
        assert_eq!(rack.sample_time().completed_samples(), 200);
        assert!(rack.world().resource::<SimulationRunning>().0);

        let mut events = Vec::new();
        for _ in 200..252 {
            rack.advance_samples(1).unwrap();
            events.extend(rack.drain_midi());
        }
        assert!(events.iter().any(|event| event.sample == 251));
        let effective_output = {
            let world = rack.world_mut();
            let mut ports = world.query::<(&oxurack_core::Port, &ChildOf, &CurrentValue)>();
            ports
                .iter(world)
                .find_map(|(port, parent, current)| {
                    (parent.0 == tm && port.name.as_ref() == "write_probability")
                        .then_some(current.0)
                })
                .unwrap()
        };
        assert_eq!(effective_output, Value::Float(0.5));
        assert_eq!(rack.sample_time().completed_samples(), 252);
    }

    #[test]
    fn live_lfo_edit_changes_next_sample_without_resetting_phase_or_time() {
        let mut rack = loaded_rack();
        rack.start();
        for _ in 0..200 {
            rack.advance_samples(1).unwrap();
            let _ = rack.drain_midi().count();
        }
        rack.set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(1.0))
            .unwrap();
        assert_eq!(rack.sample_time().completed_samples(), 200);
        rack.advance_samples(1).unwrap();
        let lfo = rack.world().resource::<PatchHandle>().modules["lfo_main"];
        let world = rack.world_mut();
        let mut query =
            world.query::<(&oxurack_core::Port, &oxurack_core::CurrentValue, &ChildOf)>();
        let observed = query
            .iter(world)
            .find_map(|(port, current, parent)| {
                (parent.0 == lfo && port.name.as_ref() == "bipolar_out").then_some(current.0)
            })
            .unwrap();
        let oxurack_core::Value::Bipolar(value) = observed else {
            panic!("LFO bipolar output had the wrong value kind: {observed:?}");
        };
        assert!((value - (std::f32::consts::TAU * 0.1).sin()).abs() < 1.0e-6);
        assert_eq!(rack.sample_time().completed_samples(), 201);
    }

    #[test]
    fn snapshot_is_owned_and_preserves_loaded_header_and_current_labels() {
        let mut rack = loaded_rack();
        let handle = rack.world().resource::<PatchHandle>().clone();
        let lfo = handle.modules["lfo_main"];
        rack.world_mut().entity_mut(lfo).insert((
            DisplayName("Current LFO".to_owned()),
            Tags(
                [
                    oxurack_core::Tag::new("z-tag").unwrap(),
                    oxurack_core::Tag::new("a-tag").unwrap(),
                ]
                .into_iter()
                .collect(),
            ),
        ));
        let cable_entity = {
            let world = rack.world_mut();
            let mut query = world.query::<(Entity, &Cable)>();
            query.iter(world).next().map(|(entity, _)| entity).unwrap()
        };
        rack.world_mut()
            .entity_mut(cable_entity)
            .insert(DisplayName("Current cable".to_owned()));
        rack.world_mut()
            .get_mut::<Cable>(cable_entity)
            .unwrap()
            .transform = Some(CableTransform::Invert);
        let mut rack_meta = RackMeta {
            display_name: "Current rack".to_owned(),
            description: "Live metadata".to_owned(),
            notes: vec!["first note".to_owned(), "second note".to_owned()],
            ..RackMeta::default()
        };
        rack_meta.tags = [
            oxurack_core::Tag::new("z-tag").unwrap(),
            oxurack_core::Tag::new("a-tag").unwrap(),
        ]
        .into_iter()
        .collect();
        rack.world_mut().insert_resource(rack_meta);
        rack.set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(1.0))
            .unwrap();
        let clock_entity = rack.world().resource::<PatchHandle>().modules["clock_main"];
        let clock_kind = rack
            .world()
            .get::<Module>(clock_entity)
            .unwrap()
            .kind
            .clone();
        rack.world_mut().resource_scope(
            |world, registry: bevy_ecs::change_detection::Mut<
                oxurack_core::ParameterRegistry,
            >| {
                registry.set_parameter(
                    world,
                    clock_entity,
                    &clock_kind,
                    "bpm",
                    ParameterValue::Float(90.0),
                )
            },
        ).unwrap();
        let snapshot = rack.snapshot_patch().unwrap();
        assert_eq!(snapshot.version, "1.0");
        assert_eq!(snapshot.master_seed, 42);
        assert_eq!(snapshot.bpm, 120.0);
        let lfo = snapshot
            .modules
            .iter()
            .find(|module| module.instance_name == "lfo_main")
            .unwrap();
        assert_eq!(lfo.display_name, "Current LFO");
        assert_eq!(lfo.tags, ["a-tag", "z-tag"]);
        assert_eq!(
            lfo.parameters.get("frequency_hz"),
            Some(&ParameterValue::Float(1.0))
        );
        assert_eq!(snapshot.modules.len(), 4);
        assert_eq!(snapshot.cables.len(), 9);
        let cable = snapshot
            .cables
            .iter()
            .find(|cable| cable.display_name == "Current cable")
            .unwrap();
        assert_eq!(cable.transform, Some(CableTransform::Invert));
        let rack = snapshot.rack.as_ref().unwrap();
        assert_eq!(rack.display_name, "Current rack");
        assert_eq!(rack.notes, ["first note", "second note"]);
        assert_eq!(rack.tags, ["a-tag", "z-tag"]);
        assert_eq!(
            snapshot
                .modules
                .iter()
                .find(|module| module.instance_name == "clock_main")
                .unwrap()
                .parameters
                .get("bpm"),
            Some(&ParameterValue::Float(90.0))
        );
    }

    #[test]
    fn snapshot_preserves_pending_midi_scope_and_continuation() {
        let mut subject = loaded_rack();
        let mut control = loaded_rack();
        let selectors = [super::super::ScopeSelector::new("lfo_main", "bipolar_out")];
        subject.configure_scope(&selectors, 1000).unwrap();
        subject.start();
        control.start();
        for _ in 0..200 {
            subject.advance_samples(1).unwrap();
            control.advance_samples(1).unwrap();
            let _ = subject.drain_midi().count();
            let _ = control.drain_midi().count();
        }
        assert_eq!(subject.sample_time().completed_samples(), 200);
        assert_eq!(
            subject.sample_time().completed_samples(),
            control.sample_time().completed_samples()
        );
        let initial_scope_records = subject.scope().unwrap().records().count();
        let initial_overwrites = subject.scope().unwrap().overwritten();
        let sample_200 = subject.sample_time();
        let driver_200 = subject.driver;
        assert!(subject.world().resource::<SimulationRunning>().0);
        let save_path =
            std::env::temp_dir().join(format!("oxurack-preservation-{}.ron", std::process::id()));
        let _ = std::fs::remove_file(&save_path);
        assert!(
            subject
                .world()
                .resource::<super::super::MidiOutputBuffer>()
                .check_admission()
                .is_ok()
        );
        let snapshot_200 = subject.snapshot_patch().unwrap();
        subject.save_patch(&save_path).unwrap();
        assert_eq!(snapshot_200, subject.snapshot_patch().unwrap());
        assert_eq!(subject.sample_time(), sample_200);
        assert_eq!(subject.driver, driver_200);
        assert!(subject.world().resource::<SimulationRunning>().0);
        assert_eq!(
            subject.scope().unwrap().records().count(),
            initial_scope_records
        );
        assert_eq!(subject.scope().unwrap().overwritten(), initial_overwrites);
        assert!(
            subject
                .world()
                .resource::<super::super::MidiOutputBuffer>()
                .check_admission()
                .is_ok()
        );

        for _ in 200..1000 {
            subject.advance_samples(1).unwrap();
            control.advance_samples(1).unwrap();
            if subject
                .world()
                .resource::<super::super::MidiOutputBuffer>()
                .check_admission()
                .is_err()
            {
                break;
            }
            let _ = subject.drain_midi().count();
            let _ = control.drain_midi().count();
        }
        let pending_before = subject
            .world()
            .resource::<super::super::MidiOutputBuffer>()
            .check_admission()
            .unwrap_err();
        assert!(matches!(
            pending_before,
            TickError::OutputBackpressure {
                pending_messages: 1,
                ..
            }
        ));
        let pending_sample = subject.sample_time().completed_samples();
        let pending_driver = subject.driver;
        assert!(subject.world().resource::<SimulationRunning>().0);
        let scope_before = subject.scope().unwrap().records().count();
        let overwrites_before = subject.scope().unwrap().overwritten();

        let snapshot = subject.snapshot_patch().unwrap();
        subject.save_patch(&save_path).unwrap();
        assert_eq!(snapshot.modules.len(), 4);
        assert_eq!(subject.sample_time().completed_samples(), pending_sample);
        assert_eq!(subject.driver, pending_driver);
        assert!(subject.world().resource::<SimulationRunning>().0);
        assert_eq!(subject.scope().unwrap().records().count(), scope_before);
        assert_eq!(subject.scope().unwrap().overwritten(), overwrites_before);
        assert!(matches!(
            subject
                .world()
                .resource::<super::super::MidiOutputBuffer>()
                .check_admission(),
            Err(TickError::OutputBackpressure {
                pending_messages: 1,
                ..
            })
        ));
        assert_eq!(snapshot, subject.snapshot_patch().unwrap());

        let subject_event: Vec<_> = subject.drain_midi().collect();
        let control_event: Vec<_> = control.drain_midi().collect();
        assert_eq!(subject_event, control_event);
        assert!(subject_event.iter().any(|event| event.sample == 251));
        for _ in pending_sample..1000 {
            subject.advance_samples(1).unwrap();
            control.advance_samples(1).unwrap();
            let left: Vec<_> = subject.drain_midi().collect();
            let right: Vec<_> = control.drain_midi().collect();
            assert_eq!(left, right);
        }
        assert_eq!(subject.sample_time(), control.sample_time());
        let _ = std::fs::remove_file(save_path);
    }

    #[test]
    fn failed_snapshot_preserves_destination_and_failed_load_keeps_identity() {
        let mut rack = loaded_rack();
        let cable_entity = {
            let world = rack.world_mut();
            let mut query = world.query::<(Entity, &Cable)>();
            query.iter(world).next().map(|(entity, _)| entity).unwrap()
        };
        rack.world_mut()
            .get_mut::<Cable>(cable_entity)
            .unwrap()
            .enabled = false;
        let destination = std::env::temp_dir().join(format!(
            "oxurack-snapshot-sentinel-{}.ron",
            std::process::id()
        ));
        std::fs::write(&destination, "sentinel").unwrap();
        assert!(matches!(
            rack.save_patch(&destination),
            Err(CoreError::Patch(PatchError::UnsupportedSnapshot { reason }))
                if reason.contains("disabled") && reason.contains("cable")
        ));
        assert_eq!(std::fs::read_to_string(&destination).unwrap(), "sentinel");
        rack.world_mut()
            .get_mut::<Cable>(cable_entity)
            .unwrap()
            .enabled = true;

        let handle = rack.world().resource::<PatchHandle>().clone();
        let source_module = handle.modules["lfo_main"];
        let external_module =
            oxurack_core::spawn_module_entity(rack.world_mut(), "external_test_module", "external");
        let external_port = oxurack_core::spawn_port_on_module(
            rack.world_mut(),
            external_module,
            "input",
            PortDirection::Input,
            ValueKind::Bipolar,
            MergePolicy::Reject,
        );
        let active_port = {
            let world = rack.world_mut();
            let mut query = world.query::<(Entity, &oxurack_core::Port, &ChildOf)>();
            query
                .iter(world)
                .find_map(|(entity, port, parent)| {
                    (parent.0 == source_module && port.name.as_ref() == "bipolar_out")
                        .then_some(entity)
                })
                .unwrap()
        };
        let crossing_cable = rack
            .world_mut()
            .spawn(Cable {
                source_port: active_port,
                target_port: external_port,
                transform: None,
                enabled: true,
            })
            .id();
        assert!(matches!(
            rack.snapshot_patch(),
            Err(CoreError::Patch(PatchError::UnsupportedSnapshot { reason }))
                if reason.contains("crosses") && reason.contains("active")
        ));
        rack.world_mut().despawn(crossing_cable);

        let missing_parent = std::env::temp_dir()
            .join(format!("oxurack-missing-parent-{}", std::process::id()))
            .join("saved.ron");
        assert!(matches!(
            rack.save_patch(&missing_parent),
            Err(CoreError::Patch(PatchError::Io(_)))
        ));
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert_eq!(rack.snapshot_patch().unwrap().master_seed, 42);

        let invalid =
            std::env::temp_dir().join(format!("oxurack-invalid-load-{}.ron", std::process::id()));
        std::fs::write(&invalid, "not a patch").unwrap();
        assert!(rack.load_patch(&invalid).is_err());
        assert_eq!(rack.snapshot_patch().unwrap().master_seed, 42);
        let _ = std::fs::remove_file(destination);
        let _ = std::fs::remove_file(invalid);
    }

    #[test]
    fn missing_getter_fails_snapshot_and_preserves_existing_file() {
        let mut rack = super::super::Oxurack::new();
        rack.try_install::<Getterless>().unwrap();
        let patch = Patch {
            version: "test".to_owned(),
            master_seed: 9,
            bpm: 100.0,
            modules: vec![ModuleConfig {
                kind: Getterless::KIND.to_owned(),
                instance_name: "no_getter".to_owned(),
                ..ModuleConfig::default()
            }],
            cables: Vec::new(),
            rack: None,
        };
        let source =
            std::env::temp_dir().join(format!("oxurack-getterless-{}.ron", std::process::id()));
        oxurack_core::save_patch_to_file(&patch, &source).unwrap();
        rack.load_patch(&source).unwrap();
        let destination = std::env::temp_dir().join(format!(
            "oxurack-getterless-dest-{}.ron",
            std::process::id()
        ));
        std::fs::write(&destination, "preserve me").unwrap();
        assert!(matches!(
            rack.snapshot_patch(),
            Err(CoreError::UnknownParameter { module, param })
                if module == "no_getter" && param == "hidden"
        ));
        let sample_before_restart = rack.sample_time();
        assert!(matches!(
            rack.restart(),
            Err(CoreError::UnknownParameter { module, param })
                if module == "no_getter" && param == "hidden"
        ));
        assert_eq!(rack.sample_time(), sample_before_restart);
        assert!(rack.world().contains_resource::<PatchHandle>());
        assert!(matches!(
            rack.save_patch(&destination),
            Err(CoreError::UnknownParameter { module, param })
                if module == "no_getter" && param == "hidden"
        ));
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "preserve me"
        );
        let _ = std::fs::remove_file(source);
        let _ = std::fs::remove_file(destination);
    }

    #[test]
    fn snapshot_uses_only_active_handle_and_accepts_empty_loaded_patch() {
        let mut rack = super::super::Oxurack::new();
        rack.try_install::<TieSource>().unwrap();
        rack.try_install::<TieSink>().unwrap();
        let path =
            std::env::temp_dir().join(format!("oxurack-active-handle-{}.ron", std::process::id()));
        let first = Patch {
            version: "first".to_owned(),
            master_seed: 1,
            bpm: 110.0,
            modules: vec![
                ModuleConfig {
                    kind: TieSource::KIND.to_owned(),
                    instance_name: "first_source".to_owned(),
                    ..ModuleConfig::default()
                },
                ModuleConfig {
                    kind: TieSink::KIND.to_owned(),
                    instance_name: "first_sink".to_owned(),
                    ..ModuleConfig::default()
                },
            ],
            cables: vec![CableConfig {
                source: ("first_source".to_owned(), "out".to_owned()),
                target: ("first_sink".to_owned(), "input".to_owned()),
                ..CableConfig::default()
            }],
            rack: None,
        };
        oxurack_core::save_patch_to_file(&first, &path).unwrap();
        rack.load_patch(&path).unwrap();
        let second = Patch {
            version: "second".to_owned(),
            master_seed: 2,
            bpm: 90.0,
            modules: vec![ModuleConfig {
                kind: TieSink::KIND.to_owned(),
                instance_name: "second_sink".to_owned(),
                ..ModuleConfig::default()
            }],
            cables: Vec::new(),
            rack: None,
        };
        oxurack_core::save_patch_to_file(&second, &path).unwrap();
        rack.load_patch(&path).unwrap();
        let snapshot = rack.snapshot_patch().unwrap();
        assert_eq!(snapshot.version, "second");
        assert_eq!(snapshot.master_seed, 2);
        assert_eq!(snapshot.modules.len(), 1);
        assert_eq!(snapshot.modules[0].instance_name, "second_sink");
        assert!(snapshot.cables.is_empty());

        let empty = Patch {
            version: "empty".to_owned(),
            master_seed: 3,
            bpm: 120.0,
            modules: Vec::new(),
            cables: Vec::new(),
            rack: None,
        };
        oxurack_core::save_patch_to_file(&empty, &path).unwrap();
        rack.load_patch(&path).unwrap();
        assert_eq!(rack.snapshot_patch().unwrap(), empty);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn parallel_cable_tie_order_survives_snapshot_and_fresh_load() {
        fn installed() -> super::super::Oxurack {
            let mut rack = super::super::Oxurack::new();
            rack.try_install::<TieSource>().unwrap();
            rack.try_install::<TieSink>().unwrap();
            rack
        }

        fn prepare() -> super::super::Oxurack {
            let mut rack = installed();
            let path =
                std::env::temp_dir().join(format!("oxurack-tie-order-{}.ron", std::process::id()));
            let patch = Patch {
                version: "test".to_owned(),
                master_seed: 5,
                bpm: 120.0,
                modules: vec![
                    ModuleConfig {
                        kind: TieSource::KIND.to_owned(),
                        instance_name: "source".to_owned(),
                        ..ModuleConfig::default()
                    },
                    ModuleConfig {
                        kind: TieSink::KIND.to_owned(),
                        instance_name: "sink".to_owned(),
                        ..ModuleConfig::default()
                    },
                ],
                cables: vec![
                    CableConfig {
                        source: ("source".to_owned(), "out".to_owned()),
                        target: ("sink".to_owned(), "input".to_owned()),
                        transform: Some(CableTransform::Affine {
                            factor: 0.0,
                            offset: 0.75,
                        }),
                        ..CableConfig::default()
                    },
                    CableConfig {
                        source: ("source".to_owned(), "out".to_owned()),
                        target: ("sink".to_owned(), "input".to_owned()),
                        transform: Some(CableTransform::Affine {
                            factor: 0.0,
                            offset: 0.25,
                        }),
                        ..CableConfig::default()
                    },
                ],
                rack: None,
            };
            oxurack_core::save_patch_to_file(&patch, &path).unwrap();
            rack.load_patch(&path).unwrap();
            let _ = std::fs::remove_file(path);
            rack.set_driver_mode(super::super::SimulationDriver::Manual);
            rack
        }

        fn set_source_and_read_sink(rack: &mut super::super::Oxurack) -> f32 {
            let handle = rack.world().resource::<PatchHandle>().clone();
            let source = handle.modules["source"];
            let sink = handle.modules["sink"];
            let world = rack.world_mut();
            let mut query =
                world.query::<(Entity, &oxurack_core::Port, &ChildOf, &mut CurrentValue)>();
            for (_, port, parent, mut current) in query.iter_mut(world) {
                if parent.0 == source && port.name.as_ref() == "out" {
                    current.0 = Value::Bipolar(1.0);
                }
            }
            rack.start();
            rack.advance_samples(1).unwrap();
            let world = rack.world_mut();
            let mut query = world.query::<(&oxurack_core::Port, &ChildOf, &CurrentValue)>();
            query
                .iter(world)
                .find_map(|(port, parent, current)| {
                    (parent.0 == sink && port.name.as_ref() == "input").then_some(current.0)
                })
                .and_then(|value| match value {
                    Value::Bipolar(number) => Some(number),
                    _ => None,
                })
                .unwrap()
        }

        let mut rack = prepare();
        let snapshot = rack.snapshot_patch().unwrap();
        let offsets: Vec<_> = snapshot
            .cables
            .iter()
            .map(|cable| match cable.transform.unwrap() {
                CableTransform::Affine { offset, .. } => offset,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(offsets, [0.75, 0.25]);

        let snapshot_path =
            std::env::temp_dir().join(format!("oxurack-tie-snapshot-{}.ron", std::process::id()));
        oxurack_core::save_patch_to_file(&snapshot, &snapshot_path).unwrap();
        let saved_once = oxurack_core::load_patch_from_file(&snapshot_path).unwrap();
        assert_eq!(
            saved_once
                .cables
                .iter()
                .map(|cable| match cable.transform.unwrap() {
                    CableTransform::Affine { offset, .. } => offset,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
            [0.75, 0.25]
        );
        let first = set_source_and_read_sink(&mut rack);
        let mut replay = installed();
        replay.load_patch(&snapshot_path).unwrap();
        replay.set_driver_mode(super::super::SimulationDriver::Manual);
        let second = set_source_and_read_sink(&mut replay);
        assert!((first - 0.25).abs() < f32::EPSILON);
        assert!((second - 0.25).abs() < f32::EPSILON);
        let replay_snapshot = replay.snapshot_patch().unwrap();
        let replay_offsets: Vec<_> = replay_snapshot
            .cables
            .iter()
            .map(|cable| match cable.transform.unwrap() {
                CableTransform::Affine { offset, .. } => offset,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(replay_offsets, [0.75, 0.25]);
        let second_path = std::env::temp_dir().join(format!(
            "oxurack-tie-snapshot-second-{}.ron",
            std::process::id()
        ));
        replay.save_patch(&second_path).unwrap();
        let saved_twice = oxurack_core::load_patch_from_file(&second_path).unwrap();
        assert_eq!(
            saved_twice
                .cables
                .iter()
                .map(|cable| match cable.transform.unwrap() {
                    CableTransform::Affine { offset, .. } => offset,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
            [0.75, 0.25]
        );
        let mut second_replay = installed();
        second_replay.load_patch(&second_path).unwrap();
        second_replay.set_driver_mode(super::super::SimulationDriver::Manual);
        assert!((set_source_and_read_sink(&mut second_replay) - 0.25).abs() < f32::EPSILON);
        let second_snapshot = second_replay.snapshot_patch().unwrap();
        let second_offsets: Vec<_> = second_snapshot
            .cables
            .iter()
            .map(|cable| match cable.transform.unwrap() {
                CableTransform::Affine { offset, .. } => offset,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(second_offsets, [0.75, 0.25]);
        let _ = std::fs::remove_file(snapshot_path);
        let _ = std::fs::remove_file(second_path);

        let mut live = prepare();
        for (occurrence, description) in [(0, "first"), (1, "second")] {
            oxurack_core::set_label_value(
                live.world_mut(),
                &oxurack_core::LabelTarget::Cable {
                    source: ("source".into(), "out".into()),
                    target: ("sink".into(), "input".into()),
                    occurrence,
                },
                &oxurack_core::LabelKind::Description,
                &oxurack_core::LabelValue::Text(description.into()),
                oxurack_core::ChangeSource::DirectApi,
            )
            .unwrap();
        }
        assert_eq!(set_source_and_read_sink(&mut live), 0.25);
        let add_path =
            std::env::temp_dir().join(format!("oxurack-live-tie-add-{}.ron", std::process::id()));
        let remove_path = std::env::temp_dir().join(format!(
            "oxurack-live-tie-remove-{}.ron",
            std::process::id()
        ));
        let live_time = live.sample_time();
        assert_eq!(
            live.patch(
                ("source", "out"),
                ("sink", "input"),
                Some(CableTransform::Affine {
                    factor: 0.0,
                    offset: 0.5
                })
            )
            .unwrap(),
            2
        );
        assert_eq!(live.sample_time(), live_time, "patch does not tick");
        assert_eq!(set_source_and_read_sink(&mut live), 0.5);
        let added_snapshot = live.snapshot_patch().unwrap();
        assert_eq!(
            added_snapshot
                .cables
                .iter()
                .map(|cable| match cable.transform.unwrap() {
                    CableTransform::Affine { offset, .. } => offset,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
            [0.75, 0.25, 0.5]
        );
        live.save_patch(&add_path).unwrap();
        let saved_add = oxurack_core::load_patch_from_file(&add_path).unwrap();
        assert_eq!(
            saved_add
                .cables
                .iter()
                .map(|cable| match cable.transform.unwrap() {
                    CableTransform::Affine { offset, .. } => offset,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
            [0.75, 0.25, 0.5]
        );
        assert_eq!(saved_add.cables[0].description, "first");
        assert_eq!(saved_add.cables[1].description, "second");
        let mut added_replay = installed();
        added_replay.load_patch(&add_path).unwrap();
        added_replay.set_driver_mode(super::super::SimulationDriver::Manual);
        assert_eq!(set_source_and_read_sink(&mut added_replay), 0.5);

        let before_remove = live.sample_time();
        live.unpatch(("source", "out"), ("sink", "input"), 2)
            .unwrap();
        assert_eq!(live.sample_time(), before_remove, "unpatch does not tick");
        assert_eq!(set_source_and_read_sink(&mut live), 0.25);
        let removed_snapshot = live.snapshot_patch().unwrap();
        assert_eq!(
            removed_snapshot
                .cables
                .iter()
                .map(|cable| match cable.transform.unwrap() {
                    CableTransform::Affine { offset, .. } => offset,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
            [0.75, 0.25]
        );
        assert_eq!(removed_snapshot.cables[0].description, "first");
        assert_eq!(removed_snapshot.cables[1].description, "second");
        live.save_patch(&remove_path).unwrap();
        let mut removed_replay = installed();
        removed_replay.load_patch(&remove_path).unwrap();
        removed_replay.set_driver_mode(super::super::SimulationDriver::Manual);
        assert_eq!(set_source_and_read_sink(&mut removed_replay), 0.25);
        assert_eq!(
            removed_replay
                .patch(
                    ("source", "out"),
                    ("sink", "input"),
                    Some(CableTransform::Affine {
                        factor: 0.0,
                        offset: 0.5
                    })
                )
                .unwrap(),
            2
        );
        removed_replay
            .unpatch(("source", "out"), ("sink", "input"), 2)
            .unwrap();
        assert_eq!(set_source_and_read_sink(&mut removed_replay), 0.25);
        let _ = std::fs::remove_file(add_path);
        let _ = std::fs::remove_file(remove_path);
    }

    #[test]
    fn parallel_cable_entity_fallback_is_saved_as_reload_order() {
        let mut rack = {
            let mut rack = super::super::Oxurack::new();
            rack.try_install::<TieSource>().unwrap();
            rack.try_install::<TieSink>().unwrap();
            let path = std::env::temp_dir()
                .join(format!("oxurack-tie-fallback-{}.ron", std::process::id()));
            let patch = Patch {
                version: "fallback".to_owned(),
                master_seed: 6,
                bpm: 120.0,
                modules: vec![
                    ModuleConfig {
                        kind: TieSource::KIND.to_owned(),
                        instance_name: "source".to_owned(),
                        ..ModuleConfig::default()
                    },
                    ModuleConfig {
                        kind: TieSink::KIND.to_owned(),
                        instance_name: "sink".to_owned(),
                        ..ModuleConfig::default()
                    },
                ],
                cables: vec![
                    CableConfig {
                        source: ("source".to_owned(), "out".to_owned()),
                        target: ("sink".to_owned(), "input".to_owned()),
                        transform: Some(CableTransform::Affine {
                            factor: 0.0,
                            offset: 0.75,
                        }),
                        ..CableConfig::default()
                    },
                    CableConfig {
                        source: ("source".to_owned(), "out".to_owned()),
                        target: ("sink".to_owned(), "input".to_owned()),
                        transform: Some(CableTransform::Affine {
                            factor: 0.0,
                            offset: 0.25,
                        }),
                        ..CableConfig::default()
                    },
                ],
                rack: None,
            };
            oxurack_core::save_patch_to_file(&patch, &path).unwrap();
            rack.load_patch(&path).unwrap();
            let _ = std::fs::remove_file(path);
            rack.set_driver_mode(super::super::SimulationDriver::Manual);
            rack
        };
        let cable_entities = {
            let world = rack.world_mut();
            let mut cables = world.query::<(Entity, &Cable)>();
            let mut entities = cables
                .iter(world)
                .map(|(entity, _)| entity)
                .collect::<Vec<_>>();
            entities.sort();
            entities
        };
        let mut ordered_offsets = Vec::new();
        for entity in cable_entities {
            let cable = rack.world().get::<Cable>(entity).unwrap();
            let CableTransform::Affine { offset, .. } = cable.transform.unwrap() else {
                unreachable!()
            };
            ordered_offsets.push(offset);
            rack.world_mut().entity_mut(entity).remove::<CableOrder>();
        }
        oxurack_core::mark_propagation_order_dirty(rack.world_mut());
        let snapshot = rack.snapshot_patch().unwrap();
        let snapshot_offsets: Vec<_> = snapshot
            .cables
            .iter()
            .map(|cable| match cable.transform.unwrap() {
                CableTransform::Affine { offset, .. } => offset,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(snapshot_offsets, ordered_offsets);
        let expected = *ordered_offsets.last().unwrap();

        let path = std::env::temp_dir().join(format!(
            "oxurack-tie-fallback-snapshot-{}.ron",
            std::process::id()
        ));
        fn set_source_and_read_sink(rack: &mut super::super::Oxurack) -> f32 {
            let handle = rack.world().resource::<PatchHandle>().clone();
            let source = handle.modules["source"];
            let sink = handle.modules["sink"];
            {
                let world = rack.world_mut();
                let mut query =
                    world.query::<(Entity, &oxurack_core::Port, &ChildOf, &mut CurrentValue)>();
                for (_, port, parent, mut current) in query.iter_mut(world) {
                    if parent.0 == source && port.name.as_ref() == "out" {
                        current.0 = Value::Bipolar(1.0);
                    }
                }
            }
            rack.start();
            rack.advance_samples(1).unwrap();
            let world = rack.world_mut();
            let mut query = world.query::<(&oxurack_core::Port, &ChildOf, &CurrentValue)>();
            query
                .iter(world)
                .find_map(|(port, parent, current)| {
                    (parent.0 == sink && port.name.as_ref() == "input").then_some(current.0)
                })
                .and_then(|value| match value {
                    Value::Bipolar(number) => Some(number),
                    _ => None,
                })
                .unwrap()
        }
        rack.save_patch(&path).unwrap();
        let mut replay = super::super::Oxurack::new();
        replay.try_install::<TieSource>().unwrap();
        replay.try_install::<TieSink>().unwrap();
        replay.load_patch(&path).unwrap();
        replay.set_driver_mode(super::super::SimulationDriver::Manual);
        assert!((set_source_and_read_sink(&mut rack) - expected).abs() < f32::EPSILON);
        assert!((set_source_and_read_sink(&mut replay) - expected).abs() < f32::EPSILON);
        assert_eq!(
            replay
                .snapshot_patch()
                .unwrap()
                .cables
                .iter()
                .map(|cable| match cable.transform.unwrap() {
                    CableTransform::Affine { offset, .. } => offset,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
            ordered_offsets
        );
        let _ = std::fs::remove_file(path);
    }
}
