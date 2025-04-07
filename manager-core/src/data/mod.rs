use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use crate::data::support::{Game, Source};
use crate::error;

pub mod support;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub game: Game,
    pub game_version: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub last_played: Option<DateTime<Utc>>
}

impl PartialEq for Collection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.game == other.game &&
        self.game_version == other.game_version
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub source: Source,
    pub api_url: String,
    pub version: String,
    pub is_enabled: bool
}

pub(crate) struct IntermediateCollection {
    pub id: String,
    pub name: String,
    pub game: Game,
    pub game_version: String,
    pub created: i64,
    pub modified: i64,
    pub last_played: Option<i64>,
}

impl TryFrom<IntermediateCollection> for Collection {
    type Error = error::KatabasisError;

    fn try_from(value: IntermediateCollection) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: value.name,
            game: value.game,
            game_version: value.game_version,
            created: Utc
                .timestamp_opt(value.created, 0)
                .single()
                .unwrap_or_else(Utc::now),
            modified: Utc
                .timestamp_opt(value.modified, 0)
                .single()
                .unwrap_or_else(Utc::now),
            last_played: value
                .last_played
                .and_then(|x| Utc.timestamp_opt(x, 0).single())
        })
    }
}
