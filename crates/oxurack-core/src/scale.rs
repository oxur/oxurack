//! Musical scales and quantisation helpers.
//!
//! [`Scale`] represents a musical scale as a root note and a set of
//! semitone intervals within a single octave. Built-in constructors
//! cover the most common Western scales and modes.
//!
//! The [`quantize`](Scale::quantize) method snaps an arbitrary MIDI
//! note number to the nearest note that belongs to the scale, making
//! it easy to constrain random or continuous note sources to musically
//! useful pitches.

use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

/// A musical scale defined by a root note and semitone intervals within
/// a single octave.
///
/// Intervals are stored as sorted, deduplicated offsets in the range
/// 0--11. The `root` field (0--11, where 0 = C) anchors the scale to a
/// specific key.
///
/// # Examples
///
/// ```
/// use oxurack_core::Scale;
///
/// let scale = Scale::major(0); // C major
/// assert_eq!(scale.quantize(61), 60); // C#4 snaps down to C4
/// assert!(scale.is_in_scale(60));      // C4 is in C major
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub struct Scale {
    /// Root note of the scale (0 = C, 1 = C#, ..., 11 = B).
    /// Values > 11 are clamped to 11 during construction.
    pub root: u8,

    /// Semitone intervals within one octave, sorted and deduplicated.
    /// Uses `Vec<u8>` for now; a future optimisation may switch to
    /// `SmallVec<[u8; 12]>` once `Reflect` support is available.
    pub intervals: Vec<u8>,

    /// Optional display name (e.g. "Major", "Dorian").
    /// Anonymous (user-defined) scales may leave this as `None`.
    pub name: Option<String>,
}

impl Scale {
    /// Creates a new scale from a list of semitone offsets (0--11), a
    /// root note, and an optional name.
    ///
    /// The intervals are filtered to 0--11, sorted, and deduplicated.
    /// If the resulting set is empty, interval `0` (the root) is
    /// inserted automatically. Root values > 11 are clamped to 11.
    #[must_use]
    pub fn new(intervals: impl IntoIterator<Item = u8>, root: u8, name: Option<String>) -> Self {
        let root = root.min(11);
        let mut ivs: Vec<u8> = intervals.into_iter().filter(|&i| i < 12).collect();
        ivs.sort_unstable();
        ivs.dedup();
        if ivs.is_empty() {
            ivs.push(0);
        }
        Self {
            root,
            intervals: ivs,
            name,
        }
    }

    /// Returns the display name of this scale, if any.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the semitone intervals that define this scale.
    #[must_use]
    pub fn intervals(&self) -> &[u8] {
        &self.intervals
    }

    /// Returns the root note (0--11).
    #[must_use]
    pub fn root(&self) -> u8 {
        self.root
    }

    /// Snaps `raw_note` to the nearest note that belongs to this scale.
    ///
    /// A MIDI note `N` is considered "in scale" when
    /// `(N as i16 - root as i16).rem_euclid(12)` appears in
    /// `self.intervals`.
    ///
    /// When two candidate notes are equidistant, the lower note is
    /// preferred. The result is clamped to the MIDI range 0--127.
    #[must_use]
    pub fn quantize(&self, raw_note: u8) -> u8 {
        if self.is_in_scale(raw_note) {
            return raw_note;
        }

        for distance in 1..=127i16 {
            let below = raw_note as i16 - distance;
            let above = raw_note as i16 + distance;

            if below >= 0 && self.is_in_scale(below as u8) {
                return below as u8;
            }
            if above <= 127 && self.is_in_scale(above as u8) {
                return above as u8;
            }
        }

        // Fallback (unreachable for any non-degenerate scale).
        raw_note
    }

    /// Returns `true` if `note` belongs to this scale.
    #[must_use]
    pub fn is_in_scale(&self, note: u8) -> bool {
        let degree = (note as i16 - self.root as i16).rem_euclid(12) as u8;
        self.intervals.contains(&degree)
    }

    // ── Built-in scale constructors ──────────────────────────────

    /// Chromatic scale -- all twelve semitones.
    #[must_use]
    pub fn chromatic(root: u8) -> Self {
        Self::new(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            root,
            Some("Chromatic".to_string()),
        )
    }

    /// Major (Ionian) scale.
    #[must_use]
    pub fn major(root: u8) -> Self {
        Self::new(
            vec![0, 2, 4, 5, 7, 9, 11],
            root,
            Some("Major".to_string()),
        )
    }

