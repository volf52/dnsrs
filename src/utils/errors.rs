use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DNSError {
    #[error("unknown error lol")]
    Unknown,

    #[error("IO Error")]
    IOError(#[from] io::Error),

    #[error("Str UTF-8 Error")]
    Utf8Error(#[from] std::str::Utf8Error),

    // DNS Buffer Errors
    #[error("Buffer Full. Reached End")]
    BufferFull,
}
