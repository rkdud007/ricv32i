//! ideally i wanted to have bit representation not bytes - should i use zig/c

pub enum RV5Instruction {
    R(RV5Rtype),
    I(RV5Itype),
    S(RV5Stype),
    SB(RV5SBtype),
    J(RV5Jtype),
    U(RVUtype),
    ECALL,
    EBREAK,
    NOP,
}

// | funct7  | rs2   | rs1   | funct3 | rd    | opcode |
// |:-------:|:-----:|:-----:|:------:|:-----:|:------:|
// | 7 bits  | 5 bits| 5 bits| 3 bits | 5 bits| 7 bits |
#[derive(Debug)]
pub struct RV5Rtype {
    pub funct7: u32,
    pub rs2: u32,
    pub rs1: u32,
    pub funct3: u32,
    pub rd: u32,
    pub opcode: u32,
}

// | imm[11:0]  | rs1   | funct3 | rd    | opcode |
// |:----------:|:-----:|:------:|:-----:|:------:|
// | 12 bits    | 5 bits| 3 bits | 5 bits| 7 bits |
#[derive(Debug)]
pub struct RV5Itype {
    pub imm: u32,
    pub rs1: u32,
    pub funct3: u32,
    pub rd: u32,
    pub opcode: u32,
}

// | imm[11:5]  | rs2   | rs1   | funct3 | imm[4:0] | opcode |
// |:----------:|:-----:|:-----:|:------:|:--------:|:------:|
// | 7 bits     | 5 bits| 5 bits| 3 bits | 5 bits   | 7 bits |
#[derive(Debug)]
pub struct RV5Stype {
    pub imm: u32,
    pub rs2: u32,
    pub rs1: u32,
    pub funct3: u32,
    pub opcode: u32,
}

// | imm[12,10:5] | rs2   | rs1   | funct3 | imm[4:1,11] | opcode |
// |:------------:|:-----:|:-----:|:------:|:-----------:|:------:|
// | 7 bits       | 5 bits| 5 bits| 3 bits | 5 bits      | 7 bits |
pub struct RV5SBtype {
    pub imm: u32,
    pub rs2: u32,
    pub rs1: u32,
    pub funct3: u32,
    pub opcode: u32,
}

// |imm[20]|  imm[10:1]  |imm[11]| imm[19:12] | rd    | opcode |
// |:-----:|:-----------:|:-----:|:----------:|:-----:|:------:|
// |1 bits |   10 bits   | 1 bits| 8 bits     | 5 bits| 7 bits |
pub struct RV5Jtype {
    pub imm: u32,
    pub rd: u32,
    pub opcode: u32,
}

// | imm[31:12]     | rd    | opcode |
// |:--------------:|:-----:|:------:|
// |   20 bits      | 5 bits| 7 bits |
pub struct RVUtype {
    pub imm20: u32,
    pub rd: u32,
    pub opcode: u32,
}

