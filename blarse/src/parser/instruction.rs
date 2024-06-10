use blalst::{LSTError, LSTInstruction, LSTInstructionKind, LSTNode, LSTNodeKind, LSTOperand};
use blex::{Token, TokenKind};
use blib::Span;

use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn instruction(&mut self, mnemonic: Token) -> Result<LSTNode, LSTError> {
        let start = mnemonic.start();

        let operands = self.operands()?;

        self.lex(TokenKind::Space);

        self.lex(TokenKind::Comment);

        let linefeed = self.try_lex(TokenKind::LineFeed)?;

        let end = linefeed.end();

        let span = Span::new(start, end);

        let kind = LSTInstructionKind::new(mnemonic.span());

        let instruction = LSTInstruction::new(span, kind, operands);

        let kind = LSTNodeKind::Instruction(instruction);

        Ok(LSTNode::new(span, kind))
    }

    fn operands(&mut self) -> Result<Vec<LSTOperand>, LSTError> {
        if self.lex(TokenKind::Space).is_none() {
            return Ok(Vec::new());
        }

        let mut operands = Vec::new();

        if let Some(operand) = self.first_operand()? {
            operands.push(operand);
        } else {
            return Ok(operands);
        }

        while {
            self.lex(TokenKind::Space);
            self.lex(TokenKind::Comma).is_some()
        } {
            self.lex(TokenKind::Space);

            operands.push(self.operand()?);
        }

        Ok(operands)
    }

    fn first_operand(&mut self) -> Result<Option<LSTOperand>, LSTError> {
        self.lex(TokenKind::Immediate)
            .or_else(|| self.lex(TokenKind::Label))
            .map(|token| token.try_into())
            .transpose()
    }

    fn operand(&mut self) -> Result<LSTOperand, LSTError> {
        self.lex(TokenKind::Immediate)
            .or_else(|| self.lex(TokenKind::Label))
            .map(|token| token.try_into())
            .transpose()?
            .ok_or_else(|| {
                LSTError::possible(
                    &[TokenKind::Immediate, TokenKind::Label],
                    self.lexer.index(),
                )
            })
    }
}
