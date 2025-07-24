use thiserror::Error;

use crate::token::Token;

pub type MalRes<T> = Result<T, MalError>;

#[derive(Debug, Error)]
pub enum MalError {
    #[error("Symbol found outside of list")]
    BadSymbolPosition,
    #[error("Mismatched closing bracket {0}")]
    Brace(String),
    #[error("No tokens to parse")]
    Empty,
    #[error("Parse Error. Expected {0} got EOF")]
    Eof(String),
    #[error(transparent)]
    FnError(#[from] FnError),
    #[error("Map keys must be strings. Found {}", .0.token_type())]
    Map(Token),
    #[error("Can't convert to map, odd number of elements")]
    MistmatchKeyValue,
    #[error("No token to peek")]
    Peek,
}

#[derive(Debug, Error)]
pub enum FnError {
    #[error("Argument error. Expected {0} args got {1}")]
    Argument(usize, usize),
    #[error("Only symbols are callable")]
    CallableError,
    #[error("Symbol ({0}) not found")]
    KeyError(String),
    #[error("Type error. Can't {0} {1} and {2}")]
    Type(String, String, String),
    #[error("Attempted to divide by 0")]
    ZeroDivision,
}
