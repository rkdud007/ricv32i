use elf::{endian::AnyEndian, ElfBytes};

use crate::{
    instruction::{RV5Instruction, RV5Itype, RV5Jtype, RV5Rtype, RV5SBtype, RV5Stype, RVUtype},
    ram::{RAM, RAM_SIZE},
};

// 32(general purpose) + 1(PC)
const REGISTER_COUNT: usize = 33;
const PC_INDEX: usize = 32;
const INITIAL_PC: usize = 0x80000000;

/// 32-bit RISC-V
#[derive(Debug)]
pub struct CPU {
    /// register
    pub reg: [u32; REGISTER_COUNT],
    /// clock cycle
    pub clk: u32,
    /// random access memory
    pub ram: RAM,
    /// process exit flag
    pub exited: bool,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn is_exited(&self) -> bool {
        self.exited
    }

    pub fn new() -> Self {
        let ram = RAM::new();
        let mut reg = [0u32; REGISTER_COUNT];
        reg[PC_INDEX] = INITIAL_PC as u32;
        CPU {
            reg,
            clk: 0,
            ram,
            exited: false,
        }
    }

    /// Load ELF binary into memory
    pub fn load_elf(&mut self, binary_data: &[u8]) {
        let elf = ElfBytes::<AnyEndian>::minimal_parse(binary_data).expect("Failed to parse ELF");

        if let Ok(Some(shdr)) = elf.section_header_by_name(".text.init") {
            let p_paddr = shdr.sh_addr;
            let (data, _) = elf
                .section_data(&shdr)
                .expect("Failed to load .text section");
            let write_addr = p_paddr - INITIAL_PC as u64;
            assert!(write_addr < RAM_SIZE as u64, "Address out of range");
            self.ram.write_bytes(write_addr as usize, data);
        }
    }

