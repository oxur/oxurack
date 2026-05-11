---
number: 13
title: "Literate Patching (Bucket 3.5) — Design Memo"
author: "treating each"
component: All
tags: [change-me]
created: 2026-05-10
updated: 2026-05-10
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# Literate Patching (Bucket 3.5) — Design Memo

**Status:** Draft
**Date:** 2026-05-10
**Type:** Design memo (precedes CC-facing implementation spec)
**Origin:** RackGraph design memo flagged 3.5 as "literate patching" between event substrate (3.25) and graph (4). 3.25 is now done; this memo specifies the literate-patching layer.

---

## Confirmed decisions

| # | Decision | Rationale |
|---|----------|-----------|
| D1 | **Three new components** on entities (modules and cables): `DisplayName`, `Description`, `Tags`. All optional — most entities won't have them. | The unit of "literate" annotation is per-entity. ECS components are the natural storage. |
| D2 | **One new resource** for rack-level metadata: `RackMeta` with display_name, description, notes, tags fields. | The rack itself (the patch as a whole) is not an entity; it's the world. Resource is the natural storage. |
| D3 | **Two-tier metadata model preserved.** Kind-level metadata (existing: `OxurackModule::DISPLAY_NAME`, `OxurackModule::DESCRIPTION`, `PortSchema::description`) stays as it is. Instance-level labels (new) override or supplement. | Don't disturb the existing kind-metadata layer; layer the new stuff on top. |
| D4 | **Display-name fallback chain.** When a consumer asks "what's the display name of `vco_1`?": (a) if `DisplayName` component is present, use it; (b) else apply formatter to instance_name (e.g., "vco_1" → "VCO 1"). | Explicit labels override; formatter fills in for the unlabeled common case. |
| D5 | **Configurable display-name formatter** as a `DisplayNameFormatter` resource. Built-in default with synth-domain acronyms (VCO, LFO, ADSR, etc.) plus snake_case → Title Case rule. User-extensible via the resource. | Synth users see "VCO 1", not "vco 1". The acronym table is opinionated for the domain. |
| D6 | **`LabelChanged` notification** extends the 3.25 catalog. Single event type with `LabelKind` enum (DisplayName, Description, Tags) and `Option<Entity>` (None means rack-level). | Consistent with 3.25's pattern. Single event with discriminant beats multiple parallel event types — fewer registrations, simpler consumer code. |
| D7 | **A-everywhere emission discipline carries forward.** Every label-mutation site emits `LabelChanged`. No reactor systems. | Same architectural rule as 3.25. |
| D8 | **Patch file format extension.** New optional fields on `ModuleConfig` (display_name, description, tags), new optional fields on `CableConfig` (same three), new top-level `[rack]` section (display_name, description, notes, tags). All optional. | Round-trip is non-negotiable; literate patches are the whole point. |
| D9 | **Test discipline carries forward from 3.25.** Per-emission-site tests with `matches!` + field checks. No `.to_string().contains(...)`. | Same architectural rule. |
| D10 | **Ports do NOT get instance-level labels in v0.** Port schemas (kind-level) already carry `name` and `description`. Per-instance port labels are deferred to a future bucket if a use case demonstrates need. | Avoid scope creep. Ports are derived from schemas; adding instance-level overrides per-port would multiply complexity for little immediate benefit. |

---

## Context: why literate patching

A patch is more than a set of modules and cables — it's a *document*. A user who saves "tutorial-1.ron" three months later wants to know: what is this patch for? Which modules are doing the interesting work? What did I learn from this? What did I plan to try next?

Today, none of that survives serialization. Module instance names (`vco_1`, `filter_1`) are the only labels in the patch file, and they're machine-readable, not human-friendly. Description, intent, annotations, and grouping — all live in the user's head and disappear when the session ends.

Literate patching solves this by treating each rack and each entity in it as a candidate for human-readable annotation:

