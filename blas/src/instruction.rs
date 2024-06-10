use asmlib::instruction::Instruction;
use asmlib::instruction::OpCode;
use blaast::ASTInstruction;
use blaast::ASTInstructionKind;

use crate::ASMError;
use crate::Register;

/// Pseudo Instructions returned by the assembler.
///
/// R-Type Instructions:
/// R-INSTR(Destination Register, Source Register 1, Source Register 2)
///
/// I-Type Instructions:
/// I-INSTR(Destination Register, Source Register, Immediate)
///
/// B-Type Instructions:
/// B-INSTR(Source Register 1, Source Register 2, Offset)
#[derive(Debug, PartialEq)]
pub enum PseudoInstruction {
    // R-Type Instructions
    ADD(Register, Register, Register),
    SUB(Register, Register, Register),
    OR(Register, Register, Register),
    AND(Register, Register, Register),
    XOR(Register, Register, Register),
    SLL(Register, Register, Register),
    SRL(Register, Register, Register),
    // I-Type Instructions
    ADDI(Register, Register, u16),
    SUBI(Register, Register, u16),
    ORI(Register, Register, u16),
    ANDI(Register, Register, u16),
    XORI(Register, Register, u16),
    SLLI(Register, Register, u16),
    SRLI(Register, Register, u16),
    // Load/Store Instructions
    LD(Register, Register, u16),
    STR(Register, Register, u16),
    // B-Type Instructions
    BE(Register, Register, i16),
    BNE(Register, Register, i16),
    BLT(Register, Register, i16),
    BGE(Register, Register, i16),
    BLTU(Register, Register, i16),
    BGEU(Register, Register, i16),
}

impl TryFrom<ASTInstruction> for PseudoInstruction {
    type Error = ASMError;

    fn try_from(instruction: ASTInstruction) -> Result<Self, Self::Error> {
        use PseudoInstruction::*;

        let operands = instruction.operands();

        match instruction.kind() {
            // R-Type Instructions
            ASTInstructionKind::ADD => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let rt = operands[2].try_into()?;

                Ok(ADD(rd, rs, rt))
            }
            ASTInstructionKind::SUB => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let rt = operands[2].try_into()?;

                Ok(SUB(rd, rs, rt))
            }
            ASTInstructionKind::OR => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let rt = operands[2].try_into()?;

                Ok(OR(rd, rs, rt))
            }
            ASTInstructionKind::AND => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let rt = operands[2].try_into()?;

                Ok(AND(rd, rs, rt))
            }
            ASTInstructionKind::XOR => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let rt = operands[2].try_into()?;

                Ok(XOR(rd, rs, rt))
            }
            ASTInstructionKind::SLL => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let rt = operands[2].try_into()?;

                Ok(SLL(rd, rs, rt))
            }
            ASTInstructionKind::SRL => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let rt = operands[2].try_into()?;

                Ok(SRL(rd, rs, rt))
            }
            // I-Type Instructions
            ASTInstructionKind::ADDI => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let imm = operands[2].into();

                Ok(ADDI(rd, rs, imm))
            }
            ASTInstructionKind::SUBI => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let imm = operands[2].into();

                Ok(SUBI(rd, rs, imm))
            }
            ASTInstructionKind::ORI => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let imm = operands[2].into();

                Ok(ORI(rd, rs, imm))
            }
            ASTInstructionKind::ANDI => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let imm = operands[2].into();

                Ok(ANDI(rd, rs, imm))
            }
            ASTInstructionKind::XORI => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let imm = operands[2].into();

                Ok(XORI(rd, rs, imm))
            }
            ASTInstructionKind::SLLI => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let imm = operands[2].into();

                Ok(SLLI(rd, rs, imm))
            }
            ASTInstructionKind::SRLI => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let imm = operands[2].into();

                Ok(SRLI(rd, rs, imm))
            }
            // Load/Store Instructions
            ASTInstructionKind::LD => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let offset = operands[2].into();

                Ok(LD(rd, rs, offset))
            }
            ASTInstructionKind::STR => {
                let rd = operands[0].try_into()?;
                let rs = operands[1].try_into()?;
                let offset = operands[2].into();

                Ok(STR(rd, rs, offset))
            }
            // B-Type Instructions
            ASTInstructionKind::BE => {
                let rs = operands[0].try_into()?;
                let rt = operands[1].try_into()?;
                let offset = operands[2].into();

                Ok(BE(rs, rt, offset))
            }
            ASTInstructionKind::BNE => {
                let rs = operands[0].try_into()?;
                let rt = operands[1].try_into()?;
                let offset = operands[2].into();

                Ok(BNE(rs, rt, offset))
            }
            ASTInstructionKind::BLT => {
                let rs = operands[0].try_into()?;
                let rt = operands[1].try_into()?;
                let offset = operands[2].into();

                Ok(BLT(rs, rt, offset))
            }
            ASTInstructionKind::BGE => {
                let rs = operands[0].try_into()?;
                let rt = operands[1].try_into()?;
                let offset = operands[2].into();

                Ok(BGE(rs, rt, offset))
            }
            ASTInstructionKind::BLTU => {
                let rs = operands[0].try_into()?;
                let rt = operands[1].try_into()?;
                let offset = operands[2].into();

                Ok(BLTU(rs, rt, offset))
            }
            ASTInstructionKind::BGEU => {
                let rs = operands[0].try_into()?;
                let rt = operands[1].try_into()?;
                let offset = operands[2].into();

                Ok(BGEU(rs, rt, offset))
            }
        }
    }
}

