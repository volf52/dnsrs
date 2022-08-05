#[derive(Debug)]
pub(super) enum HeaderMask {
    QR,
    AA,
    TC,
    RD,
    RA,
    RCode,
}

impl HeaderMask {
    pub(super) fn value(&self) -> u8 {
        match self {
            HeaderMask::QR => 0x80,
            HeaderMask::AA => 0x04,
            HeaderMask::TC => 0x02,
            HeaderMask::RD => 0x01,
            HeaderMask::RA => 0x80,
            HeaderMask::RCode => 0x0F,
        }
    }

    pub(super) fn is_set(&self, container: u8) -> bool {
        (self.value() & container) != 0
    }
}

impl std::fmt::Display for HeaderMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeaderMask({})", self.value())
    }
}
