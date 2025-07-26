use crate::token::Token;
use std::collections::VecDeque;

pub fn print(token: Token, escape: bool) -> String {
    match token {
        Token::Bool(b) => format!("{b}"),
        Token::Func(_) => format!("Function Call"),
        Token::Integer(n) => format!("{n}"),
        Token::List(tokens) => print_sequence(tokens, escape, ['(', ')'], ' '),
        Token::Map(tokens) => print_sequence(tokens, escape, ['{', '}'], ' '),
        Token::Nil => "nil".into(),
        Token::Real(n) => format!("{n}"),
        Token::String(s) => {
            if let Some(t) = s.strip_prefix("\u{29e}") {
                format!(":{t}")
            } else if escape {
                format!("\"{}\"", escape_string(&s))
            } else {
                s
            }
        }
        Token::Symbol(s) => s,
        Token::Vector(tokens) => print_sequence(tokens, escape, ['[', ']'], ' '),
    }
}

fn print_sequence(tokens: VecDeque<Token>, escape: bool, braces: [char; 2], sep: char) -> String {
    let mut s = braces[0].to_string();
    for token in tokens {
        s.push_str(&print(token, escape));
        s.push(sep);
    }
    s = s.trim_end_matches(sep).to_string();
    s.push(braces[1]);
    s
}

fn escape_string(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\n' => "\\n".to_string(),
            '\\' => "\\\\".to_string(),
            c => c.to_string(),
        })
        .collect()
}
