use std::collections::HashSet;

use symbols::{
    non_terminal::NonTerminal,
    refs::{NonTerminalRef, SymbolRef, TerminalRef},
    terminal::Terminal,
};

pub mod analysis;
pub mod generator;
pub mod symbols;

pub trait StrRepr {
    fn repr(&self, grammar: &Grammar) -> String;
}

impl<T: StrRepr> StrRepr for Vec<T> {
    fn repr(&self, grammar: &Grammar) -> String {
        self.iter()
            .map(|item| item.repr(grammar))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl<T: StrRepr> StrRepr for &[T] {
    fn repr(&self, grammar: &Grammar) -> String {
        self.iter()
            .map(|item| item.repr(grammar))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl<T: StrRepr> StrRepr for HashSet<T> {
    fn repr(&self, grammar: &Grammar) -> String {
        format!(
            "{{{}}}",
            self.iter()
                .map(|item| item.repr(grammar))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// A word is a sequence of indices into the grammar's symbols.
#[derive(Debug, Clone)]
pub struct Word(Vec<SymbolRef>);

impl Word {
    pub fn new(symbols: Vec<SymbolRef>) -> Self {
        Word(symbols)
    }

    pub fn iter(&self) -> std::slice::Iter<SymbolRef> {
        self.0.iter()
    }

    pub fn test_indices(&self, grammar: &Grammar) {
        for symbol in self.0.iter() {
            symbol.test_index(grammar);
        }
    }

    pub fn as_slice(&self) -> &[SymbolRef] {
        &self.0
    }
}

impl StrRepr for Word {
    fn repr(&self, grammar: &Grammar) -> String {
        self.0.repr(grammar)
    }
}

/// A rule indexes into the grammar's symbols.
#[derive(Debug, Clone)]
pub struct Rule {
    pub lhs: NonTerminalRef,
    pub rhs: Word,
    /// The transformation to apply to the result of the rule.
    /// The type of this expression must match the type of the non-terminal.
    /// The symbols in the word are available as `res1`, `res2`, etc.
    pub transform: String,
}

impl Rule {
    pub fn new(lhs: NonTerminalRef, rhs: Word, transform: String) -> Self {
        Rule {
            lhs,
            rhs,
            transform,
        }
    }
    pub fn lhs<'a>(&self, grammar: &'a Grammar) -> &'a NonTerminal {
        self.lhs.deref(grammar)
    }

    pub fn test_indices(&self, grammar: &Grammar) {
        self.lhs.test_index(grammar);
        self.rhs.test_indices(grammar);
    }
}

impl StrRepr for Rule {
    fn repr(&self, grammar: &Grammar) -> String {
        format!(
            "{} -> {}",
            self.lhs(grammar).repr(grammar),
            self.rhs.repr(grammar)
        )
    }
}

pub struct Grammar {
    pub start: Option<NonTerminalRef>,
    pub terminals: Vec<Terminal>,
    pub non_terminals: Vec<NonTerminal>,
    pub rules: Vec<Rule>,
}

impl Grammar {
    pub fn new() -> Self {
        Grammar {
            rules: Vec::new(),
            start: None,
            terminals: Vec::new(),
            non_terminals: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        rule.test_indices(self);

        self.rules.push(rule);
    }

    pub fn add_terminal(&mut self, terminal: Terminal) -> TerminalRef {
        if self.has_symbol_with_name(&terminal.name) {
            panic!("Symbol with name {} already exists", terminal.name);
        }

        self.terminals.push(terminal);
        TerminalRef::new(self.terminals.len() - 1)
    }

    pub fn add_non_terminal(&mut self, non_terminal: NonTerminal) -> NonTerminalRef {
        if self.has_symbol_with_name(&non_terminal.name) {
            panic!("Symbol with name {} already exists", non_terminal.name);
        }

        self.non_terminals.push(non_terminal);
        NonTerminalRef::new(self.non_terminals.len() - 1)
    }

    pub fn set_start(&mut self, start: NonTerminalRef) {
        if !self.has_non_terminal_with_name(&start.deref(self).name) {
            panic!("Start symbol {} does not exist", start.deref(self).name);
        }

        self.start = Some(start);
    }

    pub fn repr(&self) -> String {
        format!(
            "Grammar:\n  start: {}\n  terminals: {}\n  non_terminals: {}\n  rules:\n{}",
            match self.start { 
                Some(nt_ref) => nt_ref.deref(self).repr(self), 
                None => "/".to_string()
            },
            self.terminals
                .iter()
                .map(|t| t.repr(self))
                .collect::<Vec<_>>().join(", "),
            self.non_terminals
                .iter()
                .map(|nt| nt.repr(self))
                .collect::<Vec<_>>().join(", "),
            self.rules.iter().map(|r| format!("    {}", r.repr(self))).collect::<Vec<_>>().join("\n")
        )
    }

    pub fn get_start(&self) -> Option<&NonTerminal> {
        self.start.map(|nt_ref| nt_ref.deref(self))
    }

    pub fn get_terminal(&self, index: usize) -> Option<&Terminal> {
        self.terminals.get(index)
    }

    pub fn get_non_terminal(&self, index: usize) -> Option<&NonTerminal> {
        self.non_terminals.get(index)
    }

    pub fn has_terminal_with_name(&self, name: &str) -> bool {
        self.terminals.iter().any(|t| t.name == name)
    }

    pub fn has_non_terminal_with_name(&self, name: &str) -> bool {
        self.non_terminals.iter().any(|nt| nt.name == name)
    }

    pub fn has_symbol_with_name(&self, name: &str) -> bool {
        self.has_terminal_with_name(name) || self.has_non_terminal_with_name(name)
    }

    pub fn get_rules_for_non_terminal(&self, nt: &NonTerminal) -> Vec<&Rule> {
        self.rules
            .iter()
            .filter(|r| r.lhs(self).name == nt.name)
            .collect()
    }

    pub fn iter_terminals(&self) -> impl Iterator<Item = (&Terminal, TerminalRef)> {
        self.terminals
            .iter()
            .enumerate()
            .map(|(i, t)| (t, TerminalRef::new(i)))
    }

    pub fn iter_non_terminals(&self) -> impl Iterator<Item = (&NonTerminal, NonTerminalRef)> {
        self.non_terminals
            .iter()
            .enumerate()
            .map(|(i, t)| (t, NonTerminalRef::new(i)))
    }

    pub fn iter_terminal_refs(&self) -> impl Iterator<Item = TerminalRef> {
        (0..self.terminals.len()).map(|i| TerminalRef::new(i))
    }

    pub fn iter_non_terminal_refs(&self) -> impl Iterator<Item = NonTerminalRef> {
        (0..self.non_terminals.len()).map(|i| NonTerminalRef::new(i))
    }
}
