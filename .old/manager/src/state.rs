use std::sync::Arc;
use crate::database::Db;
use eyre::{Result, WrapErr};
use tokio::sync::OnceCell;

static APPLICATION: OnceCell<Arc<AppState>> = OnceCell::const_new();

/// Application state singleton.
pub struct AppState {
    pub db: Db
}

impl AppState {
    pub async fn init() -> Result<Arc<Self>> {
        Ok(Arc::clone(APPLICATION.get_or_try_init(Self::_init).await?))
    }

    async fn _init() -> Result<Arc<Self>> {
        let database = Db::init().await
            .wrap_err("Failed to initialise the application database")?;

        tracing::info!("Successfully initialised the application");

        Ok(Arc::new(Self {
            db: database
        }))
    }
}
