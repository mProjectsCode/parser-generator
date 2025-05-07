use grammar::{
    Grammar, Rule, Word,
    generator::CodeGenerator,
    symbols::{
        non_terminal::NonTerminal,
        refs::SymbolRef,
        terminal::{ByteTerminal, TrieTerminal},
    },
};
use test::parse;

pub mod grammar;

pub mod test;
pub mod test2;

fn main() {
    // test_parse();
    // generate_parser();
    test_parse_2();
    // generate_parser_2();
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
    let a = grammar.add_terminal(ByteTerminal::from_char('a'));
    let b = grammar.add_terminal(ByteTerminal::from_char('b'));
    let s = grammar.add_non_terminal(NonTerminal::new(
        "S".to_string(),
        "(char, char)".to_string(),
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

fn test_parse_2() {
    let input = "foof";
    let mut input_iter = input.bytes().peekable();

    match test2::parse(&mut input_iter) {
        Ok(_) => println!("Parsing succeeded"),
        Err(err) => eprintln!("Parsing failed: {}", err),
    }
}

fn generate_parser_2() {
    // Example usage
    let mut grammar = Grammar::new();

    let mut trie = TrieTerminal::new("a".to_string(), "String".to_string());
    trie.add_word("food".as_bytes(), "\"food\".to_string()".to_string())
        .unwrap();
    trie.add_word("foot".as_bytes(), "\"foot\".to_string()".to_string())
        .unwrap();
    trie.add_word("for".as_bytes(), "\"for\".to_string()".to_string())
        .unwrap();
    trie.add_word("bar".as_bytes(), "\"bar\".to_string()".to_string())
        .unwrap();
    trie.add_word("baz".as_bytes(), "\"baz\".to_string()".to_string())
        .unwrap();
    let a = grammar.add_terminal(trie);

    let s = grammar.add_non_terminal(NonTerminal::new("S".to_string(), "String".to_string()));
    grammar.add_rule(Rule::new(
        s,
        Word::new(vec![SymbolRef::Terminal(a)]),
        "res1".to_string(),
    ));
    grammar.set_start(s);

    let generator = CodeGenerator::new(grammar);

    match generator.generate() {
        Ok(file) => {
            file.write_to_file("src/test2.rs").unwrap();
            println!("Parser generated successfully!");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