    pub fn load_instructions(&mut self, binary_data: &[u8]) {
        for (i, chunk) in binary_data.chunks(4).enumerate() {
            println!("Writing instruction at index: {:?}", chunk);
            self.ram.write_word(
                i * 4,
                u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]),
            );
        }
    }

    /// Note that instruction is a 32-bit value
    pub fn fetch_ins(&mut self) -> u32 {
        let addr = self.reg[PC_INDEX];
        let ram_addr = addr.wrapping_sub(INITIAL_PC as u32);
        assert!(ram_addr < RAM_SIZE as u32, "Address out of range");
        self.ram.read_word(ram_addr as usize)
    }

    pub fn execute_ins(&mut self) {
        let instruction = self.fetch_ins();

        let decoded_instruction = self.decode_ins(instruction);
        match decoded_instruction {
            RV5Instruction::R(rv5_r_type) => self.execute_rtype(rv5_r_type),
            RV5Instruction::I(rv5_i_type) => self.execute_itype(rv5_i_type),
            RV5Instruction::S(rv5_s_type) => self.execute_stype(rv5_s_type),
            RV5Instruction::SB(rv5_sb_type) => self.execute_sbtype(rv5_sb_type),
            RV5Instruction::U(rv5_u_type) => self.execute_utype(rv5_u_type),
            RV5Instruction::J(rv5_j_type) => self.execute_jtype(rv5_j_type),
            RV5Instruction::ECALL => self.handle_ecall(),
            RV5Instruction::EBREAK => {
                self.exited = true;
                println!("Encountered EBREAK ending process.");
            }
            RV5Instruction::NOP => {
                self.exited = true;
                println!("Encountered NOP or uninitialized memory.");
            }
        }
        self.clk += 1;
        self.reg[PC_INDEX] = self.reg[PC_INDEX].wrapping_add(4);
    }

    fn handle_ecall(&mut self) {
        match self.reg[17] {
            1 => {
                // a0 (x10)
                println!("{}", self.reg[10]);
            }
            4 => {
                // a0 (x10)
                let addr = self.reg[10] as usize;
                let mut s = String::new();
                let mut i = addr;
                loop {
                    let byte = self.ram.data[i];
                    if byte == 0 {
                        break;
                    }
                    s.push(byte as char);
                    i += 1;
                }
                print!("{}", s);
            }
            10 => {
                println!("Program exiting.");
                self.exited = true;
            }
            _ => panic!("Unknown syscall number: {}", self.reg[17]),
        }
    }

    /// Decode the instruction
    fn decode_ins(&self, instruction: u32) -> RV5Instruction {
        RV5Instruction::new(instruction)
    }

    fn execute_rtype(&mut self, instruction: RV5Rtype) {
        let rs1_val = self.reg[instruction.rs1 as usize];
        let rs2_val = self.reg[instruction.rs2 as usize];

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

        self.reg[instruction.rd as usize] = result;
    }

    fn sign_extend(value: u32, bits: u8) -> i32 {
        let shift = 32 - bits;
        ((value << shift) as i32) >> shift
    }

    fn execute_itype(&mut self, instruction: RV5Itype) {
        let rs1_val = self.reg[instruction.rs1 as usize];
        let imm_val = Self::sign_extend(instruction.imm, 12);

        let result = match instruction.funct3 {
            0b000 => rs1_val.wrapping_add(imm_val as u32), // ADDI (add immediate)
            0b111 => rs1_val & imm_val as u32,             // ANDI
            0b110 => rs1_val | imm_val as u32,             // ORI
            _ => panic!("Unknown funct3 for I-type"),
        };

        self.reg[instruction.rd as usize] = result;
    }

    fn execute_stype(&mut self, _instruction: RV5Stype) {
        todo!()
    }

    fn execute_sbtype(&mut self, _instruction: RV5SBtype) {
        todo!()
    }

    fn execute_utype(&mut self, instruction: RVUtype) {
        let rd = instruction.rd as usize;
        let opcode = instruction.opcode;

        // sign extend
        let imm_shifted = (instruction.imm20 << 12) as i32;

        match opcode {
            0x37 => {
                // LUI: x[rd] = imm (which is already imm20 << 12 sign-extended)
                self.reg[rd] = imm_shifted as u32;
            }
            0x17 => {
                // AUIPC: x[rd] = PC + imm_shifted
                let current_pc = self.reg[PC_INDEX];
                self.reg[rd] = ((current_pc as i32).wrapping_add(imm_shifted)) as u32;
            }
            _ => panic!("Unknown U-type opcode"),
        }
    }

    fn execute_jtype(&mut self, instruction: RV5Jtype) {
        let current_pc = self.reg[PC_INDEX];
        let offset = Self::sign_extend(instruction.imm, 21) as i32;

        // The return address is PC + 4
        let return_addr = current_pc.wrapping_add(4);
        if instruction.rd != 0 {
            self.reg[instruction.rd as usize] = return_addr;
        }

        // Update the PC: PC = PC + offset
        self.reg[PC_INDEX] = (current_pc as i32).wrapping_add(offset) as u32;
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
        println!("Binary data size: {:?}", binary_data.len());
        let mut cpu = CPU::new();
        cpu.load_instructions(&binary_data);
        //  li
        cpu.execute_ins();
        cpu.execute_ins();

        // Execute ADD: x5 = x6 + x7
        cpu.execute_ins();
        assert_eq!(cpu.reg[5], 30); // x5 = 30

        // Execute SUB: x5 = x6 - x7
        cpu.execute_ins();
        assert_eq!(cpu.reg[5], -10i32 as u32); // x5 = -10 (unsigned wrap)

        // Execute AND: x5 = x6 & x7
        cpu.execute_ins();
        assert_eq!(cpu.reg[5], 10 & 20);

        // Execute OR: x5 = x6 | x7
        cpu.execute_ins();
        assert_eq!(cpu.reg[5], 10 | 20);

        // Execute XOR: x5 = x6 ^ x7
        cpu.execute_ins();
        assert_eq!(cpu.reg[5], 10 ^ 20);

        // Execute ADDI: x5 = x6 + (-5)
        cpu.execute_ins();
        assert_eq!(cpu.reg[5], 10 - 5); // x5 = 5
    }

    // #[test]
    // fn test_ecall_handling() {
    //     let binary_data = load_binary("examples/hello_world/program.bin");
    //     println!("Binary data size: {:?}", binary_data.len());
    //     let mut cpu = CPU::new();
    //     cpu.load_instructions(&binary_data);

    //     cpu.execute_ins();
    //     cpu.execute_ins();
    //     cpu.execute_ins();
    //     cpu.execute_ins();
    //     cpu.execute_ins();
    //     cpu.execute_ins();
    //     cpu.execute_ins();
    //     cpu.execute_ins();
    //     cpu.execute_ins();
    //     cpu.execute_ins();

    //     assert_eq!(cpu.clk, 10);
    // }
}
