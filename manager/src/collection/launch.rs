use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

use crate::targets::{ModLoader, ModLoaderKind};
use eyre::{OptionExt, Result};
use tokio::process::Command;

pub async fn add_loader_args(
    command: &mut Command,
    dir: &Path,
    mod_loader: &ModLoader<'_>,
) -> Result<()> {
    match mod_loader.kind {
        ModLoaderKind::BepInEx => add_bepinex_args(command, dir).await,
    }
}

async fn add_bepinex_args(command: &mut Command, dir: &Path) -> Result<()> {
    let entry_dll = bepinex_dll_path(dir).await?;

    command
        .args(["--doorstop-enable", "true", "--doorstop-target"])
        .arg(entry_dll);

    Ok(())
}

async fn bepinex_dll_path(dir: &Path) -> Result<PathBuf> {
    let core_dir = dir.join("BepInEx").join("core");

    const DLL_NAMES: &[&str] = &[
        "BepInEx.Unity.Mono.Preloader.dll",
        "BepInEx.Unity.IL2CPP.dll",
        "BepInEx.Preloader.dll",
        "BepInEx.IL2CPP.dll",
    ];

    let dll_path = tokio::task::spawn_blocking(move || -> Result<PathBuf> {
        let result = core_dir
            .read_dir()?
            .filter_map(|x| x.ok())
            .find(|x| {
                DLL_NAMES
                    .into_iter()
                    .any(|name| x.file_name().to_str() == Some(name))
            })
            .ok_or_eyre("failed to find bepinex entry dll")?
            .path();

        Ok(result)
    })
    .await??;

    Ok(dll_path)
}

/// Returns the path to the game's executable.
pub async fn app_path(app_dir: &Path) -> Result<PathBuf> {
    let app_dir = app_dir.to_path_buf();

    Ok(tokio::task::spawn_blocking(move || -> Result<PathBuf> {
        app_dir
            .read_dir()?
            .filter_map(Result::ok)
            .find(|x| {
                let file_name = PathBuf::from(x.file_name());
                let ext = file_name.extension().and_then(|ext| ext.to_str());

                let correct_ext = if cfg!(windows) {
                    matches!(ext, Some("exe"))
                } else {
                    matches!(ext, Some("sh"))
                };

                correct_ext
                    && !file_name.to_string_lossy().contains("UnityCrashHandler")
                    && !file_name.to_string_lossy().contains("server")
            })
            .map(|entry| entry.path())
            .ok_or_eyre("failed to find the game's executable")
    })
    .await??)
}

/// Copies the files to the game directory that are required
/// for launch. Such as `winhttp.dll` for `BepInEx`.
pub async fn link_files(collection_dir: &Path, game_dir: &Path) -> Result<()> {
    let collection_dir = collection_dir.to_path_buf();

    let files = tokio::task::spawn_blocking(move || -> Result<Vec<DirEntry>> {
        let files = collection_dir
            .read_dir()?
            .filter_map(|x| x.ok())
            .filter(|entry| {
                entry.file_type().is_ok_and(|ft| ft.is_file()) || entry.file_name() == "dotnet"
            })
            .collect();

        Ok(files)
    })
    .await??;

    for file in files {
        tracing::info!(
            "linking file {} to game dir",
            file.file_name().to_string_lossy()
        );

        if file.file_type().is_ok_and(|f| f.is_file()) {
            tokio::fs::copy(file.path(), game_dir.join(file.file_name())).await?;
        }

        // TODO: Copy directories here too...
    }

    Ok(())
}
