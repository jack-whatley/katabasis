use std::path::{Path, PathBuf};
use async_trait::async_trait;
use crate::utils::fs::PluginZip;
use eyre::Result;
use crate::collection::{Collection, Plugin};

pub mod bepinex;
pub mod dir_map;

/// Async trait for handling different forms of [`Plugin`] including operations like
/// extracting to the disk and installing.
#[async_trait]
pub trait PluginHandler {
    /// [`Plugin`] handler method for extracting an in-memory [`PluginZip`] to a standard
    /// directory based on the installer type.
    async fn extract(&self, zip: PluginZip, dir: PathBuf, plugin_name: &str) -> Result<()>;

    /// [`Plugin`] handler method for installing the plugin from the provided `src`
    /// directory, this is likely the directory `PluginHandler::extract` has saved the
    /// file to.
    async fn install(&self, src: &Path, plugin_name: &str, collection: &Collection) -> Result<()>;

    /// [`Plugin`] handler method for uninstalling the plugin from the provided collection,
    /// this includes modifying the [`Collection`]'s saved plugins and removing the [`Plugin`]
    /// files on disk.
    ///
    /// For example if using BepInEx this would remove `collections/test/BepInEx/plugins/notnotnotswipez-MoreCompany`.
    async fn uninstall(&self, plugin: &Plugin, collection: &Collection) -> Result<()>;

    /// [`Plugin`] handler method for toggling the state of a mod between enabled and disabled.
    async fn switch(&self, enabled: bool, plugin: &Plugin, collection: &Collection) -> Result<()>;
}
