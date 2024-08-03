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
pub struct InterpreterError {
    pub message: String,
}
impl fmt::Display for InterpreterError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
