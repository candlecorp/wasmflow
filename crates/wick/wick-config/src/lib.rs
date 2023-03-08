//! Wick Manifest implementation

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

use serde::de::DeserializeOwned;

mod helpers;

/// Module for processing JSON templates used for default values.
mod default;
pub use default::{parse_default, process_default, ERROR_STR};

/// Module for parsing parts of a manifest.
pub(crate) mod parse;

/// Wick Manifest error.
pub mod error;

/// Version 0 manifest.
pub mod v0;

/// Version 1 manifest.
pub mod v1;

/// A version-normalized format of the manifest for development.
pub mod host_definition;

/// A version-normalized format of the network manifest for development.
pub mod component_definition;
pub use component_definition::{ComponentDefinition, ComponentKind, Permissions};

/// A version-normalized format of the schematic manifest for development.
pub mod flow_definition;
pub use flow_definition::{ConnectionDefinition, ConnectionTargetDefinition, Flow, InstanceReference};
pub use flow_expression_parser::parse::v0::parse_id;

use crate::error::ManifestError;

/// The crate's error type.
pub type Error = ManifestError;

mod app_config;
pub use app_config::{
  AppConfiguration,
  CliConfig,
  ResourceDefinition,
  TcpPort,
  TriggerDefinition,
  TriggerKind,
  UdpPort,
};
mod component_config;
pub use component_config::{ComponentConfiguration, ComponentConfigurationBuilder};

pub(crate) type Result<T> = std::result::Result<T, Error>;

fn from_yaml<T>(src: &str) -> Result<T>
where
  T: DeserializeOwned,
{
  let result = serde_yaml::from_str(src).map_err(|e| ManifestError::YamlError(e.to_string()))?;
  Ok(result)
}
