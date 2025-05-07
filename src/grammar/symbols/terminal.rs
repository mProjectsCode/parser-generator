use std::collections::HashSet;

use super::refs::TerminalRef;
use crate::grammar::{Grammar, StrRepr, generator::CodeFile};

pub trait TerminalLike: StrRepr {
    fn name(&self) -> &str;
    fn first_bytes(&self) -> HashSet<u8>;
    fn gen_inner_code(&self, t_ref: TerminalRef, file: &mut CodeFile) -> Result<(), String>;
    fn result_type(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct ByteTerminal {
    pub name: String,
    pub byte: u8,
    pub result_expr: Option<String>,
    pub result_type: String,
}

impl ByteTerminal {
    pub fn new(name: String, byte: u8, result_expr: Option<String>, result_type: String) -> Self {
        ByteTerminal {
            name,
            byte,
            result_expr,
            result_type,
        }
    }

    pub fn from_char(char: char) -> Self {
        ByteTerminal {
            name: format!("char({})", char),
            byte: char as u8,
            result_expr: Some(format!("'{}'", char)),
            result_type: "char".to_string(),
        }
    }
}

impl TerminalLike for ByteTerminal {
    fn name(&self) -> &str {
        &self.name
    }

    fn first_bytes(&self) -> HashSet<u8> {
        let mut set = HashSet::new();
        set.insert(self.byte);
        set
    }

    fn gen_inner_code(&self, _t_ref: TerminalRef, file: &mut CodeFile) -> Result<(), String> {
        file.push_line(format!("    let next = input.next();"));
        file.push_line(format!("    if next == Some({}) {{", self.byte));
        file.push_line(format!(
            "        return Ok({});",
            self.result_expr.clone().unwrap_or("()".to_string())
        ));
        file.push_line(format!("    }} else {{"));
        file.push_line(format!("        return Err(format!(\"Error parsing {}: Expected {:?} but found {{:?}}\", next));", self.name, self.byte));
        file.push_line(format!("    }}"));

        Ok(())
    }

    fn result_type(&self) -> String {
        self.result_type.clone()
    }
}

impl StrRepr for ByteTerminal {
    fn repr(&self, _grammar: &Grammar) -> String {
        format!("T({})", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct TrieNode {
    pub children: Vec<TrieNode>,
    pub byte: u8,
    pub result: Option<String>,
}

impl TrieNode {
    pub fn new(byte: u8, result: Option<String>) -> Self {
        TrieNode {
            children: Vec::new(),
            byte,
            result,
        }
    }

    fn add_word(&mut self, word: &[u8], result: String) -> Result<(), String> {
        if word.is_empty() {
            return Err(format!(
                "Word already exists in trie, or is prefix of existing word: {:?}",
                word
            ));
        }

        if let Some(node) = self.children.iter_mut().find(|n| n.byte == word[0]) {
            node.add_word(&word[1..], result)
        } else {
            let new_node = if word.len() == 1 {
                TrieNode::new(word[0], Some(result))
            } else {
                let mut new_node = TrieNode::new(word[0], None);
                new_node.add_word(&word[1..], result)?;
                new_node
            };

            self.children.push(new_node);
            Ok(())
        }
    }

    fn gen_match(&self, trie: &TrieTerminal, file: &mut CodeFile) -> Result<(), String> {
        file.push_line(format!("let next = input.next();"));
        file.push_line(format!("match next {{"));

        for child in &self.children {
            file.push_line(format!("    Some({}) => {{", child.byte));
            if let Some(ref result) = child.result {
                file.push_line(format!("        return Ok({});", result));
            } else {
                child.gen_match(trie, file)?;
            }
            file.push_line(format!("    }}"));
        }
        file.push_line(format!("    _ => {{"));
        file.push_line(format!("        return Err(format!(\"Error parsing {}: Expected {:?} but found {{:?}}\", next));", trie.name, trie.words));
        file.push_line(format!("    }}"));
        file.push_line(format!("}}"));

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TrieTerminal {
    pub name: String,
    pub children: Vec<TrieNode>,
    pub words: Vec<Box<[u8]>>,
    pub result_type: String,
}

impl TrieTerminal {
    pub fn new(name: String, result_type: String) -> Self {
        TrieTerminal {
            name,
            children: Vec::new(),
            words: Vec::new(),
            result_type,
        }
    }

    pub fn add_word(&mut self, word: &[u8], result: String) -> Result<(), String> {
        if word.is_empty() {
            return Err(format!("Empty word cannot be added to trie"));
        }
        for child in &mut self.children {
            if child.byte == word[0] {
                child.add_word(&word[1..], result)?;
                self.words.push(word.to_vec().into_boxed_slice());
                return Ok(());
            }
        }
        let mut new_node = TrieNode::new(word[0], None);
        new_node.add_word(&word[1..], result)?;
        self.children.push(new_node);
        self.words.push(word.to_vec().into_boxed_slice());
        Ok(())
    }
}

impl TerminalLike for TrieTerminal {
    fn name(&self) -> &str {
        &self.name
    }

    fn first_bytes(&self) -> HashSet<u8> {
        let mut set = HashSet::new();
        for child in &self.children {
            set.insert(child.byte);
        }
        set
    }

    fn gen_inner_code(&self, _t_ref: TerminalRef, file: &mut CodeFile) -> Result<(), String> {
        file.push_line(format!("    let next = input.next();"));
        file.push_line(format!("    match next {{"));
        for child in &self.children {
            file.push_line(format!("        Some({}) => {{", child.byte));
            if let Some(ref result) = child.result {
                file.push_line(format!("            return Ok({});", result));
            } else {
                child.gen_match(&self, file)?;
            }
            file.push_line(format!("        }}"));
        }
        file.push_line(format!("        _ => {{"));
        file.push_line(format!("            return Err(format!(\"Error parsing {}: Expected {:?} but found {{:?}}\", next));", self.name, self.words));
        file.push_line(format!("        }}"));
        file.push_line(format!("    }}"));

        Ok(())
    }

    fn result_type(&self) -> String {
        self.result_type.clone()
    }
}

impl StrRepr for TrieTerminal {
    fn repr(&self, _grammar: &Grammar) -> String {
        format!("Trie({})", self.name)
    }
}
