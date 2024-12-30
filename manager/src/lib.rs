use crate::error::Error;

mod error;

mod storage;

mod utils;
pub use setup::games::*;

mod collection;

// Expose the public api
mod public;
mod setup;

pub use public::*;

pub type Result<T> = std::result::Result<T, Error>;
