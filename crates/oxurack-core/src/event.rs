//! ECS messages for patch changes, transport control, and MIDI input.
//!
//! All message types use the Bevy `Message` derive macro (Bevy 0.18's
//! replacement for the older `Event` derive). They are registered in
//! [`CorePlugin`](crate::CorePlugin) via `add_message::<T>()`.

use std::path::PathBuf;

use bevy_ecs::prelude::{Entity, Message};

/// Transport state change reported by the real-time thread.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportChanged(pub TransportState);

/// Possible transport states.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportState {
    /// Playback has started from the beginning.
    Started,
    /// Playback has stopped.
    Stopped,
    /// Playback has resumed from the current position.
    Continued,
}

/// A MIDI message received from an external input port.
#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct MidiInReceived {
    /// Index of the physical input port (0-based).
    pub port_index: u8,
    /// Timestamp in nanoseconds (relative to audio callback start).
    pub timestamp_ns: u64,
    /// The parsed MIDI message.
    pub message: crate::MidiMessage,
}

/// High-level commands for controlling the rack.
///
/// Sent from the REPL, UI, or scripting layer. Processed by the
/// command handler system during the next tick.
#[non_exhaustive]
#[derive(Message, Debug, Clone)]
pub enum CoreCommand {
    /// Load a patch from the given file path.
    LoadPatch(PathBuf),
    /// Save the current patch to the given file path.
    SavePatch(PathBuf),
    /// Set a parameter on a module instance.
    SetParameter {
        /// Module instance name.
        module: String,
        /// Parameter name.
        param: String,
        /// New value.
        value: crate::ParameterValue,
    },
    /// Add a cable between two ports.
    AddCable {
        /// `(module_instance_name, port_name)` of the source.
        source: (String, String),
        /// `(module_instance_name, port_name)` of the target.
        target: (String, String),
        /// Optional inline cable transform.
        transform: Option<crate::CableTransform>,
    },
    /// Remove a cable between two ports.
    RemoveCable {
        /// `(module_instance_name, port_name)` of the source.
        source: (String, String),
        /// `(module_instance_name, port_name)` of the target.
        target: (String, String),
    },
    /// Set the global tempo in beats per minute.
    SetBpm(f32),
    /// Panic: silence all voices and reset all modules.
    Panic,
}

/// Emitted *last* in a patch-load batch, after all individual
/// [`ModuleAdded`], [`CableConnected`], and [`ParameterChanged`]
/// notifications. Semantically: "the patch is now fully in place;
/// redraw if you redraw."
///
/// Subscribers who don't care about granular structural changes can
/// subscribe only to this event and treat it as a batch sync point.
#[derive(Message, Debug, Clone)]
pub struct PatchLoaded {
    /// The name of the patch that was loaded.
    pub patch_name: String,
    /// Where the patch came from.
    pub source: PatchSource,
    /// Number of modules in the loaded patch.
    pub module_count: usize,
    /// Number of cables in the loaded patch.
    pub cable_count: usize,
    /// Number of label-change notifications emitted in this batch.
    pub label_count: usize,
}

/// Provenance for a [`PatchLoaded`] notification.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum PatchSource {
    /// Loaded from a file at the given path.
    File(PathBuf),
    /// Constructed in memory (tests, direct API).
    InMemory,
    /// Other source (scripting, network, etc.).
    Other(String),
}

// ── Structural-mutation notifications ─────────────────────────────

/// Emitted when a module entity is added to the rack.
///
/// Producers: [`apply_patch_to_world`](crate::apply_patch_to_world)
/// (during patch load); future runtime module-install APIs.
#[derive(Message, Clone, Debug)]
pub struct ModuleAdded {
    /// The ECS entity for the new module.
    pub entity: Entity,
    /// User-facing instance name (key in `PatchHandle::modules`).
    pub instance_name: String,
    /// Module kind (matches the `Module` component's `kind` field).
    pub kind: crate::ModuleKind,
}

