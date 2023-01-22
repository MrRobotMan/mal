use std::io::{stdin, stdout, Write};

fn read(line: String) -> Result<String, String> {
    match line.lines().next() {
        Some(s) => Ok(s.to_string()),
        None => Err("No Lines to Read".into()),
    }
}

fn eval(line: String) -> String {
    line
}

fn print(line: String) -> String {
    line
}

pub fn main() {
    loop {
        let mut buffer = String::new();
        print!("user> ");
        stdout().flush().expect("Could not flush");
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                if buffer.to_lowercase() == "exit\n" || buffer.to_lowercase() == "exit\r\n" {
                    break;
                };
                buffer = match read(buffer) {
                    Ok(buffer) => buffer,
                    Err(e) => {
                        eprint!("{e}");
                        break;
                    }
                };
                buffer = eval(buffer);
                println!("{}", print(buffer));
            }
            // EOF Signal
            Err(_) => break,
        };
    }
}

#[cfg(test)]
mod test_builder;

#[cfg(test)]
mod tests {
    use super::{read, test_builder::test_builder};

    #[test]
    fn test_step0_repl() {
        if let Ok(tests) = test_builder("test_files/step0_repl.mal") {
            for test in tests {
                assert_eq!(Ok(test.output), read(test.input))
            }
        }
    }
}
