use std::path::Path;
use std::str::FromStr;
use std::time::Duration;
use sqlx::{Pool, Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use crate::error;

pub mod collection_repository;

pub mod settings_repository;

const DB_NAME: &str = "katabasis.db";

pub(crate) async fn connect_database(database_directory: impl AsRef<Path>) -> error::KatabasisResult<Pool<Sqlite>> {
    let mut db_path = database_directory
        .as_ref()
        .join(DB_NAME)
        .display()
        .to_string();

    #[cfg(debug_assertions)]
    {
        db_path = db_path.replace(".db", "_dev.db");
    }

    let db_uri = format!("sqlite:{}", db_path);

    if !Sqlite::database_exists(&db_uri).await? {
        Sqlite::create_database(&db_uri).await?;
    }

    let sql_options = SqliteConnectOptions::from_str(&db_uri)?
        .busy_timeout(Duration::from_secs(30))
        .journal_mode(SqliteJournalMode::Wal)
        .optimize_on_close(true, None);

    let sql_pool = SqlitePoolOptions::new()
        .max_connections(100)
        .connect_with(sql_options)
        .await?;

    sqlx::migrate!().run(&sql_pool).await?;

    Ok(sql_pool)
}

/// Initialise a testing in-memory database.
#[allow(dead_code)]
pub(crate) async fn initialise_database() -> SqlitePool {
    let sql_options = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .busy_timeout(Duration::from_secs(30))
        .journal_mode(SqliteJournalMode::Wal)
        .optimize_on_close(true, None);

    let pool = SqlitePoolOptions::new()
        .max_connections(100)
        .connect_with(sql_options)
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    pool
}
