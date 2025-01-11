use std::path::Path;
use reqwest::Method;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::setup::utils::get_latest_bepinex;
use crate::storage::KbApp;
use crate::{SupportedGames, SupportedLoaders};
use crate::SupportedLoaders::BepInEx;
use crate::utils::download;

/// This module contains setup code for all supported games/mod loaders

const BPX_REQUIRED_FILES: &'static [&'static str] = &["BepInEx", "doorstop_config.ini", "winhttp.dll"];

pub mod games;
mod utils;

// TODO: Should possibly swap the is_setup function to use SupportedGames enum rather than relying on the caller to provide the correct path

/// Interface for handling game mod loader setup.
pub trait SetupLoader {
    fn is_setup<P: AsRef<Path>>(&self, target_dir: P) -> crate::Result<bool>;

    async fn setup_game<P: AsRef<Path>>(&self, target_dir: P) -> crate::Result<()>;

    async fn create_mod_symlinks<P: AsRef<Path>>(&self, mod_dir: P, game_type: &SupportedGames) -> crate::Result<()>;
}

/// Returns the correct setup tool for the target loader
pub async fn get_setup_tool(target_loader: SupportedLoaders) -> crate::Result<impl SetupLoader> {
    match target_loader {
        BepInEx => Ok(BepInExLoader),
    }
}

pub struct BepInExLoader;

impl SetupLoader for BepInExLoader {
    fn is_setup<P: AsRef<Path>>(&self, target_dir: P) -> crate::Result<bool> {
        let all_files = std::fs::read_dir(target_dir.as_ref()).map_err(|e| {
            crate::Error::FileSystemError(
                format!("Failed to fetch all files in directory '{:?}': {:?}", target_dir.as_ref(), e)
            )
        })?;

        let parsed_files = all_files
            .into_iter()
            .map(|file| { file.unwrap().file_name().to_string_lossy().to_string() })
            .collect::<Vec<String>>();

        for file in BPX_REQUIRED_FILES {
            if !parsed_files.contains(&file.to_string()) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    async fn setup_game<P: AsRef<Path>>(&self, target_dir: P) -> crate::Result<()> {
        let state = KbApp::get().await?;

        let (file_name, download_url) = get_latest_bepinex(&state.net_semaphore).await?;
        let download_path = state.directories.loaders_dir().join(&file_name);

        // Download BepInEx install zip if it doesn't exist on disk, otherwise prefer to use cached version
        if !download_path.exists() {
            let downloaded_bytes = download::fetch_url(
                Method::GET,
                &download_url,
                &state.net_semaphore
            ).await?;

            let mut download_file: fs::File = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&download_path)
                .await?;

            download_file.write_all(&downloaded_bytes).await?;
        }

        let mut zip = zip::ZipArchive::new(std::fs::File::open(download_path)?)?;
        zip.extract(target_dir.as_ref())?;

        Ok(())
    }

    async fn create_mod_symlinks<P: AsRef<Path>>(&self, mod_dir: P, game_type: &SupportedGames) -> crate::Result<()> {
        // Cloning this value to pass to blocking task (there is probably a better solution)
        let clone_dir = mod_dir.as_ref().to_path_buf();

        let read_dir = tokio::task::spawn_blocking(
            move || {
                std::fs::read_dir(clone_dir)
            }
        ).await??;

        let install_dir = game_type
            .get_game_dir()?
            .join("BepInEx")
            .join("plugins");

        // This ensures the plugin directory exists, even if BepInEx hasn't been run yet.
        if !install_dir.exists() {
            fs::create_dir_all(&install_dir).await?;
        }

        for file in read_dir {
            let entry = file?;
            let result: crate::Result<()>;

            // Create symlink if file (assuming dll) else copy directories
            if entry.path().is_file() {
                result = utils::symlink_file(entry.path(), install_dir.join(entry.file_name()));
            }
            else {
                result = utils::symlink_dir(entry.path(), install_dir.join(entry.file_name()));
            }

            if result.is_err() {
                println!("Failed to create symlinks, please run this command as admin on Windows...");
            }
        }

        Ok(())
    }
}
