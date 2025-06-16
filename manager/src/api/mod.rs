use std::path::PathBuf;
use eyre::{eyre, Context};
use crate::targets::Target;
use crate::{targets, utils};
use crate::collection::Collection;
use crate::state::AppState;
use crate::utils::paths;


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

/// Returns a version of the input [`Collection`] name with any invalid
/// file system characters removed.
pub fn sanitise_name(name: &str) -> String {
    name.replace(['/', '\\', '?', '*', ':', '\'', '\"', '|', '<', '>', '!'], "_")
}

/// Initialises a new [`Collection`] into the application. Will install
/// the mod loader and set up the directory structure.
pub async fn create_collection(
    name: &str,
    slug: &str,
) -> eyre::Result<String> {
    let state = AppState::get().await?;

    let target = targets::from_slug(slug).ok_or_else(||
        eyre!("Slug '{}' does not match any supported games", slug))?;

    let collection = Collection {
        name: name.to_owned(),
        game: target,
        plugins: vec![],
    };

    // Only sanitise the name on the disc, allow user to input any name they
    // want.
    let collection_dir = paths::collection_dir(&sanitise_name(name));
    
    // TODO: Call download function here...

    state.db().save_collection(&collection).await?;

    Ok(collection.name)
}
