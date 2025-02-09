use crate::error::Error;

mod error;

mod storage;
pub use storage::plugin::{SupportedPluginSources, Plugin};

mod utils;
pub use setup::games::*;

mod collection;
pub use collection::Collection;

// Expose the public api
mod public;
mod setup;
mod api;

pub use public::*;

pub type Result<T> = std::result::Result<T, Error>;
