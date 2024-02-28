use core::fmt;
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::read_to_string;
use std::io;
use std::path::Path;

use phf::phf_map;

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

#[derive(Clone, Debug)]
enum TokenType {
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
    // Literals.
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

    EOF,
}

static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "fun" => TokenType::Fun,
    "for" => TokenType::For,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}", &self)
    }
}

struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    fn new() -> Self {
        Self { had_error: false }
    }
    fn main(&mut self) {
        let args: Vec<String> = env::args().collect();
        let args_len: u8 = args.len() as u8;
        if args_len > 2 {
            println!("Usage: ownpy [script]")
        } else if args_len == 2 {
            self.run_file(&args[1]);
        } else {
            self.run_prompt();
        }
    }
    fn run_file(&mut self, path: &String) {
        let p: &Path = Path::new(path);
        let source: String;
        if !p.exists() {
            panic!("FileNotFoundError")
        }
        match read_to_string(p) {
            Ok(res) => source = res,
            Err(_) => panic!("Failed to read content"),
        }
        self.run(&source);
        if self.had_error {
            panic!("Failed to run")
        }
    }
    fn run_prompt(&mut self) {
        for line in io::stdin().lines() {
            match line {
                Ok(line_string) => {
                    if line_string.is_empty() {
                        break;
                    }
                    self.run(&line_string);
                    self.had_error = false
                }
                Err(_) => panic!("Failed to run prompt"),
            }
        }
    }

    fn run(&self, source_ref: &String) {
        let mut parser: Parser = Parser::new(source_ref.to_string());
        let tokens: Vec<Token> = parser.parse_tokens();
    }
    fn error(&self, line: u8, message: &str) {
        self.report(line, "", message);
    }
    fn report(&self, line: u8, loc: &str, message: &str) {
        println!("[line {0}] Error {1}: {2}", line, loc, message)
    }
}

#[derive(Clone, Debug)]
enum Literal {
    None,
    String(String),
    Float(f64),
}

#[derive(Clone, Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u8,
}
impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Literal, line: u8) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} {1}", &self.token_type, &self.lexeme,)
    }
}

struct Parser {
    source_code: String,
    tokens: Vec<Token>,
    start: u8,
    curr: u8,
    line: u8,
}
impl Parser {
    fn new(source_code: String) -> Self {
        Parser {
            source_code,
            tokens: Vec::new(),
            start: 0,
            curr: 0,
            line: 1,
        }
    }
    fn is_at_end(&self) -> bool {
        self.curr >= self.source_code.len() as u8
    }
    fn parse_tokens(&mut self) -> Vec<Token> {
        let tokens: Vec<Token>;
        while !self.is_at_end() {
            self.start = self.curr;
            self.parse_single_token();
        }
        self.tokens.extend([Token::new(
            TokenType::EOF,
            "".to_string(),
            Literal::None,
            self.line,
        )]);
        tokens = self.tokens.to_vec();
        println!("{:?}", tokens);
        tokens
    }

    fn get_char(&self, idx: usize) -> Option<char> {
        self.source_code.chars().nth(idx)
    }

    fn get_substring(&self, start: usize, end: usize) -> Option<&str> {
        self.source_code.get(start..end)
    }

    fn match_expr(&mut self, expected: char) -> bool {
        let mut res: bool;
        if self.is_at_end() {
            return false;
        }
        if self.get_char(self.curr as usize).unwrap() != expected {
            return false;
        }
        self.curr += 1;
        true
    }

    fn peek(&mut self) -> char {
        match self.is_at_end() {
            true => '\0',
            false => self.get_char(self.curr as usize).unwrap(),
        }
    }

    fn peek_next(&mut self) -> char {
        if self.curr + 1 >= self.source_code.len() as u8 {
            return '\0';
        }
        self.get_char((self.curr + 1) as usize).unwrap()
    }

