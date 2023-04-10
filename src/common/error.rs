use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Received an unexpected message: '{0}'")]
    UnexpectedMessage(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("Console error: '{0}'")]
    Console(String),
}

pub type Result<A> = std::result::Result<A, Error>;
