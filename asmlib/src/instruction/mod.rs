#![allow(dead_code)]

use bitfield::bitfield;

#[derive(Debug, PartialEq)]
pub enum OpCode {

    // R
    ADD = 0b0000_0000,
    SUB = 0b0001_0000,
    OR = 0b0010_0000,
    AND = 0b0011_0000,
    XOR = 0b0100_0000,
    SLL = 0b0110_0000,
    SRL = 0b0111_0000,
    // I
    ADDI = 0b0000_0001,
    SUBI = 0b0001_0001,
    ORI = 0b0010_0001,
    ANDI = 0b0011_0001,
    XORI = 0b0100_0001,
    SLLI = 0b0110_0001,
    SRLI = 0b0111_0001,
    // Load/Store
    LD = 0b0000_0011,
    STR = 0b0001_0011,
    // B
    BE = 0b0000_0010,
    BNE = 0b0001_0010,
    BLT = 0b0100_0010,
    BGE = 0b0101_0010,
    BLTU = 0b0110_0010,
    BGEU = 0b0111_0010,
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<OpCode> for u8 {
    fn from(instr: OpCode) -> Self {
        instr as u8
    }
}

impl From<u8> for OpCode {
    fn from(instr: u8) -> Self {
        match instr {
            // R-Type Instructions
            0b0000_0000 => OpCode::ADD,
            0b0001_0000 => OpCode::SUB,
            0b0010_0000 => OpCode::OR,
            0b0011_0000 => OpCode::AND,
            0b0100_0000 => OpCode::XOR,
            0b0110_0000 => OpCode::SLL,
            0b0111_0000 => OpCode::SRL,
            // I-Type Instructions
            0b0000_0001 => OpCode::ADDI,
            0b0001_0001 => OpCode::SUBI,
            0b0010_0001 => OpCode::ORI,
            0b0011_0001 => OpCode::ANDI,
            0b0100_0001 => OpCode::XORI,
            0b0110_0001 => OpCode::SLLI,
            0b0111_0001 => OpCode::SRLI,
            // Load/Store Instructions
            0b0000_0011 => OpCode::LD,
            0b0001_0011 => OpCode::STR,
            // B-Type Instructions
            0b0000_0010 => OpCode::BE,
            0b0001_0010 => OpCode::BNE,
            0b0100_0010 => OpCode::BLT,
            0b0101_0010 => OpCode::BGE,
            0b0110_0010 => OpCode::BLTU,
            0b0111_0010 => OpCode::BGEU,
            _ => panic!("[Instruction] Unknown value : {} ", instr),
        }
    }
}

bitfield! {
    #[derive(Copy,Clone)]
    pub struct RInstruction(u32);
    pub u8, from into OpCode, get_opcode, set_opcode: 7, 0;
    pub get_rd, set_rd: 11, 8;
    pub get_rs1, set_rs1: 15, 12;
    pub get_rs2, set_rs2: 19, 16;
    pub _, __: 31, 20; // zero field
}

bitfield! {
    #[derive(Copy,Clone)]
    pub struct IInstruction(u32);
    pub u8, from into OpCode, get_opcode, set_opcode: 7, 0;
    pub get_rd, set_rd: 11, 8;
    pub get_rs1, set_rs1: 15, 12;
    pub get_immediate, set_immediate: 31, 16;
}

bitfield! {
    #[derive(Copy,Clone)]
    pub struct BInstruction(u32);
    pub u8, from into OpCode, get_opcode, set_opcode: 7, 0;
    pub get_lower, set_lower: 11, 8;
    pub get_rs1, set_rs1: 15, 12;
    pub get_rs2, set_rs2: 19, 16;
    pub get_upper, set_upper: 31, 20;
}

#[derive(Copy, Clone)]
pub enum InstructionEnum {
    RInstruction(RInstruction),
    IInstruction(IInstruction),
    BInstruction(BInstruction),
}

#[derive(Copy, Clone)]
pub struct Instruction {
    pub instruction: InstructionEnum,
}

// todo implement upper lower
impl Instruction {
    pub fn new(opcode: OpCode) -> Self {
        match opcode {
            OpCode::ADD
            | OpCode::SUB
            | OpCode::OR
            | OpCode::AND
            | OpCode::XOR
            | OpCode::SLL
            | OpCode::SRL => Self {
                instruction: InstructionEnum::RInstruction(RInstruction(opcode as u32)),
            },
            OpCode::ADDI
            | OpCode::SUBI
            | OpCode::ORI
            | OpCode::ANDI
            | OpCode::XORI
            | OpCode::SLLI
            | OpCode::SRLI
            | OpCode::LD
            | OpCode::STR => Self {
                instruction: InstructionEnum::IInstruction(IInstruction(opcode as u32)),
            },
            OpCode::BE | OpCode::BNE | OpCode::BLT | OpCode::BGE | OpCode::BLTU | OpCode::BGEU => {
                Self {
                    instruction: InstructionEnum::BInstruction(BInstruction(opcode as u32)),
                }
            }
        }
    }

