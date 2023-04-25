use crate::PacketError;

/// Errors originating from WASM components.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
  /// Thrown when a user attempts to retrieve a stream for a port that doesn't exist.
  #[error("No stream found for port '{0}'")]
  PortMissing(String),

  /// Error serializing payload.
  #[error("Error serializing payload: {1} (payload was: {:?}",.0)]
  Encode(Vec<u8>, String),

  /// Error deserializing payload.
  #[error("Error deserializing  payload: {1} (payload was: {:?}",.0)]
  Decode(Vec<u8>, String),

  /// Error converting payload into JSON.
  #[error("Error JSON-ifying payload: {0}")]
  Jsonify(String),

  /// Error communicating over a stream or channel.
  #[error("Error communicating over a stream or channel: {0}")]
  Channel(String),

  /// General error to wrap other errors.
  #[error("{0}")]
  General(String),

  /// Payload was successful but no data was provided.
  #[error("No data in payload")]
  NoData,

  /// An error that wraps a PayloadError.
  #[error("{}", .0.msg())]
  PayloadError(PacketError),

  /// Thrown when a user attempts to use a signal when they expected a payload.
  #[error("Got a Done signal in an unexpected context.")]
  UnexpectedDone,

  /// Couldn't retrieve a complete set of packets from a [crate::StreamMap]
  #[error("Could not retrieve a complete set of packets. Stream '{0}' failed to provide a packet: '{1}'")]
  StreamMapError(String /* port */, String /* error */),

  /// Couldn't retrieve a complete set of packets from a [crate::StreamMap]
  #[error("Could not retrieve a complete set of packets. Stream '{0}' completed or failed before providing a packet.")]
  StreamMapMissing(String /* port */),
}

impl From<wasmrs_rx::Error> for Error {
  fn from(value: wasmrs_rx::Error) -> Self {
    Self::Channel(value.to_string())
  }
}

impl From<Box<dyn std::error::Error>> for Error {
  fn from(value: Box<dyn std::error::Error>) -> Self {
    Self::General(value.to_string())
  }
}

#[derive(thiserror::Error, Debug)]

/// The error type for Wick Entities.
pub enum ParseError {
  /// Encountered an invalid scheme when parsing an entity URL.
  #[error("Invalid scheme {0}")]
  Scheme(String),
  /// No authority/host supplied in the entity URL.
  #[error("Missing authority/host")]
  Authority,
  /// Invalid authority/host supplied in the entity URL.
  #[error("Invalid authority/host '{0}', missing separator '.'")]
  InvalidAuthority(String),
  /// Invalid authority/host kind.
  #[error("Invalid authority/host kind '{0}'")]
  InvalidAuthorityKind(String),
  /// Error parsing an entity URL.
  #[error("{0}")]
  Parse(url::ParseError),
  /// Error converting arguments into an [crate::Entity].
  #[error(transparent)]
  Conversion(Box<dyn std::error::Error + Send + Sync>),
}
