//! Module identity types and the module registry.
//!
//! Every module in the rack has a [`ModuleKind`] (e.g. `"vco"`,
//! `"filter"`, `"lfo"`) that names its class, and a [`ModuleId`] that
//! uniquely identifies a specific instance within the patch.
//!
//! The [`OxurackModule`] trait is implemented by every concrete module
//! type and provides static metadata (kind, ports, parameters). The
//! [`ModuleRegistry`] collects these registrations so that the patch
//! loader can instantiate modules by name.

use std::collections::HashMap;
use std::fmt;

use bevy_ecs::prelude::{Component, Entity, Resource};
use bevy_ecs::world::World;

/// The class name of a module (e.g. `"vco"`, `"adsr"`, `"mixer"`).
///
/// This is a string newtype that identifies what *kind* of module
/// something is, as opposed to which *instance* it is.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleKind(String);

impl From<&str> for ModuleKind {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for ModuleKind {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for ModuleKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for ModuleKind {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Unique identifier for a module instance within a patch.
///
/// Module IDs are cheap to copy and compare. They are ordered so they
/// can be used as sort keys for deterministic processing order.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleId(pub u64);

impl ModuleId {
    /// Derive a deterministic [`ModuleId`] from an instance name.
    ///
    /// Uses a hash-based derivation so that the same instance name
    /// always produces the same ID.
    pub fn from_instance_name(instance_name: &str) -> Self {
        Self(crate::rng::derive_seed(0, instance_name))
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModuleId({})", self.0)
    }
}

// ── ECS component types ────────────────────────────────────────────

/// Identifies a module entity in the ECS world.
///
/// Every module entity carries a [`Module`] component (describing what
/// it is) and a [`ModuleId`] component (providing a deterministic
/// identity).
#[derive(Component, Debug, Clone)]
pub struct Module {
    /// The class of module (e.g. `"vco"`, `"filter"`).
    pub kind: ModuleKind,
    /// The unique instance name within the patch.
    pub instance_name: String,
}

// ── Spawn helpers ──────────────────────────────────────────────────

/// Spawn a module entity with [`Module`] and [`ModuleId`] components.
///
/// The [`ModuleId`] is deterministically derived from `instance_name`.
pub fn spawn_module_entity(world: &mut World, kind: &str, instance_name: &str) -> Entity {
    let module_id = ModuleId::from_instance_name(instance_name);
    world
        .spawn((
            Module {
                kind: ModuleKind::from(kind),
                instance_name: instance_name.to_string(),
            },
            module_id,
        ))
        .id()
}

// ── Port schema ──────────────────────────────────────────────────

/// Static metadata describing a port that a module exposes.
///
/// Used at registration time to declare the ports a module type
/// provides, before any instances are spawned.
#[derive(Debug, Clone)]
pub struct PortSchema {
    /// Machine-readable port name.
    pub name: &'static str,
    /// Whether this port is an input or output.
    pub direction: crate::PortDirection,
    /// The kind of signal this port carries.
    pub value_kind: crate::ValueKind,
    /// How multiple incoming cables are merged (only relevant for inputs).
    pub merge_policy: crate::MergePolicy,
    /// Human-readable description.
    pub description: &'static str,
}

// ── OxurackModule trait ──────────────────────────────────────────

/// Trait implemented by all oxurack modules.
///
/// Provides static metadata that is collected by the
/// [`ModuleRegistry`] at app build time. Concrete module types
/// (e.g. `TuringMachine`, `Vco`, `Filter`) implement this trait
/// and register themselves via `ModuleRegistry::register::<M>()`.
///
/// The [`spawn`](OxurackModule::spawn) method instantiates a module
/// entity in the ECS world; the default implementation creates the
/// entity and its child port entities from the port schema.
pub trait OxurackModule: Send + Sync + 'static {
    /// The machine-readable kind name (e.g. `"turing_machine"`, `"vco"`).
    const KIND: &'static str;
    /// A human-readable display name (e.g. `"Turing Machine"`, `"VCO"`).
    const DISPLAY_NAME: &'static str;
    /// An optional description of what this module does.
    const DESCRIPTION: &'static str = "";

