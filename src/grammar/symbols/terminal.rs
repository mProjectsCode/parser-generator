use crate::grammar::{Grammar, StrRepr};

#[derive(Debug, Clone)]
pub struct Terminal {
    pub name: String,
    pub bytes: Box<[u8]>,
    pub result: Option<String>,
}

impl Terminal {
    pub fn new(name: String, bytes: Box<[u8]>, result: Option<String>) -> Self {
        assert!(bytes.len() > 0, "Terminal must have at least one byte");
        Terminal {
            name,
            bytes,
            result,
        }
    }

    pub fn from_str(str: &str) -> Self {
        let bytes = str.as_bytes().to_vec().into_boxed_slice();
        Terminal {
            name: str.to_string(),
            bytes,
            result: Some(str.to_string()),
        }
    }
}

impl StrRepr for Terminal {
    fn repr(&self, _grammar: &Grammar) -> String {
        format!("T({})", self.name)
    }
}
