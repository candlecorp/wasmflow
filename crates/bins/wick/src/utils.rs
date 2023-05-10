use std::time::Duration;

use futures::StreamExt;
use wick_component_cli::options::DefaultCliOptions;
use wick_config::config::{AssetReference, ComponentConfiguration, HttpConfigBuilder};
use wick_logger::{LoggingGuard, LoggingOptions};
use wick_packet::PacketStream;

pub(crate) fn merge_config(
  def: &ComponentConfiguration,
  local_cli_opts: &crate::oci::Options,
  server_cli_opts: Option<DefaultCliOptions>,
) -> ComponentConfiguration {
  let mut merged_manifest = def.clone();
  let host_config = merged_manifest.host_mut();
  host_config.set_allow_latest(local_cli_opts.allow_latest || host_config.allow_latest());
  host_config.set_insecure_registries(
    vec![
      host_config.insecure_registries().to_vec(),
      local_cli_opts.insecure_registries.clone(),
    ]
    .concat(),
  );

  if let Some(cli_opts) = server_cli_opts {
    if let Some(to) = cli_opts.timeout {
      log_override("timeout", host_config.timeout_mut(), Duration::from_millis(to));
    }
    #[allow(clippy::option_if_let_else)]
    if let Some(manifest_opts) = host_config.rpc_mut().as_mut() {
      if !manifest_opts.enabled() {
        log_override("rpc.enabled", manifest_opts.enabled_mut(), cli_opts.rpc_enabled);
      }
      if let Some(to) = cli_opts.rpc_address {
        log_override("rpc.address", manifest_opts.address_mut(), Some(to));
      }
      if let Some(to) = cli_opts.rpc_port {
        log_override("rpc.port", manifest_opts.port_mut(), Some(to));
      }
      if let Some(to) = cli_opts.rpc_pem {
        log_override("rpc.pem", manifest_opts.pem_mut(), Some(AssetReference::new(to)));
      }
      if let Some(to) = cli_opts.rpc_ca {
        log_override("rpc.ca", manifest_opts.ca_mut(), Some(AssetReference::new(to)));
      }
      if let Some(to) = cli_opts.rpc_key {
        log_override("rpc.key", manifest_opts.key_mut(), Some(AssetReference::new(to)));
      }
    } else {
      host_config.set_rpc(
        HttpConfigBuilder::default()
          .enabled(cli_opts.rpc_enabled)
          .port(cli_opts.rpc_port)
          .address(cli_opts.rpc_address)
          .pem(cli_opts.rpc_pem.map(AssetReference::new))
          .key(cli_opts.rpc_key.map(AssetReference::new))
          .ca(cli_opts.rpc_ca.map(AssetReference::new))
          .build()
          .unwrap(),
      );
    };
  }

  merged_manifest
}

fn log_override<T: std::fmt::Debug>(field: &str, from: &mut T, to: T) {
  debug!(%field, ?from, ?to, "overriding manifest value");
  *from = to;
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn init_logger(opts: &LoggingOptions) -> crate::Result<LoggingGuard> {
  Ok(wick_logger::init(&opts.name(crate::BIN_NAME)))
}

pub(crate) async fn print_stream_json(
  mut stream: PacketStream,
  filter: &[String],
  _terse: bool,
  raw: bool,
) -> crate::Result<()> {
  if !filter.is_empty() {
    trace!("filtering only {:?}", filter);
  }
  while let Some(Ok(packet)) = stream.next().await {
    trace!(message = ?packet, "output");
    if (packet.is_done()) && !raw {
      continue;
    }
    if !filter.is_empty() && !filter.iter().any(|name| name == packet.port()) {
      tracing::debug!(port = %packet.port(), "filtering out");
      continue;
    }
    println!("{}", packet.to_json());
  }
  trace!("stream complete");
  Ok(())
}