    /// Returns the static port schema for this module type.
    fn port_schema() -> &'static [PortSchema];

    /// Returns the static parameter schema for this module type.
    fn parameter_schema() -> &'static [crate::ParameterSchema];

    /// Instantiate this module into the ECS world.
    ///
    /// Creates the module entity with all required components, spawns
    /// child port entities from the port schema, and applies the given
    /// parameter overrides.
    ///
    /// Implementors should spawn a module entity via
    /// [`spawn_module_entity`] and create ports from
    /// [`port_schema`](OxurackModule::port_schema) via
    /// [`spawn_port_on_module`](crate::spawn_port_on_module).
    /// Concrete modules can add custom components after spawning.
    fn spawn(
        world: &mut World,
        instance_name: &str,
        parameters: &HashMap<String, crate::ParameterValue>,
    ) -> Result<Entity, crate::CoreError>;
}

// ── ModuleSpawner ────────────────────────────────────────────────

/// Type alias for a module spawner function pointer.
///
/// Used by [`ModuleRegistration`] to store a type-erased version
/// of [`OxurackModule::spawn`] that can be called dynamically by
/// the patch loader.
pub type ModuleSpawner = fn(
    &mut World,
    &str,
    &HashMap<String, crate::ParameterValue>,
) -> Result<Entity, crate::CoreError>;

// ── ModuleRegistration ───────────────────────────────────────────

/// Registration entry for a module kind in the [`ModuleRegistry`].
///
/// Stores a snapshot of the static metadata from an
/// [`OxurackModule`] implementation, including a spawner function
/// pointer that can create module entities in the ECS world.
pub struct ModuleRegistration {
    /// The module kind.
    pub kind: ModuleKind,
    /// Human-readable display name.
    pub display_name: String,
    /// Description of the module.
    pub description: String,
    /// Declared port schemas.
    pub port_schemas: Vec<PortSchema>,
    /// Declared parameter schemas.
    pub parameter_schemas: Vec<crate::ParameterSchema>,
    /// Function pointer to spawn an instance of this module.
    pub spawner: ModuleSpawner,
}

impl Clone for ModuleRegistration {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            display_name: self.display_name.clone(),
            description: self.description.clone(),
            port_schemas: self.port_schemas.clone(),
            parameter_schemas: self.parameter_schemas.clone(),
            spawner: self.spawner,
        }
    }
}

impl fmt::Debug for ModuleRegistration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModuleRegistration")
            .field("kind", &self.kind)
            .field("display_name", &self.display_name)
            .field("description", &self.description)
            .field("port_schemas", &self.port_schemas)
            .field("parameter_schemas", &self.parameter_schemas)
            .field("spawner", &"<fn>")
            .finish()
    }
}

// ── ModuleRegistry ───────────────────────────────────────────────

/// Registry of available module types.
///
/// Module plugins call [`register`](ModuleRegistry::register) during
/// app build. The patch loader uses the registry to look up module
/// metadata by [`ModuleKind`].
#[derive(Resource, Default, Debug)]
pub struct ModuleRegistry {
    registrations: HashMap<ModuleKind, ModuleRegistration>,
}

impl ModuleRegistry {
    /// Register a module type in the registry.
    ///
    /// Collects the static metadata from the [`OxurackModule`]
    /// implementation and stores it keyed by [`ModuleKind`].
    /// Also captures the module's [`spawn`](OxurackModule::spawn)
    /// function pointer so the patch loader can instantiate modules
    /// dynamically by kind.
    pub fn register<M: OxurackModule>(&mut self) {
        let kind = ModuleKind::from(M::KIND);
        let reg = ModuleRegistration {
            kind: kind.clone(),
            display_name: M::DISPLAY_NAME.to_string(),
            description: M::DESCRIPTION.to_string(),
            port_schemas: M::port_schema().to_vec(),
            parameter_schemas: M::parameter_schema().to_vec(),
            spawner: M::spawn,
        };
        self.registrations.insert(kind, reg);
    }

    /// Look up a module registration by kind.
    #[must_use]
    pub fn get(&self, kind: &ModuleKind) -> Option<&ModuleRegistration> {
        self.registrations.get(kind)
    }

