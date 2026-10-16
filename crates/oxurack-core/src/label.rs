//! Instance-level labels: display names, descriptions, and tags.
//!
//! These components describe **instances** (individual module and cable
//! entities), complementing the **kind-level** metadata already stored
//! in [`ModuleRegistration`](crate::ModuleRegistration). The
//! [`DisplayNameFormatter`] provides a configurable fallback for
//! entities without an explicit [`DisplayName`] component.

use std::collections::{BTreeSet, HashMap};

use bevy_ecs::hierarchy::ChildOf;
use bevy_ecs::prelude::{Component, Entity, Resource};
use bevy_ecs::world::World;

use crate::{Cable, CableOrder, ChangeSource, CoreError, LabelChanged, LabelKind, LabelValue};

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

/// Selects rack metadata, a module instance, or one cable between named ports.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LabelTarget {
    /// Metadata stored on the rack's [`RackMeta`] resource.
    Rack,
    /// Metadata on an active module instance.
    Module(String),
    /// Metadata on the zero-based cable occurrence between exact endpoints.
    Cable {
        /// `(module instance, port name)` of the source.
        source: (String, String),
        /// `(module instance, port name)` of the target.
        target: (String, String),
        /// Zero-based occurrence after deterministic cable-order sorting.
        occurrence: usize,
    },
}

#[derive(Clone, Debug)]
enum ResolvedLabelTarget {
    Rack,
    Entity {
        entity: Entity,
        instance_name: String,
    },
}

/// Reads a raw live label value. Entity-level empty text/tag components are
/// returned as stored; formatter fallback is the graph view's responsibility.
pub fn label_value(
    world: &mut World,
    target: &LabelTarget,
    kind: &LabelKind,
) -> Result<Option<LabelValue>, CoreError> {
    require_patch(world)?;
    validate_label_kind_target(target, kind)?;
    let resolved = resolve_target(world, target)?;
    let value = match resolved {
        ResolvedLabelTarget::Rack => {
            let meta = world.resource::<RackMeta>();
            Some(match kind {
                LabelKind::DisplayName => LabelValue::Text(meta.display_name.clone()),
                LabelKind::Description => LabelValue::Text(meta.description.clone()),
                LabelKind::Tags => LabelValue::Tags(meta.tags.clone()),
                LabelKind::Notes => LabelValue::Notes(meta.notes.clone()),
            })
        }
        ResolvedLabelTarget::Entity { entity, .. } => match kind {
            LabelKind::DisplayName => world
                .get::<DisplayName>(entity)
                .map(|value| LabelValue::Text(value.0.clone())),
            LabelKind::Description => world
                .get::<Description>(entity)
                .map(|value| LabelValue::Text(value.0.clone())),
            LabelKind::Tags => world
                .get::<Tags>(entity)
                .map(|value| LabelValue::Tags(value.0.clone())),
            LabelKind::Notes => unreachable!("entity notes are rejected before resolution"),
        },
    };
    Ok(value)
}

