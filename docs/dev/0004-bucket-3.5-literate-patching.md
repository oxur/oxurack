# Bucket 3.5: Literate Patching

## Context

This is bucket 3.5 of the v0 plan. Its job is to **make patches into documents**, not just data. Per-entity `DisplayName` / `Description` / `Tags` components, a rack-level `RackMeta` resource with free-form notes, a configurable display-name formatter with a synth-domain acronym table, the `LabelChanged` notification extending the 3.25 catalog, and the patch-file format extension that round-trips all of it.

The framing is *literate patching* — Donald Knuth's literate-programming posture applied to modular synth patches: prose-about-the-patch is at least as important as the patch itself. Bucket 4 (graph) will be the *reading* side; this bucket is the *writing and storage* side.

**Bonus deliverable: rename `ParameterChanged.old` → `ParameterChanged.previous`.** 3.25 shipped with `old` as the field name; this bucket introduces `LabelChanged.previous` with the same semantics, and the consistency case for renaming `old` → `previous` retroactively is overwhelming. The field is always `None` today (`TODO(bucket-getter)`), so no consumer depends on its value. Folded into this bucket because it's mechanical and the consistency cost of waiting is higher than the rename cost. See §6 below.

---

## Required reading before recon

**`workbench/literate-patching-design.md`** is the design memo. Eleven locked decisions (D1–D10 plus the six resolved open items O1–O6). Do not re-litigate any of them in this bucket. If implementation reveals one is wrong, **stop and report** — that's a memo-level revision, not an implementation workaround.

**`assets/ai/ai-rust/skills/claude/SKILL.md`** is the Rust skill. Particularly **TD-03** (newtypes for invariants — strength **SHOULD**) which the `Tag` newtype follows verbatim, and **EH-03/EH-04** (typed errors via `thiserror`) which `TagError` follows.

**`workbench/event-substrate-design.md`** is the 3.25 design memo. Particularly the post-3.25 follow-up section (top of memo) which the `previous` rename updates.

**`workbench/deferred-tracking.md`** is the central deferred-items registry. Items D-35.1 through D-35.7 are 3.5 deferrals locked in the design memo; the spec must not silently land any of them.

---

## Recon findings (as of 2026-05-10, verify against the current tree)

All claims cite file:line as of 2026-05-10. Bucket 3.25 closed at commit `229b616`. If newer commits land before this bucket starts, re-verify each claim.

### Existing kind-level metadata (do not modify)

- **`OxurackModule::KIND: &'static str`** (`module.rs:139`).
- **`OxurackModule::DISPLAY_NAME: &'static str`** (`module.rs:141`).
- **`OxurackModule::DESCRIPTION: &'static str`** (`module.rs:143`, defaults to `""`).
- **`PortSchema.name: &'static str`** + **`PortSchema.description: &'static str`** (`module.rs:114, 122`).
- **`ModuleRegistration.display_name: String`** + **`ModuleRegistration.description: String`** (`module.rs:193, 195`) — snapshots stored in `ModuleRegistry`.

These describe **kinds**. 3.5's new components describe **instances**. Do not touch this layer.

### Patch-file format (extend)

- **`Patch`** (`patch.rs:27-38`) — top-level struct: `version`, `master_seed`, `bpm`, `modules: Vec<ModuleConfig>`, `cables: Vec<CableConfig>`. Derives `Debug, Clone, Serialize, Deserialize, PartialEq`.
- **`ModuleConfig`** (`patch.rs:46-53`) — `kind: String`, `instance_name: String`, `parameters: HashMap<String, ParameterValue>`. Same derives.
- **`CableConfig`** (`patch.rs:60-67`) — `source: (String, String)`, `target: (String, String)`, `transform: Option<CableTransform>`. Same derives.

3.5 adds optional fields to all three plus a new `RackConfig` struct attached to `Patch`. All new fields must be `Option<…>` or `Vec<…>` with `#[serde(default, skip_serializing_if = "…")]` so pre-3.5 patch files load unchanged.

### Patch loader (extend)

- **`apply_patch_to_world`** (`patch.rs:474+`) — already emits `ModuleAdded`, `ParameterChanged` (per initial param), `CableConnected`, `PatchLoaded` in that order. 3.5 adds `LabelChanged` emissions interleaved at the right points (see §4 below) and extends the `PatchLoaded` emission with the new `label_count` field.

### `ParameterChanged` field to rename

