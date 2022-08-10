use super::buffer::DNSBuffer;
use crate::utils::errors::DNSError;

const NULL_BYTE: u8 = 0x00;
const JMP_BYTE: u8 = 0xC0;
const JMP_MASK: u16 = 0xC000;
const DOT: &str = ".";

#[derive(Debug)]
pub struct LabelSequence(String);

impl TryFrom<&mut DNSBuffer> for LabelSequence {
    type Error = DNSError;

    fn try_from(buff: &mut DNSBuffer) -> std::result::Result<Self, Self::Error> {
        buff.verify_has_remaining(1)?;
        let buff_cap = buff.capacity();

        if buff.peek() == JMP_BYTE {
            let mut jmp_idx = buff.read_u16()?;
            jmp_idx ^= JMP_MASK;

            if (jmp_idx as usize) >= buff_cap {
                return Err(DNSError::InvalidJmpIdx(jmp_idx));
            }

            let copied_slice = buff.deep_slice_from(jmp_idx as usize);
            let mut inner_buff = DNSBuffer::from(copied_slice);

            Self::try_from(&mut inner_buff)
        } else {
            let mut parts = Vec::<String>::new();

            while buff.offset() < buff_cap && buff.peek() != NULL_BYTE {
                let part_length = buff.read_u8()? as usize;
                let curr_offset = buff.offset();

                let part = buff
                    .read_string(part_length)
                    .map_err(|_| DNSError::NotEnoughtBytesForLabelPart(curr_offset))?;

                parts.push(part.to_string());
            }
            buff.read_u8()
                .map_err(|_| DNSError::UnterminatedLabelSequence)?;

            let domain = parts.join(DOT);

            Ok(Self(domain))
        }
    }
}

impl LabelSequence {
    #[must_use]
    pub fn domain(&self) -> String {
        self.0.clone()
    }
}

impl std::fmt::Display for LabelSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ";; {} ;;", self.domain())
    }
}
