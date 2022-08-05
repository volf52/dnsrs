use super::mask::HeaderMask;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(super) struct HeaderFlags {
    byte1: u8,
    byte2: u8,
}

impl From<u16> for HeaderFlags {
    fn from(val: u16) -> Self {
        let byte1 = (val >> 8) as u8;
        let byte2 = (val & 0x00FF) as u8;

        Self { byte1, byte2 }
    }
}

impl From<HeaderFlags> for u16 {
    fn from(flag: HeaderFlags) -> Self {
        let mut val: u16 = flag.byte1 as u16;
        val <<= 8;
        val += flag.byte2 as u16;

        val
    }
}

impl HeaderFlags {
    fn qr(&self) -> bool {
        HeaderMask::QR.is_set(self.byte1)
    }

    fn aa(&self) -> bool {
        HeaderMask::AA.is_set(self.byte1)
    }

    fn tc(&self) -> bool {
        HeaderMask::TC.is_set(self.byte1)
    }

    fn rd(&self) -> bool {
        HeaderMask::RD.is_set(self.byte1)
    }

    fn ra(&self) -> bool {
        HeaderMask::RA.is_set(self.byte2)
    }

    fn z(&self) -> u8 {
        0x02
    }

    fn rcode(&self) -> u8 {
        HeaderMask::RCode.value() & self.byte2
    }
}

impl std::fmt::Display for HeaderFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let qr = self.qr();
        let aa = self.aa();
        let tc = self.tc();
        let rd = self.rd();
        let ra = self.ra();
        let z = self.z();
        let rcode = self.rcode();

        write!(
            f,
            "Flags: QR={}, AA={}, TC={}, RD={}, RA={}\nZ = {}, Response Code = {}",
            qr, aa, tc, rd, ra, z, rcode
        )
    }
}
