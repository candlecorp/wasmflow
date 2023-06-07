use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use serde_json::json;
use structured_output::StructuredOutput;
use tracing::Instrument;

use crate::options::get_auth_for_scope;

#[derive(Debug, Clone, Args)]
#[clap(rename_all = "kebab-case")]
#[group(skip)]
pub(crate) struct Options {
  /// OCI artifact to push.
  #[clap(action)]
  pub(crate) source: PathBuf,

  #[clap(flatten)]
  pub(crate) oci_opts: crate::oci::Options,

  #[clap(long = "tag")]
  pub(crate) tags: Vec<String>,
}

#[allow(clippy::unused_async)]
pub(crate) async fn handle(
  opts: Options,
  settings: wick_settings::Settings,
  span: tracing::Span,
) -> Result<StructuredOutput> {
  span.in_scope(|| debug!("Push artifact"));

  let mut package = wick_package::WickPackage::from_path(&opts.source)
    .instrument(span.clone())
    .await?;

  let Some(registry) =  package.registry() else  {
      span.in_scope(||error!("No registry provided in package"));
      return Err(anyhow!("No registry provided in package"));
  };

  let configured_creds = settings.credentials.iter().find(|c| c.scope == registry.host());

  let (username, password) = get_auth_for_scope(
    configured_creds,
    opts.oci_opts.username.as_deref(),
    opts.oci_opts.password.as_deref(),
  );

  let oci_opts = wick_oci_utils::OciOptions::default()
    .allow_insecure(opts.oci_opts.insecure_registries)
    .allow_latest(true)
    .username(username)
    .password(password);

  let reference = package.registry_reference().unwrap(); // unwrap OK because we know we have a reg from above.

  span.in_scope(|| {
    info!(reference, "pushing artifact");
    debug!(options=?oci_opts, reference= &reference, "pushing reference");
  });

  let mut lines = Vec::new();

  let url = if !opts.tags.is_empty() {
    for tag in &opts.tags {
      let mut pack = package.clone();
      let tagged_reference = pack.tagged_reference(tag).unwrap();
      span.in_scope(|| info!(reference = &tagged_reference, "pushing tag"));
      pack.push(&tagged_reference, &oci_opts).await?;
      lines.push(format!("Pushed tag: {}", reference));
    }

    // there must be a better way than cloning the package here, feel free to fix it.
    let url = package.clone().push(&reference, &oci_opts).await?;

    span.in_scope(|| info!(%url, "artifact pushed"));

    package.push(&reference, &oci_opts).await?;
    url
  } else {
    package.push(&reference, &oci_opts).await?
  };
  let json = json!({"url":&url, "tags": opts.tags});

  span.in_scope(|| info!(%url, "artifact pushed"));
  lines.push(format!("Pushed artifact: {}", url));

  Ok(StructuredOutput::new(lines.join("\n"), json))
}
