//! Error types for the oxurack-core crate.
//!
//! Errors are organised into a top-level [`CoreError`] that wraps the
//! domain-specific [`PatchError`] and [`TickError`] enums. All errors
//! use `thiserror` for ergonomic `Display` and `From` implementations.

use crate::{MergePolicy, ValueKind};

/// Top-level error type for the core crate.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    /// An error occurred while building or modifying a patch.
    #[error("patch error: {0}")]
    Patch(#[from] PatchError),

    /// An error occurred during a tick (frame processing).
    #[error("tick error: {0}")]
    Tick(#[from] TickError),

    /// A named parameter was not found on the given module.
    #[error("parameter '{param}' not found on module '{module}'")]
    UnknownParameter {
        /// Module instance name.
        module: String,
        /// Parameter name that was looked up.
        param: String,
    },

    /// A parameter rejected the supplied value.
    #[error("parameter '{param}' on module '{module}' rejected value: {reason}")]
    InvalidParameterValue {
        /// Module instance name.
        module: String,
        /// Parameter name.
        param: String,
        /// Human-readable explanation of why the value was rejected.
        reason: String,
    },
}

/// Errors related to patch construction and cable routing.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum PatchError {
    /// The requested module kind is not registered.
    #[error("unknown module kind: '{0}'")]
    UnknownModuleKind(String),

    /// A module instance name was used more than once.
    #[error("duplicate instance name: '{0}'")]
    DuplicateInstanceName(String),

    /// The named port does not exist on the given module.
    #[error("unknown port '{port}' on module '{module}'")]
    UnknownPort {
        /// Module instance name.
        module: String,
        /// Port name that was looked up.
        port: String,
    },

    /// The merge policy is not valid for the port's value kind.
    #[error(
        "merge policy {policy:?} is not legal for value kind {kind:?} on port '{module}::{port}'"
    )]
    IllegalMerge {
        /// Module instance name.
        module: String,
        /// Port name.
        port: String,
        /// The value kind of the port.
        kind: ValueKind,
        /// The merge policy that was rejected.
        policy: MergePolicy,
    },

    /// A feedback cycle was detected in the patch graph.
    #[error("feedback cycle detected through modules: {0:?}")]
    FeedbackCycle(Vec<String>),

    /// Source and target ports have incompatible value kinds.
    #[error(
        "cable source and target value kinds are incompatible: {source_kind:?} -> {target_kind:?}"
    )]
    KindMismatch {
        /// Value kind of the cable source port.
        source_kind: ValueKind,
        /// Value kind of the cable target port.
        target_kind: ValueKind,
    },

    /// Failed to deserialize a RON patch description.
    #[error("RON parse error: {0}")]
    Deserialize(String),

    /// An I/O error occurred reading or writing a patch file.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to serialise a patch to RON.
    #[error("RON serialization error: {0}")]
    Serialize(String),
}

