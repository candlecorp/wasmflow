use std::convert::{TryFrom, TryInto};
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use crate::error::ManifestError;
use crate::Result;

#[derive(Debug, Clone, Default)]
#[must_use]
/// Configuration options for the host to use at startup.
pub struct HostConfig {
  /// Flag to allow/disallow `:latest` tags for OCI artifacts.
  pub allow_latest: bool,

  /// The list of registries to connect via HTTP rather than HTTPS.
  pub insecure_registries: Vec<String>,

  /// The timeout for network requests.
  pub timeout: Duration,

  /// The host ID.
  pub id: Option<String>,

  /// Configuration for the Mesh.
  pub mesh: Option<MeshConfig>,

  /// Configuration for the GRPC server.
  pub rpc: Option<HttpConfig>,
}

#[derive(Debug, Default, Clone)]
/// Configuration for HTTP/S servers.
pub struct HttpConfig {
  /// Enable/disable the server.
  pub enabled: bool,

  /// The port to bind to.
  pub port: Option<u16>,

  /// The address to bind to.
  pub address: Option<Ipv4Addr>,

  /// Path to pem file for TLS.
  pub pem: Option<PathBuf>,

  /// Path to key file for TLS.
  pub key: Option<PathBuf>,

  /// Path to CA file.
  pub ca: Option<PathBuf>,
}

#[derive(Debug, Default, Clone)]
/// Configuration used to connect to the mesh.
pub struct MeshConfig {
  /// Enable/disable the mesh connection.
  pub enabled: bool,

  /// The address of the NATS server.
  pub address: String,

  /// The path to the NATS credsfile.
  pub creds_path: Option<PathBuf>,

  /// The NATS token.
  pub token: Option<String>,
}

impl TryFrom<crate::v0::HostConfig> for HostConfig {
  type Error = ManifestError;
  fn try_from(def: crate::v0::HostConfig) -> Result<Self> {
    Ok(Self {
      allow_latest: def.allow_latest,
      insecure_registries: def.insecure_registries,
      timeout: Duration::from_millis(def.timeout),
      id: def.id,
      mesh: def.mesh.and_then(|v| v.try_into().ok()),
      rpc: def.rpc.and_then(|v| v.try_into().ok()),
    })
  }
}

impl TryFrom<crate::v1::HostConfig> for HostConfig {
  type Error = ManifestError;
  fn try_from(def: crate::v1::HostConfig) -> Result<Self> {
    Ok(Self {
      allow_latest: def.allow_latest,
      insecure_registries: def.insecure_registries,
      timeout: Duration::from_millis(def.timeout),
      id: def.id,
      mesh: def.mesh.and_then(|v| v.try_into().ok()),
      rpc: def.rpc.and_then(|v| v.try_into().ok()),
    })
  }
}

impl TryFrom<crate::v0::MeshConfig> for MeshConfig {
  type Error = ManifestError;
  fn try_from(def: crate::v0::MeshConfig) -> Result<Self> {
    Ok(Self {
      enabled: def.enabled,
      address: def.address,
      creds_path: opt_str_to_pathbuf(&def.creds_path)?,
      token: def.token,
    })
  }
}

impl TryFrom<crate::v1::MeshConfig> for MeshConfig {
  type Error = ManifestError;
  fn try_from(def: crate::v1::MeshConfig) -> Result<Self> {
    Ok(Self {
      enabled: def.enabled,
      address: def.address,
      creds_path: opt_str_to_pathbuf(&def.creds_path)?,
      token: def.token,
    })
  }
}

impl TryFrom<crate::v0::HttpConfig> for HttpConfig {
  type Error = ManifestError;
  fn try_from(def: crate::v0::HttpConfig) -> Result<Self> {
    Ok(Self {
      enabled: def.enabled,
      port: def.port,
      address: opt_str_to_ipv4addr(&def.address)?,
      pem: opt_str_to_pathbuf(&def.pem)?,
      key: opt_str_to_pathbuf(&def.key)?,
      ca: opt_str_to_pathbuf(&def.ca)?,
    })
  }
}

impl TryFrom<crate::v1::HttpConfig> for HttpConfig {
  type Error = ManifestError;
  fn try_from(def: crate::v1::HttpConfig) -> Result<Self> {
    Ok(Self {
      enabled: def.enabled,
      port: def.port,
      address: opt_str_to_ipv4addr(&def.address)?,
      pem: opt_str_to_pathbuf(&def.pem)?,
      key: opt_str_to_pathbuf(&def.key)?,
      ca: opt_str_to_pathbuf(&def.ca)?,
    })
  }
}

fn opt_str_to_pathbuf(v: &Option<String>) -> Result<Option<PathBuf>> {
  Ok(match v {
    Some(v) => Some(PathBuf::from_str(v).map_err(|e| ManifestError::BadPath(e.to_string()))?),
    None => None,
  })
}

fn opt_str_to_ipv4addr(v: &Option<String>) -> Result<Option<Ipv4Addr>> {
  Ok(match v {
    Some(v) => Some(Ipv4Addr::from_str(v).map_err(|e| ManifestError::BadIpAddress(e.to_string()))?),
    None => None,
  })
}
