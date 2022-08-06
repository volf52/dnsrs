use crate::DNSError;

use super::{lbl_sequence::LabelSequence, DNSBuffer, RecordType};

#[derive(Debug)]
pub struct DNSQuestion {
    lbl: LabelSequence,
    record_type: RecordType,
}

impl TryFrom<&mut DNSBuffer> for DNSQuestion {
    type Error = DNSError;

    fn try_from(buff: &mut DNSBuffer) -> Result<Self, Self::Error> {
        let lbl = LabelSequence::try_from(&mut *buff)?;

        let rtype_val = { buff.read_u16()? };
        let record_type = RecordType::try_from(rtype_val)?;

        buff.read_u16()?; // Class - Always 1

        Ok(Self { lbl, record_type })
    }
}

impl std::fmt::Display for DNSQuestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\tIN\t{}", self.lbl, self.record_type)
    }
}
