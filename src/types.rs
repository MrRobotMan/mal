use std::collections::HashMap;

use thiserror::Error;

pub type MalRes<T> = Result<T, MalError>;
pub type MalFunc = fn(Vec<Token>) -> MalRes<Token>;

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
    Func(MalFunc),
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Symbol(l), Token::Symbol(r)) => l == r,
            (Token::Number(l), Token::Number(r)) => l == r,
            (Token::String(l), Token::String(r)) => l == r,
            (Token::List(l), Token::List(r)) => l == r,
            (Token::Vector(l), Token::Vector(r)) => l == r,
            (Token::Map(l), Token::Map(r)) => l == r,
            (Token::Nil, Token::Nil) => true,
            (Token::Bool(l), Token::Bool(r)) => l == r,
            (Token::Func(l), Token::Func(r)) => l == r,
            _ => false,
        }
    }
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
    #[error("Unknown Function {0}")]
    NoReplFunction(String),
}
