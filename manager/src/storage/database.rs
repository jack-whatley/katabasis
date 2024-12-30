use std::str::FromStr;
use std::time::Duration;
use sqlx::{Pool, Sqlite};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use tokio::fs;
use crate::storage::dir::Directories;
use crate::error;

pub(crate) const DB_NAME: &'static str = "katabasis.db";

pub(crate) async fn connect() -> crate::Result<Pool<Sqlite>> {
    let app_dir = Directories::get_default_dir().ok_or(
        error::Error::FileSystemError(
            "Failed to find app directory".to_string()
        )
    )?;

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).await.map_err(|err| {
            error::Error::FileSystemError(
                format!("Failed to create app directory: {}", err)
            )
        })?;
    }

    let db_uri = format!("sqlite:{}", app_dir.join(DB_NAME).display());

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