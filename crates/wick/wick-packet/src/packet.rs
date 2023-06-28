use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasmrs::{BoxFlux, Metadata, Payload, PayloadError, RawPayload};
use wick_interface_types::Type;

use crate::metadata::DONE_FLAG;
use crate::{
  Base64Bytes,
  ComponentReference,
  Error,
  PacketStream,
  TypeWrapper,
  WickMetadata,
  CLOSE_BRACKET,
  OPEN_BRACKET,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use]
pub struct Packet {
  pub(crate) metadata: Metadata,
  pub(crate) extra: WickMetadata,
  pub payload: PacketPayload,
}

impl PartialEq for Packet {
  fn eq(&self, other: &Self) -> bool {
    if self.metadata.index != other.metadata.index || !self.metadata.extra.eq(&other.metadata.extra) {
      return false;
    }
    if self.extra.ne(&other.extra) {
      return false;
    }
    self.payload == other.payload
  }
}

impl Packet {
  /// The port name that indicates a component-wide fatal error.
  pub const FATAL_ERROR: &str = "<error>";
  pub const NO_INPUT: &str = "<>";

  /// Create a new packet for the given port with a raw [PacketPayload], wasmRS [Metadata], and [WickMetadata].
  pub fn new_raw(payload: PacketPayload, wasmrs: Metadata, metadata: WickMetadata) -> Self {
    Self {
      payload,
      metadata: wasmrs,
      extra: metadata,
    }
  }

  /// Create a new packet for the given port with a raw [PacketPayload] value and given flags.
  pub fn new_for_port(port: impl AsRef<str>, payload: PacketPayload, flags: u8) -> Self {
    let md = Metadata::new(0);
    let wmd = WickMetadata::new(port, flags);
    Self {
      payload,
      metadata: md,
      extra: wmd,
    }
  }

  /// Returns `true` if the packet contains data in the payload.
  pub fn has_data(&self) -> bool {
    match &self.payload {
      PacketPayload::Ok(Some(data)) => !data.is_empty(),
      PacketPayload::Ok(None) => false,
      PacketPayload::Err(_) => false,
    }
  }

  pub fn no_input() -> Self {
    Self::encode(Self::NO_INPUT, ())
  }

  /// Create a new fatal error packet for the component.
  pub fn component_error(err: impl AsRef<str>) -> Self {
    Self::new_for_port(Self::FATAL_ERROR, PacketPayload::fatal_error(err), 0)
  }

  /// Create a new success packet for the given port with a raw [RawPayload] value.
  pub fn ok(port: impl AsRef<str>, payload: RawPayload) -> Self {
    Self::new_for_port(port, PacketPayload::Ok(payload.data.map(Into::into)), 0)
  }

  /// Create a new error packet for the given port with a raw [PacketError] value.
  pub fn raw_err(port: impl AsRef<str>, payload: PacketError) -> Self {
    Self::new_for_port(port, PacketPayload::Err(payload), 0)
  }

  /// Create a new error packet for the given port.
  pub fn err(port: impl AsRef<str>, msg: impl AsRef<str>) -> Self {
    Self::new_for_port(port, PacketPayload::Err(PacketError::new(msg)), 0)
  }

  /// Create a new done packet for the given port.
  pub fn done(port: impl AsRef<str>) -> Self {
    Self::new_for_port(port, PacketPayload::Ok(None), DONE_FLAG)
  }

  /// Create a new open bracket packet for the given port.
  pub fn open_bracket(port: impl AsRef<str>) -> Self {
    Self::new_for_port(port, PacketPayload::Ok(None), OPEN_BRACKET)
  }

  /// Create a close bracket packet for the given port.
  pub fn close_bracket(port: impl AsRef<str>) -> Self {
    Self::new_for_port(port, PacketPayload::Ok(None), CLOSE_BRACKET)
  }

  /// Get the context of a [crate::ContextTransport] on this packet.
  pub fn context(&self) -> Option<Base64Bytes> {
    self.extra.context.clone()
  }

  /// Set the content of a [crate::ContextTransport] on this packet.
  pub fn set_context(&mut self, context: Base64Bytes) {
    self.extra.context = Some(context);
  }

  /// Encode a value into a [Packet] for the given port.
  pub fn encode<T: Serialize>(port: impl AsRef<str>, data: T) -> Self {
    match wasmrs_codec::messagepack::serialize(&data) {
      Ok(bytes) => Self::new_for_port(port, PacketPayload::Ok(Some(bytes.into())), 0),
      Err(err) => Self::new_for_port(port, PacketPayload::Err(PacketError::new(err.to_string())), 0),
    }
  }

