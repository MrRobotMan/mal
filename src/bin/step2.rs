use mal::{pr_str, read_str, Env, MalError, MalRes, ReplEnv, Token};
use rustyline::{self, error::ReadlineError, history::MemHistory, Editor};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = rustyline::config::Builder::new()
        .auto_add_history(true)
        .history_ignore_dups(false)?
        .build();
    let history = MemHistory::new();
    let mut rl: Editor<mal::RlHelper, MemHistory> = Editor::with_history(config, history)?;
    let env = ReplEnv::default();
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                match rep(line, &env) {
                    Ok(v) => println!("{v}"),
                    Err(e) => match e {
                        MalError::Token => (),
                        MalError::Empty => (),
                        _ => println!("{e}"),
                    },
                };
            }

            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("{err}");
                break;
            }
        }
    }
    Ok(())
}

fn rep<S: Into<String>>(cmd: S, env: &dyn Env) -> MalRes<String> {
    let cmd = cmd.into();
    let mut ast = read(cmd)?;
    ast = eval(ast, env)?;
    Ok(print(ast))
}

fn read(cmd: String) -> MalRes<Token> {
    read_str(&cmd)
}

fn eval(ast: Token, env: &dyn Env) -> MalRes<Token> {
    if let Token::List(ref list) = ast {
        if list.is_empty() {
            Ok(ast)
        } else if let Token::Func(res) = eval_ast(list[0].clone(), env)? {
            Ok(res(list[1..].to_vec())?)
        } else {
            Err(MalError::NoReplFunction(
                "eval_ast did not return a func".into(),
            ))
        }
    } else {
        eval_ast(ast, env)
    }
}

fn eval_ast(ast: Token, env: &dyn Env) -> MalRes<Token> {
    match ast {
        Token::Symbol(s) => Ok(env.get(&s)?),
        Token::List(l) => Ok(Token::List(
            l.iter()
                .map(|t| eval(t.clone(), env))
                .collect::<MalRes<Vec<_>>>()?,
        )),
        _ => Ok(ast),
    }
}

fn print(ast: Token) -> String {
    pr_str(&ast, true)
}
