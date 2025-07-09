use std::fmt::Display;
use serde::{Serialize, Serializer};

pub type Result<T> = std::result::Result<T, CommandError>;

#[derive(Debug)]
pub struct CommandError(eyre::Error);

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self.0)
    }
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<T> From<T> for CommandError
where
    T: Into<eyre::Report>,
{
    fn from(t: T) -> Self {
        let report = t.into();

        tracing::error!("Command Returning Error:\n{:#?}", report);

        Self(report)
    }
}
