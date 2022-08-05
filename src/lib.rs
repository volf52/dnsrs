pub mod dns;
pub mod utils;

pub type Result<T> = std::result::Result<T, utils::errors::DNSError>;
