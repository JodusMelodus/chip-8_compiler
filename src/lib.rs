pub mod parser;
pub mod scan;

#[cfg(debug_assertions)]
use std::io::Write;

#[cfg(not(debug_assertions))]
pub fn get_source_file_path() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    args.get(1).expect("No source file").to_string()
}

#[cfg(debug_assertions)]
pub fn get_source_file_path() -> String {
    let mut buffer = String::new();
    print!("Enter path: ");
    std::io::stdout().flush().expect("Failed to flush output");
    let _ = std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_owned()
}
