use std::io::Read;

#[cfg(not(debug_assertions))]
fn get_source_file_path() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    args.get(1).expect("No source file").to_string()
}

#[cfg(debug_assertions)]
fn get_source_file_path() -> String {
    let mut buffer = String::new();
    let _ = std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_owned()
}

fn main() {
    let source_file_path = get_source_file_path();
    let mut source_file =
        std::fs::File::open(source_file_path).expect("Failed to open source file");
    let mut buffer = String::new();
    source_file
        .read_to_string(&mut buffer)
        .expect("Failed to read source file");

    println!("{buffer}");
}