impl From<PseudoInstruction> for Instruction {
    fn from(value: PseudoInstruction) -> Self {
        match value {
            PseudoInstruction::ADD(rd, rs1, rs2)
            | PseudoInstruction::SUB(rd, rs1, rs2)
            | PseudoInstruction::OR(rd, rs1, rs2)
            | PseudoInstruction::AND(rd, rs1, rs2)
            | PseudoInstruction::XOR(rd, rs1, rs2)
            | PseudoInstruction::SLL(rd, rs1, rs2)
            | PseudoInstruction::SRL(rd, rs1, rs2) => {
                let mut instr = Instruction::new(OpCode::ADD);
                instr.set_rd(rd as u32);
                instr.set_rs1(rs1 as u32);
                instr.set_rs2(rs2 as u32);

                instr
            }
            PseudoInstruction::ADDI(rd, rs1, imm)
            | PseudoInstruction::SUBI(rd, rs1, imm)
            | PseudoInstruction::ORI(rd, rs1, imm)
            | PseudoInstruction::ANDI(rd, rs1, imm)
            | PseudoInstruction::XORI(rd, rs1, imm)
            | PseudoInstruction::SLLI(rd, rs1, imm)
            | PseudoInstruction::SRLI(rd, rs1, imm) => {
                let mut instr = Instruction::new(OpCode::from(value));
                instr.set_rd(rd as u32);
                instr.set_rs1(rs1 as u32);
                instr.set_immediate(imm as u32);

                instr
            }
            PseudoInstruction::LD(rd, rs, offset) | PseudoInstruction::STR(rd, rs, offset) => {
                let mut instr = Instruction::new(OpCode::from(value));
                instr.set_rd(rd as u32);
                instr.set_rs1(rs as u32);
                instr.set_immediate(offset as u32);

                instr
            }
            PseudoInstruction::BE(rs1, rs2, offset)
            | PseudoInstruction::BNE(rs1, rs2, offset)
            | PseudoInstruction::BLT(rs1, rs2, offset)
            | PseudoInstruction::BGE(rs1, rs2, offset)
            | PseudoInstruction::BLTU(rs1, rs2, offset)
            | PseudoInstruction::BGEU(rs1, rs2, offset) => {
                let mut instr = Instruction::new(OpCode::from(value));
                instr.set_rs1(rs1 as u32);
                instr.set_rs2(rs2 as u32);
                instr.set_lower((offset & 15) as u32);
                instr.set_upper((offset & !15) as u32);

                instr
            }
        }
    }
}

impl From<PseudoInstruction> for OpCode {
    fn from(value: PseudoInstruction) -> Self {
        match value {
            PseudoInstruction::ADD(_, _, _) => OpCode::ADD,
            PseudoInstruction::SUB(_, _, _) => OpCode::SUB,
            PseudoInstruction::OR(_, _, _) => OpCode::OR,
            PseudoInstruction::AND(_, _, _) => OpCode::AND,
            PseudoInstruction::XOR(_, _, _) => OpCode::XOR,
            PseudoInstruction::SLL(_, _, _) => OpCode::SLL,
            PseudoInstruction::SRL(_, _, _) => OpCode::SRL,
            PseudoInstruction::ADDI(_, _, _) => OpCode::ADDI,
            PseudoInstruction::SUBI(_, _, _) => OpCode::SUBI,
            PseudoInstruction::ORI(_, _, _) => OpCode::ORI,
            PseudoInstruction::ANDI(_, _, _) => OpCode::ANDI,
            PseudoInstruction::XORI(_, _, _) => OpCode::XORI,
            PseudoInstruction::SLLI(_, _, _) => OpCode::SLLI,
            PseudoInstruction::SRLI(_, _, _) => OpCode::SRLI,
            PseudoInstruction::LD(_, _, _) => OpCode::LD,
            PseudoInstruction::STR(_, _, _) => OpCode::STR,
            PseudoInstruction::BE(_, _, _) => OpCode::BE,
            PseudoInstruction::BNE(_, _, _) => OpCode::BNE,
            PseudoInstruction::BLT(_, _, _) => OpCode::BLT,
            PseudoInstruction::BGE(_, _, _) => OpCode::BGE,
            PseudoInstruction::BLTU(_, _, _) => OpCode::BLTU,
            PseudoInstruction::BGEU(_, _, _) => OpCode::BGEU,
        }
    }
}
