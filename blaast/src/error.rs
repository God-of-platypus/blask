use blalst::{LSTError, LSTErrorKind};
use blib::Span;

#[derive(Clone, Debug, PartialEq)]
pub struct ASTError {
    span: Span,
    kind: ASTErrorKind,
}

impl ASTError {
    pub fn new(span: Span, kind: ASTErrorKind) -> Self {
        Self { span, kind }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> ASTErrorKind {
        self.kind.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTErrorKind {
    BadImmediate,
    UnknownInstruction,
    DuplicateLabel,
    LSTError(LSTErrorKind),
    ASTErrors(Vec<ASTError>),
}

impl From<LSTError> for ASTError {
    fn from(error: LSTError) -> Self {
        ASTError::new(error.span(), ASTErrorKind::LSTError(error.kind()))
    }
}
