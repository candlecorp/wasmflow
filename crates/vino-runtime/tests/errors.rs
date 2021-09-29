#[path = "./runtime_utils/mod.rs"]
mod utils;
use std::collections::HashMap;

use utils::*;
use vino_entity::Entity;
use vino_runtime::prelude::TransportWrapper;
use vino_transport::MessageTransport;

#[test_logger::test(actix_rt::test)]

async fn panics() -> Result<()> {
  let (network, _) = init_network_from_yaml("./manifests/v0/errors/panics.yaml").await?;

  let mut data = HashMap::new();
  data.insert("input", "input");

  let mut result = network
    .request("panics", Entity::test("panics"), &data)
    .await?;

  println!("Result: {:?}", result);
  let mut messages: Vec<TransportWrapper> = result.collect_port("output").await;
  assert_eq!(result.buffered_size(), (0, 0));
  assert_eq!(messages.len(), 1);

  let msg: TransportWrapper = messages.pop().unwrap();
  println!("Output: {:?}", msg);
  assert_eq!(
    msg.payload,
    MessageTransport::error("Component error: panic")
  );
  Ok(())
}

#[test_logger::test(actix_rt::test)]
async fn errors() -> Result<()> {
  let (network, _) = init_network_from_yaml("./manifests/v0/errors/errors.yaml").await?;

  let mut data = HashMap::new();
  data.insert("input", "input");

  let mut result = network
    .request("errors", Entity::test("errors"), &data)
    .await?;

  println!("Result: {:?}", result);
  let mut messages: Vec<TransportWrapper> = result.collect_port("output").await;
  assert_eq!(result.buffered_size(), (0, 0));
  assert_eq!(messages.len(), 1);

  let msg: TransportWrapper = messages.pop().unwrap();
  println!("Output: {:?}", msg);
  assert_eq!(
    msg.payload,
    MessageTransport::error("This component will always error")
  );
  Ok(())
}