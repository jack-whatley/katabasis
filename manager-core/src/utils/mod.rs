use crate::error;
use crate::state::directories::Directories;
use crate::storage::collection_repository;
use std::collections::HashSet;
use tokio::sync::Semaphore;

pub mod fs;
pub mod net;

/// The network semaphore, for controlling the number of concurrent downloads.
#[derive(Debug)]
pub struct NetSemaphore(pub Semaphore);

/// The IO semaphore, for controlling the number of concurrent file operations.
#[derive(Debug)]
pub struct IOSemaphore(pub Semaphore);

/// Function for deleting any Collection folders that don't have database
/// entries, and database entries that don't have associated folders.
pub async fn cleanup_collections(
    directories: &Directories,
    db_pool: &sqlx::SqlitePool
) -> error::KatabasisResult<()> {
    let fs_collections = fs::iterate_collections_dir(
        directories.collections_dir()
    ).await?;
    let db_collections = collection_repository::get_all_ids(db_pool).await?;

    let hash_fs_collections: HashSet<String> = HashSet::from_iter(fs_collections.into_iter());
    let hash_db_collections: HashSet<String> = HashSet::from_iter(db_collections.into_iter());

    let difference = hash_fs_collections
        .symmetric_difference(&hash_db_collections).collect::<Vec<_>>();

    for id in difference {
        collection_repository::remove_id(id, db_pool).await?;
        let collection_dir = directories.collection_dir(id);

        if collection_dir.exists() {
            fs::remove_dir_all(collection_dir).await?;
        }
    }

    Ok(())
}

/// Function for performing any migration actions that might
/// be required when updating versions.
///
/// Current Migrations:
/// - Cleans up old Katabasis directory in favour of the newer
/// dev.jackwhatley.katabasis working directory.
pub async fn migration_functions() -> error::KatabasisResult<()> {
    let old_working_dir = Directories::default_settings_dir()?
        .join("..")
        .join("Katabasis");

    if old_working_dir.exists() {
        fs::remove_dir_all(&old_working_dir).await?;
    }

    Ok(())
}
