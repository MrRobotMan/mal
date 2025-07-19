use std::collections::HashMap;

use crate::{
    error::{FnError, MalRes},
    token::Token,
};

type Func = Box<fn(&[Token]) -> MalRes<Token>>;

pub(crate) struct Env {
    map: HashMap<String, Func>,
}

impl Default for Env {
    fn default() -> Self {
        let mut map: HashMap<String, Func> = HashMap::with_capacity(10);
        map.insert("+".into(), Box::new(add));
        map.insert("-".into(), Box::new(sub));
        map.insert("*".into(), Box::new(mul));
        map.insert("/".into(), Box::new(div));
        Self { map }
    }
}

impl Env {
    pub fn get(&self, key: &str) -> Option<&Func> {
        self.map.get(key)
    }

    pub fn insert<S: Into<String>>(&mut self, key: S, value: Func) {
        self.map.insert(key.into(), value);
    }
}

pub(crate) fn add(ast: &[Token]) -> MalRes<Token> {
    let l = ast.len();
    if l < 2 {
        return Err(FnError::Argument(2, l).into());
    }
    let mut t1 = ast[0].clone();
    for t2 in &ast[1..] {
        t1 = match [&t1, t2] {
            [Token::Real(n1), Token::Real(n2)] => Token::Real(n1 + n2),
            [Token::Real(n1), Token::Integer(n2)] | [Token::Integer(n2), Token::Real(n1)] => {
                Token::Real(n1 + *n2 as f64)
            }
            [Token::Integer(n1), Token::Integer(n2)] => Token::Integer(n1 + n2),
            [t1, t2] => {
                return Err(FnError::Type(
                    "Add".into(),
                    t1.token_type().into(),
                    t2.token_type().into(),
                )
                .into());
            }
        };
    }
    Ok(t1)
}

pub(crate) fn sub(ast: &[Token]) -> MalRes<Token> {
    let l = ast.len();
    if l < 2 {
        return Err(FnError::Argument(2, l).into());
    }
    let mut t1 = ast[0].clone();
    for t2 in &ast[1..] {
        t1 = match [&t1, t2] {
            [Token::Real(n1), Token::Real(n2)] => Token::Real(n1 - n2),
            [Token::Real(n1), Token::Integer(n2)] | [Token::Integer(n2), Token::Real(n1)] => {
                Token::Real(n1 - *n2 as f64)
            }
            [Token::Integer(n1), Token::Integer(n2)] => Token::Integer(n1 - n2),
            [t1, t2] => {
                return Err(FnError::Type(
                    "Add".into(),
                    t1.token_type().into(),
                    t2.token_type().into(),
                )
                .into());
            }
        };
    }
    Ok(t1)
}

pub(crate) fn mul(ast: &[Token]) -> MalRes<Token> {
    let l = ast.len();
    if l < 2 {
        return Err(FnError::Argument(2, l).into());
    }
    let mut t1 = ast[0].clone();
    for t2 in &ast[1..] {
        t1 = match [&t1, t2] {
            [Token::Real(n1), Token::Real(n2)] => Token::Real(n1 * n2),
            [Token::Real(n1), Token::Integer(n2)] | [Token::Integer(n2), Token::Real(n1)] => {
                Token::Real(n1 * *n2 as f64)
            }
            [Token::Integer(n1), Token::Integer(n2)] => Token::Integer(n1 * n2),
            [t1, t2] => {
                return Err(FnError::Type(
                    "Add".into(),
                    t1.token_type().into(),
                    t2.token_type().into(),
                )
                .into());
            }
        };
    }
    Ok(t1)
}
pub(crate) fn div(ast: &[Token]) -> MalRes<Token> {
    let l = ast.len();
    if l < 2 {
        return Err(FnError::Argument(2, l).into());
    }
    let mut t1 = ast[0].clone();
    for t2 in &ast[1..] {
        t1 = match [&t1, t2] {
            [Token::Real(n1), Token::Real(n2)] => Token::Real(n1 / n2),
            [Token::Real(n1), Token::Integer(n2)] => {
                if *n2 == 0 {
                    return Err(FnError::ZeroDivision.into());
                }
                Token::Real(n1 / *n2 as f64)
            }
            [Token::Integer(n1), Token::Real(n2)] => {
                if *n2 == 0.0 {
                    return Err(FnError::ZeroDivision.into());
                }
                Token::Real(*n1 as f64 / n2)
            }
            [Token::Integer(n1), Token::Integer(n2)] => {
                if *n2 == 0 {
                    return Err(FnError::ZeroDivision.into());
                }
                Token::Integer(n1 / n2)
            }
            [t1, t2] => {
                return Err(FnError::Type(
                    "Add".into(),
                    t1.token_type().into(),
                    t2.token_type().into(),
                )
                .into());
            }
        };
    }
    Ok(t1)
}
