use std::path::PathBuf;
use async_trait::async_trait;
use log::{error, info, warn};
use serde_json::Value;
use manager_core::data::Collection;
use manager_core::data::support::InstallType;
use manager_core::error;
use manager_core::state::KatabasisApp;
use manager_core::utils::{fs, net};
use manager_core::utils::net::fetch_stream;
use manager_core::data::locator;
use crate::CollectionHandler;

pub struct BepInExCollectionHandler;

const BEPINEX_FOLDERS: &[&str] = &["core", "config", "plugins"];

#[async_trait]
impl CollectionHandler for BepInExCollectionHandler {
    async fn initialise_collection(
        &self,
        collection: &Collection,
        state: &KatabasisApp,
    ) -> error::KatabasisResult<()> {
        let collection_dir = state.directories.collection_dir(&collection.id);

        fs::create_dir(&collection_dir, false).await?;

        let zip_path = match download_latest_bepinex_v5(None).await {
            Ok(path) => path,
            Err(e) => {
                error!("Failed during process of downloading and unzipping BepInEx:\n{:#?}", e);
                return Err(e.into())
            }
        };

        fs::unzip_file_to_dir(
            &zip_path,
            &collection_dir,
        ).await?;

        for folder in BEPINEX_FOLDERS {
            let complete_path = collection_dir
                .join("BepInEx")
                .join(folder);

            if !complete_path.exists() {
                fs::create_dir(complete_path, false).await?
            }
        }

        Ok(())
    }

    async fn remove_collection(
        &self,
        collection: &Collection,
        state: &KatabasisApp,
    ) -> error::KatabasisResult<()> {
        let collection_dir = state.directories.collection_dir(&collection.id);

        if collection_dir.exists() {
            match tokio::fs::remove_dir_all(&collection_dir).await {
                Ok(()) => {},
                Err(e) => {
                    warn!(
                        "Failed to delete collection at path: '{:#?}'\n{:#?}",
                        collection_dir,
                        e);
                }
            }
        }
        
        Ok(())
    }

    async fn install_collection(
        &self,
        collection: &Collection,
        state: &KatabasisApp,
    ) -> error::KatabasisResult<()> {
        match collection.install_type {
            InstallType::Copy => Ok(install_collection_copy(collection, state).await?)
        }
    }
}

const BEPINEX_URL: &str = "https://api.github.com/repos/BepInEx/BepInEx/releases/latest";

macro_rules! parse_section {
    ($method:expr) => {
        $method.ok_or_else(
            || {
                error!("Failed to parse property: {:#?}", stringify!($method));

                error::KatabasisErrorKind::HttpGeneralError(
                    format!("Failed to parse property: {:#?}", stringify!($method))
                )
            }
        )
    };
}

const BEPINEX_WIN_64: &str = "BepInEx_win_x64_5";

async fn download_latest_bepinex_v5(file_path: Option<PathBuf>) -> error::KatabasisResult<PathBuf> {
    let state = KatabasisApp::get().await?;
    let file_path = file_path.unwrap_or_else(|| {
        state.directories.download_dir().join(format!("{}.zip", BEPINEX_WIN_64))
    });

    let latest_release = net::fetch_json::<Value>(
        BEPINEX_URL,
        &state.net_semaphore,
        &state.http_client,
        state.settings.retry_limit.get()
    ).await?;

    let assets = parse_section!(latest_release["assets"].as_array())?;

    for asset in assets {
        let name = parse_section!(asset["name"].as_str())?;
        if !name.starts_with(BEPINEX_WIN_64) { continue; }

        let url = parse_section!(asset["browser_download_url"].as_str())?;

        let mut download_stream = fetch_stream(
            url,
            &state.net_semaphore,
            &state.http_client,
            state.settings.retry_limit.get()
        ).await?;

        fs::write_stream_to_file(
            &file_path,
            &mut download_stream,
        ).await?;

        info!("Downloaded BepInEx to target file path: '{:#?}'", file_path);

        return Ok(file_path)
    }

    Err(
        error::KatabasisErrorKind::HttpGeneralError(
            format!("Failed to download the latest BepInEx version from GitHub, no match for '{:?}' found.", BEPINEX_WIN_64)
        ).into()
    )
}

/// Installs the collection via copy for BepInEx collections. The
/// process is:
///
/// 1. Wiping the current BepInEx directory if it exists.
/// 2. Copying the current collection folder to the game folder.
async fn install_collection_copy(
    collection: &Collection,
    state: &KatabasisApp
) -> error::KatabasisResult<()> {
    let install_location = locator::find_game(&collection.game).await?;
    let bepinex_location = install_location.join("BepInEx");

    if bepinex_location.exists() {
        fs::remove_dir_all(&bepinex_location).await?;
    }

    fs::create_dir(&bepinex_location, false).await?;

    let collection_dir = state.directories.collection_dir(&collection.id);
    fs::copy_contents_to(collection_dir, &bepinex_location).await?;

    Ok(())
}
