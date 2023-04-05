use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    // #[error("Error: '{0}'")]
    // Message(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

pub type CommonResult<A> = Result<A, CommonError>;
