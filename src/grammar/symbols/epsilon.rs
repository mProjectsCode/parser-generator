use crate::grammar::{Grammar, StrRepr};

#[derive(Debug, Clone)]
pub struct Epsilon;

impl Epsilon {
    pub fn new() -> Self {
        Epsilon
    }
}

impl StrRepr for Epsilon {
    fn repr(&self, _grammar: &Grammar) -> String {
        "ε".to_string()
    }
}
