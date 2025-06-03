use std::{str::FromStr, time::Duration};

use eyre::{Context, Result};
use sqlx::{
    Pool, Sqlite,
    migrate::MigrateDatabase,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::utils;

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

        // TODO: Implement as part of KB-5.
        // sqlx::migrate!().run(&sqlite_pool).await?;

        Ok((Self(sqlite_pool), db_exists))
    }

    /// Executes a database query async closure within a transaction.
    pub async fn with_transaction<F, Fut>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut sqlx::Transaction<'_, Sqlite>) -> Fut,
        Fut: Future<Output = Result<()>>,
    {
        let mut tx = self.0.begin().await?;

        f(&mut tx)
            .await
            .context("Failed to execute database query, rolling back transaction")?;

        tx.commit().await?;

        Ok(())
    }
}
