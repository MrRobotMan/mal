use core::panic;
use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use subprocess::{Popen, PopenConfig, Redirection};

fn main() {
    let options = Cli::parse();
    let test_results = TestResults::new(options.testfiles);
    let mut input = None;
    let mut output = None;
    let mut passed = 0;
    let mut failures = Vec::new();
    for line in test_results {
        match line {
            TestRunner::Message(m) => println!("{m}"),
            TestRunner::Input(i) => {
                if let Some(o) = output {
                    panic!("Input {i} being created before output {o} was cleared.");
                }
                input = Some(i);
            }
            TestRunner::Output(o) => {
                if input.is_none() {
                    panic!("Output {o} was created before input.");
                }
                output = Some(o);
                let inp = input.take().unwrap_or_else(|| panic!("No Input"));
                let out = output.take().unwrap_or_else(|| panic!("No Output"));
                println!("{inp} => {out}");
                let result = check_results(&options.testmod, &inp, &out);
                if result.pass {
                    passed += 1;
                } else {
                    failures.push(result);
                };
            }
            TestRunner::Continue => (),
        }
    }
    let failed = failures.len();
    let total = passed + failed;
    println!("\n\n--- {} TEST RESULTS ---", options.testmod);
    println!("  {passed}: passing");
    println!("  {failed}: failing");
    println!("  {total}: total");
    if failed > 0 {
        println!("\nFailures:");
        for test in failures {
            println!(
                "Input: {}, Expected: {}, Actual: {}",
                test.input, test.expected, test.actual
            );
        }
    };
}

fn check_results(module: &str, input: &str, output: &str) -> TestResult {
    let mut cmd = Popen::create(
        &["cargo", "run", "--bin", module],
        PopenConfig {
            stdin: Redirection::Pipe,
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    )
    .unwrap_or_else(|_| panic!("Failed to create process"));
    if let Ok((out, err)) = cmd.communicate(Some(input)) {
        let actual = out.unwrap_or(String::new()).trim().into();
        let err = err.unwrap_or(String::new()).trim().into();
        TestResult {
            pass: actual == output,
            input: input.into(),
            expected: output.into(),
            actual,
            _err: err,
        }
    } else {
        TestResult {
            pass: false,
            ..Default::default()
        }
    }
}

#[derive(Parser)]
struct Cli {
    testmod: String,
    testfiles: Vec<String>,
}

#[derive(Default)]
struct TestResult {
    pass: bool,
    input: String,
    expected: String,
    actual: String,
    _err: String,
}

#[derive(Default, Debug)]
struct TestResults {
    output: String,
    lines: Vec<String>,
    current: usize,
}

impl TestResults {
    fn new(files: Vec<String>) -> Self {
        let mut lines = Vec::new();
        for file in files {
            let mut path = PathBuf::from("test_files");
            path.push(file);
            let file_lines = read_to_string(path)
                .expect("Failed to open file {path:?}.")
                .lines()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            lines.extend_from_slice(&file_lines);
        }
        Self {
            lines,
            ..Default::default()
        }
    }
}

impl Iterator for TestResults {
    type Item = TestRunner;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.lines.len() {
            None
        } else {
            let line = &self.lines[self.current];
            self.current += 1;
            if line.starts_with(";;;") {
                // Comment within file.
                return Some(TestRunner::Continue);
            }
            if let Some(l) = line.strip_prefix(";; ") {
                // Output Comment
                return Some(TestRunner::Message(l.into()));
            }
            if line.starts_with(";>>>") {
                // Original repo commands / settings
                return Some(TestRunner::Continue);
            }
            if let Some(l) = line.strip_prefix(";=>") {
                //output line
                return Some(TestRunner::Output(l.into()));
            }
            if let Some(l) = line.strip_prefix(";/") {
                self.output.push_str(l);
                return Some(TestRunner::Continue);
            }
            Some(TestRunner::Input(line.into()))
        }
    }
}

#[derive(Debug)]
enum TestRunner {
    Message(String),
    Input(String),
    Output(String),
    Continue,
}
