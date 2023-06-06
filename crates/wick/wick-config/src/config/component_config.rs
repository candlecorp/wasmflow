#![allow(missing_docs)] // delete when we move away from the `property` crate.
mod composite;
mod wasm;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use asset_container::{AssetManager, Assets};
pub use composite::*;
use config::{ComponentImplementation, ComponentKind};
pub use wasm::*;
use wick_asset_reference::{AssetReference, FetchOptions};
use wick_interface_types::{ComponentMetadata, ComponentSignature, OperationSignature, TypeDefinition};

use super::common::package_definition::PackageConfig;
use super::{make_resolver, ImportBinding, TestConfiguration};
use crate::config::{BoundInterface, ResourceBinding};
use crate::import_cache::{setup_cache, ImportCache};
use crate::utils::RwOption;
use crate::{config, v1, Error, Resolver, Result};

#[derive(Debug, Default, Clone, Builder, derive_asset_container::AssetManager, property::Property)]
#[builder(derive(Debug), setter(into))]
#[property(get(public), set(public), mut(public, suffix = "_mut"))]
#[asset(asset(AssetReference))]
#[must_use]
/// A Wick component configuration.
///
/// A component configuration defines a wick component and its operations along with its dependencies
/// immediate dependencies and any dependencies that it requires be provided by the user.
pub struct ComponentConfiguration {
  #[asset(skip)]
  #[builder(setter(strip_option), default)]
  /// The name of the component configuration.
  pub(crate) name: Option<String>,
  #[builder(default = "ComponentImplementation::Composite(CompositeComponentImplementation::default())")]
  /// The component implementation.
  pub(crate) component: ComponentImplementation,
  #[asset(skip)]
  #[builder(setter(strip_option), default)]
  #[property(skip)]
  /// The source (i.e. url or file on disk) of the configuration.
  pub(crate) source: Option<PathBuf>,
  #[asset(skip)]
  #[builder(default)]
  #[property(skip)]
  /// Any types referenced or exported by this component.
  pub(crate) types: Vec<TypeDefinition>,
  #[builder(default)]
  /// Any imports this component makes available to its implementation.
  pub(crate) import: HashMap<String, ImportBinding>,
  #[asset(skip)]
  #[builder(default)]
  /// Any components or resources that must be provided to this component upon instantiation.
  pub(crate) requires: HashMap<String, BoundInterface>,
  #[builder(default)]
  /// Any resources this component defines.
  pub(crate) resources: HashMap<String, ResourceBinding>,
  #[asset(skip)]
  #[builder(default)]
  /// The configuration to use when running this component as a microservice.
  pub(crate) host: Option<config::HostConfig>,
  #[asset(skip)]
  #[builder(default)]
  /// Any embedded test cases that should be run against this component.
  pub(crate) tests: Vec<TestConfiguration>,
  #[asset(skip)]
  #[builder(default)]
  /// The metadata for this component.
  pub(crate) metadata: Option<config::Metadata>,
  #[builder(default)]
  /// The package configuration for this component.
  pub(crate) package: Option<PackageConfig>,
  #[asset(skip)]
  #[builder(setter(skip))]
  #[property(skip)]
  #[doc(hidden)]
  pub(crate) type_cache: ImportCache,
  #[asset(skip)]
  #[builder(setter(skip))]
  #[property(skip)]
  #[doc(hidden)]
  pub(crate) cached_types: RwOption<Vec<TypeDefinition>>,
}

impl ComponentConfiguration {
  /// Unwrap the inner composite component implementation or return an error.
  pub fn try_composite(&self) -> Result<&CompositeComponentImplementation> {
    match &self.component {
      ComponentImplementation::Composite(c) => Ok(c),
      _ => Err(Error::UnexpectedComponentType(
        ComponentKind::Composite,
        self.component.kind(),
      )),
    }
  }

  /// Unwrap the inner wasm component implementation or return an error.
  pub fn try_wasm(&self) -> Result<&WasmComponentImplementation> {
    match &self.component {
      ComponentImplementation::Wasm(c) => Ok(c),
      _ => Err(Error::UnexpectedComponentType(
        ComponentKind::Wasm,
        self.component.kind(),
      )),
    }
  }

  #[must_use]
  /// Retrieve the initialization configuration for this component.
  pub fn config(&self) -> &[wick_interface_types::Field] {
    match &self.component {
      ComponentImplementation::Composite(c) => c.config(),
      ComponentImplementation::Wasm(c) => c.config(),
      ComponentImplementation::Sql(_) => Default::default(),
      ComponentImplementation::HttpClient(_) => Default::default(),
    }
  }

  /// Get the package files
  #[must_use]
  pub fn package_files(&self) -> Option<Assets<AssetReference>> {
    // should return empty vec if package is None
    self.package.as_ref().map(|p| p.assets())
  }