- A patch has a **name** ("First-blush tutorial") distinct from its filename.
- A patch has a **description** ("simple VCO-VCA-out chain for testing the science rig").
- A patch has free-form **notes** ("the gain knob on vca_1 is sensitive; small moves matter").
- A patch has **tags** ("tutorial", "test", "draft").
- Each module can have its own **display name** ("Lead Voice VCO" instead of "vco_1"), **description**, and **tags**.
- Each cable can have the same.

The lineage name is deliberate — Donald Knuth's *literate programming* treats prose-about-the-code as at least as important as the code itself. *Literate patching* applies the same posture to modular synth patches: the prose about the patch is as important as the patch.

The structural payoff scales beyond personal documentation:

- **Patch sharing.** A community-shared patch with name + description + tags is discoverable; without them, it's a binary blob.
- **Search and filtering.** "Show me all my modules tagged 'lead'." "Find the patch with 'first attempt at acid bass' in the description."
- **REPL ergonomics.** Display names that read "VCO 1" instead of "vco_1" are friendlier to look at in any user-facing surface.
- **TUI rendering.** When bucket 4's graph lands and a TUI builder begins, every node needs a label. Falling back to instance names produces a sea of `vco_1`, `vco_2`, `vca_1`, `vca_2`. Falling back to formatter output produces "VCO 1", "VCO 2", "VCA 1", "VCA 2". User-set explicit names produce "Lead", "Bass", "Drums", "Reverb send".
- **Future Lykn integration.** Lykn-script-attached module configurations (the chord module use case that started this whole thread) want a place to live; the `Description` field is a natural home for human-readable explanation of what the Lykn code does, and `Tags` can drive auto-discovery of similarly-shaped scripts.

