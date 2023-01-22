#[macro_use]
extern crate lazy_static;

mod printer;
mod reader;
mod types;

use printer::pr_str;
use reader::read_str;

use rustyline::{error::ReadlineError, Editor};

pub fn main() {
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
                    Ok(val) => {
                        println!("{}", pr_str(&val, true))
                    }
                    Err(e) => println!("{e}"),
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
    fn test_step1_read_print() {
        if let Ok(tests) = test_builder("test_files/step1_read_print.mal") {
            for test in tests {
                if let Ok(val) = read_str(&test.input) {
                    assert_eq!(test.output, pr_str(&val, true))
                }
            }
        }
    }
}
