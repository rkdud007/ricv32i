// 64k Memory
pub const RAM_SIZE: usize = 1024 * 64;

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
    pub data: [u8; RAM_SIZE],
}

impl Default for RAM {
    fn default() -> Self {
        Self::new()
    }
}

impl RAM {
    pub fn new() -> Self {
        Self {
            data: [0; RAM_SIZE],
        }
    }

    pub fn write_bytes(&mut self, addr: usize, data: &[u8]) {
        // memory = memory[:addr] + data + memory[addr+len(dat):]
        self.data[addr..addr + data.len()].copy_from_slice(data);
        let mut current_addr = addr;
        // Print data in 32-byte chunks
        for chunk in data.chunks(32) {
            // print!("0x{:08X}: ", current_addr);

            // Print the chunk as 4-byte words
            for word in chunk.chunks(4) {
                let mut buf = [0u8; 4];
                for (i, &b) in word.iter().enumerate() {
                    buf[i] = b;
                }

                let val = u32::from_le_bytes(buf);
                // print!("{:08X} ", val);
            }

            // println!();
            current_addr += 32;
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
