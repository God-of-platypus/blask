//! The crate contains the Lexer structures.
//!
//! The Lexer is used with the input represented as a String.
//! The Lexer will return [Token]s thanks to an iterator.
//!
//! Examples
//!
//! ```rust
//! use blex::{Lexer, Token, TokenKind};
//! use blib::Span;
//!
//! let input = "add rda 42";
//! let lexer = Lexer::new(input);
//!
//! let mut it = lexer.iter();
//!
//! assert_eq!(it.next(), Some(Token::new(Span::new(0, 3), TokenKind::Mnemonic)));
//! assert_eq!(it.next(), Some(Token::new(Span::new(3, 4), TokenKind::Space)));
//! assert_eq!(it.next(), Some(Token::new(Span::new(4, 7), TokenKind::Mnemonic)));
//! assert_eq!(it.next(), Some(Token::new(Span::new(7, 8), TokenKind::Space)));
//! assert_eq!(it.next(), Some(Token::new(Span::new(8, 10), TokenKind::Immediate)));
//! assert_eq!(it.next(), None);
//!
//! ```

mod lexer;
mod token;

pub use lexer::{Lexer, LexerIter};
pub use token::{Token, TokenKind};
