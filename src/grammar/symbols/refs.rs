use super::{
    epsilon::Epsilon,
    non_terminal::NonTerminal,
    terminal::{ByteTerminal, TerminalLike},
};
use crate::grammar::{Grammar, StrRepr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TerminalRef(usize);

impl TerminalRef {
    pub fn new(index: usize) -> Self {
        TerminalRef(index)
    }

    pub fn deref<'a>(&self, grammar: &'a Grammar) -> &'a Box<dyn TerminalLike> {
        grammar
            .get_terminal(self.0)
            .expect("index did not point to a terminal")
    }

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn test_index(&self, grammar: &Grammar) {
        if self.index() >= grammar.terminals.len() {
            panic!("Terminal index out of bounds");
        }
    }
}

impl StrRepr for TerminalRef {
    fn repr(&self, grammar: &Grammar) -> String {
        self.deref(grammar).repr(grammar)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct NonTerminalRef(usize);

impl NonTerminalRef {
    pub fn new(index: usize) -> Self {
        NonTerminalRef(index)
    }

    pub fn deref<'a>(&self, grammar: &'a Grammar) -> &'a NonTerminal {
        grammar
            .get_non_terminal(self.0)
            .expect("index did not point to a non-terminal")
    }

    pub fn repr(&self, grammar: &Grammar) -> String {
        self.deref(grammar).repr(grammar)
    }

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn test_index(&self, grammar: &Grammar) {
        if self.index() >= grammar.terminals.len() {
            panic!("Non-terminal index out of bounds");
        }
    }
}

impl StrRepr for NonTerminalRef {
    fn repr(&self, grammar: &Grammar) -> String {
        self.deref(grammar).repr(grammar)
    }
}

/// A reference to a symbol in the grammar.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SymbolRef {
    Epsilon,
    NonTerminal(NonTerminalRef),
    Terminal(TerminalRef),
}

impl SymbolRef {
    pub fn is_epsilon(&self) -> bool {
        matches!(self, SymbolRef::Epsilon)
    }

    pub fn is_non_terminal(&self) -> bool {
        matches!(self, SymbolRef::NonTerminal(_))
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, SymbolRef::Terminal(_))
    }

    pub fn unwrap_as_epsilon(&self) -> Epsilon {
        match &self {
            SymbolRef::Epsilon => Epsilon::new(),
            _ => panic!("Expected Epsilon, found {:?}", self),
        }
    }

    pub fn unwrap_as_non_terminal<'a>(&self, grammar: &'a Grammar) -> &'a NonTerminal {
        match &self {
            SymbolRef::NonTerminal(nt_ref) => nt_ref.deref(grammar),
            _ => panic!("Expected NonTerminal, found {:?}", self),
        }
    }

    pub fn unwrap_as_terminal<'a>(&self, grammar: &'a Grammar) -> &'a Box<dyn TerminalLike> {
        match &self {
            SymbolRef::Terminal(t_ref) => t_ref.deref(grammar),
            _ => panic!("Expected Terminal, found {:?}", self),
        }
    }

    pub fn eq_terminal(&self, other: &TerminalRef) -> bool {
        match self {
            SymbolRef::Terminal(t_ref) => t_ref == other,
            _ => false,
        }
    }

    pub fn eq_non_terminal(&self, other: &NonTerminalRef) -> bool {
        match self {
            SymbolRef::NonTerminal(nt_ref) => nt_ref == other,
            _ => false,
        }
    }

    pub fn test_index(&self, grammar: &Grammar) {
        match self {
            SymbolRef::Epsilon => {}
            SymbolRef::NonTerminal(nt_ref) => nt_ref.test_index(grammar),
            SymbolRef::Terminal(t_ref) => t_ref.test_index(grammar),
        }
    }
}

impl StrRepr for SymbolRef {
    fn repr(&self, grammar: &Grammar) -> String {
        match self {
            SymbolRef::Epsilon => "ε".to_string(),
            SymbolRef::NonTerminal(nt_ref) => nt_ref.repr(grammar),
            SymbolRef::Terminal(t_ref) => t_ref.repr(grammar),
        }
    }
}
