use rustyline::{
    completion::{Candidate, Completer},
    highlight::Highlighter,
    hint::{Hint, Hinter},
    validate::Validator,
    Helper,
};

pub struct RlHelper {}

impl Completer for RlHelper {
    type Candidate = RlCandidate;
}

impl Helper for RlHelper {}

impl Highlighter for RlHelper {}

impl Hinter for RlHelper {
    type Hint = RlHint;
}

impl Validator for RlHelper {}

pub struct RlCandidate {}

impl Candidate for RlCandidate {
    fn display(&self) -> &str {
        ""
    }

    fn replacement(&self) -> &str {
        ""
    }
}

pub struct RlHint {}

impl Hint for RlHint {
    fn display(&self) -> &str {
        ""
    }

    fn completion(&self) -> Option<&str> {
        None
    }
}
