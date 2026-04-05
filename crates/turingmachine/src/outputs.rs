/// Snapshot of all outputs produced by a single step of the Turing Machine.
///
/// Each field corresponds to a physical or virtual output jack on the
/// module.  The struct is marked `#[non_exhaustive]` so that new outputs
/// can be added in future versions without a breaking change.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct StepOutputs {
    /// Quantized MIDI note number (0--127).
    pub note: Option<u8>,

    /// Velocity in the range 1--127, or `None` when the step is silent.
    pub velocity: Option<u8>,

    /// Pulse / gate output for the current step.
    pub gate: bool,

    /// Independently quantized MIDI note (may use a different scale).
    pub scale_note: Option<u8>,

    /// Six AND-gate outputs derived from register bits.
    pub pulses: [bool; 6],

    /// Per-bit gate outputs for the low eight bits of the register.
    pub gates: [bool; 8],

    /// Clock divided by 2.
    pub div2: bool,

    /// Clock divided by 4.
    pub div4: bool,

    /// Random continuous-controller value in the range 0--127.
    pub noise_cc: u8,

    /// Raw shift-register state.
    pub register_bits: u16,

    /// Currently active loop length.
    pub length: usize,

    /// Currently active write probability (0.0 = locked, 1.0 = random).
    pub write_probability: f32,
}
