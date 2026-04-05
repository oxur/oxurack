//! Scale quantization for a MIDI Turing Machine.
//!
//! Maps raw DAC byte values to MIDI notes (quantized to a musical scale)
//! and velocities. The [`Quantizer`] accepts an 8-bit DAC sample and
//! produces a note within a configurable pitch range, snapped to the
//! nearest note in the active [`Scale`].

use std::ops::RangeInclusive;

// ---------------------------------------------------------------------------
// Scale
// ---------------------------------------------------------------------------

/// A musical scale defined by its semitone intervals within a single octave.
///
/// Intervals are stored as sorted, deduplicated offsets in the range 0–11.
/// Built-in constructors are provided for the most common Western scales.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Scale {
    intervals: Vec<u8>,
    name: String,
}

impl Scale {
    /// Creates a new scale from a list of semitone offsets (0–11).
    ///
    /// The intervals are sorted, deduplicated, and filtered to the range
    /// 0–11. If the resulting set is empty, interval `0` (the root) is
    /// inserted automatically.
    #[must_use]
    pub fn new(intervals: Vec<u8>, name: impl Into<String>) -> Self {
        let mut ivs: Vec<u8> = intervals.into_iter().filter(|&i| i < 12).collect();
        ivs.sort_unstable();
        ivs.dedup();
        if ivs.is_empty() {
            ivs.push(0);
        }
        Self {
            intervals: ivs,
            name: name.into(),
        }
    }

    /// Returns the display name of this scale.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the semitone intervals that define this scale.
    #[must_use]
    pub fn intervals(&self) -> &[u8] {
        &self.intervals
    }

    /// Snaps `raw_note` to the nearest note that belongs to this scale
    /// (relative to `root`).
    ///
    /// A MIDI note `N` is considered "in scale" when
    /// `(N as i16 - root as i16).rem_euclid(12)` appears in
    /// `self.intervals`.
    ///
    /// When two candidate notes are equidistant, the lower note is
    /// preferred. The result is clamped to the MIDI range 0–127.
    #[must_use]
    pub fn quantize(&self, raw_note: u8, root: u8) -> u8 {
        // Check raw_note itself first.
        if self.is_in_scale(raw_note, root) {
            return raw_note;
        }

        // Search outward: distance 1, 2, 3, …
        // On ties, prefer the lower note (check below first).
        for distance in 1..=127i16 {
            let below = raw_note as i16 - distance;
            let above = raw_note as i16 + distance;

            if below >= 0 && self.is_in_scale(below as u8, root) {
                return below as u8;
            }
            if above <= 127 && self.is_in_scale(above as u8, root) {
                return above as u8;
            }
        }

        // Fallback (should be unreachable for any non-degenerate scale).
        raw_note
    }

    /// Returns `true` if `note` belongs to this scale relative to `root`.
    fn is_in_scale(&self, note: u8, root: u8) -> bool {
        let degree = (note as i16 - root as i16).rem_euclid(12) as u8;
        self.intervals.contains(&degree)
    }

    // -- Built-in scale constructors ----------------------------------------

    /// Chromatic scale — all twelve semitones.
    #[must_use]
    pub fn chromatic() -> Self {
        Self::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], "Chromatic")
    }

    /// Major (Ionian) scale.
    #[must_use]
    pub fn major() -> Self {
        Self::new(vec![0, 2, 4, 5, 7, 9, 11], "Major")
    }

    /// Natural minor (Aeolian) scale.
    #[must_use]
    pub fn natural_minor() -> Self {
        Self::new(vec![0, 2, 3, 5, 7, 8, 10], "Natural Minor")
    }

    /// Harmonic minor scale.
    #[must_use]
    pub fn harmonic_minor() -> Self {
        Self::new(vec![0, 2, 3, 5, 7, 8, 11], "Harmonic Minor")
    }

    /// Major pentatonic scale.
    #[must_use]
    pub fn pentatonic_major() -> Self {
        Self::new(vec![0, 2, 4, 7, 9], "Pentatonic Major")
    }

    /// Minor pentatonic scale.
    #[must_use]
    pub fn pentatonic_minor() -> Self {
        Self::new(vec![0, 3, 5, 7, 10], "Pentatonic Minor")
    }

    /// Blues scale.
    #[must_use]
    pub fn blues() -> Self {
        Self::new(vec![0, 3, 5, 6, 7, 10], "Blues")
    }

    /// Dorian mode.
    #[must_use]
    pub fn dorian() -> Self {
        Self::new(vec![0, 2, 3, 5, 7, 9, 10], "Dorian")
    }

    /// Phrygian mode.
    #[must_use]
    pub fn phrygian() -> Self {
        Self::new(vec![0, 1, 3, 5, 7, 8, 10], "Phrygian")
    }

    /// Lydian mode.
    #[must_use]
    pub fn lydian() -> Self {
        Self::new(vec![0, 2, 4, 6, 7, 9, 11], "Lydian")
    }

    /// Mixolydian mode.
    #[must_use]
    pub fn mixolydian() -> Self {
        Self::new(vec![0, 2, 4, 5, 7, 9, 10], "Mixolydian")
    }

    /// Whole-tone scale.
    #[must_use]
    pub fn whole_tone() -> Self {
        Self::new(vec![0, 2, 4, 6, 8, 10], "Whole Tone")
    }

    /// Diminished (octatonic) scale.
    #[must_use]
    pub fn diminished() -> Self {
        Self::new(vec![0, 2, 3, 5, 6, 8, 9, 11], "Diminished")
    }

    /// Augmented scale.
    #[must_use]
    pub fn augmented() -> Self {
        Self::new(vec![0, 3, 4, 7, 8, 11], "Augmented")
    }
}

