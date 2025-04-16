mod thunderstore;
mod bepinex;

use async_trait::async_trait;
use manager_core::data::support::PluginLoader;
use manager_core::data::{Collection, Plugin};
use manager_core::error;
use manager_core::state::KatabasisApp;
use crate::bepinex::BepInExCollectionHandler;

#[async_trait]
pub trait CollectionHandler {
    /// Downloads the collection loader and creates all the required folders.
    async fn initialise_collection(
        &self,
        collection: &Collection,
        app: &KatabasisApp,
    ) -> error::KatabasisResult<()>;

    /// Removes a collection from the file system.
    async fn remove_collection(
        &self,
        collection: &Collection,
        app: &KatabasisApp,
    ) -> error::KatabasisResult<()>;
}

/// Asynchronous mod downloader, there should be one implementation per
/// type of [`PluginLoader`].
#[async_trait]
pub trait ModDownloader {
    fn get_api_url(&self, ) -> String;

    async fn download_latest(&self, plugin: &Plugin) -> error::KatabasisResult<()>;

    async fn check_for_updates(&self, plugin: &Plugin) -> error::KatabasisResult<bool>;
}

pub struct ThunderstoreModDownloader;

pub fn get_collection_handler(collection_type: &PluginLoader) -> Box<impl CollectionHandler + Sized + Send + Sync> {
    match collection_type {
        PluginLoader::BepInEx => Box::new(BepInExCollectionHandler),
    }
}

// pub fn get_downloader(plugin_loader: PluginSource) -> Box<impl ModDownloader + Sized + Sync + Send> {
//     match plugin_loader {
//         PluginSource::Thunderstore => Box::new(ThunderstoreModDownloader),
//     }
// }
