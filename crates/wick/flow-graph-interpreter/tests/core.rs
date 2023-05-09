#![allow(unused_attributes, clippy::box_default)]

mod test;

use anyhow::Result;
use pretty_assertions::assert_eq;
use serde_json::json;
use wick_packet::{packets, Packet};

#[test_logger::test(tokio::test)]
async fn test_senders() -> Result<()> {
  let (interpreter, mut outputs) =
    test::common_setup("./tests/manifests/v0/core/senders.yaml", "test", Vec::new()).await?;

  assert_eq!(outputs.len(), 2);

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let expected = Packet::encode("output", "Hello world");
  assert_eq!(wrapper, expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_pluck() -> Result<()> {
  let (interpreter, mut outputs) = test::common_setup(
    "./tests/manifests/v1/core-pluck.yaml",
    "test",
    packets!(("input", json!({ "to_pluck" :"Hello world!", "to_ignore": "ignore me" }))),
  )
  .await?;

  assert_eq!(outputs.len(), 2);

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let expected = Packet::encode("output", "Hello world!");
  assert_eq!(wrapper, expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_drop() -> Result<()> {
  let (interpreter, mut outputs) = test::common_setup(
    "./tests/manifests/v1/core-drop.yaml",
    "test",
    packets!(("first", "first"), ("second", "second"), ("third", "third")),
  )
  .await?;

  assert_eq!(outputs.len(), 2);

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let expected = Packet::encode("output", "second");
  assert_eq!(wrapper, expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
// #[ignore]
async fn test_merge() -> Result<()> {
  let (interpreter, mut outputs) = test::common_setup(
    "./tests/manifests/v1/core-merge.yaml",
    "test",
    packets!(
      ("input_a", "first_value"),
      ("input_b", 2u8),
      ("input_c", ["alpha", "beta"])
    ),
  )
  .await?;

  assert_eq!(outputs.len(), 2);

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let actual = wrapper.deserialize_generic()?;
  let expected = json!({"a": "first_value", "b": 2, "c": ["alpha", "beta"]});
  assert_eq!(actual, expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_subflows() -> Result<()> {
  let (interpreter, mut outputs) = test::common_setup(
    "./tests/manifests/v1/private-flows.yaml",
    "test",
    packets!(("input", "hello WORLD")),
  )
  .await?;

  assert_eq!(outputs.len(), 2);

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let actual: String = wrapper.deserialize()?;
  let expected = "DLROW OLLEH";
  assert_eq!(actual, expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_switch_1() -> Result<()> {
  let (interpreter, mut outputs) = test::common_setup(
    "./tests/manifests/v1/core-switch.yaml",
    "test",
    packets!(("command", "want_reverse"), ("input", "hello WORLD")),
  )
  .await?;

  assert_eq!(outputs.len(), 2);

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let actual: String = wrapper.deserialize()?;
  let expected = "DLROW olleh";
  assert_eq!(actual, expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_switch_2() -> Result<()> {
  let (interpreter, mut outputs) = test::common_setup(
    "./tests/manifests/v1/core-switch.yaml",
    "test",
    packets!(("command", "want_uppercase"), ("input", "hello WORLD")),
  )
  .await?;

  assert_eq!(outputs.len(), 2);

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let actual: String = wrapper.deserialize()?;
  let expected = "HELLO WORLD";
  assert_eq!(actual, expected);
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_switch_default() -> Result<()> {
  let (interpreter, mut outputs) = test::common_setup(
    "./tests/manifests/v1/core-switch.yaml",
    "test",
    packets!(("command", "nomatch"), ("input", "hello WORLD")),
  )
  .await?;

  assert_eq!(outputs.len(), 2);

  let _ = outputs.pop();
  let wrapper = outputs.pop().unwrap().unwrap();
  let actual: String = wrapper.deserialize()?;
  let expected = "hello WORLD";
  assert_eq!(actual, expected);
  interpreter.shutdown().await?;

  Ok(())
}

// #[test_logger::test(tokio::test)]
// async fn test_merge() -> Result<()> {
//   let manifest = load("./tests/manifests/v0/core/merge.yaml")?;
//   let network = from_def(&manifest)?;
//   let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
//   let mut inputs = PacketMap::default();
//   inputs.insert("schem_one", "first value");
//   inputs.insert("schem_two", 2u8);
//   inputs.insert("schem_three", &["alpha".to_owned(), "beta".to_owned()]);

//   let invocation = Invocation::new_test("merge", Entity::local("test"), inputs, None);
//   let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
//   interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
//   let mut stream = interpreter.invoke(invocation).await?;

//   let mut outputs: Vec<_> = stream.drain().await;
//   println!("{:#?}", outputs);

//   let wrapper = outputs.pop().unwrap();

//   #[derive(serde::Deserialize, PartialEq, Debug)]
//   struct Merged {
//     one: String,
//     two: i32,
//     three: Vec<String>,
//   }

//   let result: Merged = wrapper.deserialize()?;

//   assert_eq!(
//     result,
//     Merged {
//       one: "first value".to_owned(),
//       two: 2,
//       three: vec!["alpha".to_owned(), "beta".to_owned()]
//     }
//   );
//   interpreter.shutdown().await?;

//   Ok(())
// }
