use std::iter::Peekable;
use std::str::Chars;
use std::option::Option;

use crate::parse_error::ParseError;

pub struct StringEater<'a> {
    pub string: &'a String,
    iterator: Peekable<Chars<'a>>,
    index: usize
}

impl<'a> StringEater <'a> {
    pub fn new(string: &'a String) -> StringEater<'a> {
        StringEater{ string: string, iterator: string.chars().peekable(), index: 0 }
    }

    pub fn err(&self, string: &'static str) -> ParseError {
        ParseError::SyntaxError(self.index, string)
    }

    fn _read_line(&self, index: usize) -> (String, usize, usize) {
        if index > self.string.len() {
            panic!("received character index past string length");
        }

        let mut line = String::new();

        let mut line_start_index: usize = 0;
        let mut line_number: usize = 0;
        
        // Find line start
        {
            let mut iter = self.string.chars();
            let mut lines: Vec<usize> = Vec::new();
            let mut c: usize = 0;
            loop {
                match iter.next() {
                    Some('\n') => {
                        if c > index {
                            break;
                        }

                        match iter.next() {
                            Some(_) => {
                                c += 1;
                                lines.push(c);
                                line_number += 1;
                            },
                            _ => return (String::new(), line_number, c)
                        }

                    },
                    Some(_) => (),
                    _ => break
                }
                c += 1;
            }

            if lines.len() > 0 {
                line_start_index = *lines.last().unwrap();
            }
        }

        // Read line
        let mut iter = self.string.chars().skip(line_start_index);
        let mut read: Option<char>;
        loop {
            read = iter.next();

            match read {
                Some(c) => {
                    if c == '\n' {
                        break;
                    } else {
                        line.push(c);
                    }
                },
                _ => break
            };
        };
        
        (line, line_number, line_start_index)
    }

    pub fn print_err(&self, error: &ParseError) {
        match &error {
            ParseError::SyntaxError(index, string) => {
                if *index > self.string.len() {
                    panic!("received character index past string length");
                }

                println!("Syntax error: {}", string);
                
                let line: String;
                let line_number: usize;
                let line_start_index: usize;

                (line, line_number, line_start_index) = self._read_line(*index);

                let line_number_str = line_number.to_string();

                println!("{} | {}", line_number_str, line);
                println!("{}^", " ".repeat(line_number_str.len() + 3 + (index - line_start_index)));
            },
            ParseError::UnexpectedEOF => {
                println!("Unexpected end of file")
            }
        }
    }

    pub fn backup(&self) -> usize {
        self.index
    }

    pub fn restore(&mut self, backup: usize) {
        self.index = backup;
        self.iterator = self.string.chars().peekable();

        if backup > 0 {
            self.iterator.nth(backup-1);
        }
    }

    pub fn begin_token(&self) -> (usize, usize) {
        (self.index, 0)
    }

    pub fn end_token(&self, token: &mut (usize, usize)) {
        token.1 = self.index;
    }

    pub fn trim(&mut self) -> bool {
        let mut trimmed = false;
        loop {
            match self.peek() {
                Ok(c) if c == ' ' || c == '\n' => {
                    let _ = self.next();

                    trimmed = true;
                },
                _ => break
            }
        }

        trimmed
    }

    pub fn eat(&mut self, expected: char) -> bool {
        match self.peek() {
            Ok(c) if c == expected => {
                let _ = self.next();
                true
            },
            _ => false
        }
    }

    pub fn next(&mut self) -> Result<char, ParseError> {
        match self.iterator.next() {
            Some(c) => {
                self.index = self.index + 1;

                Ok(c)
            },
            _ => Err(ParseError::UnexpectedEOF)
        }
    }

    pub fn peek(&mut self) -> Result<char, ParseError> {
        match self.iterator.peek() {
            Some(c) => Ok(*c),
            _ => Err(ParseError::UnexpectedEOF)
        }
    }

    // TODO: Replace with macros: coming_up!('a'..='z' | 'A'..='Z' | '_')
    pub fn next_is_word(&mut self) -> Result<bool, ParseError> {
        match self.peek() {
            Ok(c) => match c {
                'a'..='z' | 'A'..='Z' | '_' => Ok(true),
                _ => Ok(false)
            },
            _ => Err(ParseError::UnexpectedEOF)
        }
    }

    pub fn next_is_integer(&mut self) -> Result<bool, ParseError> {
        match self.peek() {
            Ok(c) => match c {
                '0'..='9' => Ok(true),
                _ => Ok(false)
            },
            _ => Err(ParseError::UnexpectedEOF)
        }
    }

    pub fn read_word(&mut self) -> Result<String, ParseError> {
        let mut out = String::new();
        let mut first = false;

        loop {
            let found = match self.peek() {
                Ok(c) => c,
                Err(ParseError::UnexpectedEOF) => return Ok(out),
                Err(e) => return Err(e) 
            };
            
            if first {
                match found {
                    'a'..='z' | 'A'..='Z' | '_' => out.push(found),
                    _ => return Ok(out)
                }
            } else {
                match found {
                    '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => out.push(found),
                    _ => return Ok(out)
                }
            }

            self.next()?;
            first = false;
        }
    }

    pub fn read_integer(&mut self) -> Result<u64, ParseError> {
        let mut out = String::new();

        loop {
            let found = match self.peek() {
                Ok(c) => c,
                Err(ParseError::UnexpectedEOF) => return Ok(out.parse::<u64>().unwrap()),
                Err(e) => return Err(e)
            };

            match found {
                '0'..='9' => out.push(found),
                _ => return Ok(out.parse::<u64>().unwrap())
            };

            self.next()?;
        }
    }

    pub fn read_integer_hex(&mut self) -> Result<u64, ParseError> {
        let mut out = String::new();

        loop {
            let found = match self.peek() {
                Ok(c) => c,
                Err(ParseError::UnexpectedEOF) => return Ok(u64::from_str_radix(&out, 16).unwrap()),
                Err(e) => return Err(e)
            };

            match found {
                '0'..='9' | 'a'..='f' => out.push(found),
                _ => return Ok(u64::from_str_radix(&out, 16).unwrap())
            };

            self.next()?;
        }
    }

    pub fn read_integer_bin(&mut self) -> Result<u64, ParseError> {
        let mut out = String::new();

        loop {
            let found = match self.peek() {
                Ok(c) => c,
                Err(ParseError::UnexpectedEOF) => return Ok(u64::from_str_radix(&out, 2).unwrap()),
                Err(e) => return Err(e)
            };

            match found {
                '0' | '1' => out.push(found),
                _ => return Ok(u64::from_str_radix(&out, 2).unwrap())
            };

            self.next()?;
        }
    }

    pub fn is_word(c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false
        }
    }

