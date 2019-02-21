//! Lexing Rustreeem source code.
//! Rustreeem source code can be translated into separate tokens.

pub use super::token::Token;

pub struct Lexer<T: Iterator<Item = char>> {
    input: T,
    cur: Option<char>,
    nxt: Option<char>,
    pos: Position,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Self {
        Position { row, column }
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}

impl<T> Lexer<T> where T: Iterator<Item = char> {
    pub fn new(input: T) -> Self {
        let mut lxr = Lexer {
            input,
            cur: None,
            nxt: None,
            pos: Position::new(0, 0),
        };
        lxr.read_char();
        lxr.read_char();
        // Start at top row (=1) left column (=1)
        lxr.pos.row = 1;
        lxr.pos.column = 1;
        lxr
    }

    fn read_char(&mut self) {
        self.cur = self.nxt;
        self.nxt = self.input.next();
        self.pos.column += 1;
    }

    //fn next(&mut self) -> Option<Token> {
    //    loop {
    //        match self.cur {
    //            Some(' ') => {
    //                self.read_char();
    //            }
    //        }
    //    }
    //}
}

