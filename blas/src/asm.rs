use blaast::{ASTBuilder, ASTNodeKind};
use blex::Lexer;

use crate::{ASMError, PseudoInstruction};

pub struct ASM<'a> {
    ast: ASTBuilder<'a>,
}

impl<'a> ASM<'a> {
    pub fn new(src: &'a str, lexer: &'a Lexer) -> Self {
        Self {
            ast: ASTBuilder::new(src, lexer),
        }
    }
}

impl<'a> Iterator for ASM<'a> {
    type Item = Result<PseudoInstruction, ASMError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ast.next()? {
            Ok(node) => match node.kind() {
                ASTNodeKind::Instruction(instruction) => {
                    return Some(instruction.try_into());
                }
            },
            Err(err) => return Some(Err(err.into())),
        }
    }
}
