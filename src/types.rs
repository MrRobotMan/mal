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
}
