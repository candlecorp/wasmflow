#![allow(missing_docs)] // delete when we move away from the `property` crate.
use std::borrow::Cow;
use std::collections::HashMap;

use wick_interface_types::OperationSignatures;

use super::{ComponentConfig, OperationConfig};
use crate::config::{self, Codec};

#[derive(
  Debug,
  Clone,
  derive_builder::Builder,
  PartialEq,
  derive_asset_container::AssetManager,
  property::Property,
  serde::Serialize,
)]
#[property(get(public), set(public), mut(public, suffix = "_mut"))]
#[asset(asset(config::AssetReference))]
#[builder(setter(into))]
#[must_use]
/// A component whose operations are WebSocket connections.
pub struct WebSocketClientComponentConfig {
  /// The URL of the WebSocket server.
  #[asset(skip)]
  pub(crate) resource: String,

  /// The configuration for the component.
  #[asset(skip)]
  #[builder(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub(crate) config: Vec<wick_interface_types::Field>,

  /// The headers to send with the request.
  #[asset(skip)]
  #[builder(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) headers: Option<HashMap<String, Vec<String>>>,

  /// A list of operations to expose on this component.
  #[asset(skip)]
  #[builder(default)]
  #[property(skip)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub(crate) operations: Vec<WebSocketClientOperationDefinition>,
}

impl WebSocketClientComponentConfig {}

impl OperationSignatures for WebSocketClientComponentConfig {
  fn operation_signatures(&self) -> Vec<wick_interface_types::OperationSignature> {
    self.operations.clone().into_iter().map(Into::into).collect()
  }
}

impl ComponentConfig for WebSocketClientComponentConfig {
  type Operation = WebSocketClientOperationDefinition;

  fn operations(&self) -> &[Self::Operation] {
    &self.operations
  }

  fn operations_mut(&mut self) -> &mut Vec<Self::Operation> {
    &mut self.operations
  }
}

impl OperationConfig for WebSocketClientOperationDefinition {
  fn name(&self) -> &str {
    &self.name
  }

  fn inputs(&self) -> Cow<Vec<wick_interface_types::Field>> {
    Cow::Borrowed(&self.inputs)
  }

  fn outputs(&self) -> Cow<Vec<wick_interface_types::Field>> {
    Cow::Owned(vec![wick_interface_types::Field::new(
      "message",
      wick_interface_types::Type::Object,
    )])
  }
}

impl From<WebSocketClientOperationDefinition> for wick_interface_types::OperationSignature {
  fn from(operation: WebSocketClientOperationDefinition) -> Self {
    Self {
      name: operation.name,
      config: operation.config,
      inputs: operation.inputs,
      outputs: vec![wick_interface_types::Field::new(
        "message",
        match operation.codec {
          Some(Codec::Json) => wick_interface_types::Type::Object,
          Some(Codec::Raw) => wick_interface_types::Type::Bytes,
          Some(Codec::FormData) => wick_interface_types::Type::Object,
          Some(Codec::Text) => wick_interface_types::Type::Object,
          None => wick_interface_types::Type::Object,
        },
      )],
    }
  }
}

#[derive(Debug, Clone, derive_builder::Builder, PartialEq, property::Property, serde::Serialize)]
#[property(get(public), set(private), mut(disable))]
#[builder(setter(into))]
#[must_use]
/// An operation whose implementation is a WebSocket message.
pub struct WebSocketClientOperationDefinition {
  /// The name of the operation.
  #[property(skip)]
  pub(crate) name: String,

  /// The configuration the operation needs.
  #[builder(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub(crate) config: Vec<wick_interface_types::Field>,

  /// Types of the inputs to the operation.
  #[property(skip)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub(crate) inputs: Vec<wick_interface_types::Field>,

  /// The codec to use when encoding/decoding data.
  #[builder(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) codec: Option<Codec>,

  /// Types of the outputs to the operation.
  #[property(skip)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub(crate) outputs: Vec<wick_interface_types::Field>,

  /// The message to send, processed as a structured JSON liquid template.
  #[builder(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) message: Option<liquid_json::LiquidJsonValue>,
}