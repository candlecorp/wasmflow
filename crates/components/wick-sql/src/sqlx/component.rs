#![allow(unused_imports)]
use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use flow_component::{BoxFuture, Component, ComponentError, RuntimeCallback};
use futures::stream::BoxStream;
use futures::{Stream, StreamExt};
use parking_lot::Mutex;
use serde_json::Value;
use sqlx::{PgPool, SqlitePool};
use url::Url;
use wick_config::config::components::{ComponentConfig, OperationConfig, SqlComponentConfig, SqlOperationKind};
use wick_config::config::{ErrorBehavior, Metadata};
use wick_config::{ConfigValidation, Resolver};
use wick_interface_types::{ComponentSignature, Field, OperationSignatures, Type};
use wick_packet::{FluxChannel, Invocation, Observer, Packet, PacketStream, RuntimeConfig};

use crate::common::sql_wrapper::SqlWrapper;
use crate::component::{ClientConnection, Connection, DatabaseProvider};
use crate::sqlx::{postgres, sqlite};
use crate::{common, Error};

pub(crate) struct QueryResult<'a> {
  stream: BoxStream<'a, Result<Value, Error>>,
}

impl<'a> QueryResult<'a> {
  pub(crate) fn new(stream: BoxStream<'a, Result<Value, Error>>) -> Self {
    Self { stream }
  }
}

impl<'a> Stream for QueryResult<'a> {
  type Item = Result<Value, Error>;

  fn poll_next(
    mut self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Option<Self::Item>> {
    self.stream.poll_next_unpin(cx)
  }
}

#[derive(Debug, Clone)]
enum CtxPool {
  Postgres(PgPool),
  SqlLite(SqlitePool),
}

impl CtxPool {
  fn run_query<'a, 'b>(&'a self, querystr: &'b str, args: Vec<SqlWrapper>) -> BoxStream<'a, Result<Value, Error>>
  where
    'b: 'a,
  {
    match self {
      CtxPool::Postgres(c) => {
        let mut query = sqlx::query(querystr);
        for arg in args {
          trace!(?arg, "binding arg");
          query = query.bind(arg);
        }
        let stream = query.fetch(c).map(|res| res.map(postgres::SerMapRow::from)).map(|res| {
          res
            .map(|el| serde_json::to_value(el).unwrap_or(Value::Null))
            .map_err(|e| Error::Fetch(e.to_string()))
        });

        stream.boxed()
      }
      CtxPool::SqlLite(c) => {
        let mut query = sqlx::query(querystr);
        for arg in args {
          trace!(?arg, "binding arg");
          query = query.bind(arg);
        }
        let stream = query.fetch(c).map(|res| res.map(sqlite::SerMapRow::from)).map(|res| {
          res
            .map(|el| serde_json::to_value(el).unwrap_or(Value::Null))
            .map_err(|e| Error::Fetch(e.to_string()))
        });

        stream.boxed()
      }
    }
  }

  async fn run_exec<'a, 'q>(&'a self, query: &'q str, args: Vec<SqlWrapper>) -> Result<u64, Error>
  where
    'q: 'a,
  {
    match self {
      CtxPool::Postgres(c) => {
        let mut query = sqlx::query(query);
        for arg in args {
          trace!(?arg, "binding arg");
          query = query.bind(arg);
        }

        query
          .execute(c)
          .await
          .map(|r| r.rows_affected())
          .map_err(|e| Error::Exec(e.to_string()))
      }
      CtxPool::SqlLite(c) => {
        let mut query = sqlx::query(query);
        for arg in args {
          trace!(?arg, "binding arg");
          query = query.bind(arg);
        }
        query
          .execute(c)
          .await
          .map(|r| r.rows_affected())
          .map_err(|e| Error::Exec(e.to_string()))
      }
    }
  }
}

#[derive(Clone)]
pub(crate) struct Context {
  db: CtxPool,
}

impl std::fmt::Debug for Context {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Context").finish()
  }
}

impl Context {
  async fn get<'a>(&'a self) -> Result<Connection<'a>, Error> {
    let conn = self.db.clone();
    Ok(Connection::new(Box::new(conn)))
  }
}

/// The SQLx component.
#[derive(Debug, Clone)]
#[must_use]
pub(crate) struct SqlXComponent {
  context: Context,
  signature: Arc<ComponentSignature>,
  url: Url,
  config: SqlComponentConfig,
  #[allow(unused)]
  root_config: Option<RuntimeConfig>,
  queries: HashMap<String, Arc<(String, String)>>,
}

