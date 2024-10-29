use std::fmt::{Display, Formatter};
use std::io::Error;

#[derive(Debug)]
pub enum ServerError {
    IoError(Error),
    ConnectionClosed,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::IoError(err) => write!(f, "IO error {}", err),
            ServerError::ConnectionClosed => write!(f, "Connection closed"),
        }
    }
}

// convert std error into our IoError
impl From<Error> for ServerError {
    fn from(err: Error) -> Self {
        ServerError::IoError(err)
    }
}

impl std::error::Error for ServerError {}

pub type Result<T> = std::result::Result<T, ServerError>;
