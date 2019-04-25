use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

// Enum for Abstract Syntax Tree
#[derive(Debug, PartialEq)]
pub enum AST {
    Alternation(Box<AST>, Box<AST>),
    Catenation(Box<AST>, Box<AST>),
    Closure(Box<AST>),
    OneOrMore(Box<AST>),
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

pub fn one_or_more(value: AST) -> AST {
    AST::OneOrMore(Box::new(value))
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
        let parse = parser.reg_expr();
        //checking to make sure the parser accounted for all tokens in input
        let next = parser.take_next_token();
        match next {
            Ok(token) => Err(format!("Expected end of input, found {:?}", token)),
            Err(_) => parse,
        }
    }
}

//Add Tests here

#[cfg(test)]
mod public_api {
    use super::*;

    #[test]
    fn parse_atoms() {
        let atom_char = Parser::parse(Tokenizer::new("a")).unwrap();
        assert_eq!(character('a'), atom_char);
        let atom_any_char = Parser::parse(Tokenizer::new(".")).unwrap();
        assert_eq!(AST::AnyChar, atom_any_char);
        let atom_char_paren = Parser::parse(Tokenizer::new("(a)")).unwrap();
        assert_eq!(character('a'), atom_char_paren);
    }

    #[test]
    fn parse_clo() {
        let clo_char = Parser::parse(Tokenizer::new("a*")).unwrap();
        assert_eq!(closure(character('a')), clo_char);
        let clo_any_char = Parser::parse(Tokenizer::new(".*")).unwrap();
        assert_eq!(closure(AST::AnyChar), clo_any_char);
        let clo_char_paren = Parser::parse(Tokenizer::new("(a)*")).unwrap();
        assert_eq!(closure(character('a')), clo_char_paren);
        let no_clo = Parser::parse(Tokenizer::new("a")).unwrap();
        assert_eq!(character('a'), no_clo);
    }

    #[test]
    fn parse_cat() {
        let no_cat = Parser::parse(Tokenizer::new("a")).unwrap();
        assert_eq!(character('a'), no_cat);
        let cat_atoms = Parser::parse(Tokenizer::new("ab")).unwrap();
        assert_eq!(catenation(character('a'), character('b')), cat_atoms);
        let cat_clo = Parser::parse(Tokenizer::new(".b*")).unwrap();
        assert_eq!(catenation(AST::AnyChar, closure(character('b'))), cat_clo);
        let cat_clo_paren = Parser::parse(Tokenizer::new("(ab)*")).unwrap();
        assert_eq!(
            closure(catenation(character('a'), character('b'))),
            cat_clo_paren
        );
        let cat_mult = Parser::parse(Tokenizer::new("abc")).unwrap();
        assert_eq!(
            catenation(character('a'), catenation(character('b'), character('c'))),
            cat_mult
        );
    }

    #[test]
    fn parse_reg_expr() {
        let alt_atoms = Parser::parse(Tokenizer::new("a|b")).unwrap();
        assert_eq!(alternation(character('a'), character('b')), alt_atoms);
        let alt_clo = Parser::parse(Tokenizer::new("a*|b*")).unwrap();
        assert_eq!(
            alternation(closure(character('a')), closure(character('b'))),
            alt_clo
        );
        let alt_cat = Parser::parse(Tokenizer::new("ab|cd")).unwrap();
        assert_eq!(
            alternation(
                catenation(character('a'), character('b')),
                catenation(character('c'), character('d'))
            ),
            alt_cat
        );
        let alt_everything = Parser::parse(Tokenizer::new("((ab)*c)|(.a(b|c)*)")).unwrap();
        assert_eq!(
            alternation(
                catenation(
                    closure(catenation(character('a'), character('b'))),
                    character('c')
                ),
                catenation(
                    AST::AnyChar,
                    catenation(
                        character('a'),
                        closure(alternation(character('b'), character('c')))
                    )
                )
            ),
            alt_everything
        );
        let no_alt = Parser::parse(Tokenizer::new("a")).unwrap();
        assert_eq!(character('a'), no_alt);
    }

