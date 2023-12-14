use mal::{pr_str, read_str, MalError, MalRes, Token};
use rustyline::{self, error::ReadlineError, history::MemHistory, Editor};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = rustyline::config::Builder::new()
        .auto_add_history(true)
        .history_ignore_dups(false)?
        .build();
    let history = MemHistory::new();
    let mut rl: Editor<mal::RlHelper, MemHistory> = Editor::with_history(config, history)?;
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                match rep(line) {
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

fn rep<S: Into<String>>(cmd: S) -> MalRes<String> {
    let mut cmd = cmd.into();
    let mut token = read(cmd)?;
    token = eval(token);
    cmd = print(token);
    Ok(cmd)
}

fn read(cmd: String) -> MalRes<Token> {
    read_str(&cmd)
}

fn eval(cmd: Token) -> Token {
    cmd
}

fn print(token: Token) -> String {
    pr_str(&token, true)
}
