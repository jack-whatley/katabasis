use crate::collection::{Collection, install, launch, Plugin, PluginType};
use crate::state::AppState;
use crate::targets::{self, Platform, Target};
use crate::utils::paths;
use crate::{platforms, thunderstore, utils};
use eyre::{Context, eyre, ensure};
use std::path::PathBuf;
use chrono::Utc;
use tokio::process::Command;
use crate::thunderstore::version::PackageIdent;

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

    let mut collection = Collection {
        name: name.to_owned(),
        game: target,
        plugins: vec![],
    };

    let installed_version = install::download_loader(&collection).await?;

    collection.plugins.push(Plugin::from_ident(&installed_version));

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

    let mut command = if let Some(x) = platforms::launch_command(collection.game, platform) { x }
    else {
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

    let mut collection = state.db()
        .load_collection(collection_name)
        .await?;

    let package_ident = PackageIdent::from_url(url)?;
    let package = thunderstore::query_latest_package(&package_ident).await?;

    ensure!(
        package.supports_target(&collection.game.slug),
        "package '{}' does not support target '{}'",
        package.latest.ident.name(),
        collection.game.slug
    );
    
    for dependency in package.latest.dependencies {
        collection.plugins.push(Plugin::from_ident(&dependency));
    }

    collection.plugins.push(Plugin::from_ident(&package.latest.ident));

    state.db().save_collection(&collection).await?;

    Ok(())
}
