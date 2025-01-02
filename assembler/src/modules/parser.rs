use std::io::BufRead;

#[derive(Debug)]
pub enum InstructionType {
    Acommand,
    Ccommand,
    Lcommand,
}
pub struct Parser {
    code: Vec<String>,
    position: usize,
}

impl Parser {
    pub fn new<R: BufRead>(reader: R) -> Self {
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

            code.push(String::from(line));
        }

        Self { code, position: 0 }
    }

    pub fn has_more_lines(&self) -> bool {
        self.position < self.code.len()
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn instruction_type(&self) -> Option<InstructionType> {
        let line = self.code[self.position].as_str();

        if line.starts_with('@') {
            return Some(InstructionType::Acommand);
        }

        if line.starts_with('(') {
            return Some(InstructionType::Lcommand);
        }

        Some(InstructionType::Ccommand)
    }

    pub fn symbol(&self) -> Option<&str> {
        let res = match self.instruction_type() {
            None => return None,
            Some(InstructionType::Ccommand) => return None,
            Some(InstructionType::Acommand) => &self.code[self.position],
            Some(InstructionType::Lcommand) => &self.code[self.position],
        };

        if res.starts_with('@') {
            return res.strip_prefix('@');
        }

        if res.starts_with('(') {
            if !res.ends_with(')') {
                print!("symbol input format is incorrect");
                return None;
            }
            return Some(&res[1..res.len() - 1]);
        }

        None
    }

    pub fn dest(&self) -> Option<&str> {
        let res = match self.instruction_type() {
            None => return None,
            Some(InstructionType::Ccommand) => &self.code[self.position],
            Some(InstructionType::Acommand) => return None,
            Some(InstructionType::Lcommand) => return None,
        };

        res.find('=')?;

        res.split('=').next()
    }

    pub fn comp(&self) -> Option<&str> {
        let code = match self.instruction_type() {
            Some(InstructionType::Ccommand) => &self.code[self.position],
            _ => return None,
        };

        let code = code.split(';').next().unwrap();

        let left: usize = match code.find('=') {
            None => 0,
            Some(num) => num + 1,
        };

        Some(&code[left..])
    }

    pub fn jump(&self) -> Option<&str> {
        let code = match self.instruction_type() {
            Some(InstructionType::Ccommand) => &self.code[self.position],
            _ => return None,
        };

        code.find(';')?;

        let code = code.split(';').last().unwrap();

        Some(code)
    }

    fn reset(&mut self) {
        self.position = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::io::Cursor;

    #[test]
    fn test_symbol_a_command() {
        let data = "@2\n@R0\n";
        let reader = BufReader::new(Cursor::new(data));
        let mut parser = Parser::new(reader);

        assert_eq!(parser.symbol(), Some("2"));
        parser.advance();
        assert_eq!(parser.symbol(), Some("R0"));
    }

    #[test]
    fn test_symbol_l_command() {
        let data = "(LOOP)\n(END)\n";
        let reader = BufReader::new(Cursor::new(data));
        let mut parser = Parser::new(reader);

        assert_eq!(parser.symbol(), Some("LOOP"));
        parser.advance();
        assert_eq!(parser.symbol(), Some("END"));
    }

    #[test]
    fn test_symbol_c_command() {
        let data = "D=M\nD;JGT\n";
        let reader = BufReader::new(Cursor::new(data));
        let mut parser = Parser::new(reader);

        assert_eq!(parser.symbol(), None);
        parser.advance();
        assert_eq!(parser.symbol(), None);
    }

    #[test]
    fn test_dest() {
        let data = "D=M\nD;JGT\n";
        let reader = BufReader::new(Cursor::new(data));
        let mut parser = Parser::new(reader);

        assert_eq!(parser.dest(), Some("D"));
        parser.advance();
        assert_eq!(parser.dest(), None);
    }

    #[test]
    fn test_comp() {
        let data = "D=M\nD;JGT\n";
        let reader = BufReader::new(Cursor::new(data));
        let mut parser = Parser::new(reader);

        assert_eq!(parser.comp(), Some("M"));
        parser.advance();
        assert_eq!(parser.comp(), Some("D"));
    }

    #[test]
    fn test_jump() {
        let data = "D=M\nD;JGT\n";
        let reader = BufReader::new(Cursor::new(data));
        let mut parser = Parser::new(reader);

        assert_eq!(parser.jump(), None);
        parser.advance();
        assert_eq!(parser.jump(), Some("JGT"));
    }
}
