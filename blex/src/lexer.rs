use core::str::Chars;
use std::iter::Peekable;

use blib::Span;

use crate::{Token, TokenKind};

/// Lexer
///
/// Examples
///
/// Use the iterator in a for loop.
///
/// ```rust
/// use blex::Lexer;
///
/// let input = "token1 token2";
/// let lexer = Lexer::new(input);
///
/// for token in &lexer {
///     // Perform actions based on `token`
/// }
/// ```
/// Use the iterator and peek for tokens.
///
/// ```rust
/// use blex::Lexer;
/// use std::iter::Peekable;
///
/// let input = "add ra, rb, 42";
/// let lexer = Lexer::new(input);
/// let mut it1 = lexer.iter().peekable();
/// let mut it2 = lexer.iter().peekable();
///
/// let token1_1 = it1.peek().clone();
/// let token2_1 = it2.peek().clone();
///
/// assert_eq!(token1_1, token2_1);
///
/// let token1_2 = it1.next();
/// let token2_2 = it2.peek();
///
/// assert_eq!(token1_2, token2_2.map(|t| *t));
/// ```
///
#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    /// Create a new Lexer with a String slice as an input for it.
    ///
    /// Example
    /// ```rust
    /// # use blex::Lexer;
    /// let input = "add, rega, regb, regr";
    ///
    /// let lexer = Lexer::new(input);
    /// ```
    pub fn new(src: &'a str) -> Self {
        Lexer { input: src }
    }

    /// Return the total length of the input.
    ///
    /// Example
    /// ```rust
    /// # use blex::Lexer;
    /// let input = "add rega regb";
    ///
    /// let lexer = Lexer::new(input);
    ///
    /// assert_eq!(lexer.len(), input.len());
    /// ```
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return an iterator over the Lexer.
    pub fn iter(&self) -> LexerIter<'_> {
        LexerIter::new(self)
    }

    fn input(&self) -> &'a str {
        self.input
    }
}

impl<'a> IntoIterator for &'a Lexer<'a> {
    type Item = Token;

    type IntoIter = LexerIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Represents an Iterator over a Lexer.
pub struct LexerIter<'a> {
    index: usize,
    len: usize,
    chars: Peekable<Chars<'a>>,
    token: Option<Option<Token>>,
}

impl<'a> LexerIter<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        LexerIter {
            index: 0,
            len: lexer.len(),
            chars: lexer.input().chars().peekable(),
            token: None,
        }
    }

    /// Indicates of the Iterator has arrived to the end.
    pub fn eof(&self) -> bool {
        self.index == self.len
    }

    /// Indicates the current index in the Lexer input.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Indicates the total length of the iterator.
    pub fn len(&self) -> usize {
        self.len
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.eof() {
            return None;
        }

        let start = self.index();

        let kind = match self.chars.next()? {
            ' ' => self.space(),
            '#' => self.comment(),
            ',' => TokenKind::Comma,
            '-' | '0'..='9' => self.immediate(),
            '@' if matches!(self.chars.peek(), Some('A'..='Z' | 'a'..='z' | '0'..='9')) => {
                self.label()
            }
            '\n' => TokenKind::LineFeed,
            'a'..='z' => self.mnemonic(),
            _ => TokenKind::Unknown,
        };

        // In every cases we need to eat at least 1 item.
        self.index += 1;

        let end = self.index();

        let span = Span::new(start, end);

        Some(Token::new(span, kind))
    }

    fn comment(&mut self) -> TokenKind {
        loop {
            match self.chars.peek() {
                Some('\n') | None => break,
                Some(_) => {
                    self.chars.next();
                    self.index += 1;
                }
            }
        }

        TokenKind::Comment
    }

    fn immediate(&mut self) -> TokenKind {
        while let Some('0'..='9') = self.chars.peek() {
            self.chars.next();
            self.index += 1;
        }

        TokenKind::Immediate
    }

    fn label(&mut self) -> TokenKind {
        while let Some('a'..='z' | 'A'..='Z' | '0'..='9') = self.chars.peek() {
            self.chars.next();
            self.index += 1;
        }

        TokenKind::Label
    }

    fn mnemonic(&mut self) -> TokenKind {
        while let Some('a'..='z') = self.chars.peek() {
            self.chars.next();
            self.index += 1;
        }

        TokenKind::Mnemonic
    }

    fn space(&mut self) -> TokenKind {
        while let Some(' ') = self.chars.peek() {
            self.chars.next();
            self.index += 1;
        }

        TokenKind::Space
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.token = match self.token {
            Some(t) => Some(t),
            None => Some(self.next()),
        };

        self.token.unwrap()
    }
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.token.take() {
            Some(t) => t,
            None => self.next_token(),
        }
    }
}

#[cfg(test)]
mod tests {
    use blib::Span;

    use crate::{Lexer, Token, TokenKind};

    #[test]
    fn empty() {
        // Given
        let line = "";
        let lexer = Lexer::new(line);

        // When
        for _ in &lexer {
            assert!(false); // We should never enter here
        }

        // Then
        assert_eq!(lexer.len(), 0);
    }

    #[test]
    fn space() {
        // Given
        let line = " ";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 1), TokenKind::Space))
        );
        assert_eq!(it.next(), None);
    }

    #[test]
    fn comma() {
        // Given
        let line = ",";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 1), TokenKind::Comma))
        );
        assert_eq!(it.next(), None);
    }

    #[test]
    fn line_feed() {
        // Given
        let line = "\n";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 1), TokenKind::LineFeed))
        );
        assert_eq!(it.next(), None);
    }

    #[test]
    fn immediate() {
        // Given
        let line = "0123456789";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 10), TokenKind::Immediate))
        );
        assert_eq!(it.next(), None);
    }

    #[test]
    fn label() {
        // Given
        let line = "@label";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 6), TokenKind::Label))
        );
        assert_eq!(it.next(), None);
    }

    #[test]
    fn empty_label() {
        // Given
        let line = "@";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 1), TokenKind::Unknown))
        );
        assert_eq!(it.next(), None);
    }

    #[test]
    fn mnemonic() {
        // Given
        let line = "add";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 3), TokenKind::Mnemonic))
        );
        assert_eq!(it.next(), None);
    }

    #[test]
    fn negatif_number() {
        // Given
        let line = "-42";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 3), TokenKind::Immediate))
        );
        assert_eq!(it.next(), None);
    }

    #[test]
    fn comment() {
        // Given
        let line = "#";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 1), TokenKind::Comment))
        );
        assert_eq!(it.next(), None);

        // Given
        let line = "#\n";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 1), TokenKind::Comment))
        );

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(1, 2), TokenKind::LineFeed))
        );

        // Given
        let line = "#bruh\n";
        let lexer = Lexer::new(line);
        let mut it = lexer.iter();

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(0, 5), TokenKind::Comment))
        );

        assert_eq!(
            it.next(),
            Some(Token::new(Span::new(5, 6), TokenKind::LineFeed))
        );
    }
}
