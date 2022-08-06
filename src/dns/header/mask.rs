#[derive(Debug)]
pub(super) enum HeaderMask {
    QR,
    AA,
    TC,
    RD,
    RA,
    RCode,
}

impl From<&HeaderMask> for u8 {
    fn from(mask: &HeaderMask) -> Self {
        match mask {
            HeaderMask::QR | HeaderMask::RA => 0x80,
            HeaderMask::AA => 0x04,
            HeaderMask::TC => 0x02,
            HeaderMask::RD => 0x01,
            HeaderMask::RCode => 0x0F,
        }
    }
}

impl HeaderMask {
    pub(super) fn val(&self) -> u8 {
        self.into()
    }

    pub(super) fn is_set(&self, container: u8) -> bool {
        let val: u8 = self.into();

        (val & container) != 0
    }
}

impl std::fmt::Display for HeaderMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: u8 = self.into();

        write!(f, "HeaderMask({})", val)
    }
}
