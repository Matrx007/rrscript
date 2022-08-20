use std::iter::Peekable;
use std::str::Chars;
use std::option::Option;

use crate::parse_error::ParseError;

pub struct StringEater<'a> {
    pub string: &'a String,
    iterator: Peekable<Chars<'a>>,
    index: usize,
    line_number: usize,
    line_start_indexes: Vec<usize>
}

impl<'a> StringEater <'a> {
    pub fn new(string: &'a String) -> StringEater<'a> {
        StringEater{ string: string, iterator: string.chars().peekable(), index: 0, line_number: 1, line_start_indexes: vec![0] }
    }

    pub fn err(&self, string: &'static str) -> ParseError {
        let offset = match self.line_start_indexes.last() {
            // Exclude \n
            Some(index) => {
                println!("self.index={} index={}", self.index, index);
                self.index - index
            },
            _ => 0
        };

        ParseError::SyntaxError(self.line_number, offset, string)
    }

    fn _read_line(&self, line_number: usize) -> String {
        if line_number > self.line_start_indexes.len() {
            panic!("received line number past read line count");
        }

        let mut line = String::new();
        let mut iter = self.string.chars().skip(self.line_start_indexes[line_number - 1]);

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
        
        line
    }

    pub fn print_err(&self, error: &ParseError) {
        match &error {
            ParseError::SyntaxError(line_number, line_character, string) => {
                if *line_number > self.line_start_indexes.len() {
                    panic!("received line number past read line count");
                }

                println!("Syntax error: {}", string);

                let line_number_str = line_number.to_string();

                println!("{} | {}", line_number_str, self._read_line(*line_number));
                println!("{}^", " ".repeat(line_number_str.len() + 3 + line_character));
            },
            ParseError::UnexpectedEOF => {
                println!("Unexpected end of file")
            }
        }
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

                if c == '\n' {
                    self.line_number = self.line_number + 1;
                    self.line_start_indexes.push(self.index);
                }

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