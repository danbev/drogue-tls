use crate::TlsError;
use heapless::{ArrayLength, Vec};

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    InsufficientBytes,
    InsufficientSpace,
    InvalidData,
}

pub struct ParseBuffer<'b> {
    pos: usize,
    buffer: &'b [u8],
}

impl<'b> Into<ParseBuffer<'b>> for &'b [u8] {
    fn into(self) -> ParseBuffer<'b> {
        ParseBuffer::new(self)
    }
}

impl<'b, N: ArrayLength<u8>> Into<Result<Vec<u8, N>, ()>> for ParseBuffer<'b> {
    fn into(self) -> Result<Vec<u8, N>, ()> {
        Vec::from_slice(&self.buffer[self.pos..])
    }
}

impl<'b> ParseBuffer<'b> {
    pub fn new(buffer: &'b [u8]) -> Self {
        Self { pos: 0, buffer }
    }

    pub fn is_empty(&self) -> bool {
        self.pos == self.buffer.len()
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.pos
    }

    pub fn as_slice(&self) -> &'b [u8] {
        &self.buffer[..]
    }

    pub fn slice(&mut self, len: usize) -> Result<ParseBuffer<'b>, ParseError> {
        if self.pos + len <= self.buffer.len() {
            let slice = ParseBuffer::new(&self.buffer[self.pos..self.pos + len]);
            self.pos += len;
            Ok(slice)
        } else {
            Err(ParseError::InsufficientBytes)
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, ParseError> {
        if self.pos + 1 <= self.buffer.len() {
            let value = self.buffer[self.pos];
            self.pos += 1;
            Ok(value)
        } else {
            Err(ParseError::InsufficientBytes)
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, ParseError> {
        //info!("pos={} len={}", self.pos, self.buffer.len());
        if self.pos + 2 <= self.buffer.len() {
            let value = u16::from_be_bytes([self.buffer[self.pos], self.buffer[self.pos + 1]]);
            self.pos += 2;
            Ok(value)
        } else {
            Err(ParseError::InsufficientBytes)
        }
    }

    pub fn read_u24(&mut self) -> Result<u32, ParseError> {
        if self.pos + 3 <= self.buffer.len() {
            let value = u32::from_be_bytes([
                0,
                self.buffer[self.pos],
                self.buffer[self.pos + 1],
                self.buffer[self.pos + 2],
            ]);
            self.pos += 3;
            Ok(value)
        } else {
            Err(ParseError::InsufficientBytes)
        }
    }

    pub fn read_u32(&mut self) -> Result<u32, ParseError> {
        if self.pos + 4 <= self.buffer.len() {
            let value = u32::from_be_bytes([
                self.buffer[self.pos],
                self.buffer[self.pos + 1],
                self.buffer[self.pos + 2],
                self.buffer[self.pos + 3],
            ]);
            self.pos += 4;
            Ok(value)
        } else {
            Err(ParseError::InsufficientBytes)
        }
    }

    pub fn fill(&mut self, dest: &mut [u8]) -> Result<(), ParseError> {
        if self.pos + dest.len() <= self.buffer.len() {
            dest.copy_from_slice(&self.buffer[self.pos..self.pos + dest.len()]);
            self.pos += dest.len();
            // info!("Copied {} bytes", dest.len());
            Ok(())
        } else {
            Err(ParseError::InsufficientBytes)
        }
    }

    pub fn copy<N: ArrayLength<u8>>(
        &mut self,
        dest: &mut Vec<u8, N>,
        num_bytes: usize,
    ) -> Result<(), ParseError> {
        if (dest.capacity() - dest.len()) < num_bytes {
            Err(ParseError::InsufficientSpace)
        } else if self.pos + num_bytes <= self.buffer.len() {
            dest.extend_from_slice(&self.buffer[self.pos..self.pos + num_bytes])
                .map_err(|_| ParseError::InsufficientSpace)?;
            self.pos += num_bytes;
            Ok(())
        } else {
            Err(ParseError::InsufficientBytes)
        }
    }
}

impl From<ParseError> for TlsError {
    fn from(e: ParseError) -> Self {
        TlsError::ParseError(e)
    }
}
