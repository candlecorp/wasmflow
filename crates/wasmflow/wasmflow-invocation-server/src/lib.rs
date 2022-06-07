//! Wasmflow RPC SDK

// !!START_LINTS
// Wasmflow lints
// Do not change anything between the START_LINTS and END_LINTS line.
// This is automatically generated. Add exceptions after this section.
#![deny(
  clippy::expect_used,
  clippy::explicit_deref_methods,
  clippy::option_if_let_else,
  clippy::await_holding_lock,
  clippy::cloned_instead_of_copied,
  clippy::explicit_into_iter_loop,
  clippy::flat_map_option,
  clippy::fn_params_excessive_bools,
  clippy::implicit_clone,
  clippy::inefficient_to_string,
  clippy::large_types_passed_by_value,
  clippy::manual_ok_or,
  clippy::map_flatten,
  clippy::map_unwrap_or,
  clippy::must_use_candidate,
  clippy::needless_for_each,
  clippy::needless_pass_by_value,
  clippy::option_option,
  clippy::redundant_else,
  clippy::semicolon_if_nothing_returned,
  clippy::too_many_lines,
  clippy::trivially_copy_pass_by_ref,
  clippy::unnested_or_patterns,
  clippy::future_not_send,
  clippy::useless_let_if_seq,
  clippy::str_to_string,
  clippy::inherent_to_string,
  clippy::let_and_return,
  clippy::string_to_string,
  clippy::try_err,
  clippy::unused_async,
  clippy::missing_enforced_import_renames,
  clippy::nonstandard_macro_braces,
  clippy::rc_mutex,
  clippy::unwrap_or_else_default,
  clippy::manual_split_once,
  clippy::derivable_impls,
  clippy::needless_option_as_deref,
  clippy::iter_not_returning_iterator,
  clippy::same_name_method,
  clippy::manual_assert,
  clippy::non_send_fields_in_send_ty,
  clippy::equatable_if_let,
  bad_style,
  clashing_extern_declarations,
  const_err,
  dead_code,
  deprecated,
  explicit_outlives_requirements,
  improper_ctypes,
  invalid_value,
  missing_copy_implementations,
  missing_debug_implementations,
  mutable_transmutes,
  no_mangle_generic_items,
  non_shorthand_field_patterns,
  overflowing_literals,
  path_statements,
  patterns_in_fns_without_body,
  private_in_public,
  trivial_bounds,
  trivial_casts,
  trivial_numeric_casts,
  type_alias_bounds,
  unconditional_recursion,
  unreachable_pub,
  unsafe_code,
  unstable_features,
  unused,
  unused_allocation,
  unused_comparisons,
  unused_import_braces,
  unused_parens,
  unused_qualifications,
  while_true,
  missing_docs
)]
#![allow(unused_attributes)]
// !!END_LINTS
// Add exceptions here
#![allow(unused_qualifications)]

/// Error module.
pub mod error;

pub(crate) mod conversion;
mod invocation_server;

pub use invocation_server::InvocationServer;
use tokio::task::JoinHandle;
use tonic::transport::{Channel, Server, Uri};
use wasmflow_rpc::rpc::invocation_service_client::InvocationServiceClient;
use wasmflow_rpc::rpc::invocation_service_server::InvocationServiceServer;
use wasmflow_rpc::SharedRpcHandler;
use wasmflow_transport::{MessageTransport, TransportMap};

pub(crate) type Result<T> = std::result::Result<T, error::Error>;

/// The crate's error type.
pub use crate::error::Error;

/// Exported type so consumers don't need to depend on tonic directly.
pub type InvocationClient = InvocationServiceClient<Channel>;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate derivative;

#[doc(hidden)]
pub fn make_input<K: AsRef<str>, V: serde::Serialize>(entries: Vec<(K, V)>) -> TransportMap {
  entries
    .into_iter()
    .map(|(k, v)| Ok((k.as_ref().to_owned(), MessageTransport::success(&v))))
    .filter_map(Result::ok)
    .collect()
}

/// Build and spawn an RPC server for the passed collection.
#[must_use]
pub fn make_rpc_server(
  socket: tokio::net::TcpSocket,
  collection: SharedRpcHandler,
) -> JoinHandle<std::result::Result<(), tonic::transport::Error>> {
  let component_service = InvocationServer::new(collection);

  let svc = InvocationServiceServer::new(component_service);

  let listener = tokio_stream::wrappers::TcpListenerStream::new(socket.listen(1).unwrap());

  tokio::spawn(Server::builder().add_service(svc).serve_with_incoming(listener))
}

/// Create an RPC client.
pub async fn connect_rpc_client(uri: Uri) -> Result<InvocationServiceClient<Channel>> {
  Ok(InvocationServiceClient::connect(uri).await?)
}

#[doc(hidden)]
pub fn bind_new_socket() -> Result<tokio::net::TcpSocket> {
  let socket = tokio::net::TcpSocket::new_v4()?;
  socket.bind("127.0.0.1:0".parse().unwrap())?;
  Ok(socket)
}
