use std::path::PathBuf;

const APP_GUID: &str = "dev.jackwhatley.katabasis";

/// Returns the default application folder. Is not guaranteed
/// to exist.
pub fn application_dir() -> PathBuf {
    dirs_next::config_dir().unwrap().join(APP_GUID)
}
