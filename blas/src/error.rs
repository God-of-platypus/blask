use blaast::{ASTError, ASTErrorKind};
use blib::Span;

#[derive(Clone, Debug, PartialEq)]
pub struct ASMError {
    span: Span,
    kind: ASMErrorKind,
}

impl ASMError {
    pub fn new(span: Span, kind: ASMErrorKind) -> Self {
        Self { span, kind }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> ASMErrorKind {
        self.kind.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASMErrorKind {
    ASTError(ASTErrorKind),
    InvalidRegister,
}

impl From<ASTError> for ASMError {
    fn from(error: ASTError) -> Self {
        ASMError::new(error.span(), ASMErrorKind::ASTError(error.kind()))
    }
}
