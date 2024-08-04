use crate::common::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub token: Token,
    pub message: String,
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
}
impl fmt::Display for RuntimeError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

