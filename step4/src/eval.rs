use std::collections::VecDeque;

use crate::{
    env::Env,
    error::{FnError, MalError, MalRes},
    printer::print,
    token::Token,
};

pub(crate) fn eval(ast: Token, env: &mut Env) -> MalRes<Token> {
    if let Some(Token::Bool(true)) = env.get("DEBUG-EVAL") {
        println!("EVAL: {}, {:?}", print(ast.clone(), true), env);
    };
    match ast {
        Token::List(mut tokens) => {
            if tokens.is_empty() {
                return Ok(Token::List(tokens));
            }
            let Some(Token::Symbol(symbol)) = tokens.pop_front() else {
                return Err(FnError::CallableError.into());
            };
            match symbol.as_str() {
                "def!" => {
                    let token = tokens.pop_front();
                    if let Some(Token::Symbol(key)) = token {
                        let tok = if tokens.len() == 1 {
                            tokens.pop_front().unwrap() // Unwrap ok, checking for length
                        } else {
                            Token::List(tokens)
                        };
                        let res = eval(tok, env)?;
                        env.insert(key, res.clone());
                        Ok(res)
                    } else {
                        Err(MalError::BadDef(token))
                    }
                }
                "let*" => {
                    let mut local = Env::new(Some(env));
                    let token = tokens.pop_front().unwrap(); // Unwrap ok. We've checked for empty.
                    if let Some(bindings) = token.inner_list() {
                        if bindings.len() % 2 == 1 {
                            return Err(MalError::MistmatchKeyValue("Environment".into()));
                        };
                        for idx in (0..bindings.len()).step_by(2) {
                            let res = eval(bindings[idx + 1].clone(), &mut local)?;
                            if let Token::Symbol(key) = &bindings[idx] {
                                local.insert(key, res);
                            } else {
                                return Err(MalError::BadTokenString(bindings[idx].clone()));
                            };
                        }
                        if let Some(token) = tokens.pop_front() {
                            eval(token, &mut local)
                        } else {
                            Err(MalError::Eof("Expected token, found end of list".into()))
                        }
                    } else {
                        Err(MalError::BadDef(Some(token)))
                    }
                }
                _ => {
                    let rest = tokens
                        .into_iter()
                        .map(|t| eval(t, env))
                        .collect::<MalRes<Vec<_>>>()?;
                    if let Some(Token::Func(func)) = env.get(&symbol) {
                        func(&rest)
                    } else {
                        Err(FnError::KeyError(symbol).into())
                    }
                }
            }
        }
        Token::Map(tokens) => Ok(Token::Map(
            tokens
                .into_iter()
                .map(|t| eval(t, env))
                .collect::<MalRes<VecDeque<_>>>()?,
        )),
        Token::Symbol(s) => env.get(&s).ok_or(MalError::UnknownSymbol(s)).cloned(),
        Token::Vector(tokens) => Ok(Token::Vector(
            tokens
                .into_iter()
                .map(|t| eval(t, env))
                .collect::<MalRes<VecDeque<_>>>()?,
        )),
        _ => Ok(ast),
    }
}
