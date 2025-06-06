use std::path::PathBuf;
use chrono::Utc;
use log::{error, warn};
use uuid::Uuid;
use manager_core::data::{Collection, Plugin};
use manager_core::data::support::{InstallType, PluginTarget};
use manager_core::error;
use manager_core::state::KatabasisApp;
use manager_core::storage::{collection_repository, plugin_repository};
use manager_implementations::{get_collection_handler, get_downloader, CollectionHandler, PluginHandler};

/// Initialises a new collection.
pub async fn create(
    name: String,
    game: String,
    version: String,
) -> error::KatabasisResult<Collection> {
    let state = KatabasisApp::get().await?;
    let id = Uuid::new_v4();
    let time = Utc::now();

    let parsed_game = PluginTarget::from(game);
    let mod_loader = parsed_game.get_loader();

    let collection = Collection {
        id: format!("{}", id.as_hyphenated()),
        name,
        game: parsed_game,
        game_version: version,
        install_type: InstallType::Copy, // TODO: ALLOW USER INPUT
        created: time,
        modified: time,
        last_played: None,
    };

    let collection_handler = get_collection_handler(&mod_loader);

    let insert_result = collection_repository::upsert(&collection, &state.db_pool).await;
    let initialise_result = collection_handler.initialise_collection(&collection, &state).await;

    if insert_result.is_err() || initialise_result.is_err() {
        warn!("Failed to initialise collection, undoing the creation");
        remove(&collection).await?;
    }

    Ok(collection)
}

/// Removes a collection.
pub async fn remove(collection: &Collection) -> error::KatabasisResult<()> {
    let state = KatabasisApp::get().await?;
    let mod_loader = collection.game.get_loader();
    let collection_handler = get_collection_handler(&mod_loader);

    collection_handler.remove_collection(collection, &state).await?;
    collection_repository::remove(collection, &state.db_pool).await?;

    Ok(())
}

/// Fetches a collection object based on the ID.
pub async fn get(id: &str) -> error::KatabasisResult<Collection> {
    let state = KatabasisApp::get().await?;

    collection_repository::get(id, &state.db_pool).await
}

/// Fetches all collections.
pub async fn get_all() -> error::KatabasisResult<Vec<Collection>> {
    let state = KatabasisApp::get().await?;
    
    collection_repository::get_all(None, &state.db_pool).await
}

/// Installs a collection.
pub async fn install(collection: &Collection) -> error::KatabasisResult<()> {
    let state = KatabasisApp::get().await?;
    let collection_handler = get_collection_handler(&collection.game.get_loader());

    collection_handler.install_collection(collection, &state).await?;

    Ok(())
}

/// Initialises and adds a new [`Plugin`] to the provided
/// collection, uses the url to parse the source.
pub async fn add_plugin(
    collection: &Collection,
    plugin_url: &str
) -> error::KatabasisResult<Plugin> {
    let state = KatabasisApp::get().await?;

    let plugin_handler = get_downloader(
        &collection.game,
        plugin_url
    )?;

    let mut plugin = plugin_handler.initialise_plugin(
        &state,
        plugin_url
    ).await?;

    // TODO: Retry this step on failure
    plugin_handler.download_latest(
        &state,
        collection,
        &mut plugin
    ).await?;

    plugin_repository::upsert(
        collection,
        &plugin,
        &state.db_pool
    ).await?;

    Ok(plugin)
}

/// Returns the theoretical path to the collection's directory.
pub async fn directory(collection: &Collection) -> error::KatabasisResult<PathBuf> {
    let state = KatabasisApp::get().await?;

    Ok(state.directories.collection_dir(&collection.id))
}
