use std::collections::HashMap;

use blalst::LSTNodeKind;
use blarse::Parser;
use blex::Lexer;

use crate::{ASTError, ASTErrorKind, ASTNode};

pub struct ASTBuilder<'a> {
    src: &'a str,
    parser: Parser<'a>,
    address: u16,
    labels: HashMap<String, u16>,
}

impl<'a> ASTBuilder<'a> {
    pub fn new(src: &'a str, lexer: &'a Lexer) -> Self {
        Self {
            src,
            parser: Parser::new(lexer),
            labels: HashMap::new(),
            address: 0,
        }
    }
}

impl<'a> Iterator for ASTBuilder<'a> {
    type Item = Result<ASTNode, ASTError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let node = match self.parser.next()? {
                Ok(node) => node,
                Err(err) => return Some(Err(err.into())),
            };

            match node.kind() {
                LSTNodeKind::EmptyLine => (),
                LSTNodeKind::Label => {
                    let span = node.span();
                    let start = span.start();
                    let end = span.end();
                    let label = &self.src[start..end];

                    if self.labels.contains_key(label) {
                        return Some(Err(ASTError::new(span, ASTErrorKind::DuplicateLabel)));
                    }

                    self.labels.insert(label.to_string(), self.address);
                }
                LSTNodeKind::Instruction(_) => {
                    self.address += 1;

                    return Some((node, self.src, &self.labels).try_into());
                }
            }
        }
    }
}
