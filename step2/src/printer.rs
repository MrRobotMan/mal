use crate::token::Token;

pub fn print(token: Token, escape: bool) -> String {
    match token {
        Token::Bool(b) => format!("{b}"),
        Token::List(tokens) => print_sequence(tokens, escape, ['(', ')'], ' '),
        Token::Map(map) => print_sequence(
            map.iter()
                .flat_map(|(k, v)| vec![Token::String(k.clone()), v.clone()])
                .collect(),
            escape,
            ['{', '}'],
            ' ',
        ),
        Token::Nil => "nil".into(),
        Token::Number(n) => format!("{n}"),
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

fn print_sequence(tokens: Vec<Token>, escape: bool, braces: [char; 2], sep: char) -> String {
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
