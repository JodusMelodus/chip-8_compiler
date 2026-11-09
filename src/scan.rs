pub fn scan(input: &str) -> Option<(&str, Vec<u16>)> {
    let mut parts = input.split_whitespace();
    let mut values = Vec::new();
    let opcode = parts.next()?;
    for v in parts {
        values.push(u16::from_str_radix(v.strip_prefix("0x").unwrap_or(v), 16).ok()?);
    }
    Some((opcode, values))
}
