use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::thunderstore::version::VersionIdent;

#[derive(Debug)]
pub struct Collection {
    /// Acts as the primary key for the collection, should be unique.
    pub name: String,
    pub plugins: Vec<Plugin>,
}

#[derive(Debug)]
pub struct Plugin {
    pub id: String,
    pub enabled: bool,
    pub install_time: DateTime<Utc>,
    pub kind: PluginType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PluginType {
    Thunderstore(ThunderstorePlugin),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThunderstorePlugin {
    pub ident: VersionIdent,
}
