use std::io::Read;

use chip8_parser::{
    get_source_file_path,
    parser::{self, ParserError},
};

fn main() -> Result<(), ParserError> {
    let source_file_path = get_source_file_path();
    let mut source_file =
        std::fs::File::open(source_file_path).expect("Failed to open source file");
    let mut buffer = String::new();
    source_file
        .read_to_string(&mut buffer)
        .expect("Failed to read source file");

    let words = parser::parse(&buffer)?;
    let bytes = words
        .iter()
        .flat_map(|w| w.to_be_bytes())
        .collect::<Vec<u8>>();

    std::fs::write("out.ch8", &bytes).expect("Failed to write out.ch8");
    println!("Wrote {} bytes", bytes.len());
    Ok(())
}
