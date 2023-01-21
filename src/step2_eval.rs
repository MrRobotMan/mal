#[macro_use]
extern crate lazy_static;

mod printer;
mod reader;
mod types;

use printer::pr_str;
use reader::read_str;
use types::MalVal;

use rustyline::{error::ReadlineError, Editor};
use std::collections::HashMap;

fn eval_ast(ast: &MalVal, env: &HashMap<String, MalVal>) -> Result<MalVal, String> {
    match ast {
        MalVal::Sym(s) => {
            if let Some(func) = env.get(s) {
                Ok(func.clone())
            } else {
                Err(format!("No function for {s}"))
            }
        }
        MalVal::List(v) => {
            let mut collected: Vec<MalVal> = Vec::new();
            for mal in v.iter() {
                collected.push(eval(mal, env)?)
            }
            Ok(MalVal::list(collected))
        }
        MalVal::Vector(v) => {
            let mut collected: Vec<MalVal> = Vec::new();
            for mal in v.iter() {
                collected.push(eval(mal, env)?)
            }
            Ok(MalVal::vector(collected))
        }
        MalVal::Hashmap(v) => {
            let mut collected: Vec<MalVal> = Vec::new();
            for mal in v.iter() {
                collected.push(eval(mal, env)?)
            }
            Ok(MalVal::hashmap(collected))
        }
        _ => Ok(ast.clone()),
    }
}

fn eval(ast: &MalVal, env: &HashMap<String, MalVal>) -> Result<MalVal, String> {
    match ast {
        MalVal::List(v) => {
            if v.is_empty() {
                Ok(ast.clone())
            } else {
                let ret = eval_ast(ast, env)?;
                if let MalVal::List(v) = ret {
                    if let [MalVal::Func(f), MalVal::Int(l), MalVal::Int(r)] = v[0..=2] {
                        Ok(f(l, r))
                    } else {
                        Err(
                            "eval_ast returned a list that did not meet (func, int, int)"
                                .to_string(),
                        )
                    }
                } else {
                    Err("eval_ast should have returned a list.".to_string())
                }
            }
        }
        MalVal::Vector(v) => {
            if v.is_empty() {
                Ok(ast.clone())
            } else {
                let mut collected = Vec::new();
                for a in v.iter() {
                    collected.push(eval_ast(a, env)?)
                }
                Ok(MalVal::vector(collected))
            }
        }
        MalVal::Hashmap(v) => {
            if v.is_empty() {
                Ok(ast.clone())
            } else {
                let mut collected = Vec::new();
                for a in v.iter() {
                    collected.push(eval_ast(a, env)?)
                }
                Ok(MalVal::hashmap(collected))
            }
        }
        _ => eval_ast(ast, env),
    }
}

pub fn main() {
    let mut repl_env = HashMap::new();
    repl_env.insert("+".to_string(), MalVal::Func(|a, b| MalVal::Int(a + b)));
    repl_env.insert("-".to_string(), MalVal::Func(|a, b| MalVal::Int(a - b)));
    repl_env.insert("*".to_string(), MalVal::Func(|a, b| MalVal::Int(a * b)));
    repl_env.insert("/".to_string(), MalVal::Func(|a, b| MalVal::Int(a / b)));
    let mut rl = match Editor::<()>::new() {
        Ok(rl) => rl,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    };
    if rl.load_history(".mal_history").is_err() {
        eprintln!("No previous history.");
    }
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) if line.is_empty() => (),
            Ok(line) if line.to_lowercase() == "exit" => break,
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match read_str(&line) {
                    Ok(val) => match eval(&val, &repl_env) {
                        Ok(val) => println!("{}", pr_str(&val, true)),
                        Err(e) => println!("ERROR: {e}"),
                    },
                    Err(e) => println!("ERROR: {e}"),
                };
            }
            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => break,
            Err(err) => {
                eprintln!("{err}");
                break;
            }
        }
    }
    match rl.save_history(".mal_history") {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    };
}
