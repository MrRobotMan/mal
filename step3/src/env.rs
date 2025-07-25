use std::collections::{HashMap, hash_map::Keys};

use crate::{funcs, token::Token};

#[derive(Debug)]
pub(crate) struct Env<'a> {
    symbols: HashMap<String, Token>,
    parent: Option<&'a Self>,
}

impl Default for Env<'_> {
    fn default() -> Self {
        let mut symbols: HashMap<String, Token> = HashMap::with_capacity(10);
        symbols.insert("+".into(), Token::Func(Box::new(funcs::add)));
        symbols.insert("-".into(), Token::Func(Box::new(funcs::sub)));
        symbols.insert("*".into(), Token::Func(Box::new(funcs::mul)));
        symbols.insert("/".into(), Token::Func(Box::new(funcs::div)));
        Self {
            symbols,
            parent: None,
        }
    }
}

impl<'a> Env<'a> {
    pub fn new(parent: Option<&'a Self>) -> Env<'a> {
        Self {
            symbols: HashMap::new(),
            parent,
        }
    }

    pub fn with_debug(self) -> Self {
        let mut s = self;
        s.insert("DEBUG-EVAL", Token::Bool(true));
        s
    }

    pub fn get(&self, key: &str) -> Option<&Token> {
        self.symbols.get(key).or_else(|| self.parent?.get(key))
    }

    pub fn insert<S: Into<String>>(&mut self, key: S, value: Token) {
        self.symbols.insert(key.into(), value);
    }

    pub fn symbols(&self) -> Keys<'_, String, Token> {
        self.symbols.keys()
    }
}
