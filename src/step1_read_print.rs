use super::{printer::pr_str, reader::read_str};
use rustyline::{error::ReadlineError, Editor};

pub fn rep() {
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
