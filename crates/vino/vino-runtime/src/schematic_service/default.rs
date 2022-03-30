use std::borrow::Cow;

use vino_manifest::process_default;

use crate::dev::prelude::*;

pub(crate) fn make_default_transport(json: &serde_json::Value, message: &str) -> MessageTransport {
  process_default(Cow::Borrowed(json), message).map_or(
    MessageTransport::error("Error processing default value"),
    |result| {
      mp_serialize(&result).map_or(MessageTransport::error("Error serializing default value"), |bytes| {
        MessageTransport::Success(Success::MessagePack(bytes))
      })
    },
  )
}

#[cfg(test)]
mod tests {

  use vino_manifest::parse_default;

  use super::*;
  use crate::test::prelude::{assert_eq, *};

  #[test_logger::test]
  fn test_to_transport() -> TestResult<()> {
    let json_str = r#"
    "Error: $ERROR"
    "#;

    let json = parse_default(json_str)?;

    let err = "This is my error message";
    let message: String = make_default_transport(&json, err).deserialize()?;

    assert_eq!(message, format!("Error: {}", err));

    Ok(())
  }
}
