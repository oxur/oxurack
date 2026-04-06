//! Top-level Turing Machine engine.
//!
//! [`TuringMachine`] wires together the shift register, write knob, length
//! selector, quantizers, and clock dividers into a single step-driven
//! sequencer.  Each call to [`tick`](TuringMachine::tick) advances the
//! internal state by one clock pulse and returns a [`StepOutputs`] snapshot
//! of every output.

use std::fmt;
use std::num::NonZeroU32;
use std::ops::RangeInclusive;

use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};

use crate::clock::ClockDivider;
use crate::length::LengthSelector;
use crate::outputs::StepOutputs;
use crate::quantizer::{Quantizer, Scale};
use crate::shift_register::ShiftRegister;
use crate::write_knob::WriteKnob;

/// A complete MIDI Turing Machine engine.
///
/// Combines a 16-bit shift register, write-probability knob, loop-length
/// selector, two quantizers (main and scale output), and two clock dividers
/// into a single step-driven sequencer.
#[derive(Debug)]
pub struct TuringMachine {
    register: ShiftRegister,
    write_knob: WriteKnob,
    length: LengthSelector,
    quantizer: Quantizer,
    scale_quantizer: Quantizer,
    div2: ClockDivider,
    div4: ClockDivider,
    rng: SmallRng,
    step_count: u64,
}

impl TuringMachine {
    /// Creates a new `TuringMachine` with default settings.
    ///
    /// Defaults:
    /// - Length: 16
    /// - Write probability: 0.5
    /// - Scale: chromatic
    /// - Root: C (0)
    /// - Note range: 36..=84 (C2–C6)
    /// - RNG seeded from the operating system
    #[must_use]
    pub fn new() -> Self {
        Self::build(SmallRng::from_rng(&mut rand::rng()))
    }

    /// Creates a new `TuringMachine` with a deterministic seed.
    ///
    /// Two engines built with the same seed will produce identical output
    /// sequences, making this constructor ideal for testing and
    /// reproducibility.
    #[must_use]
    pub fn with_seed(seed: u64) -> Self {
        Self::build(SmallRng::seed_from_u64(seed))
    }

    /// Shared constructor logic.
    fn build(mut rng: SmallRng) -> Self {
        let register = ShiftRegister::new_random(&mut rng);
        let mut length = LengthSelector::new();
        length.set_length(16);

        Self {
            register,
            write_knob: WriteKnob::new(0.5),
            length,
            quantizer: Quantizer::new(Scale::chromatic(), 0),
            scale_quantizer: Quantizer::new(Scale::chromatic(), 0),
            div2: ClockDivider::new(NonZeroU32::new(2).unwrap()),
            div4: ClockDivider::new(NonZeroU32::new(4).unwrap()),
            rng,
            step_count: 0,
        }
    }

    /// Advances the engine by one clock pulse and returns all outputs.
    ///
    /// Signal path:
    /// 1. Read the feedback bit from the loop window.
    /// 2. Resolve the write decision (keep or randomize).
    /// 3. Clock the shift register with the resolved bit.
    /// 4. Read the DAC byte and derive note, velocity, gate, and auxiliary
    ///    outputs.
    /// 5. Tick the clock dividers.
    /// 6. Increment the step counter.
    pub fn tick(&mut self) -> StepOutputs {
        let outputs = self.step_inner(true);
        self.step_count += 1;
        outputs
    }

    /// Resets the engine to its initial state.
    ///
    /// Clears the shift register, resets both clock dividers, and zeroes
    /// the step counter.  Scale, root, write probability, and length
    /// settings are preserved.
    pub fn reset(&mut self) {
        self.register.reset();
        self.div2.reset();
        self.div4.reset();
        self.step_count = 0;
    }

    /// Advances the register by one step without ticking clock dividers
    /// or incrementing the step counter.
    ///
    /// Useful for "preview" or manual-advance scenarios where the master
    /// clock should not progress.
    pub fn move_step(&mut self) -> StepOutputs {
        self.step_inner(false)
    }

