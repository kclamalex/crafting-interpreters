use std::env;
use std::fs::read_to_string;
use std::io;
use std::path::Path;

mod common;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod utils;

struct Prompt {
    had_error: bool,
}

impl Prompt {
    pub fn new() -> Self {
        Self { had_error: false }
    }
    pub fn main(&mut self) {
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
        let mut scanner: scanner::Scanner = scanner::Scanner::new(source_ref.to_string());
        let tokens: Vec<common::Token> = scanner.scan_tokens();
        let mut parser: parser::Parser = parser::Parser::new(tokens);
        let expressions: Box<common::Expr> = parser.parse();
        let mut interpreter: interpreter::Interpreter = interpreter::Interpreter {};
        let value: common::LiteralValue = interpreter.evaluate(expressions.clone()).unwrap();
        let mut ast_expr_str: String = String::new();
        utils::ast_print(&mut ast_expr_str, expressions.clone());
        println!("{}", ast_expr_str);
        println!("{}", value);
    }
    fn error(&self, line: u8, message: &str) {
        self.report(line, "", message);
    }
    fn report(&self, line: u8, loc: &str, message: &str) {
        println!("[line {0}] Error {1}: {2}", line, loc, message)
    }
}
fn main() {
    let mut prompt: Prompt = Prompt::new();
    prompt.main();
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
