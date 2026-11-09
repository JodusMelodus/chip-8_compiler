pub fn parse(source_code: String) {
    let lines = source_code
        .split("\n")
        .map(|s| s.split(';').next().unwrap_or(s).trim().to_owned())
        .collect::<Vec<String>>();

    for line in lines {
        let instruction = line
            .split(' ')
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>();

        match instruction.len() {
            0 => continue,
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            _ => panic!("HELP"),
        }
    }
}
