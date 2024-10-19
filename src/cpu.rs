use crate::{instruction::RV5Instruction, ram::RAM};

const REGISTER_COUNT: usize = 32;

#[derive(Debug)]
pub struct Register {
    pub data: [u32; REGISTER_COUNT],
}

/// 32-bit RISC-V
#[derive(Debug)]
pub struct CPU {
    /// 32 registers of 32-bit
    pub reg: Register,
    /// clock cycle
    pub clk: u32,
    /// program counter of instruction
    pub pc: *const u8,
    /// random access memory
    pub ram: RAM,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        let ram = RAM::new();
        let ptr = ram.data.as_ptr();
        CPU {
            reg: Register {
                data: [0u32; REGISTER_COUNT],
            },
            clk: 0,
            pc: ptr,
            ram,
        }
    }

    /// Note that instruction is a 32-bit value
    pub fn fetch_ins(&mut self) -> u32 {
        unsafe {
            let mut instruction_bytes = [0u8; 4];
            std::ptr::copy_nonoverlapping(self.pc, instruction_bytes.as_mut_ptr(), 4);
            self.pc = self.pc.add(4);
            u32::from_le_bytes(instruction_bytes)
        }
    }

    /// Decode the instruction
    pub fn decode_ins(&self, instruction: u32) -> RV5Instruction {
        RV5Instruction::new(instruction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cpu_fetch_instruction() {
        let mut cpu = CPU::new();
        println!("cpu.pc = {:?}", cpu.pc);
        println!("cpu.clk = {:?}", cpu.clk);
        assert_eq!(cpu.fetch_ins(), 0);
        println!("cpu.pc = {:?}", cpu.pc);
        println!("cpu.clk = {:?}", cpu.clk);

        cpu.decode_ins(0x007302b3);
    }
}
