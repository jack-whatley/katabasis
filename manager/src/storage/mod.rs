use std::sync::Arc;
use tokio::sync::{OnceCell, Semaphore};
use crate::storage::dir::Directories;

mod dir;
mod database;
pub mod plugin;

static KB_STATE: OnceCell<Arc<KbApp>> = OnceCell::const_new();

#[derive(Debug)]
pub struct NetSemaphore(pub Semaphore);

pub struct KbApp {
    pub directories: Directories,
    pub(crate) db_pool: sqlx::SqlitePool,
    pub(crate) net_semaphore: NetSemaphore,
}

impl KbApp {
    /// Fetch an arc reference to the katabasis backend state management struct.
    pub async fn get() -> crate::Result<Arc<Self>> {
        Ok(Arc::clone(KB_STATE.get_or_try_init(Self::initialise_kb_state).await?))
    }

    async fn initialise_kb_state() -> crate::Result<Arc<KbApp>> {
        let db_pool = database::connect().await?;
        let directories = Directories::init().await?;

        let net_semaphore = NetSemaphore(Semaphore::new(10));

        Ok(
            Arc::new(
                Self {
                    directories,
                    db_pool,
                    net_semaphore
                }
            )
        )
    }
}