    /// Natural minor (Aeolian) scale.
    #[must_use]
    pub fn natural_minor(root: u8) -> Self {
        Self::new(
            vec![0, 2, 3, 5, 7, 8, 10],
            root,
            Some("Natural Minor".to_string()),
        )
    }

    /// Harmonic minor scale.
    #[must_use]
    pub fn harmonic_minor(root: u8) -> Self {
        Self::new(
            vec![0, 2, 3, 5, 7, 8, 11],
            root,
            Some("Harmonic Minor".to_string()),
        )
    }

    /// Major pentatonic scale.
    #[must_use]
    pub fn pentatonic_major(root: u8) -> Self {
        Self::new(
            vec![0, 2, 4, 7, 9],
            root,
            Some("Pentatonic Major".to_string()),
        )
    }

    /// Minor pentatonic scale.
    #[must_use]
    pub fn pentatonic_minor(root: u8) -> Self {
        Self::new(
            vec![0, 3, 5, 7, 10],
            root,
            Some("Pentatonic Minor".to_string()),
        )
    }

    /// Blues scale.
    #[must_use]
    pub fn blues(root: u8) -> Self {
        Self::new(
            vec![0, 3, 5, 6, 7, 10],
            root,
            Some("Blues".to_string()),
        )
    }

    /// Dorian mode.
    #[must_use]
    pub fn dorian(root: u8) -> Self {
        Self::new(
            vec![0, 2, 3, 5, 7, 9, 10],
            root,
            Some("Dorian".to_string()),
        )
    }

    /// Phrygian mode.
    #[must_use]
    pub fn phrygian(root: u8) -> Self {
        Self::new(
            vec![0, 1, 3, 5, 7, 8, 10],
            root,
            Some("Phrygian".to_string()),
        )
    }

    /// Lydian mode.
    #[must_use]
    pub fn lydian(root: u8) -> Self {
        Self::new(
            vec![0, 2, 4, 6, 7, 9, 11],
            root,
            Some("Lydian".to_string()),
        )
    }

    /// Mixolydian mode.
    #[must_use]
    pub fn mixolydian(root: u8) -> Self {
        Self::new(
            vec![0, 2, 4, 5, 7, 9, 10],
            root,
            Some("Mixolydian".to_string()),
        )
    }

    /// Whole-tone scale.
    #[must_use]
    pub fn whole_tone(root: u8) -> Self {
        Self::new(
            vec![0, 2, 4, 6, 8, 10],
            root,
            Some("Whole Tone".to_string()),
        )
    }

    /// Diminished (octatonic) scale.
    #[must_use]
    pub fn diminished(root: u8) -> Self {
        Self::new(
            vec![0, 2, 3, 5, 6, 8, 9, 11],
            root,
            Some("Diminished".to_string()),
        )
    }

