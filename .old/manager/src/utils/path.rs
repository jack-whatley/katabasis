use std::path::PathBuf;

pub const APP_GUID: &str = "dev.jackwhatley.katabasis";

pub fn default_app_data_dir() -> PathBuf {
    let mut path = dirs_next::data_dir()
        .unwrap_or_else(|| panic!("Unable to find computer data directory"));
    
    path.push(APP_GUID);
    
    path
}
