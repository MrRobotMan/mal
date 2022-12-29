use std::rc::Rc;

use super::types::MalVal;
use regex::Regex;

pub fn read_str(st: &str) -> Result<MalVal, String> {
    let mut reader = Reader::tokenize(st);
    match reader.read_form() {
        Ok(val) => Ok(val),
        Err(s) => Err(s.into()),
    }
}

#[derive(Debug)]
struct Reader {
    tokens: Vec<String>,
    index: usize,
}

const REG: &str = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;
lazy_static! {
    static ref TOKEN: Regex = Regex::new(REG).expect("Unexpected regex parse error");
    static ref INT_RE: Regex = Regex::new(r"^-?\d+$").expect("Unexpected int parse error");
    static ref STR_RE: Regex =
        Regex::new(r#""(?:\\.|[^\\"])*""#).expect("Unexpected int parse error");
}

impl Reader {
    fn tokenize(text: &str) -> Self {
        Self {
            tokens: TOKEN
                .captures_iter(text)
                .filter_map(|c| c.get(1))
                .map(|s| s.as_str().into())
                .collect(),
            index: 0,
        }
    }

    fn next(&mut self) -> &str {
        self.index += 1;
        &self.tokens[self.index - 1]
    }

    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.index).map(|s| s.as_str())
    }

    fn read_form(&mut self) -> Result<MalVal, &str> {
        match self.peek() {
            None => Err("Unexpected EOF"),
            Some("(") => self.read_list(),
            _ => self.read_atom(),
        }
    }

    fn read_atom(&mut self) -> Result<MalVal, &str> {
        use MalVal::*;
        match self.next() {
            "nil" => Ok(Nil),
            "true" => Ok(Bool(true)),
            "false" => Ok(Bool(false)),
            s => {
                if INT_RE.is_match(s) {
                    Ok(Int(s.parse::<i64>().unwrap()))
                } else if STR_RE.is_match(s) {
                    Ok(Str(s.into()))
                } else {
                    Ok(Sym(s.into()))
                }
            }
        }
    }

    fn read_list(&mut self) -> Result<MalVal, &str> {
        let mut result = vec![];
        loop {
            match self.peek() {
                None => return Err("Expected \")\" got EOF"),
                Some(")") => break,
                Some(_) => {
                    if let Ok(val) = self.read_form() {
                        result.push(val)
                    }
                }
            }
        }
        Ok(MalVal::List(Rc::new(result)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_test() {
        let test_input = "(+ 1 2)";
        let reader = Reader::tokenize(test_input);
        assert_eq!(reader.tokens, vec!["(", "+", "1", "2", ")"]);
    }
}
