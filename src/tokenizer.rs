use std::iter::Peekable;
use std::str::Chars;

/**
 * Token types for 'thegrep' are defined below
 */
#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    UnionBar,
    KleeneStar,
    AnyChar,
    Char(char),
}

pub struct Tokenizer<'str> {
    chars: Peekable<Chars<'str>>,
}

impl<'str> Tokenizer<'str> {
    pub fn new(input: &'str str) -> Tokenizer {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;
    /**
     * The 'next' method returns the next
     * complete token in the Tokenizer's
     * input string or None at all
     */
    fn next(&mut self) -> Option<Token> {
        if let Some(c) = self.chars.peek() {
            Some(match c {
                '(' | ')' => self.lex_paren(),
                '|' => self.lex_union(),
                '*' => self.lex_kleene(),
                '.' => self.lex_anychar(),
                _ => self.lex_char(),
            })
        } else {
            None
        }
    }
}

/**
 * Unit tests for 'next' method
 */

#[cfg(test)]
mod iterator {
    use super::*;

    #[test]
    fn empty() {
        let mut tokens = Tokenizer::new("");
        assert_eq!(tokens.next(), None);
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn single_char() {
        let mut tokens = Tokenizer::new("a");
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn parens() {
        let mut tokens = Tokenizer::new("(a)");
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), Some(Token::RParen));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn any_char() {
        let mut tokens = Tokenizer::new(".");
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn union_bar() {
        let mut tokens = Tokenizer::new("a|b");
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), Some(Token::Char('b')));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn kleene_star() {
        let mut tokens = Tokenizer::new("a*");
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn whitespace() {
        let mut tokens = Tokenizer::new("\n\t ");
        assert_eq!(tokens.next(), Some(Token::Char('\n')));
        assert_eq!(tokens.next(), Some(Token::Char('\t')));
        assert_eq!(tokens.next(), Some(Token::Char(' ')));
        assert_eq!(tokens.next(), None);
    }
}

/**
 * Helper methods for each token
 * type are defined below
 */

impl<'str> Tokenizer<'str> {
    fn lex_paren(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => panic!("unknown register"),
        }
    }

    fn lex_union(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '|' => Token::UnionBar,
            _ => panic!("unknown register"),
        }
    }

    fn lex_kleene(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '*' => Token::KleeneStar,
            _ => panic!("unknown register"),
        }
    }

    fn lex_anychar(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '.' => Token::AnyChar,
            _ => panic!("unknown register"),
        }
    }

    fn lex_char(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            c => Token::Char(c),
        }
    }
}

/**
 * Tests for helper methods
 */

#[cfg(test)]
mod helper_method {
    use super::*;

    #[test]
    fn lex_paren() {
        let mut tokens = Tokenizer::new(")");
        assert_eq!(tokens.lex_paren(), Token::RParen);
        assert_eq!(tokens.chars.next(), None);
    }

    #[test]
    fn lex_union() {
        let mut tokens = Tokenizer::new("|");
        assert_eq!(tokens.lex_union(), Token::UnionBar);
        assert_eq!(tokens.chars.next(), None);
    }

    #[test]
    fn lex_kleene() {
        let mut tokens = Tokenizer::new("*");
        assert_eq!(tokens.lex_kleene(), Token::KleeneStar);
        assert_eq!(tokens.chars.next(), None);
    }

    #[test]
    fn lex_anychar() {
        let mut tokens = Tokenizer::new(".");
        assert_eq!(tokens.lex_anychar(), Token::AnyChar);
        assert_eq!(tokens.chars.next(), None);
    }

    #[test]
    fn lex_char() {
        let mut tokens = Tokenizer::new("a");
        assert_eq!(tokens.lex_char(), Token::Char('a'));
        assert_eq!(tokens.chars.next(), None);
    }
}
