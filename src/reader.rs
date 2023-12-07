use crate::{MalError, MalRes, Token};
use lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
            .unwrap();
}

struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    fn peek_token(&self) -> Result<char, MalError> {
        Ok(self
            .tokens
            .get(self.pos)
            .ok_or(MalError::Index(self.pos))?
            .chars()
            .next()
            .unwrap())
    }

    fn next_token(&mut self) -> Result<String, MalError> {
        let res = Ok(self
            .tokens
            .get(self.pos)
            .ok_or(MalError::Index(self.pos))?
            .to_string());
        self.pos += 1;
        res
    }

    fn read_form(&self) -> MalRes {
        match self.peek_token()? {
            '(' => todo!(),
            ')' => todo!(),
            c => Err(MalError::UnknownToken(c)),
        }
    }
}

fn tokenize(token_string: &str) -> Vec<String> {
    REGEX
        .captures_iter(token_string)
        .map(|c| c.get(1).map_or_else(|| "", |m| m.as_str()))
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

pub fn read_str(input: &str) -> MalRes {
    let tokens = tokenize(input);
    if tokens.is_empty() {
        return Err(MalError::Empty);
    }
    let pos = 0;
    let mut reader = Reader { tokens, pos };
    reader.read_form()
}
