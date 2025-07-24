use crate::{
    env::Env,
    error::{FnError, MalError, MalRes},
    printer::print,
    token::Token,
};

pub(crate) fn eval(ast: Token, env: &Env) -> MalRes<Token> {
    if let Some(f) = env.get("DEBUG-EVAL") {
        if let Ok(Token::Bool(true)) = f(&[]) {
            println!("EVAL: {}", print(ast.clone(), true));
        };
    }
    match ast {
        Token::List(mut tokens) => {
            if tokens.is_empty() {
                return Ok(Token::List(vec![]));
            }
            let Token::Symbol(symbol) = tokens.remove(0) else {
                return Err(FnError::CallableError.into());
            };
            let rest = tokens
                .into_iter()
                .map(|t| eval(t, env))
                .collect::<MalRes<Vec<_>>>()?;
            if let Some(func) = env.get(&symbol) {
                func(&rest)
            } else {
                Err(FnError::KeyError(symbol).into())
            }
        }
        Token::Map(hash_map) => Ok(Token::Map(
            hash_map
                .into_iter()
                .map(|t| eval(t, env))
                .collect::<MalRes<Vec<_>>>()?,
        )),
        Token::Symbol(_) => Err(MalError::BadSymbolPosition),
        Token::Vector(tokens) => Ok(Token::Vector(
            tokens
                .into_iter()
                .map(|t| eval(t, env))
                .collect::<MalRes<Vec<_>>>()?,
        )),
        _ => Ok(ast),
    }
}
