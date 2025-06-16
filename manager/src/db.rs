use std::{str::FromStr, time::Duration};

use eyre::{eyre, Context, OptionExt, Result};
use sqlx::{
    Pool, Sqlite,
    migrate::MigrateDatabase,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::{collection::Collection, targets, utils};

pub struct Db(Pool<Sqlite>);

impl Db {
    /// Initialises a new instance of [`Db`] and returns
    /// whether the database existed already.
    pub async fn init() -> Result<(Self, bool)> {
        let db_path = utils::paths::db_path();
        let db_exists = db_path.exists();

        tracing::info!(
            "Initialising database connection to '{}' (exists: {})",
            db_path.display(),
            db_exists
        );

        let db_uri = format!("sqlite:{}", db_path.display());

        if !Sqlite::database_exists(&db_uri).await? {
            Sqlite::create_database(&db_uri)
                .await
                .context("Failed to create the database file")?;
        }

        let sqlite_opts = SqliteConnectOptions::from_str(&db_uri)?
            .busy_timeout(Duration::from_secs(30))
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .optimize_on_close(true, None);

        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(100)
            .connect_with(sqlite_opts)
            .await
            .context("Failed to open connection to database")?;

        sqlx::migrate!().run(&sqlite_pool).await?;

        Ok((Self(sqlite_pool), db_exists))
    }

    /// Executes a database query async closure within a transaction.
    async fn with_transaction<F>(&self, f: F) -> Result<()>
    where
        F: AsyncFnOnce(&mut sqlx::Transaction<'_, Sqlite>) -> Result<()>,
    {
        let mut tx = self.0.begin().await?;

        f(&mut tx)
            .await
            .context("Failed to execute database query, rolling back transaction")?;

        tx.commit().await?;

        Ok(())
    }

    /// Saves a single [`Collection`] to the application database.
    pub async fn save_collection(&self, collection: &Collection) -> Result<()> {
        self.with_transaction(async |tx| {
            let json_plugins = serde_json::to_string(&collection.plugins)?;

            sqlx::query!(
                "INSERT OR REPLACE INTO collections (
                    name, plugins, game
                ) VALUES (
                    $1, $2, $3
                )",
                collection.name,
                json_plugins,
                collection.game.slug
            )
            .execute(&mut **tx)
            .await?;

            Ok(())
        })
        .await?;

        Ok(())
    }

    pub async fn load_collection(&self, id: &str) -> Result<Collection> {
        let record = sqlx::query!("SELECT name, plugins, game FROM collections WHERE name = $1", id)
            .fetch_one(&self.0)
            .await?;

        Ok(Collection {
            name: record.name,
            plugins: serde_json::from_str(&record.plugins)?,
            game: targets::from_slug(&record.game).ok_or_else(||
                eyre!("Slug '{}' does not match any supported games", &record.game))?,
        })
    }

    pub async fn load_all_collections(&self) -> Result<Vec<Collection>> {
        let collections = sqlx::query!("SELECT name, plugins, game FROM collections")
            .fetch_all(&self.0)
            .await?;

        let collections = collections
            .into_iter()
            .map(|row| -> Result<Collection> {
                let plugins = serde_json::from_str(&row.plugins)?;
                let game = targets::from_slug(&row.game).ok_or_eyre("target is not recognised")?;

                Ok(Collection {
                    name: row.name,
                    plugins,
                    game,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(collections)
    }

    pub async fn remove_collection(&self, collection: &Collection) -> Result<()> {
        sqlx::query!("DELETE FROM collections WHERE name = $1", collection.name).execute(&self.0).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_in_memory_database() -> Db {
        let db_uri = "sqlite::memory:";

        let sqlite_opts = SqliteConnectOptions::from_str(&db_uri)
            .unwrap()
            .busy_timeout(Duration::from_secs(30))
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .optimize_on_close(true, None);

        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(100)
            .connect_with(sqlite_opts)
            .await
            .context("Failed to open connection to database")
            .unwrap();

        sqlx::migrate!().run(&sqlite_pool).await.unwrap();

        Db(sqlite_pool)
    }

    #[tokio::test]
    async fn save_and_load_collections() {
        let db = create_in_memory_database().await;

        let collection = Collection {
            name: "EXAMPLE".to_owned(),
            game: targets::from_slug("valheim").unwrap(),
            plugins: vec![],
        };

        assert!(db.save_collection(&collection).await.is_ok());

        let save_data = db.load_all_collections().await.unwrap();

        assert_eq!(save_data.len(), 1);
        assert_eq!(save_data[0].name, "EXAMPLE");
    }
}