    /// Returns `true` if the given kind is registered.
    #[must_use]
    pub fn contains(&self, kind: &ModuleKind) -> bool {
        self.registrations.contains_key(kind)
    }

    /// Returns an iterator over all registered module kinds.
    pub fn kinds(&self) -> impl Iterator<Item = &ModuleKind> {
        self.registrations.keys()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    // ── ModuleKind ──────────────────────────────────────────────────

    #[test]
    fn test_module_kind_from_str() {
        let kind = ModuleKind::from("vco");
        assert_eq!(kind.as_ref(), "vco");
    }

    #[test]
    fn test_module_kind_from_string() {
        let kind = ModuleKind::from(String::from("filter"));
        assert_eq!(kind.as_ref(), "filter");
    }

    #[test]
    fn test_module_kind_display() {
        let kind = ModuleKind::from("lfo");
        assert_eq!(format!("{kind}"), "lfo");
    }

    #[test]
    fn test_module_kind_equality() {
        let a = ModuleKind::from("vco");
        let b = ModuleKind::from("vco");
        let c = ModuleKind::from("lfo");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_module_kind_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ModuleKind::from("vco"));
        set.insert(ModuleKind::from("lfo"));
        set.insert(ModuleKind::from("vco")); // duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_module_kind_debug() {
        let kind = ModuleKind::from("adsr");
        let debug = format!("{kind:?}");
        assert!(
            debug.contains("adsr"),
            "expected 'adsr' in debug output, got: {debug}"
        );
    }

    #[test]
    fn test_module_kind_clone() {
        let kind = ModuleKind::from("mixer");
        let cloned = kind.clone();
        assert_eq!(kind, cloned);
    }

    // ── ModuleId ────────────────────────────────────────────────────

    #[test]
    fn test_module_id_equality() {
        assert_eq!(ModuleId(1), ModuleId(1));
        assert_ne!(ModuleId(1), ModuleId(2));
    }

    #[test]
    fn test_module_id_ordering() {
        assert!(ModuleId(1) < ModuleId(2));
        assert!(ModuleId(100) > ModuleId(50));
        assert!(ModuleId(0) <= ModuleId(0));
    }

    #[test]
    fn test_module_id_sorting() {
        let mut ids = vec![ModuleId(5), ModuleId(1), ModuleId(3), ModuleId(2)];
        ids.sort();
        assert_eq!(
            ids,
            vec![ModuleId(1), ModuleId(2), ModuleId(3), ModuleId(5)]
        );
    }

    #[test]
    fn test_module_id_display() {
        assert_eq!(format!("{}", ModuleId(42)), "ModuleId(42)");
    }

    #[test]
    fn test_module_id_debug() {
        let debug = format!("{:?}", ModuleId(7));
        assert!(
            debug.contains("7"),
            "expected '7' in debug output, got: {debug}"
        );
    }