This bucket completes the substrate. Consumers (bucket 4's graph, future TUI, future REPL display, future Lykn) all read the same per-entity labels and rack metadata.

---

## Architecture: instance labels layered over kind metadata

### Kind-level metadata (already in codebase, do not modify)

Verified in `oxurack-core` as of 2026-05-10:

- **`OxurackModule::DISPLAY_NAME: &'static str`** (`module.rs:141`) — kind-level human-readable name, e.g., "Voltage-Controlled Oscillator" for the VCO module kind. Same string for every instance of that kind.
- **`OxurackModule::DESCRIPTION: &'static str`** (`module.rs:143`, defaults to `""`) — kind-level description.
- **`PortSchema::description: &'static str`** (`module.rs:122`) — kind-level port description, declared at registration time.
- **`ModuleRegistration::display_name: String`** (`module.rs:193`) and **`ModuleRegistration::description: String`** (`module.rs:195`) — snapshots of the above, stored in the `ModuleRegistry` resource for lookup.

These describe *what a kind is*. 3.5 does not change them.

### Instance-level labels (new in 3.5)

Three new components, all in `oxurack-core::label` (new module) or `oxurack-core::module` (alongside `Module`). I lean a new `label.rs` module for clarity; the CC spec can lock placement.

```rust
/// Optional instance-level human-readable name.
///
/// When present, overrides the kind-level [`ModuleRegistration::display_name`]
/// for display purposes. When absent, consumers fall back to the formatter
/// applied to the instance name (see [`DisplayNameFormatter`]).
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct DisplayName(pub String);

/// Optional instance-level description.
///
/// When present, supplements (does not replace) the kind-level
/// [`ModuleRegistration::description`]. Consumers may show both,
/// or prefer the instance description when it exists.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Description(pub String);

/// Optional instance-level tag set.
///
/// Tags are flat (non-hierarchical) strings used for grouping,
/// filtering, and search. Stored as a [`BTreeSet`] for deterministic
/// serialization order and O(log n) membership tests.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Tags(pub BTreeSet<Tag>);

/// A single tag.
///
/// Newtype around `String` with validation at construction
/// (Rust guidelines TD-03: "Newtypes for Semantic Distinctions and
/// Invariants"). Construction is the only way to produce a `Tag`;
/// the inner field is private. Future buckets may add length limits,
/// internal-whitespace rules, or case normalization without breaking
/// callers — all callers go through [`Tag::new`].
///
/// v0 validation rules: non-empty, no leading or trailing whitespace.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(String);

impl Tag {
    /// Construct a new tag, validating the input.
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

    /// Borrow the tag as a string slice.
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
```

Eligible target entities for v0: **modules and cables**. Ports do not get instance-level labels (D10).

### Rack-level metadata (new in 3.5)

```rust
/// Rack-level metadata.
///
/// A single resource per world. Describes the rack as a whole —
/// the patch's display name, description, free-form notes, and tags.
/// Distinct from per-entity labels (which describe specific modules
/// or cables within the rack).
///
/// All fields are optional (empty strings / empty collections);
/// `RackMeta::default()` produces an unannotated rack.
#[derive(Resource, Default, Clone, Debug)]
pub struct RackMeta {
    /// Human-readable patch name (distinct from filename).
    pub display_name: String,
    /// Patch description / purpose / usage notes.
    pub description: String,
    /// Free-form notes attached to the rack as a whole.
    ///
    /// Each `String` is a paragraph or block of prose. Use for
    /// reminders, observations, ideas, anything that doesn't fit
    /// the `description` field's "what is this" framing.
    pub notes: Vec<String>,
    /// Patch-level tags.
    pub tags: BTreeSet<Tag>,
}
```

`RackMeta::default()` is an empty annotation — that's the unannotated case. Tests and bare-bones patches don't need to populate it.

### The display-name fallback chain

When a consumer asks "what should I display for `vco_1`?", the algorithm:

1. **If the entity has a `DisplayName` component**, use it verbatim. *(User-set; highest priority.)*
2. **Else apply the `DisplayNameFormatter` to the instance name.** e.g., `"vco_1"` → `"VCO 1"`. *(Algorithmic; falls back gracefully.)*

The kind-level `display_name` is NOT in this chain. Reason: it's a type label ("Voltage-Controlled Oscillator"), not an instance label. Showing "Voltage-Controlled Oscillator 1, Voltage-Controlled Oscillator 2" everywhere would clutter UI. The kind-level name is for module catalogs, type pickers, and tooltips — surfaces where the user is choosing or learning a *kind*, not identifying a specific *instance*.

### The `DisplayNameFormatter`

```rust
/// Configurable formatter that derives a presentable display name
/// from a machine-readable instance name.
///
/// Default behavior:
/// - Split on `_` or `-` into tokens.
/// - For each token: if (lowercased) appears in `acronyms`, use the
///   table's casing; else apply Title Case (first letter upper, rest
///   lower); pure-digit tokens kept verbatim.
/// - Join tokens with spaces.
///
/// Example: `"vco_1"` → `"VCO 1"`, `"lead_voice_vco"` →
/// `"Lead Voice VCO"`, `"adsr_envelope_2"` → `"ADSR Envelope 2"`.
#[derive(Resource, Clone, Debug)]
pub struct DisplayNameFormatter {
    /// Acronym table: lowercase token → display casing.
    pub acronyms: HashMap<String, String>,
    /// Separator characters for tokenization. Default `['_', '-']`.
    pub separators: Vec<char>,
}

impl Default for DisplayNameFormatter {
    fn default() -> Self {
        let mut acronyms = HashMap::new();
        // Synth domain defaults; users can extend or override.
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
    /// Format a machine-readable instance name as a display name.
    pub fn format(&self, instance_name: &str) -> String { /* … */ }
}
```

The formatter is a resource so users (or future config systems) can extend or override the acronym table via:

```rust
let mut formatter = world.resource_mut::<DisplayNameFormatter>();
formatter.acronyms.insert("kvm".into(), "KVM".into());
```

### Default acronym table

The starter set (opinionated for synth/audio contexts; CC spec can adjust):

| Lowercase | Display |
|-----------|---------|
| `vco` | VCO |
| `vca` | VCA |
| `vcf` | VCF |
| `dco` | DCO |
| `dca` | DCA |
| `dcf` | DCF |
| `lfo` | LFO |
| `adsr` | ADSR |
| `ahdsr` | AHDSR |
| `ar` | AR |
| `asr` | ASR |
| `pwm` | PWM |
| `fm` | FM |
| `am` | AM |
| `wt` | WT |
| `cv` | CV |
| `bpm` | BPM |
| `midi` | MIDI |
| `eq` | EQ |
| `fx` | FX |
| `io` | I/O |
| `lr` | L/R |

Capitalization choices intentional: I/O and L/R use slashes; everything else is plain uppercase. These render at glance in a TUI without losing the standard typography.

---

## Patch file format extension

All new fields optional. Existing patch files load unchanged.

### Module config

```toml
[[modules]]
kind = "vco"
instance = "vco_1"
display_name = "Lead Voice VCO"      # NEW, optional
description = "Primary voice"        # NEW, optional
tags = ["lead", "voice"]             # NEW, optional

[modules.parameters]
freq = 440.0
```

### Cable config

```toml
[[cables]]
source = ["vco_1", "out"]
target = ["filter_1", "in"]
display_name = "VCO → Filter"        # NEW, optional
description = "Main signal path"     # NEW, optional
tags = ["audio"]                     # NEW, optional
```

### Rack-level section

```toml
[rack]
display_name = "First Tutorial Patch"
description = "VCO-VCA-out chain for testing the science rig"
notes = [
  "The gain on vca_1 is sensitive — small moves matter",
  "Try the VCO at 220 Hz; sounds nicer in this range",
]
tags = ["tutorial", "test"]
```

Format choice: `[rack]` is a top-level table, distinct from `[[modules]]` (array) and `[[cables]]` (array). RON equivalent has the same structure with different syntax.

If `[rack]` is omitted entirely, the loader produces `RackMeta::default()` (empty annotation). Existing patch files without `[rack]` continue to work.

---

## `LabelChanged` notification

Extends the 3.25 event catalog. Single event type with discriminants:

```rust
/// Emitted when an instance-level label or rack-level metadata changes.
///
/// Producers: future label-mutation APIs (`set_display_name`, etc.);
/// patch-load emission during `apply_patch_to_world` for initial
/// labels.
///
/// `entity` is `None` for rack-level changes (`RackMeta`) and
/// `Some(e)` for entity-level changes.
#[derive(Message, Clone, Debug)]
pub struct LabelChanged {
    /// Target entity, or `None` for rack-level metadata changes.
    pub entity: Option<Entity>,
    /// User-facing instance name, for entity changes. Empty for
    /// rack-level changes.
    pub instance_name: String,
    /// Which label changed.
    pub kind: LabelKind,
    /// Previous label payload, if available. `None` for first-time
    /// label set; `Some(prev)` when overwriting an existing label.
    pub previous: Option<LabelValue>,
    /// New label payload.
    pub new: LabelValue,
    /// Provenance: what triggered this change.
    pub source: ChangeSource,
}

/// Which label was changed.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LabelKind {
    DisplayName,
    Description,
    Tags,
    /// Rack-level notes (added/removed/edited).
    Notes,
}

/// Payload for a `LabelChanged` notification.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LabelValue {
    /// A `DisplayName` or `Description` change. `String::new()` means cleared.
    Text(String),
    /// A `Tags` change. Empty set means cleared.
    Tags(BTreeSet<Tag>),
    /// A `Notes` change (rack-level only). Carries the full list.
    Notes(Vec<String>),
}
```

`ChangeSource` is the existing enum from 3.25 (`Command`, `DirectApi`, `PatchLoad`). Reused verbatim.

`previous: Option<LabelValue>` is populated by reading the component value before overwriting (`world.get::<DisplayName>(entity).cloned()`). Unlike `ParameterChanged.old` (which is deferred because `ParameterRegistry` has no getter machinery), label reads are straightforward — no registry symmetry required. The field carries the prior value so audit-log consumers, undo, and diff renderers all get the full delta in one event.

Naming note: `previous`, not `old`. *Previous* describes the temporal relationship; *old* carries connotations (deprecation, staleness, obsolescence) that don't fit a normal value-change event. See "Consistency follow-up for `ParameterChanged`" below.

---

## Emission discipline

A-everywhere, same as 3.25.

### Emission sites for 3.5

| Notification | Emission site |
|---|---|
| `LabelChanged` (entity-level) | Future `set_display_name` / `set_description` / `set_tags` APIs (not in 3.5 scope; emit when added) |
| `LabelChanged` (entity-level, `source: PatchLoad`) | `apply_patch_to_world` — emit when initial labels are set during module/cable spawn |
| `LabelChanged` (rack-level, `source: PatchLoad`) | `apply_patch_to_world` — emit once per non-empty `RackMeta` field after parsing `[rack]` section |

For v0, the only active emission site is patch-load. Runtime mutation APIs (set_display_name etc.) come in whichever bucket adds REPL commands for them — likely the same bucket that adds the command-tier `SetDisplayName` / `SetDescription` / `SetTags` `CoreCommand` variants.

### Batch semantics

Carry forward 3.25's pattern: during patch load, individual `LabelChanged` events fire as each labeled entity is processed, *before* the final `PatchLoaded` summary. So the patch-load emission order becomes:

1. For each module: `ModuleAdded` → (per labeled field: `LabelChanged`) → (per initial parameter: `ParameterChanged`)
2. For each cable: `CableConnected` → (per labeled field: `LabelChanged`)
3. For non-empty `RackMeta` fields: `LabelChanged` with `entity: None`
4. `PatchLoaded` (extended with one new field, see below)

The `PatchLoaded` summary count gets an extra field:

```rust
pub struct PatchLoaded {
    pub patch_name: String,
    pub source: PatchSource,
    pub module_count: usize,
    pub cable_count: usize,
    pub label_count: usize,           // NEW in 3.5
}
```

`label_count` is the count of `LabelChanged` events emitted during this load. Lets consumers do quick "how rich is this patch's documentation" queries without scanning entities.

---

## Test discipline

Carries forward from 3.25 verbatim. Every emission site has a test; every test uses `matches!` with field checks; no `.to_string().contains(...)` patterns; cross-type ordering accepted as structural-guarantee (same caveat as 3.25's test).

Specific tests required (final list lives in the CC spec):

- `test_display_name_component_basic` — insert component, read back, assert.
- `test_tags_component_basic` — same with tag set.
- `test_display_name_formatter_default_acronyms` — assert `"vco_1"` → `"VCO 1"`, `"lead_voice_vco"` → `"Lead Voice VCO"`, etc.
- `test_display_name_formatter_extends` — extend the acronym table at runtime, verify new entry honored.
- `test_display_name_fallback_uses_formatter_when_component_absent` — query an unlabeled module, assert formatted instance_name returned.
- `test_display_name_fallback_uses_component_when_present` — set component, query, assert component value returned.
- `test_patch_load_emits_label_changed` — load patch with module + display_name field; assert `LabelChanged` emitted with `LabelKind::DisplayName`, `source: PatchLoad`.
- `test_rack_meta_loaded_from_patch` — load patch with `[rack]` section; assert `RackMeta` resource populated; assert `LabelChanged` events for non-empty fields.
- `test_rack_meta_default_when_section_absent` — load patch without `[rack]` section; assert `RackMeta::default()`.
- `test_patch_load_label_count_matches` — load patch with N labels; assert `PatchLoaded.label_count == N`.
- `test_legacy_patch_files_load_unchanged` — load a patch file without any 3.5 fields; assert it works.

---

## Consistency follow-up for `ParameterChanged`

3.5 introduces `LabelChanged.previous`. 3.25 shipped with `ParameterChanged.old` (always `None` today, with `TODO(bucket-getter)`). The two notifications carry semantically-identical fields under different names. The inconsistency will be visible to every consumer that reads both events.

**Proposal: rename `ParameterChanged.old` → `ParameterChanged.previous` as part of bucket 3.5.**

The rename is cheap to do now:

- The field is currently always `None`, so no consumer has come to depend on its value.
- Affected sites: two emission sites (`event.rs:dispatch_set_parameter`, `patch.rs:apply_patch_to_world`), seven tests that `matches!` on `old: None`, the field's doc-comment, the `TODO(bucket-getter)` reference, the design memo's post-3.25 follow-up section.
- All sites are mechanical updates (`old:` → `previous:`).

Cost of *not* renaming: every future consumer of both events sees the naming inconsistency and has to remember which is which. Cost of renaming: ~10 lines of mechanical change. Strongly favors the rename.

If you sign off on this, CC's 3.5 bucket includes the rename as a small auxiliary deliverable: rename the field, update emission sites and tests, update the field's doc-comment to match the new convention, update the design memo's post-3.25 follow-up section.

---

## Locked decisions (recap)

(See table at top.)

D1–D10 are locked, modulo your sign-off. Of those, D5 (formatter as resource), D10 (no port labels in v0), and the acronym table contents are the most opinionated — call out if you disagree.

---

## Open items — resolved 2026-05-10

All six open items decided. Decisions baked into the body of the memo above; this section is the audit trail.

**O1 — Acronym table: accept the proposed synth-domain default.** Table is runtime-extensible via `DisplayNameFormatter.acronyms`, so wrong calls in the starter set aren't permanent. **Locked.**

**O2 — `Tag` validates at construction, with a private inner field.** Per Rust guidelines TD-03 ("Newtypes for Semantic Distinctions and Invariants", strength = **SHOULD**), the canonical newtype pattern is private inner field + `Tag::new(s) -> Result<Self, TagError>` constructor + accessor methods. Bare `pub struct Tag(pub String)` is the anti-pattern — the public field defeats the newtype's invariant enforcement. **Locked: private inner field, `Tag::new` constructor returning `Result`, `Tag::as_str` accessor.** v0 validation rules: non-empty, no leading/trailing whitespace. (Length limits, internal-whitespace rules, normalization deferred until a use case demonstrates need.)

**O3 — `LabelChanged` carries `previous: Option<LabelValue>`, populated from pre-mutation component read.** Naming note: `previous`, not `old`. *Previous* is semantically precise — it's the value that came before; *old* carries connotations of deprecation, staleness, or obsolescence that don't fit. **Locked.**

**O4 — Notes mutations: single event with full new list at patch-load.** When runtime note-CRUD APIs land in a future bucket, that bucket refines the event shape (possibly to per-note CRUD events with note IDs). For v0 the single-event-with-full-list shape is sufficient. **Locked.**

**O5 — Tags collection: `BTreeSet<Tag>`.** Deterministic serialization order beats insertion-order preservation for patch-file diff readability. **Locked.**

**O6 — Component placement: new `oxurack-core::label` module** (`crates/oxurack-core/src/label.rs`). Domain-generic types deserve a dedicated home. `lib.rs` re-exports at crate root. **Locked.**

---

## Deferred to later buckets

| Item | Bucket / when | Reason |
|---|---|---|
| Runtime label-mutation `CoreCommand` variants (`SetDisplayName`, `SetDescription`, etc.) and dispatch | Future bucket (probably 3.6 or later) | 3.5 establishes the substrate (components, formatter, patch format). Mutation commands are a separate scope. |
| Port-level instance labels | Future, if a use case demonstrates need | Schema-level port labels exist; per-instance overrides are speculative for v0. |
| Tag validation (whitespace, length, normalization) | Future, if drift becomes real | Newtype provides the seam. |
| Hierarchical tags (`audio/lead/voice`) | Future | Flat tags are simpler and sufficient for v0. |
| Note IDs and per-note CRUD events | Future bucket that adds runtime note APIs | Patch-load emits notes-as-whole; runtime mutation can refine. |
| Search/filter machinery driven by tags | Future bucket (REPL? TUI?) | 3.5 stores tags; consumers come later. |
| `LabelChanged.old: Option<LabelValue>` if O3 = B | Whichever bucket needs it | Symmetric with `ParameterChanged.old` deferral. |
| User config for acronym-table persistence across sessions | Future bucket (config system) | 3.5 supports runtime modification of the formatter resource; persistence is a separate concern. |

---

## Migration notes

1. **`PatchLoaded` extension.** Add `label_count: usize` field. Update existing construction sites (the post-3.25 site at `patch.rs:576` and any test fixtures). Existing assertions on `PatchLoaded` continue to work via `..` patterns or get extended explicitly.

2. **`PatchSource` reuse.** No changes to `PatchSource`. `LabelChanged` reuses `ChangeSource` from 3.25 verbatim.

3. **Component derivation.** The three new components derive `Component, Clone, Debug, PartialEq, Eq`. `PartialEq + Eq` enables Bevy change detection (`Changed<DisplayName>` queries become available — useful for future reactor systems if we decide to relax the A-everywhere rule for label changes).

4. **No new feature flags.** All 3.5 functionality is core; the literate-patching layer is not optional.

5. **Patch-file backward compatibility.** All new fields optional. A pre-3.5 patch file (no `display_name`, no `tags`, no `[rack]` section) loads with the new code without error. Test `test_legacy_patch_files_load_unchanged` is non-negotiable.

---

## Appendix A — Relationship to bucket 4 (graph)

The graph (bucket 4) consumes the notification tier — `ModuleAdded`, `CableConnected`, `ParameterChanged`, and now `LabelChanged`. The graph stores entity refs only, not labels. When the graph's consumer (TUI renderer, REPL printer) wants to display a node, it queries the ECS world for the entity's current label state — passing through the fallback chain (component → formatter → instance name) to derive a display string.

This keeps the graph free of label-tracking and preserves the orthogonality argument:

- Notifications carry facts about changes.
- Components carry current state.
- The graph indexes structure.
- Renderers and consumers combine them at query time.

If you skip `LabelChanged` in the graph subscription, the graph still works — it just doesn't know when labels change. Renderers that re-query labels on every render naturally pick up the latest values. The label-changed events are useful for *cache invalidation* in renderers that memoize label strings, but they're not load-bearing for graph correctness.

---

## Appendix B — Why "literate patching"?

The phrase is deliberate. Donald Knuth's *literate programming* posits that a program's prose is at least as important as its code — the program is meant to be read by humans, not just executed by machines. The implementation tools (TeX, WEB, CWEB) treat the documentary and the executable as co-equal artifacts.

Modular synth patches are similar in shape: they're documents-of-intent that happen to be executable. A patch file with no annotation is like a programming source file with no comments — runnable, but opaque to its own author six months later.

3.5's deliverable is the substrate that lets patch authors *write* literately. Bucket 4's deliverable (the graph) is the substrate that lets readers *read* literately — the graph plus labels gives a renderable, navigable document. Together they constitute the literate-patching feature; 3.5 is the storage and serialization layer.

---

## Suggested next step

If this memo lands, the CC-facing implementation spec follows our established six-element structure (work-mode block, citations, hunt list, anti-softpedal, did-not-find section, no pre-listed decisions). The DoD will be substantial — three new components, one new resource, the formatter and acronym table, the `LabelChanged` event and `LabelKind`/`LabelValue` enums, the patch-file format extension, the `PatchLoaded.label_count` extension, and the test suite — but each piece is small individually.

Open items O1–O6 want quick decisions before the spec is written. None are blockers; all have my lean stated.