    pub fn get_upper(&self) -> u32 {
        match self.instruction {
            InstructionEnum::RInstruction(_) => panic!("RInstruction does not support get_upper"),
            InstructionEnum::IInstruction(_) => panic!("IInstruction does not support get_upper"),
            InstructionEnum::BInstruction(inst) => inst.get_upper(),
        }
    }

    pub fn get_lower(&self) -> u32 {
        match self.instruction {
            InstructionEnum::RInstruction(_) => panic!("RInstruction does not support get_upper"),
            InstructionEnum::IInstruction(_) => panic!("IInstruction does not support get_upper"),
            InstructionEnum::BInstruction(inst) => inst.get_lower(),
        }
    }

    pub fn set_upper(&self, upper: u32) {
        match self.instruction {
            InstructionEnum::RInstruction(_) => panic!("RInstruction does not support get_upper"),
            InstructionEnum::IInstruction(_) => panic!("IInstruction does not support get_upper"),
            InstructionEnum::BInstruction(mut inst) => inst.set_upper(upper),
        }
    }

    pub fn set_lower(&mut self, lower: u32) {
        match self.instruction {
            InstructionEnum::RInstruction(_) => panic!("RInstruction does not support get_upper"),
            InstructionEnum::IInstruction(_) => panic!("IInstruction does not support get_upper"),
            InstructionEnum::BInstruction(mut inst) => {
                inst.set_lower(lower);
                self.instruction = InstructionEnum::BInstruction(inst);
            }
        }
    }

    pub fn get_rd(&self) -> u32 {
        match self.instruction {
            InstructionEnum::RInstruction(inst) => inst.get_rd(),
            InstructionEnum::IInstruction(inst) => inst.get_rd(),
            InstructionEnum::BInstruction(_) => panic!("BInstruction does not support get_rd"),
        }
    }

    pub fn set_rd(&mut self, rd: u32) {
        match self.instruction {
            InstructionEnum::RInstruction(mut inst) => {
                inst.set_rd(rd);
                self.instruction = InstructionEnum::RInstruction(inst);
            }
            InstructionEnum::IInstruction(mut inst) => {
                inst.set_rd(rd);
                self.instruction = InstructionEnum::IInstruction(inst);
            }
            InstructionEnum::BInstruction(_) => panic!("BInstruction does not support set_rd"),
        }
    }

    pub fn get_rs1(&self) -> u32 {
        match self.instruction {
            InstructionEnum::RInstruction(inst) => inst.get_rs1(),
            InstructionEnum::IInstruction(inst) => inst.get_rs1(),
            InstructionEnum::BInstruction(inst) => inst.get_rs1(),
        }
    }

    pub fn set_rs1(&mut self, rs1: u32) {
        match self.instruction {
            InstructionEnum::RInstruction(mut inst) => {
                inst.set_rs1(rs1);
                self.instruction = InstructionEnum::RInstruction(inst);
            }
            InstructionEnum::IInstruction(mut inst) => {
                inst.set_rs1(rs1);
                self.instruction = InstructionEnum::IInstruction(inst);
            }
            InstructionEnum::BInstruction(mut inst) => {
                inst.set_rs1(rs1);
                self.instruction = InstructionEnum::BInstruction(inst);
            }
        }
    }

    pub fn get_rs2(&self) -> u32 {
        match self.instruction {
            InstructionEnum::RInstruction(inst) => inst.get_rs2(),
            InstructionEnum::IInstruction(_) => panic!("BInstruction does not support get_rs2"),
            InstructionEnum::BInstruction(inst) => inst.get_rs2(),
        }
    }

    pub fn set_rs2(&mut self, rs2: u32) {
        match self.instruction {
            InstructionEnum::RInstruction(mut inst) => {
                inst.set_rs2(rs2);
                self.instruction = InstructionEnum::RInstruction(inst);
            }
            InstructionEnum::IInstruction(_) => panic!("BInstruction does not support set_rs2"),
            InstructionEnum::BInstruction(mut inst) => {
                inst.set_rs2(rs2);
                self.instruction = InstructionEnum::BInstruction(inst);
            }
        }
    }

    pub fn get_opcode(&self) -> OpCode {
        match self.instruction {
            InstructionEnum::RInstruction(inst) => inst.get_opcode(),
            InstructionEnum::IInstruction(inst) => inst.get_opcode(),
            InstructionEnum::BInstruction(inst) => inst.get_opcode(),
        }
    }

