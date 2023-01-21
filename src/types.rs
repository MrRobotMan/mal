use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MalVal {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    Sym(String),
    List(Rc<Vec<MalVal>>),
    Vector(Rc<Vec<MalVal>>),
    Hashmap(Rc<Vec<MalVal>>),
}

impl MalVal {
    pub fn list(vals: Vec<MalVal>) -> Self {
        Self::List(Rc::new(vals))
    }
    pub fn vector(vals: Vec<MalVal>) -> Self {
        Self::Vector(Rc::new(vals))
    }
    pub fn hashmap(vals: Vec<MalVal>) -> Self {
        Self::Hashmap(Rc::new(vals))
    }
}
