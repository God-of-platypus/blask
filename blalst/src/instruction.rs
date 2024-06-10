use blib::Span;

use crate::LSTOperand;

#[derive(Clone, Debug)]
pub struct LSTInstruction {
    span: Span,
    kind: LSTInstructionKind,
    operands: Vec<LSTOperand>,
}

impl LSTInstruction {
    pub fn new(span: Span, kind: LSTInstructionKind, operands: Vec<LSTOperand>) -> Self {
        Self {
            span,
            kind,
            operands,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> LSTInstructionKind {
        self.kind
    }

    pub fn operands(&self) -> Vec<LSTOperand> {
        self.operands.clone()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LSTInstructionKind {
    span: Span,
}

impl LSTInstructionKind {
    pub fn new(span: Span) -> Self {
        Self { span }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
