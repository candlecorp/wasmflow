use std::collections::HashMap;

use pretty_assertions::assert_eq;
use runtime_testutils::*;
use wasmflow_sdk::v1::transport::{Failure, MessageTransport, TransportWrapper};
use wasmflow_sdk::v1::{Entity, Invocation};
type Result<T> = anyhow::Result<T, anyhow::Error>;

#[test_logger::test(tokio::test)]
async fn panics() -> Result<()> {
  let (network, _) = init_network_from_yaml("./manifests/v0/errors/panics.wafl").await?;

  let mut data = HashMap::new();
  data.insert("input", "input");

  let mut result = network
    .invoke(Invocation::new(
      Entity::test("panics"),
      Entity::local("panics"),
      data.try_into()?,
      None,
    ))
    .await?;

  println!("Result: {:?}", result);
  let mut messages: Vec<TransportWrapper> = result.drain_port("output").await?;
  assert_eq!(result.buffered_size(), (0, 0));
  assert_eq!(messages.len(), 1);

  let msg: TransportWrapper = messages.pop().unwrap();
  println!("Output: {:?}", msg);

  assert!(matches!(msg.payload, MessageTransport::Failure(Failure::Error(_))));
  Ok(())
}

#[test_logger::test(tokio::test)]
async fn errors() -> Result<()> {
  let (network, _) = init_network_from_yaml("./manifests/v0/errors/errors.wafl").await?;

  let mut data = HashMap::new();
  data.insert("input", "input");

  let mut result = network
    .invoke(Invocation::new(
      Entity::test("errors"),
      Entity::local("errors"),
      data.try_into()?,
      None,
    ))
    .await?;

  println!("Result: {:?}", result);
  let mut messages: Vec<TransportWrapper> = result.drain_port("output").await?;
  assert_eq!(result.buffered_size(), (0, 0));
  assert_eq!(messages.len(), 1);

  let msg: TransportWrapper = messages.pop().unwrap();
  println!("Output: {:?}", msg);
  assert_eq!(msg.payload, MessageTransport::error("This component will always error"));
  Ok(())
}
