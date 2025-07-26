use std::collections::VecDeque;

use crate::error::MalRes;

type Func = Box<fn(&[Token]) -> MalRes<Token>>;

#[derive(Debug, Clone)]
pub(crate) enum Token {
    Bool(bool),
    Integer(i64),
    List(VecDeque<Token>),
    Map(VecDeque<Token>),
    Nil,
    Real(f64),
    String(String),
    Symbol(String),
    Vector(VecDeque<Token>),
    Func(Func),
}

impl Token {
    pub(crate) fn token_type(&self) -> &'static str {
        match self {
            Self::Bool(_) => "bool",
            Self::Func(_) => "function",
            Self::Integer(_) => "integer",
            Self::List(_) => "list",
            Self::Map(_) => "map",
            Self::Nil => "nil",
            Self::Real(_) => "real",
            Self::String(_) => "string",
            Self::Symbol(_) => "symbol",
            Self::Vector(_) => "vector",
        }
    }

    pub(crate) fn try_token_type(token: &Option<Token>) -> &'static str {
        match token {
            Some(t) => t.token_type(),
            None => "None",
        }
    }

    pub(crate) fn inner_list(&self) -> Option<&VecDeque<Token>> {
        match self {
            Self::List(v) | Self::Vector(v) | Self::Map(v) => Some(v),
            _ => None,
        }
    }
}
