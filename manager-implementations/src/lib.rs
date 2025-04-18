mod thunderstore;
mod bepinex;

use async_trait::async_trait;
use manager_core::data::support::{PluginLoader, PluginSource};
use manager_core::data::{Collection, Plugin};
use manager_core::error;
use manager_core::state::KatabasisApp;
use crate::bepinex::BepInExCollectionHandler;
use crate::thunderstore::ThunderstorePluginHandler;

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
    fn get_api_url(&self, ) -> String;

    async fn download_latest(&self, plugin: &Plugin) -> error::KatabasisResult<()>;

    async fn check_for_updates(&self, plugin: &Plugin) -> error::KatabasisResult<bool>;
}

pub fn get_collection_handler(collection_type: &PluginLoader) -> Box<impl CollectionHandler + Sized + Send + Sync> {
    match collection_type {
        PluginLoader::BepInEx => Box::new(BepInExCollectionHandler),
    }
}

pub fn get_downloader(plugin_loader: PluginSource) -> Box<impl PluginHandler + Sized + Send + Sync> {
    match plugin_loader {
        PluginSource::Thunderstore => Box::new(ThunderstorePluginHandler),
    }
}
