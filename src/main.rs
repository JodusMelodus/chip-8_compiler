use std::io::Read;

use chip8::{
    get_source_file_path,
    parser::{self, ParserError},
};

fn main() -> Result<(), ParserError> {
    let args = std::env::args().collect::<Vec<String>>();
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

    let out_file_path = args.get(2);
    std::fs::write(
        format!("{}.ch8", out_file_path.map_or("out", |v| v)),
        &bytes,
    )
    .expect("Failed to write out.ch8");
    println!("Wrote {} bytes", bytes.len());
    Ok(())
}