impl SqlXComponent {
  #[allow(clippy::needless_pass_by_value)]
  /// Create a new SQLx component.
  pub(crate) async fn new(
    config: SqlComponentConfig,
    root_config: Option<RuntimeConfig>,
    metadata: Option<Metadata>,
    resolver: &Resolver,
  ) -> Result<Self, Error> {
    validate(&config, resolver)?;
    let sig = common::gen_signature(
      "wick/component/sql",
      config.operation_signatures(),
      config.config(),
      &metadata,
    )?;
    let url = common::convert_url_resource(resolver, config.resource())?;
    let context = init_context(&config, &url).await?;
    let mut queries = HashMap::new();
    trace!(count=%config.operations().len(), "preparing queries");
    for op in config.operations() {
      // let query: Query<Postgres, _> = sqlx::query(&op.query);
      // TODO: this is a hack to during the sqlx transition and this needs to support prepared queries properly.
      queries.insert(
        op.name().to_owned(),
        Arc::new((op.query().to_owned(), op.query().to_owned())),
      );
    }

    Ok(Self {
      context,
      signature: Arc::new(sig),
      url,
      config,
      root_config,
      queries,
    })
  }
}

#[async_trait::async_trait]
impl DatabaseProvider for SqlXComponent {
  fn get_statement(&self, id: &str) -> Option<&str> {
    self.queries.get(id).map(|e| e.1.as_str())
  }

  async fn get_connection<'a, 'b>(&'a self) -> Result<Connection<'b>, Error>
  where
    'a: 'a,
  {
    Ok(Connection::new(Box::new(self.context.db.clone())))
  }
}

#[async_trait::async_trait]
impl<'a> ClientConnection<'a> for CtxPool {
  async fn finish(&'a mut self) -> Result<(), Error> {
    todo!()
  }

  async fn handle_error(&'a mut self, _e: Error, _behavior: ErrorBehavior) -> Result<(), Error> {
    todo!()
  }

  async fn exec(&'a mut self, stmt: String, bound_args: Vec<SqlWrapper>) -> Result<u64, Error> {
    self.run_exec(&stmt, bound_args).await
  }

  async fn query<'b>(
    &'a mut self,
    stmt: &'b str,
    bound_args: Vec<SqlWrapper>,
  ) -> Result<BoxStream<Result<Value, Error>>, Error> {
    let stream = self.run_query(stmt.as_ref(), bound_args);
    Ok(stream.boxed())
  }
}

// impl Component for SqlXComponent {
//   fn handle(
//     &self,
//     invocation: Invocation,
//     _data: Option<RuntimeConfig>,
//     _callback: Arc<RuntimeCallback>,
//   ) -> BoxFuture<Result<PacketStream, ComponentError>> {
//     let ctx = self.context.clone();

//     Box::pin(async move {
//       let (opdef, client, stmt) = match ctx.lock().as_ref() {
//         Some(ctx) => {
//           let opdef = ctx
//             .config
//             .get_operation(invocation.target.operation_id())
//             .ok_or_else(|| Error::MissingOperation(invocation.target.operation_id().to_owned()))?
//             .clone();
//           let client = ctx.db.clone();
//           let stmt = ctx.queries.get(invocation.target.operation_id()).unwrap().clone();
//           (opdef, client, stmt)
//         }
//         None => return Err(Error::Uninitialized.into()),
//       };

//       let input_names: Vec<_> = opdef.inputs().iter().map(|i| i.name.clone()).collect();
//       let (tx, rx) = invocation.make_response();
//       let mut input_streams = wick_packet::split_stream(invocation.packets, input_names);
//       tokio::spawn(async move {
//         'outer: loop {
//           let mut incoming_packets = Vec::new();
//           for input in &mut input_streams {
//             incoming_packets.push(input.next().await);
//           }
//           let num_done = incoming_packets.iter().filter(|r| r.is_none()).count();
//           if num_done > 0 {
//             if num_done != opdef.inputs().len() {
//               let _ = tx.error(wick_packet::Error::component_error("Missing input"));
//             }
//             break 'outer;
//           }
//           let incoming_packets = incoming_packets.into_iter().map(|r| r.unwrap()).collect::<Vec<_>>();

//           if let Some(Err(e)) = incoming_packets.iter().find(|r| r.is_err()) {
//             let _ = tx.error(wick_packet::Error::component_error(e.to_string()));
//             break 'outer;
//           }

//           let fields = opdef.inputs();
//           let mut type_wrappers = Vec::new();

//           for packet in incoming_packets {
//             let packet = packet.unwrap();
//             if packet.is_done() {
//               break 'outer;
//             }
//             let ty = fields.iter().find(|f| f.name() == packet.port()).unwrap().ty().clone();
//             type_wrappers.push((ty, packet));
//           }

//           let result = match &opdef {
//             SqlOperationKind::Query(_) => {
//               query(client.clone(), tx.clone(), opdef.clone(), type_wrappers, stmt.clone()).await
//             }
//             SqlOperationKind::Exec(_) => {
//               exec(client.clone(), tx.clone(), opdef.clone(), type_wrappers, stmt.clone()).await
//             }
//           };

//           if let Err(e) = result {
//             error!(error = %e, "error executing query");
//             let _ = tx.send(Packet::component_error(e.to_string()));
//           }
//         }
//         let _ = tx.send(Packet::done("output"));
//       });

//       Ok(rx)
//     })
//   }

