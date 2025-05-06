use std::iter::Peekable;

// THIS IS A GENERATED PARSER FILE

// Grammar:
//   start: NT(S)
//   terminals: T(a), T(b)
//   non_terminals: NT(S)
//   rules:
//     NT(S) -> T(a) T(b)
//     NT(S) -> T(b) T(a)

// Terminal: a
#[inline]
fn _parse_t_0x00000000(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<String, String> {
    let next = input.next();
    if next == Some(97) {
        return Ok("a".to_string());
    } else {
        return Err(format!("Error parsing a: Expected [97] but found {:?}", next));
    }
}
// Terminal: b
#[inline]
fn _parse_t_0x00000001(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<String, String> {
    let next = input.next();
    if next == Some(98) {
        return Ok("b".to_string());
    } else {
        return Err(format!("Error parsing b: Expected [98] but found {:?}", next));
    }
}
// Non-terminal: S
fn _parse_nt_0x00000000(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<(String, String), String> {
    let next = input.peek();
    // Rule: NT(S) -> T(a) T(b)
    // Predict: {T(a)}
    if next == Some(&97) {
        // Symbol: T(a)
        let res1 = _parse_t_0x00000000(input)?;
        // Symbol: T(b)
        let res2 = _parse_t_0x00000001(input)?;
        return Ok((res1, res2));
    }
    // Rule: NT(S) -> T(b) T(a)
    // Predict: {T(b)}
    if next == Some(&98) {
        // Symbol: T(b)
        let res1 = _parse_t_0x00000001(input)?;
        // Symbol: T(a)
        let res2 = _parse_t_0x00000000(input)?;
        return Ok((res1, res2));
    }
    return Err(format!("Error parsing S: No matching rule found for {:?}", input.peek()));
}
pub fn parse(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<(String, String), String> {
    _parse_nt_0x00000000(input)
}