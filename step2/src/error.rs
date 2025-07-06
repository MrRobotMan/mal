use thiserror::Error;

use crate::token::Token;

pub type MalRes<T> = Result<T, MalError>;

#[derive(Debug, Error)]
pub enum MalError {
    #[error("Unknown token {0}")]
    _BadToken(char),
    #[error("Mismatched closing bracket {0}")]
    Brace(String),
    #[error("No tokens to parse")]
    Empty,
    #[error("Parse Error. Expects {0} got EOF")]
    Eof(String),
    #[error("Map keys must be strings. Found {}", .0.token_type())]
    Map(Token),
    #[error("Can't convert to map, odd number of elements")]
    MistmatchKeyValue,
    #[error("No token to peek")]
    Peek,
    #[error("Could not parse {0} as a number")]
    ParseNumError(String),
}
