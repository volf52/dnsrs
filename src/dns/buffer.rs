use std::str;

use crate::{utils::errors::DNSError, Result};

pub const DEFAULT_BUFFER_SIZE: usize = 1024; // 1kB

#[derive(Debug)]
pub struct DNSBuffer {
    data: Vec<u8>,
    offset: usize,
    capacity: usize,
}

impl Default for DNSBuffer {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_BUFFER_SIZE)
    }
}

impl From<Vec<u8>> for DNSBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let cap = vec.len();

        let mut buff = Self::with_capacity(cap);
        buff.data.copy_from_slice(&vec[..cap]);

        buff
    }
}

impl From<&[u8]> for DNSBuffer {
    fn from(b: &[u8]) -> Self {
        let cap = b.len();

        let mut buff = Self::with_capacity(cap);
        buff.data.copy_from_slice(b);

        buff
    }
}

impl DNSBuffer {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity],
            offset: 0,
            capacity,
        }
    }

    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    #[must_use]
    pub fn slice_from(&self, start_idx: usize) -> &[u8] {
        &self.data[start_idx..]
    }

    #[must_use]
    pub fn deep_slice_from(&self, start: usize) -> Vec<u8> {
        let mut v = vec![0; self.capacity - start];

        v.copy_from_slice(&self.data[start..]);

        v
    }

    pub(crate) const fn verify_has_remaining(&self, n_bytes: usize) -> Result<()> {
        if self.offset + n_bytes > self.capacity {
            Err(DNSError::BufferFull)
        } else {
            Ok(())
        }
    }

    // Read Methods
    #[must_use]
    pub fn peek(&self) -> u8 {
        self.data[self.offset]
    }

    /// Reads a u8 from the buffer
    pub fn read_u8(&mut self) -> Result<u8> {
        self.verify_has_remaining(1)?;

        let byte = self.data[self.offset];
        self.offset += 1;

        Ok(byte)
    }

    /// Reads a u16 from the buffer
    pub fn read_u16(&mut self) -> Result<u16> {
        self.verify_has_remaining(2)?;

        let d1 = self.data[self.offset];
        let d2 = self.data[self.offset + 1];
        self.offset += 2;

        Ok(u16::from_be_bytes([d1, d2]))
    }

    /// Reads a u32 from the buffer
    pub fn read_u32(&mut self) -> Result<u32> {
        self.verify_has_remaining(4)?;

        let d1 = self.data[self.offset];
        let d2 = self.data[self.offset + 1];
        let d3 = self.data[self.offset + 2];
        let d4 = self.data[self.offset + 3];
        self.offset += 4;

        Ok(u32::from_be_bytes([d1, d2, d3, d4]))
    }

    /// Reads a string of length [`len`] from the buffer
    pub fn read_string(&mut self, len: usize) -> Result<&str> {
        self.verify_has_remaining(len)?;

        let b = &self.data[self.offset..self.offset + len];
        let val = std::str::from_utf8(b)?;
        self.offset += len;

        Ok(val)
    }

    // Write Methods

    /// Writes a u8 to the buffer
    pub fn write_u8(&mut self, val: u8) -> Result<()> {
        self.verify_has_remaining(1)?;

        self.data[self.offset] = val;
        self.offset += 1;

        Ok(())
    }

    /// Writes a u16 to the buffer
    pub fn write_u16(&mut self, val: u16) -> Result<()> {
        self.verify_has_remaining(2)?;

        let d: [u8; 2] = val.to_be_bytes();

        self.data[self.offset] = d[0];
        self.data[self.offset + 1] = d[1];

        self.offset += 2;

        Ok(())
    }

    /// Writes a u32 to the buffer
    pub fn write_u32(&mut self, val: u32) -> Result<()> {
        self.verify_has_remaining(4)?;

        let d: [u8; 4] = val.to_be_bytes();

        self.data[self.offset] = d[0];
        self.data[self.offset + 1] = d[1];
        self.data[self.offset + 2] = d[2];
        self.data[self.offset + 3] = d[3];

        self.offset += 4;

        Ok(())
    }

    /// Writes the string [`s`] to the buffer
    pub fn write_string(&mut self, s: &str) -> Result<()> {
        let len = s.len();
        self.verify_has_remaining(len)?;

        self.data[self.offset..self.offset + len].copy_from_slice(s.as_bytes());
        self.offset += len;

        Ok(())
    }
}
