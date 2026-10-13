use std::marker::PhantomData;
use std::sync::Arc;

use oxurack_core::{CoreError, OxurackModule, ParameterValue};

use crate::Oxurack;

#[derive(Clone)]
pub(super) struct Binding {
    lifetime: Arc<()>,
    instance: Arc<()>,
}

#[derive(Clone)]
struct Reference {
    module: String,
    member: String,
    binding: Option<Binding>,
}

/// An unbound module name for a current rack lookup.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleRef(String);

impl ModuleRef {
    pub fn new(module: impl Into<String>) -> Self {
        Self(module.into())
    }
    pub fn module(&self) -> &str {
        &self.0
    }
    pub fn port(&self, port: impl Into<String>) -> PortRef {
        PortRef(Reference {
            module: self.0.clone(),
            member: port.into(),
            binding: None,
        })
    }
    pub fn knob(&self, parameter: impl Into<String>) -> KnobRef {
        KnobRef(Reference {
            module: self.0.clone(),
            member: parameter.into(),
            binding: None,
        })
    }
}

/// An exact module and port name, optionally bound to one live instance.
#[derive(Clone)]
pub struct PortRef(Reference);

impl std::fmt::Debug for PortRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PortRef")
            .field("module", &self.0.module)
            .field("port", &self.0.member)
            .field("bound", &self.0.binding.is_some())
            .finish()
    }
}

impl PortRef {
    pub fn new(module: impl Into<String>, port: impl Into<String>) -> Self {
        ModuleRef::new(module).port(port)
    }
    pub fn from_dotted(value: &str) -> Result<Self, CoreError> {
        let Some((module, port)) = value.split_once('.') else {
            return Err(invalid_reference(value));
        };
        if module.is_empty() || port.is_empty() || port.contains('.') {
            return Err(invalid_reference(value));
        }
        Ok(Self::new(module, port))
    }
    pub fn module(&self) -> &str {
        &self.0.module
    }
    pub fn port(&self) -> &str {
        &self.0.member
    }
}

/// An exact module and parameter name, optionally bound to one live instance.
#[derive(Clone)]
pub struct KnobRef(Reference);

impl std::fmt::Debug for KnobRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KnobRef")
            .field("module", &self.0.module)
            .field("parameter", &self.0.member)
            .field("bound", &self.0.binding.is_some())
            .finish()
    }
}

impl KnobRef {
    pub fn new(module: impl Into<String>, parameter: impl Into<String>) -> Self {
        ModuleRef::new(module).knob(parameter)
    }
    pub fn from_dotted(value: &str) -> Result<Self, CoreError> {
        let Some((module, parameter)) = value.split_once('.') else {
            return Err(invalid_reference(value));
        };
        if module.is_empty() || parameter.is_empty() || parameter.contains('.') {
            return Err(invalid_reference(value));
        }
        Ok(Self::new(module, parameter))
    }
    pub fn module(&self) -> &str {
        &self.0.module
    }
    pub fn parameter(&self) -> &str {
        &self.0.member
    }
}

/// A typed handle bound to one exact rack lifetime and module instance.
pub struct Handle<M: OxurackModule> {
    name: String,
    binding: Binding,
    marker: PhantomData<fn(M) -> M>,
}

impl<M: OxurackModule> Clone for Handle<M> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            binding: self.binding.clone(),
            marker: PhantomData,
        }
    }
}
impl<M: OxurackModule> std::fmt::Debug for Handle<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl<M: OxurackModule> Handle<M> {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn parameter(&self, rack: &Oxurack, param: &str) -> Result<ParameterValue, CoreError> {
        rack.validate_bound(&self.name, &self.binding)?;
        rack.parameter(&self.name, param)
    }
    pub fn set_parameter(
        &self,
        rack: &mut Oxurack,
        param: &str,
        value: ParameterValue,
    ) -> Result<(), CoreError> {
        rack.validate_bound(&self.name, &self.binding)?;
        rack.set_parameter(&self.name, param, value)
    }
    pub fn port(&self, name: &str) -> PortRef {
        PortRef(Reference {
            module: self.name.clone(),
            member: name.to_owned(),
            binding: Some(self.binding.clone()),
        })
    }
    pub fn knob(&self, name: &str) -> KnobRef {
        KnobRef(Reference {
            module: self.name.clone(),
            member: name.to_owned(),
            binding: Some(self.binding.clone()),
        })
    }
}

