pub(crate) mod init_data;

use std::convert::TryInto;
use std::sync::Arc;

use futures::Stream;
use parking_lot::{Condvar, Mutex};
use vino_transport::{Invocation, TransportMap};

use crate::dev::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct InvocationMessage {
  inner: Invocation,
  core_data: InitData,
}

impl InvocationMessage {
  pub(crate) fn with_data(invocation: Invocation, data: InitData) -> Self {
    Self {
      inner: invocation,
      core_data: data,
    }
  }

  pub(crate) fn new(invocation: Invocation) -> Self {
    Self {
      inner: invocation,
      core_data: InitData::default(),
    }
  }

  pub(crate) fn into_inner(self) -> Invocation {
    self.inner
  }

  pub(crate) fn get_tx_id(&self) -> &str {
    &self.inner.tx_id
  }

  pub(crate) fn get_target(&self) -> &Entity {
    &self.inner.target
  }

  pub(crate) fn get_target_url(&self) -> String {
    self.inner.target.url()
  }

  pub(crate) fn get_origin(&self) -> &Entity {
    &self.inner.origin
  }

  pub(crate) fn get_payload(&self) -> &TransportMap {
    &self.inner.payload
  }

  pub(crate) fn get_init_data(&self) -> &InitData {
    &self.core_data
  }
}

impl From<Invocation> for InvocationMessage {
  fn from(inv: Invocation) -> Self {
    Self {
      inner: inv,
      core_data: InitData::default(),
    }
  }
}

impl TryFrom<InvocationMessage> for vino_rpc::rpc::Invocation {
  type Error = RuntimeError;
  fn try_from(inv: InvocationMessage) -> Result<Self, RuntimeError> {
    Ok(inv.inner.try_into()?)
  }
}

#[derive(Debug)]
#[must_use]
pub enum InvocationResponse {
  Stream { tx_id: String, rx: TransportStream },
  Error { tx_id: String, msg: String },
}

pub(crate) fn inv_error(tx_id: &str, msg: &str) -> InvocationResponse {
  InvocationResponse::error(tx_id.to_owned(), msg.to_owned())
}

impl InvocationResponse {
  /// Creates a successful invocation response stream. Response include the receiving end.
  /// of an unbounded channel to listen for future output.
  pub fn stream(tx_id: String, rx: impl Stream<Item = TransportWrapper> + Send + 'static) -> InvocationResponse {
    InvocationResponse::Stream {
      tx_id,
      rx: TransportStream::new(rx),
    }
  }

  /// Creates an error response.
  pub fn error(tx_id: String, msg: String) -> InvocationResponse {
    InvocationResponse::Error { tx_id, msg }
  }

  pub fn tx_id(&self) -> &str {
    match self {
      InvocationResponse::Stream { tx_id, .. } => tx_id,
      InvocationResponse::Error { tx_id, .. } => tx_id,
    }
  }

  pub fn ok(self) -> Result<TransportStream, InvocationError> {
    match self {
      InvocationResponse::Stream { rx, .. } => Ok(rx),
      InvocationResponse::Error { msg, .. } => Err(InvocationError(msg)),
    }
  }
}

#[derive(thiserror::Error, Debug)]
pub enum DispatchError {
  #[error("Thread died")]
  JoinFailed,
  #[error("{0}")]
  EntityFailure(String),
  #[error("{0}")]
  EntityNotAvailable(String),
  #[error("{0}")]
  CallFailure(String),
}

impl From<vino_entity::Error> for DispatchError {
  fn from(e: vino_entity::Error) -> Self {
    DispatchError::EntityFailure(e.to_string())
  }
}

impl From<ProviderError> for DispatchError {
  fn from(e: ProviderError) -> Self {
    DispatchError::CallFailure(e.to_string())
  }
}

#[allow(unused)]
pub(crate) async fn network_invoke_async(
  network_id: String,
  invocation: Invocation,
) -> Result<Vec<TransportWrapper>, DispatchError> {
  let network = NetworkService::for_id(&network_id);

  let rcpt = network
    .get_recipient(&invocation.target)
    .map_err(|e| DispatchError::EntityNotAvailable(e.to_string()))?;

  let response = rcpt.invoke(InvocationMessage::new(invocation))?.await?;
  match response {
    InvocationResponse::Stream { rx, .. } => {
      let packets: Vec<TransportWrapper> = rx.collect().await;
      trace!("PROV:WASM:LINK_CALL:RESPONSE[{} packets]", packets.len());
      debug!("PROV:WASM:LINK_CALL:RESPONSE:{:?}", packets);
      Ok(packets)
    }
    InvocationResponse::Error { msg, .. } => Err(DispatchError::CallFailure(msg)),
  }
}

#[allow(unused)]
pub(crate) fn network_invoke_sync(
  network_id: String,
  invocation: Invocation,
) -> Result<Vec<TransportWrapper>, DispatchError> {
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  let inner = Arc::clone(&pair);

  let handle = std::thread::spawn(move || {
    let system = tokio::runtime::Runtime::new().unwrap();
    let (lock, cvar) = &*inner;
    let mut started = lock.lock();
    *started = true;
    let re = system.block_on(network_invoke_async(network_id, invocation));
    cvar.notify_one();
    re
  });

  let (lock, cvar) = &*pair;
  let mut started = lock.lock();
  while !*started {
    std::thread::yield_now();
    cvar.wait(&mut started);
  }

  let packets = handle.join().map_err(|_| DispatchError::JoinFailed)??;
  Ok(packets)
}

#[cfg(test)]
mod tests {

  use tokio::sync::oneshot;

  use super::*;
  use crate::test::prelude::{assert_eq, *};
  #[test_logger::test(tokio::test)]
  async fn invoke_async() -> TestResult<()> {
    let (_, nuid) = init_network_from_yaml("./manifests/v0/echo.yaml").await?;

    let target = Entity::component("self", "echo");
    let map = TransportMap::from(vec![("input", "hello")]);
    let invocation = Invocation::new_test(file!(), target, map);

    let packets = network_invoke_async(nuid, invocation).await?;
    debug!("{:?}", packets);
    assert_eq!(packets.len(), 1);
    let rv: String = packets[0].payload.clone().deserialize()?;
    assert_eq!(rv, "hello");

    Ok(())
  }

  #[test_logger::test(tokio::test)]
  async fn invoke_sync() -> TestResult<()> {
    let (tx, rx) = oneshot::channel::<String>();
    let (tx2, rx2) = oneshot::channel::<bool>();
    std::thread::spawn(|| {
      let system = tokio::runtime::Runtime::new().unwrap();

      let (_, nuid) = system
        .block_on(init_network_from_yaml("./manifests/v0/echo.yaml"))
        .unwrap();
      let _ = tx.send(nuid);
      let _ = system.block_on(rx2);
    });
    let nuid = rx.await?;

    let target = Entity::component("self", "echo");
    let map = TransportMap::from(vec![("input", "hello")]);
    let invocation = Invocation::new_test(file!(), target, map);

    let packets = network_invoke_sync(nuid, invocation)?;
    let _ = tx2.send(true);

    debug!("{:?}", packets);
    assert_eq!(packets.len(), 1);
    let rv: String = packets[0].payload.clone().deserialize()?;
    assert_eq!(rv, "hello");

    Ok(())
  }
}