- **`ParameterChanged.old: Option<ParameterValue>`** declared at `event.rs:193`.
- **Doc-comment** at `event.rs:191-192` mentions `bucket-getter`.
- **Emission site 1**: `event.rs:351-360` (in `dispatch_set_parameter`).
- **Emission site 2**: `patch.rs:502-512` (in `apply_patch_to_world` per-parameter loop).
- **Test assertions**: grep for `old: None` in `crates/oxurack-core/src/*.rs`; expect roughly 7 hits across the test modules of `event.rs` and `patch.rs`.

### Plugin registration site

- **`build_core_plugin`** (`lib.rs:160+`) — current `add_message` calls at lines 164-175. New types added to this list: `LabelChanged`. New resources added via `init_resource`: `RackMeta`, `DisplayNameFormatter`. Mirror in `CorePluginFixed` if the existing registrations mirror.

### Bevy 0.18 Component derive

- Existing components use `#[derive(Component, ...)]` (e.g., `Module` at `module.rs:80-85`, `Cable` at `cable.rs:132+`). Follow the same pattern: `#[derive(Component, Clone, Debug, PartialEq, Eq)]` for the three new label components.

### Existing test patterns

- Bucket 3.25 established `world.resource::<Messages<T>>().iter_current_update_messages()` for assertion drains; carry this forward verbatim.
- `setup_set_parameter_world` (`event.rs:608+`) is the existing test helper; 3.5 will need a similar helper for label tests.

### Bevy `Resource` derive

- Existing resources: `ModuleRegistry` (`module.rs:237`), `ParameterRegistry` (`parameter.rs:96`), `PatchHandle` (`patch.rs:454`), `CableIndex`, `PropagationOrderDirty`. New resources for 3.5 use `#[derive(Resource, Default, Clone, Debug)]`.

---

## Decision for CC

### 1. Commit shape — single bucket commit, or rename as a preliminary commit

The `ParameterChanged.old` → `ParameterChanged.previous` rename is a small mechanical change. Two ways to land it:

**(A) One commit.** Rename is folded into the 3.5 commit alongside everything else. Single git log entry covers both. Simplest for the user.

**(B) Two commits.** First commit is `Rename ParameterChanged.old to previous (3.25 follow-up)`, second is `Bucket 3.5: literate patching`. Clean git history; `git blame` on the renamed field points at a focused commit rather than a sprawling one.

**Recommendation: (B).** Easier to revert if the rename surfaces issues; clean history; the bucket commit stays focused on the new feature. If you take (A), justify in the completion report.

---

## Locked decisions from design memo (do not re-litigate)

| # | Decision |
|---|----------|
| L1 | Three new components (`DisplayName`, `Description`, `Tags`) in `oxurack-core::label`. |
| L2 | New `RackMeta` resource for rack-level metadata. |
| L3 | Two-tier metadata preserved: kind-level (existing, do not touch) + instance-level (new). |
| L4 | Display-name fallback chain: explicit `DisplayName` component → formatter applied to instance_name. Kind-level `DisplayName` is **not** in the fallback chain. |
| L5 | `DisplayNameFormatter` resource with extensible `acronyms` HashMap. |
| L6 | `LabelChanged` event with `LabelKind` discriminant + `LabelValue` payload + `previous: Option<LabelValue>` + `entity: Option<Entity>` (None for rack-level). |
| L7 | A-everywhere emission discipline. No reactor systems. |
| L8 | Patch-file format extension — all new fields optional, pre-3.5 patches load unchanged. |
| L9 | Test discipline: `matches!` + field checks, no `.to_string().contains(...)`. |
| L10 | Ports do not get instance-level labels in v0. |
| O1 | Acronym table contents: synth-domain default (VCO, LFO, ADSR, etc. — full list below). |
| O2 | `Tag` validates at construction (TD-03). Private inner field, `Tag::new` returns `Result<Self, TagError>`. v0 rules: non-empty, no leading/trailing whitespace. |
| O3 | `LabelChanged.previous` field naming (not `old`). |
| O4 | Notes emission shape: single `LabelChanged` with full new list at patch-load. Runtime CRUD events deferred. |
| O5 | `BTreeSet<Tag>` for tag collections. |
| O6 | Components in new `oxurack-core::label` module. |

If implementation reveals any are wrong, **stop and report**.

---

## Work mode

