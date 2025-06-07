use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::thunderstore::version::VersionIdent;

#[derive(Debug)]
pub struct Collection {
    /// Acts as the primary key for the collection, should be unique.
    pub name: String,
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub id: String,
    pub enabled: bool,
    pub install_time: DateTime<Utc>,
    #[serde(flatten)]
    pub kind: PluginType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PluginType {
    Thunderstore(ThunderstorePlugin),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThunderstorePlugin {
    pub ident: VersionIdent,
}
