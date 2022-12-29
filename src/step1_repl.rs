use super::reader::Reader;
use rustyline::{error::ReadlineError, Editor};

use std::{
    error::Error,
    io::{self, stdin, stdout, Write},
};

fn read(line: String) -> String {
    line
}

fn eval(line: String) -> String {
    line
}

fn print(line: String) {
    println!("{line}");
}

pub fn rep() -> () {
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
            Ok(line) if line.to_lowercase() == "exit" => break,
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut buffer = read(line);
                buffer = eval(buffer);
                print(buffer)
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
