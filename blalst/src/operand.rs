use blex::{Token, TokenKind};
use blib::Span;

use crate::LSTError;

#[derive(Clone, Copy, Debug)]
pub struct LSTOperand {
    span: Span,
    kind: LSTOperandKind,
}

impl LSTOperand {
    pub fn new(span: Span, kind: LSTOperandKind) -> Self {
        Self { span, kind }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> LSTOperandKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LSTOperandKind {
    Immediate,
    Label,
}

impl TryFrom<Token> for LSTOperand {
    type Error = LSTError;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        let span = token.span();

        let kind = match token.kind() {
            TokenKind::Immediate => LSTOperandKind::Immediate,
            TokenKind::Label => LSTOperandKind::Label,
            _ => {
                return Err(LSTError::possible(
                    &[TokenKind::Immediate, TokenKind::Label],
                    span.start(),
                ))
            }
        };

        Ok(LSTOperand::new(span, kind))
    }
}
