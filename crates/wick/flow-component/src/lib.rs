//! This crate contains the core definitions and traits that make up a flow-based component or process that can execute within the [flow-graph-interpreter](https://crates.io/crates/flow-graph-interpreter).

// !!START_LINTS
// Wick lints
// Do not change anything between the START_LINTS and END_LINTS line.
// This is automatically generated. Add exceptions after this section.
#![allow(unknown_lints)]
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
#![allow(unused_attributes, clippy::derive_partial_eq_without_eq, clippy::box_default)]
// !!END_LINTS
// Add exceptions here
#![allow()]

// mod config;
// pub use config::*;

mod context;
pub use context::*;
#[cfg(feature = "invocation")]
mod traits;

#[cfg(feature = "invocation")]
pub use traits::*;

/// A boxed future that can be sent across threads.
pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn futures::Future<Output = T> + Send + 'a>>;

pub use serde_json::Value;

#[derive(Debug)]
#[must_use]
/// A generic error type for components.
pub struct ComponentError {
  source: Box<dyn std::error::Error + Send + Sync>,
}
impl std::error::Error for ComponentError {}
impl std::fmt::Display for ComponentError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.source.to_string().as_str())
  }
}
impl ComponentError {
  /// Create a new error from a boxed error.
  pub fn new(source: impl std::error::Error + Send + Sync + 'static) -> Self {
    Self {
      source: Box::new(source),
    }
  }

  /// Create a new error from a string.
  pub fn message(msg: &str) -> Self {
    Self {
      source: Box::new(GenericError(msg.to_owned())),
    }
  }
}
impl From<Box<dyn std::error::Error + Send + Sync>> for ComponentError {
  fn from(source: Box<dyn std::error::Error + Send + Sync>) -> Self {
    Self { source }
  }
}

impl From<anyhow::Error> for ComponentError {
  fn from(source: anyhow::Error) -> Self {
    Self::message(&source.to_string())
  }
}

#[derive(Debug)]
struct GenericError(String);
impl std::error::Error for GenericError {}
impl std::fmt::Display for GenericError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
