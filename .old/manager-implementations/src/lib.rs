mod thunderstore;
mod bepinex;

use crate::bepinex::BepInExCollectionHandler;
use crate::thunderstore::{ThunderstorePluginHandler, THUNDERSTORE_RE};
use async_trait::async_trait;
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
    /// directory. Updates the plugin structs plugin_path field
    /// to the correct install location.
    async fn download_latest(
        &self,
        state: &KatabasisApp,
        collection: &Collection,
        plugin: &mut Plugin
    ) -> error::KatabasisResult<()>;

    /// Checks if the current mod version number is different to the
    /// latest from the plugin source, returns true if that is the case.
    async fn has_update(
        &self,
        state: &KatabasisApp,
        plugin: &Plugin,
    ) -> error::KatabasisResult<bool>;

    /// Enables or disables the plugin file based on input. This involves
    /// renaming the plugin file to contain _DISABLED. Takes the current state
    /// from the passed in plugin.
    async fn switch_plugin_state(
        &self,
        state: &KatabasisApp,
        plugin: &Plugin,
    ) -> error::KatabasisResult<()>;
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
    Ok(get_downloader_direct(&determine_url_source(collection_target, plugin_source)?))
}

pub fn get_downloader_direct(
    plugin_source: &PluginSource,
) -> Box<impl PluginHandler + Sized + Send + Sync> {
    match plugin_source {
        PluginSource::Thunderstore => Box::new(ThunderstorePluginHandler),
    }
}

/// Returns the correct plugin source based on the provided URL. If one
/// can't be found then an error will be returned.
fn determine_url_source(
    collection_target: &PluginTarget,
    url: &str
) -> error::KatabasisResult<PluginSource> {
    if let Some(captures) = THUNDERSTORE_RE.captures(url) {
        let target_name = match collection_target {
            PluginTarget::LethalCompany => "lethal-company",
        };

        if target_name == &captures["game"] {
            Ok(PluginSource::Thunderstore)
        }
        else {
            Err(
                error::KatabasisErrorKind::InvalidPluginUrl(
                    format!("Captured game '{}' did not match target name '{}'", &captures["game"], target_name)
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
