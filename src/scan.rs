pub fn scanf(input: &str, fmt: &str) -> Option<Vec<u16>> {
    let mut rest = input;
    let mut fmt = fmt;
    let mut out = Vec::new();

    loop {
        let Some(open) = fmt.find('{') else {
            return (fmt == rest).then_some(out);
        };
        rest = rest.strip_prefix(&fmt[..open])?;

        let close = open + fmt[open..].find('}')?;
        fmt = &fmt[close + 1..];

        let end = if fmt.is_empty() {
            rest.len()
        } else {
            let lit_len = fmt.find('{').unwrap_or(fmt.len());
            if lit_len == 0 {
                return None;
            }
            rest.find(&fmt[..lit_len])?
        };
        out.push(rest[..end].parse::<u16>().ok()?);
        rest = &rest[end..];
    }
}

pub fn scan(input: &str) -> Option<(&str, Vec<u16>)> {
    let mut parts = input.split_whitespace();
    let mut values = Vec::new();
    let opcode = parts.next()?;
    for v in parts {
        values.push(v.parse::<u16>().ok()?);
    }
    Some((opcode, values))
}