impl RV5Instruction {
    pub fn new(instruction: u32) -> Self {
        if instruction == 0x00000073 {
            println!("ecall");
            return Self::ECALL;
        } else if instruction == 0x00000000 {
            println!("Encountered NOP or uninitialized memory.");
            return Self::NOP;
        }
        let opcode = instruction & 0x7F; // bits 6-0
                                         // println!("ins: 0b{:07b} ", opcode);
        match opcode {
            0b0110011 => {
                let funct7 = (instruction >> 25) & 0x7F; // bits 31-25
                let rs2 = (instruction >> 20) & 0x1F; // bits 24-20
                let rs1 = (instruction >> 15) & 0x1F; // bits 19-15
                let funct3 = (instruction >> 12) & 0x7; // bits 14-12
                let rd = (instruction >> 7) & 0x1F; // bits 11-7
                RV5Instruction::R(RV5Rtype {
                    funct7,
                    rs2,
                    rs1,
                    funct3,
                    rd,
                    opcode,
                })
            }
            0b0000011 | 0b0010011 | 0b1100111 => {
                let imm = (instruction >> 20) & 0xFFF; // bits 31-20
                let rs1 = (instruction >> 15) & 0x1F; // bits 19-15
                let funct3 = (instruction >> 12) & 0x7; // bits 14-12
                let rd = (instruction >> 7) & 0x1F; // bits 11-7
                RV5Instruction::I(RV5Itype {
                    imm,
                    rs1,
                    funct3,
                    rd,
                    opcode,
                })
            }
            0b0100011 => {
                let imm_11_5 = (instruction >> 25) & 0x7F; // bits 31-25
                let rs2 = (instruction >> 20) & 0x1F; // bits 24-20
                let rs1 = (instruction >> 15) & 0x1F; // bits 19-15
                let funct3 = (instruction >> 12) & 0x7; // bits 14-12
                let imm_4_0 = (instruction >> 7) & 0x1F; // bits 11-7
                let imm = (imm_11_5 << 5) | imm_4_0; // Combine imm[11:5] and imm[4:0]
                RV5Instruction::S(RV5Stype {
                    imm,
                    rs2,
                    rs1,
                    funct3,
                    opcode,
                })
            }
            0b1100011 => {
                let imm_12 = (instruction >> 31) & 0x1; // bit 31 (imm[12])
                let imm_10_5 = (instruction >> 25) & 0x3F; // bits 30-25 (imm[10:5])
                let rs2 = (instruction >> 20) & 0x1F; // bits 24-20
                let rs1 = (instruction >> 15) & 0x1F; // bits 19-15
                let funct3 = (instruction >> 12) & 0x7; // bits 14-12
                let imm_4_1 = (instruction >> 8) & 0xF; // bits 11-8 (imm[4:1])
                let imm_11 = (instruction >> 7) & 0x1; // bit 7 (imm[11])

                // Combine the imm parts: imm[12|10:5|4:1|11] << 1
                let imm = (imm_12 << 12) | (imm_11 << 11) | (imm_10_5 << 5) | (imm_4_1 << 1);
                let opcode = instruction & 0x7F; // bits 6-0
                RV5Instruction::SB(RV5SBtype {
                    imm,
                    rs2,
                    rs1,
                    funct3,
                    opcode,
                })
            }
            0b0010111 | 0b0110111 => {
                let imm20 = (instruction >> 12) & 0x09; // bit 31-12
                let rd = (instruction >> 7) & 0x1F; // bits 11-7
                let opcode = instruction & 0x7F; // bits 6-0
                RV5Instruction::U(RVUtype { imm20, rd, opcode })
            }
            0b1101111 => {
                let rd = (instruction >> 7) & 0x1F;
                let imm_19_12 = (instruction >> 12) & 0xFF; // bits 19:12 (8 bits)
                let imm_11 = ((instruction >> 20) & 0x1) << 11;
                let imm_10_1 = ((instruction >> 21) & 0x3FF) << 1; // bits 30:21 (10 bits)
                let imm_20 = ((instruction >> 31) & 0x1) << 20;

                let imm = imm_20 | (imm_19_12 << 12) | imm_11 | imm_10_1;

                RV5Instruction::J(RV5Jtype { imm, rd, opcode })
            }
            0b1110011 => RV5Instruction::EBREAK,
            _ => panic!("Unknown instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rv5_instruction_r() {
        let instruction = 0x15A04B3; // add x9, x20, x21
        let rv5_instruction = RV5Instruction::new(instruction);

        match rv5_instruction {
            RV5Instruction::R(rv5_r_type) => {
                assert_eq!(rv5_r_type.funct7, 0b0000000);
                assert_eq!(rv5_r_type.rs2, 0b10101);
                assert_eq!(rv5_r_type.rs1, 0b10100);
                assert_eq!(rv5_r_type.funct3, 0b000);
                assert_eq!(rv5_r_type.rd, 0b01001);
                assert_eq!(rv5_r_type.opcode, 0b0110011);
            }
            _ => panic!("Expected RV5Rtype"),
        }
    }

    #[test]
    fn test_rv5_instruction_i() {
        let instruction = 0x3E813083; // ld x1, 1000(x2)
        let rv5_instruction = RV5Instruction::new(instruction);
        match rv5_instruction {
            RV5Instruction::I(rv5_i_type) => {
                assert_eq!(rv5_i_type.imm, 0b001111101000);
                assert_eq!(rv5_i_type.rs1, 0b00010);
                assert_eq!(rv5_i_type.funct3, 0b011);
                assert_eq!(rv5_i_type.rd, 0b00001);
                assert_eq!(rv5_i_type.opcode, 0b0000011);
            }
            _ => panic!("Expected RV5Itype"),
        }
    }

    #[test]
    fn test_rv5_instruction_s() {
        let instruction = 0x3E113423; // sd x1, 1000(x2)
        let rv5_instruction = RV5Instruction::new(instruction);
        match rv5_instruction {
            RV5Instruction::S(rv5_s_type) => {
                assert_eq!(rv5_s_type.imm, 0b001111101000);
                assert_eq!(rv5_s_type.rs2, 0b00001);
                assert_eq!(rv5_s_type.rs1, 0b00010);
                assert_eq!(rv5_s_type.funct3, 0b011);
                assert_eq!(rv5_s_type.opcode, 0b0100011);
            }
            _ => panic!("Expected RV5Stype"),
        }
    }

    #[test]
    fn test_rv5_instruction_sb() {
        let instruction = 0x7CB51863; // bne x10, x11, 2000
        let rv5_instruction = RV5Instruction::new(instruction);
        match rv5_instruction {
            RV5Instruction::SB(rv5_sb_type) => {
                assert_eq!(rv5_sb_type.imm, 0b0011111010000);
                assert_eq!(rv5_sb_type.rs2, 0b01011);
                assert_eq!(rv5_sb_type.rs1, 0b01010);
                assert_eq!(rv5_sb_type.funct3, 0b001);
                assert_eq!(rv5_sb_type.opcode, 0b1100011);
            }
            _ => panic!("Expected RV5SBtype"),
        }
    }
}
