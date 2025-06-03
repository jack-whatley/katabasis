use eyre::Result;
use std::sync::Arc;
use tokio::sync::OnceCell;

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub struct AppState;

impl AppState {
    /// Fetches the [`AppState`] singleton struct that provides access
    /// to all the application essentials.
    pub async fn get() -> Result<Arc<Self>> {
        Ok(Arc::clone(APP_STATE.get_or_try_init(Self::init).await?))
    }

    async fn init() -> Result<Arc<Self>> {
        let app_dir = crate::utils::paths::default_app_dir();
        let app_state = Self {};

        // Initialising the application directory, should always exist.
        if !app_dir.exists() {
            tokio::fs::create_dir_all(&app_dir).await?;
        }

        Ok(Arc::new(app_state))
    }
}
