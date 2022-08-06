use crate::DNSError;

const A_VAL: u16 = 1u16;
const CNAME_VAL: u16 = 5u16;
const AAAA_VAL: u16 = 28u16;

#[derive(Debug)]
pub enum RecordType {
    A,
    CNAME,
    AAAA,
}

impl TryFrom<u16> for RecordType {
    type Error = DNSError;

    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        match value {
            A_VAL => Ok(Self::A),
            CNAME_VAL => Ok(Self::CNAME),
            AAAA_VAL => Ok(Self::AAAA),
            _ => Err(DNSError::UnknownRecordType(value)),
        }
    }
}

impl From<&RecordType> for u16 {
    fn from(r: &RecordType) -> Self {
        match r {
            RecordType::A => A_VAL,
            RecordType::CNAME => CNAME_VAL,
            RecordType::AAAA => AAAA_VAL,
        }
    }
}

impl From<RecordType> for u16 {
    fn from(r: RecordType) -> Self {
        (&r).into()
    }
}

impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: u16 = self.into();

        write!(f, "RecordType({:?} = {})", self, val)
    }
}
