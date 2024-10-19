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
