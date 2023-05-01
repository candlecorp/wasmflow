use wick_asset_reference::AssetReference;

use super::Glob;

#[derive(Debug, Clone, Default, Builder, derive_asset_container::AssetManager)]
#[asset(AssetReference)]
/// The package details for an application or component.
pub struct PackageConfig {
  /// The list of files and folders to be included with the package.
  #[builder(default)]
  pub files: Vec<Glob>,

  /// Configuration for publishing the package to a registry. This will be used if the package is published without any additional arguments on the command line. If a tag is specified on the command line, that tag will be used instead.
  #[builder(default)]
  #[asset(skip)]
  pub registry: Option<RegistryConfig>,
}

#[derive(Debug, Default, Builder, Clone, PartialEq)]
pub struct RegistryConfig {
  /// The registry to publish to.
  #[builder(default)]
  pub registry: String,
  /// The namespace on the registry. ex: registry.candle.dev/&lt;namespace&gt;/&lt;myWickApp&gt;
  #[builder(default)]
  pub namespace: String,
}
