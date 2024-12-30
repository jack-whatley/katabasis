use std::path::PathBuf;
use crate::collection::Collection;
use crate::storage::KbApp;

/// Public collections API for CRUD operations on the app collections.

pub mod create;

/// Fetches all [`Collection`]'s currently stored by the katabasis backend.
pub async fn get_all(limit: Option<u32>) -> crate::Result<Vec<Collection>> {
    let state = KbApp::get().await?;

    Collection::get_all(limit, &state.db_pool).await
}

/// Removes a [`Collection`].
pub async fn remove(id: String) -> crate::Result<()> {
    let state = KbApp::get().await?;
    let collection = Collection::get(&id, &state.db_pool).await?;

    Ok(collection.unwrap().remove(&state.db_pool).await?)
}

pub async fn get_full_path(id: &str) -> crate::Result<PathBuf> {
    let state = KbApp::get().await?;
    let collection_dir = state.directories.collection(&id);

    let complete_path = dunce::canonicalize(collection_dir).map_err(|e| {
        crate::Error::FileSystemError(
            format!("Failed to canonicalize collection dir: {:?}", e)
        )
    })?;

    Ok(complete_path)
}