    fn next(&mut self) -> char {
        match self.get_char(self.curr as usize) {
            Some(c) => {
                self.curr += 1;
                c
            }
            None => panic!("Empty char"),
        }
    }
    fn parse_single_token(&mut self) {
        let c: char = self.next();
        match c {
            '(' => {
                self.add_token(TokenType::LeftParen, Literal::None);
            }
            ')' => {
                self.add_token(TokenType::RightParen, Literal::None);
            }
            '{' => {
                self.add_token(TokenType::LeftBrace, Literal::None);
            }
            '}' => {
                self.add_token(TokenType::RightBrace, Literal::None);
            }
            ',' => {
                self.add_token(TokenType::Comma, Literal::None);
            }
            '.' => {
                self.add_token(TokenType::Dot, Literal::None);
            }
            '-' => {
                self.add_token(TokenType::Minus, Literal::None);
            }
            '+' => {
                self.add_token(TokenType::Plus, Literal::None);
            }
            ';' => {
                self.add_token(TokenType::Semicolon, Literal::None);
            }
            '*' => {
                self.add_token(TokenType::Star, Literal::None);
            }
            '!' => {
                match self.match_expr('=') {
                    true => self.add_token(TokenType::BangEqual, Literal::None),
                    false => self.add_token(TokenType::Bang, Literal::None),
                };
            }
            '=' => {
                match self.match_expr('=') {
                    true => self.add_token(TokenType::EqualEqual, Literal::None),
                    false => self.add_token(TokenType::Equal, Literal::None),
                };
            }
            '<' => {
                match self.match_expr('=') {
                    true => self.add_token(TokenType::LessEqual, Literal::None),
                    false => self.add_token(TokenType::Less, Literal::None),
                };
            }
            '>' => {
                match self.match_expr('=') {
                    true => self.add_token(TokenType::GreaterEqual, Literal::None),
                    false => self.add_token(TokenType::Greater, Literal::None),
                };
            }
            '/' => {
                match self.match_expr('/') {
                    true => {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.next();
                        }
                    }
                    false => match self.match_expr('*') {
                        true => {
                            while !self.is_at_end()
                            {
                                self.next();
                                if self.peek() == '*' && self.peek_next() == '/'{
                                    self.next();
                                    self.next();
                                    break
                                }
                            }
                        }
                        false => self.add_token(TokenType::Slash, Literal::None),
                    },
                };
            }
            '"' => self.parse_string(),
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}
            _ => {
                if is_digit(c) {
                    self.parse_number()
                } else if is_alpha(c) {
                    self.parse_identifer()
                } else {
                    panic!("Unexpected character.")
                }
            }
        }
    }

    fn parse_identifer(&mut self) {
        while is_alpha_numeric(self.peek()) && !self.is_at_end() {
            self.next();
        }
        let text = self
            .get_substring(self.start as usize, self.curr as usize)
            .unwrap();

        if KEYWORDS.contains_key(text) {
            self.add_token(KEYWORDS.get(text).unwrap().clone(), Literal::None)
        } else {
            self.add_token(TokenType::Identifier, Literal::None)
        }
    }

    fn parse_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.next();
        }
        if self.is_at_end() {
            panic!("Unterminated string.")
        }
        self.next();

        let text = self
            .get_substring((self.start + 1) as usize, (self.curr - 1) as usize)
            .unwrap()
            .to_string();
        self.add_token(TokenType::String, Literal::String(text));
    }

    fn parse_number(&mut self) {
        while is_digit(self.peek()) {
            self.next();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.next();
        }
        while is_digit(self.peek()) {
            self.next();
        }

        let text = self
            .get_substring(self.start as usize, self.curr as usize)
            .unwrap()
            .to_string();

        self.add_token(
            TokenType::Number,
            Literal::Float(text.parse::<f64>().unwrap()),
        );
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = self
            .get_substring(self.start as usize, self.curr as usize)
            .unwrap();
        self.tokens
            .extend([Token::new(token_type, text.to_string(), literal, self.line)]);
    }
}

fn main() {
    let mut interpreter: Interpreter = Interpreter::new();
    interpreter.main();
}