    // -- Parameter setters ----------------------------------------------------

    /// Sets the write-knob probability (0.0 = fully random, 1.0 = locked).
    pub fn set_write(&mut self, probability: f32) {
        self.write_knob.set_probability(probability);
    }

    /// Adds an offset to the current write probability (result is clamped).
    pub fn modulate_write(&mut self, offset: f32) {
        self.write_knob.modulate(offset);
    }

    /// Sets the length-selector rotary-switch position (0..=8).
    pub fn set_length_position(&mut self, pos: usize) {
        self.length.set_position(pos);
    }

    /// Sets the loop length to the nearest valid value.
    pub fn set_length(&mut self, len: usize) {
        self.length.set_length(len);
    }

    /// Replaces the main quantizer's scale.
    pub fn set_scale(&mut self, scale: Scale) {
        self.quantizer.set_scale(scale);
    }

    /// Sets the main quantizer's root note (0 = C, …, 11 = B).
    pub fn set_root(&mut self, root: u8) {
        self.quantizer.set_root(root);
    }

    /// Sets the main quantizer's MIDI note output range.
    pub fn set_note_range(&mut self, range: RangeInclusive<u8>) {
        self.quantizer.set_range(range);
    }

    /// Replaces the scale-output quantizer's scale.
    pub fn set_scale_output_scale(&mut self, scale: Scale) {
        self.scale_quantizer.set_scale(scale);
    }

    /// Sets the scale-output quantizer's root note (0 = C, …, 11 = B).
    pub fn set_scale_output_root(&mut self, root: u8) {
        self.scale_quantizer.set_root(root);
    }

    // -- Inspection -----------------------------------------------------------

    /// Returns the raw 16-bit shift-register contents.
    #[must_use]
    pub fn register_bits(&self) -> u16 {
        self.register.bits()
    }

    /// Returns the currently active loop length.
    #[must_use]
    pub fn current_length(&self) -> usize {
        self.length.length()
    }

    /// Returns the current write-knob probability.
    #[must_use]
    pub fn write_probability(&self) -> f32 {
        self.write_knob.probability()
    }

    /// Returns the number of ticks processed since creation or the last
    /// reset.
    #[must_use]
    pub fn step_count(&self) -> u64 {
        self.step_count
    }

    // -- Private helpers ------------------------------------------------------

    /// Core step logic shared by [`tick`] and [`move_step`].
    ///
    /// When `advance_clock` is `true` the clock dividers are ticked;
    /// otherwise their outputs are reported as `false`.
    fn step_inner(&mut self, advance_clock: bool) -> StepOutputs {
        let len = self.length.length();
        let fb = self.register.feedback_bit(len);
        let new_bit = self.write_knob.resolve(fb, &mut self.rng);
        self.register.clock(new_bit);

        let dac = self.register.dac_byte(len);

        // Per-bit gate outputs (low 8 bits of the register).
        let bit = |pos: usize| -> bool { (self.register.bits() >> pos) & 1 == 1 };

        let mut gates = [false; 8];
        for (n, gate) in gates.iter_mut().enumerate() {
            *gate = bit(n);
        }

        // AND-gate pulse outputs: pulse[n] = bit(n) && bit(n+1).
        let mut pulses = [false; 6];
        for (n, pulse) in pulses.iter_mut().enumerate() {
            *pulse = bit(n) && bit(n + 1);
        }

        let (d2, d4) = if advance_clock {
            (self.div2.tick(), self.div4.tick())
        } else {
            (false, false)
        };

        StepOutputs {
            note: Some(self.quantizer.note_from_dac(dac)),
            velocity: Some(self.quantizer.velocity_from_dac(dac)),
            gate: self.register.pulse_bit(len),
            scale_note: Some(self.scale_quantizer.note_from_dac(dac)),
            pulses,
            gates,
            div2: d2,
            div4: d4,
            noise_cc: self.rng.random::<u8>() & 0x7F,
            register_bits: self.register.bits(),
            length: len,
            write_probability: self.write_knob.probability(),
        }
    }
}

