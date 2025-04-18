use std::fmt::Display;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// The supported methods for installing a collection to
/// the target game directory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum InstallType {
    /// Copies all files inside the collection directory into
    /// the game directory. Overwriting any already existing ones
    /// in the process.
    Copy,
}

impl From<String> for InstallType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "copy" => Self::Copy,
            _ => Self::Copy,
        }
    }
}

/// An enum representing all supported installation targets. Effectively
/// meaning all games that are supported.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, EnumIter, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum PluginTarget {
    LethalCompany,
}

impl PluginTarget {
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
    }
}
