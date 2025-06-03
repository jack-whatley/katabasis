use eyre::Result;
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::db::Db;

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub struct AppState {
    db: Db,
    db_existed: bool,
}

impl AppState {
    /// Fetches the [`AppState`] singleton struct that provides access
    /// to all the application essentials.
    pub async fn get() -> Result<Arc<Self>> {
        Ok(Arc::clone(APP_STATE.get_or_try_init(Self::init).await?))
    }

    /// Returns whether the database existed on startup. Used
    /// to work out if this is the first time the application
    /// has been run.
    pub fn db_existed(&self) -> bool {
        self.db_existed
    }

    async fn init() -> Result<Arc<Self>> {
        let app_dir = crate::utils::paths::default_app_dir();

        let (db, db_existed) = Db::init().await?;

        let app_state = Self { db, db_existed };

        // Initialising the application directory, should always exist.
        if !app_dir.exists() {
            tokio::fs::create_dir_all(&app_dir).await?;
        }

        Ok(Arc::new(app_state))
    }
}