/// Writes one live label field and emits one `LabelChanged` message on success.
pub fn set_label_value(
    world: &mut World,
    target: &LabelTarget,
    kind: &LabelKind,
    value: &LabelValue,
    source: ChangeSource,
) -> Result<(), CoreError> {
    require_patch(world)?;
    validate_label_value(target, kind, value)?;
    let resolved = resolve_target(world, target)?;

    let (entity, instance_name, previous) = match &resolved {
        ResolvedLabelTarget::Rack => {
            let meta = world.resource::<RackMeta>();
            let previous = match kind {
                LabelKind::DisplayName => LabelValue::Text(meta.display_name.clone()),
                LabelKind::Description => LabelValue::Text(meta.description.clone()),
                LabelKind::Tags => LabelValue::Tags(meta.tags.clone()),
                LabelKind::Notes => LabelValue::Notes(meta.notes.clone()),
            };
            (None, String::new(), Some(previous))
        }
        ResolvedLabelTarget::Entity {
            entity,
            instance_name,
        } => {
            let previous = match kind {
                LabelKind::DisplayName => world
                    .get::<DisplayName>(*entity)
                    .map(|value| LabelValue::Text(value.0.clone())),
                LabelKind::Description => world
                    .get::<Description>(*entity)
                    .map(|value| LabelValue::Text(value.0.clone())),
                LabelKind::Tags => world
                    .get::<Tags>(*entity)
                    .map(|value| LabelValue::Tags(value.0.clone())),
                LabelKind::Notes => unreachable!("entity notes are rejected before resolution"),
            };
            (Some(*entity), instance_name.clone(), previous)
        }
    };

    match resolved {
        ResolvedLabelTarget::Rack => {
            let mut meta = world.resource_mut::<RackMeta>();
            match (kind, value) {
                (LabelKind::DisplayName, LabelValue::Text(text)) => {
                    meta.display_name.clone_from(text)
                }
                (LabelKind::Description, LabelValue::Text(text)) => {
                    meta.description.clone_from(text)
                }
                (LabelKind::Tags, LabelValue::Tags(tags)) => meta.tags.clone_from(tags),
                (LabelKind::Notes, LabelValue::Notes(notes)) => meta.notes.clone_from(notes),
                _ => unreachable!("payload validated before mutation"),
            }
        }
        ResolvedLabelTarget::Entity { entity, .. } => match (kind, value) {
            (LabelKind::DisplayName, LabelValue::Text(text)) => {
                if text.is_empty() {
                    world.entity_mut(entity).remove::<DisplayName>();
                } else {
                    world.entity_mut(entity).insert(DisplayName(text.clone()));
                }
            }
            (LabelKind::Description, LabelValue::Text(text)) => {
                if text.is_empty() {
                    world.entity_mut(entity).remove::<Description>();
                } else {
                    world.entity_mut(entity).insert(Description(text.clone()));
                }
            }
            (LabelKind::Tags, LabelValue::Tags(tags)) => {
                if tags.is_empty() {
                    world.entity_mut(entity).remove::<Tags>();
                } else {
                    world.entity_mut(entity).insert(Tags(tags.clone()));
                }
            }
            _ => unreachable!("payload validated before mutation"),
        },
    }

    world.write_message(LabelChanged {
        entity,
        instance_name,
        kind: kind.clone(),
        previous: previous.clone(),
        new: value.clone(),
        source: source.clone(),
    });
    let logical_target = match target {
        LabelTarget::Rack => Some(crate::LogicalTarget::Rack),
        LabelTarget::Module(name) => Some(crate::LogicalTarget::Module(name.clone())),
        LabelTarget::Cable { .. } => entity
            .and_then(|entity| crate::observation::describe_cable(world, entity))
            .map(crate::LogicalTarget::Cable),
    };
    if let Some(target) = logical_target {
        crate::observation::stage(
            world,
            crate::OwnedChange::LabelChanged {
                target,
                kind: kind.clone(),
                previous,
                new: value.clone(),
                source,
            },
        );
    }
    Ok(())
}

fn require_patch(world: &World) -> Result<(), CoreError> {
    world
        .get_resource::<crate::PatchHandle>()
        .map(|_| ())
        .ok_or(CoreError::NoPatchLoaded)
}

fn validate_label_kind_target(target: &LabelTarget, kind: &LabelKind) -> Result<(), CoreError> {
    if matches!(kind, LabelKind::Notes) && !matches!(target, LabelTarget::Rack) {
        return Err(invalid_label(kind, "notes are supported only on the rack"));
    }
    Ok(())
}

fn validate_label_value(
    target: &LabelTarget,
    kind: &LabelKind,
    value: &LabelValue,
) -> Result<(), CoreError> {
    validate_label_kind_target(target, kind)?;
    let valid = matches!(
        (kind, value),
        (
            LabelKind::DisplayName | LabelKind::Description,
            LabelValue::Text(_)
        ) | (LabelKind::Tags, LabelValue::Tags(_))
            | (LabelKind::Notes, LabelValue::Notes(_))
    );
    if valid {
        Ok(())
    } else {
        Err(invalid_label(
            kind,
            "payload type does not match label kind",
        ))
    }
}

fn invalid_label(kind: &LabelKind, reason: &str) -> CoreError {
    CoreError::InvalidLabelValue {
        kind: kind.clone(),
        reason: reason.to_owned(),
    }
}

fn resolve_target(
    world: &mut World,
    target: &LabelTarget,
) -> Result<ResolvedLabelTarget, CoreError> {
    match target {
        LabelTarget::Rack => Ok(ResolvedLabelTarget::Rack),
        LabelTarget::Module(name) => {
            resolve_module(world, name).map(|(entity, instance_name)| ResolvedLabelTarget::Entity {
                entity,
                instance_name,
            })
        }
        LabelTarget::Cable {
            source,
            target,
            occurrence,
        } => {
            let (source_module, source_name) = resolve_module(world, &source.0)?;
            let source_port = resolve_port(world, source_module, &source.0, &source.1)?;
            let (target_module, target_name) = resolve_module(world, &target.0)?;
            let target_port = resolve_port(world, target_module, &target.0, &target.1)?;
            let mut query = world.query::<(Entity, &Cable, Option<&CableOrder>)>();
            let mut matches: Vec<_> = query
                .iter(world)
                .filter_map(|(entity, cable, order)| {
                    (cable.source_port == source_port && cable.target_port == target_port)
                        .then_some((order.is_none(), order.map_or(0, |order| order.0), entity))
                })
                .collect();
            matches.sort_unstable();
            let Some((_, _, entity)) = matches.get(*occurrence).copied() else {
                return Err(CoreError::UnknownCable {
                    source_endpoint: source.clone(),
                    target_endpoint: target.clone(),
                    occurrence: *occurrence,
                });
            };
            Ok(ResolvedLabelTarget::Entity {
                entity,
                instance_name: format!(
                    "{}.{} -> {}.{}",
                    source_name, source.1, target_name, target.1
                ),
            })
        }
    }
}

