use grammar::{
    Grammar, Rule, Word,
    generator::CodeGenerator,
    symbols::{non_terminal::NonTerminal, refs::SymbolRef, terminal::Terminal},
};
use test::parse;

pub mod grammar;

pub mod test;

fn main() {
    // test_parse();
    generate_parser();
}

fn test_parse() {
    let input = "aa";
    let mut input_iter = input.bytes().peekable();

    match parse(&mut input_iter) {
        Ok(_) => println!("Parsing succeeded"),
        Err(err) => eprintln!("Parsing failed: {}", err),
    }
}

fn generate_parser() {
    // Example usage
    let mut grammar = Grammar::new();
    let a = grammar.add_terminal(Terminal::from_str("a"));
    let b = grammar.add_terminal(Terminal::from_str("b"));
    let s = grammar.add_non_terminal(NonTerminal::new(
        "S".to_string(),
        "(String, String)".to_string(),
    ));
    grammar.add_rule(Rule::new(
        s,
        Word::new(vec![SymbolRef::Terminal(a), SymbolRef::Terminal(b)]),
        "(res1, res2)".to_string(),
    ));
    grammar.add_rule(Rule::new(
        s,
        Word::new(vec![SymbolRef::Terminal(b), SymbolRef::Terminal(a)]),
        "(res1, res2)".to_string(),
    ));
    grammar.set_start(s);

    let generator = CodeGenerator::new(grammar);

    match generator.generate() {
        Ok(file) => {
            file.write_to_file("src/test.rs").unwrap();
            println!("Parser generated successfully!");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
