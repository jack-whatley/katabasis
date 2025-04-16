use std::path::{Path, PathBuf};
use bytes::Bytes;
use futures::StreamExt;
use log::error;
use tokio::task::JoinError;
use zip::result::ZipError;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum FsError {
    #[error("Path Canonicalize Error: {0}\n{1}")]
    PathCanonicalizationError(String, String),

    #[error("Folder Creation Error: {0}")]
    FolderCreationError(String),

    #[error("Folder Removal Error: {0}")]
    FolderRemovalError(String),

    #[error("File Creation Error: {0}")]
    FileCreationError(String),

    #[error("File System Error: {0}")]
    FileSystemError(#[from] std::io::Error),

    #[error("Join Error: {0}")]
    JoinError(#[from] JoinError),

    #[error("Zip Error: {0}")]
    ZipError(#[from] ZipError),
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

pub async fn remove_dir_all(path: impl AsRef<Path>) -> Result<(), FsError> {
    tokio::fs::remove_dir_all(path.as_ref()).await.map_err(
        |err| {
            error!("Failed to delete directory '{}':\n{:#?}", path.as_ref().display(), err);

            FsError::FolderRemovalError(
                format!("Failed to delete directory '{}': {:#?}", path.as_ref().display(), err)
            )
        }
    )
}

pub async fn write_stream_to_file(
    path: impl AsRef<Path>,
    bytes: &mut (impl futures::Stream<Item=Result<Bytes, reqwest::Error>> + Unpin),
) -> Result<(), FsError> {
    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path.as_ref())
        .await?;

    while let Some(chunk) = bytes.next().await {
        match chunk {
            Ok(x) => {
                let _ = tokio::io::copy(&mut x.as_ref(), &mut file).await?;
            },
            Err(e) => {
                error!("Encountered error in downloaded bytes stream:\n{:#?}", e);
                return Err(
                    FsError::FileCreationError(
                        format!("Encountered error in downloaded bytes stream:\n{:#?}", e)
                    )
                )
            }
        }
    }

    Ok(())
}

pub async fn unzip_file_to_dir(
    file_path: impl Into<PathBuf> + Send,
    target_dir: impl Into<PathBuf> + Send,
) -> Result<(), FsError> {
    let path_clone = file_path
        .into()
        .clone();

    let target_clone = target_dir
        .into()
        .clone();

    tokio::task::spawn_blocking(move || -> Result<(), FsError> {
        let file = std::fs::File::open(&path_clone)?;
        let mut zip = zip::ZipArchive::new(file)?;

        zip.extract(&target_clone)?;

        Ok(())
    }).await??;

    Ok(())
}

/// Iterates through the collection directory and returns all
/// fetched collection ID's.
pub async fn iterate_collections_dir(path: impl Into<PathBuf>) -> Result<Vec<String>, FsError> {
    let collections_dir = path.into();
    let mut all_folders = tokio::fs::read_dir(collections_dir).await?;

    let mut found_ids: Vec<String> = vec![];

    while let Some(entry) = all_folders.next_entry().await? {
        if let Ok(id) = Uuid::parse_str(entry.file_name().to_str().unwrap()) {
            found_ids.push(format!("{}", id.hyphenated()));
        }
    }

    Ok(found_ids)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use uuid::Uuid;
    use super::iterate_collections_dir;

    async fn initialise_test_dir() -> PathBuf {
        let test_dir = std::env::temp_dir()
            .join("dev.jackwhatley.katabasis.tests")
            .join("core_fs_tests");

        tokio::fs::create_dir_all(&test_dir).await.unwrap();

        test_dir
    }

    async fn cleanup_test_dir(test_dir: &Path) {
        tokio::fs::remove_dir_all(test_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_iterate_collections_dir() {
        let test_dir = initialise_test_dir().await;
        let mut ids: Vec<String> = vec![];

        println!("TestDir: {:#?}", &test_dir);

        for _ in 0..5 {
            let uuid = Uuid::new_v4();
            ids.push(format!("{}", uuid.hyphenated()));
        }

        for id in &ids {
            tokio::fs::create_dir_all(test_dir.join(id)).await.unwrap();
        }

        let mut file_ids = iterate_collections_dir(&test_dir).await.unwrap();

        ids.sort();
        file_ids.sort();

        cleanup_test_dir(&test_dir).await;

        assert_eq!(ids, file_ids);
    }
}
