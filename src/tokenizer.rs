use std::iter::Peekable;
use std::str::Chars;

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
        Tokenizer { chars: input.chars().peekable(), }
    }
}

impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.lex_whitespace();
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

impl<'str> Tokenizer<'str> {
    fn lex_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\n' => self.chars.next(),
                _ => break,
            };
        }
    }

    fn lex_paren(&mut self) {
        let c = self.chars.next().unwrap();
        match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => panic!("unknown register"),
        }
    }

    fn lex_union(&mut self) {
        let c = self.chars.next().unwrap();
        match c {
            '|' => Token::UnionBar,
            _ => panic!("unknown register"),
        }
    }

    fn lex_kleene(&mut self) {
        let c = self.chars.next().unwrap();
        match c {
            '*' => Token::KleeneStar,
            _ => panic!("unknown register"),
        }
    }

    fn lex_anychar(&mut self) {
        let c = self.chars.next().unwrap();
        match c {
            '.' => Token::AnyChar,
            _ => panic!("unknown register"),
        }
    }

    fn lex_char(&mut self) {
        let c = self.chars.next().unwrap();
        match c {
            !('(' | ')' | '|' | '.' | '*') => Token::Char(c),
            _ => panic!("unknwon register"),
    }
}
