use std::io::Write;

use assembler::modules::arg_parser;
use assembler::modules::code;
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

    let mut writer = match arg_parser::parse_filename_to_output(&args) {
        Ok(writer) => writer,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!()
        }
    };

    let mut parser = parser::Parser::new(reader);

    while parser.has_more_lines() {
        match parser.instruction_type() {
            Some(parser::InstructionType::Ccommand) => {
                let comp = match parser.comp() {
                    Some(comp) => code::comp(comp),
                    None => panic!("comp is not found"),
                };

                let dest = match parser.dest() {
                    Some(dest) => code::dest(dest),
                    None => "000",
                };

                let jump = match parser.jump() {
                    Some(jump) => code::jump(jump),
                    None => "000",
                };

                let _ = writer.write(format!("111{}{}{}\n", comp, dest, jump).as_bytes());
            }

            Some(parser::InstructionType::Acommand) => {
                let symbol = match parser.symbol() {
                    Some(symbol) => symbol,
                    None => panic!("symbol is not found"),
                };

                let address = match symbol.parse::<i32>() {
                    Ok(address) => address,
                    Err(_) => {
                        eprintln!("Error: invalid address");
                        panic!()
                    }
                };

                let _ = writer.write(format!("{:016b}\n", address).as_bytes());
            }

            Some(parser::InstructionType::Lcommand) => {}
            _ => panic!("unknown instruction type"),
        }
        parser.advance();
    }
}
