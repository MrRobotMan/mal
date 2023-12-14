use crate::{MalError, MalRes, Token};
use lazy_static;
use regex::{Captures, Regex};
use std::collections::{HashMap, VecDeque};

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
            .unwrap();
    static ref INTEGER: Regex = Regex::new(r#"^-?\d+$"#).unwrap();
    static ref FLOAT: Regex = Regex::new(r#"^-?\d+\.\d*$"#).unwrap();
    static ref STRING: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
    static ref UNESCAPE: Regex = Regex::new(r#"\\(.)"#).unwrap();
}

struct Reader {
    tokens: VecDeque<String>,
}

impl Reader {
    fn tokenize(token_string: &str) -> Self {
        Self {
            tokens: REGEX
                .captures_iter(token_string)
                .map(|c| c.get(1).map_or_else(|| "", |m| m.as_str()))
                .map(|s| s.to_string())
                .collect::<VecDeque<_>>(),
        }
    }

    fn peek_token(&self) -> MalRes<String> {
        Ok(self.tokens.front().ok_or(MalError::Index)?.into())
    }

    fn next_token(&mut self) -> MalRes<String> {
        Ok(self.tokens.pop_front().ok_or(MalError::Index)?.to_string())
    }

    fn read_form(&mut self) -> MalRes<Token> {
        match self.peek_token()?.as_str() {
            c if "({[".contains(c) => self.read_sequence(),
            c if ")}]".contains(c) => Err(MalError::Brace(c.into())),
            "'" => {
                let _ = self.next_token(); // Clear the peeked token
                Ok(Token::List(vec![
                    Token::Symbol("quote".into()),
                    self.read_form()?,
                ]))
            }
            "`" => {
                let _ = self.next_token(); // Clear the peeked token
                Ok(Token::List(vec![
                    Token::Symbol("quasiquote".into()),
                    self.read_form()?,
                ]))
            }
            "~" => {
                let _ = self.next_token(); // Clear the peeked token
                Ok(Token::List(vec![
                    Token::Symbol("unquote".into()),
                    self.read_form()?,
                ]))
            }
            "~@" => {
                let _ = self.next_token(); // Clear the peeked token
                Ok(Token::List(vec![
                    Token::Symbol("splice-unquote".into()),
                    self.read_form()?,
                ]))
            }
            "@" => {
                let _ = self.next_token(); // Clear the peeked token
                Ok(Token::List(vec![
                    Token::Symbol("deref".into()),
                    self.read_form()?,
                ]))
            }
            "^" => {
                let _ = self.next_token(); // Clear the peeked token
                let meta = self.read_form()?;
                Ok(Token::List(vec![
                    Token::Symbol("with-meta".into()),
                    self.read_form()?,
                    meta,
                ]))
            }
            _ => self.read_atom(),
        }
    }

    fn read_sequence(&mut self) -> MalRes<Token> {
        let mut sequence = Vec::new();
        let closing = match self.next_token()?.as_str() {
            "(" => ")",
            "{" => "}",
            "[" => "]",
            c => return Err(MalError::UnknownToken(c.into())),
        };
        loop {
            match self.peek_token() {
                Ok(t) => match t {
                    t if t == closing => {
                        break;
                    }
                    _ => sequence.push(self.read_form()?),
                },
                Err(_) => return Err(MalError::Eof(closing.into())),
            };
        }
        // Call next token here to clear out the closing brace.
        let _ = self.next_token();
        match closing {
            ")" => Ok(Token::List(sequence)),
            "]" => Ok(Token::Vector(sequence)),
            "}" => {
                if sequence.len() % 2 == 1 {
                    Err(MalError::Map)
                } else {
                    let mut map = HashMap::new();
                    for chunk in sequence.chunks(2) {
                        match &chunk[0] {
                            Token::String(s) => map.insert(s.clone(), chunk[1].clone()),
                            t => return Err(MalError::MapKey(t.clone())),
                        };
                    }
                    Ok(Token::Map(map))
                }
            }
            c => Err(MalError::UnknownToken(c.into())),
        }
    }

    fn read_atom(&mut self) -> MalRes<Token> {
        match self.next_token()?.as_str() {
            "nil" => Ok(Token::Nil),
            "true" => Ok(Token::Bool(true)),
            "false" => Ok(Token::Bool(false)),
            token => {
                if FLOAT.is_match(token) {
                    Ok(Token::Number(token.parse().unwrap()))
                } else if INTEGER.is_match(token) {
                    Ok(Token::Number(token.parse::<i64>().unwrap() as f64))
                } else if STRING.is_match(token) {
                    Ok(Token::String(unescape(&token[1..token.len() - 1])))
                } else if let Some(t) = token.strip_prefix(':') {
                    Ok(Token::String(format!("\u{29e}{t}")))
                } else if token.starts_with('\"') {
                    Err(MalError::Eof("\"".into()))
                } else {
                    Ok(Token::Symbol(token.to_string()))
                }
            }
        }
    }
}

fn unescape(s: &str) -> String {
    UNESCAPE
        .replace_all(s, |capture: &Captures| {
            (if &capture[1] == "n" {
                "\n"
            } else {
                &capture[1]
            })
            .to_string()
        })
        .into()
}

pub fn read_str(input: &str) -> MalRes<Token> {
    let mut reader = Reader::tokenize(input);
    if reader.tokens.is_empty() {
        return Err(MalError::Empty);
    }
    reader.read_form()
}
