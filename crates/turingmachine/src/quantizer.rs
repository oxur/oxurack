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

        self.scale
            .quantize(raw, self.root)
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

    // -- set_root clamping ---------------------------------------------------

    #[test]
    fn test_quantizer_set_root_wraps_or_clamps() {
        let mut q = Quantizer::new(Scale::chromatic(), 0);
        q.set_root(12);
        assert_eq!(q.root, 11, "root 12 should be clamped to 11");
        q.set_root(255);
        assert_eq!(q.root, 11, "root 255 should be clamped to 11");
    }

    // -- root-only scale -----------------------------------------------------

    #[test]
    fn test_scale_root_only() {
        // Scale with only interval 0: only root notes (multiples of 12
        // relative to root) are valid.
        let scale = Scale::new(vec![0], "Root Only");

        // With root = 0, valid notes are 0, 12, 24, 36, …
        // Quantize 5 → nearest root note is 0 (dist 5) vs 12 (dist 7) → 0.
        assert_eq!(scale.quantize(5, 0), 0);
        // Quantize 7 → nearest is 12 (dist 5) vs 0 (dist 7).
        // But tie-break: below first. 0 is dist 7, 12 is dist 5 → 12 wins.
        assert_eq!(scale.quantize(7, 0), 12);
        // Quantize 6 → 0 is dist 6, 12 is dist 6 → tie, prefer lower → 0.
        assert_eq!(scale.quantize(6, 0), 0);
        // Quantize 60 → exactly on a root note → stays.
        assert_eq!(scale.quantize(60, 0), 60);
    }

    // -- zero-width range (start == end) -------------------------------------

    #[test]
    fn test_note_from_dac_zero_range() {
        let q = Quantizer {
            scale: Scale::chromatic(),
            root: 0,
            range: 60..=60,
        };
        // Any DAC value should produce note 60.
        assert_eq!(q.note_from_dac(0), 60);
        assert_eq!(q.note_from_dac(128), 60);
        assert_eq!(q.note_from_dac(255), 60);
    }

    // -- exact match in quantize ---------------------------------------------

    #[test]
    fn test_quantize_exact_match() {
        let scale = Scale::major();
        // C major: C(0), D(2), E(4), F(5), G(7), A(9), B(11)
        // C4 = 60 → (60-0)%12 = 0 → in scale → returned unchanged.
        assert_eq!(scale.quantize(60, 0), 60);
        // G4 = 67 → (67-0)%12 = 7 → in scale → returned unchanged.
        assert_eq!(scale.quantize(67, 0), 67);
        // B4 = 71 → (71-0)%12 = 11 → in scale → returned unchanged.
        assert_eq!(scale.quantize(71, 0), 71);
    }

    // -- velocity_from_dac edge cases ----------------------------------------

    #[test]
    fn test_velocity_from_dac_zero() {
        let q = Quantizer::new(Scale::chromatic(), 0);
        // DAC 0: lower 7 bits = 0, clamped to minimum velocity 1.
        assert_eq!(q.velocity_from_dac(0), 1);
    }

    #[test]
    fn test_velocity_from_dac_max() {
        let q = Quantizer::new(Scale::chromatic(), 0);
        // DAC 255: lower 7 bits = 127.
        assert_eq!(q.velocity_from_dac(255), 127);
    }

    // -- note_from_dac with different range widths ---------------------------

    #[test]
    fn test_note_from_dac_full_range() {
        let q = Quantizer {
            scale: Scale::chromatic(),
            root: 0,
            range: 0..=127,
        };
        // Verify that low DAC maps near the bottom and high DAC maps near
        // the top.
        let low = q.note_from_dac(0);
        let high = q.note_from_dac(255);
        assert!(low >= 1, "note must be >= 1, got {low}");
        assert!(high <= 127, "note must be <= 127, got {high}");
        // The full range should produce meaningful spread.
        assert!(
            high - low > 100,
            "full range should span at least 100 notes, got {}",
            high - low
        );
    }

    #[test]
    fn test_note_from_dac_narrow_range() {
        let q = Quantizer {
            scale: Scale::chromatic(),
            root: 0,
            range: 60..=72,
        };
        // Every DAC value should produce a note within 60..=72.
        for dac in 0..=255u8 {
            let note = q.note_from_dac(dac);
            assert!(
                (60..=72).contains(&note),
                "note {note} out of range 60..=72 for dac={dac}"
            );
        }
    }

    // -- Quantizer::set_scale -------------------------------------------------

    #[test]
    fn test_quantizer_set_scale() {
        let mut q = Quantizer::new(Scale::chromatic(), 0);
        q.set_scale(Scale::major());

        // After switching to major, quantize should snap to major scale
        // notes. C#4 (61) is not in C major, so it should snap.
        let note = q.note_from_dac(128);
        // The note should be in C major: degree (note % 12) in [0,2,4,5,7,9,11].
        let degree = note % 12;
        assert!(
            [0, 2, 4, 5, 7, 9, 11].contains(&degree),
            "note {note} (degree {degree}) should be in C major after set_scale"
        );
    }

    // -- Quantizer::set_range -------------------------------------------------

    #[test]
    fn test_quantizer_set_range() {
        let mut q = Quantizer::new(Scale::chromatic(), 0);
        q.set_range(48..=72);

        for dac in 0..=255u8 {
            let note = q.note_from_dac(dac);
            assert!(
                (48..=72).contains(&note),
                "note {note} should be within 48..=72 for dac={dac}"
            );
        }
    }

    // -- quantize: above branch (near note 0) ---------------------------------

    #[test]
    fn test_quantize_near_zero_finds_above() {
        // Pentatonic major with root 0: intervals [0, 2, 4, 7, 9].
        // Note 1 is out of scale. Below: 0 (dist 1, in scale). Above: 2 (dist 1).
        // Tie -> prefer lower -> 0.
        let scale = Scale::pentatonic_major();
        assert_eq!(scale.quantize(1, 0), 0);

        // With root = 2: in-scale notes at 2, 4, 6, 9, 11, 14, ...
        // Note 0: below is negative, only above works. Nearest above: 2 (dist 2).
        assert_eq!(scale.quantize(0, 2), 2);

        // Note 1: below = 0 (not in scale with root=2, (0-2)%12 = 10, not in
        // [0,2,4,7,9]). Above = 2 (in scale, (2-2)%12=0). dist = 1.
        // Check below: note 0, (0-2) rem_euclid 12 = 10, not in scale.
        // So above = 2 is the nearest at dist 1.
        assert_eq!(scale.quantize(1, 2), 2);
    }

    #[test]
    fn test_quantize_note_zero_not_in_scale() {
        // Root-only scale with root = 5 (F): in-scale notes are 5, 17, 29, ...
        // Note 0: below < 0 so only above branch works. Nearest: 5 (dist 5).
        let scale = Scale::new(vec![0], "Root Only");
        assert_eq!(scale.quantize(0, 5), 5);
    }

    // -- quantize: below branch only (near note 127) --------------------------

    #[test]
    fn test_quantize_near_127_finds_below() {
        // Root-only scale with root = 0: in-scale notes are 0, 12, 24, ..., 120.
        // Note 127: (127-0)%12 = 7, not in scale.
        // Above: 128, 129, ... all > 127, so above branch never matches.
        // Below: 126 (not in scale), ..., 120 ((120-0)%12=0, in scale). dist = 7.
        let scale = Scale::new(vec![0], "Root Only");
        assert_eq!(scale.quantize(127, 0), 120);
    }

    #[test]
    fn test_quantize_note_126_root_only() {
        // Note 126: (126-0)%12 = 6, not in scale [0].
        // Below: 125(5), 124(4), 123(3), 122(2), 121(1), 120(0) -> 120 at dist 6.
        // Above: 127(7) not in scale, 128+ out of range.
        let scale = Scale::new(vec![0], "Root Only");
        assert_eq!(scale.quantize(126, 0), 120);
    }

    // -- Built-in scale constructors ------------------------------------------

    #[test]
    fn test_natural_minor_intervals() {
        let scale = Scale::natural_minor();
        assert_eq!(scale.intervals(), &[0, 2, 3, 5, 7, 8, 10]);
        assert_eq!(scale.name(), "Natural Minor");
    }

    #[test]
    fn test_harmonic_minor_intervals() {
        let scale = Scale::harmonic_minor();
        assert_eq!(scale.intervals(), &[0, 2, 3, 5, 7, 8, 11]);
        assert_eq!(scale.name(), "Harmonic Minor");
    }

    #[test]
    fn test_pentatonic_minor_intervals() {
        let scale = Scale::pentatonic_minor();
        assert_eq!(scale.intervals(), &[0, 3, 5, 7, 10]);
        assert_eq!(scale.name(), "Pentatonic Minor");
    }

    #[test]
    fn test_dorian_intervals() {
        let scale = Scale::dorian();
        assert_eq!(scale.intervals(), &[0, 2, 3, 5, 7, 9, 10]);
        assert_eq!(scale.name(), "Dorian");
    }

    #[test]
    fn test_phrygian_intervals() {
        let scale = Scale::phrygian();
        assert_eq!(scale.intervals(), &[0, 1, 3, 5, 7, 8, 10]);
        assert_eq!(scale.name(), "Phrygian");
    }

    #[test]
    fn test_lydian_intervals() {
        let scale = Scale::lydian();
        assert_eq!(scale.intervals(), &[0, 2, 4, 6, 7, 9, 11]);
        assert_eq!(scale.name(), "Lydian");
    }

    #[test]
    fn test_mixolydian_intervals() {
        let scale = Scale::mixolydian();
        assert_eq!(scale.intervals(), &[0, 2, 4, 5, 7, 9, 10]);
        assert_eq!(scale.name(), "Mixolydian");
    }

    #[test]
    fn test_whole_tone_intervals() {
        let scale = Scale::whole_tone();
        assert_eq!(scale.intervals(), &[0, 2, 4, 6, 8, 10]);
        assert_eq!(scale.name(), "Whole Tone");
    }

    #[test]
    fn test_diminished_intervals() {
        let scale = Scale::diminished();
        assert_eq!(scale.intervals(), &[0, 2, 3, 5, 6, 8, 9, 11]);
        assert_eq!(scale.name(), "Diminished");
    }

    #[test]
    fn test_augmented_intervals() {
        let scale = Scale::augmented();
        assert_eq!(scale.intervals(), &[0, 3, 4, 7, 8, 11]);
        assert_eq!(scale.name(), "Augmented");
    }

    // -- quantize with various built-in scales --------------------------------

    #[test]
    fn test_quantize_harmonic_minor() {
        let scale = Scale::harmonic_minor();
        // A harmonic minor (root = 9): A B C D E F G#
        // In-scale degrees: [0,2,3,5,7,8,11].
        // Note 70 (Bb4): (70-9)%12 = 1, not in scale.
        // Below: 69 ((69-9)%12=0, in scale) dist 1. -> 69.
        assert_eq!(scale.quantize(70, 9), 69);
    }

    #[test]
    fn test_quantize_blues() {
        let scale = Scale::blues();
        // C blues: C Eb F F# G Bb -> [0, 3, 5, 6, 7, 10].
        // Note 62 (D4): (62-0)%12 = 2, not in scale.
        // Below: 61 (1, not in scale), 60 (0, in scale) dist 2.
        // Above: 63 (3, in scale) dist 1. -> 63.
        assert_eq!(scale.quantize(62, 0), 63);
    }

    #[test]
    fn test_quantize_whole_tone() {
        let scale = Scale::whole_tone();
        // C whole tone: C D E F# G# A# -> [0, 2, 4, 6, 8, 10].
        // Note 61 (C#4): (61-0)%12 = 1, not in scale.
        // Below: 60 (0, in scale) dist 1. Above: 62 (2, in scale) dist 1.
        // Tie -> prefer lower -> 60.
        assert_eq!(scale.quantize(61, 0), 60);
    }

    // -- note_from_dac with non-chromatic scale -------------------------------

    #[test]
    fn test_note_from_dac_with_major_scale() {
        let q = Quantizer::new(Scale::major(), 0);
        // All DAC values should produce notes in C major within the
        // default range 36..=84.
        for dac in 0..=255u8 {
            let note = q.note_from_dac(dac);
            assert!(
                (36..=84).contains(&note),
                "note {note} out of range 36..=84 for dac={dac}"
            );
            let degree = note % 12;
            assert!(
                [0, 2, 4, 5, 7, 9, 11].contains(&degree),
                "note {note} (degree {degree}) should be in C major for dac={dac}"
            );
        }
    }

    #[test]
    fn test_note_from_dac_with_pentatonic_and_root() {
        let q = Quantizer {
            scale: Scale::pentatonic_major(),
            root: 7, // G pentatonic: G A B D E -> degrees [0,2,4,7,9] relative to G
            range: 48..=84,
        };
        for dac in 0..=255u8 {
            let note = q.note_from_dac(dac);
            assert!(
                (48..=84).contains(&note),
                "note {note} out of range for dac={dac}"
            );
        }
    }

    // -- note_from_dac range-start at 1 (minimum MIDI note) -------------------

    #[test]
    fn test_note_from_dac_low_range_start() {
        let q = Quantizer {
            scale: Scale::chromatic(),
            root: 0,
            range: 1..=12,
        };
        let note = q.note_from_dac(0);
        assert!(note >= 1, "note must be >= 1, got {note}");
        assert!(note <= 12, "note must be <= 12, got {note}");
    }

    // -- Quantizer with high range end ----------------------------------------

    #[test]
    fn test_note_from_dac_high_range() {
        let q = Quantizer {
            scale: Scale::chromatic(),
            root: 0,
            range: 100..=127,
        };
        for dac in 0..=255u8 {
            let note = q.note_from_dac(dac);
            assert!(
                (100..=127).contains(&note),
                "note {note} out of range 100..=127 for dac={dac}"
            );
        }
    }

    // -- Scale equality -------------------------------------------------------

    #[test]
    fn test_scale_eq() {
        let a = Scale::major();
        let b = Scale::new(vec![0, 2, 4, 5, 7, 9, 11], "Major");
        assert_eq!(a, b);
    }

    #[test]
    fn test_scale_ne() {
        assert_ne!(Scale::major(), Scale::natural_minor());
    }

    // -- Scale clone ----------------------------------------------------------

    #[test]
    fn test_scale_clone() {
        let original = Scale::major();
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    // -- Quantizer clone and eq -----------------------------------------------

    #[test]
    fn test_quantizer_clone_eq() {
        let q = Quantizer::new(Scale::major(), 5);
        let cloned = q.clone();
        assert_eq!(q, cloned);
    }

    // -- Quantizer debug output -----------------------------------------------

    #[test]
    fn test_quantizer_debug() {
        let q = Quantizer::new(Scale::chromatic(), 0);
        let debug = format!("{q:?}");
        assert!(
            debug.contains("Quantizer"),
            "debug output should contain 'Quantizer': {debug}"
        );
        assert!(
            debug.contains("Chromatic"),
            "debug output should contain scale name: {debug}"
        );
    }

    // -- Scale debug output ---------------------------------------------------

    #[test]
    fn test_scale_debug() {
        let scale = Scale::major();
        let debug = format!("{scale:?}");
        assert!(
            debug.contains("Major"),
            "debug output should contain 'Major': {debug}"
        );
    }

    // -- new with all out-of-range intervals ----------------------------------

    #[test]
    fn test_scale_new_all_out_of_range() {
        let scale = Scale::new(vec![12, 13, 255], "AllBad");
        // All filtered out, so root (0) should be inserted.
        assert_eq!(scale.intervals(), &[0]);
        assert_eq!(scale.name(), "AllBad");
    }
}