    pub fn set_opcode(&mut self, opcode: OpCode) {
        match self.instruction {
            InstructionEnum::RInstruction(mut inst) => {
                inst.set_opcode(opcode);
                self.instruction = InstructionEnum::RInstruction(inst);
            }
            InstructionEnum::IInstruction(mut inst) => {
                inst.set_opcode(opcode);
                self.instruction = InstructionEnum::IInstruction(inst);
            }
            InstructionEnum::BInstruction(mut inst) => {
                inst.set_opcode(opcode);
                self.instruction = InstructionEnum::BInstruction(inst);
            }
        }
    }

    pub fn get_immediate(&self) -> u32 {
        match self.instruction {
            InstructionEnum::RInstruction(_) => {
                panic!("RInstruction does not support get_immediate")
            }
            InstructionEnum::IInstruction(inst) => inst.get_immediate(),
            InstructionEnum::BInstruction(_) => {
                panic!("BInstruction does not support get_immediate")
            }
        }
    }

    pub fn set_immediate(&mut self, immediate: u32) {
        match self.instruction {
            InstructionEnum::RInstruction(_) => {
                panic!("RInstruction does not support get_immediate")
            }
            InstructionEnum::IInstruction(mut inst) => {
                inst.set_immediate(immediate);
                self.instruction = InstructionEnum::IInstruction(inst);
            }
            InstructionEnum::BInstruction(_) => {
                panic!("BInstruction does not support get_immediate")
            }
        }
    }
}

pub fn encode_instruction(instruction: Instruction) -> u32 {
    match instruction.instruction {
        InstructionEnum::RInstruction(inst) => inst.0 as u32,
        InstructionEnum::IInstruction(inst) => inst.0 as u32,
        InstructionEnum::BInstruction(inst) => inst.0 as u32,
    }
}

pub fn decode_instruction(instruction: u32) -> Instruction {
    if instruction >> 1 & 1 == 0 && instruction & 1 == 0 {
        Instruction {
            instruction: InstructionEnum::RInstruction(RInstruction(instruction)),
        }
    } else if instruction >> 1 & 1 == 0 && instruction & 1 == 1 {
        Instruction {
            instruction: InstructionEnum::IInstruction(IInstruction(instruction)),
        }
    } else if instruction >> 1 & 1 == 1 && instruction & 1 == 0 {
        Instruction {
            instruction: InstructionEnum::BInstruction(BInstruction(instruction)),
        }
    } else {
        panic!(
            "[read_instruction] unrecognized instruction bit 1 is \"{}\" bit 0 is \"{}\"",
            instruction >> 1 & 1,
            instruction & 1
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::*;

    #[test]
    fn instruction_decode_add() {
        let inst: Instruction = decode_instruction(0b0001_0011_0111_0000_0000);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_opcode(), OpCode::ADD);
    }

    #[test]
    fn instruction_decode_sub() {
        let inst: Instruction = decode_instruction(0b0001_0011_0111_0001_0000);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_opcode(), OpCode::SUB);
    }

    #[test]
    fn instruction_decode_or() {
        let inst: Instruction = decode_instruction(0b0001_0011_0111_0010_0000);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_opcode(), OpCode::OR);
    }

