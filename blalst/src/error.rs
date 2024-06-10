use blex::{Token, TokenKind};

use blib::Span;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSTError {
    span: Span,
    kind: LSTErrorKind,
}

impl LSTError {
    pub fn new(span: Span, kind: LSTErrorKind) -> Self {
        Self { span, kind }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> LSTErrorKind {
        self.kind
    }
}

impl LSTError {
    pub fn expected(kind: TokenKind, position: usize) -> Self {
        let span = Span::new(position, position + 1);

        let kind = LSTErrorKind::ExpectedToken(kind);

        Self::new(span, kind)
    }

    pub fn unexpected(token: Token) -> Self {
        let span = token.span();

        let kind = LSTErrorKind::UnexpectedToken(token.kind());

        Self::new(span, kind)
    }

    pub fn possible(kinds: &'static [TokenKind], position: usize) -> Self {
        let span = Span::new(position, position + 1);

        let kind = LSTErrorKind::PossibleTokens(kinds);

        Self::new(span, kind)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSTErrorKind {
    UnknownToken,
    ExpectedToken(TokenKind),
    UnexpectedToken(TokenKind),
    PossibleTokens(&'static [TokenKind]),
}
