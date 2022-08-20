#[derive(Debug)]
pub enum ParseError {
    SyntaxError(usize, &'static str),
    UnexpectedEOF
}