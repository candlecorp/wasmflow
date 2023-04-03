use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub(super) mod component_component;
pub(super) mod core_collection;
pub(crate) mod internal_collection;
pub(super) mod schematic_component;

use serde_json::Value;
use wick_interface_types::ComponentSignature;
use wick_packet::{Invocation, PacketStream, StreamMap};

use self::core_collection::CoreCollection;
use self::internal_collection::InternalCollection;
use crate::constants::*;
use crate::error::InterpreterError;
use crate::graph::types::Network;
use crate::{BoxError, BoxFuture, SharedHandler};

pub(crate) type ComponentMap = HashMap<String, ComponentSignature>;

#[derive(Debug)]
#[must_use]
pub struct HandlerMap {
  components: HashMap<String, NamespaceHandler>,
}

impl Default for HandlerMap {
  fn default() -> Self {
    Self::new(Vec::new()).unwrap()
  }
}

impl HandlerMap {
  pub fn new(components: Vec<NamespaceHandler>) -> Result<Self, InterpreterError> {
    let mut map = Self {
      components: Default::default(),
    };
    for collection in components {
      map.add(collection)?;
    }

    map.add(NamespaceHandler::new(
      NS_INTERNAL,
      Box::new(InternalCollection::default()),
    ))?;

    Ok(map)
  }

  pub fn add_core(&mut self, network: &Network) -> Result<(), InterpreterError> {
    self.add(NamespaceHandler::new(NS_CORE, Box::new(CoreCollection::new(network))))
  }

  #[must_use]
  pub fn inner(&self) -> &HashMap<String, NamespaceHandler> {
    &self.components
  }

  #[must_use]
  pub fn component_signatures(&self) -> ComponentMap {
    self
      .components
      .iter()
      .map(|(name, p)| (name.clone(), p.component.list().clone()))
      .collect::<HashMap<String, ComponentSignature>>()
  }

  #[must_use]
  pub fn get(&self, namespace: &str) -> Option<&NamespaceHandler> {
    self.components.get(namespace)
  }

  pub fn add(&mut self, component: NamespaceHandler) -> Result<(), InterpreterError> {
    trace!(namespace = %component.namespace, "adding component");
    if self.components.contains_key(&component.namespace) {
      return Err(InterpreterError::DuplicateNamespace(component.namespace));
    }
    self.components.insert(component.namespace.clone(), component);
    Ok(())
  }
}

pub(crate) fn dyn_component_id(name: &str, schematic: &str, instance: &str) -> String {
  format!("{}<{}::{}>", name, schematic, instance)
}

pub(crate) fn get_id(ns: &str, name: &str, schematic: &str, instance: &str) -> String {
  if ns == NS_CORE && name == CORE_ID_MERGE {
    dyn_component_id(name, schematic, instance)
  } else {
    name.to_owned()
  }
}

#[derive(Clone)]
#[must_use]
pub struct NamespaceHandler {
  pub(crate) namespace: String,
  pub(crate) component: SharedHandler,
  pub(crate) exposed: Arc<AtomicBool>,
}

impl NamespaceHandler {
  pub fn new<T: AsRef<str>>(namespace: T, collection: Box<dyn Component + Send + Sync>) -> Self {
    Self {
      namespace: namespace.as_ref().to_owned(),
      component: Arc::new(collection),
      exposed: Arc::new(AtomicBool::new(false)),
    }
  }

  #[must_use]
  pub fn component(&self) -> &SharedHandler {
    &self.component
  }

  pub fn expose(&self) {
    self.exposed.store(true, std::sync::atomic::Ordering::Relaxed);
  }

  #[must_use]
  pub fn is_exposed(&self) -> bool {
    self.exposed.load(std::sync::atomic::Ordering::Relaxed)
  }
}

impl Debug for NamespaceHandler {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NamespaceHandler")
      .field("namespace", &self.namespace)
      .field("collection", &self.component.list())
      .finish()
  }
}

pub trait Component {
  fn handle(
    &self,
    invocation: Invocation,
    stream: PacketStream,
    data: Option<Value>,
  ) -> BoxFuture<Result<PacketStream, BoxError>>;
  fn list(&self) -> &ComponentSignature;
  fn shutdown(&self) -> BoxFuture<Result<(), BoxError>> {
    // Override if you need a more explicit shutdown.
    Box::pin(async move { Ok(()) })
  }
}

pub trait Operation {
  fn handle(&self, payload: StreamMap, data: Option<Value>) -> BoxFuture<Result<PacketStream, BoxError>>;
}