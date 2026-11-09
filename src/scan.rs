use crate::parser::ParserError;

pub fn scan(input: &str) -> Result<(&str, Vec<u16>), ParserError> {
    let mut parts = input.split_whitespace();
    let mut values = Vec::new();
    let opcode = parts.next().ok_or(ParserError::MissingOpcode)?;
    for v in parts {
        values.push(u16::from_str_radix(v.strip_prefix("0x").unwrap_or(v), 16)?);
    }
    Ok((opcode, values))
}
