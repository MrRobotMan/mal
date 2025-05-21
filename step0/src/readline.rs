use rustyline::{
    Editor, Helper, Result,
    completion::{Candidate, Completer},
    highlight::Highlighter,
    hint::{Hint, Hinter},
    history::MemHistory,
    validate::Validator,
};

pub fn readline() -> Result<Editor<RlHelper, MemHistory>> {
    let config = rustyline::config::Builder::new()
        .history_ignore_dups(false)?
        .auto_add_history(true)
        .build();
    let history = MemHistory::new();
    Editor::with_history(config, history)
}

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
