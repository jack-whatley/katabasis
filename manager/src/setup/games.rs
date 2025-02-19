use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use steamlocate::SteamDir;

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum SupportedGames {
    LethalCompany,
}

#[derive(Debug, Clone)]
pub enum SupportedLoaders {
    BepInEx
}

impl SupportedGames {
    pub fn get_game_id(&self) -> u32 {
        match self {
            SupportedGames::LethalCompany => 1966720
        }
    }

    pub fn get_loader(&self) -> SupportedLoaders {
        match self {
            SupportedGames::LethalCompany => SupportedLoaders::BepInEx,
        }
    }

    /// Returns the directory of the game executable.
    pub fn get_game_dir(&self) -> crate::Result<PathBuf> {
        let steam_dir = SteamDir::locate()?;

        let (game, lib) = steam_dir.find_app(self.get_game_id())?.ok_or(
            Error::FileSystemError(
                format!("Failed to find steam game with id: '{}'", self.get_game_id())
            )
        )?;

        Ok(lib.resolve_app_dir(&game))
    }
}

impl FromStr for SupportedGames {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lethalcompany" => Ok(Self::LethalCompany),
            "lethal-company" => Ok(Self::LethalCompany),
            "lethal company" => Ok(Self::LethalCompany),
            "lc" => Ok(Self::LethalCompany),
            _ => Err(
                Error::GameStringParseError(
                    format!("Failed to parse provided game string: '{}'", s)
                )
            ), // default to lc
        }
    }
}

/// This implementation is required for sqlx conversions.
impl From<String> for SupportedGames {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}

impl std::fmt::Display for SupportedGames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
