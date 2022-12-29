use std::rc::Rc;

use crate::types::MalVal;

pub fn pr_str(val: &MalVal) -> String {
    use MalVal::*;
    match val {
        Nil => String::from("nil"),
        Bool(v) => format!("{v}"),
        Int(i) => format!("{i}"),
        Str(s) => s.clone(),
        Sym(s) => s.clone(),
        List(mv) => pr_list(mv),
    }
}

fn pr_list(val: &Rc<Vec<MalVal>>) -> String {
    let formatted = val.iter().map(|val| pr_str(val)).collect::<Vec<String>>();
    format!("({})", formatted.join(" "))
}
