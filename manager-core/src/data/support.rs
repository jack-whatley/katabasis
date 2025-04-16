use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum PluginTarget {
    LethalCompany,
}

impl PluginTarget {
    /// Gets the Steam ID of a game, will return 0 for non-Steam games.
    pub fn get_id(&self) -> u32 {
        match self {
            PluginTarget::LethalCompany => 1966720,
        }
    }

    /// Gets the [`PluginLoader`] of a specific game.
    pub fn get_loader(&self) -> PluginLoader {
        match self {
            PluginTarget::LethalCompany => PluginLoader::BepInEx,
        }
    }
}

impl From<String> for PluginTarget {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "lethalcompany" => PluginTarget::LethalCompany,
            _ => PluginTarget::LethalCompany,
        }
    }
}

impl Display for PluginTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PluginLoader {
    BepInEx
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum PluginSource {
    Thunderstore
}

impl From<String> for PluginSource {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "thunderstore" => PluginSource::Thunderstore,
            _ => PluginSource::Thunderstore
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PluginTarget, PluginLoader};

    #[test]
    fn lethal_company_to_string() {
        let string_name = PluginTarget::LethalCompany.to_string();

        assert_eq!(string_name, "LethalCompany");
    }

    #[test]
    fn string_to_lethal_company() {
        let string_name = "lethalcompany";

        assert_eq!(PluginTarget::from(string_name.to_string()), PluginTarget::LethalCompany);
    }

    #[test]
    fn lethal_company_correct_loader_id() {
        assert_eq!(PluginTarget::LethalCompany.get_loader(), PluginLoader::BepInEx);
        assert_eq!(PluginTarget::LethalCompany.get_id(), 1966720u32);
    }
}