/// Emitted when a module entity is removed from the rack.
///
/// Producers: future `remove_module` API (no emission site today).
#[derive(Message, Clone, Debug)]
pub struct ModuleRemoved {
    /// The ECS entity that was removed.
    pub entity: Entity,
    /// User-facing instance name.
    pub instance_name: String,
    /// Module kind.
    pub kind: crate::ModuleKind,
}

/// Emitted when a cable entity is added to the rack.
///
/// Producers: [`apply_patch_to_world`](crate::apply_patch_to_world)
/// (during patch load); future `dispatch_add_cable` (currently
/// `NotImplemented`).
#[derive(Message, Clone, Debug)]
pub struct CableConnected {
    /// The ECS entity for the new cable.
    pub cable_entity: Entity,
    /// The output port entity this cable reads from.
    pub source_port: Entity,
    /// The input port entity this cable writes to.
    pub target_port: Entity,
    /// Optional inline signal transform.
    pub transform: Option<crate::CableTransform>,
}

/// Emitted when a cable entity is removed from the rack.
///
/// Producers: future `dispatch_remove_cable` (currently
/// `NotImplemented`).
#[derive(Message, Clone, Debug)]
pub struct CableDisconnected {
    /// The ECS entity that was removed.
    pub cable_entity: Entity,
    /// The output port entity this cable was reading from.
    pub source_port: Entity,
    /// The input port entity this cable was writing to.
    pub target_port: Entity,
}

/// Emitted after a parameter is successfully written.
///
/// Carries `old` (when available) and `new` so consumers (audit log,
/// undo, diff renderers) get the full delta in one notification. The
/// `source` field carries provenance — was this command-driven,
/// patch-load-driven, or direct-API-driven.
///
/// Naming note: `ParameterChanged`, not `KnobChanged`. Parameters
/// have multiple form factors — knobs, switches, sliders, MIDI-CC,
/// modulation destinations — so the notification names the substrate,
/// not the UI affordance.
#[derive(Message, Clone, Debug)]
pub struct ParameterChanged {
    /// The module entity whose parameter changed.
    pub entity: Entity,
    /// User-facing instance name of the module.
    pub instance_name: String,
    /// The parameter that changed.
    pub param: crate::ParameterName,
    /// Previous value, if available.
    // TODO(bucket-getter): Populate when ParameterRegistry getter
    // machinery lands. Currently None at all emission sites.
    pub previous: Option<crate::ParameterValue>,
    /// New value that was written.
    pub new: crate::ParameterValue,
    /// Provenance: what triggered this change.
    pub source: ChangeSource,
}

/// Provenance for a [`ParameterChanged`] notification.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum ChangeSource {
    /// Direct user/script command via [`CoreCommand::SetParameter`].
    Command,
    /// Direct API mutation (future API surfaces).
    DirectApi,
    /// Patch-load initialization. One `ParameterChanged` per
    /// parameter set during patch load.
    PatchLoad,
}

/// Emitted when an instance-level label or rack-level metadata changes.
///
/// `entity` is `None` for rack-level changes ([`RackMeta`](crate::RackMeta))
/// and `Some(e)` for entity-level changes. `previous` is `Some(prev)` when
/// overwriting an existing label, `None` for first-time set during patch load.
#[derive(Message, Clone, Debug)]
pub struct LabelChanged {
    /// Target entity, or `None` for rack-level metadata.
    pub entity: Option<Entity>,
    /// Instance name (empty string for rack-level).
    pub instance_name: String,
    /// Which label field changed.
    pub kind: LabelKind,
    /// Previous value, if overwriting.
    pub previous: Option<LabelValue>,
    /// New value.
    pub new: LabelValue,
    /// Provenance.
    pub source: ChangeSource,
}

/// Discriminant for which label field changed.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LabelKind {
    /// Instance or rack display name.
    DisplayName,
    /// Instance or rack description.
    Description,
    /// Instance or rack tags.
    Tags,
    /// Rack-level notes.
    Notes,
}

