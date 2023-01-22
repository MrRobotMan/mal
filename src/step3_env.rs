#[macro_use]
extern crate lazy_static;

mod env;
mod printer;
mod reader;
mod types;

use std::collections::HashMap;

use env::Env;
use printer::pr_str;
use reader::read_str;
use types::MalVal;

use rustyline::{error::ReadlineError, Editor};

fn eval_ast(ast: &MalVal, env: &Env) -> Result<MalVal, String> {
    use MalVal::*;
    match ast {
        Sym(s) => {
            if let Ok(func) = env.get(s) {
                Ok(func.clone())
            } else {
                Err(format!("No function for {s}"))
            }
        }
        List(v) => {
            let mut collected: Vec<MalVal> = Vec::new();
            for mal in v.iter() {
                collected.push(eval(mal, env)?)
            }
            Ok(MalVal::list(collected))
        }
        Vector(v) => {
            let mut collected: Vec<MalVal> = Vec::new();
            for mal in v.iter() {
                collected.push(eval(mal, env)?)
            }
            Ok(MalVal::vector(collected))
        }
        MalVal::Hashmap(hm) => {
            let mut collected: HashMap<String, MalVal> = HashMap::new();
            for (key, mal) in hm.iter() {
                collected.insert(key.clone(), eval(mal, env)?);
            }
            Ok(MalVal::hashmap(collected))
        }
        _ => Ok(ast.clone()),
    }
}

fn eval(ast: &MalVal, env: &Env) -> Result<MalVal, String> {
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
    let mut env = Env::default();
    env.set(
        "+".to_string(),
        MalVal::Func(|a: Vec<MalVal>| int_operation(|a, b| a + b, a)),
    );
    env.set(
        "-".to_string(),
        MalVal::Func(|a: Vec<MalVal>| int_operation(|a, b| a - b, a)),
    );
    env.set(
        "*".to_string(),
        MalVal::Func(|a: Vec<MalVal>| int_operation(|a, b| a * b, a)),
    );
    env.set(
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
                    Ok(val) => match eval(&val, &env) {
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

#[cfg(test)]
mod test_builder;

#[cfg(test)]
mod tests {
    use crate::{printer::pr_str, reader::read_str, test_builder::test_builder};

    #[test]
    fn test_step2_eval() {
        if let Ok(tests) = test_builder("test_files/step3_env.mal") {
            for test in tests {
                if let Ok(val) = read_str(&test.input) {
                    assert_eq!(test.output, pr_str(&val, true))
                }
            }
        }
    }
}
