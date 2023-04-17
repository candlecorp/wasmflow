use std::collections::HashMap;

use wick_interface_types::TypeDefinition;

use crate::component_config::OperationSignature;

#[derive(Debug, Default, Clone, derive_asset_container::AssetManager)]
#[asset(crate::config::AssetReference)]
#[must_use]
/// The Wick representation of an interface.
pub struct InterfaceDefinition {
  /// Types used by the interface's operations.
  #[asset(skip)]
  pub(crate) types: Vec<TypeDefinition>,

  /// The operations the interface exposes.
  #[asset(skip)]
  pub(crate) operations: HashMap<String, OperationSignature>,
}

impl InterfaceDefinition {
  /// Returns the type definitions used by this interfaces's operations.
  pub fn types(&self) -> &[TypeDefinition] {
    &self.types
  }

  /// Returns the operations in this interface.
  #[must_use]
  pub fn operations(&self) -> &HashMap<String, OperationSignature> {
    &self.operations
  }
}
