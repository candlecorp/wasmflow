use std::collections::HashMap;

use flow_expression_parser::ast::{self};
use wick_interface_types::Field;
use wick_packet::GenericConfig;

use crate::config::{self, ExecutionSettings};

#[derive(Debug, Default, Clone, derive_asset_container::AssetManager, Builder, property::Property)]
#[property(get(public), set(public), mut(public, suffix = "_mut"))]
#[builder(setter(into))]
#[asset(asset(crate::config::AssetReference))]
#[must_use]
/// The internal representation of a Wick manifest.
pub struct CompositeComponentImplementation {
  /// The configuration for the component.
  #[asset(skip)]
  #[builder(default)]
  pub(crate) config: Vec<Field>,

  /// The operations defined by the component.
  #[asset(skip)]
  #[builder(default)]
  pub(crate) operations: HashMap<String, FlowOperation>,
}

impl CompositeComponentImplementation {
  /// Get a [FlowOperation] by name.
  #[must_use]
  pub fn flow(&self, name: &str) -> Option<&FlowOperation> {
    self.operations.iter().find(|(n, _)| name == *n).map(|(_, v)| v)
  }

  /// Get the signature of the component as defined by the manifest.
  #[must_use]
  pub fn operation_signatures(&self) -> Vec<wick_interface_types::OperationSignature> {
    self.operations.values().cloned().map(Into::into).collect()
  }
}

impl From<FlowOperation> for wick_interface_types::OperationSignature {
  fn from(operation: FlowOperation) -> Self {
    Self {
      name: operation.name,
      inputs: operation.inputs,
      outputs: operation.outputs,
    }
  }
}

#[derive(Debug, Clone, Builder, Default, property::Property)]
#[property(get(public), set(private), mut(public, suffix = "_mut"))]
#[builder(setter(into))]
/// The SchematicDefinition struct is a normalized representation of a Wick [SchematicManifest].
/// It handles the job of translating manifest versions into a consistent data structure.
#[must_use]
pub struct FlowOperation {
  /// The name of the schematic.
  pub(crate) name: String,

  /// A list of the input types for the operation.
  #[builder(default)]
  pub(crate) inputs: Vec<Field>,

  /// A list of the input types for the operation.
  #[builder(default)]
  pub(crate) outputs: Vec<Field>,

  /// Any configuration required for the component to operate.
  #[builder(default)]
  pub(crate) config: Vec<Field>,

  /// A mapping of instance names to the components they refer to.
  #[builder(default)]
  pub(crate) instances: HashMap<String, InstanceReference>,

  /// A list of connections from and to ports on instances defined in the instance map.
  #[builder(default)]
  pub(crate) expressions: Vec<ast::FlowExpression>,

  /// Additional flows scoped to this operation.
  #[builder(default)]
  pub(crate) flows: Vec<FlowOperation>,
}

impl From<FlowOperation> for config::OperationSignature {
  fn from(value: FlowOperation) -> Self {
    Self {
      name: value.name,
      inputs: value.inputs,
      outputs: value.outputs,
      config: value.config,
    }
  }
}

#[derive(Debug, Clone, PartialEq, property::Property)]
#[property(get(public), set(private), mut(disable))]
/// A definition of a component used to reference a component registered under a collection.
/// Note: [InstanceReference] include embed the concept of a namespace so two identical.
/// components registered on different namespaces will not be equal.
pub struct InstanceReference {
  /// The operation's name.
  pub(crate) name: String,
  /// The id of the component.
  pub(crate) component_id: String,
  /// Data associated with the component instance.
  pub(crate) data: Option<GenericConfig>,
  /// Per-operation settings that override global execution settings.
  pub(crate) settings: Option<ExecutionSettings>,
}

impl InstanceReference {
  /// Returns the fully qualified ID for the component, i.e. namespace::name.
  #[must_use]
  pub fn id(&self) -> String {
    format!("{}::{}", self.component_id, self.name)
  }
}

impl std::fmt::Display for InstanceReference {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.id())
  }
}
