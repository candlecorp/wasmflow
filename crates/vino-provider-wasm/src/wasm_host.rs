use std::collections::{
  HashSet,
  VecDeque,
};
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;

use parking_lot::Mutex;
use tokio::sync::mpsc::unbounded_channel;
use vino_component::v0::Payload;
use vino_component::Packet;
use vino_provider::OutputSignal;
use vino_transport::{
  MessageTransportStream,
  TransportWrapper,
};
use vino_types::signatures::ComponentSignature;
use vino_wascap::{
  Claims,
  ComponentClaims,
};
use wapc::WapcHost;

use crate::wapc_module::WapcModule;
use crate::{
  Error,
  Result,
};

type PortBuffer = VecDeque<(String, Packet)>;

#[derive(Debug)]
pub struct WasmHost {
  host: WapcHost,
  claims: Claims<ComponentClaims>,
  buffer: Arc<Mutex<PortBuffer>>,
  closed_ports: Arc<Mutex<HashSet<String>>>,
}

impl TryFrom<&WapcModule> for WasmHost {
  type Error = Error;

  fn try_from(module: &WapcModule) -> Result<Self> {
    let jwt = &module.token.jwt;

    // Ensure that the JWT we found on this actor is valid, not expired, can be used,
    // has a verified signature, etc.
    vino_wascap::validate_token::<ComponentClaims>(jwt).map_err(Error::ClaimsError)?;

    let time = Instant::now();
    #[cfg(feature = "wasmtime")]
    #[allow(unused)]
    let engine = {
      let engine = wasmtime_provider::WasmtimeEngineProvider::new(&module.bytes, None);
      trace!(
        "PRV:WASM:Wasmtime thread loaded in {} μs",
        time.elapsed().as_micros()
      );
      engine
    };
    #[cfg(feature = "wasm3")]
    #[allow(unused)]
    let engine = {
      let engine = wasm3_provider::Wasm3EngineProvider::new(&module.bytes);
      trace!(
        "PRV:WASM:wasm3 thread loaded in {} μs",
        time.elapsed().as_micros()
      );
      engine
    };

    let engine = Box::new(engine);
    let buffer = Arc::new(Mutex::new(PortBuffer::new()));
    let buffer_inner = buffer.clone();
    let closed_ports = Arc::new(Mutex::new(HashSet::new()));
    let ports_inner = closed_ports.clone();

    let host = WapcHost::new(engine, move |_id, _inv_id, port, output_signal, payload| {
      trace!("PRV:WASM:WAPC_CALLBACK:{:?}", payload);
      let mut ports_locked = ports_inner.lock();
      let mut buffer_locked = buffer_inner.lock();
      match OutputSignal::from_str(output_signal) {
        Ok(signal) => match signal {
          OutputSignal::Output => {
            if ports_locked.contains(port) {
              Err("Closed".into())
            } else {
              buffer_locked.push_back((port.to_owned(), payload.into()));
              Ok(vec![])
            }
          }
          OutputSignal::OutputDone => {
            if ports_locked.contains(port) {
              Err("Closed".into())
            } else {
              buffer_locked.push_back((port.to_owned(), payload.into()));
              buffer_locked.push_back((port.to_owned(), Packet::V0(Payload::Done)));
              ports_locked.insert(port.to_owned());
              Ok(vec![])
            }
          }
          OutputSignal::Done => {
            buffer_locked.push_back((port.to_owned(), Packet::V0(Payload::Done)));
            ports_locked.insert(port.to_owned());
            Ok(vec![])
          }
        },
        Err(_) => Err("Invalid signal".into()),
      }
    })?;

    info!(
      "Wasmtime thread initialized in {} μs",
      time.elapsed().as_micros()
    );
    Ok(Self {
      claims: module.claims().clone(),
      host,
      buffer,
      closed_ports,
    })
  }
}

impl WasmHost {
  pub fn call(&mut self, component_name: &str, payload: &[u8]) -> Result<MessageTransportStream> {
    {
      self.buffer.lock().clear();
      self.closed_ports.lock().clear();
    }
    trace!("PRV:WASM:INVOKE:{}:START", component_name);
    let _result = self.host.call(component_name, payload)?;
    trace!("PRV:WASM:INVOKE:{}:FINISH", component_name);
    let (tx, rx) = unbounded_channel();
    let mut locked = self.buffer.lock();
    while let Some((port, payload)) = locked.pop_front() {
      let transport = TransportWrapper {
        port,
        payload: payload.into(),
      };
      tx.send(transport).map_err(|_| Error::SendError)?;
    }

    Ok(MessageTransportStream::new(rx))
  }

  pub fn get_components(&self) -> &Vec<ComponentSignature> {
    let claims = &self.claims;
    let components = &claims.metadata.as_ref().unwrap().interface.components;
    components
  }
}