    /// Augmented scale.
    #[must_use]
    pub fn augmented(root: u8) -> Self {
        Self::new(
            vec![0, 3, 4, 7, 8, 11],
            root,
            Some("Augmented".to_string()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // ── Constructor / normalisation ──────────────────────────────

    #[test]
    fn test_new_sorts_and_deduplicates() {
        let scale = Scale::new(vec![7, 0, 4, 0, 4], 0, None);
        assert_eq!(scale.intervals(), &[0, 4, 7]);
    }

    #[test]
    fn test_new_filters_out_of_range() {
        let scale = Scale::new(vec![0, 12, 15, 5, 255], 0, None);
        assert_eq!(scale.intervals(), &[0, 5]);
    }

    #[test]
    fn test_new_empty_intervals_defaults_to_root() {
        let scale = Scale::new(vec![], 0, None);
        assert_eq!(scale.intervals(), &[0]);
    }

    #[test]
    fn test_new_all_out_of_range_defaults_to_root() {
        let scale = Scale::new(vec![12, 13, 200], 0, None);
        assert_eq!(scale.intervals(), &[0]);
    }

    #[test]
    fn test_root_clamped_to_11() {
        let scale = Scale::new(vec![0], 15, None);
        assert_eq!(scale.root(), 11);
    }

    #[test]
    fn test_root_within_range() {
        let scale = Scale::new(vec![0], 5, None);
        assert_eq!(scale.root(), 5);
    }

    #[test]
    fn test_name_some() {
        let scale = Scale::new(vec![0], 0, Some("Test".to_string()));
        assert_eq!(scale.name(), Some("Test"));
    }

    #[test]
    fn test_name_none() {
        let scale = Scale::new(vec![0], 0, None);
        assert_eq!(scale.name(), None);
    }

    // ── Built-in constructors ───────────────────────────────────

    #[test]
    fn test_chromatic_intervals() {
        let scale = Scale::chromatic(0);
        assert_eq!(
            scale.intervals(),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        );
        assert_eq!(scale.name(), Some("Chromatic"));
    }

    #[test]
    fn test_major_intervals() {
        let scale = Scale::major(0);
        assert_eq!(scale.intervals(), &[0, 2, 4, 5, 7, 9, 11]);
        assert_eq!(scale.name(), Some("Major"));
    }

    #[test]
    fn test_natural_minor_intervals() {
        let scale = Scale::natural_minor(0);
        assert_eq!(scale.intervals(), &[0, 2, 3, 5, 7, 8, 10]);
        assert_eq!(scale.name(), Some("Natural Minor"));
    }

    #[test]
    fn test_harmonic_minor_intervals() {
        let scale = Scale::harmonic_minor(0);
        assert_eq!(scale.intervals(), &[0, 2, 3, 5, 7, 8, 11]);
    }

    #[test]
    fn test_pentatonic_major_intervals() {
        let scale = Scale::pentatonic_major(0);
        assert_eq!(scale.intervals(), &[0, 2, 4, 7, 9]);
    }

    #[test]
    fn test_pentatonic_minor_intervals() {
        let scale = Scale::pentatonic_minor(0);
        assert_eq!(scale.intervals(), &[0, 3, 5, 7, 10]);
    }

    #[test]
    fn test_blues_intervals() {
        let scale = Scale::blues(0);
        assert_eq!(scale.intervals(), &[0, 3, 5, 6, 7, 10]);
    }

    #[test]
    fn test_dorian_intervals() {
        let scale = Scale::dorian(0);
        assert_eq!(scale.intervals(), &[0, 2, 3, 5, 7, 9, 10]);
    }

    #[test]
    fn test_phrygian_intervals() {
        let scale = Scale::phrygian(0);
        assert_eq!(scale.intervals(), &[0, 1, 3, 5, 7, 8, 10]);
    }

    #[test]
    fn test_lydian_intervals() {
        let scale = Scale::lydian(0);
        assert_eq!(scale.intervals(), &[0, 2, 4, 6, 7, 9, 11]);
    }

    #[test]
    fn test_mixolydian_intervals() {
        let scale = Scale::mixolydian(0);
        assert_eq!(scale.intervals(), &[0, 2, 4, 5, 7, 9, 10]);
    }

    #[test]
    fn test_whole_tone_intervals() {
        let scale = Scale::whole_tone(0);
        assert_eq!(scale.intervals(), &[0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_diminished_intervals() {
        let scale = Scale::diminished(0);
        assert_eq!(scale.intervals(), &[0, 2, 3, 5, 6, 8, 9, 11]);
    }

    #[test]
    fn test_augmented_intervals() {
        let scale = Scale::augmented(0);
        assert_eq!(scale.intervals(), &[0, 3, 4, 7, 8, 11]);
    }

    // ── quantize ────────────────────────────────────────────────

    #[test]
    fn test_quantize_in_scale_unchanged() {
        let scale = Scale::major(0); // C major: C D E F G A B
        assert_eq!(scale.quantize(60), 60); // C4 stays
        assert_eq!(scale.quantize(62), 62); // D4 stays
        assert_eq!(scale.quantize(64), 64); // E4 stays
    }

    #[test]
    fn test_quantize_snaps_down_on_tie() {
        let scale = Scale::major(0); // C=0, D=2
        // Note 61 (C#) is 1 semitone from C (60) and 1 from D (62).
        // Tie: prefer lower note.
        assert_eq!(scale.quantize(61), 60);
    }

    #[test]
    fn test_quantize_snaps_to_nearest() {
        let scale = Scale::major(0);
        // Note 63 (Eb) is 1 from D(62) and 2 from E(64). Snap down to D.
        assert_eq!(scale.quantize(63), 62);
    }

    #[test]
    fn test_quantize_chromatic_identity() {
        let scale = Scale::chromatic(0);
        for note in 0..=127u8 {
            assert_eq!(scale.quantize(note), note);
        }
    }

    #[test]
    fn test_quantize_with_root() {
        // D major (root=2): D E F# G A B C#
        let scale = Scale::major(2);
        // Note 60 = C. In D major, C is not in scale.
        // C# (61) is in scale (interval 11 from D). B (59) is in scale (interval 9 from D).
        // Distance from 60: C#=1, B=1. Tie: prefer lower -> 59 (B).
        assert_eq!(scale.quantize(60), 59);
    }

    #[test]
    fn test_quantize_edge_note_0() {
        let scale = Scale::major(0);
        // Note 0 = C, which is in C major.
        assert_eq!(scale.quantize(0), 0);
    }

    #[test]
    fn test_quantize_edge_note_127() {
        let scale = Scale::major(0);
        // Note 127 = G9. G is in C major (interval 7).
        assert_eq!(scale.quantize(127), 127);
    }

    #[test]
    fn test_quantize_edge_note_1_not_in_c_major() {
        let scale = Scale::major(0);
        // Note 1 = C#. Nearest in-scale: C(0) below, D(2) above.
        // Tie: prefer lower -> 0.
        assert_eq!(scale.quantize(1), 0);
    }

    // ── is_in_scale ─────────────────────────────────────────────

    #[test]
    fn test_is_in_scale_root_c() {
        let scale = Scale::major(0);
        // C major: C D E F G A B => notes 0,2,4,5,7,9,11 (+ octaves)
        assert!(scale.is_in_scale(0));  // C
        assert!(!scale.is_in_scale(1)); // C#
        assert!(scale.is_in_scale(2));  // D
        assert!(!scale.is_in_scale(3)); // D#
        assert!(scale.is_in_scale(4));  // E
        assert!(scale.is_in_scale(5));  // F
        assert!(!scale.is_in_scale(6)); // F#
        assert!(scale.is_in_scale(7));  // G
        assert!(!scale.is_in_scale(8)); // G#
        assert!(scale.is_in_scale(9));  // A
        assert!(!scale.is_in_scale(10)); // A#
        assert!(scale.is_in_scale(11)); // B
    }

    #[test]
    fn test_is_in_scale_with_root() {
        // D major (root=2): intervals 0,2,4,5,7,9,11
        // In absolute terms: D(2) E(4) F#(6) G(7) A(9) B(11) C#(1)
        let scale = Scale::major(2);
        assert!(scale.is_in_scale(2));  // D
        assert!(!scale.is_in_scale(3)); // D#
        assert!(scale.is_in_scale(4));  // E
        assert!(!scale.is_in_scale(5)); // F
        assert!(scale.is_in_scale(6));  // F#
        assert!(scale.is_in_scale(7));  // G
        assert!(!scale.is_in_scale(8)); // G#
        assert!(scale.is_in_scale(9));  // A
        assert!(!scale.is_in_scale(10)); // A#
        assert!(scale.is_in_scale(11)); // B
        assert!(!scale.is_in_scale(0)); // C
        assert!(scale.is_in_scale(1));  // C#
    }

    #[test]
    fn test_is_in_scale_octave_equivalence() {
        let scale = Scale::major(0);
        // C in various octaves
        assert!(scale.is_in_scale(0));
        assert!(scale.is_in_scale(12));
        assert!(scale.is_in_scale(24));
        assert!(scale.is_in_scale(60));
    }

    // ── Serde round-trip ────────────────────────────────────────

    #[test]
    fn test_serde_ron_roundtrip() {
        let scale = Scale::major(2);
        let ron_str = ron::to_string(&scale).expect("serialize to RON");
        let deserialized: Scale = ron::from_str(&ron_str).expect("deserialize from RON");
        assert_eq!(scale, deserialized);
    }

    #[test]
    fn test_serde_ron_roundtrip_anonymous() {
        let scale = Scale::new(vec![0, 3, 7], 5, None);
        let ron_str = ron::to_string(&scale).expect("serialize to RON");
        let deserialized: Scale = ron::from_str(&ron_str).expect("deserialize from RON");
        assert_eq!(scale, deserialized);
    }

    // ── Debug / Clone / PartialEq ───────────────────────────────

    #[test]
    fn test_debug() {
        let scale = Scale::major(0);
        let debug = format!("{scale:?}");
        assert!(debug.contains("Major"), "expected 'Major' in: {debug}");
    }

    #[test]
    fn test_clone_eq() {
        let a = Scale::blues(3);
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_ne() {
        let a = Scale::major(0);
        let b = Scale::minor_pentatonic_alias(0);
        assert_ne!(a, b);
    }
}

// Test-only helper (keep out of the main impl to avoid polluting the
// public API).
#[cfg(test)]
impl Scale {
    fn minor_pentatonic_alias(root: u8) -> Self {
        Self::pentatonic_minor(root)
    }
}
