//! Module parameter descriptors, values, and the parameter registry.
//!
//! The parameter system provides a uniform way to get and set
//! module-specific parameters at runtime. Each parameter has a
//! [`ParameterName`], a dynamic [`ParameterValue`], and an optional
//! [`ParameterSchema`] for introspection.
//!
//! The [`ParameterRegistry`] maps `(ModuleKind, ParameterName)` pairs
//! to setter functions that know how to apply a value to a module
//! entity in the ECS world.

use std::collections::HashMap;
use std::fmt;

use bevy_ecs::prelude::{Entity, Resource, World};
use bevy_reflect::Reflect;

/// Name of a module parameter (e.g. `"write_probability"`, `"scale"`).
///
/// A lightweight string newtype used as the key in parameter lookups.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParameterName(String);

impl From<&str> for ParameterName {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for ParameterName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for ParameterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for ParameterName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Dynamic parameter value.
///
/// Each variant corresponds to one of the supported parameter types.
/// The `Scale` variant carries a full [`Scale`](crate::Scale) value,
/// enabling scale parameters to be set via the same registry mechanism
/// as simple numeric or boolean parameters.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum ParameterValue {
    /// A floating-point parameter.
    Float(f32),
    /// An integer parameter.
    Int(i64),
    /// A boolean parameter.
    Bool(bool),
    /// A string parameter.
    String(String),
    /// A musical scale parameter.
    Scale(crate::Scale),
}

/// Type-level description of a parameter for introspection.
///
/// Used by [`OxurackModule::parameter_schema`](crate::OxurackModule)
/// to declare the parameters a module exposes at registration time.
#[derive(Debug, Clone)]
pub struct ParameterSchema {
    /// Machine-readable parameter name.
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Default value for this parameter.
    pub default: ParameterValue,
}

/// Setter function signature for applying parameter values to module
/// entities.
///
/// The function receives a mutable reference to the ECS `World`, the
/// entity of the module being configured, and the new value. It returns
/// `Ok(())` on success or a [`CoreError`](crate::CoreError) if the
/// value is rejected.
pub type ParameterSetter = fn(&mut World, Entity, ParameterValue) -> Result<(), crate::CoreError>;

/// Registry mapping `(ModuleKind, ParameterName)` to setter functions.
///
/// Inserted as a Bevy [`Resource`] by [`CorePlugin`](crate::CorePlugin).
/// Module plugins register their setters during app build.
#[derive(Resource, Default)]
pub struct ParameterRegistry {
    setters: HashMap<(crate::ModuleKind, ParameterName), ParameterSetter>,
}

impl ParameterRegistry {
    /// Register a setter for a `(module_kind, param_name)` pair.
    pub fn register(
        &mut self,
        module_kind: crate::ModuleKind,
        param_name: impl Into<ParameterName>,
        setter: ParameterSetter,
    ) {
        self.setters
            .insert((module_kind, param_name.into()), setter);
    }

