use std::collections::HashMap;

use thiserror::Error;

pub type MalRes<T> = Result<T, MalError>;

pub const SYMBOLS: &str = "+-/*";

#[derive(Debug, Clone)]
pub enum Token {
    Symbol(String),
    Number(f64),
    String(String),
    List(Vec<Token>),
    Vector(Vec<Token>),
    Map(HashMap<String, Token>),
    Nil,
    Bool(bool),
}

#[derive(Error, Debug)]
pub enum MalError {
    #[error("No token in string")]
    Token,
    #[error("No tokens found")]
    Empty,
    #[error("No token at given index")]
    Index,
    #[error("Unknown token {0}")]
    UnknownToken(String),
    #[error("Mismatched brace {0}")]
    Brace(String),
    #[error("Parse error. Expected {0} got EOF")]
    Eof(String),
    #[error("Can't convert to map, odd number of elements")]
    Map,
    #[error("Map keys must be strings, not {0:?}")]
    MapKey(Token),
}
