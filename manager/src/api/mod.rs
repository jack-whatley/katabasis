use std::path::PathBuf;

use crate::utils;

pub fn app_dir() -> PathBuf {
    utils::paths::default_app_dir()
}
