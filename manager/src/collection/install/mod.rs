use std::path::Path;
use eyre::Result;
use crate::collection::Collection;
use crate::targets::ModLoaderKind;
use crate::utils::paths;

/// Downloads and sets up the relevant loader and file structure for
/// the provided [`Collection`].
pub async fn download_loader(
    collection: &Collection,
) -> Result<()> {
    let collection_dir = paths::collection_dir(&collection.name);
    
    tokio::fs::create_dir_all(&collection_dir).await?;
    
    match collection.game.mod_loader.kind {
        ModLoaderKind::BepInEx => download_bepinex_loader(collection, &collection_dir).await,
    }
}

async fn download_bepinex_loader(
    collection: &Collection,
    dir: &Path,
) -> Result<()> {
    let loader_package = collection.game.mod_loader.loader_package();

    Ok(())
}