impl Default for TuringMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.length.length();
        let bools = self.register.to_bools();
        let split = 16 - len;
        for (i, &b) in bools.iter().enumerate() {
            if i == split {
                f.write_str("[")?;
            }
            f.write_str(if b { "1" } else { "0" })?;
        }
        f.write_str("]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn deterministic_with_seed() {
        let mut a = TuringMachine::with_seed(42);
        let mut b = TuringMachine::with_seed(42);

        let outputs_a: Vec<StepOutputs> = (0..32).map(|_| a.tick()).collect();
        let outputs_b: Vec<StepOutputs> = (0..32).map(|_| b.tick()).collect();

        assert_eq!(outputs_a, outputs_b);
    }

    #[test]
    fn locked_loop_repeats() {
        let mut tm = TuringMachine::with_seed(99);
        tm.set_write(1.0);
        tm.set_length(8);

        let outputs: Vec<StepOutputs> = (0..16).map(|_| tm.tick()).collect();

        // After `length` ticks the loop window has fully cycled.  Ticks
        // (length+1) through (2*length) should reproduce ticks 1 through
        // length.  We compare notes because clock-divider state differs.
        for i in 0..8 {
            assert_eq!(
                outputs[i].note,
                outputs[i + 8].note,
                "note mismatch at step {i}"
            );
            assert_eq!(
                outputs[i].velocity,
                outputs[i + 8].velocity,
                "velocity mismatch at step {i}"
            );
            assert_eq!(
                outputs[i].gate,
                outputs[i + 8].gate,
                "gate mismatch at step {i}"
            );
        }
    }

    #[test]
    fn fully_random_no_repeat() {
        let mut tm = TuringMachine::with_seed(7);
        tm.set_write(0.0);

        let outputs: Vec<StepOutputs> = (0..64).map(|_| tm.tick()).collect();
        let first_note = outputs[0].note;
        let all_same = outputs.iter().all(|o| o.note == first_note);

        assert!(
            !all_same,
            "at write=0.0, 64 ticks should not all produce the same note"
        );
    }

    #[test]
    fn reset_zeroes_register() {
        let mut tm = TuringMachine::with_seed(42);
        for _ in 0..10 {
            tm.tick();
        }
        tm.reset();
        assert_eq!(tm.register_bits(), 0);
    }

    #[test]
    fn display_shows_brackets() {
        let mut tm = TuringMachine::with_seed(42);
        tm.set_length(8);

        let display = tm.to_string();

        assert!(display.contains('['), "display should contain '['");
        assert!(display.contains(']'), "display should contain ']'");
        assert_eq!(display.len(), 18, "display should be 18 chars: {display}");

        // Exactly 8 characters between the brackets.
        let open = display.find('[').unwrap();
        let close = display.find(']').unwrap();
        let between = close - open - 1;
        assert_eq!(
            between, 8,
            "expected 8 chars between brackets, got {between}: {display}"
        );
    }

    #[test]
    fn step_count_increments() {
        let mut tm = TuringMachine::with_seed(42);
        assert_eq!(tm.step_count(), 0);
        tm.tick();
        assert_eq!(tm.step_count(), 1);
    }

    #[test]
    fn pulses_are_and_of_adjacent_bits() {
        let mut tm = TuringMachine::with_seed(42);
        let outputs = tm.tick();

        for n in 0..6 {
            assert_eq!(
                outputs.pulses[n],
                outputs.gates[n] && outputs.gates[n + 1],
                "pulse[{n}] should equal gate[{n}] && gate[{}]",
                n + 1
            );
        }
    }
}
