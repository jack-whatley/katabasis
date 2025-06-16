use std::path::PathBuf;

const APP_GUID: &str = "dev.jackwhatley.katabasis";

#[cfg(debug_assertions)]
const DB_FILE_NAME: &str = "katabasis_dev.db";

#[cfg(not(debug_assertions))]
const DB_FILE_NAME: &str = "katabasis.db";

/// Returns the default application folder. Is not guaranteed
/// to exist.
pub fn default_app_dir() -> PathBuf {
    dirs_next::config_dir().unwrap().join(APP_GUID)
}

/// Returns the path to the database file.
pub fn db_path() -> PathBuf {
    default_app_dir().join(DB_FILE_NAME)
}

/// Returns the path to a specific collection.
pub fn collection_dir(id: &str) -> PathBuf {
    default_app_dir().join("collections").join(id)
}
