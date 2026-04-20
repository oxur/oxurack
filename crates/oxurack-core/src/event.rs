//! ECS messages for patch changes, transport control, and MIDI input.
//!
//! All message types use the Bevy `Message` derive macro (Bevy 0.18's
//! replacement for the older `Event` derive). They are registered in
//! [`CorePlugin`](crate::CorePlugin) via `add_message::<T>()`.

use std::path::PathBuf;

use bevy_ecs::prelude::Message;

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

/// Emitted after a patch has been successfully loaded.
#[derive(Message, Debug, Clone)]
pub struct PatchLoaded {
    /// The name of the patch that was loaded.
    pub patch_name: String,
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
/// - [`CoreCommand::Panic`] -- returns `Ok(())` (module reset will be
///   implemented when concrete modules exist).
/// - [`CoreCommand::SetBpm`] -- returns `Ok(())` (tempo propagation
///   will be wired when the RT bridge is integrated into the umbrella
///   crate).
///
/// # Not yet implemented
///
/// The following commands return
/// [`Err(NotImplemented(..))`](crate::CoreError::NotImplemented)
/// because they require infrastructure provided by the umbrella crate
/// (module-entity lookup, world reconstruction, port-entity
/// resolution):
///
/// - [`CoreCommand::LoadPatch`]
/// - [`CoreCommand::SetParameter`]
/// - [`CoreCommand::SavePatch`]
/// - [`CoreCommand::AddCable`] / [`CoreCommand::RemoveCable`]
pub fn dispatch_core_command(
    _world: &mut bevy_ecs::world::World,
    command: &CoreCommand,
) -> Result<(), crate::CoreError> {
    match command {
        CoreCommand::Panic | CoreCommand::SetBpm(_) => Ok(()),
        CoreCommand::LoadPatch(_) => Err(crate::CoreError::NotImplemented("LoadPatch")),
        CoreCommand::SetParameter { .. } => Err(crate::CoreError::NotImplemented("SetParameter")),
        CoreCommand::SavePatch(_) => Err(crate::CoreError::NotImplemented("SavePatch")),
        CoreCommand::AddCable { .. } => Err(crate::CoreError::NotImplemented("AddCable")),
        CoreCommand::RemoveCable { .. } => Err(crate::CoreError::NotImplemented("RemoveCable")),
    }
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

    #[test]
    fn test_dispatch_set_parameter_returns_not_implemented() {
        let mut world = bevy_ecs::world::World::new();
        let result = dispatch_core_command(
            &mut world,
            &CoreCommand::SetParameter {
                module: "vco_1".into(),
                param: "freq".into(),
                value: crate::ParameterValue::Float(440.0),
            },
        );
        assert!(
            matches!(
                result,
                Err(crate::CoreError::NotImplemented("SetParameter"))
            ),
            "SetParameter should return NotImplemented: {result:?}"
        );
    }

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
