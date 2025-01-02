use std::fs::File;
use std::{io::BufReader, path::Path};

pub fn parse_filename(args: &[String]) -> Result<BufReader<File>, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = args[1].clone();
    if !Path::new(&filename).exists() {
        return Err("file does not exist");
    }

    let f = match File::open(&filename) {
        Ok(f) => f,
        Err(_) => return Err("cannot open file"),
    };

    Ok(BufReader::new(f))
}
