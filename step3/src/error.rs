use thiserror::Error;

use crate::token::Token;

pub type MalRes<T> = Result<T, MalError>;

#[derive(Debug, Error)]
pub enum MalError {
    #[error("Bad function definition. Name should be string, got {}", Token::try_token_type(.0))]
    BadDef(Option<Token>),
    #[error("Symbol found outside of list")]
    BadSymbolPosition,
    #[error("Expected string, got {}", .0.token_type())]
    BadTokenString(Token),
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
    #[error("Can't convert to {0}, odd number of elements")]
    MistmatchKeyValue(String),
    #[error("No token to peek")]
    Peek,
    #[error("Unknown symbol {0}")]
    UnknownSymbol(String),
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
