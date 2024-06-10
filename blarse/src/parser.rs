mod instruction;

use blalst::{LSTError, LSTNode, LSTNodeKind};
use blex::{Lexer, LexerIter, Token, TokenKind};

pub struct Parser<'a> {
    lexer: LexerIter<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        Self {
            lexer: lexer.iter(),
        }
    }

    fn bump(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    fn peek(&mut self) -> Option<Token> {
        self.lexer.peek()
    }

    fn lex(&mut self, kind: TokenKind) -> Option<Token> {
        let token = self.peek()?;

        if token.kind() == kind {
            self.bump()
        } else {
            None
        }
    }

    fn try_lex(&mut self, kind: TokenKind) -> Result<Token, LSTError> {
        match self.bump() {
            None => Err(LSTError::expected(kind, self.lexer.len())),
            Some(token) if token.kind() == kind => Ok(token),
            Some(token) => Err(LSTError::expected(kind, token.span().start())),
        }
    }
}

impl<'a> Parser<'a> {
    fn empty_line(&mut self, token: Token) -> Result<LSTNode, LSTError> {
        let kind = LSTNodeKind::EmptyLine;

        Ok(LSTNode::new(token.span(), kind))
    }

    fn label(&mut self, token: Token) -> Result<LSTNode, LSTError> {
        let kind = LSTNodeKind::Label;

        Ok(LSTNode::new(token.span(), kind))
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<LSTNode, LSTError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex(TokenKind::Immediate);

        let token = self.bump()?;

        let node = match token.kind() {
            TokenKind::Label => self.label(token),
            TokenKind::LineFeed | TokenKind::Comment => self.empty_line(token),
            TokenKind::Mnemonic => self.instruction(token),
            _ => Err(LSTError::unexpected(token)),
        };

        Some(node)
    }
}