impl Oxurack {
    pub fn module<M: OxurackModule>(&self, name: &str) -> Result<Handle<M>, CoreError> {
        let entity = self
            .app
            .world()
            .get_resource::<oxurack_core::PatchHandle>()
            .ok_or(CoreError::NoPatchLoaded)?
            .modules
            .get(name)
            .copied()
            .ok_or_else(|| CoreError::UnknownModule(name.to_owned()))?;
        let module = self
            .app
            .world()
            .get::<oxurack_core::Module>(entity)
            .ok_or_else(|| CoreError::UnknownModule(name.to_owned()))?;
        let actual_type = self
            .installed_modules
            .get(&module.kind)
            .map(|index| self.install_order[*index].type_id);
        if module.kind.as_ref() != M::KIND || actual_type != Some(std::any::TypeId::of::<M>()) {
            let actual = self.installed_modules.get(&module.kind).map_or_else(
                || module.kind.to_string(),
                |index| self.install_order[*index].type_name.to_owned(),
            );
            return Err(CoreError::ModuleTypeMismatch {
                module: name.to_owned(),
                expected: std::any::type_name::<M>().to_owned(),
                actual,
            });
        }
        let binding = Binding {
            lifetime: self.lifetime_token.clone(),
            instance: self.instance_tokens.get(name).cloned().ok_or_else(|| {
                CoreError::StaleHandle {
                    module: name.to_owned(),
                }
            })?,
        };
        Ok(Handle {
            name: name.to_owned(),
            binding,
            marker: PhantomData,
        })
    }

    pub(super) fn validate_bound(&self, name: &str, binding: &Binding) -> Result<(), CoreError> {
        let current = self.instance_tokens.get(name);
        if !Arc::ptr_eq(&binding.lifetime, &self.lifetime_token)
            || current.is_none_or(|token| !Arc::ptr_eq(&binding.instance, token))
        {
            return Err(CoreError::StaleHandle {
                module: name.to_owned(),
            });
        }
        let handle = self
            .app
            .world()
            .get_resource::<oxurack_core::PatchHandle>()
            .ok_or_else(|| CoreError::StaleHandle {
                module: name.to_owned(),
            })?;
        if !handle.modules.contains_key(name) {
            return Err(CoreError::StaleHandle {
                module: name.to_owned(),
            });
        }
        Ok(())
    }

    pub fn patch_ports(
        &mut self,
        source: &PortRef,
        target: &PortRef,
        transform: Option<oxurack_core::CableTransform>,
    ) -> Result<usize, CoreError> {
        self.validate_reference(&source.0)?;
        self.validate_reference(&target.0)?;
        self.patch(
            (&source.0.module, &source.0.member),
            (&target.0.module, &target.0.member),
            transform,
        )
    }
    pub fn unpatch_ports(
        &mut self,
        source: &PortRef,
        target: &PortRef,
        occurrence: usize,
    ) -> Result<(), CoreError> {
        self.validate_reference(&source.0)?;
        self.validate_reference(&target.0)?;
        self.unpatch(
            (&source.0.module, &source.0.member),
            (&target.0.module, &target.0.member),
            occurrence,
        )
    }
    pub fn knob(&self, reference: &KnobRef) -> Result<ParameterValue, CoreError> {
        self.validate_reference(&reference.0)?;
        self.parameter(&reference.0.module, &reference.0.member)
    }
    pub fn set_knob(
        &mut self,
        reference: &KnobRef,
        value: ParameterValue,
    ) -> Result<(), CoreError> {
        self.validate_reference(&reference.0)?;
        self.set_parameter(&reference.0.module, &reference.0.member, value)
    }
    fn validate_reference(&self, reference: &Reference) -> Result<(), CoreError> {
        if let Some(binding) = &reference.binding {
            self.validate_bound(&reference.module, binding)?;
        }
        Ok(())
    }
}

fn invalid_reference(value: &str) -> CoreError {
    CoreError::InvalidReference {
        value: value.to_owned(),
    }
}
