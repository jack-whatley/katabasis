use chrono::Utc;
use tokio::fs;
use uuid::Uuid;
use crate::collection::Collection;
use crate::error;
use crate::storage::KbApp;
use crate::setup::games::SupportedGames;
use crate::setup;
use crate::setup::SetupLoader;

pub async fn create(
    name: String,
    game: SupportedGames,
    game_version: String
) -> crate::Result<String> {
    let state = KbApp::get().await?;
    let uuid = Uuid::new_v4();

    let current_time = Utc::now();

    let loader_setup = setup::get_setup_tool(game.get_loader()).await?;
    let game_dir = game.get_game_dir()?;

    if !loader_setup.is_setup(&game_dir)? {
        loader_setup.setup_game(&game_dir).await?;
    }

    let collection = Collection {
        id: format!("{}", uuid.as_hyphenated()),
        name,
        game,
        game_version,
        created: current_time,
        modified: current_time,
        last_played: None
    };

    collection.update(&state.db_pool).await?;

    fs::create_dir_all(state.directories.collection(&collection.id)).await.map_err(|err| {
        error::Error::FileSystemError(
            format!("Failed to create specific collection directory: {}", err)
        )
    })?;

    fs::create_dir_all(state.directories.collection_plugin_dir(&collection.id)).await.map_err(|err| {
        error::Error::FileSystemError(
            format!("Failed to create specific collection plugin directory: {}", err)
        )
    })?;

    Ok(collection.id)
}