use std::io::BufRead;

pub struct LineChecker {
    tokens: Vec<String>,
}

impl LineChecker {
    pub fn new(query: &crate::Query) -> Self {
        Self {
            tokens: query.tokens.clone(),
        }
    }
}

impl crate::Checker for LineChecker {
    fn check_file(&self, _file: &dyn BufRead) {}
    fn end_of_search(&self) -> bool {
        false
    }
    fn check_line(&self, line: &str) -> bool {
        let mut found: bool = true;
        for token in &self.tokens {
            if !line.contains(token) {
                found = false;
                break;
            }
        }
        found
    }
}
