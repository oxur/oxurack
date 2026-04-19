//! Module identity types.
//!
//! Every module in the rack has a [`ModuleKind`] (e.g. `"vco"`,
//! `"filter"`, `"lfo"`) that names its class, and a [`ModuleId`] that
//! uniquely identifies a specific instance within the patch.

use std::fmt;

use bevy_ecs::prelude::{Component, Entity};
use bevy_ecs::world::World;
use bevy_reflect::Reflect;

/// The class name of a module (e.g. `"vco"`, `"adsr"`, `"mixer"`).
///
/// This is a string newtype that identifies what *kind* of module
/// something is, as opposed to which *instance* it is.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
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
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Reflect)]
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
#[derive(Component, Debug, Clone, Reflect)]
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
        assert_eq!(ids, vec![ModuleId(1), ModuleId(2), ModuleId(3), ModuleId(5)]);
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
}
