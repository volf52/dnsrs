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

    #[error("Invalid Jmp Idx `{0}`")]
    InvalidJmpIdx(u16),

    #[error("Unterminated Label Sequence")]
    UnterminatedLabelSequence,

    #[error("Not enough bytes left in buffer for next label part. Current idx {0}")]
    NotEnoughtBytesForLabelPart(usize),
}