    pub fn is_integer(c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_eater1() {
        println!("Testing eat(), trim() and next()");

        let string: String = "Hello\n   world!".to_string();

        let mut eater = StringEater::new(&string);

        assert_eq!(eater.eat('H'), true);
        assert_eq!(eater.eat('e'), true);
        assert_eq!(eater.eat('l'), true);
        assert_eq!(eater.eat('l'), true);
        assert_eq!(eater.eat('a'), false);
        assert_eq!(eater.eat('o'), true);

        assert_eq!(eater.trim(), true);
        
        assert_eq!(eater.eat('w'), true);
        assert_eq!(eater.eat('o'), true);
        assert_eq!(eater.eat('r'), true);
        assert_eq!(eater.eat('l'), true);
        assert_eq!(eater.eat('d'), true);
        assert_eq!(eater.eat('!'), true);

        eater.next().expect_err("Expected end of string");
    }

    #[test]
    fn test_string_eater2() {
        println!("Testing _read_line()");

        let string: String = "Hello\n   world  \nto\nyou!".to_string();

        let mut eater = StringEater::new(&string);

        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");

        assert_eq!(eater.eat('o'), true);

        // Index points to 'r' in "world"
        let index = eater.index;
                
        let line: String;
        let line_number: usize;
        let line_start_index: usize;

        (line, line_number, line_start_index) = eater._read_line(index);

        println!("{}, {}, {}", line, line_number, line_start_index);

        assert_eq!(line, "   world  ");
    }

    #[test]
    fn test_string_eater3() {
        println!("Testing _read_line()");

        let string: String = "Hello\n   world  \nto\nyou!".to_string();

        let mut eater = StringEater::new(&string);

        // Index points to first character
        let index = eater.index;
                
        let line: String;
        let line_number: usize;
        let line_start_index: usize;

        (line, line_number, line_start_index) = eater._read_line(index);

        println!("{}, {}, {}", line, line_number, line_start_index);

        assert_eq!(line, "Hello");
    }

    #[test]
    fn test_string_eater4() {
        println!("Testing _read_line()");

        let string: String = "Hello\n   world  \nto\nyou!".to_string();

        let mut eater = StringEater::new(&string);

        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");

        assert_eq!(eater.eat('u'), true);

        // Index points to last character
        let index = eater.index;
                
        let line: String;
        let line_number: usize;
        let line_start_index: usize;

        (line, line_number, line_start_index) = eater._read_line(index);

        println!("{}, {}, {}", line, line_number, line_start_index);

        assert_eq!(line, "you!");
    }

    #[test]
    fn test_string_eater5() {
        println!("Testing _read_line()");

        let string: String = "Hello\n   world  \nto\nyou!".to_string();

        let mut eater = StringEater::new(&string);

        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");

        assert_eq!(eater.eat(' '), true);
        assert_eq!(eater.eat(' '), true);

        // Index points to '\n' in "\nto"
        let index = eater.index;
                
        let line: String;
        let line_number: usize;
        let line_start_index: usize;

        (line, line_number, line_start_index) = eater._read_line(index);

        println!("{}, {}, {}", line, line_number, line_start_index);

        assert_eq!(line, "to");
    }

    #[test]
    fn test_string_eater6() {
        println!("Testing backup() and restore()");

        let string: String = "Hello\n   world  \nto\nyou!".to_string();

        let mut eater = StringEater::new(&string);

        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");

        let backup = eater.backup();

        assert_eq!(eater.eat('w'), true);

        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");
        eater.next().expect("Expected character");

        assert_eq!(eater.eat(' '), true);
        assert_eq!(eater.eat(' '), true);

        eater.restore(backup);

        assert_eq!(eater.eat('w'), true);

        // Index points to 'o' in "world"
        let index = eater.index;
                
        let line: String;
        let line_number: usize;
        let line_start_index: usize;

        (line, line_number, line_start_index) = eater._read_line(index);

        println!("{}, {}, {}", line, line_number, line_start_index);

        assert_eq!(line, "   world  ");
    }
}