use crate::common::{Expr, LiteralValue, Statement, Token, TokenType};
use crate::error::ParserError;

pub struct Parser {
    tokens: Vec<Token>,
    current: i128,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
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
        return self.assignment();
    }
    fn assignment(&mut self) -> Result<Box<Expr>, ParserError> {
        let expr = *self.equality().unwrap();
        if self.match_type(vec![TokenType::Equal]) {
            let operator: Token = self.previous();
            let value = self.assignment().unwrap();
            if let Expr::Var { name } = expr {
                return Ok(Box::new(Expr::Assign { name, value }));
            } else {
                self.error(operator, "Invalid assignment target");
            }
        }
        Ok(Box::new(expr))
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
                                operator,
                                right,
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
                                operator,
                                right,
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
                while self.match_type(vec![TokenType::BitwiseAnd, TokenType::BitwiseOr]) {
                    let operator = self.previous();
                    match self.term() {
                        Ok(right) => {
                            expr = Box::new(Expr::Binary {
                                left: expr,
                                operator,
                                right,
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
                                operator,
                                right,
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
                                operator,
                                right,
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
                    return Ok(Box::new(Expr::Unary { operator, right }));
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        return self.primary();
    }
    fn primary(&mut self) -> Result<Box<Expr>, ParserError> {
        if self.match_type(vec![TokenType::Number, TokenType::String]) {
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
        } else if self.match_type(vec![TokenType::Identifier]) {
            let previous_val = self.previous();
            return Ok(Box::new(Expr::Var { name: previous_val }));
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
            return Ok(self.advance());
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
            new_message.push_str(message.clone());
            self.report(token.line, " at '", new_message.as_str());
        }
    }

    fn report(&self, line: u8, loc: &str, message: &str) {
        println!("[line {0}:{1}] Error: {2}", line, loc, message)
    }
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
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
            | TokenType::Return => {
                return;
            }
            _ => {
                self.advance();
            }
        }
    }
    fn print_statement(&mut self) -> Result<Statement, ParserError> {
        match self.expression() {
            Ok(expr) => {
                if let Err(err) =
                    self.consume(TokenType::Semicolon, "Expected ';' after print statement")
                {
                    return Err(err);
                }
                Ok(Statement::Print { expr: *expr })
            }
            Err(err) => Err(err),
        }
    }
    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        match self.expression() {
            Ok(expr) => {
                if let Err(err) =
                    self.consume(TokenType::Semicolon, "Expected ';' after print statement")
                {
                    return Err(err);
                }
                Ok(Statement::Expr { expr: *expr })
            }
            Err(err) => Err(err),
        }
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        if self.match_type(vec![TokenType::Print]) {
            return self.print_statement();
        } else if self.match_type(vec![TokenType::LeftBrace]) {
            return self.block_statement();
        }
        self.expression_statement()
    }
    fn var_declaration(&mut self) -> Result<Statement, ParserError> {
        let name = self
            .consume(TokenType::Identifier, "Expected variable name")
            .unwrap();
        let mut initialiser: Box<Expr> = Box::new(Expr::Literal {
            value: LiteralValue::None,
        });
        if self.match_type(vec![TokenType::Equal]) {
            initialiser = self.expression().unwrap();
        }
        let _ = self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration",
        );
        return Ok(Statement::Var {
            name,
            expr: *initialiser,
        });
    }
    fn declaration(&mut self) -> Result<Statement, ParserError> {
        if self.match_type(vec![TokenType::Var]) {
            return self.var_declaration();
        }
        return self.statement();
    }
    fn block_statement(&mut self) -> Result<Statement, ParserError> {
        let mut stmt_vec = Vec::new();
        while !self.match_type(vec![TokenType::RightBrace]) {
            let new_stmt = self.declaration().unwrap();
            stmt_vec.push(new_stmt);
        }
        let _ = self.consume(TokenType::RightBrace, "Expect '}' after block.");
        Ok(Statement::Block {
            statements: stmt_vec,
        })
    }
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(error) => {
                    self.error(error.token, &error.message);
                    self.synchronize();
                    return vec![Statement::Expr {
                        expr: Expr::Literal {
                            value: LiteralValue::None,
                        },
                    }];
                }
            }
        }
        statements
    }
}
