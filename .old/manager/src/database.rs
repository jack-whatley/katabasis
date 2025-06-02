use std::str::FromStr;
use std::time::Duration;
use sqlx::{Pool, Sqlite};
use eyre::Result;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use crate::utils;

pub const FILE_NAME: &str = "katabasis.db";

pub const DEV_FILE_NAME: &str = "katabasis.db";

/// Database management struct. Owns the database pool and contains
/// all the methods for querying and saving data.
pub struct Db {
    pool: Pool<Sqlite>
}

impl Db {
    pub async fn init() -> Result<Self> {
        let mut db_path = utils::path::default_app_data_dir().join(FILE_NAME);
        let existed = db_path.exists();

        #[cfg(debug_assertions)]
        db_path.set_file_name(DEV_FILE_NAME);

        tracing::info!(
            "Initialising Database connection at '{}' (exists: {})",
            db_path.display(),
            existed
        );

        let db_uri = format!("sqlite:{}", db_path.display());

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

        Ok(Self {
            pool: sql_pool
        })
    }
}
