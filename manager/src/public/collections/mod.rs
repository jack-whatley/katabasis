use std::path::{Path, PathBuf};
use tokio::fs;
use crate::collection::{Collection, ExportCollection};
use crate::storage::KbApp;
use crate::{sanitize_file_name, setup, setup::SetupLoader, SupportedPluginSources};
use crate::storage::plugin::{Plugin, SourceHandler};
use crate::utils::fs::SymlinkTool;

/// Public collections API for CRUD operations on the katabasis-app collections.

pub mod create;

pub async fn get_one(id: &str) -> crate::Result<Collection> {
    let state = KbApp::get().await?;

    Collection::get(id, &state.db_pool).await?.ok_or(
        crate::Error::SQLiteStringError(
            format!("Failed to fetch a single collection with id: '{}'", id)
        )
    )
}

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

    let install_dir = collection.game
        .get_game_dir()?
        .join("BepInEx")
        .join("plugins");

    // TODO: Find better solution to removing install dir...
    if install_dir.exists() {
        fs::remove_dir_all(&install_dir).await?;
    }

    fs::create_dir_all(&install_dir).await?;

    let mut symlink_tool = SymlinkTool::init().await?;

    for plugin in all_plugins {
        if !plugin.is_enabled { continue; }
        
        let source_handler = plugin.source.get_handler();

        let file_dir = match source_handler.get_plugin_file_dir(&plugin).await {
            Ok(file_dir) => file_dir,
            Err(_) => continue,
        };

        let setup_loader = setup::get_setup_tool(collection.game.get_loader()).await?;

        setup_loader.install_mod(&file_dir, &collection.game, &mut symlink_tool).await?
    }

    symlink_tool.terminate().await?;

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

pub async fn import<P: AsRef<Path>>(file_path: P) -> crate::Result<String> {
    let import_file = fs::read(&file_path).await?;

    let string_contents = String::from_utf8(import_file).map_err(|e| {
        crate::Error::FileSystemError(
            format!("Failed to read from file: {:?}", e)
        )
    })?;

    let exported_collection = toml::from_str::<ExportCollection>(&string_contents).map_err(|err| {
        crate::Error::FileSystemError(
            format!("Failed to parse TOML file: {:?}", err)
        )
    })?;

    let col_id = create::create(
        exported_collection.name.clone(),
        exported_collection.game,
        exported_collection.game_version
    ).await?;

    for exported_plugin in exported_collection.plugins {
        add_plugin(&col_id, exported_plugin.source, &exported_plugin.api_url).await?;
    }

    Ok(exported_collection.name)
}

pub async fn switch_plugin(plugin_id: &str, is_enabled: bool) -> crate::Result<()> {
    let state = KbApp::get().await?;

    let mut plugin = Plugin::get(plugin_id, &state.db_pool).await?.ok_or(
        crate::Error::SQLiteStringError(
            format!("Plugin {} not found", plugin_id)
        )
    )?;

    let collection_id = plugin.get_collection(&state.db_pool).await?.id;

    plugin.is_enabled = is_enabled;

    Ok(plugin.update(collection_id, &state.db_pool).await?)
}
