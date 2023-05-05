use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;

use flow_component::RuntimeCallback;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use wasmrs::{GenericError, OperationHandler, RSocket, RawPayload};
use wasmrs_codec::messagepack::serialize;
use wasmrs_host::{CallContext, Host, WasiParams};
use wasmrs_rx::{FluxChannel, Observer};
use wick_interface_types::ComponentSignature;
use wick_packet::{from_raw_wasmrs, from_wasmrs, into_wasmrs, ComponentReference, Entity, Invocation, PacketStream};
use wick_wascap::{Claims, CollectionClaims};

use crate::error::WasmCollectionError;
use crate::wasm_module::WickWasmModule;
use crate::{Error, Result};

#[must_use]
pub struct WasmHostBuilder {
  wasi_params: Option<WasiParams>,
  callback: Option<Arc<RuntimeCallback>>,
  min_threads: usize,
  max_threads: usize,
}

impl std::fmt::Debug for WasmHostBuilder {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("WasmHostBuilder")
      .field("wasi_params", &self.wasi_params)
      .finish()
  }
}

impl WasmHostBuilder {
  pub fn new() -> Self {
    Self {
      wasi_params: None,
      callback: None,
      min_threads: 1,
      max_threads: 1,
    }
  }

  pub fn wasi_params(mut self, params: WasiParams) -> Self {
    self.wasi_params = Some(params);
    self
  }

  pub fn link_callback(mut self, callback: Arc<RuntimeCallback>) -> Self {
    self.callback = Some(callback);
    self
  }

  pub fn preopened_dirs(mut self, dirs: Vec<String>) -> Self {
    let mut params = self.wasi_params.take().unwrap_or_default();
    params.preopened_dirs = dirs;
    self.wasi_params.replace(params);
    self
  }

  pub fn build(self, module: &WickWasmModule) -> Result<WasmHost> {
    WasmHost::try_load(
      module,
      self.wasi_params,
      &self.callback,
      self.min_threads,
      self.max_threads,
    )
  }

  pub fn max_threads(mut self, max_threads: usize) -> Self {
    self.max_threads = max_threads;
    self
  }

  pub fn min_threads(mut self, min_threads: usize) -> Self {
    self.min_threads = min_threads;
    self
  }
}

impl Default for WasmHostBuilder {
  fn default() -> Self {
    Self::new()
  }
}

#[derive()]
pub struct WasmHost {
  #[allow(unused)]
  host: Arc<Mutex<Host>>,
  claims: Claims<CollectionClaims>,
  ctx: Arc<CallContext>,
  _rng: seeded_random::Random,
}

impl std::fmt::Debug for WasmHost {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("WasmHost").field("claims", &self.claims).finish()
  }
}

impl WasmHost {
  pub fn try_load(
    module: &WickWasmModule,
    wasi_options: Option<WasiParams>,
    callback: &Option<Arc<RuntimeCallback>>,
    _min_threads: usize,
    _max_threads: usize,
  ) -> Result<Self> {
    let jwt = &module.token.jwt;

    wick_wascap::validate_token::<CollectionClaims>(jwt).map_err(|e| Error::ClaimsInvalid(e.to_string()))?;

    let time = Instant::now();

    let engine = wasmrs_wasmtime::WasmtimeBuilder::new(&module.bytes).enable_cache(None);
    let engine = if let Some(wasi_options) = wasi_options {
      engine.wasi_params(wasi_options)
    } else {
      engine
    };
    let engine = engine
      .build()
      .map_err(|e| WasmCollectionError::EngineFailure(e.to_string()))?;
    trace!(duration_μs = %time.elapsed().as_micros(), "wasmtime instance loaded");

    let host = Host::new(engine).map_err(|e| WasmCollectionError::EngineFailure(e.to_string()))?;

    debug!(duration_μs = ?time.elapsed().as_micros(), "wasmtime initialize");
    if let Some(callback) = callback {
      let index = host.register_request_channel("wick", "__callback", make_host_callback(callback));
      trace!(index, "wasmrs callback index");
    }
    let ctx = host.new_context(128 * 1024, 128 * 1024).unwrap();

    Ok(Self {
      claims: module.claims().clone(),
      host: Arc::new(Mutex::new(host)),
      ctx: Arc::new(ctx),
      _rng: seeded_random::Random::new(),
    })
  }

