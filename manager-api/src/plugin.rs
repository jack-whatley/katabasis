use manager_core::error;
use manager_core::state::KatabasisApp;
use manager_core::storage::plugin_repository;

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
