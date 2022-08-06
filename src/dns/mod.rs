pub mod buffer;
pub mod header;
pub mod lbl_sequence;
pub mod question;
pub mod record_type;

pub use buffer::DNSBuffer;
pub use header::DNSHeader;
pub use question::DNSQuestion;
pub use record_type::RecordType;