/// Payload for a label-change notification.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LabelValue {
    /// A text value (display name or description).
    Text(String),
    /// A tag set.
    Tags(std::collections::BTreeSet<crate::label::Tag>),
    /// A notes list (rack-level only).
    Notes(Vec<String>),
}

/// Non-fatal warning reported by the real-time thread.
///
/// The [`RtWarningCode`] discriminant classifies the warning. These
/// messages are emitted by the RT bridge's drain system when it
/// encounters a `NonFatalError` event from the RT queue.
#[derive(Message, Debug, Clone, Copy)]
pub struct RtWarning {
    /// The classification of this warning.
    pub code: RtWarningCode,
}

/// Classification of non-fatal warnings from the RT thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtWarningCode {
    /// RT priority elevation failed; timing jitter may be higher.
    PriorityElevationFailed,
    /// The slave clock detected a dropout in the external clock signal.
    ClockDropout,
    /// The slave clock has not yet locked to an external clock source.
    ClockNotLocked,
    /// The RT-to-ECS queue overflowed; some events were dropped.
    QueueOverflow,
    /// A MIDI output port was disconnected or became unavailable.
    OutputPortLost,
    /// A MIDI input port was disconnected or became unavailable.
    InputPortLost,
    /// The ECS-to-RT output command queue was full; MIDI commands were dropped.
    OutputQueueFull,
}

/// Emitted when the RT thread reports a MIDI Song Position Pointer update.
#[derive(Message, Debug, Clone, Copy)]
pub struct SongPositionChanged {
    /// 14-bit song position in MIDI beats (6 clocks per beat).
    pub position: u16,
}

/// Dispatches a [`CoreCommand`] against the world.
///
/// Called by the REPL, umbrella crate's command handler, or any other
/// system that processes [`CoreCommand`] messages. This is a standalone
/// function (not a Bevy system) because several commands require
/// exclusive `&mut World` access.
///
/// # Currently implemented
///
/// - [`CoreCommand::SetParameter`] -- resolves the target module by
///   instance name via [`PatchHandle`](crate::PatchHandle), reads the
///   [`ModuleKind`](crate::ModuleKind) from the entity's
///   [`Module`](crate::Module) component, and invokes the registered
///   setter via [`ParameterRegistry`](crate::ParameterRegistry).
/// - [`CoreCommand::Panic`] -- returns `Ok(())` (module reset will be
///   implemented when concrete modules exist).
/// - [`CoreCommand::SetBpm`] -- returns `Ok(())` (tempo propagation
///   will be wired when the RT bridge is integrated into the umbrella
///   crate).
///
/// # Emits
///
/// - `SetParameter` (on success): [`ParameterChanged`] with
///   `source: ChangeSource::Command`.
///
/// # Not yet implemented
///
/// The following commands return
/// [`Err(NotImplemented(..))`](crate::CoreError::NotImplemented)
/// because they require infrastructure provided by the umbrella crate:
///
/// - [`CoreCommand::LoadPatch`]
/// - [`CoreCommand::SavePatch`]
/// - [`CoreCommand::AddCable`] / [`CoreCommand::RemoveCable`]
pub fn dispatch_core_command(
    world: &mut bevy_ecs::world::World,
    command: &CoreCommand,
) -> Result<(), crate::CoreError> {
    match command {
        CoreCommand::Panic | CoreCommand::SetBpm(_) => Ok(()),
        CoreCommand::SetParameter {
            module,
            param,
            value,
        } => dispatch_set_parameter(world, module, param, value),
        CoreCommand::LoadPatch(_) => Err(crate::CoreError::NotImplemented("LoadPatch")),
        CoreCommand::SavePatch(_) => Err(crate::CoreError::NotImplemented("SavePatch")),
        CoreCommand::AddCable { .. } => Err(crate::CoreError::NotImplemented("AddCable")),
        CoreCommand::RemoveCable { .. } => Err(crate::CoreError::NotImplemented("RemoveCable")),
    }
}

