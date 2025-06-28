use crate::collection::export::ExportCollection;
use crate::collection::{install, launch, Collection};
use crate::state::AppState;
use crate::targets::{self, Target};
use crate::utils::paths;
use crate::platforms;
use eyre::{eyre, Context};
use std::path::PathBuf;
use tokio::process::Command;

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

    tokio::fs::create_dir_all(paths::collection_dir(&collection.name)).await?;

    state.db().save_collection(&collection).await?;

    Ok(collection.name)
}

/// Fire and forget function for launching a [`Collection`].
pub async fn launch_collection_detached(name: &str) -> eyre::Result<()> {
    let state = AppState::get().await?;

    let collection = state
        .db()
        .load_collection(name)
        .await
        .with_context(|| format!("failed to load collection with name '{}'", name))?;

    let platform = collection.game.platforms.iter().next().unwrap();
    let game_dir = platforms::game_dir(collection.game, platform)?;
    let collection_dir = PathBuf::from(paths::collection_dir(&collection.name));

    launch::link_files(&collection_dir, &game_dir).await?;

    let mut command = if let Some(x) = platforms::launch_command(collection.game, platform) {
        x
    } else {
        launch::app_path(&game_dir).await.map(Command::new)?
    };

    launch::add_loader_args(&mut command, &collection_dir, &collection.game.mod_loader).await?;

    command.spawn()?;

    Ok(())
}

pub async fn list_collections() -> eyre::Result<Vec<Collection>> {
    let state = AppState::get().await?;

    Ok(state.db().load_all_collections().await?)
}

pub async fn add_plugin(collection_name: &str, url: &str) -> eyre::Result<()> {
    let state = AppState::get().await?;

    let mut collection = state.db().load_collection(collection_name).await?;

    install::install_with_deps(
        &mut collection,
        url
    ).await?;

    state.db().save_collection(&collection).await?;

    Ok(())
}

pub async fn clear_cache() -> eyre::Result<()> {
    let cache_dir = paths::cache_dir();

    tokio::fs::remove_dir_all(&cache_dir).await?;

    Ok(())
}

pub async fn export_collection(collection_name: &str) -> eyre::Result<()> {
    let state = AppState::get().await?;
    let collection = state.db().load_collection(collection_name).await?;
    let export = ExportCollection::from_collection(&collection);

    export.export().await?;

    Ok(())
}

pub async fn import_collection(collection_path: &str) -> eyre::Result<()> {
    let state = AppState::get().await?;
    let export = ExportCollection::from_file(collection_path).await?;

    let new_id = create_collection(&export.name, &export.slug).await?;
    let mut collection = state.db().load_collection(&new_id).await?;

    install::install_without_deps(&mut collection, export.plugins.as_slice()).await?;

    state.db().save_collection(&collection).await?;

    Ok(())
}

pub async fn remove_collection(collection_name: &str) -> eyre::Result<()> {
    let state = AppState::get().await?;
    let collection = state.db().load_collection(collection_name).await?;

    tokio::fs::remove_dir_all(paths::collection_dir(&collection.name)).await?;
    state.db().remove_collection(&collection).await?;

    Ok(())
}

pub fn log_path() -> PathBuf {
    paths::log_path()
}

pub async fn create_shortcut(collection_name: &str) -> eyre::Result<()> {
    let state = AppState::get().await?;
    let collection = state.db().load_collection(collection_name).await?;

    launch::create_link(&collection).await?;

    Ok(())
}
