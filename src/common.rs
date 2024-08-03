use core::fmt;
use std::fmt::Display;
use std::cmp::PartialEq;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // LiteralValues.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // bitwise
    BitwiseAnd,
    BitwiseOr,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}", &self)
    }
}

#[derive(Clone, Debug)]
pub enum LiteralValue {
    None,
    Bool(bool),
    String(String),
    Float(f64),
    Integer(i64),
}
impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            LiteralValue::None => match other {
                LiteralValue::None => true,
                _ => false,
            },
            LiteralValue::String(value) => match other {
                LiteralValue::String(other_value) => value == other_value,
                _ => false,
            },
            LiteralValue::Integer(value) => match other {
                LiteralValue::Integer(other_value) => value == other_value,
                _ => false,
            },
            LiteralValue::Bool(value) => match other {
                LiteralValue::Bool(other_value) => value == other_value,
                _ => false,
            },
            LiteralValue::Float(value) => match other {
                LiteralValue::Float(other_value) => value == other_value,
                _ => false,
            },
        }
    }
}
impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::String(string_val) => {
                write!(f, "{}", string_val)
            }
            LiteralValue::Float(float_val) => {
                write!(f, "{}", float_val)
            }
            LiteralValue::Integer(integer_val) => {
                write!(f, "{}", integer_val)
            }
            LiteralValue::Bool(bool_val) => {
                write!(f, "{}", bool_val)
            }
            LiteralValue::None => {
                write!(f, "")
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal_value: LiteralValue,
    pub line: u8,
}
impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal_value: LiteralValue,
        line: u8,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal_value,
            line,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} {1}", &self.token_type, &self.lexeme,)
    }
}
#[derive(Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
