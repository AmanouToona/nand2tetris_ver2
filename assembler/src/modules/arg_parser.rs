use std::fs::File;
use std::{io::BufReader, io::BufWriter, path::Path};

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

pub fn parse_filename_to_output(args: &[String]) -> Result<BufWriter<File>, &'static str> {
    // 与えられたパスの引数を .asm -> .hack に変更して、ファイルを作成する

    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let mut filename = args[1].clone();
    if let Some(pos) = filename.rfind('.') {
        filename.replace_range(pos.., ".hack");
    } else {
        filename.push_str(".hack");
    }
    let f = match File::create(filename) {
        Ok(f) => f,
        Err(_) => return Err("cannot create file"),
    };

    Ok(BufWriter::new(f))
}
