use std::{error::Error, process::exit};

mod step0_repl;

fn main() {
    let runners: Vec<fn() -> Result<(), Box<dyn Error>>> = vec![step0_repl::rep];
    let args = std::env::args().collect::<Vec<String>>();

    let runner = if args.len() == 2 {
        let step = if let Ok(step) = args[1].parse::<usize>() {
            step
        } else if args[1] == "A" {
            10
        } else {
            eprintln!("Invalid step {}", args[1]);
            exit(1);
        };

        if !(0..=10).contains(&step) {
            eprintln!("Step must be in range 0..10 (use 10 for 'A')");
            exit(1);
        }
        runners[step]
    } else {
        *runners.last().unwrap()
    };
    match runner() {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }
}
