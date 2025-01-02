use assembler::modules::arg_parser;
use assembler::modules::parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let reader = match arg_parser::parse_filename(&args) {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!()
        }
    };

    let parser = parser::Parser::new(reader);
}
