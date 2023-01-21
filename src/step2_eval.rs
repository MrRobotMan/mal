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
            } else if let MalVal::List(v) = eval_ast(ast, env)? {
                v[0].apply(v[1..].to_vec())
            } else {
                Err("eval_ast should have returned a list.".to_string())
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

fn int_operation(op: fn(i64, i64) -> i64, a: Vec<MalVal>) -> Result<MalVal, String> {
    if let (MalVal::Int(l), MalVal::Int(r)) = (&a[0], &a[1]) {
        Ok(MalVal::Int(op(*l, *r)))
    } else {
        Err("Invalid operands.".to_string())
    }
}

pub fn main() {
    let mut repl_env = HashMap::new();
    repl_env.insert(
        "+".to_string(),
        MalVal::Func(|a: Vec<MalVal>| int_operation(|a, b| a + b, a)),
    );
    repl_env.insert(
        "-".to_string(),
        MalVal::Func(|a: Vec<MalVal>| int_operation(|a, b| a - b, a)),
    );
    repl_env.insert(
        "*".to_string(),
        MalVal::Func(|a: Vec<MalVal>| int_operation(|a, b| a * b, a)),
    );
    repl_env.insert(
        "/".to_string(),
        MalVal::Func(|a: Vec<MalVal>| int_operation(|a, b| a / b, a)),
    );
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
