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

pub fn character(value: char) -> AST {
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
        //checking to make sure the parser accounted for all tokens in input
        let next = parser.take_next_token();
        match next {
            Ok(token) => Err(format!("Expected end of input, found {:?}", token)),
            Err(e) => parse,
        }
    }
}

//Add Tests here

//helper functions for implementing thegrep grammar
impl<'tokens> Parser<'tokens> {

    //reg_expr calls for a catenation and then checks to see if a union bar is present. If there
    //is, reg_expr is called again and a alternation expression is created with the original expr
    //and the one that was just retrieved with ast_two. If there is no union bar, then just the
    //original expr is returned
    fn reg_expr(&mut self) -> Result<AST, String> {
        let ast = self.cat()?;
        if let Some(token) = self.tokens.peek(){
            match token {
                Token::UnionBar => {
                    self.consume_token(Token::UnionBar);
                    let ast_two = self.reg_expr()?;
                    Ok(alternation(ast, ast_two))
                },
                _ => Ok(ast),
            }
        } else {
            Ok(ast)
        }
        
    }

    //cat calls clo and keeps working along the grammar, the result of calling clo is stored in
    //closure. Next, we check to see if there is anything we need to catenate this value with. If
    //there is any token that signifies a new atom, we call clo again. Otherwise, we simply return
    //the "first" closure
    fn cat(&mut self) -> Result<AST, String> {
        let closure = self.clo()?;
        if let Some(token) = self.tokens.peek() {
            match token {
                Token::UnionBar => Ok(closure),
                Token::LParen | Token::AnyChar | Token::Char(_) => {
                    let closure_two = self.cat()?;
                    Ok(catenation(closure, closure_two))
                },
                _ => Ok(closure),
            }
        } else {
            Ok(closure)
        }
        
    }
    
    //clo calls atom and stores this in a variable. Next, we check to see if this atom has a kleene
    //star after it. If so, we return a closure that enveloped this atom, otherwise, it simply
    //returns the atom
    fn clo(&mut self) -> Result<AST, String> {
        let atom = self.atom()?;
        if let Some(kleene) = self.tokens.peek() {
            match kleene {
                Token::KleeneStar => {
                    self.take_next_token();
                    Ok(closure(atom))
                },
                _ => Ok(atom),
            }
        } else {
            Ok(atom)
        }
    }

    //atom deals with the most basic building blocks of the grammar. If there is a Lparen, we look
    //for the reg_expr inside of it, if there is AnyChar, we return AnyChar, and if there is a char
    //we just return an AST char object enveloping the character. 
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
            Ok(Token::Char(c)) => Ok(character(c)),
            _ => Err(format!("Unexpected end of input"))
        }
    }
}

//helper functions for parsing
impl<'tokens> Parser<'tokens> {
    
    //this functions moves the iterator over the tokens forward and returns the token that was
    //next, or returns an error if this method was called and there were no more tokens
    fn take_next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

    //this function does the same thing as take_next_token except it allows us to pass in an
    //expected value of the next token and if they don't match, returns an error, otherwise it
    //returns the expected token (given it exists)
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