    #[test]
    fn instruction_decode_and() {
        let inst: Instruction = decode_instruction(0b0001_0011_0111_0011_0000);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_opcode(), OpCode::AND);
    }

    #[test]
    fn instruction_decode_xor() {
        let inst: Instruction = decode_instruction(0b0001_0011_0111_0100_0000);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_opcode(), OpCode::XOR);
    }

    #[test]
    fn instruction_decode_sll() {
        let inst: Instruction = decode_instruction(0b0001_0011_0111_0110_0000);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_opcode(), OpCode::SLL);
    }

    #[test]
    fn instruction_decode_srl() {
        let inst: Instruction = decode_instruction(0b0001_0011_0111_0111_0000);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_opcode(), OpCode::SRL);
    }

    #[test]
    fn instruction_decode_addi() {
        let inst: Instruction = decode_instruction(0b0001_0010_0100_1001_0011_0111_0000_0001);
        assert_eq!(inst.get_opcode(), OpCode::ADDI);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_immediate(), 4681);
    }

    #[test]
    fn instruction_decode_subi() {
        let inst: Instruction = decode_instruction(0b0001_0010_0100_1001_0011_0111_0001_0001);
        assert_eq!(inst.get_opcode(), OpCode::SUBI);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_immediate(), 4681);
    }

    #[test]
    fn instruction_decode_ori() {
        let inst: Instruction = decode_instruction(0b0001_0010_0100_1001_0011_0111_0010_0001);
        assert_eq!(inst.get_opcode(), OpCode::ORI);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_immediate(), 4681);
    }

    #[test]
    fn instruction_decode_andi() {
        let inst: Instruction = decode_instruction(0b0001_0010_0100_1001_0011_0111_0011_0001);
        assert_eq!(inst.get_opcode(), OpCode::ANDI);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_immediate(), 4681);
    }

    #[test]
    fn instruction_decode_xori() {
        let inst: Instruction = decode_instruction(0b0001_0010_0100_1001_0011_0111_0100_0001);
        assert_eq!(inst.get_opcode(), OpCode::XORI);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_immediate(), 4681);
    }

    #[test]
    fn instruction_decode_slli() {
        let inst: Instruction = decode_instruction(0b0001_0010_0100_1001_0011_0111_0110_0001);
        assert_eq!(inst.get_opcode(), OpCode::SLLI);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_immediate(), 4681);
    }

    #[test]
    fn instruction_decode_srli() {
        let inst: Instruction = decode_instruction(0b0001_0010_0100_1001_0011_0111_0111_0001);
        assert_eq!(inst.get_opcode(), OpCode::SRLI);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_immediate(), 4681);
    }

    #[test]
    fn rinstruction_decode_be() {
        let inst: Instruction = decode_instruction(0b0001_1010_0001_0011_0100_0000_0010);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_lower(), 4);
        assert_eq!(inst.get_upper(), 26);
        assert_eq!(inst.get_opcode(), OpCode::BE);
    }

    #[test]
    fn rinstruction_decode_bne() {
        let inst: Instruction = decode_instruction(0b0001_1010_0001_0011_0100_0001_0010);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_lower(), 4);
        assert_eq!(inst.get_upper(), 26);
        assert_eq!(inst.get_opcode(), OpCode::BNE);
    }

    #[test]
    fn rinstruction_decode_blt() {
        let inst: Instruction = decode_instruction(0b0001_1010_0001_0011_0100_0100_0010);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_lower(), 4);
        assert_eq!(inst.get_upper(), 26);
        assert_eq!(inst.get_opcode(), OpCode::BLT);
    }

    #[test]
    fn instruction_decode_bge() {
        let inst: Instruction = decode_instruction(0b0001_1010_0001_0011_0100_0101_0010);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_lower(), 4);
        assert_eq!(inst.get_upper(), 26);
        assert_eq!(inst.get_opcode(), OpCode::BGE);
    }

    #[test]
    fn instruction_decode_bltu() {
        let inst: Instruction = decode_instruction(0b0001_1010_0001_0011_0100_0110_0010);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_lower(), 4);
        assert_eq!(inst.get_upper(), 26);
        assert_eq!(inst.get_opcode(), OpCode::BLTU);
    }

    #[test]
    fn instruction_decode_bgeu() {
        let inst: Instruction = decode_instruction(0b0001_1010_0001_0011_0100_0111_0010);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_lower(), 4);
        assert_eq!(inst.get_upper(), 26);
        assert_eq!(inst.get_opcode(), OpCode::BGEU);
    }

    #[test]
    fn rinstruction_encoder_test() {
        let mut inst: RInstruction = RInstruction(0);
        inst.set_opcode(OpCode::ADD);
        inst.set_rs2(1);
        inst.set_rs1(3);
        inst.set_rd(7);
        let binary = inst.0;
        // | rs2 19 ... 16 | rs1 15 ... 12 | rd 11 .. 8 | (instruction) 7 ..... 0
        let expected = 0b0001_0011_0111_0000_0000;
        assert_eq!(
            binary, expected,
            "expected 0b{:b} got 0b{:b}",
            expected, binary
        );
    }

    #[test]
    fn rinstruction_decoder_test() {
        let inst: RInstruction = RInstruction(0b0001_0011_0111_0000_0000);
        assert_eq!(inst.get_rs2(), 1);
        assert_eq!(inst.get_rs1(), 3);
        assert_eq!(inst.get_rd(), 7);
        assert_eq!(inst.get_opcode(), OpCode::ADD);
    }

    #[test]
    fn general_instruction_decoder_test() {
        let mut inst: Instruction = Instruction {
            instruction: InstructionEnum::RInstruction(RInstruction(0b0001_0011_0111_0000_0000)),
        };
        inst.set_opcode(OpCode::ADD);
        assert_eq!(inst.get_opcode(), OpCode::ADD);
    }

    #[test]
    fn decode_instruction_iinstruction() {
        let instruction = decode_instruction(0b0001_0001_0010_0000_0001);
        assert_eq!(instruction.get_immediate(), 1);
        assert_eq!(instruction.get_rd(), 2);
        assert_eq!(instruction.get_rs1(), 1);
        assert_eq!(instruction.get_opcode(), OpCode::ADDI);
        assert!(matches!(
            instruction.instruction,
            InstructionEnum::IInstruction(_)
        ));
    }
}
