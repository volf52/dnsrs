#![allow(dead_code)]
use crate::{DNSError, Result};

use super::{lbl_sequence::LabelSequence, DNSBuffer, RecordType};

#[derive(Debug)]
pub struct DNSAnswer {
    lbl: LabelSequence,
    rtype: RecordType,
    ttl: u32,
    len: u16,
    rdata: String,
}

impl DNSAnswer {
    fn parse_ipv4(buff: &mut DNSBuffer) -> Result<String> {
        let mut ip_parts = vec!["".to_string(); 4];

        for p in ip_parts.iter_mut().take(4) {
            *p = buff.read_u8().map(|val| val.to_string())?;
        }

        Ok(ip_parts.join("."))
    }

    fn parse_cname(buff: &mut DNSBuffer) -> Result<String> {
        LabelSequence::try_from(buff).map(|lbl| lbl.domain())
    }

    fn parse_ipv6(buff: &mut DNSBuffer) -> Result<String> {
        let mut ip_parts = Vec::<String>::with_capacity(4);

        for _ in 0..4 {
            ip_parts.push(buff.read_u8().map(|val| val.to_string())?);
        }

        Ok(ip_parts.join("."))
    }
}

impl TryFrom<&mut DNSBuffer> for DNSAnswer {
    type Error = DNSError;

    fn try_from(buff: &mut DNSBuffer) -> std::result::Result<Self, Self::Error> {
        let lbl = LabelSequence::try_from(&mut *buff)?;
        let rtype_val = buff.read_u16()?;
        let rtype = RecordType::try_from(rtype_val)?;
        buff.read_u16()?; // class
        let ttl = buff.read_u32()?;
        let len = buff.read_u16()?;

        buff.verify_has_remaining(len as usize)?;

        let rdata = match rtype {
            RecordType::A => Self::parse_ipv4(buff),
            RecordType::CNAME => Self::parse_cname(buff),
            RecordType::AAAA => Self::parse_ipv6(buff),
        }?;

        Ok(Self {
            lbl,
            rtype,
            ttl,
            len,
            rdata,
        })
    }
}

impl std::fmt::Display for DNSAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\tIN\t{}\n\t\t\tRDATA={}\tTTL={}",
            self.lbl, self.rtype, self.rdata, self.ttl,
        )
    }
}
