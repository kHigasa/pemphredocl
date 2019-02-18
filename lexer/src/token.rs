//! Rustreeem token definitions.
use num_bigint::BigInt;

//! Rustreeem source code can be tokenized in a sequence of these tokens
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Int(BigInt),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,

    PipeStream,
    Rarrow,

    // Keywords (alphabetically)
    Lambda,
}

