use crate::common::{LiteralValue, Token, TokenType};
use crate::utils::{is_alpha, is_alpha_numeric, is_digit};

pub struct Scanner {
    source_code: String,
    tokens: Vec<Token>,
    start: u8,
    curr: u8,
    line: u8,
}
impl Scanner {
    pub fn new(source_code: String) -> Self {
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
    pub fn scan_tokens(&mut self) -> Vec<Token> {
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
            '&' => self.add_token(TokenType::BitwiseAnd, LiteralValue::None),
            '|' => self.add_token(TokenType::BitwiseOr, LiteralValue::None),
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
        let token_type: TokenType = match text {
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
            _ => TokenType::Identifier,
        };
        self.add_token(token_type, LiteralValue::None)
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

    fn add_token(&mut self, token_type: TokenType, literal_value: LiteralValue) {
        let text = self
            .get_substring(self.start as usize, self.curr as usize)
            .unwrap();
        self.tokens.extend([Token::new(
            token_type,
            text.to_string(),
            literal_value,
            self.line,
        )]);
    }
}
