use flow_component::{Component, ComponentError, RuntimeCallback};
use futures::FutureExt;
use wick_interface_types::{operation, ComponentSignature};
use wick_packet::{GenericConfig, Invocation, PacketStream};

use crate::constants::*;
use crate::BoxFuture;

#[derive(Debug)]
pub(crate) struct NullComponent {
  signature: ComponentSignature,
}

impl NullComponent {
  pub(crate) fn new() -> Self {
    let mut this = Self {
      signature: ComponentSignature::new(NS_NULL).version("0.0.0"),
    };
    this.signature = this
      .signature
      .add_operation(operation! {"drop"=>{inputs:{"input"=>"object"},outputs:{},}});

    this
  }
}

impl Component for NullComponent {
  fn handle(
    &self,
    invocation: Invocation,
    _data: Option<GenericConfig>,
    _callback: std::sync::Arc<RuntimeCallback>,
  ) -> BoxFuture<Result<PacketStream, ComponentError>> {
    invocation.trace(|| trace!(target = %invocation.target, namespace = NS_CORE));
    async move { Ok(PacketStream::empty()) }.boxed()
  }

  fn signature(&self) -> &ComponentSignature {
    &self.signature
  }
}
