use eyre::Result;
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::{db::Db, utils::net};

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub struct AppState {
    db: Db,
    http: reqwest::Client,
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

    /// Returns the HTTP client.
    pub fn http(&self) -> &reqwest::Client {
        &self.http
    }
    
    pub fn db(&self) -> &Db {
        &self.db
    }

    async fn init() -> Result<Arc<Self>> {
        let app_dir = crate::utils::paths::default_app_dir();

        // Initialising the application directory, should always exist.
        if !app_dir.exists() {
            tokio::fs::create_dir_all(&app_dir).await?;
        }

        let (db, db_existed) = Db::init().await?;
        let http = net::init()?;

        let app_state = Self {
            db,
            http,
            db_existed,
        };

        Ok(Arc::new(app_state))
    }
}
