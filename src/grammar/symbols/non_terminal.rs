use crate::grammar::{Grammar, StrRepr};

#[derive(Debug, Clone)]
pub struct NonTerminal {
    pub name: String,
    /// The AST type of the non-terminal
    pub ast_type: String,
}

impl NonTerminal {
    pub fn new(name: String, ast_type: String) -> Self {
        NonTerminal { name, ast_type }
    }
}

impl StrRepr for NonTerminal {
    fn repr(&self, _grammar: &Grammar) -> String {
        format!("NT({})", self.name)
    }
}