//   fn signature(&self) -> &ComponentSignature {
//     &self.signature
//   }

//   fn init(&self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), ComponentError>> + Send + 'static>> {
//     let ctx = self.context.clone();
//     let addr = self.url.clone();
//     let config = self.config.clone();

//     Box::pin(async move {
//       let new_ctx = init_context(config, addr).await?;

//       ctx.lock().replace(new_ctx);

//       Ok(())
//     })
//   }
// }

impl ConfigValidation for SqlXComponent {
  type Config = SqlComponentConfig;
  fn validate(config: &Self::Config, resolver: &Resolver) -> Result<(), ComponentError> {
    Ok(validate(config, resolver)?)
  }
}

fn validate(config: &SqlComponentConfig, _resolver: &Resolver) -> Result<(), Error> {
  let bad_ops: Vec<_> = config
    .operations()
    .iter()
    .filter(|op| {
      let outputs = op.outputs();
      outputs.len() > 1 || outputs.len() == 1 && outputs[0] != Field::new("output", Type::Object)
    })
    .map(|op| op.name().to_owned())
    .collect();

  if !bad_ops.is_empty() {
    return Err(Error::InvalidOutput(bad_ops));
  }

  Ok(())
}

async fn init_client(config: &SqlComponentConfig, addr: &Url) -> Result<CtxPool, Error> {
  /*****************
   *
   * NOTE: This component was *supposed* to handle sql server (mssql) but sqlx was so buggy it
   * wouldn't work. The mssql logic is still in this crate for now, but it's not used and may be
   * removed in the future.
   *
   */
  let pool = match addr.scheme() {
    "sqlite" => CtxPool::SqlLite(sqlite::connect(config, addr).await?),
    "postgres" => CtxPool::Postgres(postgres::connect(config, addr).await?),
    "mysql" => unimplemented!("MySql is not supported yet"),
    "mssql" => unreachable!(),
    s => return Err(Error::InvalidScheme(s.to_owned())),
  };
  debug!(%addr, "connected to db");
  Ok(pool)
}

async fn init_context(config: &SqlComponentConfig, addr: &Url) -> Result<Context, Error> {
  let client = init_client(config, addr).await?;
  let db = client;

  Ok(Context { db })
}

// async fn query(
//   client: CtxPool,
//   tx: FluxChannel<Packet, wick_packet::Error>,
//   def: SqlOperationKind,
//   args: Vec<(Type, Packet)>,
//   stmt: Arc<(String, String)>,
// ) -> Result<(), Error> {
//   debug!(stmt = %stmt.0, "executing query");

//   let bound_args = common::bind_args(def.arguments(), &args)?;

//   let mut result = client.query(&stmt.1, bound_args);

//   while let Some(row) = result.next().await {
//     if let Err(e) = row {
//       let _ = tx.error(wick_packet::Error::component_error(e.to_string()));
//       return Err(Error::Fetch(e.to_string()));
//     }
//     let row = row.unwrap();
//     let packet = Packet::encode("output", row);
//     let _ = tx.send(packet);
//   }

//   Ok(())
// }

// async fn exec(
//   client: CtxPool,
//   tx: FluxChannel<Packet, wick_packet::Error>,
//   def: SqlOperationKind,
//   args: Vec<(Type, Packet)>,
//   stmt: Arc<(String, String)>,
// ) -> Result<(), Error> {
//   debug!(stmt = %stmt.0, "executing query");

//   let bound_args = common::bind_args(def.arguments(), &args)?;

//   let rows = client.exec(&stmt.1, bound_args).await?;

//   let _ = tx.send(Packet::encode("output", rows));
//   Ok(())
// }

#[cfg(test)]
mod test {
  use anyhow::Result;
  use wick_config::config::components::{SqlComponentConfigBuilder, SqlOperationDefinitionBuilder};
  use wick_config::config::{ResourceDefinition, TcpPort};
  use wick_interface_types::{Field, Type};

  use super::*;

  #[test]
  fn test_component() {
    fn is_send_sync<T: Sync>() {}
    is_send_sync::<SqlXComponent>();
  }

  #[test_logger::test(test)]
  fn test_validate() -> Result<()> {
    let mut config = SqlComponentConfigBuilder::default()
      .resource("db")
      .tls(false)
      .build()
      .unwrap();
    let op = SqlOperationDefinitionBuilder::default()
      .name("test")
      .query("select * from users where user_id = $1;")
      .inputs([Field::new("input", Type::I32)])
      .outputs([Field::new("output", Type::String)])
      .arguments(["input".to_owned()])
      .build()
      .unwrap();

    config.operations_mut().push(SqlOperationKind::Query(op));
    let mut app_config = wick_config::config::AppConfiguration::default();
    app_config.add_resource("db", ResourceDefinition::TcpPort(TcpPort::new("0.0.0.0", 11111)));

    let result = validate(&config, &app_config.resolver());
    assert!(result.is_err());
    Ok(())
  }
}