  /// Get the flags for this packet.
  pub fn flags(&self) -> u8 {
    self.extra.flags
  }

  /// Get the operation index associated with this packet.
  pub fn index(&self) -> u32 {
    self.metadata.index
  }

  /// Try to deserialize a [Packet] into the target type.
  pub fn decode<T: DeserializeOwned>(self) -> Result<T, Error> {
    self.payload.decode()
  }

  /// Partially decode a [Packet] and wrap it into a [TypeWrapper].
  pub fn to_type_wrapper(self, ty: Type) -> Result<TypeWrapper, Error> {
    self.payload.type_wrapper(ty)
  }

  /// Decode a [Packet] into a [serde_json::Value].
  pub fn decode_value(self) -> Result<serde_json::Value, Error> {
    self.payload.decode()
  }

  /// Set the port for this packet.
  pub fn set_port(mut self, port: impl AsRef<str>) -> Self {
    self.extra.port = port.as_ref().to_owned();
    self
  }

  /// Get the port for this packet.
  pub fn port(&self) -> &str {
    &self.extra.port
  }

  /// Return `true` if this is a No-Op packet. No action should be taken.
  pub fn is_noop(&self) -> bool {
    self.port() == Self::NO_INPUT
  }

  /// Return `true` if this is a fatal, component wide error packet.
  pub fn is_fatal_error(&self) -> bool {
    self.port() == Self::FATAL_ERROR
  }

  /// Return `true` if this is an error packet.
  pub fn is_error(&self) -> bool {
    matches!(self.payload, PacketPayload::Err(_))
  }

  /// Get the inner payload of this packet.
  pub fn payload(&self) -> &PacketPayload {
    &self.payload
  }

  /// Returns true if this packet is a done packet.
  pub fn is_done(&self) -> bool {
    self.extra.is_done()
  }

  /// Returns true if this packet is an open bracket packet.
  pub fn is_open_bracket(&self) -> bool {
    self.extra.is_open_bracket()
  }

  /// Returns true if this packet is a close bracket packet.
  pub fn is_close_bracket(&self) -> bool {
    self.extra.is_close_bracket()
  }

  /// Returns the payload, panicking if it is an error.
  pub fn unwrap_payload(self) -> Option<Base64Bytes> {
    match self.payload {
      PacketPayload::Ok(v) => v,
      _ => panic!("Packet is an error"),
    }
  }

  /// Returns the error, panicking if the packet was a success packet.
  pub fn unwrap_err(self) -> PacketError {
    match self.payload {
      PacketPayload::Err(err) => err,
      _ => panic!("Packet is not an error"),
    }
  }

  /// Return a simplified JSON representation of this packet.
  pub fn to_json(&self) -> serde_json::Value {
    if self.flags() > 0 {
      let mut map = serde_json::json!({
        "flags": self.flags(),
        "port": self.port()
      });
      if self.has_data() {
        map
          .as_object_mut()
          .unwrap()
          .insert("payload".to_owned(), self.payload.to_json());
      }
      map
    } else {
      serde_json::json!({
        "port": self.port(),
        "payload": self.payload.to_json(),
      })
    }
  }
}

impl PartialEq for PacketPayload {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Ok(l0), Self::Ok(r0)) => l0 == r0,
      (Self::Err(l0), Self::Err(r0)) => l0.msg == r0.msg,
      _ => core::mem::discriminant(self) == core::mem::discriminant(other),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketPayload {
  Ok(Option<Base64Bytes>),
  Err(PacketError),
}

impl PacketPayload {
  pub fn fatal_error(err: impl AsRef<str>) -> Self {
    Self::Err(PacketError::new(err))
  }

  /// Try to deserialize a [Packet] into the target type
  pub fn decode<T: DeserializeOwned>(self) -> Result<T, Error> {
    match self {
      PacketPayload::Ok(Some(bytes)) => match wasmrs_codec::messagepack::deserialize(&bytes) {
        Ok(data) => Ok(data),
        Err(err) => Err(crate::Error::Decode {
          as_json: wasmrs_codec::messagepack::deserialize::<serde_json::Value>(&bytes)
            .map_or_else(|_e| "could not convert".to_owned(), |v| v.to_string()),
          payload: bytes.into(),
          error: err.to_string(),
        }),
      },
      PacketPayload::Ok(None) => Err(crate::Error::NoData),
      PacketPayload::Err(err) => Err(crate::Error::PayloadError(err)),
    }
  }