  /// Set the source location of the configuration.
  pub fn set_source(&mut self, source: &Path) {
    let mut source = source.to_path_buf();
    self.source = Some(source.clone());
    // Source is (should be) a file, so pop the filename before setting the baseurl.
    if !source.is_dir() {
      source.pop();
    }
    self.set_baseurl(&source);
  }

  /// Returns a function that resolves a binding to a configuration item.
  #[must_use]
  pub fn resolver(&self) -> Box<Resolver> {
    let imports = self.import.clone();
    let resources = self.resources.clone();

    make_resolver(imports, resources)
  }

  /// Returns an [ImportBinding] if it exists in the configuration.
  #[must_use]
  pub fn get_import(&self, name: &str) -> Option<&ImportBinding> {
    self.import.get(name)
  }

  /// Get the kind of this component implementation.
  pub fn kind(&self) -> ComponentKind {
    self.component.kind()
  }

  /// Determine if the configuration allows for fetching artifacts with the :latest tag.
  #[must_use]
  pub fn allow_latest(&self) -> bool {
    self.host.as_ref().map_or(false, |v| v.allow_latest)
  }

  /// Return the list of insecure registries defined in the manifest
  #[must_use]
  pub fn insecure_registries(&self) -> Option<&[String]> {
    self.host.as_ref().map(|v| v.insecure_registries.as_ref())
  }

  /// Return the version of the component.
  #[must_use]
  pub fn version(&self) -> Option<&str> {
    self.metadata.as_ref().map(|m| m.version.as_str())
  }

  /// Return the underlying version of the source manifest.
  #[must_use]
  pub fn source(&self) -> Option<&Path> {
    self.source.as_deref()
  }

  /// Return the types defined in this component.
  pub fn types(&self) -> Result<Vec<TypeDefinition>> {
    self.cached_types.read().as_ref().map_or_else(
      || {
        if self.import.is_empty() {
          Ok(self.types.clone())
        } else {
          Err(Error::TypesNotFetched)
        }
      },
      |types| Ok(types.clone()),
    )
  }

  /// Get a mutable reference to the type definitions for this component.
  pub fn types_mut(&mut self) -> &mut Vec<TypeDefinition> {
    &mut self.types
  }

  /// Fetch/cache anything critical to the first use of this configuration.
  pub(crate) async fn setup_cache(&self, options: FetchOptions) -> Result<()> {
    setup_cache(
      &self.type_cache,
      self.import.values(),
      &self.cached_types,
      self.types.clone(),
      options,
    )
    .await
  }

  /// Get the component signature for this configuration.
  pub fn signature(&self) -> Result<ComponentSignature> {
    let mut sig = wick_interface_types::component! {
      name: self.name().cloned().unwrap_or_else(||self.component.default_name().to_owned()),
      version: self.version(),
      operations: self.component.operation_signatures(),
    };
    sig.config = self.config().to_vec();
    sig.types = self.types()?;
    Ok(sig)
  }

  /// Return the V1 yaml representation of this configuration.
  pub fn into_v1_yaml(self) -> Result<String> {
    let v1_manifest: v1::ComponentConfiguration = self.try_into()?;
    Ok(serde_yaml::to_string(&v1_manifest).unwrap())
  }
}

impl ComponentConfigurationBuilder {
  #[must_use]
  /// Initialize a new component configuration builder from an existing configuration.
  pub fn from_base(config: ComponentConfiguration) -> Self {
    let mut this = Self::default();
    this
      .component(config.component)
      .host(config.host)
      .tests(config.tests)
      .types(config.types)
      .requires(config.requires)
      .resources(config.resources)
      .metadata(config.metadata)
      .import(config.import);

    if let Some(name) = config.name {
      this.name(name);
    }
    if let Some(source) = config.source {
      this.source(source);
    }

    this
  }

  /// Add an imported component to the builder.
  pub fn add_import(&mut self, import: ImportBinding) {
    if let Some(imports) = &mut self.import {
      imports.insert(import.id.clone(), import);
    } else {
      let mut imports = HashMap::new();
      imports.insert(import.id.clone(), import);
      self.import = Some(imports);
    }
  }

  /// Add an imported resource to the builder.
  pub fn add_resource(&mut self, resource: ResourceBinding) {
    if let Some(r) = &mut self.resources {
      r.insert(resource.id.clone(), resource);
    } else {
      let mut r = HashMap::new();
      r.insert(resource.id.clone(), resource);
      self.resources = Some(r);
    }
  }
}

impl From<config::Metadata> for ComponentMetadata {
  fn from(value: config::Metadata) -> Self {
    Self {
      version: Some(value.version),
    }
  }
}

impl From<config::OperationSignature> for OperationSignature {
  fn from(value: config::OperationSignature) -> Self {
    Self {
      name: value.name,
      inputs: value.inputs,
      outputs: value.outputs,
    }
  }
}
