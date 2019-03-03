//! Rustreeem token definitions.
//! Rustreeem source code can be tokenized in a sequence of these tokens

use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Ident(String),
    Int(BigInt),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
}

