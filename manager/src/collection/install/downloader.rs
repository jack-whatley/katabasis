use crate::collection::{Collection, Plugin};
use crate::thunderstore::version::VersionIdent;
use eyre::Result;

/// Downloads all provided plugins to the collection. Will not check for duplicates.
pub async fn download_all_plugins(collection: &Collection, plugins: &Vec<Plugin>) -> Result<()> {
    
}
