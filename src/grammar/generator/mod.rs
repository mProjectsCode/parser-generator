use std::{fs::File, io::Write};

use gen_source::GenSource;

use super::Grammar;

pub mod gen_source;

pub struct CodeFile {
    pub lines: Vec<String>,
}

impl CodeFile {
    pub fn new() -> Self {
        CodeFile { lines: Vec::new() }
    }

    pub fn push_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(self.lines.join("\n").as_bytes())?;
        Ok(())
    }
}

pub struct CodeGenerator {
    pub grammar: Grammar,
}

impl CodeGenerator {
    pub fn new(grammar: Grammar) -> Self {
        CodeGenerator { grammar }
    }

    pub fn generate(&self) -> Result<CodeFile, String> {
        let start = self.grammar.start.expect("Start symbol not set");

        let mut file = CodeFile::new();

        file.push_line("use std::iter::Peekable;".to_string());

        file.push_line(format!("\n// THIS IS A GENERATED PARSER FILE\n"));
        
        for line in self.grammar.repr().split('\n') {
            file.push_line(format!("// {}", line));
        }

        file.push_line(format!(""));

        for t_ref in self.grammar.iter_terminal_refs() {
            t_ref.gen_function(&self.grammar, &mut file)?;
        }

        for nt_ref in self.grammar.iter_non_terminal_refs() {
            nt_ref.gen_function(&self.grammar, &mut file)?;
        }

        file.push_line(format!(
            "pub fn parse(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<{}, String> {{",
            start.deref(&self.grammar).ast_type
        ));
        file.push_line(format!(
            "    {}",
            start.gen_call(&self.grammar, "input".to_string())
        ));
        file.push_line(format!("}}"));

        Ok(file)
    }
}