    #[test]
    fn test_module_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ModuleId(1));
        set.insert(ModuleId(2));
        set.insert(ModuleId(1)); // duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_module_id_copy() {
        let id = ModuleId(99);
        let copied = id; // Copy, not move
        assert_eq!(id, copied);
    }

    // ── ModuleId::from_instance_name ───────────────────────────────

    #[test]
    fn test_module_id_from_instance_name_deterministic() {
        let a = ModuleId::from_instance_name("vco_1");
        let b = ModuleId::from_instance_name("vco_1");
        assert_eq!(a, b);
    }

    #[test]
    fn test_module_id_from_instance_name_differs_for_different_names() {
        let a = ModuleId::from_instance_name("vco_1");
        let b = ModuleId::from_instance_name("vco_2");
        assert_ne!(a, b);
    }

    // ── Module component tests ─────────────────────────────────────

    #[test]
    fn test_module_component_roundtrip() {
        let mut world = World::new();

        let entity = world
            .spawn((
                Module {
                    kind: ModuleKind::from("vco"),
                    instance_name: "vco_1".into(),
                },
                ModuleId(42),
            ))
            .id();

        let module = world.entity(entity).get::<Module>().unwrap();
        assert_eq!(module.kind, ModuleKind::from("vco"));
        assert_eq!(module.instance_name, "vco_1");

        let id = world.entity(entity).get::<ModuleId>().unwrap();
        assert_eq!(*id, ModuleId(42));
    }

    // ── spawn_module_entity tests ──────────────────────────────────

    #[test]
    fn test_spawn_module_entity_creates_module_and_id() {
        let mut world = World::new();

        let entity = spawn_module_entity(&mut world, "filter", "lpf_1");

        let module = world.entity(entity).get::<Module>().unwrap();
        assert_eq!(module.kind, ModuleKind::from("filter"));
        assert_eq!(module.instance_name, "lpf_1");

        let id = world.entity(entity).get::<ModuleId>().unwrap();
        assert_eq!(*id, ModuleId::from_instance_name("lpf_1"));
    }

    #[test]
    fn test_spawn_module_entity_deterministic_id() {
        let mut world = World::new();

        let e1 = spawn_module_entity(&mut world, "vco", "osc_1");
        let e2 = spawn_module_entity(&mut world, "vco", "osc_1");

        let id1 = *world.entity(e1).get::<ModuleId>().unwrap();
        let id2 = *world.entity(e2).get::<ModuleId>().unwrap();
        assert_eq!(id1, id2);
    }

    // ── PortSchema tests ──────────────────────────────────────────

    #[test]
    fn test_port_schema_debug() {
        let schema = PortSchema {
            name: "pitch",
            direction: crate::PortDirection::Input,
            value_kind: crate::ValueKind::Float,
            merge_policy: crate::MergePolicy::LastWins,
            description: "Pitch CV input",
        };
        let debug = format!("{schema:?}");
        assert!(debug.contains("pitch"), "expected 'pitch' in: {debug}");
    }

    #[test]
    fn test_port_schema_clone() {
        let schema = PortSchema {
            name: "out",
            direction: crate::PortDirection::Output,
            value_kind: crate::ValueKind::Bipolar,
            merge_policy: crate::MergePolicy::Reject,
            description: "Audio output",
        };
        let cloned = schema.clone();
        assert_eq!(cloned.name, "out");
        assert_eq!(cloned.direction, crate::PortDirection::Output);
    }

    // ── OxurackModule / ModuleRegistry tests ──────────────────────

    /// A dummy module for testing the registry.
    struct DummyVco;

    impl OxurackModule for DummyVco {
        const KIND: &'static str = "dummy_vco";
        const DISPLAY_NAME: &'static str = "Dummy VCO";
        const DESCRIPTION: &'static str = "A test oscillator module";

        fn port_schema() -> &'static [PortSchema] {
            &[
                PortSchema {
                    name: "pitch",
                    direction: crate::PortDirection::Input,
                    value_kind: crate::ValueKind::Float,
                    merge_policy: crate::MergePolicy::LastWins,
                    description: "Pitch CV input",
                },
                PortSchema {
                    name: "out",
                    direction: crate::PortDirection::Output,
                    value_kind: crate::ValueKind::Bipolar,
                    merge_policy: crate::MergePolicy::Reject,
                    description: "Audio output",
                },
            ]
        }

        fn parameter_schema() -> &'static [crate::ParameterSchema] {
            &[crate::ParameterSchema {
                name: "waveform",
                description: "Oscillator waveform",
                default: crate::ParameterValue::Int(0),
            }]
        }

        fn spawn(
            world: &mut bevy_ecs::world::World,
            instance_name: &str,
            _parameters: &std::collections::HashMap<String, crate::ParameterValue>,
        ) -> Result<bevy_ecs::prelude::Entity, crate::CoreError> {
            let module_entity = crate::spawn_module_entity(world, Self::KIND, instance_name);
            for schema in Self::port_schema() {
                crate::spawn_port_on_module(
                    world,
                    module_entity,
                    schema.name,
                    schema.direction,
                    schema.value_kind,
                    schema.merge_policy,
                );
            }
            Ok(module_entity)
        }
    }

    /// A second dummy module to verify multiple registrations.
    struct DummyFilter;

    impl OxurackModule for DummyFilter {
        const KIND: &'static str = "dummy_filter";
        const DISPLAY_NAME: &'static str = "Dummy Filter";

        fn port_schema() -> &'static [PortSchema] {
            &[]
        }

        fn parameter_schema() -> &'static [crate::ParameterSchema] {
            &[]
        }

        fn spawn(
            world: &mut bevy_ecs::world::World,
            instance_name: &str,
            _parameters: &std::collections::HashMap<String, crate::ParameterValue>,
        ) -> Result<bevy_ecs::prelude::Entity, crate::CoreError> {
            let module_entity = crate::spawn_module_entity(world, Self::KIND, instance_name);
            for schema in Self::port_schema() {
                crate::spawn_port_on_module(
                    world,
                    module_entity,
                    schema.name,
                    schema.direction,
                    schema.value_kind,
                    schema.merge_policy,
                );
            }
            Ok(module_entity)
        }
    }

    #[test]
    fn test_module_registry_register_and_get() {
        let mut registry = ModuleRegistry::default();
        registry.register::<DummyVco>();

        let kind = ModuleKind::from("dummy_vco");
        let reg = registry.get(&kind).expect("should be registered");
        assert_eq!(reg.display_name, "Dummy VCO");
        assert_eq!(reg.description, "A test oscillator module");
        assert_eq!(reg.port_schemas.len(), 2);
        assert_eq!(reg.parameter_schemas.len(), 1);
    }

    #[test]
    fn test_module_registry_contains() {
        let mut registry = ModuleRegistry::default();
        registry.register::<DummyVco>();

        assert!(registry.contains(&ModuleKind::from("dummy_vco")));
        assert!(!registry.contains(&ModuleKind::from("unknown")));
    }

    #[test]
    fn test_module_registry_kinds() {
        let mut registry = ModuleRegistry::default();
        registry.register::<DummyVco>();
        registry.register::<DummyFilter>();

        let mut kinds: Vec<String> = registry.kinds().map(|k| k.to_string()).collect();
        kinds.sort();
        assert_eq!(kinds, vec!["dummy_filter", "dummy_vco"]);
    }

    #[test]
    fn test_module_registry_get_unknown() {
        let registry = ModuleRegistry::default();
        assert!(registry.get(&ModuleKind::from("nope")).is_none());
    }

    #[test]
    fn test_module_registry_port_schema_accessible() {
        let mut registry = ModuleRegistry::default();
        registry.register::<DummyVco>();

        let kind = ModuleKind::from("dummy_vco");
        let reg = registry.get(&kind).unwrap();

        assert_eq!(reg.port_schemas[0].name, "pitch");
        assert_eq!(reg.port_schemas[0].direction, crate::PortDirection::Input);
        assert_eq!(reg.port_schemas[1].name, "out");
        assert_eq!(reg.port_schemas[1].direction, crate::PortDirection::Output);
    }

    #[test]
    fn test_module_registry_parameter_schema_accessible() {
        let mut registry = ModuleRegistry::default();
        registry.register::<DummyVco>();

        let kind = ModuleKind::from("dummy_vco");
        let reg = registry.get(&kind).unwrap();

        assert_eq!(reg.parameter_schemas[0].name, "waveform");
        assert_eq!(
            reg.parameter_schemas[0].default,
            crate::ParameterValue::Int(0)
        );
    }

    #[test]
    fn test_module_registry_default_description() {
        let mut registry = ModuleRegistry::default();
        registry.register::<DummyFilter>();

        let kind = ModuleKind::from("dummy_filter");
        let reg = registry.get(&kind).unwrap();
        // Default description is empty string.
        assert_eq!(reg.description, "");
    }

    #[test]
    fn test_module_registry_debug() {
        let mut registry = ModuleRegistry::default();
        registry.register::<DummyVco>();
        let debug = format!("{registry:?}");
        assert!(
            debug.contains("ModuleRegistry"),
            "expected 'ModuleRegistry' in: {debug}"
        );
    }

    #[test]
    fn test_module_registration_clone() {
        let mut registry = ModuleRegistry::default();
        registry.register::<DummyVco>();

        let kind = ModuleKind::from("dummy_vco");
        let reg = registry.get(&kind).unwrap();
        let cloned = reg.clone();
        assert_eq!(cloned.kind, kind);
        assert_eq!(cloned.display_name, reg.display_name);
    }
}