**Thinking work stays in your context** with the Rust skill loaded. This includes: the `Tag::new` validation rule details; how to integrate `RackConfig` into the existing `Patch` struct without breaking pre-3.5 patches; whether the formatter algorithm needs special-cases for instance names that start with digits or contain only acronyms; the right placement of rack-level `LabelChanged` emissions relative to entity emissions; the borrow-checker pattern for emitting `LabelChanged` inside the existing per-module/per-cable loops in `apply_patch_to_world`.

**Lookup work may delegate to subagents.** This includes: grepping for all `old: None` occurrences for the rename; verifying the `serde` patterns used elsewhere in patch.rs for optional fields; finding existing `init_resource` patterns in `build_core_plugin`; counting the test sites that need updating for the rename.

Serial execution on thinking. Quality over speed. Subagent parallelism welcome on lookup only.

---

## The task

Substantial deliverable. Roughly seven sub-tasks; tackle in the order below or surface a reason for divergence.

### 1. Rename `ParameterChanged.old` → `ParameterChanged.previous` (3.25 follow-up)

Mechanical rename. As recommended in Decision 1, do this as a separate preliminary commit.

Sites to update:

- **`event.rs:193`** — field declaration.
- **`event.rs:191-192`** — field doc-comment (the line that mentions `bucket-getter`).
- **`event.rs:351-360`** — emission site in `dispatch_set_parameter` (specifically `old: None,`).
- **`patch.rs:502-512`** — emission site in `apply_patch_to_world` per-parameter loop (specifically `old: None,`).
- **All `old: None` test assertions** in `event.rs` and `patch.rs` test modules.
- **`event-substrate-design.md`** post-3.25 follow-up section — update field references.

Verify with `rg 'ParameterChanged\s*\{[^}]*\bold\s*:' crates/` returning zero hits after the rename.

Commit message suggestion: `Rename ParameterChanged.old to previous (3.25 follow-up)`.

### 2. Create `oxurack-core::label` module

New file: `crates/oxurack-core/src/label.rs`. Three components, one newtype with validated constructor, one error type, one resource for rack metadata, one resource for the formatter, the default acronym table.

```rust
use bevy_ecs::prelude::Component;
use bevy_ecs::prelude::Resource;
use std::collections::{BTreeSet, HashMap};

/// Optional instance-level human-readable name.
///
/// When present, overrides the kind-level
/// `ModuleRegistration::display_name` for display purposes. When
/// absent, consumers fall back to the formatter applied to the
/// instance name (see `DisplayNameFormatter`).
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
/// and O(log n) membership tests. Empty `Tags` is the absence of
/// tagging; the component is omitted entirely when no tags apply.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Tags(pub BTreeSet<Tag>);

/// A single tag.
///
/// Newtype with validation at construction (Rust guidelines TD-03).
/// Construction is the only way to produce a `Tag`; the inner field
/// is private. Future buckets may add length limits, internal-
/// whitespace rules, or case normalization without breaking callers.
///
/// v0 validation: non-empty, no leading or trailing whitespace.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(String);

impl Tag {
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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[non_exhaustive]
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum TagError {
    #[error("tag must not be empty")]
    Empty,
    #[error("tag must not have leading or trailing whitespace")]
    LeadingOrTrailingWhitespace,
}

/// Rack-level metadata. One resource per world.
#[derive(Resource, Default, Clone, Debug)]
pub struct RackMeta {
    pub display_name: String,
    pub description: String,
    pub notes: Vec<String>,
    pub tags: BTreeSet<Tag>,
}

/// Configurable formatter that derives a presentable display name
/// from an instance name.
///
/// Default behavior: split on `_` or `-`; for each token, look up
/// in `acronyms` (lowercased), else apply Title Case; pure-digit
/// tokens kept verbatim; join with spaces. Example: `"vco_1"` →
/// `"VCO 1"`, `"lead_voice_vco"` → `"Lead Voice VCO"`.
#[derive(Resource, Clone, Debug)]
pub struct DisplayNameFormatter {
    pub acronyms: HashMap<String, String>,
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
        // Tokenize on separators, transform each token, join with " ".
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
        // Title case: first letter upper, rest lower.
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
/// Returns the explicit `DisplayName` component if present, else
/// the formatter applied to the instance name.
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
```

Register the module in `lib.rs`:

