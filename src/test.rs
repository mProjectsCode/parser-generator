use std::iter::Peekable;

// THIS IS A GENERATED PARSER FILE

// Grammar:
//   start: NT(S)
//   terminals: T(char(a)), T(char(b))
//   non_terminals: NT(S)
//   rules:
//     NT(S) -> T(char(a)) T(char(b))
//     NT(S) -> T(char(b)) T(char(a))

// Terminal: char(a)
#[inline]
fn _parse_t_0x00000000(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<char, String> {
    let next = input.next();
    if next == Some(97) {
        return Ok('a');
    } else {
        return Err(format!(
            "Error parsing char(a): Expected 97 but found {:?}",
            next
        ));
    }
}
// Terminal: char(b)
#[inline]
fn _parse_t_0x00000001(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<char, String> {
    let next = input.next();
    if next == Some(98) {
        return Ok('b');
    } else {
        return Err(format!(
            "Error parsing char(b): Expected 98 but found {:?}",
            next
        ));
    }
}
// Non-terminal: S
fn _parse_nt_0x00000000(
    input: &mut Peekable<impl Iterator<Item = u8>>,
) -> Result<(char, char), String> {
    let next = input.peek();
    // Rule: NT(S) -> T(char(a)) T(char(b))
    // Predict: {97}
    if next == Some(&97) {
        // Symbol: T(char(a))
        let res1 = _parse_t_0x00000000(input)?;
        // Symbol: T(char(b))
        let res2 = _parse_t_0x00000001(input)?;
        return Ok((res1, res2));
    }
    // Rule: NT(S) -> T(char(b)) T(char(a))
    // Predict: {98}
    if next == Some(&98) {
        // Symbol: T(char(b))
        let res1 = _parse_t_0x00000001(input)?;
        // Symbol: T(char(a))
        let res2 = _parse_t_0x00000000(input)?;
        return Ok((res1, res2));
    }
    return Err(format!(
        "Error parsing S: No matching rule found for {:?}",
        input.peek()
    ));
}
pub fn parse(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<(char, char), String> {
    _parse_nt_0x00000000(input)
}
