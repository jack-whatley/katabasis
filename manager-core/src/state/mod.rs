use crate::{error, storage};
use crate::state::directories::Directories;
use std::sync::Arc;
use log::{error, info};
use tokio::sync::{OnceCell, Semaphore};
use crate::utils::NetSemaphore;

mod directories;

static KATABASIS_APPLICATION: OnceCell<Arc<KatabasisApp>> = OnceCell::const_new();

/// The Katabasis Mod Manager Application State Singleton
pub struct KatabasisApp {
    pub directories: Directories,
    pub db_pool: sqlx::SqlitePool,
    pub http_client: reqwest::Client,
    pub net_semaphore: NetSemaphore,
}

impl KatabasisApp {
    /// Fetches or lazily initialises the application state asynchronously.
    pub async fn get() -> error::KatabasisResult<Arc<Self>> {
        Ok(Arc::clone(KATABASIS_APPLICATION.get_or_try_init(Self::initialise_app).await?))
    }

    /// Initialises the application state, should include all required setup tasks.
    #[tracing::instrument]
    async fn initialise_app() -> error::KatabasisResult<Arc<Self>> {
        let directories = match Directories::init(None).await {
            Ok(directories) => directories,
            Err(error) => {
                error!("Failed to initialise the directories service:\n{:#?}", error);
                return Err(error)
            },
        };

        let db_pool = match storage::connect_database(
            directories.working_dir.as_path()).await {
            Ok(db_pool) => db_pool,
            Err(error) => {
                error!("Failed to initialise the database service:\n{:#?}", error);
                return Err(error)
            },
        };

        let http_client = match build_reqwest_client() {
            Ok(http_client) => http_client,
            Err(error) => {
                error!("Failed to initialise the http client:\n{:#?}", error);
                return Err(error)
            },
        };

        let net_semaphore = NetSemaphore(Semaphore::new(10));

        info!("Successfully initialised the KatabasisApp");

        Ok(Arc::new(Self {
            directories,
            db_pool,
            http_client,
            net_semaphore,
        }))
    }
}

fn build_reqwest_client() -> error::KatabasisResult<reqwest::Client> {
    let mut headers = reqwest::header::HeaderMap::new();

    let header = reqwest::header::HeaderValue::from_str(
        format!("katabasis {}", env!("CARGO_PKG_VERSION")).as_str()
    ).map_err(|error| {
        error::KatabasisErrorKind::HttpGeneralError(
            format!("Failed to initialise application headers: {:#?}", error)
        )
    })?;

    headers.insert(reqwest::header::USER_AGENT, header);

    Ok(
        reqwest::Client::builder()
            .tcp_keepalive(Some(std::time::Duration::from_secs(10)))
            .default_headers(headers)
            .build()
            .map_err(|error| {
                error::KatabasisErrorKind::HttpGeneralError(
                    format!("Failed to initialise application http client: {:#?}", error)
                )
            })?
    )
}
