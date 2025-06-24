use crate::collection::{Collection, Plugin};
use eyre::{ensure, Result};
use iter_tools::Itertools;
use crate::thunderstore;
use crate::thunderstore::version::{PackageIdent, VersionIdent};

pub mod downloader;
pub mod handler;

pub async fn install_with_deps(
    collection: &mut Collection,
    plugin_url: &str,
) -> Result<()> {
    let package_ident = PackageIdent::from_url(plugin_url)?;
    let package = thunderstore::query_latest_package(&package_ident).await?;

    ensure!(
        package.supports_target(&collection.game.slug),
        "package '{}' does not support target '{}'",
        package.latest.ident.name(),
        collection.game.slug
    );

    let all_plugins = fetch_all_plugins(&package.latest.ident.as_package_ident()).await?;
    let all_current_plugins = &collection.plugins.iter().map(|x| x.ident()).collect::<Vec<_>>();

    let all_plugins = all_plugins.into_iter()
        .filter(|plugin| !all_current_plugins.contains(&plugin))
        .map(Plugin::from_moved_ident)
        .collect::<Vec<_>>();

    downloader::install_plugins(collection, &all_plugins).await?;

    collection.plugins.extend(all_plugins);

    Ok(())
}

/// Fetches a package based on its [`PackageIdent`] as well as all its dependencies.
async fn fetch_all_plugins(ident: &PackageIdent) -> Result<Vec<VersionIdent>> {
    let mut all_plugins: Vec<VersionIdent> = vec![];
    let package = thunderstore::query_latest_package(ident).await?;

    all_plugins.push(package.latest.ident);

    for dependencies in package.latest.dependencies {
        let dependent_dependencies = Box::pin(fetch_all_plugins(&dependencies.as_package_ident())).await?;

        all_plugins.extend(dependent_dependencies);
    }

    Ok(all_plugins.into_iter()
        .unique_by(|id| id.as_str().to_owned())
        .rev()
        .collect_vec())
}
