use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Game {
    LethalCompany,
}

impl Game {
    /// Gets the Steam ID of a game, will return 0 for non-Steam games.
    pub fn get_id(&self) -> u32 {
        match self {
            Game::LethalCompany => 1966720,
        }
    }

    /// Gets the [`Loader`] of a specific game.
    pub fn get_loader(&self) -> Loader {
        match self {
            Game::LethalCompany => Loader::BepInEx,
        }
    }
}

impl From<String> for Game {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "lethalcompany" => Game::LethalCompany,
            _ => Game::LethalCompany,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Loader {
    BepInEx
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Source {
    Thunderstore
}

impl From<String> for Source {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "thunderstore" => Source::Thunderstore,
            _ => Source::Thunderstore
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Loader};

    #[test]
    fn lethal_company_to_string() {
        let string_name = Game::LethalCompany.to_string();

        assert_eq!(string_name, "LethalCompany");
    }

    #[test]
    fn string_to_lethal_company() {
        let string_name = "lethalcompany";

        assert_eq!(Game::from(string_name.to_string()), Game::LethalCompany);
    }

    #[test]
    fn lethal_company_correct_loader_id() {
        assert_eq!(Game::LethalCompany.get_loader(), Loader::BepInEx);
        assert_eq!(Game::LethalCompany.get_id(), 1966720u32);
    }
}
