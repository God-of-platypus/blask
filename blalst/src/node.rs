use blib::Span;

use crate::LSTInstruction;

#[derive(Clone, Debug)]
pub struct LSTNode {
    span: Span,
    kind: LSTNodeKind,
}

impl LSTNode {
    pub fn new(span: Span, kind: LSTNodeKind) -> Self {
        Self { span, kind }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> LSTNodeKind {
        self.kind.clone()
    }
}

#[derive(Clone, Debug)]
pub enum LSTNodeKind {
    EmptyLine,
    Instruction(LSTInstruction),
    Label,
}
