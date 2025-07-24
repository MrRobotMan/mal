#[derive(Debug, Clone)]
pub(crate) enum Token {
    Bool(bool),
    Integer(i64),
    List(Vec<Token>),
    Map(Vec<Token>),
    Nil,
    Real(f64),
    String(String),
    Symbol(String),
    Vector(Vec<Token>),
}

impl Token {
    pub(crate) fn token_type(&self) -> &str {
        match self {
            Self::Bool(_) => "bool",
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
}
