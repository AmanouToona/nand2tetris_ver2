use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Parser {
    code: Vec<String>,
    position: usize,
}

impl Parser {
    pub fn new(reader: BufReader<File>) -> Self {
        let mut code = Vec::new();

        for line in reader.lines() {
            let line = match line {
                Ok(s) => s,
                Err(_) => continue,
            };

            // コメントを排除
            let line = line.split("//").next().unwrap().trim();

            if line.is_empty() {
                continue;
            }

            println!("{}", line);

            code.push(String::from(line));
        }

        Self { code, position: 0 }
    }
}
