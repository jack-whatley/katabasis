use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{targets::Target, thunderstore::version::VersionIdent};

pub mod install;
pub mod launch;

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

    pub fn ident(&self) -> &VersionIdent {
        match self.kind {
            PluginType::Thunderstore { ref ident } => ident,
        }
    }
}