  #[allow(clippy::needless_pass_by_value)]
  pub fn call(
    &self,
    invocation: Invocation,
    stream: PacketStream,
    config: Option<wick_packet::OperationConfig>,
  ) -> Result<PacketStream> {
    let component_name = invocation.target.operation_id();
    debug!(component = component_name, "wasm invoke");
    let seed = invocation.seed();
    let now = Instant::now();
    let ctx = self.ctx.clone();
    let index = ctx
      .get_export("wick", component_name)
      .map_err(|_| crate::Error::OperationNotFound(component_name.to_owned(), ctx.get_exports()))?;
    if let Some(config) = config {
      stream.set_context(config, seed);
    }

    let s = into_wasmrs(index, stream);
    let out = ctx.request_channel(Box::pin(s));
    trace!(
      component = component_name,
      duration_μs = ?now.elapsed().as_micros(),
      "wasm call finished"
    );
    Ok(from_raw_wasmrs(out))
  }

  pub async fn setup(&self, provided: SetupPayload) -> Result<()> {
    debug!("wasm setup");

    let ctx = self.ctx.clone();
    let index = ctx
      .get_export("wick", "__setup")
      .map_err(|_| crate::Error::SetupOperation)?;
    let metadata = wasmrs::Metadata::new(index);
    let data = serialize(&provided).unwrap();
    let payload = RawPayload::new(metadata.encode(), data.into());
    match ctx.request_response(payload).await {
      Ok(_) => {
        debug!("setup finished");
      }
      Err(e) => {
        error!("setup failed: {}", e);
        return Err(Error::Setup(e));
      }
    }

    trace!("wasm setup finished");
    Ok(())
  }

  pub fn get_operations(&self) -> &ComponentSignature {
    let claims = &self.claims;
    &claims.metadata.as_ref().unwrap().interface
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use]
struct InvocationPayload {
  reference: ComponentReference,
  operation: String,
}

fn make_host_callback(
  rt_cb: &Arc<RuntimeCallback>,
) -> OperationHandler<wasmrs::IncomingStream, wasmrs::OutgoingStream> {
  let cb = rt_cb.clone();
  let func = move |mut incoming: wasmrs::IncomingStream| -> std::result::Result<wasmrs::OutgoingStream, GenericError> {
    use tokio_stream::StreamExt;
    let (tx, rx) = FluxChannel::new_parts();
    let cb = cb.clone();
    tokio::spawn(async move {
      let first = incoming.next().await;
      let meta = if let Some(Ok(first)) = first {
        match wasmrs_codec::messagepack::deserialize::<InvocationPayload>(&first.data) {
          Ok(p) => p,
          Err(e) => {
            error!("bad component ref invocation: {}", e);
            let _ = tx.error(wick_packet::Error::component_error(e.to_string()));
            return;
          }
        }
      } else {
        error!("bad component ref invocation: no payload");
        let _ = tx.error(wick_packet::Error::component_error("no payload"));
        return;
      };
      let stream = from_wasmrs(incoming);
      match cb(meta.reference, meta.operation, stream, None, None).await {
        Ok(mut response) => {
          while let Some(p) = response.next().await {
            let _ = tx.send_result(p);
          }
        }
        Err(e) => {
          error!("bad component ref invocation: {}", e);
          let _ = tx.error(wick_packet::Error::component_error(e.to_string()));
        }
      }
    });
    Ok(into_wasmrs(0, PacketStream::new(Box::new(rx))))
  };
  Box::new(func)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use]
pub struct SetupPayload {
  provided: HashMap<String, ComponentReference>,
}

impl SetupPayload {
  pub fn new(origin: &Entity, provided: HashMap<String, String>) -> Self {
    let provided = provided
      .into_iter()
      .map(|(k, v)| {
        (
          k,
          ComponentReference::new(origin.clone(), Entity::from_str(&v).unwrap()),
        )
      })
      .collect();
    Self { provided }
  }
}
