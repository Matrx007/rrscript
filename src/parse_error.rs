#[derive(Debug)]
pub enum ParseError {
    SyntaxError(usize, usize, &'static str),
    UnexpectedEOF
}