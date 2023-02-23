#![allow(unused_attributes, clippy::box_default)]

mod test;

use anyhow::Result;
use rot::*;
use seeded_random::Seed;
use test::JsonWriter;
use wasmflow_interpreter::graph::from_def;
use wasmflow_packet_stream::{packet_stream, Packet};

#[test_logger::test(tokio::test)]
async fn test_echo() -> Result<()> {
  let (interpreter, mut outputs) = interp!(
    "./tests/manifests/v0/echo.wafl",
    "echo",
    packet_stream!(("input", "hello world"))
  );

  assert_equal!(outputs.len(), 2);

  let _wrapper = outputs.pop().unwrap(); //done signal
  let wrapper = outputs.pop().unwrap();
  let expected = Packet::encode("output", "hello world");

  assert_equal!(wrapper.unwrap(), expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_external_collection() -> Result<()> {
  let (interpreter, mut outputs) = interp!(
    "./tests/manifests/v0/external.wafl",
    "test",
    packet_stream!(("input", "hello world"))
  );

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let expected = Packet::encode("output", "hello world");

  assert_equal!(wrapper, expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_self() -> Result<()> {
  let (interpreter, mut outputs) = interp!(
    "./tests/manifests/v0/reference-self.wafl",
    "test",
    packet_stream!(("parent_input", "Hello world"))
  );

  assert_equal!(outputs.len(), 2);

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let expected = Packet::encode("parent_output", "Hello world");

  assert_equal!(wrapper, expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_spread() -> Result<()> {
  let (interpreter, mut outputs) = interp!(
    "./tests/manifests/v0/spread.wafl",
    "test",
    packet_stream!(("input", "Hello world"))
  );

  assert_equal!(outputs.len(), 4);

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let expected = Packet::encode("output2", "Hello world");
  assert_equal!(wrapper, expected);
  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let expected = Packet::encode("output1", "Hello world");
  assert_equal!(wrapper, expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_stream() -> Result<()> {
  let (interpreter, mut outputs) = interp!(
    "./tests/manifests/v0/stream.wafl",
    "test",
    packet_stream!(("input", "Hello world"))
  );

  assert_equal!(outputs.len(), 6);

  let _ = outputs.pop();
  let expected = Packet::encode("output", "Hello world");

  for wrapper in outputs {
    assert_equal!(wrapper.unwrap(), expected);
  }
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_stream_multi() -> Result<()> {
  let (interpreter, outputs) = interp!(
    "./tests/manifests/v0/stream-multi.wafl",
    "test",
    packet_stream!(("input", "hello world"))
  );
  assert_equal!(outputs.len(), 13);

  let (mut vowels, mut rest): (Vec<_>, Vec<_>) = outputs
    .into_iter()
    .map(|p| p.unwrap())
    .partition(|wrapper| wrapper.port_name() == "vowels");
  vowels.pop();
  rest.pop();

  let mut expected_vowels: Vec<_> = "eoo".chars().collect();
  while let Some(ch) = expected_vowels.pop() {
    let wrapper = vowels.pop().unwrap();
    assert_equal!(wrapper, Packet::encode("vowels", ch));
  }

  let mut expected_other: Vec<_> = "hll wrld".chars().collect();
  while let Some(ch) = expected_other.pop() {
    let wrapper = rest.pop().unwrap();
    assert_equal!(wrapper, Packet::encode("rest", ch));
  }
  interpreter.shutdown().await?;

  Ok(())
}

// #[test_logger::test(tokio::test)]
// async fn test_exception_default() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/exception-default.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
//   let inputs = PacketMap::from([("input", "Hello world".to_owned())]);

//   let invocation = invocation("exception-default","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let mut outputs: Vec<_> = stream.drain().await;
//   println!("{:#?}", outputs);

//   let wrapper = outputs.pop().unwrap();
//   let result: String = wrapper.deserialize()?;

//   assert_equal!(result, "eulav tluafeD".to_owned());

//   interpreter.shutdown().await?;

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// async fn test_exception_nodefault() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/exception-nodefault.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
//   let inputs = PacketMap::from([("input", "Hello world".to_owned())]);

//   let invocation = invocation("exception-nodefault","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let mut outputs: Vec<_> = stream.drain().await;
//   println!("{:#?}", outputs);

//   let wrapper = outputs.pop().unwrap();
//   assert_true!(matches!(wrapper.payload, MessageTransport::Failure(_)));

//   interpreter.shutdown().await?;

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// async fn test_inherent() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/inherent.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
//   let inputs = PacketMap::default();

//   let invocation = invocation("inherent","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let mut outputs: Vec<_> = stream.drain().await;
//   println!("{:#?}", outputs);

//   let wrapper = outputs.pop().unwrap();
//   assert_true!(matches!(wrapper.payload, MessageTransport::Success(_)));

//   interpreter.shutdown().await?;

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// async fn test_inherent_nested() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/inherent-nested.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
//   let inputs = PacketMap::default();

//   let invocation = invocation("inherent_nested","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let mut outputs: Vec<_> = stream.drain().await;
//   interpreter.shutdown().await?;
//   println!("{:#?}", outputs);

//   let wrapper = outputs.pop().unwrap();
//   assert_true!(matches!(wrapper.payload, MessageTransport::Success(_)));

//   let wrapper = outputs.pop().unwrap();
//   assert_true!(matches!(wrapper.payload, MessageTransport::Success(_)));

//   let wrapper = outputs.pop().unwrap();
//   assert_true!(matches!(wrapper.payload, MessageTransport::Success(_)));

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// async fn test_inherent_disconnected() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/inherent-disconnected.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
//   let inputs = PacketMap::from([("input", "Hello world".to_owned())]);

//   let invocation = invocation("inherent_disconnected","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let mut outputs: Vec<_> = stream.drain().await;
//   println!("{:#?}", outputs);
//   assert_equal!(outputs.len(), 1);

//   let wrapper = outputs.pop().unwrap();
//   assert_true!(matches!(wrapper.payload, MessageTransport::Success(_)));

//   interpreter.shutdown().await?;

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// async fn test_stream() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/stream.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
//   let input_str = "Hello world".to_owned();
//   let inputs = PacketMap::from([("input", input_str.clone())]);

//   let invocation = invocation("stream","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let outputs: Vec<_> = stream.drain().await;
//   println!("{:#?}", outputs);
//   assert_equal!(outputs.len(), 5);

//   for wrapper in outputs {
//     let output: String = wrapper.payload.deserialize()?;
//     assert_equal!(output, input_str);
//   }
//   interpreter.shutdown().await?;

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// async fn test_generator() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/generator.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

//   let inputs = PacketMap::default();
//   let invocation = invocation("generator","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let mut outputs: Vec<_> = stream.drain().await;
//   interpreter.shutdown().await?;
//   println!("{:#?}", outputs);
//   assert_equal!(outputs.len(), 1);

//   let wrapper = outputs.pop().unwrap();
//   assert_true!(matches!(wrapper.payload, MessageTransport::Success(_)));

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// async fn test_generator_sibling() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/generator-sibling.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

//   let inputs = PacketMap::from([("input", "my-input".to_owned())]);
//   let invocation = invocation("generator-sibling","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let mut outputs: Vec<_> = stream.drain().await;
//   interpreter.shutdown().await?;
//   println!("{:#?}", outputs);
//   assert_equal!(outputs.len(), 1);

//   let wrapper = outputs.pop().unwrap();
//   assert_true!(matches!(wrapper.payload, MessageTransport::Success(_)));

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// async fn test_generator_multi_sibling() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/generator-multi-sibling.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

//   let inputs = PacketMap::from([
//     ("one", "one".to_owned()),
//     ("two", "two".to_owned()),
//     ("three", "three".to_owned()),
//     ("four", "four".to_owned()),
//   ]);
//   let invocation = invocation("generator-sibling","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let mut outputs: Vec<_> = stream.drain().await;
//   interpreter.shutdown().await?;
//   println!("{:#?}", outputs);
//   assert_equal!(outputs.len(), 1);

//   let wrapper = outputs.pop().unwrap();
//   assert_true!(matches!(wrapper.payload, MessageTransport::Success(_)));

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// #[ignore]
// async fn test_stream_collection_ref() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/stream-collection-ref.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

//   let inputs = PacketMap::from([("input", "my-input".to_owned())]);
//   let invocation = invocation("stream_collection_ref","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let outputs: Vec<_> = stream.drain().await;
//   interpreter.shutdown().await?;
//   println!("{:#?}", outputs);

//   assert_equal!(outputs.len(), 5);

//   for wrapper in outputs {
//     assert_true!(matches!(wrapper.payload, MessageTransport::Success(_)));
//   }

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// async fn test_stream_multi() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/stream-multi.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

//   let payload_stream: Flux<Payload, PayloadError> = Flux::new();
//   // TODO metadata
//   let metadata = b"";
//   let payload = Payload::new_data(metadata, wasmrs_codec::messagepack::serialize("hello world")?);
//   payload_stream.send(payload);

//   let invocation = InvocationStream::new(
//     Entity::Test("test_stream_multi"),
//     Entity::local("test"),
//     payload_stream.take_rx().unwrap(),
//     None,
//   );

//   // let inputs = PacketMap::from([("input", "hello world".to_owned())]);
//   // let invocation = invocation("stream_multi","test");

//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let outputs: Vec<_> = stream.drain().await;
//   interpreter.shutdown().await?;
//   println!("{:#?}", outputs);
//   assert_equal!(outputs.len(), 11);

//   let (mut vowels, mut rest): (Vec<_>, Vec<_>) = outputs.into_iter().partition(|wrapper| wrapper.port == "vowels");

//   let mut expected_vowels: Vec<_> = "eoo".chars().collect();
//   while let Some(ch) = expected_vowels.pop() {
//     let wrapper = vowels.pop().unwrap();
//     assert_equal!(wrapper.payload, MessageTransport::success(&ch));
//   }

//   let mut expected_other: Vec<_> = "hll wrld".chars().collect();
//   while let Some(ch) = expected_other.pop() {
//     let wrapper = rest.pop().unwrap();
//     assert_equal!(wrapper.payload, MessageTransport::success(&ch));
//   }

//   Ok(())
// }

// #[test_logger::test(tokio::test)]
// async fn test_no_inputs() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/no-inputs.wafl")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

//   let inputs = PacketMap::default();

//   let invocation = invocation("no-inputs","test");
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let stream = interpreter.invoke(invocation).await?;

//   let mut outputs: Vec<_> = stream.collect().await;
//   println!("{:#?}", outputs);
//   assert_equal!(outputs.len(), 2);

//   let _wrapper = outputs.pop().unwrap(); //done signal
//   let wrapper = outputs.pop().unwrap();
//   let result: String = wrapper.deserialize()?;

//   assert_equal!(result, "Hello world".to_owned());
//   interpreter.shutdown().await?;

//   Ok(())
// }
