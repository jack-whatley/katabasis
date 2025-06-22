use crate::collection::{Collection, Plugin};
use crate::thunderstore::version::VersionIdent;
use crate::utils::paths;
use eyre::Result;

/// Downloads all provided plugins to the collection. Will not check for duplicates.
pub async fn install_plugins(collection: &Collection, plugins: &Vec<Plugin>) -> Result<()> {}

/// Checks cache directory for plugin and installs it if it exists.
async fn try_install_plugin(ident: &VersionIdent) -> Result<bool> {
    let cache_dir = paths::plugin_cache_dir().join(ident.as_str());

    if !cache_dir.exists() {
        return Ok(false);
    }

    todo!()
}
