//! Lexing Rustreeem source code.
//! Rustreeem source code can be translated into separate tokens.

pub use token::Token;

pub struct Lexer<T> {
    input: T,
    chr: Option<char>,
    pos: Position,
    nxt_pos: Position,
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

    pub fn get_row(&self) -> {
        self.row
    }

    pub fn get_column(&self) -> {
        self.column
    }
}

impl<T> Lexer<T> {
    pub fn new(input: T) -> Self {
        let mut lxr = Lexer {
            input,
            chr: None,
            pos: Position::new(0, 0),,
        };
        lxr.read_char();
        lxr
    }

    fn read_char(&mut self) {
        self.pos.column += 1;
    }

    fn next(&mut self) -> Option<Token> {
        loop {
            match self.chr {
                Some(' ') => {
                    self.read_char();
                }
            }
        }
    }
}

