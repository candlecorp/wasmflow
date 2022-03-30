use futures::prelude::*;
use log::debug;
use test_vino_provider::Provider;
use vino_rpc::RpcHandler;

#[test_logger::test(tokio::test)]
async fn request() -> anyhow::Result<()> {
  let provider = Provider::default();
  let input = "some_input";
  let job_payload = vec![("input", input)].into();
  let invocation = vino_transport::Invocation::new_test(
    file!(),
    vino_entity::Entity::local_component("test-component"),
    job_payload,
    None,
  );

  let mut outputs = provider.invoke(invocation).await?;
  let output = outputs.next().await.unwrap();
  println!("Received payload from [{}]", output.port);
  let payload: String = output.payload.deserialize()?;

  println!("outputs: {:?}", payload);
  assert_eq!(payload, "TEST: some_input");

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn list() -> anyhow::Result<()> {
  let provider = Provider::default();

  let response = provider.get_list()?;
  debug!("list response : {:?}", response);

  Ok(())
}
