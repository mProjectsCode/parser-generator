use std::collections::HashSet;

use super::CodeFile;
use crate::grammar::{
    Grammar, StrRepr,
    analysis::predict,
    symbols::refs::{NonTerminalRef, SymbolRef, TerminalRef},
};

pub fn index_to_hex(n: usize) -> String {
    format!("{:#010x}", n)
}

pub trait GenSource {
    fn gen_function(&self, grammar: &Grammar, file: &mut CodeFile) -> Result<(), String>;
    fn gen_call(&self, grammar: &Grammar, args: String) -> String;
}

impl GenSource for SymbolRef {
    fn gen_function(&self, grammar: &Grammar, file: &mut CodeFile) -> Result<(), String> {
        match self {
            SymbolRef::Terminal(t) => t.gen_function(grammar, file),
            SymbolRef::NonTerminal(nt) => nt.gen_function(grammar, file),
            SymbolRef::Epsilon => Ok(()),
        }
    }

    fn gen_call(&self, grammar: &Grammar, args: String) -> String {
        match self {
            SymbolRef::Terminal(t) => t.gen_call(grammar, args),
            SymbolRef::NonTerminal(nt) => nt.gen_call(grammar, args),
            SymbolRef::Epsilon => "Ok(())".to_string(),
        }
    }
}

impl GenSource for TerminalRef {
    fn gen_function(&self, grammar: &Grammar, file: &mut CodeFile) -> Result<(), String> {
        let t = self.deref(grammar);

        file.push_line(format!("// Terminal: {}", t.name()));
        file.push_line(format!("#[inline]"));
        file.push_line(format!(
            "fn _parse_t_{}(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<{}, String> {{",
            index_to_hex(self.index()),
            t.result_type()
        ));

        t.gen_inner_code(*self, file)?;

        file.push_line(format!("}}"));

        Ok(())
    }

    fn gen_call(&self, _grammar: &Grammar, args: String) -> String {
        format!("_parse_t_{}({})", index_to_hex(self.index()), args)
    }
}

impl GenSource for NonTerminalRef {
    fn gen_function(&self, grammar: &Grammar, file: &mut CodeFile) -> Result<(), String> {
        let nt = self.deref(grammar);

        file.push_line(format!("// Non-terminal: {}", nt.name));
        file.push_line(format!(
            "fn _parse_nt_{}(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<{}, String> {{",
            index_to_hex(self.index()),
            nt.ast_type,
        ));

        let mut predicts = HashSet::new();

        assert!(
            grammar.get_rules_for_non_terminal(nt).len() > 0,
            "No rules for non-terminal {}",
            nt.name
        );

        file.push_line(format!("    let next = input.peek();"));

        for rule in grammar.get_rules_for_non_terminal(nt) {
            let predict = predict(rule, grammar);
            file.push_line(format!("    // Rule: {}", rule.repr(grammar)));
            file.push_line(format!("    // Predict: {:?}", predict));

            // The predict set for all rules of a non-terminal must be disjoint
            if !predict.is_disjoint(&predicts) {
                return Err(format!(
                    "Grammar is not LL(1): Predict sets for rules of non-terminal {} are not disjoint",
                    nt.name
                ));
            }

            predicts.extend(predict.clone());

            file.push_line(format!(
                "    if {} {{",
                predict
                    .iter()
                    .map(|t| format!("next == Some(&{})", t))
                    .collect::<Vec<_>>()
                    .join(" || ")
            ));

            let mut i = 1;

            for symbol in rule.rhs.iter() {
                if symbol.is_epsilon() {
                    continue;
                }
                file.push_line(format!("        // Symbol: {}", symbol.repr(grammar)));
                file.push_line(format!(
                    "        let res{} = {}?;",
                    i,
                    symbol.gen_call(grammar, "input".to_string())
                ));
                i += 1;
            }

            file.push_line(format!("        return Ok({});", rule.transform));
            file.push_line(format!("    }}"));
        }

        file.push_line(format!("    return Err(format!(\"Error parsing {}: No matching rule found for {{:?}}\", input.peek()));", nt.name));
        file.push_line(format!("}}"));

        Ok(())
    }

    fn gen_call(&self, _grammar: &Grammar, args: String) -> String {
        format!("_parse_nt_{}({})", index_to_hex(self.index()), args)
    }
}
