use crate::utils::fs::FsError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use sqlx::migrate::MigrateError;
use tracing_error::InstrumentError;

#[derive(thiserror::Error, Debug)]
pub enum KatabasisErrorKind {
    #[error("File System Error: {0}")]
    FSError(String),

    #[error("File System Error: {0}")]
    KatabasisFSError(#[from] FsError),

    #[error("SQLite Error: {0}")]
    SQLiteError(#[from] sqlx::Error),

    #[error("DB Migration Error: {0}")]
    DBMigrationError(#[from] MigrateError),

    #[error("HTTP General Error: {0}")]
    HttpGeneralError(String),
    
    #[error("Managed Property Error: {0}")]
    InvalidManagedProperty(String),
}

#[derive(Debug)]
pub struct KatabasisError {
    pub raw: Arc<KatabasisErrorKind>,
    pub source: tracing_error::TracedError<Arc<KatabasisErrorKind>>,
}

impl Display for KatabasisError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl Error for KatabasisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.source()
    }
}

impl<E: Into<KatabasisErrorKind>> From<E> for KatabasisError {
    fn from(error_source: E) -> Self {
        let error = Into::<KatabasisErrorKind>::into(error_source);
        let boxed_err = Arc::new(error);

        Self {
            raw: boxed_err.clone(),
            source: boxed_err.in_current_span(),
        }
    }
}

impl KatabasisError {
    pub fn to_error(self) -> KatabasisError {
        self.into()
    }
}

pub type KatabasisResult<T> = Result<T, KatabasisError>;
