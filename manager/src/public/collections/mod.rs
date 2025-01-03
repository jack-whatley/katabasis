use std::path::PathBuf;
use crate::collection::Collection;
use crate::storage::KbApp;
use crate::SupportedPluginSources;
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

    created_plugin.update(
        collection_id.to_string(),
        &state.db_pool
    ).await?;

    // download step happens here...

    Ok(())
}

pub async fn fetch_all_plugins(collection_id: &str) -> crate::Result<Vec<Plugin>> {
    let state = KbApp::get().await?;

    Plugin::from_collection(collection_id, &state.db_pool).await
}
