use crate::{
    instruction::{RV5Instruction, RV5Itype, RV5Rtype, RV5SBtype, RV5Stype},
    ram::RAM,
};

const REGISTER_COUNT: usize = 32;

#[derive(Debug)]
pub struct Register {
    /// general purpose registers
    pub gen_reg: [u32; REGISTER_COUNT],
    /// program counter of instruction
    pub pc: *const u8,
}

/// 32-bit RISC-V
#[derive(Debug)]
pub struct CPU {
    /// register
    pub reg: Register,
    /// clock cycle
    pub clk: u32,
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
                gen_reg: [0u32; REGISTER_COUNT],
                pc: ptr,
            },
            clk: 0,
            ram,
        }
    }

    pub fn load_instructions(&mut self, binary_data: &[u8]) {
        for (i, chunk) in binary_data.chunks(4).enumerate() {
            self.ram.write_word(
                i * 4,
                u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]),
            );
        }
    }

    /// Note that instruction is a 32-bit value
    pub fn fetch_ins(&mut self) -> u32 {
        unsafe {
            let mut instruction_bytes = [0u8; 4];
            std::ptr::copy_nonoverlapping(self.reg.pc, instruction_bytes.as_mut_ptr(), 4);
            self.reg.pc = self.reg.pc.add(4);
            u32::from_le_bytes(instruction_bytes)
        }
    }

    pub fn execute_ins(&mut self) {
        let instruction = self.fetch_ins();
        let decoded_instruction = self.decode_ins(instruction);
        match decoded_instruction {
            RV5Instruction::R(rv5_r_type) => self.execute_rtype(rv5_r_type),
            RV5Instruction::I(rv5_i_type) => self.execute_itype(rv5_i_type),
            RV5Instruction::S(rv5_s_type) => self.execute_stype(rv5_s_type),
            RV5Instruction::SB(rv5_sb_type) => self.execute_sbtype(rv5_sb_type),
        }
        self.clk += 1;
    }

    /// Decode the instruction
    fn decode_ins(&self, instruction: u32) -> RV5Instruction {
        RV5Instruction::new(instruction)
    }

    fn execute_rtype(&mut self, instruction: RV5Rtype) {
        let rs1_val = self.reg.gen_reg[instruction.rs1 as usize];
        let rs2_val = self.reg.gen_reg[instruction.rs2 as usize];

        let result = match instruction.funct3 {
            0b000 => match instruction.funct7 {
                0b0000000 => rs1_val.wrapping_add(rs2_val), // ADD
                0b0100000 => rs1_val.wrapping_sub(rs2_val), // SUB
                _ => panic!("Unknown funct7 for R-type"),
            },
            0b111 => rs1_val & rs2_val, // AND
            0b110 => rs1_val | rs2_val, // OR
            0b100 => rs1_val ^ rs2_val, // XOR
            _ => panic!("Unknown funct3 for R-type"),
        };

        self.reg.gen_reg[instruction.rd as usize] = result;
    }

    fn sign_extend(value: u32, bits: u8) -> i32 {
        let shift = 32 - bits;
        ((value << shift) as i32) >> shift
    }

    fn execute_itype(&mut self, instruction: RV5Itype) {
        let rs1_val = self.reg.gen_reg[instruction.rs1 as usize];
        let imm_val = Self::sign_extend(instruction.imm, 12);

        let result = match instruction.funct3 {
            0b000 => rs1_val.wrapping_add(imm_val as u32), // ADDI (add immediate)
            0b111 => rs1_val & imm_val as u32,             // ANDI
            0b110 => rs1_val | imm_val as u32,             // ORI
            _ => panic!("Unknown funct3 for I-type"),
        };

        self.reg.gen_reg[instruction.rd as usize] = result;
    }

    fn execute_stype(&mut self, _instruction: RV5Stype) {
        todo!()
    }

    fn execute_sbtype(&mut self, _instruction: RV5SBtype) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::Read;

    fn load_binary(file_path: &str) -> Vec<u8> {
        let mut file = File::open(file_path).expect("Unable to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Unable to read file");
        buffer
    }

    #[test]
    fn test_cpu_fetch_and_execute_instruction() {
        let binary_data = load_binary("examples/simple/program.bin");
        let mut cpu = CPU::new();

        cpu.load_instructions(&binary_data);
        //  li
        cpu.execute_ins();
        cpu.execute_ins();

        // Execute ADD: x5 = x6 + x7
        cpu.execute_ins();
        assert_eq!(cpu.reg.gen_reg[5], 30); // x5 = 30

        // Execute SUB: x5 = x6 - x7
        cpu.execute_ins();
        assert_eq!(cpu.reg.gen_reg[5], -10i32 as u32); // x5 = -10 (unsigned wrap)

        // Execute AND: x5 = x6 & x7
        cpu.execute_ins();
        assert_eq!(cpu.reg.gen_reg[5], 10 & 20);

        // Execute OR: x5 = x6 | x7
        cpu.execute_ins();
        assert_eq!(cpu.reg.gen_reg[5], 10 | 20);

        // Execute XOR: x5 = x6 ^ x7
        cpu.execute_ins();
        assert_eq!(cpu.reg.gen_reg[5], 10 ^ 20);

        // Execute ADDI: x5 = x6 + (-5)
        cpu.execute_ins();
        assert_eq!(cpu.reg.gen_reg[5], 10 - 5); // x5 = 5
    }
}