  /// Partially process a [Packet] as [Type].
  pub fn type_wrapper(self, sig: Type) -> Result<TypeWrapper, Error> {
    let val = match sig {
      Type::I8 => TypeWrapper::new(sig, self.decode::<i8>()?.into()),
      Type::I16 => TypeWrapper::new(sig, self.decode::<i16>()?.into()),
      Type::I32 => TypeWrapper::new(sig, self.decode::<i32>()?.into()),
      Type::I64 => TypeWrapper::new(sig, self.decode::<i64>()?.into()),
      Type::U8 => TypeWrapper::new(sig, self.decode::<u8>()?.into()),
      Type::U16 => TypeWrapper::new(sig, self.decode::<u16>()?.into()),
      Type::U32 => TypeWrapper::new(sig, self.decode::<u32>()?.into()),
      Type::U64 => TypeWrapper::new(sig, self.decode::<u64>()?.into()),
      Type::F32 => TypeWrapper::new(sig, self.decode::<f32>()?.into()),
      Type::F64 => TypeWrapper::new(sig, self.decode::<f64>()?.into()),
      Type::Bool => TypeWrapper::new(sig, self.decode::<bool>()?.into()),
      Type::String => TypeWrapper::new(sig, self.decode::<String>()?.into()),
      Type::Datetime => TypeWrapper::new(sig, self.decode::<String>()?.into()),
      Type::Bytes => TypeWrapper::new(sig, self.decode::<Vec<u8>>()?.into()),
      Type::Named(_) => TypeWrapper::new(sig, self.decode::<serde_json::Value>()?),
      Type::List { .. } => TypeWrapper::new(sig, self.decode::<Vec<serde_json::Value>>()?.into()),
      Type::Optional { .. } => TypeWrapper::new(sig, self.decode::<Option<serde_json::Value>>()?.into()),
      Type::Map { .. } => TypeWrapper::new(
        sig,
        serde_json::Value::Object(self.decode::<serde_json::Map<String, serde_json::Value>>()?),
      ),
      #[allow(deprecated)]
      Type::Link { .. } => TypeWrapper::new(
        sig,
        serde_json::Value::String(self.decode::<ComponentReference>()?.to_string()),
      ),
      Type::Object => TypeWrapper::new(sig, self.decode::<serde_json::Value>()?),
      Type::AnonymousStruct(_) => unimplemented!(),
    };
    Ok(val)
  }

  pub fn bytes(&self) -> Option<&Base64Bytes> {
    match self {
      Self::Ok(b) => b.as_ref(),
      _ => None,
    }
  }

  pub fn into_bytes(self) -> Option<Base64Bytes> {
    match self {
      Self::Ok(b) => b,
      _ => None,
    }
  }

  pub fn to_json(&self) -> serde_json::Value {
    match self {
      Self::Ok(Some(b)) => match wasmrs_codec::messagepack::deserialize::<serde_json::Value>(b) {
        Ok(data) => serde_json::json!({ "value": data }),
        Err(err) => serde_json::json! ({"error" : crate::Error::Jsonify(err.to_string()).to_string()}),
      },
      Self::Ok(None) => serde_json::Value::Null,
      Self::Err(err) => serde_json::json! ({"error" : crate::Error::PayloadError(err.clone()).to_string()}),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PacketError {
  msg: String,
}

impl PacketError {
  pub fn new(msg: impl AsRef<str>) -> Self {
    Self {
      msg: msg.as_ref().to_owned(),
    }
  }

  #[must_use]
  pub fn msg(&self) -> &str {
    &self.msg
  }
}

impl std::error::Error for PacketError {}

impl std::fmt::Display for PacketError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.msg)
  }
}

#[must_use]
pub fn packetstream_to_wasmrs(index: u32, stream: PacketStream) -> BoxFlux<RawPayload, PayloadError> {
  let s = tokio_stream::StreamExt::map(stream, move |p| {
    p.map_or_else(
      |e| Err(PayloadError::application_error(e.to_string(), None)),
      |p| {
        let md = wasmrs::Metadata::new_extra(index, p.extra.encode()).encode();
        match p.payload {
          PacketPayload::Ok(b) => Ok(wasmrs::RawPayload::new_data(Some(md), b.map(Into::into))),
          PacketPayload::Err(e) => Err(wasmrs::PayloadError::application_error(e.msg(), Some(md))),
        }
      },
    )
  });
  Box::pin(s)
}

