use manager_core::data::Plugin;
use manager_core::error;
use manager_core::state::KatabasisApp;
use manager_core::storage::plugin_repository;
use manager_implementations::get_downloader_direct;
use manager_implementations::PluginHandler;

pub async fn switch_plugin_state(
    id: &str,
    plugin_state: bool
) -> error::KatabasisResult<()> {
    let state = KatabasisApp::get().await?;

    let mut plugin = plugin_repository::get(
        id,
        &state.db_pool
    ).await?;

    let collection = plugin_repository::get_collection(
        id,
        &state.db_pool
    ).await?;

    plugin.is_enabled = plugin_state;

    plugin_repository::upsert(
        &collection,
        &plugin,
        &state.db_pool
    ).await?;

    let plugin_handler = get_downloader_direct(&plugin.source);

    plugin_handler.switch_plugin_state(
        &state,
        &plugin
    ).await?;

    Ok(())
}

pub async fn state(
    id: &str,
) -> error::KatabasisResult<bool> {
    let state = KatabasisApp::get().await?;

    let plugin = plugin_repository::get(
        id,
        &state.db_pool
    ).await?;

    Ok(plugin.is_enabled)
}

pub async fn get_all(collection_id: &str) -> error::KatabasisResult<Vec<Plugin>> {
    let state = KatabasisApp::get().await?;

    plugin_repository::get_all(collection_id, &state.db_pool).await
}

pub async fn remove(plugin_id: &str) -> error::KatabasisResult<()> {
    let state = KatabasisApp::get().await?;

    let plugin = plugin_repository::get(
        plugin_id,
        &state.db_pool
    ).await?;

    plugin_repository::remove(
        &plugin,
        &state.db_pool
    ).await?;

    tokio::fs::remove_file(
        &plugin.plugin_path,
    ).await?;

    Ok(())
}
