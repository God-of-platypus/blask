use std::collections::HashMap;

use blalst::{LSTNode, LSTNodeKind};
use blib::Span;

use crate::{ASTError, ASTInstruction};

#[derive(Clone, Debug)]
pub struct ASTNode {
    span: Span,
    kind: ASTNodeKind,
}

impl ASTNode {
    pub fn new(span: Span, kind: ASTNodeKind) -> Self {
        Self { span, kind }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn kind(&self) -> ASTNodeKind {
        self.kind.clone()
    }
}

#[derive(Clone, Debug)]
pub enum ASTNodeKind {
    Instruction(ASTInstruction),
}

impl TryFrom<(LSTNode, &str, &HashMap<String, u16>)> for ASTNode {
    type Error = ASTError;

    fn try_from(
        (node, src, labels): (LSTNode, &str, &HashMap<String, u16>),
    ) -> Result<Self, Self::Error> {
        Ok(ASTNode::new(
            node.span(),
            (node.kind(), src, labels).try_into()?,
        ))
    }
}
impl TryFrom<(LSTNodeKind, &str, &HashMap<String, u16>)> for ASTNodeKind {
    type Error = ASTError;

    fn try_from(
        (kind, src, labels): (LSTNodeKind, &str, &HashMap<String, u16>),
    ) -> Result<Self, Self::Error> {
        match kind {
            LSTNodeKind::Instruction(instruction) => Ok(ASTNodeKind::Instruction(
                (instruction, src, labels).try_into()?,
            )),
            _ => unreachable!(),
        }
    }
}
