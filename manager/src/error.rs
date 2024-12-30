use thiserror::Error;
use crate::utils::download::HttpRequestError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("File System Error: {0}")]
    FileSystemError(String),

    #[error("Tokio File System Error: {0}")]
    TokioFileSystemError(#[from] tokio::io::Error),

    #[error("SQLite Error: {0}")]
    SQLiteError(#[from] sqlx::Error),

    #[error("SQLite Migration Error: {0}")]
    SQLiteMigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("Game String Parse Error: {0}")]
    GameStringParseError(String),

    #[error("Http Request Error: {0}")]
    HttpRequestError(#[from] HttpRequestError),

    #[error("Parse Error: {0}")]
    ParseError(String),

    #[error("Steam Locate Error: {0}")]
    SteamLocateError(#[from] steamlocate::Error),

    #[error("Zip Opening Error: {0}")]
    ZipOpeningError(#[from] zip::result::ZipError),
}

// impl<E: Into<Error>> From<E> for Error {
//     fn from(e: E) -> Self {
//         Into::<Error>::into(e)
//     }
// }
