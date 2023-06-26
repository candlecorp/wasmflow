use std::path::PathBuf;

use wick_config::config::components::ComponentConfig;
use wick_config::config::{AppConfiguration, CompositeComponentImplementation};
use wick_config::error::ManifestError;
use wick_config::*;
use wick_packet::RuntimeConfig;

async fn load(path: &str) -> Result<WickConfiguration, ManifestError> {
  let path = PathBuf::from(path);
  let mut config = WickConfiguration::load_from_file(path).await?;
  config.set_env(Some(std::env::vars().collect()));
  config.set_root_config(Some(RuntimeConfig::from([("component_config_name", "test".into())])));
  config.finish()
}

async fn load_app(path: &str) -> Result<AppConfiguration, ManifestError> {
  load(path).await?.try_app_config()
}

async fn load_composite(path: &str) -> Result<CompositeComponentImplementation, ManifestError> {
  Ok(load(path).await?.try_component_config()?.try_composite()?.clone())
}

#[test_logger::test(tokio::test)]
async fn test_basics() -> Result<(), ManifestError> {
  let component = load_composite("./tests/manifests/v1/logger.yaml").await?;

  assert_eq!(component.flow("logger").map(|s| s.instances().len()), Some(2));

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_types() -> Result<(), ManifestError> {
  let types = load("./tests/manifests/v1/http-types.yaml").await?.try_types_config()?;
  assert_eq!(types.types().len(), 6);

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_tests() -> Result<(), ManifestError> {
  let tests = load("./tests/manifests/v1/tests.yaml").await?.try_test_config()?;

  assert_eq!(tests.cases().len(), 1);

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_operations() -> Result<(), ManifestError> {
  let component = load_composite("./tests/manifests/v1/operations.yaml").await?;
  assert_eq!(component.operations().len(), 1);

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_main() -> Result<(), ManifestError> {
  let component = load("./tests/manifests/v1/component.yaml")
    .await?
    .try_component_config()?;

  assert!(matches!(component.component().kind(), config::ComponentKind::Wasm));

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn regression_issue_42() -> Result<(), ManifestError> {
  let component = load_app("./tests/manifests/v1/template-expansion.yaml").await?;
  println!("{:?}", component);
  let coll = component.imports().get("test").unwrap();
  let value: String = coll.kind().config().unwrap().coerce_key("pwd").unwrap();
  let expected = std::env::var("CARGO_MANIFEST_DIR").unwrap();

  assert_eq!(value, expected);
  Ok(())
}