fn dispatch_set_parameter(
    world: &mut bevy_ecs::world::World,
    instance_name: &str,
    param: &str,
    value: &crate::ParameterValue,
) -> Result<(), crate::CoreError> {
    let entity = world
        .get_resource::<crate::PatchHandle>()
        .ok_or(crate::CoreError::NoPatchLoaded)?
        .modules
        .get(instance_name)
        .copied()
        .ok_or_else(|| crate::CoreError::UnknownModule(instance_name.to_string()))?;

    let kind = world
        .get::<crate::Module>(entity)
        .ok_or_else(|| crate::CoreError::UnknownModule(instance_name.to_string()))?
        .kind
        .clone();

    // TODO(bucket-4): Centralise schema validation here once
    // OxurackModule exposes per-parameter setters via the trait and
    // install() wires them with schemas into ParameterRegistry.
    // Currently, validation is the responsibility of each setter fn.

    let result = world.resource_scope(
        |world, registry: bevy_ecs::change_detection::Mut<crate::ParameterRegistry>| {
            registry
                .set_parameter(world, entity, &kind, param, value.clone())
                .map_err(|e| match e {
                    crate::CoreError::UnknownParameter { param, .. } => {
                        crate::CoreError::UnknownParameter {
                            module: instance_name.to_string(),
                            param,
                        }
                    }
                    crate::CoreError::InvalidParameterValue { param, reason, .. } => {
                        crate::CoreError::InvalidParameterValue {
                            module: instance_name.to_string(),
                            param,
                            reason,
                        }
                    }
                    other => other,
                })
        },
    );

    if result.is_ok() {
        world.write_message(ParameterChanged {
            entity,
            instance_name: instance_name.to_string(),
            param: crate::ParameterName::from(param),
            // TODO(bucket-getter): Populate when ParameterRegistry
            // getter machinery lands.
            previous: None,
            new: value.clone(),
            source: ChangeSource::Command,
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── TransportState / TransportChanged ────────────────────────

    #[test]
    fn test_transport_changed_debug() {
        let tc = TransportChanged(TransportState::Started);
        let debug = format!("{tc:?}");
        assert!(debug.contains("Started"), "expected 'Started' in: {debug}");
    }

    #[test]
    fn test_transport_changed_clone_eq() {
        let a = TransportChanged(TransportState::Stopped);
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_transport_state_all_variants() {
        let variants = [
            TransportState::Started,
            TransportState::Stopped,
            TransportState::Continued,
        ];
        for v in &variants {
            let debug = format!("{v:?}");
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_transport_state_eq() {
        assert_eq!(TransportState::Started, TransportState::Started);
        assert_ne!(TransportState::Started, TransportState::Stopped);
    }

    // ── MidiInReceived ──────────────────────────────────────────

    #[test]
    fn test_midi_in_received_debug() {
        let msg = MidiInReceived {
            port_index: 0,
            timestamp_ns: 12345,
            message: crate::MidiMessage::NoteOn {
                channel: 0,
                note: 60,
                velocity: 100,
            },
        };
        let debug = format!("{msg:?}");
        assert!(debug.contains("NoteOn"), "expected 'NoteOn' in: {debug}");
    }

    #[test]
    fn test_midi_in_received_clone_eq() {
        let msg = MidiInReceived {
            port_index: 1,
            timestamp_ns: 0,
            message: crate::MidiMessage::Clock,
        };
        let cloned = msg;
        assert_eq!(msg, cloned);
    }

    // ── CoreCommand ─────────────────────────────────────────────

    #[test]
    fn test_core_command_load_patch_debug() {
        let cmd = CoreCommand::LoadPatch(PathBuf::from("patch.ron"));
        let debug = format!("{cmd:?}");
        assert!(
            debug.contains("LoadPatch"),
            "expected 'LoadPatch' in: {debug}"
        );
    }

    #[test]
    fn test_core_command_save_patch_debug() {
        let cmd = CoreCommand::SavePatch(PathBuf::from("out.ron"));
        let debug = format!("{cmd:?}");
        assert!(
            debug.contains("SavePatch"),
            "expected 'SavePatch' in: {debug}"
        );
    }

    #[test]
    fn test_core_command_set_parameter_debug() {
        let cmd = CoreCommand::SetParameter {
            module: "vco_1".into(),
            param: "freq".into(),
            value: crate::ParameterValue::Float(440.0),
        };
        let debug = format!("{cmd:?}");
        assert!(
            debug.contains("SetParameter"),
            "expected 'SetParameter' in: {debug}"
        );
    }

    #[test]
    fn test_core_command_add_cable_debug() {
        let cmd = CoreCommand::AddCable {
            source: ("vco_1".into(), "out".into()),
            target: ("filter_1".into(), "in".into()),
            transform: None,
        };
        let debug = format!("{cmd:?}");
        assert!(
            debug.contains("AddCable"),
            "expected 'AddCable' in: {debug}"
        );
    }

    #[test]
    fn test_core_command_remove_cable_debug() {
        let cmd = CoreCommand::RemoveCable {
            source: ("vco_1".into(), "out".into()),
            target: ("filter_1".into(), "in".into()),
        };
        let debug = format!("{cmd:?}");
        assert!(
            debug.contains("RemoveCable"),
            "expected 'RemoveCable' in: {debug}"
        );
    }

    #[test]
    fn test_core_command_set_bpm_debug() {
        let cmd = CoreCommand::SetBpm(120.0);
        let debug = format!("{cmd:?}");
        assert!(debug.contains("SetBpm"), "expected 'SetBpm' in: {debug}");
    }

    #[test]
    fn test_core_command_panic_debug() {
        let cmd = CoreCommand::Panic;
        let debug = format!("{cmd:?}");
        assert!(debug.contains("Panic"), "expected 'Panic' in: {debug}");
    }

    #[test]
    fn test_core_command_clone() {
        let cmd = CoreCommand::SetBpm(140.0);
        let cloned = cmd.clone();
        let debug = format!("{cloned:?}");
        assert!(debug.contains("140"));
    }

    // ── PatchLoaded ─────────────────────────────────────────────

    #[test]
    fn test_patch_loaded_debug() {
        let evt = PatchLoaded {
            patch_name: "berlin_school".into(),
            source: PatchSource::InMemory,
            module_count: 2,
            cable_count: 1,
            label_count: 0,
        };
        let debug = format!("{evt:?}");
        assert!(
            debug.contains("berlin_school"),
            "expected 'berlin_school' in: {debug}"
        );
    }

    #[test]
    fn test_patch_loaded_clone() {
        let evt = PatchLoaded {
            patch_name: "test_patch".into(),
            source: PatchSource::InMemory,
            module_count: 1,
            cable_count: 0,
            label_count: 0,
        };
        let cloned = evt.clone();
        assert_eq!(cloned.patch_name, "test_patch");
    }

    // ── dispatch_core_command ───────────────────────────────────

    #[test]
    fn test_dispatch_panic_command() {
        let mut world = bevy_ecs::world::World::new();
        let result = dispatch_core_command(&mut world, &CoreCommand::Panic);
        assert!(result.is_ok(), "Panic command should return Ok: {result:?}");
    }

    #[test]
    fn test_dispatch_set_bpm_command() {
        let mut world = bevy_ecs::world::World::new();
        let result = dispatch_core_command(&mut world, &CoreCommand::SetBpm(140.0));
        assert!(
            result.is_ok(),
            "SetBpm command should return Ok: {result:?}"
        );
    }

    #[test]
    fn test_dispatch_load_patch_returns_not_implemented() {
        let mut world = bevy_ecs::world::World::new();
        let result = dispatch_core_command(
            &mut world,
            &CoreCommand::LoadPatch(PathBuf::from("patch.ron")),
        );
        assert!(
            matches!(result, Err(crate::CoreError::NotImplemented("LoadPatch"))),
            "LoadPatch should return NotImplemented: {result:?}"
        );
    }

    // ── SetParameter dispatch tests ───────────────────────────────

    #[derive(bevy_ecs::prelude::Component, Debug, Clone, PartialEq)]
    struct TestParamState {
        gain: f32,
    }

    fn test_gain_setter(
        world: &mut bevy_ecs::world::World,
        entity: bevy_ecs::prelude::Entity,
        value: crate::ParameterValue,
    ) -> Result<(), crate::CoreError> {
        match value {
            crate::ParameterValue::Float(f) => {
                world
                    .entity_mut(entity)
                    .get_mut::<TestParamState>()
                    .expect("TestParamState should exist")
                    .gain = f;
                Ok(())
            }
            _ => Err(crate::CoreError::InvalidParameterValue {
                module: "test".into(),
                param: "gain".into(),
                reason: "expected Float".into(),
            }),
        }
    }

    fn setup_set_parameter_world() -> bevy_ecs::world::World {
        let mut world = bevy_ecs::world::World::new();

        let module_entity = crate::spawn_module_entity(&mut world, "test_gen", "gen_1");
        world
            .entity_mut(module_entity)
            .insert(TestParamState { gain: 0.0 });

        let mut handle = crate::PatchHandle {
            modules: std::collections::HashMap::new(),
        };
        handle.modules.insert("gen_1".to_string(), module_entity);
        world.insert_resource(handle);

        let mut registry = crate::ParameterRegistry::default();
        registry.register(
            crate::ModuleKind::from("test_gen"),
            "gain",
            test_gain_setter,
        );
        world.insert_resource(registry);

        world
    }

    #[test]
    fn test_dispatch_set_parameter_positive_path() {
        let mut world = setup_set_parameter_world();
        let result = dispatch_core_command(
            &mut world,
            &CoreCommand::SetParameter {
                module: "gen_1".into(),
                param: "gain".into(),
                value: crate::ParameterValue::Float(0.75),
            },
        );
        assert!(
            result.is_ok(),
            "valid SetParameter should return Ok: {result:?}"
        );

        let handle = world.get_resource::<crate::PatchHandle>().unwrap();
        let entity = handle.modules["gen_1"];
        let state = world.get::<TestParamState>(entity).unwrap();
        assert_eq!(state.gain, 0.75, "setter should have updated the component");
    }

    #[test]
    fn test_parameter_changed_emitted_on_set_parameter() {
        let mut world = setup_set_parameter_world();
        world.init_resource::<bevy_ecs::prelude::Messages<ParameterChanged>>();

        dispatch_core_command(
            &mut world,
            &CoreCommand::SetParameter {
                module: "gen_1".into(),
                param: "gain".into(),
                value: crate::ParameterValue::Float(0.9),
            },
        )
        .unwrap();

        let messages = world.resource::<bevy_ecs::prelude::Messages<ParameterChanged>>();
        let collected: Vec<_> = messages.iter_current_update_messages().cloned().collect();
        assert_eq!(collected.len(), 1, "expected exactly one ParameterChanged");
        assert!(matches!(
            &collected[0],
            ParameterChanged {
                instance_name,
                param,
                previous: None,
                new: crate::ParameterValue::Float(v),
                source: ChangeSource::Command,
                ..
            } if instance_name == "gen_1"
              && param.as_ref() == "gain"
              && (*v - 0.9).abs() < f32::EPSILON
        ));
    }

    #[test]
    fn test_parameter_changed_not_emitted_on_set_parameter_failure() {
        let mut world = setup_set_parameter_world();
        world.init_resource::<bevy_ecs::prelude::Messages<ParameterChanged>>();

        let _ = dispatch_core_command(
            &mut world,
            &CoreCommand::SetParameter {
                module: "nonexistent".into(),
                param: "gain".into(),
                value: crate::ParameterValue::Float(0.5),
            },
        );

        let messages = world.resource::<bevy_ecs::prelude::Messages<ParameterChanged>>();
        let collected: Vec<_> = messages.iter_current_update_messages().cloned().collect();
        assert_eq!(
            collected.len(),
            0,
            "no ParameterChanged should be emitted on failure"
        );
    }

    #[test]
    fn test_dispatch_set_parameter_no_patch_loaded() {
        let mut world = bevy_ecs::world::World::new();
        world.init_resource::<crate::ParameterRegistry>();

        let result = dispatch_core_command(
            &mut world,
            &CoreCommand::SetParameter {
                module: "gen_1".into(),
                param: "gain".into(),
                value: crate::ParameterValue::Float(0.5),
            },
        );
        assert!(
            matches!(result, Err(crate::CoreError::NoPatchLoaded)),
            "should return NoPatchLoaded when no patch is loaded: {result:?}"
        );
    }

    #[test]
    fn test_dispatch_set_parameter_unknown_module() {
        let mut world = setup_set_parameter_world();
        let result = dispatch_core_command(
            &mut world,
            &CoreCommand::SetParameter {
                module: "nonexistent".into(),
                param: "gain".into(),
                value: crate::ParameterValue::Float(0.5),
            },
        );
        assert!(
            matches!(result, Err(crate::CoreError::UnknownModule(ref name)) if name == "nonexistent"),
            "should return UnknownModule for missing instance: {result:?}"
        );
    }

    #[test]
    fn test_dispatch_set_parameter_unknown_parameter() {
        let mut world = setup_set_parameter_world();
        let result = dispatch_core_command(
            &mut world,
            &CoreCommand::SetParameter {
                module: "gen_1".into(),
                param: "nonexistent".into(),
                value: crate::ParameterValue::Float(0.5),
            },
        );
        assert!(
            matches!(
                result,
                Err(crate::CoreError::UnknownParameter { ref module, ref param })
                    if module == "gen_1" && param == "nonexistent"
            ),
            "should return UnknownParameter with instance name: {result:?}"
        );
    }

    #[test]
    fn test_dispatch_set_parameter_invalid_value() {
        let mut world = setup_set_parameter_world();
        let result = dispatch_core_command(
            &mut world,
            &CoreCommand::SetParameter {
                module: "gen_1".into(),
                param: "gain".into(),
                value: crate::ParameterValue::Bool(true),
            },
        );
        assert!(
            matches!(
                result,
                Err(crate::CoreError::InvalidParameterValue { ref module, ref param, .. })
                    if module == "gen_1" && param == "gain"
            ),
            "should return InvalidParameterValue with instance name: {result:?}"
        );
    }

    // Schema validation is deferred to Bucket 4, when OxurackModule
    // exposes per-parameter setters and install() wires them with
    // schemas into ParameterRegistry. Currently each setter is
    // responsible for its own validation (see test_gain_setter above).

    #[test]
    fn test_dispatch_save_patch_returns_not_implemented() {
        let mut world = bevy_ecs::world::World::new();
        let result = dispatch_core_command(
            &mut world,
            &CoreCommand::SavePatch(PathBuf::from("out.ron")),
        );
        assert!(
            matches!(result, Err(crate::CoreError::NotImplemented("SavePatch"))),
            "SavePatch should return NotImplemented: {result:?}"
        );
    }

    #[test]
    fn test_dispatch_add_cable_returns_not_implemented() {
        let mut world = bevy_ecs::world::World::new();
        let result = dispatch_core_command(
            &mut world,
            &CoreCommand::AddCable {
                source: ("vco_1".into(), "out".into()),
                target: ("filter_1".into(), "in".into()),
                transform: None,
            },
        );
        assert!(
            matches!(result, Err(crate::CoreError::NotImplemented("AddCable"))),
            "AddCable should return NotImplemented: {result:?}"
        );
    }

    #[test]
    fn test_dispatch_remove_cable_returns_not_implemented() {
        let mut world = bevy_ecs::world::World::new();
        let result = dispatch_core_command(
            &mut world,
            &CoreCommand::RemoveCable {
                source: ("vco_1".into(), "out".into()),
                target: ("filter_1".into(), "in".into()),
            },
        );
        assert!(
            matches!(result, Err(crate::CoreError::NotImplemented("RemoveCable"))),
            "RemoveCable should return NotImplemented: {result:?}"
        );
    }
}
