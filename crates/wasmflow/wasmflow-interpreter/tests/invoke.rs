use std::path::Path;

mod test;
use anyhow::Result;
use futures::future::BoxFuture;
use serde_json::Value;
use test::{JsonWriter, TestProvider};
use wasmflow_interpreter::graph::from_def;
use wasmflow_interpreter::{BoxError, HandlerMap, Interpreter, NamespaceHandler, Provider};
use wasmflow_manifest::Loadable;
use seeded_random::Seed;
use wasmflow_transport::TransportStream;
use wasmflow_entity::Entity;
use wasmflow_interface::ProviderSignature;
use wasmflow_invocation::Invocation;
use wasmflow_packet::PacketMap;
struct SignatureProvider(ProviderSignature);
impl Provider for SignatureProvider {
  fn handle(&self, _payload: Invocation, _config: Option<Value>) -> BoxFuture<Result<TransportStream, BoxError>> {
    todo!()
  }

  fn list(&self) -> &wasmflow_interface::ProviderSignature {
    &self.0
  }
}

fn load<T: AsRef<Path>>(path: T) -> Result<wasmflow_manifest::HostManifest> {
  Ok(wasmflow_manifest::HostManifest::load_from_file(path.as_ref())?)
}

#[test_logger::test(tokio::test)]
async fn test_invoke_provider() -> Result<()> {
  let manifest = load("./tests/manifests/v0/external.yaml")?;
  let network = from_def(&manifest.network().try_into()?)?;
  let providers = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestProvider::new()))]);

  let inputs = PacketMap::from([("input", "Hello world".to_owned())]);

  let entity = Entity::component("test", "echo");

  let invocation = Invocation::new_test("invoke provider", entity, inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(providers))?;
  interpreter.start(None, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  println!("{:#?}", outputs);

  let wrapper = outputs.pop().unwrap();
  let result: String = wrapper.deserialize()?;

  assert_eq!(result, "Hello world".to_owned());
  interpreter.shutdown().await?;

  Ok(())
}
