use std::path::PathBuf;

use eyre::{OptionExt, Result, bail};
use tokio::process::Command;

use crate::targets::{Platform, Target};

/// Works out the correct launch command for the provided [`Target`] and [`Platform`].
pub fn launch_command(target: Target, platform: Platform) -> Result<Option<Command>> {
    match platform {
        Platform::Steam => steam_command(target).map(Some),
    }
}

/// Works out the directory of the game based on the provided [`Platform`].
pub fn game_dir(target: Target, platform: Platform) -> Result<PathBuf> {
    match platform {
        Platform::Steam => steam_game_dir(target),
    }
}

fn steam_command(target: Target) -> Result<Command> {
    let Some(steam) = &target.platforms.steam else {
        bail!("Target {} is not available on Steam", target.name);
    };

    let Some(steam_exe) = find_steam_exe() else {
        bail!("Steam executable not found, game should be launched directly");
    };

    let mut command = Command::new(steam_exe);
    command.arg("-applaunch").arg(steam.id.to_string());

    Ok(command)
}

fn find_steam_exe() -> Option<PathBuf> {
    let steam_dir = steamlocate::SteamDir::locate().ok()?;
    let steam_exe = steam_dir.path().join("steam.exe");

    if steam_exe.exists() {
        Some(steam_exe)
    } else {
        tracing::warn!(
            "Steam executable not found, it is likely that another platform is being used"
        );

        None
    }
}

fn steam_game_dir(target: Target) -> Result<PathBuf> {
    let Some(steam) = &target.platforms.steam else {
        bail!("Target {} does not support Steam", target.name);
    };

    let steam_dir = steamlocate::SteamDir::locate()?;

    let (app, lib) = steam_dir.find_app(steam.id)?.ok_or_eyre(format!(
        "Failed to find app {} in Steam library",
        target.name
    ))?;

    Ok(lib.resolve_app_dir(&app))
}
