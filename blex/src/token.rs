use blib::Span;

/// Token that are created by the Lexer.
///
/// A Token is defined by 2 elements:
/// - Span: The range identifying where the Token is in the input.
/// - Kind: Contains the [TokenKind] of the Token.
///
/// Example
/// ```rust
/// use blib::Span;
/// use blex::{Token, TokenKind};
///
/// // Asume we have the input: "42"
/// // The span indicates that the token starts at index 0 and ends at index 2.
/// let span = Span::new(0, 2);
///
/// // The token is an immediate because it's composed of digits.
/// let kind = TokenKind::Immediate;
///
/// // Finaly, we assemble the Token with the Span and the TokenKind.
/// let token = Token::new(span, kind);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    span: Span,
    kind: TokenKind,
}

impl Token {
    /// Create a new token.
    ///
    /// Example
    /// ```rust
    /// # use blib::Span;
    /// # use blex::{Token, TokenKind};
    /// let token = Token::new(Span::new(4, 6), TokenKind::Mnemonic);
    /// ```
    pub fn new(span: Span, kind: TokenKind) -> Self {
        Self { span, kind }
    }

    /// Return the [Span] of this Token.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Return the [TokenKind] of the Token.
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Return the start of the [Span] of this token.
    pub fn start(&self) -> usize {
        self.span.start()
    }

    /// Return the end of the [Span] of this token.
    pub fn end(&self) -> usize {
        self.span.end()
    }
}

/// Define all the Token Kind available for the Lexer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// | ','
    Comma,
    /// | '#'
    Comment,
    /// | ('-')? ('0' .. '9')+
    Immediate,
    /// | '@' ('0' .. '9' | 'a' .. 'z' | 'A' .. 'Z')+
    Label,
    /// | '\n'
    LineFeed,
    /// | ('a' .. 'z')+
    Mnemonic,
    /// | ' '
    Space,
    Unknown,
}
