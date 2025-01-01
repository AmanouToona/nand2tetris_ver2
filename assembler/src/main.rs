use assembler::modules::arg_parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match arg_parser::parse_filename(&args) {
        Ok(filename) => {
            println!("Filename provided: {}", filename);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
