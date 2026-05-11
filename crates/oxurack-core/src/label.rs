//! Instance-level labels: display names, descriptions, and tags.
//!
//! These components describe **instances** (individual module and cable
//! entities), complementing the **kind-level** metadata already stored
//! in [`ModuleRegistration`](crate::ModuleRegistration). The
//! [`DisplayNameFormatter`] provides a configurable fallback for
//! entities without an explicit [`DisplayName`] component.

use std::collections::{BTreeSet, HashMap};

use bevy_ecs::prelude::{Component, Resource};

/// Optional instance-level human-readable name.
///
/// When present, overrides the formatter-derived name for display
/// purposes. When absent, consumers fall back to
/// [`DisplayNameFormatter::format`] applied to the instance name.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct DisplayName(pub String);

/// Optional instance-level description.
///
/// Supplements (does not replace) the kind-level
/// `ModuleRegistration::description`.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Description(pub String);

/// Optional instance-level tag set.
///
/// Stored as `BTreeSet<Tag>` for deterministic serialization order
/// and O(log n) membership tests.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Tags(pub BTreeSet<Tag>);

/// A single tag.
///
/// Newtype with validated construction (Rust guidelines TD-03).
/// The inner field is private; construction via [`Tag::new`] is
/// the only way to produce a `Tag`.
///
/// v0 validation: non-empty, no leading or trailing whitespace.
/// Internal whitespace is permitted.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(String);

impl Tag {
    /// Creates a new tag, validating the input.
    ///
    /// # Errors
    ///
    /// Returns [`TagError::Empty`] if the string is empty, or
    /// [`TagError::LeadingOrTrailingWhitespace`] if the string has
    /// leading or trailing whitespace.
    pub fn new(s: impl Into<String>) -> Result<Self, TagError> {
        let s = s.into();
        if s.is_empty() {
            return Err(TagError::Empty);
        }
        if s.trim().len() != s.len() {
            return Err(TagError::LeadingOrTrailingWhitespace);
        }
        Ok(Self(s))
    }

