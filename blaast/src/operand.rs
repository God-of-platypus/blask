use std::collections::HashMap;

use blalst::{LSTOperand, LSTOperandKind};
use blib::Span;

use crate::{ASTError, ASTErrorKind};

#[derive(Clone, Copy, Debug)]
pub struct ASTOperand {
    span: Span,
    kind: ASTOperandKind,
}

impl ASTOperand {
    pub fn new(span: Span, kind: ASTOperandKind) -> Self {
        Self { span, kind }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> ASTOperandKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASTOperandKind {
    Immediate(u16),
}

impl TryFrom<(LSTOperand, &str, &HashMap<String, u16>)> for ASTOperand {
    type Error = ASTError;

    fn try_from(
        (operand, src, labels): (LSTOperand, &str, &HashMap<String, u16>),
    ) -> Result<Self, Self::Error> {
        let kind = operand.kind();
        let span = operand.span();

        let min = i16::MIN as i32;
        let max = u16::MAX as i32;
        let range = min..max;

        match kind {
            LSTOperandKind::Immediate => match src[span.range()].parse() {
                Ok(immediate) if range.contains(&immediate) => {
                    let kind = ASTOperandKind::Immediate(immediate as u16);
                    let operand = ASTOperand::new(span, kind);

                    Ok(operand)
                }
                _ => Err(ASTError::new(span, ASTErrorKind::BadImmediate)),
            },
            LSTOperandKind::Label => {
                let label = &src[span.range()];
                let kind = ASTOperandKind::Immediate(*labels.get(label).unwrap());
                let operand = ASTOperand::new(span, kind);

                Ok(operand)
            }
        }
    }
}

impl From<ASTOperand> for u16 {
    fn from(operand: ASTOperand) -> Self {
        match operand.kind {
            ASTOperandKind::Immediate(immediate) => immediate,
        }
    }
}

impl From<ASTOperand> for i16 {
    fn from(operand: ASTOperand) -> Self {
        match operand.kind {
            ASTOperandKind::Immediate(immediate) => immediate as i16,
        }
    }
}
