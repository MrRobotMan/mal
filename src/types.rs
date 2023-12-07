use thiserror::Error;

pub type MalRes = Result<Token, MalError>;

#[derive(Debug, Clone)]
pub enum Token {}

#[derive(Error, Debug)]
pub enum MalError {
    #[error("No token in string")]
    Token,
    #[error("No tokens found")]
    Empty,
    #[error("No token at given index {0}")]
    Index(usize),
    #[error("Unknown token {0}")]
    UnknownToken(char),
}
