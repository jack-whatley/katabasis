use crate::collection::{Collection, Plugin};
use crate::thunderstore;
use crate::thunderstore::version::VersionIdent;
use crate::utils::paths;
use eyre::{ensure, Result};
use std::path::PathBuf;

/// Downloads all provided plugins to the collection. Will not check for duplicates.
pub async fn install_plugins(collection: &Collection, plugins: &Vec<Plugin>) -> Result<()> {
    for plugin in plugins {
        if !try_cache_install(collection, plugin).await? {
            download_to_cache(collection, plugin).await?;

            ensure!(
                try_cache_install(collection, plugin).await?,
                "failed to install plugin after downloading it to cache"
            )
        }
    }

    Ok(())
}

/// Checks cache directory for plugin and installs it if it exists.
async fn try_cache_install(collection: &Collection, plugin: &Plugin) -> Result<bool> {
    let cache_dir = cache_path(plugin.ident());

    if !cache_dir.exists() {
        return Ok(false);
    }

    let installer = collection.game
        .mod_loader
        .installer_for_plugin(plugin.ident().full_name());

    installer.install(&cache_dir, plugin.ident().full_name(), collection).await?;

    Ok(true)
}

async fn download_to_cache(collection: &Collection, plugin: &Plugin) -> Result<()> {
    let zip = thunderstore::download_specific_package(plugin.ident()).await?;

    let installer = collection.game
        .mod_loader
        .installer_for_plugin(plugin.ident().full_name());

    installer.extract(zip, cache_path(plugin.ident()), plugin.ident().full_name()).await?;

    Ok(())
}

fn cache_path(ident: &VersionIdent) -> PathBuf {
    paths::plugin_cache_dir().join(ident.as_str())
}
