use std::path::PathBuf;
use crate::error;
use tokio::fs;

pub(crate) const APP_DIR: &'static str = "Katabasis";

pub(crate) const COLLECTIONS_DIR: &'static str = "collections";

pub(crate) const LOADERS_DIR: &'static str = "loaders";

pub(crate) const PLUGIN_DIR: &'static str = "plugins";

pub struct Directories {
    pub app_dir: PathBuf
}

impl Directories {
    pub fn get_default_dir() -> Option<PathBuf> {
        Some(dirs::data_dir()?.join(APP_DIR))
    }

    pub async fn init() -> crate::Result<Self> {
        let default_dir = Self::get_default_dir().ok_or(
            error::Error::FileSystemError(
                "Failed to find the default application settings directory".to_string()
            )
        )?;

        fs::create_dir_all(&default_dir).await.map_err(|err| {
            error::Error::FileSystemError(
                format!("Failed to create the default application directory: {}", err)
            )
        })?;

        let directories = Self { app_dir: default_dir };

        fs::create_dir_all(directories.collections_dir()).await.map_err(|err| {
            error::Error::FileSystemError(
                format!("Failed to create the collections directory: {}", err)
            )
        })?;

        fs::create_dir_all(directories.loaders_dir()).await.map_err(|err| {
            error::Error::FileSystemError(
                format!("Failed to create the collections directory: {}", err)
            )
        })?;

        Ok(directories)
    }

    #[inline]
    pub fn collections_dir(&self) -> PathBuf {
        self.app_dir.join(COLLECTIONS_DIR)
    }

    #[inline]
    pub fn loaders_dir(&self) -> PathBuf {
        self.app_dir.join(LOADERS_DIR)
    }

    #[inline]
    pub fn collection(&self, collection_id: &str) -> PathBuf {
        self.collections_dir().join(collection_id)
    }

    #[inline]
    pub fn collection_plugin_dir(&self, collection_id: &str) -> PathBuf {
        self.collection(collection_id).join(PLUGIN_DIR)
    }
}
