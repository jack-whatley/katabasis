use crate::collection::{Collection, install};
use crate::state::AppState;
use crate::targets::{self, Target};
use crate::utils;
use eyre::eyre;
use std::path::PathBuf;

/// Returns the [`PathBuf`] to the applications default directory.
pub fn app_dir() -> PathBuf {
    utils::paths::default_app_dir()
}

/// Returns an iterator of all currently supported application
/// targets.
pub fn all_targets() -> impl Iterator<Item = Target> {
    targets::all()
}

/// Returns a specific target based on the provided slug.
pub fn specific_target(slug: &str) -> Option<Target> {
    targets::from_slug(slug)
}

/// Initialises a new [`Collection`] into the application. Will install
/// the mod loader and set up the directory structure.
pub async fn create_collection(name: &str, slug: &str) -> eyre::Result<String> {
    let state = AppState::get().await?;

    let target = targets::from_slug(slug)
        .ok_or_else(|| eyre!("Slug '{}' does not match any supported games", slug))?;

    let collection = Collection {
        name: name.to_owned(),
        game: target,
        plugins: vec![],
    };

    install::download_loader(&collection).await?;

    state.db().save_collection(&collection).await?;

    Ok(collection.name)
}
