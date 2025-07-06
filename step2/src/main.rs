use rustyline::{Editor, config::Builder, error::ReadlineError, history::FileHistory};
use std::error::Error;

mod error;
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
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                println!("{}", rep(line)?);
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

fn rep<S: AsRef<str>>(cmd: S) -> error::MalRes<String> {
    let cmd = cmd.as_ref();
    let cmd = reader::read(cmd)?;
    let cmd = eval(cmd)?;
    Ok(printer::print(cmd, true))
}

fn eval(input: token::Token) -> error::MalRes<token::Token> {
    Ok(input)
}

#[cfg(test)]
mod tests;
