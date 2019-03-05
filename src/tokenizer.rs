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
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;

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
            _ => panic!("unknwon register"),
        }
    }
}
