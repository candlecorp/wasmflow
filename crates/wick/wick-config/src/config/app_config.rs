use std::collections::HashMap;
pub(super) mod resources;
pub(super) mod triggers;

pub use self::resources::*;
pub use self::triggers::*;
use super::common::component_definition::{BoundComponent, ComponentDefinition};
use super::common::host_definition::HostConfig;
use crate::error::ReferenceError;
use crate::{v1, Result};

#[derive(Debug, Clone)]
#[must_use]
/// The internal representation of a Wick manifest.
pub struct AppConfiguration {
  pub name: String,
  pub(crate) source: Option<String>,
  pub(crate) version: String,
  pub(crate) import: HashMap<String, BoundComponent>,
  pub(crate) resources: HashMap<String, BoundResource>,
  pub(crate) triggers: Vec<TriggerDefinition>,
  pub(crate) host: HostConfig,
}

impl Default for AppConfiguration {
  fn default() -> Self {
    Self {
      name: "".to_owned(),
      source: None,
      version: "0.0.1".to_owned(),
      host: HostConfig::default(),
      import: HashMap::new(),
      resources: HashMap::new(),
      triggers: vec![],
    }
  }
}

impl AppConfiguration {
  /// Get the configuration item a binding points to.
  #[must_use]
  pub fn resolve_binding(&self, name: &str) -> Option<ConfigurationItem> {
    if let Some(component) = self.import.get(name) {
      return Some(ConfigurationItem::Component(&component.kind));
    }
    None
  }

  /// Return the underlying version of the source manifest.
  #[must_use]
  pub fn version(&self) -> &str {
    &self.version
  }

  /// Return the underlying version of the source manifest.
  #[must_use]
  pub fn source(&self) -> &Option<String> {
    &self.source
  }

  #[must_use]
  /// Get the name for this manifest.
  pub fn name(&self) -> &str {
    &self.name
  }

  #[must_use]
  /// Get the application's imports.
  pub fn imports(&self) -> &HashMap<String, BoundComponent> {
    &self.import
  }

  #[must_use]
  /// Get the application's resources.
  pub fn resources(&self) -> &HashMap<String, BoundResource> {
    &self.resources
  }

  #[must_use]
  /// Get the application's triggers.
  pub fn triggers(&self) -> &Vec<TriggerDefinition> {
    &self.triggers
  }

  pub fn into_v1_yaml(self) -> Result<String> {
    let v1_manifest: v1::AppConfiguration = self.try_into()?;
    Ok(serde_yaml::to_string(&v1_manifest).unwrap())
  }
}

/// A configuration item
#[derive(Debug, Clone, PartialEq)]
#[must_use]
pub enum ConfigurationItem<'a> {
  /// A component definition.
  Component(&'a ComponentDefinition),
  /// A resource definition.
  Resource(&'a ResourceDefinition),
}

impl<'a> ConfigurationItem<'a> {
  /// Get the component definition or return an error.
  pub fn component(&self) -> std::result::Result<&'a ComponentDefinition, ReferenceError> {
    match self {
      ConfigurationItem::Component(c) => Ok(c),
      _ => Err(ReferenceError::Component),
    }
  }
}