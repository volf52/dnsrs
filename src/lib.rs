#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
pub mod dns;
pub mod utils;

pub type Result<T> = std::result::Result<T, utils::errors::DNSError>;
