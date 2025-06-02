use std::ffi::OsStr;
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

/// Iterates through a directory, returning the full path to each
/// item in it.
pub async fn iterate_directory(path: impl Into<PathBuf>, recursive: bool) -> Result<Vec<PathBuf>, FsError> {
    let mut paths: Vec<PathBuf> = vec![];
    let mut items = tokio::fs::read_dir(path.into()).await?;

    while let Some(entry) = items.next_entry().await? {
        paths.push(entry.path());

        if recursive && entry.path().is_dir() {
            paths.append(&mut Box::pin(iterate_directory(entry.path().clone(), true)).await?)
        }
    }

    Ok(paths)
}

/// Iterates through the collection directory and returns all
/// fetched collection ID's.
pub async fn iterate_collections_dir(path: impl Into<PathBuf>) -> Result<Vec<String>, FsError> {
    let mut found_ids: Vec<String> = vec![];
    let all_folders = iterate_directory(path.into(), false).await?;

    for path in all_folders {
        if path.is_dir() {
            let name = path
                .file_name()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap_or("");

            if let Ok(id) = Uuid::parse_str(name) {
                found_ids.push(format!("{}", id.hyphenated()));
            }
        }
    }

    Ok(found_ids)
}

/// Copies all the contents of a provided directory into another
/// overwriting any duplicates that appear.
pub async fn copy_contents_to(
    source_dir: impl Into<PathBuf>,
    target_dir: impl Into<PathBuf>,
) -> Result<(), FsError> {
    let source_contents = iterate_directory(source_dir.into(), false).await?;
    let target_dir = target_dir.into();
    let ignore = vec!["_DISABLED"];

    for path in source_contents {
        if let Some(name) = path.file_name() {
            if path.is_dir() {
                copy_contents_recursive(&path, target_dir.join(name), &ignore).await?;
            }
            else {
                tokio::fs::copy(&path, target_dir.join(name)).await?;
            }
        }
    }

    Ok(())
}

async fn copy_contents_recursive(
    source_dir: impl Into<PathBuf>,
    target_dir: impl Into<PathBuf>,
    ignore_targets: &Vec<&str>,
) -> Result<(), FsError> {
    let source_contents = iterate_directory(source_dir.into(), false).await?;
    let target_dir = target_dir.into();

    if !target_dir.exists() {
        tokio::fs::create_dir(target_dir.clone()).await?;
    }

    // MUST USE path.clone() here otherwise a recursion overflow is
    // encountered when compiling (from references).
    for path in source_contents {
        if let Some(name) = path.file_name() {
            if path.is_dir() {
                Box::pin(
                    copy_contents_recursive(path.clone(), target_dir.join(name), ignore_targets)
                ).await?;
            }
            else {
                if ignore_targets.iter().any(|t| path.ends_with(t)) {
                    continue;
                }

                tokio::fs::copy(path.clone(), target_dir.join(name)).await?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use uuid::Uuid;
    use super::{copy_contents_to, iterate_collections_dir, iterate_directory};

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

    #[tokio::test]
    async fn test_copy_contents_to() {
        let source_dir = std::env::temp_dir()
            .join("dev.jackwhatley.katabasis.tests")
            .join("core_fs_tests_src");
        let target_dir = std::env::temp_dir()
            .join("dev.jackwhatley.katabasis.tests")
            .join("core_fs_tests_tgt");

        tokio::fs::create_dir_all(&source_dir).await.unwrap();
        tokio::fs::create_dir_all(&target_dir).await.unwrap();

        tokio::fs::create_dir_all(
            source_dir.join("test.path")
        ).await.unwrap();

        tokio::fs::create_dir_all(
            source_dir.join("test.path2")
        ).await.unwrap();

        tokio::fs::create_dir_all(
            source_dir.join("test.path").join("sub.path")
        ).await.unwrap();

        copy_contents_to(&source_dir, &target_dir).await.unwrap();

        let all_folders = iterate_directory(&target_dir, false).await.unwrap();
        let all_sub_folders = iterate_directory(
            target_dir.join("test.path"),
            false
        ).await.unwrap();

        println!("TestDir: {:#?}", &source_dir);
        println!("TestDir: {:#?}", &target_dir);

        cleanup_test_dir(&source_dir).await;
        cleanup_test_dir(&target_dir).await;

        assert_eq!(all_folders.len(), 2usize);
        assert_eq!(all_sub_folders.len(), 1usize);
    }
}
