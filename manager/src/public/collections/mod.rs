use std::path::PathBuf;
use tokio::fs;
use crate::collection::Collection;
use crate::storage::KbApp;
use crate::{sanitize_file_name, setup, setup::SetupLoader, SupportedPluginSources};
use crate::storage::plugin::{Plugin, SourceHandler};

/// Public collections API for CRUD operations on the app collections.

pub mod create;

/// Fetches all [`Collection`]'s currently stored by the katabasis backend.
pub async fn get_all(limit: Option<u32>) -> crate::Result<Vec<Collection>> {
    let state = KbApp::get().await?;

    Collection::get_all(limit, &state.db_pool).await
}

/// Removes a [`Collection`].
pub async fn remove(id: String) -> crate::Result<()> {
    let state = KbApp::get().await?;
    let collection = Collection::get(&id, &state.db_pool).await?;

    Ok(collection.unwrap().remove(&state.db_pool).await?)
}

/// Removes all [`Collection`] objects stored by the application. May not catch all if there are more than a thousand.
pub async fn remove_all() -> crate::Result<()> {
    let all_collections = get_all(None).await?;

    for collection in all_collections {
        remove(collection.id).await?;
    }

    Ok(())
}

pub async fn get_full_path(id: &str) -> crate::Result<PathBuf> {
    let state = KbApp::get().await?;
    let collection_dir = state.directories.collection(&id);

    let complete_path = dunce::canonicalize(collection_dir).map_err(|e| {
        crate::Error::FileSystemError(
            format!("Failed to canonicalize collection dir: {:?}", e)
        )
    })?;

    Ok(complete_path)
}

pub async fn add_plugin(collection_id: &str, plugin_source: SupportedPluginSources, plugin_url: &str) -> crate::Result<()> {
    let state = KbApp::get().await?;
    let plugin_handler = plugin_source.get_handler();

    let created_plugin = plugin_handler.parse_share_url(plugin_url).await?;

    plugin_handler.download_plugin(&collection_id, &created_plugin).await?;

    created_plugin.update(
        collection_id.to_string(),
        &state.db_pool
    ).await?;

    Ok(())
}

pub async fn remove_plugin(collection_id: &str, plugin_id: &str) -> crate::Result<()> {
    let state = KbApp::get().await?;

    let target_plugin = Plugin::get(plugin_id, &state.db_pool).await?.ok_or(
        crate::error::Error::SQLiteStringError(
            format!("Plugin {} not found", plugin_id)
        )
    )?;

    target_plugin.remove(collection_id, &state.directories, &state.db_pool).await?;

    Ok(())
}

pub async fn fetch_all_plugins(collection_id: &str) -> crate::Result<Vec<Plugin>> {
    let state = KbApp::get().await?;

    Plugin::from_collection(collection_id, &state.db_pool).await
}

/// Install all plugins in a collection to the relevant mod directory, uses symlinks to make switching
/// plugins easier.
pub async fn install(collection_id: &str) -> crate::Result<()> {
    let state = KbApp::get().await?;

    let collection = Collection::get(collection_id, &state.db_pool).await?.ok_or(
        crate::Error::SQLiteStringError(
            format!("Collection {} not found", collection_id)
        )
    )?;

    let all_plugins = Plugin::from_collection(collection_id, &state.db_pool).await?;

    for plugin in all_plugins {
        let source_handler = plugin.source.get_handler();
        let file_dir: PathBuf = source_handler.get_plugin_file_dir(&plugin).await?;
        let setup_loader = setup::get_setup_tool(collection.game.get_loader()).await?;

        setup_loader.create_mod_symlinks(file_dir, &collection.game).await?
    }

    Ok(())
}

pub async fn export(collection_id: &str) -> crate::Result<String> {
    let state = KbApp::get().await?;

    let collection = Collection::get(collection_id, &state.db_pool).await?.ok_or(
        crate::Error::SQLiteStringError(
            format!("Collection {} not found", collection_id)
        )
    )?;

    let export_path = state.directories
        .export_dir()
        .join(format!("{}.toml", sanitize_file_name(&collection.name)));

    if export_path.exists() {
        fs::remove_file(&export_path).await?;
    }

    collection.export_to_file(&export_path, &state.db_pool).await?;

    Ok(export_path.display().to_string())
}
