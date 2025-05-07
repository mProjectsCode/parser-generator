use std::iter::Peekable;

// THIS IS A GENERATED PARSER FILE

// Grammar:
//   start: NT(S)
//   terminals: Trie(a)
//   non_terminals: NT(S)
//   rules:
//     NT(S) -> Trie(a)

// Terminal: a
#[inline]
fn _parse_t_0x00000000(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<String, String> {
    let next = input.next();
    match next {
        Some(102) => {
            let next = input.next();
            match next {
                Some(111) => {
                    let next = input.next();
                    match next {
                        Some(111) => {
                            let next = input.next();
                            match next {
                                Some(100) => {
                                    return Ok("food".to_string());
                                }
                                Some(116) => {
                                    return Ok("foot".to_string());
                                }
                                _ => {
                                    return Err(format!(
                                        "Error parsing a: Expected [[102, 111, 111, 100], [102, 111, 111, 116], [102, 111, 114], [98, 97, 114], [98, 97, 122]] but found {:?}",
                                        next
                                    ));
                                }
                            }
                        }
                        Some(114) => {
                            return Ok("for".to_string());
                        }
                        _ => {
                            return Err(format!(
                                "Error parsing a: Expected [[102, 111, 111, 100], [102, 111, 111, 116], [102, 111, 114], [98, 97, 114], [98, 97, 122]] but found {:?}",
                                next
                            ));
                        }
                    }
                }
                _ => {
                    return Err(format!(
                        "Error parsing a: Expected [[102, 111, 111, 100], [102, 111, 111, 116], [102, 111, 114], [98, 97, 114], [98, 97, 122]] but found {:?}",
                        next
                    ));
                }
            }
        }
        Some(98) => {
            let next = input.next();
            match next {
                Some(97) => {
                    let next = input.next();
                    match next {
                        Some(114) => {
                            return Ok("bar".to_string());
                        }
                        Some(122) => {
                            return Ok("baz".to_string());
                        }
                        _ => {
                            return Err(format!(
                                "Error parsing a: Expected [[102, 111, 111, 100], [102, 111, 111, 116], [102, 111, 114], [98, 97, 114], [98, 97, 122]] but found {:?}",
                                next
                            ));
                        }
                    }
                }
                _ => {
                    return Err(format!(
                        "Error parsing a: Expected [[102, 111, 111, 100], [102, 111, 111, 116], [102, 111, 114], [98, 97, 114], [98, 97, 122]] but found {:?}",
                        next
                    ));
                }
            }
        }
        _ => {
            return Err(format!(
                "Error parsing a: Expected [[102, 111, 111, 100], [102, 111, 111, 116], [102, 111, 114], [98, 97, 114], [98, 97, 122]] but found {:?}",
                next
            ));
        }
    }
}
// Non-terminal: S
fn _parse_nt_0x00000000(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<String, String> {
    let next = input.peek();
    // Rule: NT(S) -> Trie(a)
    // Predict: {98, 102}
    if next == Some(&98) || next == Some(&102) {
        // Symbol: Trie(a)
        let res1 = _parse_t_0x00000000(input)?;
        return Ok(res1);
    }
    return Err(format!(
        "Error parsing S: No matching rule found for {:?}",
        input.peek()
    ));
}
pub fn parse(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<String, String> {
    _parse_nt_0x00000000(input)
}
