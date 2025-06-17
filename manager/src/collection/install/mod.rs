use crate::collection::Collection;
use crate::targets::ModLoaderKind;
use crate::thunderstore;
use crate::thunderstore::version::PackageIdent;
use crate::utils::{fs, paths};
use eyre::Result;
use std::borrow::Cow;
use std::path::PathBuf;
use std::str::FromStr;

/// Downloads and sets up the relevant loader and file structure for
/// the provided [`Collection`].
pub async fn download_loader(collection: &Collection) -> Result<()> {
    let collection_dir = paths::collection_dir(&collection.name);

    tokio::fs::create_dir_all(&collection_dir).await?;

    match collection.game.mod_loader.kind {
        ModLoaderKind::BepInEx => download_bepinex_loader(collection, collection_dir).await,
    }
}

async fn download_bepinex_loader(collection: &Collection, dir: PathBuf) -> Result<()> {
    let loader_package = collection.game.mod_loader.loader_package();
    let package_ident = PackageIdent::from_str(&loader_package)?;
    let zip = thunderstore::download_latest_package(&package_ident).await?;

    tokio::task::spawn_blocking(move || -> Result<()> {
        fs::extract_archive(zip, dir, |rel_path| {
            let mut components = rel_path.components();

            if components.clone().count() == 1 {
                return Ok(None);
            }

            components.next();

            Ok(Some(Cow::Borrowed(components.as_path())))
        })?;

        Ok(())
    })
    .await??;

    Ok(())
}