fn resolve_module(world: &World, name: &str) -> Result<(Entity, String), CoreError> {
    let handle = world
        .get_resource::<crate::PatchHandle>()
        .ok_or(CoreError::NoPatchLoaded)?;
    let entity = handle
        .modules
        .get(name)
        .copied()
        .ok_or_else(|| CoreError::UnknownModule(name.to_owned()))?;
    let module = world
        .get::<crate::Module>(entity)
        .filter(|module| module.instance_name == name)
        .ok_or_else(|| CoreError::UnknownModule(name.to_owned()))?;
    Ok((entity, module.instance_name.clone()))
}

fn resolve_port(
    world: &mut World,
    module_entity: Entity,
    module_name: &str,
    port_name: &str,
) -> Result<Entity, CoreError> {
    let mut query = world.query::<(Entity, &crate::port::Port, &ChildOf)>();
    query
        .iter(world)
        .find_map(|(entity, port, parent)| {
            (parent.parent() == module_entity && port.name.as_ref() == port_name).then_some(entity)
        })
        .ok_or_else(|| {
            CoreError::Patch(crate::PatchError::UnknownPort {
                module: module_name.to_owned(),
                port: port_name.to_owned(),
            })
        })
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
        fmt.acronyms.insert("osc".to_string(), "OSC".to_string());
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

    #[test]
    fn cable_occurrence_uses_active_ports_then_order_and_entity_tiebreak() {
        use bevy_app::App;
        use bevy_ecs::prelude::Messages;

        let mut app = App::new();
        app.add_plugins(crate::CorePlugin);
        app.world_mut().init_resource::<Messages<LabelChanged>>();
        let world = app.world_mut();
        let source_module = crate::spawn_module_entity(world, "clock", "clock_main");
        let target_module = crate::spawn_module_entity(world, "tm", "tm_main");
        let unrelated_module = crate::spawn_module_entity(world, "clock", "other_clock");
        let source_port = crate::spawn_port_on_module(
            world,
            source_module,
            "pulse",
            crate::PortDirection::Output,
            crate::ValueKind::Gate,
            crate::MergePolicy::Reject,
        );
        let target_port = crate::spawn_port_on_module(
            world,
            target_module,
            "clock",
            crate::PortDirection::Input,
            crate::ValueKind::Gate,
            crate::MergePolicy::LastWins,
        );
        let unrelated_port = crate::spawn_port_on_module(
            world,
            unrelated_module,
            "pulse",
            crate::PortDirection::Output,
            crate::ValueKind::Gate,
            crate::MergePolicy::Reject,
        );

        let cable = |source_port, target_port| Cable {
            source_port,
            target_port,
            transform: Some(crate::CableTransform::Invert),
            enabled: false,
        };
        let entity_ord1 = world
            .spawn((cable(source_port, target_port), CableOrder(1)))
            .id();
        let entity_ord0_first = world
            .spawn((cable(source_port, target_port), CableOrder(0)))
            .id();
        let entity_ord0_second = world
            .spawn((cable(source_port, target_port), CableOrder(0)))
            .id();
        let entity_unordered = world.spawn(cable(source_port, target_port)).id();
        let _unrelated_lower_ordinal = world
            .spawn((cable(unrelated_port, target_port), CableOrder(0)))
            .id();
        world.insert_resource(crate::PatchHandle {
            modules: [
                ("clock_main".into(), source_module),
                ("tm_main".into(), target_module),
                ("other_clock".into(), unrelated_module),
            ]
            .into(),
        });
        let mut tied_ord0 = [entity_ord0_first, entity_ord0_second];
        tied_ord0.sort_unstable();
        let expected = [tied_ord0[0], tied_ord0[1], entity_ord1, entity_unordered];
        for (occurrence, expected_entity) in expected.into_iter().enumerate() {
            let cable_target = LabelTarget::Cable {
                source: ("clock_main".into(), "pulse".into()),
                target: ("tm_main".into(), "clock".into()),
                occurrence,
            };
            set_label_value(
                world,
                &cable_target,
                &LabelKind::DisplayName,
                &LabelValue::Text(format!("occurrence {occurrence}")),
                ChangeSource::DirectApi,
            )
            .unwrap();
            assert_eq!(
                world.get::<DisplayName>(expected_entity).unwrap().0,
                format!("occurrence {occurrence}")
            );
            assert_eq!(
                world.get::<Cable>(expected_entity).unwrap().transform,
                Some(crate::CableTransform::Invert)
            );
            let expected_order = match occurrence {
                0 | 1 => Some(0),
                2 => Some(1),
                _ => None,
            };
            assert_eq!(
                world
                    .get::<CableOrder>(expected_entity)
                    .map(|order| order.0),
                expected_order
            );
            let latest = world
                .resource_mut::<Messages<LabelChanged>>()
                .drain()
                .last()
                .unwrap();
            assert_eq!(latest.entity, Some(expected_entity));
            assert_eq!(latest.instance_name, "clock_main.pulse -> tm_main.clock");
        }

        world
            .entity_mut(tied_ord0[0])
            .insert(DisplayName(String::new()));
        let first = LabelTarget::Cable {
            source: ("clock_main".into(), "pulse".into()),
            target: ("tm_main".into(), "clock".into()),
            occurrence: 0,
        };
        assert_eq!(
            label_value(world, &first, &LabelKind::DisplayName).unwrap(),
            Some(LabelValue::Text(String::new()))
        );
        set_label_value(
            world,
            &first,
            &LabelKind::DisplayName,
            &LabelValue::Text(String::new()),
            ChangeSource::DirectApi,
        )
        .unwrap();
        assert_eq!(
            label_value(world, &first, &LabelKind::DisplayName).unwrap(),
            None
        );

        let missing = LabelTarget::Cable {
            source: ("clock_main".into(), "pulse".into()),
            target: ("tm_main".into(), "clock".into()),
            occurrence: 4,
        };
        assert!(matches!(
            set_label_value(
                world,
                &missing,
                &LabelKind::Tags,
                &LabelValue::Tags(BTreeSet::new()),
                ChangeSource::DirectApi,
            ),
            Err(CoreError::UnknownCable {
                source_endpoint,
                target_endpoint,
                occurrence: 4,
            }) if source_endpoint == ("clock_main".into(), "pulse".into())
                && target_endpoint == ("tm_main".into(), "clock".into())
        ));

        let bad_both = LabelTarget::Cable {
            source: ("clock_main".into(), "absent".into()),
            target: ("tm_main".into(), "absent".into()),
            occurrence: 0,
        };
        assert!(matches!(
            label_value(world, &bad_both, &LabelKind::DisplayName),
            Err(CoreError::Patch(crate::PatchError::UnknownPort { module, port }))
                if module == "clock_main" && port == "absent"
        ));
        let source_port_precedes_target_module = LabelTarget::Cable {
            source: ("clock_main".into(), "absent".into()),
            target: ("not_active".into(), "clock".into()),
            occurrence: 0,
        };
        assert!(matches!(
            label_value(world, &source_port_precedes_target_module, &LabelKind::DisplayName),
            Err(CoreError::Patch(crate::PatchError::UnknownPort { module, port }))
                if module == "clock_main" && port == "absent"
        ));
    }

    #[test]
    fn module_resolution_uses_only_the_active_handle_and_rejects_stale_entities() {
        use bevy_ecs::world::World;

        let mut world = World::new();
        let active = crate::spawn_module_entity(&mut world, "test", "active");
        let _outside = crate::spawn_module_entity(&mut world, "test", "outside");
        let stale = crate::spawn_module_entity(&mut world, "test", "stale");
        assert!(world.despawn(stale));
        world.insert_resource(crate::PatchHandle {
            modules: [("active".into(), active), ("stale".into(), stale)].into(),
        });
        world.insert_resource(RackMeta::default());

        assert!(matches!(
            label_value(&mut world, &LabelTarget::Module("outside".into()), &LabelKind::DisplayName),
            Err(CoreError::UnknownModule(name)) if name == "outside"
        ));
        assert!(matches!(
            label_value(&mut world, &LabelTarget::Module("stale".into()), &LabelKind::DisplayName),
            Err(CoreError::UnknownModule(name)) if name == "stale"
        ));
        assert_eq!(
            label_value(&mut world, &LabelTarget::Rack, &LabelKind::DisplayName).unwrap(),
            Some(LabelValue::Text(String::new()))
        );

        let mut no_patch = World::new();
        for target in [LabelTarget::Rack, LabelTarget::Module("active".into())] {
            assert!(matches!(
                label_value(&mut no_patch, &target, &LabelKind::DisplayName),
                Err(CoreError::NoPatchLoaded)
            ));
        }
    }
}
