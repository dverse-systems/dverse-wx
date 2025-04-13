use crate::error::KedgeError;

/// WASM binary reader with LEB128 decoding and bounds checking
pub struct Reader<'a> {
    pub bytes: &'a [u8],
    pub position: usize,
}

impl<'a> Reader<'a> {
    /// Create a new binary reader from byte slice
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    /// Read one byte and advance the cursor
    pub fn read_u8(&mut self) -> Result<u8, KedgeError> {
        if self.position >= self.bytes.len() {
            return Err(KedgeError::DecodeError("EOF reached"));
        }
        let byte = self.bytes[self.position];
        self.position += 1;
        Ok(byte)
    }

    /// Read a slice of bytes with the given length
    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], KedgeError> {
        if self.position + len > self.bytes.len() {
            return Err(KedgeError::DecodeError("not enough bytes"));
        }
        let slice = &self.bytes[self.position..self.position + len];
        self.position += len;
        Ok(slice)
    }

    /// Read an unsigned LEB128 (up to 32-bit)
    pub fn read_u32_leb128(&mut self) -> Result<u32, KedgeError> {
        let mut result = 0u32;
        let mut shift = 0;

        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as u32) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift > 35 {
                return Err(KedgeError::DecodeError("LEB128 overflow"));
            }
        }

        Ok(result)
    }

    /// Read a signed LEB128 (up to 32-bit)
    pub fn read_i32_leb128(&mut self) -> Result<i32, KedgeError> {
        let mut result = 0i32;
        let mut shift = 0;
        let mut byte;

        loop {
            byte = self.read_u8()?;
            result |= ((byte & 0x7F) as i32) << shift;
            shift += 7;

            if byte & 0x80 == 0 {
                break;
            }

            if shift > 35 {
                return Err(KedgeError::DecodeError("signed LEB128 overflow"));
            }
        }

        // sign extend
        if shift < 32 && (byte & 0x40) != 0 {
            result |= -1 << shift;
        }

        Ok(result)
    }

    /// Peek the next byte without advancing
    pub fn peek(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    /// Get current cursor position
    pub fn offset(&self) -> usize {
        self.position
    }

    /// Set the position manually (for jumps/backpatching)
    pub fn seek(&mut self, pos: usize) -> Result<(), KedgeError> {
        if pos > self.bytes.len() {
            return Err(KedgeError::DecodeError("seek out of bounds"));
        }
        self.position = pos;
        Ok(())
    }

    /// Whether the reader has consumed all bytes
    pub fn is_done(&self) -> bool {
        self.position >= self.bytes.len()
    }
}