    /// Returns the tag's string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Errors from [`Tag::new`] construction.
#[non_exhaustive]
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum TagError {
    /// The tag string was empty.
    #[error("tag must not be empty")]
    Empty,
    /// The tag string had leading or trailing whitespace.
    #[error("tag must not have leading or trailing whitespace")]
    LeadingOrTrailingWhitespace,
}

/// Rack-level metadata. One resource per world.
#[derive(Resource, Default, Clone, Debug)]
pub struct RackMeta {
    /// Rack display name.
    pub display_name: String,
    /// Rack description.
    pub description: String,
    /// Free-form notes (one string per note).
    pub notes: Vec<String>,
    /// Rack-level tags.
    pub tags: BTreeSet<Tag>,
}

/// Configurable formatter that derives a presentable display name
/// from an instance name.
///
/// Default behavior: split on `_` or `-`; for each token, look up
/// in `acronyms` (lowercased key → display form), else apply Title
/// Case; pure-digit tokens kept verbatim; join with spaces.
///
/// Example: `"vco_1"` → `"VCO 1"`, `"lead_voice_vco"` → `"Lead Voice VCO"`.
#[derive(Resource, Clone, Debug)]
pub struct DisplayNameFormatter {
    /// Acronym table: lowercase key → display form.
    pub acronyms: HashMap<String, String>,
    /// Characters to split on when tokenizing instance names.
    pub separators: Vec<char>,
}

impl Default for DisplayNameFormatter {
    fn default() -> Self {
        let mut acronyms = HashMap::new();
        for &(lower, display) in DEFAULT_ACRONYMS {
            acronyms.insert(lower.to_string(), display.to_string());
        }
        Self {
            acronyms,
            separators: vec!['_', '-'],
        }
    }
}

impl DisplayNameFormatter {
    /// Format an instance name as a presentable display name.
    pub fn format(&self, instance_name: &str) -> String {
        instance_name
            .split(|c: char| self.separators.contains(&c))
            .filter(|t| !t.is_empty())
            .map(|token| self.format_token(token))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn format_token(&self, token: &str) -> String {
        let lower = token.to_lowercase();
        if let Some(display) = self.acronyms.get(&lower) {
            return display.clone();
        }
        if token.chars().all(|c| c.is_ascii_digit()) {
            return token.to_string();
        }
        let mut chars = lower.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

/// Default synth-domain acronym table.
const DEFAULT_ACRONYMS: &[(&str, &str)] = &[
    ("vco", "VCO"),
    ("vca", "VCA"),
    ("vcf", "VCF"),
    ("dco", "DCO"),
    ("dca", "DCA"),
    ("dcf", "DCF"),
    ("lfo", "LFO"),
    ("adsr", "ADSR"),
    ("ahdsr", "AHDSR"),
    ("ar", "AR"),
    ("asr", "ASR"),
    ("pwm", "PWM"),
    ("fm", "FM"),
    ("am", "AM"),
    ("wt", "WT"),
    ("cv", "CV"),
    ("bpm", "BPM"),
    ("midi", "MIDI"),
    ("eq", "EQ"),
    ("fx", "FX"),
    ("io", "I/O"),
    ("lr", "L/R"),
];

/// Fallback-aware display-name lookup.
///
/// Returns the explicit [`DisplayName`] component value if present
/// and non-empty, else the formatter applied to the instance name.
pub fn display_name_of(
    formatter: &DisplayNameFormatter,
    explicit: Option<&DisplayName>,
    instance_name: &str,
) -> String {
    match explicit {
        Some(DisplayName(s)) if !s.is_empty() => s.clone(),
        _ => formatter.format(instance_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::world::World;
    use pretty_assertions::assert_eq;

    // ── Component basics ──────────────────────────────────────────

    #[test]
    fn test_display_name_component_insert_and_read() {
        let mut world = World::new();
        let entity = world.spawn(DisplayName("Lead VCO".into())).id();
        let dn = world.get::<DisplayName>(entity).unwrap();
        assert_eq!(dn.0, "Lead VCO");
    }

    #[test]
    fn test_description_component_insert_and_read() {
        let mut world = World::new();
        let entity = world.spawn(Description("Main oscillator".into())).id();
        let desc = world.get::<Description>(entity).unwrap();
        assert_eq!(desc.0, "Main oscillator");
    }

    #[test]
    fn test_tags_component_insert_and_read() {
        let mut world = World::new();
        let tags = BTreeSet::from([
            Tag::new("lead").unwrap(),
            Tag::new("bass").unwrap(),
            Tag::new("ambient").unwrap(),
        ]);
        let entity = world.spawn(Tags(tags.clone())).id();
        let read = world.get::<Tags>(entity).unwrap();
        assert_eq!(read.0, tags);
        let names: Vec<_> = read.0.iter().map(|t| t.as_str()).collect();
        assert_eq!(names, vec!["ambient", "bass", "lead"]);
    }

    // ── Tag validation ────────────────────────────────────────────

    #[test]
    fn test_tag_new_valid_input() {
        let tag = Tag::new("lead").unwrap();
        assert_eq!(tag.as_str(), "lead");
    }

    #[test]
    fn test_tag_new_rejects_empty() {
        assert_eq!(Tag::new(""), Err(TagError::Empty));
    }

    #[test]
    fn test_tag_new_rejects_leading_whitespace() {
        assert_eq!(
            Tag::new(" lead"),
            Err(TagError::LeadingOrTrailingWhitespace)
        );
    }

    #[test]
    fn test_tag_new_rejects_trailing_whitespace() {
        assert_eq!(
            Tag::new("lead "),
            Err(TagError::LeadingOrTrailingWhitespace)
        );
    }

    #[test]
    fn test_tag_new_allows_internal_whitespace() {
        let tag = Tag::new("lead voice").unwrap();
        assert_eq!(tag.as_str(), "lead voice");
    }

    #[test]
    fn test_tag_error_display() {
        assert_eq!(TagError::Empty.to_string(), "tag must not be empty");
        assert_eq!(
            TagError::LeadingOrTrailingWhitespace.to_string(),
            "tag must not have leading or trailing whitespace"
        );
    }

    // ── DisplayNameFormatter ──────────────────────────────────────

    #[test]
    fn test_formatter_default_acronyms() {
        let fmt = DisplayNameFormatter::default();
        assert_eq!(fmt.format("vco_1"), "VCO 1");
        assert_eq!(fmt.format("lfo_master"), "LFO Master");
        assert_eq!(fmt.format("adsr_envelope"), "ADSR Envelope");
    }

    #[test]
    fn test_formatter_title_case() {
        let fmt = DisplayNameFormatter::default();
        assert_eq!(fmt.format("lead_voice"), "Lead Voice");
    }

    #[test]
    fn test_formatter_mixed() {
        let fmt = DisplayNameFormatter::default();
        assert_eq!(fmt.format("lead_voice_vco"), "Lead Voice VCO");
    }

    #[test]
    fn test_formatter_runtime_extension() {
        let mut fmt = DisplayNameFormatter::default();
        fmt.acronyms
            .insert("osc".to_string(), "OSC".to_string());
        assert_eq!(fmt.format("osc_1"), "OSC 1");
    }

    #[test]
    fn test_formatter_digit_tokens() {
        let fmt = DisplayNameFormatter::default();
        assert_eq!(fmt.format("vco_1"), "VCO 1");
        assert_eq!(fmt.format("track_42"), "Track 42");
    }

    #[test]
    fn test_formatter_dash_separator() {
        let fmt = DisplayNameFormatter::default();
        assert_eq!(fmt.format("lead-vco"), "Lead VCO");
    }

    // ── Fallback chain ────────────────────────────────────────────

    #[test]
    fn test_display_name_of_uses_component_when_present() {
        let fmt = DisplayNameFormatter::default();
        let dn = DisplayName("Lead".into());
        assert_eq!(display_name_of(&fmt, Some(&dn), "vco_1"), "Lead");
    }

    #[test]
    fn test_display_name_of_uses_formatter_when_absent() {
        let fmt = DisplayNameFormatter::default();
        assert_eq!(display_name_of(&fmt, None, "vco_1"), "VCO 1");
    }

    #[test]
    fn test_display_name_of_treats_empty_string_as_absent() {
        let fmt = DisplayNameFormatter::default();
        let dn = DisplayName(String::new());
        assert_eq!(display_name_of(&fmt, Some(&dn), "vco_1"), "VCO 1");
    }
}
