use std::path::PathBuf;
use crate::targets::Target;
use crate::{targets, utils};

/// Returns the [`PathBuf`] to the applications default directory.
pub fn app_dir() -> PathBuf {
    utils::paths::default_app_dir()
}

/// Returns an iterator of all currently supported application
/// targets.
pub fn all_targets() -> impl Iterator<Item = Target> {
    targets::all()
}

/// Returns a specific target based on the provided slug.
pub fn specific_target(slug: &str) -> Option<Target> {
    targets::from_slug(slug)
}