    /// Look up and invoke the setter for the given module and parameter.
    ///
    /// Returns [`CoreError::UnknownParameter`](crate::CoreError::UnknownParameter)
    /// if no setter is registered for the `(module_kind, param_name)` pair.
    pub fn set_parameter(
        &self,
        world: &mut World,
        module_entity: Entity,
        module_kind: &crate::ModuleKind,
        param_name: &str,
        value: ParameterValue,
    ) -> Result<(), crate::CoreError> {
        let key = (module_kind.clone(), ParameterName::from(param_name));
        let setter = self.setters.get(&key).ok_or_else(|| {
            crate::CoreError::UnknownParameter {
                module: module_kind.to_string(),
                param: param_name.to_string(),
            }
        })?;
        setter(world, module_entity, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::Component;
    use bevy_ecs::world::World;
    use pretty_assertions::assert_eq;

    // ── ParameterName ───────────────────────────────────────────

    #[test]
    fn test_parameter_name_from_str() {
        let name = ParameterName::from("cutoff");
        assert_eq!(name.as_ref(), "cutoff");
    }

    #[test]
    fn test_parameter_name_from_string() {
        let name = ParameterName::from(String::from("resonance"));
        assert_eq!(name.as_ref(), "resonance");
    }

    #[test]
    fn test_parameter_name_display() {
        let name = ParameterName::from("detune");
        assert_eq!(format!("{name}"), "detune");
    }

    #[test]
    fn test_parameter_name_equality() {
        let a = ParameterName::from("freq");
        let b = ParameterName::from("freq");
        let c = ParameterName::from("amp");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_parameter_name_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ParameterName::from("a"));
        set.insert(ParameterName::from("b"));
        set.insert(ParameterName::from("a")); // duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_parameter_name_debug() {
        let name = ParameterName::from("test");
        let debug = format!("{name:?}");
        assert!(
            debug.contains("test"),
            "expected 'test' in debug: {debug}"
        );
    }

    // ── ParameterValue ──────────────────────────────────────────

    #[test]
    fn test_parameter_value_float() {
        let v = ParameterValue::Float(0.5);
        assert_eq!(v, ParameterValue::Float(0.5));
        let debug = format!("{v:?}");
        assert!(debug.contains("Float"), "expected 'Float' in: {debug}");
    }

    #[test]
    fn test_parameter_value_int() {
        let v = ParameterValue::Int(42);
        assert_eq!(v, ParameterValue::Int(42));
        let debug = format!("{v:?}");
        assert!(debug.contains("Int"), "expected 'Int' in: {debug}");
    }

    #[test]
    fn test_parameter_value_bool() {
        let v = ParameterValue::Bool(true);
        assert_eq!(v, ParameterValue::Bool(true));
        let debug = format!("{v:?}");
        assert!(debug.contains("Bool"), "expected 'Bool' in: {debug}");
    }

    #[test]
    fn test_parameter_value_string() {
        let v = ParameterValue::String("hello".into());
        assert_eq!(v, ParameterValue::String("hello".into()));
        let debug = format!("{v:?}");
        assert!(debug.contains("String"), "expected 'String' in: {debug}");
    }

    #[test]
    fn test_parameter_value_scale() {
        let scale = crate::Scale::major(0);
        let v = ParameterValue::Scale(scale.clone());
        assert_eq!(v, ParameterValue::Scale(scale));
        let debug = format!("{v:?}");
        assert!(debug.contains("Scale"), "expected 'Scale' in: {debug}");
    }

    #[test]
    fn test_parameter_value_clone() {
        let v = ParameterValue::Float(1.0);
        let cloned = v.clone();
        assert_eq!(v, cloned);
    }

    // ── ParameterSchema ─────────────────────────────────────────

    #[test]
    fn test_parameter_schema_debug() {
        let schema = ParameterSchema {
            name: "cutoff",
            description: "Filter cutoff frequency",
            default: ParameterValue::Float(0.5),
        };
        let debug = format!("{schema:?}");
        assert!(
            debug.contains("cutoff"),
            "expected 'cutoff' in: {debug}"
        );
    }

    #[test]
    fn test_parameter_schema_clone() {
        let schema = ParameterSchema {
            name: "resonance",
            description: "Filter resonance",
            default: ParameterValue::Float(0.0),
        };
        let cloned = schema.clone();
        assert_eq!(cloned.name, "resonance");
        assert_eq!(cloned.default, ParameterValue::Float(0.0));
    }

    // ── ParameterRegistry ───────────────────────────────────────

    /// A test component that a setter can modify.
    #[derive(Component, Debug, Clone, PartialEq)]
    struct TestParam {
        value: f32,
    }

    fn test_setter(
        world: &mut World,
        entity: Entity,
        value: ParameterValue,
    ) -> Result<(), crate::CoreError> {
        match value {
            ParameterValue::Float(f) => {
                world
                    .entity_mut(entity)
                    .get_mut::<TestParam>()
                    .expect("TestParam component should exist")
                    .value = f;
                Ok(())
            }
            _ => Err(crate::CoreError::InvalidParameterValue {
                module: "test".into(),
                param: "gain".into(),
                reason: "expected Float".into(),
            }),
        }
    }

    #[test]
    fn test_registry_register_and_set() {
        let mut world = World::new();
        let entity = world.spawn(TestParam { value: 0.0 }).id();

        let mut registry = ParameterRegistry::default();
        let kind = crate::ModuleKind::from("test_module");
        registry.register(kind.clone(), "gain", test_setter);

        registry
            .set_parameter(
                &mut world,
                entity,
                &kind,
                "gain",
                ParameterValue::Float(0.75),
            )
            .expect("set_parameter should succeed");

        let param = world.entity(entity).get::<TestParam>().unwrap();
        assert_eq!(param.value, 0.75);
    }

    #[test]
    fn test_registry_unknown_parameter() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();

        let registry = ParameterRegistry::default();
        let kind = crate::ModuleKind::from("test_module");

        let result = registry.set_parameter(
            &mut world,
            entity,
            &kind,
            "nonexistent",
            ParameterValue::Float(1.0),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = format!("{err}");
        assert!(
            msg.contains("nonexistent"),
            "expected 'nonexistent' in: {msg}"
        );
        assert!(
            msg.contains("test_module"),
            "expected 'test_module' in: {msg}"
        );
    }

    #[test]
    fn test_registry_setter_can_reject_value() {
        let mut world = World::new();
        let entity = world.spawn(TestParam { value: 0.0 }).id();

        let mut registry = ParameterRegistry::default();
        let kind = crate::ModuleKind::from("test_module");
        registry.register(kind.clone(), "gain", test_setter);

        // Pass wrong type: Bool instead of Float.
        let result = registry.set_parameter(
            &mut world,
            entity,
            &kind,
            "gain",
            ParameterValue::Bool(true),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::CoreError::InvalidParameterValue { .. }),
            "expected InvalidParameterValue, got: {err:?}"
        );
    }
}
