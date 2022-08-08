use std::collections::HashMap;

use thiserror::Error;

use crate::program::{ExtensionId, TemplateArg};

mod core;

/// Error option while using extensions.
#[derive(Error, Debug, PartialEq)]
pub enum SpecializationError {
    #[error("Count not find the requested extension")]
    UnsupportedLibCallName,
    #[error("Expected a different number of template arguments")]
    WrongNumberOfTemplateArgs,
    #[error("Provided template arg is unsupported")]
    UnsupportedTemplateArg,
}

/// Error option while using extensions.
#[derive(Error, Debug, PartialEq)]
pub enum ExtensionError {
    #[error("Count not specialize extension")]
    Specialization { extension_id: ExtensionId, error: SpecializationError },
}

/// Handles extensions usages.
pub struct Extensions {
    specializers: HashMap<ExtensionId, ExtensionBox>,
}
impl Default for Extensions {
    fn default() -> Self {
        Extensions { specializers: core::all_core_extensions() }
    }
}
impl Extensions {
    pub fn specialize(
        &self,
        extension_id: &ExtensionId,
        tmpl_args: &[TemplateArg],
    ) -> Result<ConcreteExtensionBox, ExtensionError> {
        self.specializers
            .get(extension_id)
            .ok_or_else(move || ExtensionError::Specialization {
                extension_id: extension_id.clone(),
                error: SpecializationError::UnsupportedLibCallName,
            })?
            .specialize(tmpl_args)
            .map_err(move |error| ExtensionError::Specialization {
                extension_id: extension_id.clone(),
                error,
            })
    }
}

/// Trait for implementing a specialization generator.
trait Extension {
    /// Creates the specialization with the template arguments.
    fn specialize(
        &self,
        tmpl_args: &[TemplateArg],
    ) -> Result<ConcreteExtensionBox, SpecializationError>;
}

pub trait ConcreteExtension {}

type ExtensionBox = Box<dyn Extension>;
type ConcreteExtensionBox = Box<dyn ConcreteExtension>;

#[cfg(test)]
mod test;
