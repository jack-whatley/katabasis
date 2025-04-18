use std::path::PathBuf;
use log::{error, warn};
use strum::IntoEnumIterator;
use crate::data::support::PluginTarget;
use crate::error;

/// Locates a game on the users computer, returning the full path
/// to its working directory (with the exe in).
#[tracing::instrument]
pub async fn find_game(target: &PluginTarget) -> error::KatabasisResult<PathBuf> {
    match target {
        PluginTarget::LethalCompany => find_steam_game(target).await,
    }
}

pub async fn list_installed() -> error::KatabasisResult<Vec<PluginTarget>> {
    let mut installed_games: Vec<PluginTarget> = vec![];

    for target in PluginTarget::iter() {
        match find_game(&target).await {
            Ok(_) => installed_games.push(target),
            Err(err) => {
                warn!("Failed to check installation status of game {:#?}:\n{:#?}", target, err);
            }
        }
    }

    Ok(installed_games)
}

async fn find_steam_game(target: &PluginTarget) -> error::KatabasisResult<PathBuf> {
    let app_id = match target {
        PluginTarget::LethalCompany => 1966720u32
    };

    let search_result = tokio::task::spawn_blocking(
        move || -> error::KatabasisResult<PathBuf> {
            let steam_dir = steamlocate::SteamDir::locate()?;

            let (app, library) = match steam_dir.find_app(app_id)? {
                Some(app) => app,
                None => return Err(
                    error::KatabasisErrorKind::FSError(
                        format!("Failed to locate steam game with ID: {}", app_id)
                    ).into()
                )
            };

            Ok(library.resolve_app_dir(&app))
        }
    ).await;

    match search_result {
        Ok(locate_result) => {
            match locate_result {
                Ok(path) => Ok(path),
                Err(err) => {
                    error!("Encountered error whilst searching for steam game:\n{:#?}", err);
                    Err(err)
                }
            }
        },
        Err(err) => {
            error!("Failed to join spawned steam locate task:\n{:#?}", err);
            Err(err.into())
        }
    }
}
