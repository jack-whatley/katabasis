use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::thunderstore::version::VersionIdent;

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub latest: PackageVersion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageVersion {
    #[serde(rename = "full_name")]
    pub ident: VersionIdent,
    pub description: String,
    pub icon: String,
    pub dependencies: Vec<VersionIdent>,
    pub download_url: String,
    pub downloads: i64,
    pub date_created: DateTime<Utc>,
    pub website_url: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageMetric {
    pub downloads: i64,
    pub rating_score: i64,
    pub latest_version: String,
}
