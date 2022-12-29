use regex::Regex;

pub struct Reader {
    tokens: Vec<String>,
    index: usize,
}

impl Reader {
    pub fn next(&mut self) -> String {
        let token = self.tokens[self.index].clone();
        self.index += 1;
        token
    }

    pub fn peek(&self) -> String {
        self.tokens[self.index].clone()
    }
    pub fn tokenize(text: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let reg = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;
        let re = Regex::new(reg)?;
        Ok(Self {
            tokens: re
                .captures_iter(text)
                .filter_map(|c| c.get(1))
                .map(|s| s.as_str().into())
                .collect(),
            index: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_test() {
        let test_input = "(+ 1 2)";
        let reader = Reader::tokenize(test_input).unwrap();
        assert_eq!(reader.tokens, vec!["(", "+", "1", "2", ")"]);
    }
}
