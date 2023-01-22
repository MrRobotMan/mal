#![cfg(test)]

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

#[derive(Debug)]
pub struct Test {
    pub input: String,
    pub output: String,
}
pub fn test_builder<T: AsRef<Path>>(file: T) -> Result<Vec<Test>> {
    let reader = read_lines(file)?;
    let lines = lines_to_vec(reader)?;
    let mut result = Vec::new();
    for (line, contents) in lines.iter().enumerate() {
        if let Some(output) = contents.strip_prefix(";=>") {
            result.push(Test {
                input: lines[line - 1].clone(),
                output: output.into(),
            })
        }
    }

    Ok(result)
}

fn read_lines<T: AsRef<Path>>(file: T) -> Result<Lines<BufReader<File>>> {
    let file = File::open(file)?;
    Ok(BufReader::new(file).lines())
}

fn lines_to_vec(lines: Lines<BufReader<File>>) -> Result<Vec<String>> {
    let mut collected = Vec::new();
    for line in lines {
        collected.push(line?.trim().into())
    }
    Ok(collected)
}
