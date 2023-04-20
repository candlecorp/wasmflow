use wick_asset_reference::AssetReference;

use crate::config::{ComponentDefinition, ComponentOperationExpression};

#[derive(Debug, Clone, PartialEq, derive_asset_container::AssetManager)]
#[asset(AssetReference)]

/// Normalized representation of a CLI trigger configuration.
pub struct CliConfig {
  pub(crate) operation: ComponentOperationExpression,
  pub(crate) app: Option<ComponentDefinition>,
}

impl CliConfig {
  /// Returns the component id for the CLI trigger.
  pub fn component(&self) -> &ComponentDefinition {
    &self.operation.component
  }

  /// Returns the operation name for the CLI trigger.
  #[must_use]
  pub fn operation(&self) -> &str {
    &self.operation.operation
  }

  /// Returns the app definition for the CLI trigger.
  #[must_use]
  pub fn app(&self) -> Option<&ComponentDefinition> {
    self.app.as_ref()
  }
}
