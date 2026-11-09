pub fn scan(input: &str, fmt: &str) -> Option<Vec<u16>> {
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
