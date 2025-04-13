use crate::error::KedgeError;

/// Initial size of the memory (in pages of 64KiB)
const PAGE_SIZE: usize = 65536;
const DEFAULT_PAGES: usize = 1;

/// A simple linear memory model (like WASM's)
pub struct LinearMemory {
    data: Vec<u8>,
}

impl LinearMemory {
    /// Create a new memory with the default size (64KiB)
    pub fn new() -> Self {
        Self {
            data: vec![0u8; PAGE_SIZE * DEFAULT_PAGES],
        }
    }

    /// Read a slice of bytes from memory
    pub fn read(&self, offset: usize, length: usize) -> Result<&[u8], KedgeError> {
        if offset + length > self.data.len() {
            return Err(KedgeError::MemoryViolation);
        }
        Ok(&self.data[offset..offset + length])
    }

    /// Read a 32-bit little-endian integer from memory
    pub fn read_i32(&self, offset: usize) -> Result<i32, KedgeError> {
        let bytes = self.read(offset, 4)?;
        Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
    }

    /// Write a slice of bytes into memory
    pub fn write(&mut self, offset: usize, src: &[u8]) -> Result<(), KedgeError> {
        if offset + src.len() > self.data.len() {
            return Err(KedgeError::MemoryViolation);
        }
        self.data[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }

    /// Write a 32-bit integer into memory (little-endian)
    pub fn write_i32(&mut self, offset: usize, value: i32) -> Result<(), KedgeError> {
        self.write(offset, &value.to_le_bytes())
    }

    /// Return the current memory size (in bytes)
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Grow memory by additional pages
    pub fn grow(&mut self, pages: usize) {
        self.data.resize(self.data.len() + (pages * PAGE_SIZE), 0);
    }
}
