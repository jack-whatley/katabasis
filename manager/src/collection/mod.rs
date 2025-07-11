use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{targets::Target, thunderstore::version::VersionIdent};

pub mod install;
pub mod launch;
pub mod export;

#[derive(Debug, sqlx::FromRow)]
pub struct Collection {
    /// Acts as the primary key for the collection, should be unique.
    pub name: String,
    pub game: Target,
    #[sqlx(json)]
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub enabled: bool,
    pub install_time: DateTime<Utc>,
    #[serde(flatten)]
    pub kind: PluginType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PluginType {
    Thunderstore { ident: VersionIdent, },
}

impl Plugin {
    pub fn from_ident(ident: &VersionIdent) -> Self {
        Plugin {
            enabled: true,
            install_time: Utc::now(),
            kind: PluginType::Thunderstore { ident: ident.clone(), },
        }
    }

    pub fn from_moved_ident(ident: VersionIdent) -> Self {
        Self {
            enabled: true,
            install_time: Utc::now(),
            kind: PluginType::Thunderstore { ident },
        }
    }

    pub fn ident(&self) -> &VersionIdent {
        match self.kind {
            PluginType::Thunderstore { ref ident } => ident,
        }
    }
}

impl PluginType {
    pub fn ident(&self) -> &VersionIdent {
        match self {
            PluginType::Thunderstore { ident } => ident,
        }
    }

    pub fn name(&self) -> &str {
        self.ident().name()
    }

    pub fn full_name(&self) -> &str {
        self.ident().full_name()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendCollection {
    pub name: String,
    pub target: String,
    pub plugins: Vec<FrontendPlugin>,
    pub mod_loader: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendPlugin {
    pub enabled: bool,
    pub install_time: DateTime<Utc>,
    pub ident: String,
    pub full_name: String,
}

impl Into<FrontendPlugin> for Plugin {
    fn into(self) -> FrontendPlugin {
        FrontendPlugin {
            enabled: self.enabled,
            install_time: self.install_time,
            ident: self.ident().as_str().to_string(),
            full_name: self.ident().full_name().to_string(),
        }
    }
}

impl Into<FrontendCollection> for Collection {
    fn into(self) -> FrontendCollection {
        FrontendCollection {
            name: self.name,
            target: self.game.name.to_owned(),
            plugins: self.plugins.into_iter().map(Into::into).collect(),
            mod_loader: self.game.mod_loader.to_string(),
        }
    }
}
