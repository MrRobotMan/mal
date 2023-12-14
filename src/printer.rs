use crate::Token;

pub fn pr_str(token: &Token, escaped: bool) -> String {
    match token {
        Token::Symbol(s) => s.clone(),
        Token::Number(n) => format!("{n}"),
        Token::String(s) => {
            if let Some(tail) = s.strip_prefix('\u{29e}') {
                format!(":{tail}")
            } else if escaped {
                format!("\"{}\"", escape(s))
            } else {
                s.clone()
            }
        }
        Token::List(l) => pr_sequence(l, escaped, ('(', ')'), ' '),
        Token::Vector(v) => pr_sequence(v, escaped, ('[', ']'), ' '),
        Token::Map(m) => {
            let flat = m
                .iter()
                .flat_map(|(k, v)| vec![Token::String(k.clone()), v.clone()])
                .collect::<Vec<_>>();
            pr_sequence(&flat, escaped, ('{', '}'), ' ')
        }
        Token::Nil => "nil".into(),
        Token::Bool(b) => format!("{b}"),
    }
}

fn pr_sequence(tokens: &[Token], escaped: bool, braces: (char, char), sep: char) -> String {
    let mut res = braces.0.to_string();
    for token in tokens {
        res.push_str(&pr_str(token, escaped));
        res.push(sep);
    }
    res = res.trim_end_matches(sep).to_string();
    res.push(braces.1);
    res
}

fn escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\n' => "\\n".to_string(),
            '\\' => "\\\\".to_string(),
            c => c.to_string(),
        })
        .collect()
}
