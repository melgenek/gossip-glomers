use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("Error: '{0}'")]
    // Message(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

pub type Result<A> = std::result::Result<A, Error>;
