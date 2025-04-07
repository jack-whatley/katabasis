use std::path::{Path, PathBuf};
use log::error;

#[derive(Debug, thiserror::Error)]
pub enum FsError {
    #[error("Path Canonicalize Error: {0}\n{1}")]
    PathCanonicalizationError(String, String),

    #[error("Folder Creation Error: {0}")]
    FolderCreationError(String),
}

pub fn canonicalize_path(target: impl AsRef<Path>) -> Result<PathBuf, FsError> {
    let path = target.as_ref();

    dunce::canonicalize(path).map_err(|err|
        FsError::PathCanonicalizationError(
            format!("Failed to canonicalize path: {}", err.to_string()),
            path.display().to_string()))
}

pub async fn create_dir(path: impl AsRef<Path>, create_all: bool) -> Result<(), FsError> {
    if create_all {
        tokio::fs::create_dir_all(path.as_ref()).await.map_err(
            |err| {
                error!("Failed to create directory '{}': {}", path.as_ref().display(), err);

                FsError::FolderCreationError(
                    format!("Failed to create all directories recursively: {}", err.to_string())
                )
            }
        )
    }
    else {
        tokio::fs::create_dir(path.as_ref()).await.map_err(
            |err| {
                error!("Failed to create directory '{}': {}", path.as_ref().display(), err);

                FsError::FolderCreationError(
                    format!("Failed to create directory: {}", err.to_string())
                )
            }
        )
    }
}