```rust
pub mod label;

// In the `pub use` block:
pub use label::{
    DisplayName, Description, DisplayNameFormatter, RackMeta, Tag, TagError, Tags,
    display_name_of,
};
```

### 3. Extend the `LabelChanged` event in `oxurack-core::event`

Add to `event.rs`:

```rust
/// Emitted when an instance-level label or rack-level metadata changes.
///
/// `entity` is `None` for rack-level changes (RackMeta) and `Some(e)`
/// for entity-level changes. `previous` is `Some(prev)` when
/// overwriting an existing label, `None` for first-time set during
/// patch load.
#[derive(Message, Clone, Debug)]
pub struct LabelChanged {
    pub entity: Option<Entity>,
    pub instance_name: String,
    pub kind: LabelKind,
    pub previous: Option<LabelValue>,
    pub new: LabelValue,
    pub source: ChangeSource,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LabelKind {
    DisplayName,
    Description,
    Tags,
    /// Rack-level notes only.
    Notes,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LabelValue {
    Text(String),
    Tags(std::collections::BTreeSet<crate::label::Tag>),
    Notes(Vec<String>),
}
```

Export from `lib.rs` alongside the existing event exports.

### 4. Extend `PatchLoaded` with `label_count`

```rust
#[derive(Message, Debug, Clone)]
pub struct PatchLoaded {
    pub patch_name: String,
    pub source: PatchSource,
    pub module_count: usize,
    pub cable_count: usize,
    pub label_count: usize,  // NEW in 3.5
}
```

Update existing construction sites:

- `patch.rs:576-581` — emission in `apply_patch_to_world`. New construction takes label_count as a parameter (see §6).
- All test sites that construct `PatchLoaded {…}` — update to populate `label_count`.

### 5. Patch-file format extension

Extend the three structs in `patch.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Patch {
    pub version: String,
    pub master_seed: u64,
    pub bpm: f32,
    pub modules: Vec<ModuleConfig>,
    pub cables: Vec<CableConfig>,

    /// Rack-level metadata. Omitted from older patch files; loads
    /// as `None` and produces `RackMeta::default()` when applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rack: Option<RackConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RackConfig {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModuleConfig {
    pub kind: String,
    pub instance_name: String,
    pub parameters: HashMap<String, crate::ParameterValue>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CableConfig {
    pub source: (String, String),
    pub target: (String, String),
    pub transform: Option<crate::CableTransform>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
```

Why `String` (with `is_empty` check) rather than `Option<String>`: pre-3.5 RON files don't have the field at all; `#[serde(default)]` gives `String::new()`; `skip_serializing_if = "String::is_empty"` keeps freshly-saved patches clean. The contract "empty string = no label" works cleanly throughout.

**Tags storage choice:** `Vec<String>` in the patch file (for sequence-friendly serialization), converted to `BTreeSet<Tag>` at apply-patch time. If any tag string fails `Tag::new`, return a new error variant (see §5b).

### 5b. Patch loader integration — error handling for invalid tags

Patch-file `tags: Vec<String>` may contain invalid tag strings. The loader must convert each via `Tag::new(s)?` and propagate failures. Add a new variant to `PatchError` (`error.rs`):

```rust
#[error("invalid tag '{tag}' on {context}: {reason}")]
InvalidTag {
    /// Where the bad tag appeared, e.g. "module 'vco_1'" or "rack".
    context: String,
    tag: String,
    reason: crate::TagError,
},
```

Patch load fails fast on the first invalid tag. The error message must identify which entity (or "rack") the bad tag came from. Add a Display test for the new variant.

### 6. Extend `apply_patch_to_world` emission

Update `apply_patch_to_world` at `patch.rs:474+` to emit `LabelChanged` events at the right points and to track the label count for `PatchLoaded.label_count`.

**Emission order within the patch-load batch:**

1. For each module in declaration order:
   - Spawn module entity (existing).
   - Insert `DisplayName` / `Description` / `Tags` components if the `ModuleConfig` has non-empty values.
   - Emit `ModuleAdded` (existing).
   - **NEW: Emit `LabelChanged` for each non-empty label field**, with `entity: Some(module_entity)`, `instance_name`, `kind` matching, `previous: None`, `new: LabelValue::{Text|Tags}(...)`, `source: ChangeSource::PatchLoad`. Increment `label_count`.
   - Emit `ParameterChanged` per initial parameter (existing; uses renamed `previous` field).
