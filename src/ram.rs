const RAM_SIZE: usize = 1024;

pub enum MemoryAccessSize {
    Byte,
    Word,
    DoubleWord,
}

impl MemoryAccessSize {
    pub fn size(&self) -> u32 {
        match self {
            MemoryAccessSize::Byte => 8,
            MemoryAccessSize::Word => 32,
            MemoryAccessSize::DoubleWord => 64,
        }
    }

    pub fn byte_size(&self) -> u32 {
        match self {
            MemoryAccessSize::Byte => 1,
            MemoryAccessSize::Word => 4,
            MemoryAccessSize::DoubleWord => 8,
        }
    }
}

#[derive(Debug)]
pub struct RAM {
    /// allocated on heap to keep the pointer alive
    pub data: Box<[u8]>,
}

impl Default for RAM {
    fn default() -> Self {
        Self::new()
    }
}

impl RAM {
    pub fn new() -> Self {
        Self {
            data: vec![0u8; RAM_SIZE].into_boxed_slice(),
        }
    }

    /// Write a word (32-bit) into RAM at the given address
    pub fn write_word(&mut self, addr: usize, value: u32) {
        let bytes = value.to_le_bytes();
        self.data[addr..addr + 4].copy_from_slice(&bytes);
    }

    /// Read a word (32-bit) from RAM at the given address
    pub fn read_word(&self, addr: usize) -> u32 {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.data[addr..addr + 4]);
        u32::from_le_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ram_initialization() {
        let ram = RAM::new();
        println!("RAM initialized with size: {}", ram.data.len());
    }
}
