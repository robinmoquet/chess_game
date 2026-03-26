use std::fmt;

#[derive(Debug)]
pub struct ParseFenError {
    message: &'static str,
}

impl ParseFenError {
    pub fn new(message: &'static str) -> Self {
        ParseFenError { message }
    }
}

impl fmt::Display for ParseFenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse FEN Error: {}", self.message)
    }
}
