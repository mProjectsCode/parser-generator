use std::collections::HashSet;

use super::{Grammar, NonTerminalRef, Rule, SymbolRef, TerminalRef};

pub fn eps(
    word: &[SymbolRef],
    grammar: &Grammar,
    mut visited: &mut HashSet<NonTerminalRef>,
) -> bool {
    for symbol in word.iter() {
        match symbol {
            SymbolRef::Epsilon => {
                continue;
            }
            SymbolRef::Terminal(_) => {
                return false;
            }
            SymbolRef::NonTerminal(nt_ref) => {
                if visited.contains(nt_ref) {
                    return false;
                }
                visited.insert(nt_ref.clone());

                let nt = symbol.unwrap_as_non_terminal(grammar);

                if !grammar
                    .get_rules_for_non_terminal(nt)
                    .iter()
                    .any(|rule| eps(rule.rhs.as_slice(), grammar, &mut visited))
                {
                    return false;
                }
            }
        }
    }

    true
}

pub fn first(
    word: &[SymbolRef],
    grammar: &Grammar,
    mut visited: &mut HashSet<NonTerminalRef>,
) -> HashSet<TerminalRef> {
    let mut first_set = HashSet::new();

    for symbol in word.iter() {
        match symbol {
            SymbolRef::Epsilon => {
                continue;
            }
            SymbolRef::Terminal(t) => {
                first_set.insert(t.clone());
                break;
            }
            SymbolRef::NonTerminal(nt_ref) => {
                if visited.contains(nt_ref) {
                    break;
                }
                visited.insert(nt_ref.clone());

                let nt = nt_ref.deref(grammar);
                for rule in grammar.get_rules_for_non_terminal(nt) {
                    first_set.extend(first(rule.rhs.as_slice(), grammar, &mut visited));
                }

                if !grammar
                    .get_rules_for_non_terminal(nt)
                    .iter()
                    .any(|rule| eps(rule.rhs.as_slice(), grammar, &mut HashSet::new()))
                {
                    break;
                }
            }
        }
    }

    first_set
}

pub fn follow(
    nt: &NonTerminalRef,
    grammar: &Grammar,
    mut visited: &mut HashSet<NonTerminalRef>,
) -> HashSet<TerminalRef> {
    let mut follow_set = HashSet::new();

    for rule in &grammar.rules {
        if !rule.rhs.iter().any(|s| s.eq_non_terminal(nt)) {
            continue;
        }

        for (i, symbol) in rule.rhs.iter().enumerate() {
            if !symbol.eq_non_terminal(nt) {
                continue;
            }

            let rest = &rule.rhs.0[i + 1..];
            follow_set.extend(first(rest, grammar, &mut HashSet::new()));

            if !visited.contains(&rule.lhs) && eps(rest, grammar, &mut HashSet::new()) {
                visited.insert(rule.lhs.clone());

                follow_set.extend(follow(&rule.lhs, grammar, &mut visited));
            }
        }
    }

    follow_set
}

/// Predict is the set of terminals that can appear at the beginning of a string
/// derived from the rule. It contains at least the FIRST set of the rule's RHS.
/// If the rule's RHS can derive epsilon, it also contains the FOLLOW set of the
/// rule's LHS.
pub fn predict(rule: &Rule, grammar: &Grammar) -> HashSet<TerminalRef> {
    let mut predict = first(rule.rhs.as_slice(), grammar, &mut HashSet::new());

    if eps(rule.rhs.as_slice(), grammar, &mut HashSet::new()) {
        predict.extend(follow(&rule.lhs, grammar, &mut HashSet::new()));
    }

    predict
}
