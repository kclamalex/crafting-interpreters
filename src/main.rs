use core::fmt;
use std::env;
use std::fmt::Display;
use std::fs::read_to_string;
use std::io;
use std::path::Path;

use phf;

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

#[derive(Clone, Debug, PartialEq)]
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
    BITWISE_AND,
    BITWISE_OR,

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

struct Prompt {
    had_error: bool,
}

impl Prompt {
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
        let mut scanner: Scanner = Scanner::new(source_ref.to_string());
        let tokens: Vec<Token> = sBcanner.scan_tokens();
        let mut parser: Parser = Parser::new(tokens);
        let expressions: Box<Expr> = parser.parse();
        let mut ast_expr_str: String = String::new();
        ast_print(&mut ast_expr_str, expressions);
        println!("{}", ast_expr_str);
    }
    fn error(&self, line: u8, message: &str) {
        self.report(line, "", message);
    }
    fn report(&self, line: u8, loc: &str, message: &str) {
        println!("[line {0}] Error {1}: {2}", line, loc, message)
    }
}

#[derive(Clone, Debug)]
enum LiteralValue {
    None,
    Bool(bool),
    String(String),
    Float(f64),
    Integer(i64),
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
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal_value: LiteralValue,
    line: u8,
}
impl Token {
    fn new(token_type: TokenType, lexeme: String, literal_value: LiteralValue, line: u8) -> Self {
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

struct Scanner {
    source_code: String,
    tokens: Vec<Token>,
    start: u8,
    curr: u8,
    line: u8,
}
impl Scanner {
    fn new(source_code: String) -> Self {
        Scanner {
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
    fn scan_tokens(&mut self) -> Vec<Token> {
        let tokens: Vec<Token>;
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_single_token();
        }
        self.tokens.extend([Token::new(
            TokenType::EOF,
            "".to_string(),
            LiteralValue::None,
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
    fn scan_single_token(&mut self) {
        let c: char = self.next();
        match c {
            '(' => {
                self.add_token(TokenType::LeftParen, LiteralValue::None);
            }
            ')' => {
                self.add_token(TokenType::RightParen, LiteralValue::None);
            }
            '{' => {
                self.add_token(TokenType::LeftBrace, LiteralValue::None);
            }
            '}' => {
                self.add_token(TokenType::RightBrace, LiteralValue::None);
            }
            ',' => {
                self.add_token(TokenType::Comma, LiteralValue::None);
            }
            '.' => {
                self.add_token(TokenType::Dot, LiteralValue::None);
            }
            '-' => {
                self.add_token(TokenType::Minus, LiteralValue::None);
            }
            '+' => {
                self.add_token(TokenType::Plus, LiteralValue::None);
            }
            ';' => {
                self.add_token(TokenType::Semicolon, LiteralValue::None);
            }
            '*' => {
                self.add_token(TokenType::Star, LiteralValue::None);
            }
            '&' => self.add_token(TokenType::BITWISE_AND, LiteralValue::None),
            '|' => self.add_token(TokenType::BITWISE_OR, LiteralValue::None),
            '!' => {
                match self.match_expr('=') {
                    true => self.add_token(TokenType::BangEqual, LiteralValue::None),
                    false => self.add_token(TokenType::Bang, LiteralValue::None),
                };
            }
            '=' => {
                match self.match_expr('=') {
                    true => self.add_token(TokenType::EqualEqual, LiteralValue::None),
                    false => self.add_token(TokenType::Equal, LiteralValue::None),
                };
            }
            '<' => {
                match self.match_expr('=') {
                    true => self.add_token(TokenType::LessEqual, LiteralValue::None),
                    false => self.add_token(TokenType::Less, LiteralValue::None),
                };
            }
            '>' => {
                match self.match_expr('=') {
                    true => self.add_token(TokenType::GreaterEqual, LiteralValue::None),
                    false => self.add_token(TokenType::Greater, LiteralValue::None),
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
                            while !self.is_at_end() {
                                self.next();
                                if self.peek() == '*' && self.peek_next() == '/' {
                                    self.next();
                                    self.next();
                                    break;
                                }
                            }
                        }
                        false => self.add_token(TokenType::Slash, LiteralValue::None),
                    },
                };
            }
            '"' => self.scan_string(),
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}
            _ => {
                if is_digit(c) {
                    self.scan_number()
                } else if is_alpha(c) {
                    self.scan_identifer()
                } else {
                    panic!("Unexpected character.")
                }
            }
        }
    }

    fn scan_identifer(&mut self) {
        while is_alpha_numeric(self.peek()) && !self.is_at_end() {
            self.next();
        }
        let text = self
            .get_substring(self.start as usize, self.curr as usize)
            .unwrap();

        if KEYWORDS.contains_key(text) {
            self.add_token(KEYWORDS.get(text).unwrap().clone(), LiteralValue::None)
        } else {
            self.add_token(TokenType::Identifier, LiteralValue::None)
        }
    }

    fn scan_string(&mut self) {
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
        self.add_token(TokenType::String, LiteralValue::String(text));
    }

    fn scan_number(&mut self) {
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
            LiteralValue::Float(text.parse::<f64>().unwrap()),
        );
    }

    fn add_token(&mut self, token_type: TokenType, LiteralValue: LiteralValue) {
        let text = self
            .get_substring(self.start as usize, self.curr as usize)
            .unwrap();
        self.tokens.extend([Token::new(
            token_type,
            text.to_string(),
            LiteralValue,
            self.line,
        )]);
    }
}

enum Expr {
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

fn ast_print(ast_expr_str: &mut String, expr: Box<Expr>) {
    match *expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            ast_expr_str.push_str("(");
            ast_expr_str.push_str(" ");
            ast_expr_str.push_str(operator.lexeme.as_str());
            ast_print(ast_expr_str, left);
            ast_print(ast_expr_str, right);
            ast_expr_str.push_str(")");
        }
        Expr::Grouping { expression } => {
            ast_expr_str.push_str("(");
            ast_expr_str.push_str(" ");
            ast_expr_str.push_str("group ");
            ast_print(ast_expr_str, expression);
            ast_expr_str.push_str(")");
        }
        Expr::Literal { value } => {
            ast_expr_str.push_str(value.to_string().as_str());
        }
        Expr::Unary { operator, right } => {
            ast_expr_str.push_str("(");
            ast_expr_str.push_str(" ");
            ast_expr_str.push_str(operator.lexeme.as_str());
            ast_print(ast_expr_str, right);
            ast_expr_str.push_str(")");
        }
    }
}

#[derive(Debug, Clone)]
struct ParserError {
    token: Token,
    message: String,
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: i128,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }
    fn match_type(&mut self, token_types: Vec<TokenType>) -> bool {
        for tt in token_types {
            if self.check_type(tt) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn check_type(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn previous(&self) -> Token {
        return self
            .tokens
            .get((self.current - 1) as usize)
            .unwrap()
            .clone();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }
    fn peek(&self) -> &Token {
        return self.tokens.get(self.current as usize).unwrap();
    }
    fn expression(&mut self) -> Result<Box<Expr>, ParserError> {
        return self.equality();
    }
    fn equality(&mut self) -> Result<Box<Expr>, ParserError> {
        match self.comparison() {
            Ok(mut expr) => {
                while self.match_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
                    let operator = self.previous();
                    match self.comparison() {
                        Ok(right) => {
                            expr = Box::new(Expr::Binary {
                                left: expr,
                                operator: operator,
                                right: right,
                            });
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                return Ok(expr);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
    fn comparison(&mut self) -> Result<Box<Expr>, ParserError> {
        match self.bitwise() {
            Ok(mut expr) => {
                while self.match_type(vec![
                    TokenType::Greater,
                    TokenType::GreaterEqual,
                    TokenType::Less,
                    TokenType::LessEqual,
                ]) {
                    let operator = self.previous();
                    match self.bitwise() {
                        Ok(right) => {
                            expr = Box::new(Expr::Binary {
                                left: expr,
                                operator: operator,
                                right: right,
                            });
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                return Ok(expr);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
    fn bitwise(&mut self) -> Result<Box<Expr>, ParserError> {
        match self.term() {
            Ok(mut expr) => {
                while self.match_type(vec![TokenType::BITWISE_AND, TokenType::BITWISE_OR]) {
                    let operator = self.previous();
                    match self.term() {
                        Ok(right) => {
                            expr = Box::new(Expr::Binary {
                                left: expr,
                                operator: operator,
                                right: right,
                            });
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                return Ok(expr);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
    fn term(&mut self) -> Result<Box<Expr>, ParserError> {
        match self.factor() {
            Ok(mut expr) => {
                while self.match_type(vec![TokenType::Minus, TokenType::Plus]) {
                    let operator = self.previous();
                    match self.factor() {
                        Ok(right) => {
                            expr = Box::new(Expr::Binary {
                                left: expr,
                                operator: operator,
                                right: right,
                            });
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                return Ok(expr);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
    fn factor(&mut self) -> Result<Box<Expr>, ParserError> {
        match self.unary() {
            Ok(mut expr) => {
                while self.match_type(vec![TokenType::Slash, TokenType::Star]) {
                    let operator = self.previous();
                    match self.unary() {
                        Ok(right) => {
                            expr = Box::new(Expr::Binary {
                                left: expr,
                                operator: operator,
                                right: right,
                            });
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                return Ok(expr);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
    fn unary(&mut self) -> Result<Box<Expr>, ParserError> {
        if self.match_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            match self.unary() {
                Ok(right) => {
                    return Ok(Box::new(Expr::Unary {
                        operator: operator,
                        right: right,
                    }));
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        return self.primary();
    }
    fn primary(&mut self) -> Result<Box<Expr>, ParserError> {
        if self.match_type(vec![TokenType::Number]) {
            return Ok(Box::new(Expr::Literal {
                value: self.previous().literal_value,
            }));
        } else if self.match_type(vec![TokenType::True]) {
            return Ok(Box::new(Expr::Literal {
                value: LiteralValue::Bool(true),
            }));
        } else if self.match_type(vec![TokenType::False]) {
            return Ok(Box::new(Expr::Literal {
                value: LiteralValue::Bool(false),
            }));
        } else if self.match_type(vec![TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal {
                value: LiteralValue::None,
            }));
        } else if self.match_type(vec![TokenType::LeftParen]) {
            let expr = self.expression().unwrap();
            if let Err(error) = self.consume(TokenType::RightParen, "Expect ')' after expression.")
            {
                return Err(error);
            }
            return Ok(Box::new(Expr::Grouping { expression: expr }));
        } else {
            let message = " Expect expression.";
            return Err(ParserError {
                token: self.peek().clone(),
                message: message.to_string(),
            });
        }
    }
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParserError> {
        if self.check_type(token_type) {
            return Ok((self.advance()));
        }
        return Err(ParserError {
            token: self.peek().clone(),
            message: message.to_string(),
        });
    }
    fn error(&self, token: Token, message: &str) {
        if token.token_type == TokenType::EOF {
            self.report(token.line, " at end", message);
        } else {
            let mut new_message = String::new();
            new_message.push_str(token.lexeme.as_str());
            new_message.push_str("'");
            new_message.push_str(message.clone());
            self.report(token.line, " at '", new_message.as_str());
        }
    }

    fn report(&self, line: u8, loc: &str, message: &str) {
        println!("[line {0}] Error {1}: {2}", line, loc, message)
    }
    fn synchronize(&mut self) {
        self.advance();
        while (!self.is_at_end()) {
            if (self.previous().token_type == TokenType::Semicolon) {
                return;
            }
        }
        match self.peek().token_type {
            TokenType::Class
            | TokenType::Fun
            | TokenType::Var
            | TokenType::For
            | TokenType::If
            | TokenType::While
            | TokenType::Print
            | TokenType::While
            | TokenType::Return => {
                return;
            }
            _ => {
                self.advance();
            }
        }
    }
    fn parse(&mut self) -> Box<Expr> {
        match self.expression() {
            Ok(expr) => {
                return expr;
            }
            Err(error) => {
                self.error(error.token, &error.message);
                self.synchronize();
                return Box::new(Expr::Literal {
                    value: LiteralValue::None,
                });
            }
        }
    }
}

struct Interpreter {}

impl Interpreter {
    fn is_truthy(&self, literal_value: LiteralValue ) -> bool {
        match literal_value {
            LiteralValue::None => false, 
            LiteralValue::Bool(value) => value,
            LiteralValue::Integer(value) => {
                if value == 0 { 
                    false
                } else {
                    true
                }
            },
            LiteralValue::String(value) => {
                if value == "" {
                    false
                } else {
                    true
                }
            },
            LiteralValue::Float(value) => {
                if value == 0.0 {
                    false
                } else {
                    true
                }
            }
        }
    }
    fn evaluate(&mut self, expr: Box<Expr>) -> LiteralValue{
        match *expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_literal_val = self.evaluate(left)
                let right_literal_val = self.evaluate(right)
                }
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => value,
            Expr::Unary { operator, right } => {
                let right_literal_val: LiteralValue = self.evaluate(right);
                match operator.token_type {
                    TokenType::Minus=> {
                        match right_literal_val {
                            LiteralValue::Float( value ) => LiteralValue::Float( -value ),
                            LiteralValue::Integer( value ) => LiteralValue::Integer( -value ),
                            _ => { panic!() }
                        }
                    }
                    TokenType::Bang => {
                        LiteralValue::Bool(!self.is_truthy(right_literal_val))
                    }
                    _ => { LiteralValue::None }
                }
            }
        }
    }
}

fn main() {
    let mut Prompt: Prompt = Prompt::new();
    Prompt.main();
    // let expr = Box::new(Expr::Binary {
    //     left: Box::new(Expr::Unary {
    //         operator: Token::new(TokenType::Minus, "-".to_string(), LiteralValue::None, 1),
    //         right: Box::new(Expr::Literal {
    //             value: LiteralValue::Integer(123),
    //         }),
    //     }),
    //     operator: Token::new(TokenType::Star, "*".to_string(), LiteralValue::None, 1),
    //     right: Box::new(Expr::Grouping {
    //         expression: Box::new(Expr::Literal {
    //             value: LiteralValue::Float(45.67),
    //         }),
    //     }),
    // });
    // let mut ast_str = String::new();
    // ast_print(&mut ast_str, expr);
    // println!("{}", ast_str);
}
