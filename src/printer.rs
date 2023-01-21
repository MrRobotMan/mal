use std::rc::Rc;

use crate::types::MalVal;

pub fn pr_str(val: &MalVal, print_readably: bool) -> String {
    use MalVal::*;
    match val {
        Nil => String::from("nil"),
        Bool(v) => format!("{v}"),
        Int(i) => format!("{i}"),
        Str(s) => {
            if let Some(keyword) = s.strip_prefix('\u{29e}') {
                format!(":{keyword}",)
            } else if print_readably {
                format!("\"{}\"", escape(s))
            } else {
                s.clone()
            }
        }
        Sym(s) => s.clone(),
        List(val) => pr_list(val, ('(', ')'), print_readably),
        Vector(val) => pr_list(val, ('[', ']'), print_readably),
        Hashmap(val) => pr_list(val, ('{', '}'), print_readably),
        Func(f) => format!("<fn {f:?}>"),
    }
}

fn pr_list(val: &Rc<Vec<MalVal>>, braces: (char, char), print_readably: bool) -> String {
    let formatted = val
        .iter()
        .map(|val| pr_str(val, print_readably))
        .collect::<Vec<String>>();
    format!("{}{}{}", braces.0, formatted.join(" "), braces.1)
}

fn escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            '"' => "\\\"".to_string(),
            _ => c.to_string(),
        })
        .collect::<String>()
}
