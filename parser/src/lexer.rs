//! Lexing Rustreeem source code.
//! Rustreeem source code can be translated into separate tokens.

pub use super::token::Tok;
use num_bigint::BigInt;
use std::collections::HashMap;

pub type Spanned<Tok> = Result<(Loc, Tok, Loc), LexicalError>;

#[derive(Debug)]
pub enum LexicalError {
    StringError,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Loc {
    row: usize,
    column: usize,
}

impl Loc {
    pub fn new(row: usize, column: usize) -> Self {
        Loc { row, column }
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}

pub fn get_keywords() -> HashMap<String, Tok> {
    let mut keywords: HashMap<String, Tok> = HashMap::new();
    keywords.insert(String::from("lambda"), Tok::Lambda);
    keywords
}

pub struct Lexer<T: Iterator<Item = char>> {
    chars: T,
    chr0: Option<char>,
    chr1: Option<char>,
    location: Loc,
}

impl<T> Lexer<T> where T: Iterator<Item = char> {
    pub fn new(input: T) -> Self {
        let mut lxr = Lexer {
            chars: input,
            chr0: None,
            chr1: None,
            location: Loc::new(0, 0),
        };
        lxr.next_char();
        lxr.next_char();
        // Start at top row (=1) left column (=1).
        lxr.location.row = 1;
        lxr.location.column = 1;
        lxr
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chr0;
        self.chr0 = self.chr1;
        self.chr1 = self.chars.next();
        self.location.column += 1;
        c
    }

    fn get_loc(&self) -> Loc {
        self.location.clone()
    }

    // Check functions:
    fn is_char(&self) -> bool {
        match self.chr0 {
            Some('a'..='z') | Some('A'..='Z') | Some('_') | Some('0'..='9') => true,
            _ => false,
        }
    }

    fn is_number(&self, radix: u32) -> bool {
        match radix {
            10 => match self.chr0 {
                Some('0'..='9') => true,
                _ => false,
            },
            x => unimplemented!("Radix not implemented: {}", x),
        }
    }

    // Lexer helper functions:
    fn lex_identifier(&mut self) -> Spanned<Tok> {
        let mut ident = String::new();
        let tok_start = self.get_loc();
        while self.is_char() {
            ident.push(self.next_char().unwrap());
        }
        let tok_end = self.get_loc();
        let mut keywords = get_keywords();
        if keywords.contains_key(&ident) {
            Ok((tok_start, keywords.remove(&ident).unwrap(), tok_end))
        } else {
            Ok((tok_start, Tok::Ident(String::from(ident)), tok_end))
        }
    }

    fn lex_number(&mut self) -> Spanned<Tok> {
        let mut value_str = String::new();
        let tok_start = self.get_loc();
        while self.is_number(10) {
            value_str.push(self.next_char().unwrap());
        }
        let tok_end = self.get_loc();
        Ok((tok_start, Tok::Int(value_str.parse::<BigInt>().unwrap()), tok_end))
    }
}

impl<T> Iterator for Lexer<T> where T: Iterator<Item = char> {
    type Item = Spanned<Tok>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chr0 {
                Some('0'..='9') => return Some(self.lex_number()),
                Some('_') | Some('a'..='z') | Some('A'..='Z') => return Some(self.lex_identifier()),
                Some('+') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::PlusEqual, tok_end)));
                        }
                        _ => {
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Plus, tok_end)));
                        }
                    }
                }
                Some('-') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::MinusEqual, tok_end)));
                        }
                        Some('>') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Rarrow, tok_end)));
                        }
                        _ => {
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Minus, tok_end)));
                        }
                    }
                }
                Some('*') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::StarEqual, tok_end)));
                        }
                        _ => {
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Star, tok_end)));
                        }
                    }
                }
                Some('/') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::SlashEqual, tok_end)));
                        }
                        _ => {
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Slash, tok_end)));
                        }
                    }
                }
                Some('|') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('>') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Rarrow, tok_end)));
                        }
                        _ => {
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Vbar, tok_end)));
                        }
                    }
                }
                None => return None,
                _ => {
                    let c = self.next_char();
                    panic!("Not impl {:?}", c)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Tok};
    use num_bigint::BigInt;
    use std::iter::FromIterator;
    use std::iter::Iterator;

    pub fn lex_source(source: &String) -> Vec<Tok> {
        let lexer = Lexer::new(source.chars());
        Vec::from_iter(lexer.map(|x| x.unwrap().1))
    }

    #[test]
    fn test_operators() {
        let source = String::from("++=*/*=-->3");
        let tokens = lex_source(&source);
        assert_eq!(tokens, vec![
                   Tok::Plus,
                   Tok::PlusEqual,
                   Tok::Star,
                   Tok::Slash,
                   Tok::StarEqual,
                   Tok::Minus,
                   Tok::Rarrow,
                   Tok::Int(BigInt::from(3)),
        ]);
    }
}

