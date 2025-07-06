mod error;
use error::MalRes;
use token::Token;

mod printer;
mod reader;
mod token;

pub fn rep<S: AsRef<str>>(cmd: S) -> MalRes<String> {
    let cmd = cmd.as_ref();
    let cmd = reader::read(cmd)?;
    let cmd = eval(cmd)?;
    Ok(printer::print(cmd, true))
}

fn eval(input: Token) -> MalRes<Token> {
    Ok(input)
}

#[cfg(test)]
mod tests;