/// Errors that can occur during frame processing (tick).
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum TickError {
    /// A module panicked during its tick callback.
    #[error("module '{0}' panicked during tick")]
    ModulePanic(String),

    /// The real-time MIDI queue overflowed; events were dropped.
    #[error("RT queue full; dropped {0} MIDI events this frame")]
    MidiQueueOverflow(usize),
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    // ── CoreError Display ───────────────────────────────────────────

    #[test]
    fn test_core_error_from_patch_error() {
        let patch_err = PatchError::UnknownModuleKind("reverb".into());
        let core_err = CoreError::from(patch_err);
        let msg = format!("{core_err}");
        assert!(
            msg.contains("patch error"),
            "expected 'patch error' in: {msg}"
        );
        assert!(msg.contains("reverb"), "expected 'reverb' in: {msg}");
    }

    #[test]
    fn test_core_error_from_tick_error() {
        let tick_err = TickError::ModulePanic("vco_1".into());
        let core_err = CoreError::from(tick_err);
        let msg = format!("{core_err}");
        assert!(
            msg.contains("tick error"),
            "expected 'tick error' in: {msg}"
        );
        assert!(msg.contains("vco_1"), "expected 'vco_1' in: {msg}");
    }

    #[test]
    fn test_core_error_unknown_parameter() {
        let err = CoreError::UnknownParameter {
            module: "vco_1".into(),
            param: "detune".into(),
        };
        let msg = format!("{err}");
        assert_eq!(msg, "parameter 'detune' not found on module 'vco_1'");
    }

    #[test]
    fn test_core_error_invalid_parameter_value() {
        let err = CoreError::InvalidParameterValue {
            module: "filter".into(),
            param: "cutoff".into(),
            reason: "must be between 20 and 20000".into(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("filter"), "expected 'filter' in: {msg}");
        assert!(msg.contains("cutoff"), "expected 'cutoff' in: {msg}");
        assert!(msg.contains("must be between"), "expected reason in: {msg}");
    }

    // ── PatchError Display ──────────────────────────────────────────

    #[test]
    fn test_patch_error_unknown_module_kind() {
        let err = PatchError::UnknownModuleKind("reverb".into());
        assert_eq!(format!("{err}"), "unknown module kind: 'reverb'");
    }

    #[test]
    fn test_patch_error_duplicate_instance_name() {
        let err = PatchError::DuplicateInstanceName("vco_1".into());
        assert_eq!(format!("{err}"), "duplicate instance name: 'vco_1'");
    }

    #[test]
    fn test_patch_error_unknown_port() {
        let err = PatchError::UnknownPort {
            module: "vco_1".into(),
            port: "freq".into(),
        };
        assert_eq!(format!("{err}"), "unknown port 'freq' on module 'vco_1'");
    }

    #[test]
    fn test_patch_error_illegal_merge() {
        let err = PatchError::IllegalMerge {
            module: "mixer".into(),
            port: "in_1".into(),
            kind: ValueKind::Midi,
            policy: MergePolicy::Average,
        };
        let msg = format!("{err}");
        assert!(msg.contains("Average"), "expected 'Average' in: {msg}");
        assert!(msg.contains("Midi"), "expected 'Midi' in: {msg}");
        assert!(
            msg.contains("mixer::in_1"),
            "expected 'mixer::in_1' in: {msg}"
        );
    }

    #[test]
    fn test_patch_error_feedback_cycle() {
        let err = PatchError::FeedbackCycle(vec!["vco".into(), "filter".into(), "vca".into()]);
        let msg = format!("{err}");
        assert!(
            msg.contains("feedback cycle"),
            "expected 'feedback cycle' in: {msg}"
        );
        assert!(msg.contains("vco"), "expected 'vco' in: {msg}");
    }

    #[test]
    fn test_patch_error_kind_mismatch() {
        let err = PatchError::KindMismatch {
            source_kind: ValueKind::Float,
            target_kind: ValueKind::Midi,
        };
        let msg = format!("{err}");
        assert!(msg.contains("Float"), "expected 'Float' in: {msg}");
        assert!(msg.contains("Midi"), "expected 'Midi' in: {msg}");
    }

    #[test]
    fn test_patch_error_deserialize() {
        let err = PatchError::Deserialize("unexpected token at line 5".into());
        let msg = format!("{err}");
        assert!(
            msg.contains("RON parse error"),
            "expected 'RON parse error' in: {msg}"
        );
        assert!(
            msg.contains("unexpected token"),
            "expected detail in: {msg}"
        );
    }

    #[test]
    fn test_patch_error_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = PatchError::Io(io_err);
        let msg = format!("{err}");
        assert!(msg.contains("I/O error"), "expected 'I/O error' in: {msg}");
        assert!(msg.contains("file not found"), "expected detail in: {msg}");
    }

    #[test]
    fn test_patch_error_serialize() {
        let err = PatchError::Serialize("failed to serialize".into());
        let msg = format!("{err}");
        assert!(
            msg.contains("RON serialization error"),
            "expected 'RON serialization error' in: {msg}"
        );
        assert!(
            msg.contains("failed to serialize"),
            "expected detail in: {msg}"
        );
    }

    #[test]
    fn test_patch_error_io_from_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let patch_err: PatchError = io_err.into();
        assert!(
            matches!(patch_err, PatchError::Io(_)),
            "expected PatchError::Io, got: {patch_err:?}"
        );
    }

    // ── TickError Display ───────────────────────────────────────────

    #[test]
    fn test_tick_error_module_panic() {
        let err = TickError::ModulePanic("reverb_1".into());
        assert_eq!(format!("{err}"), "module 'reverb_1' panicked during tick");
    }

    #[test]
    fn test_tick_error_midi_queue_overflow() {
        let err = TickError::MidiQueueOverflow(12);
        assert_eq!(
            format!("{err}"),
            "RT queue full; dropped 12 MIDI events this frame"
        );
    }

    // ── From conversions ────────────────────────────────────────────

    #[test]
    fn test_from_patch_error_to_core_error() {
        let patch: PatchError = PatchError::DuplicateInstanceName("a".into());
        let core: CoreError = patch.into();
        assert!(matches!(core, CoreError::Patch(_)));
    }

    #[test]
    fn test_from_tick_error_to_core_error() {
        let tick: TickError = TickError::MidiQueueOverflow(5);
        let core: CoreError = tick.into();
        assert!(matches!(core, CoreError::Tick(_)));
    }
}
