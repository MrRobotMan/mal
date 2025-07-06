use crate::{
    error::{MalError, MalRes},
    token::Token,
};
use regex::{Captures, Regex};
use std::{
    collections::{HashMap, VecDeque},
    sync::LazyLock,
};

static TOKENS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap()
});

static INTEGER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-?\d+$").unwrap());
static FLOAT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-?(\d+\.\d*|\d*\.\d+)$").unwrap());
static STRING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap());
static UNESCAPE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\(.)").unwrap());

pub(crate) fn read(input: &str) -> MalRes<Token> {
    let mut reader = tokenize(input);
    if reader.is_empty() {
        Err(MalError::Empty)
    } else {
        read_tokens(&mut reader)
    }
}

fn tokenize(input: &str) -> VecDeque<&str> {
    TOKENS
        .captures_iter(input)
        .map(|c| c.get(1).map_or_else(|| "", |m| m.as_str()))
        .collect()
}

fn next<'r>(tokens: &mut VecDeque<&'r str>) -> MalRes<&'r str> {
    tokens.pop_front().ok_or(MalError::Peek)
}

fn peek<'r>(tokens: &VecDeque<&'r str>) -> MalRes<&'r str> {
    Ok(tokens.front().ok_or(MalError::Peek)?)
}

fn read_tokens(tokens: &mut VecDeque<&str>) -> MalRes<Token> {
    match peek(tokens)? {
        // Unwrap is ok here. Peek errors if there's no token.
        c if "({[".contains(c) => read_sequence(tokens),
        c if "]})".contains(c) => Err(MalError::Brace(c.into())),
        "'" => {
            let _ = next(tokens); // Clear the peeked token
            Ok(Token::List(vec![
                Token::Symbol("quote".into()),
                read_tokens(tokens)?,
            ]))
        }
        "`" => {
            let _ = next(tokens); // Clear the peeked token
            Ok(Token::List(vec![
                Token::Symbol("quasiquote".into()),
                read_tokens(tokens)?,
            ]))
        }
        "~" => {
            let _ = next(tokens); // Clear the peeked token
            Ok(Token::List(vec![
                Token::Symbol("unquote".into()),
                read_tokens(tokens)?,
            ]))
        }
        "~@" => {
            let _ = next(tokens); // Clear the peeked token
            Ok(Token::List(vec![
                Token::Symbol("splice-unquote".into()),
                read_tokens(tokens)?,
            ]))
        }
        "@" => {
            let _ = next(tokens); // Clear the peeked token
            Ok(Token::List(vec![
                Token::Symbol("deref".into()),
                read_tokens(tokens)?,
            ]))
        }
        "^" => {
            let _ = next(tokens); // Clear the peeked token
            let meta = read_tokens(tokens)?;

            Ok(Token::List(vec![
                Token::Symbol("with-meta".into()),
                read_tokens(tokens)?,
                meta,
            ]))
        }
        _ => read_atom(tokens),
    }
}

fn read_atom(tokens: &mut VecDeque<&str>) -> MalRes<Token> {
    match next(tokens)? {
        "nil" => Ok(Token::Nil),
        "true" => Ok(Token::Bool(true)),
        "false" => Ok(Token::Bool(false)),
        token => {
            if FLOAT.is_match(token) || INTEGER.is_match(token) {
                match token.parse::<f64>() {
                    Ok(n) => Ok(Token::Number(n)),
                    Err(_) => Err(MalError::ParseNumError(token.into())),
                }
            } else if STRING.is_match(token) {
                Ok(Token::String(unescape(&(token[1..token.len() - 1])).into()))
            } else if let Some(t) = token.strip_prefix(':') {
                Ok(Token::String(format!("\u{29e}{t}")))
            } else if token.starts_with('\"') {
                Err(MalError::Eof("\"".into()))
            } else {
                Ok(Token::Symbol(token.into()))
            }
        }
    }
}

fn unescape(token: &str) -> std::borrow::Cow<'_, str> {
    UNESCAPE.replace_all(token, |captures: &Captures| {
        if &captures[1] == "n" {
            String::from('\n')
        } else {
            captures[1].into()
        }
    })
}

fn read_sequence(tokens: &mut VecDeque<&str>) -> MalRes<Token> {
    let mut list = Vec::new();
    // Read the opening brace
    let closing = match next(tokens)? {
        "[" => "]",
        "{" => "}",
        "(" => ")",
        _ => unreachable!(),
    };
    loop {
        if let Ok(t) = peek(tokens) {
            if t == closing {
                break;
            } else {
                list.push(read_tokens(tokens)?);
            }
        } else {
            return Err(MalError::Eof(closing.into()));
        }
    }

    // Call to next to pass over the closing brace
    let _ = next(tokens);

    match closing {
        ")" => Ok(Token::List(list)),
        "]" => Ok(Token::Vector(list)),
        "}" => {
            if list.len() % 2 == 1 {
                return Err(MalError::MistmatchKeyValue);
            };
            let mut map = HashMap::new();
            for chunk in list.chunks(2) {
                if let Token::String(s) = &chunk[0] {
                    map.insert(s.clone(), chunk[1].clone());
                } else {
                    return Err(MalError::Map(chunk[0].clone()));
                }
            }
            Ok(Token::Map(map))
        }
        _ => unreachable!(),
    }
}
