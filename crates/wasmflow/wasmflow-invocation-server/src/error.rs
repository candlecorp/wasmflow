use thiserror::Error;

/// The RPC Error type.
#[derive(Error, Debug)]
pub enum Error {
  /// Error during the parsing of an IP address and port.
  #[error(transparent)]
  AddrParseError(#[from] std::net::AddrParseError),

  /// Upstream error from Tonic.
  #[error(transparent)]
  TranportFailure(#[from] tonic::transport::Error),

  /// Upstream error from [wasmflow_rpc].
  #[error(transparent)]
  RpcError(#[from] wasmflow_rpc::Error),

  /// Internal Error.
  #[error("Internal Error: {0}")]
  InternalError(String),

  /// Upstream Error from [wasmflow_sdk].
  #[error(transparent)]
  Sdk(#[from] wasmflow_sdk::v1::error::Error),

  /// Error used by collections.
  #[error("{0}")]
  CollectionError(String),

  /// Error generated by a collection's components.
  #[error("Component error: {0}")]
  ComponentError(String),

  /// Error sending output to channel.
  #[error("Error sending output to channel")]
  SendError,

  /// General Error.
  #[error("General error: {0}")]
  Other(String),
}

impl From<tokio::task::JoinError> for Error {
  fn from(e: tokio::task::JoinError) -> Self {
    Error::InternalError(format!("Tokio Error: {}", e))
  }
}

impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Error::InternalError(format!("IO Error: {}", e))
  }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
  fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
    Error::CollectionError(e.to_string())
  }
}

impl From<wasmflow_sdk::v1::error::Error> for Box<Error> {
  fn from(e: wasmflow_sdk::v1::error::Error) -> Self {
    Box::new(Error::Sdk(e))
  }
}