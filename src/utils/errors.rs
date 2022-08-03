use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DNSError {
    #[error("unknown error lol")]
    Unknown,

    #[error("IO Error")]
    IOError(#[from] io::Error),
}
