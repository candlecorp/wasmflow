use thiserror::Error;

/// The RPC Error type
#[derive(Error, Debug)]
pub enum RpcError {
  /// Error during the parsing of an IP address and port
  #[error(transparent)]
  AddrParseError(#[from] std::net::AddrParseError),
  /// Upstream error from Tonic
  #[error(transparent)]
  TransportError(#[from] tonic::transport::Error),

  /// Internal Error
  #[error("Internal Error: {0}")]
  InternalError(String),

  /// Upstream Error from [vino_entity]
  #[error(transparent)]
  EntityError(#[from] vino_entity::Error),

  /// Invalid [crate::rpc::OutputKind]
  #[error("Invalid output kind {0}")]
  InvalidOutputKind(i32),

  /// Error used by providers
  #[error("Error: {0}")]
  ProviderError(String),

  /// Error generated by a provider's components
  #[error("Error: {0}")]
  ComponentError(String),

  /// General Error
  #[error("General error : {0}")]
  Other(String),
}

impl From<tokio::task::JoinError> for RpcError {
  fn from(e: tokio::task::JoinError) -> Self {
    RpcError::InternalError(format!("Tokio Error: {}", e))
  }
}

impl From<std::io::Error> for RpcError {
  fn from(e: std::io::Error) -> Self {
    RpcError::InternalError(format!("IO Error: {}", e))
  }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for RpcError {
  fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
    RpcError::ProviderError(e.to_string())
  }
}

impl From<vino_entity::Error> for Box<RpcError> {
  fn from(e: vino_entity::Error) -> Self {
    Box::new(RpcError::EntityError(e))
  }
}
