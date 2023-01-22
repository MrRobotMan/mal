#![allow(dead_code)]

use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MalVal {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    Sym(String),
    List(Rc<Vec<MalVal>>),
    Vector(Rc<Vec<MalVal>>),
    Hashmap(Rc<HashMap<String, MalVal>>),
    Func(fn(Vec<MalVal>) -> Result<MalVal, String>),
}

impl MalVal {
    pub fn list(vals: Vec<MalVal>) -> Self {
        Self::List(Rc::new(vals))
    }
    pub fn vector(vals: Vec<MalVal>) -> Self {
        Self::Vector(Rc::new(vals))
    }
    pub fn hashmap(vals: HashMap<String, MalVal>) -> Self {
        Self::Hashmap(Rc::new(vals))
    }

    pub fn apply(&self, args: Vec<MalVal>) -> Result<MalVal, String> {
        match *self {
            MalVal::Func(f) => f(args),
            _ => Err("Attempted to call a function on a non-function.".to_string()),
        }
    }
}
