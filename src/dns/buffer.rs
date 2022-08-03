pub const DEFAULT_BUFFER_SIZE: usize = 1024; // 1kB

#[derive(Debug)]
pub struct Buffer {
    data: Vec<u8>,
    offset: usize,
    // capacity: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_BUFFER_SIZE)
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(vec: Vec<u8>) -> Self {
        let cap = vec.len();

        let mut buff = Self::with_capacity(cap);
        buff.data.copy_from_slice(&vec[..cap]);

        buff
    }
}

impl From<&[u8]> for Buffer {
    fn from(b: &[u8]) -> Self {
        let cap = b.len();

        let mut buff = Self::with_capacity(cap);
        buff.data.copy_from_slice(b);

        buff
    }
}

impl Buffer {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity],
            offset: 0,
            // capacity,
        }
    }

    // Read Methods
    pub fn peek(&self) -> u8 {
        self.data[self.offset]
    }

    pub fn read_u8(&mut self) -> u8 {
        let byte = self.data[self.offset];
        self.offset += 1;

        byte
    }

    pub fn read_u16(&mut self) -> u16 {
        let d1 = self.data[self.offset];
        let d2 = self.data[self.offset + 1];
        self.offset += 2;

        u16::from_be_bytes([d1, d2])
    }

    pub fn read_u32(&mut self) -> u32 {
        let d1 = self.data[self.offset];
        let d2 = self.data[self.offset + 1];
        let d3 = self.data[self.offset + 2];
        let d4 = self.data[self.offset + 3];
        self.offset += 4;

        u32::from_be_bytes([d1, d2, d3, d4])
    }

    // Write Methods
    pub fn write_u8(&mut self, val: u8) {
        self.data[self.offset] = val;
        self.offset += 1;
    }

    pub fn write_u16(&mut self, val: u16) {
        let d: [u8; 2] = val.to_be_bytes();

        self.data[self.offset] = d[0];
        self.data[self.offset + 1] = d[1];

        self.offset += 2;
    }

    pub fn write_u32(&mut self, val: u32) {
        let d: [u8; 4] = val.to_be_bytes();

        self.data[self.offset] = d[0];
        self.data[self.offset + 1] = d[1];
        self.data[self.offset + 2] = d[2];
        self.data[self.offset + 3] = d[3];

        self.offset += 4;
    }
}
