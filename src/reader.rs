use super::types::MalVal;
use regex::Regex;

pub fn read_str(st: &str) -> Result<MalVal, String> {
    let mut reader = Reader::tokenize(st);
    reader.read_form()
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

    fn read_form(&mut self) -> Result<MalVal, String> {
        if self.tokens.is_empty() {
            return Err("".into());
        }
        match self.peek() {
            Some(token) => match token {
                ")" | "}" | "]" => Err(format!("Unexpected '{token}'!")),
                "(" | "{" | "[" => self.read_list(),
                "'" => {
                    self.next();
                    Ok(MalVal::list(vec![
                        MalVal::Sym("quote".to_string()),
                        self.read_form()?,
                    ]))
                }
                "`" => {
                    self.next();
                    Ok(MalVal::list(vec![
                        MalVal::Sym("quasiquote".to_string()),
                        self.read_form()?,
                    ]))
                }
                "~" => {
                    self.next();
                    Ok(MalVal::list(vec![
                        MalVal::Sym("unquote".to_string()),
                        self.read_form()?,
                    ]))
                }
                "~@" => {
                    self.next();
                    Ok(MalVal::list(vec![
                        MalVal::Sym("splice-unquote".to_string()),
                        self.read_form()?,
                    ]))
                }
                "^" => {
                    self.next();
                    let meta = self.read_form()?;
                    Ok(MalVal::list(vec![
                        MalVal::Sym("with-meta".to_string()),
                        self.read_form()?,
                        meta,
                    ]))
                }
                "@" => {
                    self.next();
                    Ok(MalVal::list(vec![
                        MalVal::Sym("deref".to_string()),
                        self.read_form()?,
                    ]))
                }
                _ => self.read_atom(),
            },
            None => Err("Unexpected EOF".into()),
        }
    }

    fn read_atom(&mut self) -> Result<MalVal, String> {
        use MalVal::*;
        match self.next() {
            "nil" => Ok(Nil),
            "true" => Ok(Bool(true)),
            "false" => Ok(Bool(false)),
            s => {
                if INT_RE.is_match(s) {
                    Ok(Int(s.parse::<i64>().unwrap()))
                } else if STR_RE.is_match(s) {
                    Ok(Str(unescape(&s[1..s.len() - 1])))
                } else if s.starts_with('"') {
                    Err("Expected '\"', got EOF".into())
                } else if let Some(keyword) = s.strip_prefix(':') {
                    Ok(Str(format!("\u{29e}{keyword}")))
                } else {
                    Ok(Sym(s.into()))
                }
            }
        }
    }

    fn read_list(&mut self) -> Result<MalVal, String> {
        let mut result = vec![];
        let (opening, closing) = match self.next() {
            "{" => ("{", "}"), // Hashmap
            "[" => ("[", "]"), // Vector
            "(" => ("(", ")"), // List (default)
            _ => return Err("Unknown opening bracket!".into()),
        };
        loop {
            match self.peek() {
                None => return Err(format!("Expected \"{closing}\", got EOF")),
                Some(s) if s == closing => break,
                Some(_) => match self.read_form() {
                    Ok(val) => result.push(val),
                    Err(e) => return Err(e),
                },
            }
        }
        self.next(); // Clear the "), }, or ]"
        match opening {
            "{" => Ok(MalVal::hashmap(result)), // Hashmap
            "[" => Ok(MalVal::vector(result)),  // Vector
            "(" => Ok(MalVal::list(result)),    // List (default)
            _ => Err("Unknown ending bracket".into()),
        }
    }
}

fn unescape(s: &str) -> String {
    s.replace("\\n", "\n")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
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
        let expected = MalVal::list(vec![Sym("+".to_string()), Int(1), Int(2)]);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_nested_list() {
        let result = read_str("(+ 1 (* 2 3))");
        let expected = MalVal::list(vec![
            Sym("+".to_string()),
            Int(1),
            MalVal::list(vec![Sym("*".to_string()), Int(2), Int(3)]),
        ]);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_unescape() {
        let input = "Some\\n\\\"Text\\\"That's been\\\\escaped!";
        let result = unescape(input);
        assert_eq!(result, "Some\n\"Text\"That's been\\escaped!")
    }

    #[test]
    fn test_mismatched_brackets() {
        let result = read_str("[1 2 3 (+ 3 4]");
        assert_eq!(result, Err("Unexpected ']'!".into()));
    }
}
