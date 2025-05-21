use rustyline::error::ReadlineError;
use std::error::Error;
use step0::{readline, rep};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = readline()?;
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                println!("{}", rep(line));
            }

            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => break,
            Err(err) => {
                println!("{err}");
                break;
            }
        }
    }
    Ok(())
}
