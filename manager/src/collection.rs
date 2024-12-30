use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use tokio::fs;
use crate::setup::games::SupportedGames;

/// The data structure used for storing mod collections in katabasis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub game: SupportedGames,
    pub game_version: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub last_played: Option<DateTime<Utc>>,
}

/// An intermediate state of a collections just retrieved from a get related query,
/// it still needs to be parsed before being ready for use.
struct IntermediateCollection {
    id: String,
    name: String,
    game: SupportedGames,
    game_version: String,
    created: i64,
    modified: i64,
    last_played: Option<i64>,
}

impl TryFrom<IntermediateCollection> for Collection {
    type Error = crate::Error;

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
                .and_then(|x|
                    Utc.timestamp_opt(x, 0).single()
                )
        })
    }
}

impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Collection: '{}'\t[{}, {}, {}]", self.name, self.game, self.game_version, self.id)
    }
}

/// Macro for generating fetch queries for collections,
/// including parsing them into an [`IntermediateCollection`]
macro_rules! fetch_query {
    ($sql_condition:tt, $($args:tt)*) => {
        sqlx::query_as!(
            IntermediateCollection,
            r#"
                SELECT id, name, game, game_version, created, modified, last_played
                FROM collections
            "#
            + $sql_condition,
            $($args)*
        )
    };
}

// The repository pattern style implementation for katabasis mod collections
impl Collection {
    /// Fetches a [`Collection`] based on the ID provided.
    pub async fn get(
        id: &str,
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
    ) -> crate::Result<Option<Self>> {
        Ok(Self::get_many(&[id], db).await?.into_iter().next())
    }

    /// Fetches a [`Collection`] per ID provided in the array.
    pub async fn get_many(
        ids: &[&str],
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
    ) -> crate::Result<Vec<Self>> {
        let joined_ids = ids.join(", ");

        let query_results: Vec<IntermediateCollection> = fetch_query!(
            "WHERE id IN ($1)",
            joined_ids
        ).fetch_all(db).await?;

        query_results
            .into_iter()
            .map(|r| r.try_into())
            .collect::<crate::Result<Vec<_>>>()
    }

    /// Fetches all the [`Collection`] structs found within the app database.
    pub async fn get_all(
        limit: Option<u32>,
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
    ) -> crate::Result<Vec<Self>> {
        let true_limit = limit.unwrap_or(1000);

        let query_results: Vec<IntermediateCollection> = fetch_query!(
            "
                LIMIT $1
            ",
            true_limit
        ).fetch_all(db).await?;

        query_results
            .into_iter()
            .map(|r| r.try_into())
            .collect::<crate::Result<Vec<_>>>()
    }

    /// Inserts or updates an already existing [`Collection`] struct into the applications SQLite database.
    pub async fn update(
        &self,
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
    ) -> crate::Result<()> {
        let created_timestamp = self.created.timestamp();
        let modified_timestamp = self.modified.timestamp();
        let last_played = self.last_played.map(|x| x.timestamp());

        sqlx::query!(
            "
                INSERT INTO collections (
                    id, name, game, game_version, created, modified, last_played
                )
                VALUES (
                    $1, $2, $3, $4, $5, $6, $7
                )
                ON CONFLICT (id) DO UPDATE SET
                    name = $2,
                    game = $3,
                    game_version = $4,
                    created = $5,
                    modified = $6,
                    last_played = $7
            ",
            self.id,
            self.name,
            self.game,
            self.game_version,
            created_timestamp,
            modified_timestamp,
            last_played,
        ).execute(db).await?;

        Ok(())
    }

    /// Removes a [`Collection`] from the database.
    pub async fn remove(
        &self,
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
    ) -> crate::Result<()> {
        sqlx::query!(
            "
                DELETE FROM collections WHERE id = $1
            ",
            self.id
        ).execute(db).await?;

        if let Ok(path) = crate::public::collections::get_full_path(&self.id).await {
            if path.exists() {
                fs::remove_dir_all(&path).await.map_err(|e| {
                   crate::Error::FileSystemError(
                       format!("Failed to remove directory '{}': {}", path.display(), e)
                   )
                })?;
            }
        }

        Ok(())
    }
}