2. For each cable in declaration order:
   - Spawn cable entity (existing).
   - Insert `DisplayName` / `Description` / `Tags` components if `CableConfig` has non-empty values.
   - Emit `CableConnected` (existing).
   - **NEW: Emit `LabelChanged` for each non-empty label field on the cable**. Same shape as module label emission, `entity: Some(cable_entity)`.
3. **NEW: Rack-level processing** — if `Patch.rack` is `Some(RackConfig)`:
   - Build a `RackMeta` value, validating tags via `Tag::new`.
   - Insert `RackMeta` as a resource (`world.insert_resource(rack_meta.clone())`).
   - For each non-empty rack-level field, emit `LabelChanged` with `entity: None`, `instance_name: String::new()`, `kind: LabelKind::{DisplayName|Description|Notes|Tags}`, `previous: None`, `new: LabelValue::...`, `source: ChangeSource::PatchLoad`. Increment `label_count`.
   - If `Patch.rack` is `None`, do nothing for rack-level (don't insert `RackMeta::default()` — `init_resource` in the plugin handles the default; the patch loader only writes `RackMeta` when the patch declares one).
4. Emit `PatchLoaded` with the new `label_count` field populated.

**Within a single module's labels**, emit in the order: DisplayName → Description → Tags. Within rack-level, emit in order: DisplayName → Description → Notes → Tags. Consistent ordering aids consumer code and test assertions.

**Tag conversion:** wrap `Tag::new(s).map_err(|e| PatchError::InvalidTag { context: …, tag: s.clone(), reason: e })` for every tag string. Module context is `format!("module '{}'", module_config.instance_name)`. Cable context is more awkward (cables don't have names yet); use `format!("cable from '{}.{}' to '{}.{}'", source.0, source.1, target.0, target.1)`. Rack context is `"rack"`.

### 7. Plugin registration

In `build_core_plugin` (`lib.rs:160+`), add:

```rust
.add_message::<LabelChanged>()
.init_resource::<RackMeta>()
.init_resource::<DisplayNameFormatter>()
```

Place them alongside the existing message/resource registrations. If `CorePluginFixed` mirrors, mirror there too.

---

## Tests

Per-test discipline carries forward from 3.25 verbatim: `matches!` + field checks; no `.to_string().contains(...)`; positive paths assert observable state; negative paths drain empty.

### Component basics

1. `test_display_name_component_insert_and_read` — insert component, read back, assert value.
2. `test_description_component_insert_and_read` — same with `Description`.
3. `test_tags_component_insert_and_read` — same with `Tags`; assert `BTreeSet` ordering.

### Tag validation

4. `test_tag_new_valid_input` — `Tag::new("lead")` returns `Ok`, `as_str()` returns `"lead"`.
5. `test_tag_new_rejects_empty` — `Tag::new("")` returns `Err(TagError::Empty)`.
6. `test_tag_new_rejects_leading_whitespace` — `Tag::new(" lead")` returns `Err(TagError::LeadingOrTrailingWhitespace)`.
7. `test_tag_new_rejects_trailing_whitespace` — `Tag::new("lead ")` returns `Err(TagError::LeadingOrTrailingWhitespace)`.
8. `test_tag_new_allows_internal_whitespace` — `Tag::new("lead voice")` returns `Ok`. (Internal whitespace permitted by design.)
9. `test_tag_error_display` — `TagError::Empty.to_string()` etc. Standard Display test per existing convention.

### DisplayNameFormatter

10. `test_formatter_default_acronyms` — assert `"vco_1"` → `"VCO 1"`, `"lfo_master"` → `"LFO Master"`, `"adsr_envelope"` → `"ADSR Envelope"`.
11. `test_formatter_title_case` — assert `"lead_voice"` → `"Lead Voice"` (no acronyms).
12. `test_formatter_mixed` — assert `"lead_voice_vco"` → `"Lead Voice VCO"` (mixed title-case and acronym).
13. `test_formatter_runtime_extension` — extend the acronym table with a new entry; assert it's honored.
14. `test_formatter_digit_tokens` — assert `"vco_1"` keeps `"1"` verbatim; assert `"track_42"` → `"Track 42"`.
15. `test_formatter_dash_separator` — assert `"lead-vco"` → `"Lead VCO"` (dashes work like underscores).

### Fallback chain

16. `test_display_name_of_uses_component_when_present` — given `Some(DisplayName("Lead"))`, `display_name_of` returns `"Lead"` regardless of instance_name or formatter.
17. `test_display_name_of_uses_formatter_when_absent` — given `None`, returns formatter-applied result.
18. `test_display_name_of_treats_empty_string_as_absent` — given `Some(DisplayName(""))`, falls back to formatter. (Empty string DisplayName = no label.)

### LabelChanged emission

19. `test_label_changed_emitted_on_patch_load_module_with_display_name` — patch with one module that has a non-empty `display_name`; load; assert exactly one `LabelChanged` with `kind: DisplayName`, `previous: None`, `source: PatchLoad`, `entity: Some(_)`.
20. `test_label_changed_emitted_on_patch_load_module_with_tags` — same shape, `kind: Tags`, `new: LabelValue::Tags(...)`.
21. `test_label_changed_emitted_on_patch_load_cable_with_label` — patch with one cable that has a `display_name`; assert emission.
22. `test_label_changed_rack_level_emissions` — patch with `[rack]` section having display_name, description, notes, tags; assert four `LabelChanged` events with `entity: None`, in DisplayName → Description → Notes → Tags order.
23. `test_label_changed_not_emitted_when_labels_empty` — load patch with no labels anywhere; assert zero `LabelChanged` events.

### RackMeta loading

24. `test_rack_meta_loaded_from_patch` — patch with `[rack]` section; load; assert `RackMeta` resource has expected values.
25. `test_rack_meta_default_when_section_absent` — patch without `[rack]` section; load; assert `RackMeta::default()` (the plugin-init default is unchanged).
26. `test_rack_meta_invalid_tag_returns_error` — patch with `[rack].tags = [" bad"]`; load; assert `PatchError::InvalidTag { context: "rack", … }`.

### PatchLoaded extension

27. `test_patch_loaded_label_count_zero_for_unlabeled_patch` — unlabeled patch; assert `label_count == 0`.
28. `test_patch_loaded_label_count_counts_all_emissions` — patch with N entity labels + M rack-level labels; assert `label_count == N + M`.

### Backward compatibility

29. `test_legacy_patch_files_load_unchanged` — load a patch file that omits all 3.5 fields and the `[rack]` section; assert it loads successfully and behaves identically to a 3.5-aware patch with no labels.

### Tag conversion error path

30. `test_module_invalid_tag_returns_error` — patch with `modules[0].tags = ["bad "]`; load; assert `PatchError::InvalidTag { context: "module 'vco_1'", tag: "bad ", reason: TagError::LeadingOrTrailingWhitespace }` (or similar).
31. `test_cable_invalid_tag_returns_error` — same shape for a cable tag.

### Rename verification

32. `test_parameter_changed_uses_previous_field_name` — already covered by extending the existing bucket-3.25 tests to use `previous: None` instead of `old: None`. No new test required; just verify the rename landed in all assertions.

---

## Out of scope for this bucket

- Runtime label-mutation `CoreCommand` variants (`SetDisplayName`, etc.) — deferred (D-35.1).
- Port-level instance labels — deferred (D-35.2).
- Tag validation extensions beyond non-empty + no leading/trailing whitespace — deferred (D-35.3).
- Hierarchical tags — deferred (D-35.4).
- Note IDs and per-note CRUD events — deferred (D-35.5).
- Search/filter machinery driven by tags — deferred (D-35.6).
- User-config persistence for acronym table — deferred (D-35.7).
- Closing any other `NotImplemented` arms in `dispatch_core_command`. The four still-pending arms (`LoadPatch`, `SavePatch`, `AddCable`, `RemoveCable`) stay `NotImplemented`.
- Populating `ParameterChanged.previous` with a real value. The rename is mechanical; populating it still depends on `bucket-getter` (D-25.1).

---

## Suggested steps

1. **Recon against the current tree.** Verify cited file:line locations. Confirm Bevy 0.18 conventions match expectations. Run grep for `old: None` to count rename sites.
2. **Preliminary commit: ParameterChanged rename.** Mechanical, focused commit. Run `make check`. Confirms clean baseline before the bigger work.
3. **Create `oxurack-core::label` module.** Three components, `Tag`/`TagError`, `RackMeta`, `DisplayNameFormatter` with default acronyms, `display_name_of` helper. Plus unit tests for components, Tag validation, and formatter behavior — these stand alone without patch-load infrastructure.
4. **Extend `event.rs`.** Add `LabelChanged`, `LabelKind`, `LabelValue`. Extend `PatchLoaded` with `label_count`. Export from `lib.rs`. Register with the plugin.
5. **Extend patch-file structs.** Add new optional fields to `Patch`, `ModuleConfig`, `CableConfig`. Add `RackConfig`. Add `PatchError::InvalidTag` variant + Display test.
6. **Extend `apply_patch_to_world`.** Insert components, emit `LabelChanged`, build `RackMeta`, increment `label_count`. Get the emission order right (modules → labels → params → next module; then cables → labels; then rack-level; then `PatchLoaded`).
7. **Test suite.** Per the list above (32 tests; some are existing-test updates from the rename, most are new).
8. **Update `PatchLoaded` doc-comment** to mention `label_count`. Update `apply_patch_to_world` doc-comment to describe the new emission discipline.
9. **Run `make format`, `make lint`, `make test`.** No `#[ignore]`. No silent failures.

---

## DoD (every criterion is a required pass condition)

1. `cargo clean && make check` passes from a fresh state.
2. **Preliminary rename commit.** `ParameterChanged.old` → `ParameterChanged.previous` lands as a separate commit before the main 3.5 commit. Grep `rg 'ParameterChanged\s*\{[^}]*\bold\s*:' crates/` returns zero hits.
3. **New `oxurack-core::label` module** exists with `DisplayName`, `Description`, `Tags`, `Tag`, `TagError`, `RackMeta`, `DisplayNameFormatter`, and the `display_name_of` helper function. All exported from `oxurack-core` crate root.
4. **`Tag` has private inner field**, `Tag::new` constructor returning `Result`, `Tag::as_str` accessor. No `pub struct Tag(pub String)`.
5. **`TagError` derives `thiserror::Error`** with `Empty` and `LeadingOrTrailingWhitespace` variants. `#[non_exhaustive]`.
6. **`DisplayNameFormatter` default** populated with the synth-domain acronym table from §2 (22 entries: VCO, VCA, VCF, DCO, DCA, DCF, LFO, ADSR, AHDSR, AR, ASR, PWM, FM, AM, WT, CV, BPM, MIDI, EQ, FX, I/O, L/R).
7. **`LabelChanged` event** with `entity: Option<Entity>`, `instance_name: String`, `kind: LabelKind`, `previous: Option<LabelValue>`, `new: LabelValue`, `source: ChangeSource`. Derives `Message, Clone, Debug`.
8. **`LabelKind` and `LabelValue` enums** with `#[non_exhaustive]`.
9. **`PatchLoaded` extended** with `label_count: usize` field. All existing construction sites updated.
10. **`Patch`, `ModuleConfig`, `CableConfig` extended** with optional label fields per §5. New `RackConfig` struct exists.
11. **`PatchError::InvalidTag`** variant added with `context`, `tag`, `reason` fields. Display test included.
12. **Plugin registration** — `LabelChanged` added to `build_core_plugin`. `RackMeta` and `DisplayNameFormatter` registered via `init_resource`.
13. **`apply_patch_to_world` emissions** in correct order: per-module (ModuleAdded → labels → ParameterChanged) → per-cable (CableConnected → labels) → rack-level labels → PatchLoaded(extended). `label_count` correctly counts all `LabelChanged` emissions in this batch.
14. **Rack-level emissions use `entity: None`**, entity-level emissions use `entity: Some(e)`. Provenance correctly `ChangeSource::PatchLoad`.
15. **Tag conversion in patch loader** uses `Tag::new(s)?` and propagates `PatchError::InvalidTag` with correct context strings.
16. **Pre-3.5 patch files load unchanged.** `test_legacy_patch_files_load_unchanged` passes.
17. All 32 listed tests pass (some are existing-test updates, most are new).
18. **Doc-comments updated.** `PatchLoaded` mentions `label_count`. `apply_patch_to_world` documents the new emission discipline including label_count contract. `Tag` and `TagError` docs follow Rust skill DC-01 conventions.
19. **No `.to_string().contains(...)` patterns** in any new or modified test.
20. **Four `NotImplemented` arms unchanged** in `dispatch_core_command`. Grep verifies `LoadPatch`, `SavePatch`, `AddCable`, `RemoveCable` still return `NotImplemented`.
21. **Zero `old: None` in `ParameterChanged` constructions.** Rename complete.
22. **The 3.5 commit** lands as a single atomic commit (after the preliminary rename commit). Clear message (e.g., `Bucket 3.5: literate patching — labels, RackMeta, formatter, LabelChanged event`).

---

## Completion report format

```
Bucket 3.5 complete at <commit sha>.
Preliminary rename commit: <commit sha>.

Decision 1 (commit shape): <A / B> — <justification>

Recon findings:
- ParameterChanged rename sites updated: <N>
- Existing apply_patch_to_world signature: <accurate / drifted — describe>
- Plugin registration site location: <file:line>
- Backward compatibility of patch files verified via test: <yes / no>
- <any other recon-revealed shape difference>

New types added in oxurack-core::label (N):
- <list with one-line description>

New types added in oxurack-core::event (N):
- <list>

Types extended (N):
- <list with what changed>

Plugin registrations added (N):
- <list>

Emission sites wired (N):
- <list with one-line description per site>

Patch file format changes:
- <list of new optional fields>

Code files changed (N):
- <list>

Tests added (N) / modified (M):
- <list with name + what each asserts>

Verification: cargo clean && make check — <passed / failures>
Verification: PatchLoaded.label_count counts all label emissions — <yes / no>
Verification: pre-3.5 patches load unchanged — <yes / no>
Verification: Tag::new rejects invalid input with correct error variants — <yes / no>
Verification: rack-level emissions use entity: None — <yes / no>
Verification: zero `old: None` in ParameterChanged constructions — <yes / no>
Verification: four NotImplemented arms unchanged — <yes / no>

Cross-bucket dependencies surfaced:
- <anything bucket 4+ will need to handle, or "none">

Design memo follow-up edits required:
- <list, or "none">

Deferred items registry updates required:
- <which D-35.x entries should be removed / updated / added>

Intentional exceptions or deferrals:
- <list with explicit justification, or "none">

Spec-adherence walk:
- DoD #1: <met / not met — explanation if not>
- DoD #2: <met / not met>
- ... (all 22)

Nothing silently skipped.
```

---

## Anti-softpedal reminder

Watch-items particular to this bucket:

- **`Tag(pub String)` slips back in.** TD-03 is unambiguous: private inner field. If a refactor accidentally exposes the inner field via a `pub` on the tuple struct, the invariant is broken. Verify the field is private (no `pub` keyword before `String`).
- **`label_count` doesn't match the emitted count.** If `apply_patch_to_world` increments `label_count` separately from the emissions, the two can drift. Either compute `label_count` from a running counter incremented at each emission site, or assert in tests that `label_count` exactly matches the queue length sum across `Messages<LabelChanged>` for this batch.
- **Rack-level emission uses wrong `instance_name`.** The empty string is the convention; do not use `"rack"` or `"world"` or anything else. Consumers distinguish via `entity: None`, not by inspecting `instance_name`. Wrong `instance_name` won't fail any test but pollutes the audit log.
- **Empty-string `display_name` triggers spurious `LabelChanged`.** A patch with `display_name = ""` (explicitly empty) should not emit a `LabelChanged` — `String::is_empty()` is the discriminator. Test the boundary.
- **Backward compatibility silently breaks.** A pre-3.5 patch file that loads with the new code must produce the same behavior as before — no new components inserted, no spurious `LabelChanged` events, `RackMeta::default()` resource present (from plugin init, not from loader). Test #29 is non-negotiable.
- **`PatchError::InvalidTag` swallows the underlying reason.** The `reason: TagError` field must propagate; do not stringify it before stuffing into the error.
- **The rename misses a test site and CI passes anyway because the test wasn't running.** Verify all `old: None` sites updated via grep, not just the ones you remember.

If recon reveals the existing `apply_patch_to_world` has changed shape since the cited line numbers, stop and report rather than guessing. If `serde` patterns elsewhere in patch.rs use a different optional-field convention than `String::is_empty`, follow the existing convention rather than introducing a new one.

---

## After you report

I'll do trust-but-verify: confirm Decision 1's choice is consistent with the commit shape; grep for `pub struct Tag(pub String)` (zero hits expected); read the `apply_patch_to_world` emission section and confirm the order matches the spec; check that backward-compat test loads a genuinely-old patch file and not a synthesized "old-shaped" struct; verify `label_count` matches actual emission count; check that the four `NotImplemented` arms are preserved; verify the `ParameterChanged.previous` rename is complete. Then we sign off or loop back.
