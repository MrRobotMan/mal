use std::collections::HashMap;

use crate::{MalFunc, MalRes, Token};

pub trait Env {
    fn get(&self, symbol: &str) -> MalRes<Token>;
}

pub struct ReplEnv {
    functions: HashMap<String, MalFunc>,
}

impl Env for ReplEnv {
    fn get(&self, symbol: &str) -> MalRes<Token> {
        match self.functions.get(symbol) {
            Some(f) => Ok(Token::Func(*f)),
            None => Err(crate::MalError::NoReplFunction(symbol.to_string())),
        }
    }
}

impl Default for ReplEnv {
    fn default() -> Self {
        Self {
            functions: HashMap::from_iter([
                ("+".to_string(), add as MalFunc),
                ("-".to_string(), sub as MalFunc),
                ("/".to_string(), div as MalFunc),
                ("*".to_string(), mul as MalFunc),
            ]),
        }
    }
}

pub fn add(_tokens: Vec<Token>) -> MalRes<Token> {
    todo!()
}
pub fn sub(_tokens: Vec<Token>) -> MalRes<Token> {
    todo!()
}
pub fn div(_tokens: Vec<Token>) -> MalRes<Token> {
    todo!()
}
pub fn mul(_tokens: Vec<Token>) -> MalRes<Token> {
    todo!()
}
