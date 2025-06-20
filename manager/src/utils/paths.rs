use std::path::PathBuf;
use crate::collection::Collection;
use crate::targets::ModLoaderKind;

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

/// Returns a version of the input [`Collection`] name with any invalid
/// file system characters removed.
pub fn sanitise_name(name: &str) -> String {
    name.replace(
        ['/', '\\', '?', '*', ':', '\'', '\"', '|', '<', '>', '!'],
        "_",
    )
}

/// Returns the path to a specific collection.
pub fn collection_dir(id: &str) -> PathBuf {
    default_app_dir()
        .join("collections")
        .join(sanitise_name(id))
}

pub fn plugin_dir(collection: &Collection) -> PathBuf {
    match collection.game.mod_loader.kind {
        ModLoaderKind::BepInEx => {
            collection_dir(&collection.name)
                .join("BepInEx")
                .join("plugins")
        }
    }
}
