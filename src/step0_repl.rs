use std::{
    error::Error,
    io::{self, stdin, stdout, Write},
};

fn read(line: String) -> Result<String, Box<dyn Error>> {
    match line.lines().next() {
        Some(s) => Ok(s.to_string()),
        None => Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "No Lines to Read",
        ))),
    }
}

fn eval(line: String) -> String {
    line
}

fn print(line: String) {
    println!("{line}");
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
                print(buffer);
            }
            // EOF Signal
            Err(_) => break,
        };
    }
}
