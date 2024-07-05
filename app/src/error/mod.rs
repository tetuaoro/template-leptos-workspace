use std::env::VarError;
use surrealdb::Error as SurrealError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Env(String),
    Database(String),
}

impl From<SurrealError> for Error {
    fn from(e: SurrealError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        match e {
            VarError::NotPresent => Self::Env("Missing environment variables".to_string()),
            VarError::NotUnicode(msg) => Self::Env(
                msg.into_string()
                    .unwrap_or(String::from("Could not parse the environment variables")),
            ),
        }
    }
}
