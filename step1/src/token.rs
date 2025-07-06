use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Bool(bool),
    List(Vec<Token>),
    Map(HashMap<String, Token>),
    Nil,
    Number(f64),
    String(String),
    Symbol(String),
    Vector(Vec<Token>),
}

impl Token {
    pub(crate) fn token_type(&self) -> &str {
        match self {
            Token::Bool(_) => "bool",
            Token::List(_) => "list",
            Token::Map(_) => "map",
            Token::Nil => "nil",
            Token::Number(_) => "number",
            Token::String(_) => "string",
            Token::Symbol(_) => "symbol",
            Token::Vector(_) => "vector",
        }
    }
}
