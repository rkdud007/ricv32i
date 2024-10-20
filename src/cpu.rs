use crate::{
    instruction::{RV5Instruction, RV5Itype, RV5Rtype, RV5SBtype, RV5Stype},
    ram::RAM,
};

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

    /// Load instructions into RAM at the given address
    pub fn load_instructions(&mut self, instructions: &[u32], v_addr: usize) {
        for (i, &instruction) in instructions.iter().enumerate() {
            self.ram.write_word(v_addr + i * 4, instruction); // Write each instruction as a 32-bit word
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
        let rs1_val = self.reg.data[instruction.rs1 as usize];
        let rs2_val = self.reg.data[instruction.rs2 as usize];

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

        // Write the result to the destination register
        self.reg.data[instruction.rd as usize] = result;
    }

    fn sign_extend(value: u32, bits: u8) -> i32 {
        let shift = 32 - bits;
        ((value << shift) as i32) >> shift
    }

    fn execute_itype(&mut self, instruction: RV5Itype) {
        let rs1_val = self.reg.data[instruction.rs1 as usize];
        let imm_val = Self::sign_extend(instruction.imm, 12); // Correctly sign-extend the immediate

        let result = match instruction.funct3 {
            0b000 => rs1_val.wrapping_add(imm_val as u32), // ADDI (add immediate)
            0b111 => rs1_val & imm_val as u32,             // ANDI
            0b110 => rs1_val | imm_val as u32,             // ORI
            _ => panic!("Unknown funct3 for I-type"),
        };

        self.reg.data[instruction.rd as usize] = result;
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

    #[test]
    fn test_cpu_fetch_and_execute_instruction() {
        let mut cpu = CPU::new();

        let instructions = [
            0x007302b3, // add x5, x6, x7
            0x00528293, // addi x5, x5, 5
            0x0062f3b3, // and x7, x5, x6
        ];

        cpu.load_instructions(&instructions, 0);
        cpu.reg.data[6] = 10; // x6 = 10
        cpu.reg.data[7] = 20; // x7 = 20

        cpu.execute_ins();
        assert_eq!(cpu.reg.data[5], 30); // x5 = 10 + 20 = 30
        assert_eq!(cpu.clk, 1);

        cpu.execute_ins();
        assert_eq!(cpu.reg.data[5], 35); // x5 = 30 + 5 = 35
        assert_eq!(cpu.clk, 2);

        cpu.execute_ins();
        assert_eq!(cpu.reg.data[7], 35 & 10); // x7 = x5 & x6 = 35 & 10 = 2
        assert_eq!(cpu.clk, 3);
    }

    #[test]
    fn test_cpu_sign_extension_with_negative_imm() {
        let mut cpu = CPU::new();

        // Instruction: addi x5, x6, -5
        let imm = (-5i16 as u16) as u32;
        let instruction = (imm << 20) | (6 << 15) | (5 << 7) | 0b0010011;
        // https://luplab.gitlab.io/rvcodecjs/#q=0xffb30293&abi=false&isa=AUTO
        println!("Instruction: {:#x}", instruction);

        cpu.load_instructions(&[instruction], 0);
        cpu.reg.data[6] = 10; // x6 = 10

        cpu.execute_ins();

        // Expected result: x5 = x6 + (-5) = 10 + (-5) = 5
        assert_eq!(cpu.reg.data[5], 5);
        assert_eq!(cpu.clk, 1);
    }
}
