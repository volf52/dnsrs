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
    pub(super) const fn value(&self) -> u8 {
        match self {
            Self::QR | Self::RA => 0x80,
            Self::AA => 0x04,
            Self::TC => 0x02,
            Self::RD => 0x01,
            Self::RCode => 0x0F,
        }
    }

    pub(super) const fn is_set(&self, container: u8) -> bool {
        (self.value() & container) != 0
    }
}

impl std::fmt::Display for HeaderMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeaderMask({})", self.value())
    }
}
