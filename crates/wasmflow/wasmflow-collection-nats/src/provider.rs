use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use futures::executor::block_on;
use wasmflow_lattice::Lattice;
use wasmflow_rpc::error::RpcError;
use wasmflow_rpc::{RpcHandler, RpcResult};
use wasmflow_transport::TransportStream;
use wasmflow_invocation::Invocation;

use crate::Error;

#[derive(Debug, Default)]
pub struct Context {
  pub documents: HashMap<String, String>,
  pub collections: HashMap<String, Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct Provider {
  lattice_id: String,
  lattice: Arc<Lattice>,
}

impl Provider {
  pub async fn new(lattice_id: String, lattice: Arc<Lattice>) -> Result<Self, Error> {
    Ok(Self { lattice_id, lattice })
  }
}

#[async_trait]
impl RpcHandler for Provider {
  async fn invoke(&self, invocation: Invocation) -> RpcResult<TransportStream> {
    let target_url = invocation.target_url();
    trace!(target = %target_url, "lattice invoke");

    let start = Instant::now();
    let stream = self
      .lattice
      .invoke(&self.lattice_id, invocation)
      .await
      .map_err(|e| RpcError::ProviderError(e.to_string()))?;

    trace!(
      target = %target_url,
      duration_ms = %start.elapsed().as_millis(),
      "response stream received",
    );

    Ok(stream)
  }

  fn get_list(&self) -> RpcResult<Vec<wasmflow_interface::HostedType>> {
    let components = block_on(self.lattice.list_components(self.lattice_id.clone()))
      .map_err(|e| RpcError::ProviderError(e.to_string()))?;

    Ok(components)
  }
}

#[cfg(test)]
mod tests {

  use anyhow::Result as TestResult;
  use wasmflow_lattice::LatticeBuilder;
  use wasmflow_rpc::SharedRpcHandler;
  use wasmflow_transport::MessageTransport;
  use wasmflow_entity::Entity;
  use wasmflow_packet::PacketMap;

  use super::*;

  fn get_provider() -> SharedRpcHandler {
    Arc::new(test_native_provider::Provider::default())
  }

  #[test_logger::test(tokio::test)]
  async fn integration_test_component() -> TestResult<()> {
    let lattice_builder = LatticeBuilder::new_from_env("test")?;
    let lattice = lattice_builder.build().await?;
    let ns = "some_namespace";

    lattice.handle_namespace(ns.to_owned(), get_provider()).await?;

    let provider = Provider::new(ns.to_owned(), Arc::new(lattice)).await?;
    let user_data = "Hello world";

    let job_payload = PacketMap::from([("input", user_data)]);
    let invocation = Invocation::new_test(file!(), Entity::component(ns, "test-component"), job_payload, None);

    let mut stream = provider.invoke(invocation).await?;
    let output = stream.drain_port("output").await?[0].clone();

    println!("payload from [{}]: {:?}", output.port, output.payload);
    let output: String = output.payload.deserialize()?;

    println!("output: {:?}", output);
    assert_eq!(output, format!("TEST: {}", user_data));
    Ok(())
  }

  #[test_logger::test(tokio::test)]
  async fn integration_test_error() -> TestResult<()> {
    let lattice_builder = LatticeBuilder::new_from_env("test")?;
    let lattice = lattice_builder.build().await?;
    let ns = "some_namespace";

    lattice.handle_namespace(ns.to_owned(), get_provider()).await?;

    let provider = Provider::new(ns.to_owned(), Arc::new(lattice)).await?;
    let user_data = "Hello world";

    let job_payload = PacketMap::from([("input", user_data)]);

    let invocation = Invocation::new_test(file!(), Entity::component(ns, "error"), job_payload, None);

    let mut stream = provider.invoke(invocation).await?;
    let outputs = stream.drain().await;
    let output = outputs[0].clone();
    println!("payload from [{}]: {:?}", output.port, output.payload);
    assert_eq!(output.payload, MessageTransport::error("This always errors"));
    Ok(())
  }
}
