use crate::{error, storage};
use crate::state::directories::Directories;
use std::sync::Arc;
use tokio::sync::OnceCell;

mod directories;

static KATABASIS_APPLICATION: OnceCell<Arc<KatabasisApp>> = OnceCell::const_new();

/// The Katabasis Mod Manager Application State Singleton
pub struct KatabasisApp {
    pub directories: Directories,
    pub db_pool: sqlx::SqlitePool,
}

impl KatabasisApp {
    /// Fetches or lazily initialises the application state asynchronously.
    pub async fn get() -> error::KatabasisResult<Arc<Self>> {
        Ok(Arc::clone(KATABASIS_APPLICATION.get_or_try_init(Self::initialise_app).await?))
    }

    /// Initialises the application state, should include all required setup tasks.
    #[tracing::instrument]
    async fn initialise_app() -> error::KatabasisResult<Arc<Self>> {
        let directories = Directories::init(None).await?;
        let db_pool = storage::connect_database(
            directories.working_dir.as_path()).await?;

        Ok(Arc::new(Self {
            directories,
            db_pool,
        }))
    }
}
