use rustyline::{Editor, config::Builder, error::ReadlineError, history::FileHistory};
use std::error::Error;
use token::Token;

mod env;
mod error;
mod eval;
mod printer;
mod reader;
mod token;

const HISTORY_FILE: &str = ".mal_history.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let config = Builder::new()
        .max_history_size(1000)
        .unwrap_or_default()
        .build();
    let history = FileHistory::with_config(config);
    let mut rl: Editor<(), FileHistory> = Editor::with_history(config, history)?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.")
    }
    let mut env = env::Env::default();
    env.insert("DEBUG-EVAL", Box::new(|_| Ok(Token::Bool(false))));
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match rep(line, &env) {
                    Ok(res) => println!("{res}"),
                    Err(err) => println!("Error: {err}"),
                }
            }

            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => break,
            Err(err) => {
                println!("{err}");
                break;
            }
        }
    }
    rl.save_history(HISTORY_FILE)?;
    Ok(())
}

fn rep<S: AsRef<str>>(cmd: S, env: &env::Env) -> error::MalRes<String> {
    let cmd = cmd.as_ref();
    let ast = reader::read(cmd)?;
    let expr = eval::eval(ast, env)?;
    Ok(printer::print(expr, true))
}

#[cfg(test)]
mod tests;
