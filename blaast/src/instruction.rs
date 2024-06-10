use std::collections::HashMap;

use blalst::{LSTInstruction, LSTInstructionKind};
use blib::Span;

use crate::{ASTError, ASTErrorKind, ASTOperand};

#[derive(Clone, Debug)]
pub struct ASTInstruction {
    span: Span,
    kind: ASTInstructionKind,
    operands: Vec<ASTOperand>,
}

impl ASTInstruction {
    pub fn new(span: Span, kind: ASTInstructionKind, operands: Vec<ASTOperand>) -> Self {
        Self {
            span,
            kind,
            operands,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> ASTInstructionKind {
        self.kind
    }

    pub fn operands(&self) -> Vec<ASTOperand> {
        self.operands.clone()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASTInstructionKind {
    // R-Type Instructions
    ADD,
    SUB,
    OR,
    AND,
    XOR,
    SLL,
    SRL,
    // I-Type Instructions
    ADDI,
    SUBI,
    ORI,
    ANDI,
    XORI,
    SLLI,
    SRLI,
    // Load/Store Instructions
    LD,
    STR,
    // B-Type Instructions
    BE,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
}

impl TryFrom<(LSTInstruction, &str, &HashMap<String, u16>)> for ASTInstruction {
    type Error = ASTError;

    fn try_from(
        (instruction, src, labels): (LSTInstruction, &str, &HashMap<String, u16>),
    ) -> Result<Self, Self::Error> {
        use core::iter;

        let operands = instruction.operands();
        let operands = operands
            .into_iter()
            .map(|operand| (operand, src, labels).try_into());

        let kind = (instruction.kind(), src).try_into();

        let errors: Vec<_> = iter::once(kind.clone())
            .filter_map(|result| result.err())
            .chain(operands.clone().filter_map(|result| result.err()))
            .collect();

        if errors.is_empty() {
            let operands = operands.filter_map(|result| result.ok()).collect();

            Ok(ASTInstruction::new(instruction.span(), kind?, operands))
        } else {
            let span = instruction.span();

            let kind = ASTErrorKind::ASTErrors(errors);

            Err(ASTError::new(span, kind))
        }
    }
}

impl TryFrom<(LSTInstructionKind, &str)> for ASTInstructionKind {
    type Error = ASTError;

    fn try_from((kind, src): (LSTInstructionKind, &str)) -> Result<Self, Self::Error> {
        use ASTInstructionKind::*;

        let span = kind.span();

        let kind = match &src[span.range()] {
            // R-Type Instructions
            "add" => ADD,
            "sub" => SUB,
            "or" => OR,
            "and" => AND,
            "xor" => XOR,
            "sll" => SLL,
            "srl" => SRL,
            // I-Type Instructions
            "addi" => ADDI,
            "subi" => SUBI,
            "ori" => ORI,
            "andi" => ANDI,
            "xori" => XORI,
            "slli" => SLLI,
            "srli" => SRLI,
            // Load/Store Instructions
            "ld" => LD,
            "str" => STR,
            // B-Type Instructions
            "be" => BE,
            "bne" => BNE,
            "blt" => BLT,
            "bge" => BGE,
            "bltu" => BLTU,
            "bgeu" => BGEU,
            _ => {
                let kind = ASTErrorKind::UnknownInstruction;

                return Err(ASTError::new(span, kind));
            }
        };

        Ok(kind)
    }
}
