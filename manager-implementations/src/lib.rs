mod thunderstore;
mod bepinex;

use crate::bepinex::BepInExCollectionHandler;
use crate::thunderstore::ThunderstorePluginHandler;
use async_trait::async_trait;
use log::info;
use once_cell::sync::Lazy;
use phf::{phf_map, Map};
use regex::Regex;
use manager_core::data::support::{PluginLoader, PluginSource, PluginTarget};
use manager_core::data::{Collection, Plugin};
use manager_core::error;
use manager_core::state::KatabasisApp;

#[async_trait]
pub trait CollectionHandler {
    /// Downloads the collection loader and creates all the required folders. Does
    /// not initialise the collection into the database.
    async fn initialise_collection(
        &self,
        collection: &Collection,
        state: &KatabasisApp,
    ) -> error::KatabasisResult<()>;

    /// Removes a collection from the file system. Will still appear in
    /// the database even if this is removed.
    async fn remove_collection(
        &self,
        collection: &Collection,
        state: &KatabasisApp,
    ) -> error::KatabasisResult<()>;

    /// Installs the collection to its target game directory. See the
    /// notes on [`InstallType`] for how that is implemented.
    async fn install_collection(
        &self,
        collection: &Collection,
        state: &KatabasisApp,
    ) -> error::KatabasisResult<()>;
}

/// Asynchronous mod downloader, there should be one implementation per
/// type of [`PluginSource`].
#[async_trait]
pub trait PluginHandler {
    /// Initialises a plugin from the provided URL.
    async fn initialise_plugin(
        &self,
        state: &KatabasisApp,
        url: &str,
    ) -> error::KatabasisResult<Plugin>;

    /// Downloads the plugin and install it into the collection
    /// directory.
    async fn download_latest(
        &self,
        state: &KatabasisApp,
        collection: &Collection,
        plugin: &Plugin
    ) -> error::KatabasisResult<()>;

    /// Checks if the current mod version number is different to the
    /// latest from the plugin source, returns true if that is the case.
    async fn has_update(
        &self,
        state: &KatabasisApp,
        plugin: &Plugin,
    ) -> error::KatabasisResult<bool>;
}

/// Fetches a relevant collection handler from the provided type.
pub fn get_collection_handler(
    collection_type: &PluginLoader
) -> Box<impl CollectionHandler + Sized + Send + Sync> {
    match collection_type {
        PluginLoader::BepInEx => Box::new(BepInExCollectionHandler),
    }
}

/// Parses a user provided url to determine if a link to a downloadable
/// plugin has been provided.
pub fn get_downloader(
    collection_target: &PluginTarget,
    plugin_source: &str
) -> error::KatabasisResult<Box<impl PluginHandler + Sized + Send + Sync>> {
    match determine_url_source(collection_target, plugin_source)? {
        PluginSource::Thunderstore => Ok(Box::new(ThunderstorePluginHandler)),
    }
}

/// Returns the correct plugin source based on the provided URL. If one
/// can't be found then an error will be returned.
fn determine_url_source(
    collection_target: &PluginTarget,
    url: &str
) -> error::KatabasisResult<PluginSource> {
    static RE: Lazy<Regex> = Lazy::new(
        || {
            Regex::new(r"https://([A-Za-z0-9.]{4})?thunderstore\.io/c/(?<game>[A-Za-z0-9-]+)/p/(?<namespace>[A-Za-z0-9]+)/(?<name>[A-Za-z0-9]+)/")
                .unwrap()
        }
    );

    if let Some(captures) = RE.captures(url) {
        let target_name = match collection_target {
            PluginTarget::LethalCompany => "lethal-company",
        };

        if target_name == &captures["game"] {
            Ok(PluginSource::Thunderstore)
        }
        else {
            Err(
                error::KatabasisErrorKind::InvalidPluginUrl(
                    format!("Failed to extract game from Thunderstore URL: {}", url)
                ).into()
            )
        }
    }
    else {
        Err(
            error::KatabasisErrorKind::InvalidPluginUrl(
                format!("Provided URL is not a supported source: {}", url)
            ).into()
        )
    }
}