    #[test]
    fn reg_expr() {
        assert_eq!(Parser::from("a").reg_expr().unwrap(), character('a'));
        assert_eq!(
            Parser::from("a|b").reg_expr().unwrap(),
            alternation(character('a'), character('b'))
        );
        assert_eq!(
            Parser::from("a*|b*").reg_expr().unwrap(),
            alternation(closure(character('a')), closure(character('b')))
        );
        assert_eq!(
            Parser::from("ab|cd").reg_expr().unwrap(),
            alternation(
                catenation(character('a'), character('b')),
                catenation(character('c'), character('d'))
            )
        );
        assert_eq!(
            Parser::from("((ab)*c)|(.a(b|c)*)").reg_expr().unwrap(),
            alternation(
                catenation(
                    closure(catenation(character('a'), character('b'))),
                    character('c')
                ),
                catenation(
                    AST::AnyChar,
                    catenation(
                        character('a'),
                        closure(alternation(character('b'), character('c')))
                    )
                )
            )
        );
    }

    #[test]
    fn cat() {
        assert_eq!(
            Parser::from("ab").cat().unwrap(),
            catenation(character('a'), character('b'))
        );
        assert_eq!(
            Parser::from(".a*").cat().unwrap(),
            catenation(AST::AnyChar, closure(character('a')))
        );
        assert_eq!(
            Parser::from("(ab)*").cat().unwrap(),
            closure(catenation(character('a'), character('b')))
        );
        assert_eq!(
            Parser::from("abc").cat().unwrap(),
            catenation(character('a'), catenation(character('b'), character('c')))
        );
        assert_eq!(Parser::from("a").cat().unwrap(), character('a'));
    }

    #[test]
    fn clo() {
        assert_eq!(Parser::from("a*").clo().unwrap(), closure(character('a')));
        assert_eq!(Parser::from(".*").clo().unwrap(), closure(AST::AnyChar));
        assert_eq!(Parser::from("(a)*").clo().unwrap(), closure(character('a')));
        assert_eq!(Parser::from("a").clo().unwrap(), character('a'));
    }

    #[test]
    fn atom() {
        assert_eq!(Parser::from("a").atom().unwrap(), character('a'));
        assert_eq!(Parser::from(".").atom().unwrap(), AST::AnyChar);
        assert_eq!(Parser::from("(a)").atom().unwrap(), character('a'));
    }

}

//helper functions for implementing thegrep grammar
impl<'tokens> Parser<'tokens> {
    //reg_expr calls for a catenation and then checks to see if a union bar is present. If there
    //is, reg_expr is called again and a alternation expression is created with the original expr
    //and the one that was just retrieved with ast_two. If there is no union bar, then just the
    //original expr is returned
    fn reg_expr(&mut self) -> Result<AST, String> {
        let ast = self.cat()?;
        if let Some(token) = self.tokens.peek() {
            match token {
                Token::UnionBar => {
                    self.consume_token(Token::UnionBar)?;
                    let ast_two = self.reg_expr()?;
                    Ok(alternation(ast, ast_two))
                }
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
                }
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
        let atom = self.plus()?;
        if let Some(kleene) = self.tokens.peek() {
            match kleene {
                Token::KleeneStar => {
                    self.take_next_token()?;
                    Ok(closure(atom))
                }
                _ => Ok(atom),
            }
        } else {
            Ok(atom)
        }
    }

    fn plus(&mut self) -> Result<AST, String> {
        let atom = self.atom()?;
        if let Some(plus) = self.tokens.peek() {
            match plus {
                Token::KleenePlus => {
                    self.take_next_token()?;
                    Ok(one_or_more(atom))
                }
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
                self.consume_token(Token::RParen)?;
                Ok(reg_expr)
            }
            Ok(Token::AnyChar) => Ok(AST::AnyChar),
            Ok(Token::Char(c)) => Ok(character(c)),
            _ => Err(format!("Unexpected end of input")),
        }
    }
}

//helper functions for parsing
impl<'tokens> Parser<'tokens> {

    //helper method for constructing parsers in unit tests
    fn from(input: &'tokens str) -> Parser<'tokens> {
        Parser {
            tokens: Tokenizer::new(input).peekable(),
        }
    }
    
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
