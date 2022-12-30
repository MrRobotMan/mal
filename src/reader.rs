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

/*
    [\s,]*: Matches any number of whitespaces or commas. This is not captured so it will
            be ignored and not tokenized.
    ~@: Captures the special two-characters ~@ (tokenized).
    [\[\]{}()'`~^@]: Captures any special single character, one of []{}()'`~^@ (tokenized).
    "(?:\\.|[^\\"])*"?: Starts capturing at a double-quote and stops at the next double-quote
                        unless it was preceded by a backslash in which case it includes
                        it until the next double-quote (tokenized). It will also match
                        unbalanced strings (no ending double-quote) which should be
                        reported as an error.
    ;.*: Captures any sequence of characters starting with ; (tokenized).
    [^\s\[\]{}('"`,;)]*: Captures a sequence of zero or more non special characters
                         (e.g. symbols, numbers, "true", "false", and "nil") and is sort
                         of the inverse of the one above that captures special characters.
                         Matches NOT whitespace, []{}()'"`, or ;
                         (tokenized).
*/
const REG: &str = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#; // String to match from instructions
lazy_static! {
    static ref TOKEN: Regex = Regex::new(REG).expect("Unexpected regex parse error");
    static ref INT_RE: Regex = Regex::new(r"^-?\d+$").expect("Unexpected int parse error");  // Match leading "-" 0 or 1 times and then 1 or more digits.
    static ref STR_RE: Regex =
        Regex::new(r#""(?:\\.|[^\\"])*""#).expect("Unexpected int parse error");  // Match from opening to closing " and everything between.
}

impl Reader {
    fn tokenize(text: &str) -> Self {
        Self {
            tokens: TOKEN
                .captures_iter(text)
                .filter_map(|c| c.get(1))
                .map(|s| s.as_str().to_string())
                .filter(|s| !s.starts_with(';')) // Filter out comments
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
        if self.tokens.is_empty() {
            return Err("");
        }
        match self.peek() {
            Some(token) => {
                if ")}]".contains(token) {
                    Err(r#"unexpected "{token}""#)
                } else if "({[".contains(token) {
                    self.read_list()
                } else {
                    self.read_atom()
                }
            }
            None => Err("Unexpected EOF"),
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
                    Ok(Str(s.to_string()))
                } else {
                    Ok(Sym(s.into()))
                }
            }
        }
    }

    fn read_list(&mut self) -> Result<MalVal, &str> {
        let mut result = vec![];
        let (opening, closing) = match self.next() {
            // TODO: "{" => ("{", "}"), // Hashmap
            "[" => ("[", "]"), // Vector
            _ => ("(", ")"),   // List (default)
        };
        loop {
            match self.peek() {
                None => return Err("Expected \")\", got EOF"),
                Some(s) if s == closing => break,
                Some(_) => {
                    if let Ok(val) = self.read_form() {
                        result.push(val)
                    }
                }
            }
        }
        self.next(); // Clear the "), }, or ]"
        match opening {
            // TODO: "{" => Ok(MalVal::List(Rc::new(result))),   // Hashmap
            "[" => Ok(MalVal::Vector(Rc::new(result))), // Vector
            _ => Ok(MalVal::List(Rc::new(result))),     // List (default)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MalVal::*;

    #[test]
    fn tokenize_test() {
        let test_input = "(+ 1 2)";
        let reader = Reader::tokenize(test_input);
        assert_eq!(reader.tokens, vec!["(", "+", "1", "2", ")"]);
    }

    #[test]
    fn test_single_value() {
        let result = read_str("0");
        assert_eq!(result, Ok(Int(0)));
    }

    #[test]
    fn test_single_list() {
        let result = read_str("(+ 1 2)");
        let expected = List(Rc::new(vec![Sym("+".to_string()), Int(1), Int(2)]));
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_nested_list() {
        let result = read_str("(+ 1 (* 2 3))");
        let expected = List(Rc::new(vec![
            Sym("+".to_string()),
            Int(1),
            List(Rc::new(vec![Sym("*".to_string()), Int(2), Int(3)])),
        ]));
        assert_eq!(result, Ok(expected));
    }
}
