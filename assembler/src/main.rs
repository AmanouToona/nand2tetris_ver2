use std::io::Write;

use assembler::modules::arg_parser;
use assembler::modules::code;
use assembler::modules::parser;
use assembler::modules::symbol_table::SymbolTable;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let reader = match arg_parser::parse_filename(&args) {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!()
        }
    };

    let mut parser = parser::Parser::new(reader);
    let mut symbol_table = SymbolTable::new();
    let mut address_num = 0;
    while parser.has_more_lines() {
        match parser.instruction_type() {
            Some(parser::InstructionType::Lcommand) => {
                let symbol = match parser.symbol() {
                    Some(symbol) => symbol,
                    None => continue,
                };

                symbol_table.add_entry(symbol.to_string(), address_num);
            }
            _ => address_num += 1,
        }
        parser.advance();
    }
    parser.reset();

    let mut writer = match arg_parser::parse_filename_to_output(&args) {
        Ok(writer) => writer,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!()
        }
    };

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

                let address: usize = if symbol_table.contains(symbol) {
                    symbol_table.get_address(symbol)
                } else {
                    match symbol.parse::<usize>() {
                        Ok(address) => address,
                        Err(_) => {
                            eprintln!("Error: invalid address");
                            panic!()
                        }
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
