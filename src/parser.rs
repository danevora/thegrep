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

// Helper functions for building AST's
pub fn alternation(lhs: AST, rhs: AST) -> AST {
    AST::Alternation(Box::new(lhs), Box::new(rhs))
}

pub fn catenation(lhs: AST, rhs: AST) -> AST {
    AST::Catenation(Box::new(lhs), Box::new(rhs))
}

pub fn closure(value: AST) -> AST {
    AST::Closure(Box::new(value))
}

pub fn char(value: char) -> AST {
    AST::Char(value)
}

pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

// Public parse function to establish parse tree
impl<'tokens> Parser<'tokens> {
    pub fn parse(tokenizer: Tokenizer<'tokens>) -> Result<AST, String> {
        let mut parser = Parser {
            tokens: tokenizer.peekable(),
        };
        let mut parse = parser.reg_expr();
        let next = parser.take_next_token();
        match next {
            Ok(token) => Err(format!("Expected end of input, found {:?}", token)),
            Err(e) => parse,
        }
    }
}

//Add Tests here

impl<'tokens> Parser<'tokens> {

    fn reg_expr(&mut self) -> Result<AST, String> {
       // let mut cat;
        if let Some(token) = self.tokens.peek() {
           self.atom()
        } else {
            Err(format!("Unexpected end of input"))
        }
        /*
        let next = self.tokens.peek();
        if let Some(uni) = next {
            match next.unwrap() {
                '|' => self.alt(),
                _ => Ok(cat),
            }
        } else {
            Ok(cat)
        }
        */
    }

    fn cat(&mut self) -> Result<AST, String> {
        let clos = self.clo()?;
        
    }

    fn clo(&mut self) -> Result<AST, String> {
        let at = self.atom()?;
        if let Some(kleene) = self.tokens.peek() {
            match kleene {
                Token::KleeneStar => {
                    self.take_next_token();
                    Ok(closure(at))
                },
                _ => Ok(at),
            }
        } else {
            Ok(at)
        }
    }


    fn atom(&mut self) -> Result<AST, String> {
        let next = self.take_next_token();
        match next {
            Ok(Token::LParen) => {
                let reg_expr = self.reg_expr()?;
                let right_paren = self.consume_token(Token::RParen)?;
                Ok(reg_expr)
            },
            Ok(Token::AnyChar) => {
                Ok(AST::AnyChar)
            },
            Ok(Token::Char(c)) => Ok(AST::Char(c)),
            _ => Err(format!("Unexpected end of input"))
        }
    }
}

impl<'tokens> Parser<'tokens> {

    fn take_next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

    fn consume_token(&mut self, expected: Token) -> Result<Token, String> {
        if let Some(next) = self.tokens.next() {
            if next != expected {
                Err(format!("Expected: {:?} - Found {:?}", expected, next))
            } else {
                Ok(next)
            }
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }
}
