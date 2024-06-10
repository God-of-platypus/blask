use blaast::ASTOperand;
use blaast::ASTOperandKind;

use crate::ASMError;
use crate::ASMErrorKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R0 = 0x00,
    R1 = 0x01,
    R2 = 0x02,
    R3 = 0x03,
    R4 = 0x04,
    R5 = 0x05,
    R6 = 0x06,
    R7 = 0x07,
    R8 = 0x08,
    R9 = 0x09,
    R10 = 0x0A,
    R11 = 0x0B,
    R12 = 0x0C,
    R13 = 0x0D,
    R14 = 0x0E,
    R15 = 0x0F,
    R16 = 0x10,
    R17 = 0x11,
    R18 = 0x12,
    R19 = 0x13,
    R20 = 0x14,
    R21 = 0x15,
    R22 = 0x16,
    R23 = 0x17,
    R24 = 0x18,
    R25 = 0x19,
    R26 = 0x1A,
    R27 = 0x1B,
    R28 = 0x1C,
    R29 = 0x1D,
    R30 = 0x1E,
    R31 = 0x1F,
}

impl TryFrom<ASTOperand> for Register {
    type Error = ASMError;

    fn try_from(operand: ASTOperand) -> Result<Self, Self::Error> {
        use Register::*;

        let immediate = match operand.kind() {
            ASTOperandKind::Immediate(x) => x,
        };

        let register = match immediate {
            0x00 => R0,
            0x01 => R1,
            0x02 => R2,
            0x03 => R3,
            0x04 => R4,
            0x05 => R5,
            0x06 => R6,
            0x07 => R7,
            0x08 => R8,
            0x09 => R9,
            0x0A => R10,
            0x0B => R11,
            0x0C => R12,
            0x0D => R13,
            0x0E => R14,
            0x0F => R15,
            0x10 => R16,
            0x11 => R17,
            0x12 => R18,
            0x13 => R19,
            0x14 => R20,
            0x15 => R21,
            0x16 => R22,
            0x17 => R23,
            0x18 => R24,
            0x19 => R25,
            0x1A => R26,
            0x1B => R27,
            0x1C => R28,
            0x1D => R29,
            0x1E => R30,
            0x1F => R31,
            _ => return Err(ASMError::new(operand.span(), ASMErrorKind::InvalidRegister)),
        };

        Ok(register)
    }
}
