use serde::{Deserialize, Serialize};

use crate::is_false;

/// Signatures of enum type definitions.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq)]
#[must_use]
pub struct EnumDefinition {
  /// The name of the enum.
  pub name: String,
  /// The variants in the enum.
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub variants: Vec<EnumVariant>,
  /// The optional description of the enum.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// Whether this type is imported.
  #[serde(default, skip_serializing_if = "is_false")]
  pub imported: bool,
}

impl EnumDefinition {
  /// Constructor for [EnumDefinition]
  pub fn new<T: AsRef<str>>(name: T, variants: Vec<EnumVariant>) -> Self {
    Self {
      name: name.as_ref().to_owned(),
      variants,
      imported: false,
      description: None,
    }
  }
}

impl PartialEq for EnumDefinition {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.variants == other.variants
  }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[must_use]
/// An enum variant definition
pub struct EnumVariant {
  /// The name of the variant.
  pub name: String,
  /// The index of the variant.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index: Option<u32>,
  /// The optional value of the variant.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub value: Option<String>,
  /// The optional description of the variant.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

impl EnumVariant {
  /// Constructor for [EnumVariant]
  pub fn new<T: AsRef<str>>(name: T, index: Option<u32>, value: Option<String>) -> Self {
    Self {
      name: name.as_ref().to_owned(),
      index,
      value,
      description: None,
    }
  }
}
