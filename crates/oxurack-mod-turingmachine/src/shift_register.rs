//! Models four chained CD4015 4-bit shift registers (16 bits total)
//! for a MIDI Turing Machine.
//!
//! Bit layout:
//! - Bit 0 is the **newest** (rightmost) position.
//! - Bit 15 is the **oldest** (leftmost) position.
//!
//! On each clock pulse the register shifts left by one position and
//! the incoming bit is inserted at position 0.

use std::num::NonZeroUsize;

use rand::{Rng, RngExt};

/// A 16-bit shift register modelling four chained CD4015 ICs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftRegister {
    bits: u16,
}

impl ShiftRegister {
    /// Creates a new shift register with all bits cleared.
    #[must_use]
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    /// Creates a new shift register filled with random bits.
    #[must_use]
    pub fn new_random(rng: &mut impl Rng) -> Self {
        Self {
            bits: rng.random::<u16>(),
        }
    }

    /// Shifts the register left by one position and inserts `new_bit`
    /// at position 0.
    pub fn clock(&mut self, new_bit: bool) {
        self.bits = (self.bits << 1) | u16::from(new_bit);
    }

    /// Returns the feedback bit — the bit at position `length - 1`.
    #[must_use]
    pub fn feedback_bit(&self, length: NonZeroUsize) -> bool {
        (self.bits >> (length.get() - 1)) & 1 == 1
    }

    /// Returns the raw 16-bit contents of the register.
    #[must_use]
    pub fn bits(&self) -> u16 {
        self.bits
    }

    /// Returns the 8-bit value the DAC0808 would see.
    ///
    /// The DAC reads the top 8 bits of the active loop window, i.e.
    /// bits `[length-1, length-2, …, length-8]`.  When `length < 8`
    /// the lower bits come from below the loop window, matching the
    /// hardware behaviour.
    #[must_use]
    pub fn dac_byte(&self, length: NonZeroUsize) -> u8 {
        ((self.bits >> length.get().saturating_sub(8)) & 0xFF) as u8
    }

    /// Returns the MSB (bit 7) of [`dac_byte`](Self::dac_byte), which
    /// drives the pulse output on the hardware.
    #[must_use]
    pub fn pulse_bit(&self, length: NonZeroUsize) -> bool {
        (self.dac_byte(length) >> 7) & 1 == 1
    }

    /// Converts the register to an array of bools.
    ///
    /// Index 0 corresponds to bit 15 (oldest / leftmost) and index 15
    /// corresponds to bit 0 (newest / rightmost).
    #[must_use]
    pub fn to_bools(&self) -> [bool; 16] {
        let mut out = [false; 16];
        for (i, slot) in out.iter_mut().enumerate() {
            *slot = (self.bits >> (15 - i)) & 1 == 1;
        }
        out
    }

    /// Clears all bits to zero.
    pub fn reset(&mut self) {
        self.bits = 0;
    }
}

impl Default for ShiftRegister {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn nz(n: usize) -> NonZeroUsize {
        NonZeroUsize::new(n).unwrap()
    }

    #[test]
    fn test_new_returns_zeroed_register() {
        let sr = ShiftRegister::new();
        assert_eq!(sr.bits(), 0);
    }

    #[test]
    fn test_default_delegates_to_new() {
        assert_eq!(ShiftRegister::default(), ShiftRegister::new());
    }

    #[test]
    fn test_new_random_produces_value() {
        let mut rng = rand::rng();
        let sr = ShiftRegister::new_random(&mut rng);
        // We cannot assert a specific value, but we can confirm it
        // round-trips through bits().
        let _ = sr.bits();
    }

    #[test]
    fn test_clock_shifts_left() {
        let mut sr = ShiftRegister::new();

        sr.clock(true);
        assert_eq!(sr.bits(), 0b1);

        sr.clock(false);
        assert_eq!(sr.bits(), 0b10);

        sr.clock(true);
        assert_eq!(sr.bits(), 0b101);
    }

    #[test]
    fn test_feedback_bit_respects_length() {
        let mut sr = ShiftRegister::new();
        // Clock in pattern: bits = 0b1010  (bit3=1, bit2=0, bit1=1, bit0=0)
        sr.clock(true); // 0b1
        sr.clock(false); // 0b10
        sr.clock(true); // 0b101
        sr.clock(false); // 0b1010

        // length=4 → feedback is bit 3
        assert!(sr.feedback_bit(nz(4)));

        // length=3 → feedback is bit 2
        assert!(!sr.feedback_bit(nz(3)));

        // length=2 → feedback is bit 1
        assert!(sr.feedback_bit(nz(2)));
    }

    #[test]
    fn test_dac_byte_length_16_is_upper_byte() {
        let sr = ShiftRegister { bits: 0xAB_CD };

        // length=16 → shift right by 8 → upper byte 0xAB
        assert_eq!(sr.dac_byte(nz(16)), 0xAB);
    }

    #[test]
    fn test_dac_byte_length_8_is_lower_byte() {
        let sr = ShiftRegister { bits: 0xAB_CD };

        // length=8 → shift right by 0 → lower byte 0xCD
        assert_eq!(sr.dac_byte(nz(8)), 0xCD);
    }

    #[test]
    fn test_dac_byte_length_less_than_8() {
        let sr = ShiftRegister { bits: 0x00_3F };

        // length=4 → saturating_sub(8) = 0 → lower byte = 0x3F
        assert_eq!(sr.dac_byte(nz(4)), 0x3F);
    }

    #[test]
    fn test_pulse_bit_is_msb_of_dac_byte() {
        let sr = ShiftRegister { bits: 0xFF_00 };

        // length=16 → dac_byte = 0xFF → MSB = 1
        assert!(sr.pulse_bit(nz(16)));

        // length=8 → dac_byte = 0x00 → MSB = 0
        assert!(!sr.pulse_bit(nz(8)));
    }

    #[test]
    fn test_to_bools_order() {
        let sr = ShiftRegister {
            bits: 0b1000_0000_0000_0001,
        };
        let bools = sr.to_bools();

        // Index 0 = bit 15 (oldest) = true
        assert!(bools[0]);
        // Index 15 = bit 0 (newest) = true
        assert!(bools[15]);
        // Everything else is false
        for (i, val) in bools.iter().enumerate().skip(1).take(14) {
            assert!(!val, "expected false at index {i}");
        }
    }

    #[test]
    fn test_reset_clears_register() {
        let mut sr = ShiftRegister { bits: 0xFFFF };
        sr.reset();
        assert_eq!(sr.bits(), 0);
    }

    #[test]
    fn test_feedback_bit_length_one() {
        let mut sr = ShiftRegister::new();
        sr.clock(true); // bit 0 = 1
        assert!(sr.feedback_bit(nz(1)));
        sr.clock(false); // bit 0 = 0
        assert!(!sr.feedback_bit(nz(1)));
    }

    #[test]
    fn test_feedback_bit_maximum_length() {
        let sr = ShiftRegister { bits: 0x8000 }; // bit 15 = 1
        assert!(sr.feedback_bit(nz(16)));
        let sr = ShiftRegister { bits: 0x7FFF }; // bit 15 = 0
        assert!(!sr.feedback_bit(nz(16)));
    }

    #[test]
    fn test_clock_overflow_discards_bit_16() {
        let mut sr = ShiftRegister { bits: 0xFFFF };
        sr.clock(false);
        // All bits shifted left, bit 15 fell off, bit 0 = false
        assert_eq!(sr.bits(), 0xFFFE);
    }
}
