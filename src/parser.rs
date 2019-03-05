use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

// Enum for Abstract Syntax Tree
#[derive(Debug, PartialEq)]
pub enum AST {
    Alternation(Box<AST>, Box<AST>),
    Catenation(Box<AST>, Box<AST>),
    Closure(Box<AST>),
    Char(char),
    AnyChar,
}

pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    pub fn parse(tokenizer: Tokenizer<'tokens>) -> Result<Expr, String> {
        let mut parser = Parser {
            tokens: tokenizer.peekable(),
        };
    }
}
