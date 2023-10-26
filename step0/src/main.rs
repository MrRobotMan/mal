use rustyline::{self, error::ReadlineError, history::MemHistory, Editor};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = rustyline::config::Builder::new()
        .auto_add_history(true)
        .history_ignore_dups(false)?
        .build();
    let history = MemHistory::new();
    // let mut rl = rustyline::DefaultEditor::new()?;
    let mut rl: Editor<helper::RlHelper, MemHistory> = Editor::with_history(config, history)?;
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                println!("{}", rep(line));
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

fn rep<S: Into<String>>(cmd: S) -> String {
    let mut cmd = cmd.into();
    cmd = read(cmd);
    cmd = eval(cmd);
    cmd = print(cmd);
    cmd
}

fn read(cmd: String) -> String {
    cmd
}

fn eval(cmd: String) -> String {
    cmd
}

fn print(cmd: String) -> String {
    cmd
}

#[cfg(test)]
mod test;
