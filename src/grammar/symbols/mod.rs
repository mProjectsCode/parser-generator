use epsilon::Epsilon;
use non_terminal::NonTerminal;
use terminal::ByteTerminal;

use super::{Grammar, StrRepr};

pub mod epsilon;
pub mod non_terminal;
pub mod refs;
pub mod terminal;

pub enum Symbol {
    Epsilon(Epsilon),
    NonTerminal(NonTerminal),
    Terminal(ByteTerminal),
}

impl Symbol {
    pub fn is_epsilon(&self) -> bool {
        matches!(self, Symbol::Epsilon(_))
    }

    pub fn is_non_terminal(&self) -> bool {
        matches!(self, Symbol::NonTerminal(_))
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Symbol::Terminal(_))
    }
}

impl StrRepr for Symbol {
    fn repr(&self, grammar: &Grammar) -> String {
        match self {
            Symbol::Epsilon(e) => e.repr(grammar),
            Symbol::NonTerminal(nt) => nt.repr(grammar),
            Symbol::Terminal(t) => t.repr(grammar),
        }
    }
}