// ---------------------------------------------------------------------------
// Quantizer
// ---------------------------------------------------------------------------

/// Maps raw 8-bit DAC values to scale-quantized MIDI notes and velocities.
///
/// The DAC byte is linearly mapped into a configurable pitch [`range`],
/// then snapped to the nearest note in the active [`Scale`].
///
/// [`range`]: Quantizer::set_range
#[derive(Debug, Clone, PartialEq)]
pub struct Quantizer {
    scale: Scale,
    root: u8,
    range: RangeInclusive<u8>,
}

impl Quantizer {
    /// Creates a new quantizer with the given scale and root note.
    ///
    /// `root` is clamped to 0–11. The default output range is MIDI
    /// notes 36–84 (C2–C6).
    #[must_use]
    pub fn new(scale: Scale, root: u8) -> Self {
        Self {
            scale,
            root: root.min(11),
            range: 36..=84,
        }
    }

    /// Replaces the active scale.
    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
    }

    /// Sets the root note (0 = C, 1 = C#, …, 11 = B), clamped to 0–11.
    pub fn set_root(&mut self, root: u8) {
        self.root = root.min(11);
    }

    /// Sets the MIDI note output range.
    pub fn set_range(&mut self, range: RangeInclusive<u8>) {
        self.range = range;
    }

    /// Converts a raw 8-bit DAC value to a scale-quantized MIDI note.
    ///
    /// 1. Linearly maps `dac` (0–255) into the configured pitch range.
    /// 2. Quantizes the result to the nearest in-scale note.
    /// 3. Clamps to the range and ensures the output is at least 1
    ///    (MIDI note 0 is never produced).
    #[must_use]
    pub fn note_from_dac(&self, dac: u8) -> u8 {
        let lo = *self.range.start() as u16;
        let hi = *self.range.end() as u16;
        let span = hi.saturating_sub(lo);

        // Linear interpolation: dac 0 → lo, dac 255 → hi.
        let raw = lo + (u16::from(dac) * span + 127) / 255;
        let raw = raw.min(127) as u8;

        let quantized = self.scale.quantize(raw, self.root);

        quantized
            .clamp(*self.range.start(), *self.range.end())
            .max(1)
    }

    /// Converts a raw 8-bit DAC value to a MIDI velocity (1–127).
    ///
    /// Uses the lower 7 bits of `dac`, with a minimum of 1 so that
    /// velocity zero (note-off) is never produced.
    #[must_use]
    pub fn velocity_from_dac(&self, dac: u8) -> u8 {
        (dac & 0x7F).max(1)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chromatic_passthrough() {
        let q = Quantizer {
            scale: Scale::chromatic(),
            root: 0,
            range: 0..=127,
        };

        // With chromatic scale and full range, every note is valid.
        // The mapping should be approximately linear.
        assert_eq!(q.note_from_dac(0), 1, "MIDI note 0 must never be output");
        assert!(q.note_from_dac(255) <= 127);

        // Mid-range DAC should map near the middle of 0–127.
        let mid = q.note_from_dac(128);
        assert!(
            (58..=68).contains(&mid),
            "mid-range DAC should map near 64, got {mid}"
        );
    }

    #[test]
    fn test_major_snaps_correctly() {
        let scale = Scale::major();
        // C major, root = 0. C# (61) is not in the scale.
        // Nearest in-scale notes: C4 = 60, D4 = 62.
        // Equidistant (both 1 away), prefer lower → 60.
        assert_eq!(scale.quantize(61, 0), 60);
    }

    #[test]
    fn test_range_respected() {
        let q = Quantizer::new(Scale::major(), 0);
        // Default range is 36..=84.
        assert!(q.note_from_dac(0) >= 36, "must be >= range start");
        assert!(q.note_from_dac(255) <= 84, "must be <= range end");
    }

    #[test]
    fn test_velocity_never_zero() {
        let q = Quantizer::new(Scale::chromatic(), 0);
        for dac in 0..=255u8 {
            let vel = q.velocity_from_dac(dac);
            assert!(vel >= 1, "velocity must be >= 1 for dac={dac}, got {vel}");
            assert!(vel <= 127, "velocity must be <= 127 for dac={dac}");
        }
    }

    #[test]
    fn test_pentatonic_quantize() {
        let scale = Scale::pentatonic_major();
        // C pentatonic major: C(0), D(2), E(4), G(7), A(9)
        // Root = 0 (C).

        // C4 = 60 is in scale → stays.
        assert_eq!(scale.quantize(60, 0), 60);

        // C#4 = 61 is NOT in scale. Nearest: C=60 (dist 1) vs D=62 (dist 1).
        // Tie → prefer lower.
        assert_eq!(scale.quantize(61, 0), 60);

        // F4 = 65 is NOT in scale. Nearest: E=64 (dist 1) vs G=67 (dist 2).
        assert_eq!(scale.quantize(65, 0), 64);

        // F#4 = 66 is NOT in scale. Nearest: E=64 (dist 2) vs G=67 (dist 1).
        assert_eq!(scale.quantize(66, 0), 67);

        // Bb4 = 70 is NOT in scale. Nearest: A=69 (dist 1) vs C=72 (dist 2).
        assert_eq!(scale.quantize(70, 0), 69);
    }

    #[test]
    fn test_scale_new_sorts_and_deduplicates() {
        let scale = Scale::new(vec![7, 0, 4, 4, 0, 2], "Test");
        assert_eq!(scale.intervals(), &[0, 2, 4, 7]);
    }

    #[test]
    fn test_scale_new_empty_gets_root() {
        let scale = Scale::new(vec![], "Empty");
        assert_eq!(scale.intervals(), &[0]);
    }

    #[test]
    fn test_scale_new_filters_out_of_range() {
        let scale = Scale::new(vec![0, 5, 12, 15, 200], "Filtered");
        assert_eq!(scale.intervals(), &[0, 5]);
    }

    #[test]
    fn test_scale_name() {
        assert_eq!(Scale::major().name(), "Major");
        assert_eq!(Scale::blues().name(), "Blues");
        assert_eq!(Scale::chromatic().name(), "Chromatic");
    }

    #[test]
    fn test_quantize_at_boundaries() {
        let scale = Scale::major();
        // Note 0 (C) with root 0 → in scale.
        assert_eq!(scale.quantize(0, 0), 0);
        // Note 127 (G9) with root 0 → 127 % 12 = 7 → G, in major scale.
        assert_eq!(scale.quantize(127, 0), 127);
    }

    #[test]
    fn test_quantizer_set_root_clamps() {
        let mut q = Quantizer::new(Scale::chromatic(), 0);
        q.set_root(15);
        // Root should be clamped to 11.
        assert_eq!(q.root, 11);
    }

    #[test]
    fn test_note_from_dac_never_zero() {
        // Even with range starting at 0, output must be >= 1.
        let q = Quantizer {
            scale: Scale::chromatic(),
            root: 0,
            range: 0..=127,
        };
        assert!(q.note_from_dac(0) >= 1);
    }

    #[test]
    fn test_velocity_from_dac_lower_7_bits() {
        let q = Quantizer::new(Scale::chromatic(), 0);
        // 0b1000_0000 = 128 → lower 7 bits = 0 → clamped to 1.
        assert_eq!(q.velocity_from_dac(128), 1);
        // 0b1111_1111 = 255 → lower 7 bits = 127.
        assert_eq!(q.velocity_from_dac(255), 127);
        // 0b0100_0000 = 64 → lower 7 bits = 64.
        assert_eq!(q.velocity_from_dac(64), 64);
    }

    #[test]
    fn test_quantize_with_nonzero_root() {
        let scale = Scale::major();
        // D major (root = 2): D E F# G A B C#
        // In-scale notes around 60: C#4=61 (degree 11=B→no, 61-2=59 → 59%12=11 → C# rel to D
        // Actually: degree = (N - root) % 12
        // N=60: (60-2)%12 = 10 → not in [0,2,4,5,7,9,11] → out
        // N=61: (61-2)%12 = 11 → in scale (C# = leading tone of D major) → stays
        assert_eq!(scale.quantize(61, 2), 61);
        // N=60: nearest in-scale below = 59? (59-2)%12=9 → in scale → yes
        // nearest in-scale above = 61 → in scale
        // dist 1 each way, prefer lower → 59
        assert_eq!(scale.quantize(60, 2), 59);
    }
}
