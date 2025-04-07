use std::path::PathBuf;
use log::error;
use crate::error;
use crate::utils::fs;

const APPLICATION_DIRECTORIES: &[&str] = &["collections", "downloads", "loaders"];

pub struct Directories {
    pub working_dir: PathBuf,
}

// TODO: ADD UNIT TESTS, USING TEMPORARY FILES
impl Directories {
    pub fn default_settings_dir() -> error::KatabasisResult<PathBuf> {
        Ok(dirs::data_dir().ok_or(
            error::KatabasisErrorKind::FSError(
                "Failed to get the path to the %AppData% directory.".to_owned()
            )
        )?.join("dev.jackwhatley.katabasis"))
    }

    #[tracing::instrument]
    pub async fn init(working_dir: Option<PathBuf>) -> error::KatabasisResult<Self> {
        let working_dir = working_dir.map_or(
            Self::default_settings_dir(), |x| Ok(x.into()));

        if working_dir.is_err() {
            error!("Failed to find the working directory");

            return Err(
                error::KatabasisErrorKind::FSError(
                    format!("Failed to get the working directory: {:?}", working_dir.err())
                ).into()
            )
        }

        let working_dir = working_dir?;

        if !working_dir.exists() {
            fs::create_dir(&working_dir, true).await?;
        }

        for path in APPLICATION_DIRECTORIES {
            let full_path = working_dir.join(path);

            fs::create_dir(full_path, false).await?;
        }

        Ok(Self { working_dir })
    }

    #[inline]
    pub fn collections_dir(&self) -> PathBuf {
        self.working_dir.join("collections")
    }

    #[inline]
    pub fn collection_dir(&self, id: &str) -> PathBuf {
        self.collections_dir().join(id)
    }

    #[inline]
    pub fn download_dir(&self) -> PathBuf {
        self.working_dir.join("downloads")
    }

    #[inline]
    pub fn loaders_dir(&self) -> PathBuf {
        self.working_dir.join("loaders")
    }
}