pub fn from_raw_wasmrs(stream: BoxFlux<RawPayload, PayloadError>) -> PacketStream {
  let s = tokio_stream::StreamExt::map(stream, move |p| {
    let p = p.map_or_else(
      |e| {
        if let Some(mut metadata) = e.metadata {
          let md = wasmrs::Metadata::decode(&mut metadata);

          let wmd = md.map_or_else(
            |_e| WickMetadata::default(),
            |m| {
              m.extra
                .map_or_else(WickMetadata::default, |extra| WickMetadata::decode(extra).unwrap())
            },
          );
          Packet::raw_err(wmd.port, PacketError::new(e.msg))
        } else {
          Packet::component_error(e.msg)
        }
      },
      |p| {
        if let Some(mut metadata) = p.metadata {
          let md = wasmrs::Metadata::decode(&mut metadata);

          let wmd = md.map_or_else(
            |_e| WickMetadata::default(),
            |m| {
              m.extra
                .map_or_else(WickMetadata::default, |extra| WickMetadata::decode(extra).unwrap())
            },
          );
          // Potential danger zone: this converts empty payload to None which *should* be the
          // same thing. Calling this out as a potential source for weird bugs if they pop up.
          let data = p.data.and_then(|b| (!b.is_empty()).then_some(b));
          Packet::new_for_port(wmd.port(), PacketPayload::Ok(data.map(Into::into)), wmd.flags())
        } else {
          Packet::component_error("invalid wasmrs packet with no metadata.")
        }
      },
    );
    Ok(p)
  });

  PacketStream::new(Box::new(s))
}

pub fn from_wasmrs(stream: BoxFlux<Payload, PayloadError>) -> PacketStream {
  let s = tokio_stream::StreamExt::map(stream, move |p| {
    let p = p.map_or_else(
      |e| {
        let md = wasmrs::Metadata::decode(&mut e.metadata.unwrap());

        let wmd = md.map_or_else(
          |_e| WickMetadata::default(),
          |m| WickMetadata::decode(m.extra.unwrap()).unwrap(),
        );
        Packet::raw_err(wmd.port, PacketError::new(e.msg))
      },
      |p| {
        let md = p.metadata;
        let wmd = WickMetadata::decode(md.extra.unwrap()).unwrap();
        // Potential danger zone: this converts empty payload to None which *should* be the
        // same thing. Calling this out as a potential source for weird bugs if they pop up.
        let data = p.data;
        Packet::new_for_port(wmd.port(), PacketPayload::Ok(Some(data.into())), wmd.flags())
      },
    );
    Ok(p)
  });
  PacketStream::new(Box::new(s))
}

impl From<Payload> for Packet {
  fn from(mut value: Payload) -> Self {
    let ex = value.metadata.extra.take();

    Self {
      extra: WickMetadata::decode(ex.unwrap()).unwrap(),
      metadata: value.metadata,
      payload: PacketPayload::Ok(Some(value.data.into())),
    }
  }
}

impl From<Packet> for Result<RawPayload, PayloadError> {
  fn from(value: Packet) -> Self {
    let mut md = value.metadata;
    md.extra = Some(value.extra.encode());
    match value.payload {
      PacketPayload::Ok(b) => Ok(RawPayload::new_data(Some(md.encode()), b.map(Into::into))),
      PacketPayload::Err(e) => Err(PayloadError::application_error(e.msg(), Some(md.encode()))),
    }
  }
}

#[cfg(test)]
mod test {
  use anyhow::Result;
  use serde_json::Value;

  use crate::{Base64Bytes, Packet};

  #[test]
  fn test_basic() -> Result<()> {
    let packet = Packet::encode("test", 10);
    let res: i32 = packet.decode()?;
    assert_eq!(res, 10);
    Ok(())
  }

  #[rstest::rstest]
  #[case(u64::MIN, Value::Number(serde_json::Number::from(u64::MIN)))]
  #[case(u64::MAX, Value::Number(serde_json::Number::from(u64::MAX)))]
  #[case(&[1,2,3,4,5,6], vec![1,2,3,4,5,6].into())]
  #[case("test", Value::String("test".to_owned()))]
  #[case(Base64Bytes::new(b"test".as_slice()), Value::String("dGVzdA==".to_owned()))]
  fn test_encode_to_generic<T>(#[case] value: T, #[case] expected: Value) -> Result<()>
  where
    T: serde::Serialize + std::fmt::Debug,
  {
    let packet = Packet::encode("test", value);
    println!("{:?}", packet);
    let res = packet.decode_value()?;
    assert_eq!(res, expected);
    Ok(())
  }

  #[rstest::rstest]
  #[case("dGVzdA==", b"test")]
  fn test_from_b64(#[case] value: &str, #[case] expected: &[u8]) -> Result<()> {
    let packet = Packet::encode("test", value);
    println!("{:?}", packet);
    let res = packet.decode_value()?;
    let bytes: Base64Bytes = serde_json::from_value(res).unwrap();
    assert_eq!(bytes, expected);
    Ok(())
  }
}
