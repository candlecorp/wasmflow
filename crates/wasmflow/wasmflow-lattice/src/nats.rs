use std::path::PathBuf;
use std::time::Duration;

use futures::future::BoxFuture;
use futures::FutureExt;
use nats::asynk::{Connection, Message, Subscription};
use serde::Deserialize;
use tokio::time::timeout;
use wasmflow_codec::messagepack::{deserialize, serialize};

use crate::error::LatticeError;
use crate::LatticeRpcResponse;

type Result<T> = std::result::Result<T, LatticeError>;

#[derive(Debug, Clone)]
pub struct NatsOptions {
  pub address: String,
  pub client_id: String,
  pub creds_path: Option<PathBuf>,
  pub token: Option<String>,
  pub timeout: Duration,
}

#[derive(Clone, Debug)]
pub(crate) struct Nats {
  nc: Connection,
  timeout: Duration,
}

impl Nats {
  pub(crate) async fn connect(nopts: NatsOptions) -> Result<Self> {
    let opts = if let Some(creds_path) = &nopts.creds_path {
      nats::asynk::Options::with_credentials(creds_path)
    } else if let Some(token) = &nopts.token {
      nats::asynk::Options::with_token(token)
    } else {
      nats::asynk::Options::new()
    };

    let timeout = nopts.timeout;
    trace!(
      "LATTICE:CONNECT[{}]:ID[{}]:TIMEOUT[{:?}]",
      nopts.address,
      nopts.client_id,
      nopts.timeout
    );
    let nc = opts
      .with_name(&nopts.client_id)
      .connect(&nopts.address)
      .await
      .map_err(LatticeError::ConnectionFailed)?;

    Ok(Self { nc, timeout })
  }

  pub(crate) async fn disconnect(&self) -> Result<()> {
    self.nc.drain().await.map_err(LatticeError::ShutdownError)?;
    self.nc.flush().await.map_err(LatticeError::ShutdownError)?;
    Ok(())
  }

  pub(crate) async fn flush(&self) -> Result<()> {
    self.nc.flush().await.map_err(LatticeError::ShutdownError)
  }

  pub(crate) async fn request(&self, topic: &str, payload: &[u8]) -> Result<NatsSubscription> {
    trace!(topic, ?payload, "lattice request");
    let sub = self
      .nc
      .request_multi(topic, payload)
      .await
      .map_err(|e| LatticeError::RequestFail(e.to_string()))?;
    Ok(NatsSubscription {
      inner: sub,
      timeout: self.timeout,
    })
  }

  #[allow(unused)]
  pub(crate) async fn subscribe(&self, topic: String) -> Result<NatsSubscription> {
    let sub = self
      .nc
      .subscribe(&topic)
      .await
      .map_err(|e| LatticeError::PublishFail(e.to_string()))?;
    Ok(NatsSubscription {
      inner: sub,
      timeout: self.timeout,
    })
  }

  pub(crate) async fn queue_subscribe(&self, topic: String, group: String) -> Result<NatsSubscription> {
    trace!(%topic, %group, "lattice subscribe");
    let sub = self
      .nc
      .queue_subscribe(&topic, &group)
      .await
      .map_err(|e| LatticeError::PublishFail(e.to_string()))?;
    Ok(NatsSubscription {
      inner: sub,
      timeout: self.timeout,
    })
  }
}

pub(crate) struct NatsSubscription {
  inner: Subscription,
  timeout: Duration,
}

impl NatsSubscription {
  pub(crate) fn next(&self) -> BoxFuture<Result<Option<NatsMessage>>> {
    let fut = self.inner.next();
    timeout(self.timeout, fut.map(|msg| msg.map(|msg| NatsMessage { inner: msg })))
      .map(|r| r.map_err(LatticeError::WaitTimeout))
      .boxed()
  }
  pub(crate) fn next_wait(&self) -> BoxFuture<Option<NatsMessage>> {
    self
      .inner
      .next()
      .map(|msg| msg.map(|msg| NatsMessage { inner: msg }))
      .boxed()
  }
}

#[derive(Debug)]
pub(crate) struct NatsMessage {
  inner: Message,
}

impl From<Message> for NatsMessage {
  fn from(msg: Message) -> Self {
    Self { inner: msg }
  }
}

impl NatsMessage {
  pub(crate) async fn respond(&self, response: &LatticeRpcResponse) -> Result<()> {
    let data = serialize(response).unwrap_or_else(|e| serialize(&LatticeRpcResponse::Error(e.to_string())).unwrap());
    trace!(
      target = %self.inner.reply.as_ref().unwrap_or(&"".to_owned()),
      ?data,
      "lattice respond"
    );
    let msg = self.inner.clone();
    msg.respond(data).await.map_err(LatticeError::ResponseFail)
  }

  pub(crate) fn data(&self) -> &[u8] {
    &self.inner.data
  }

  pub(crate) fn deserialize<'de, T: Deserialize<'de>>(&'de self) -> Result<T> {
    deserialize(&self.inner.data).map_err(|e| LatticeError::MessageDeserialization(e.to_string()))
  }
}
