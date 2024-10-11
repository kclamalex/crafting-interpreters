use crate::common::{Expr, LiteralValue, Statement, TokenType};
use crate::environment::Environment;
use crate::error::RuntimeError;
use std::default::Default;

pub struct Interpreter {
    had_error: bool,
    had_runtime_error: bool,
    environment: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            had_error: false,
            had_runtime_error: false,
            environment: Environment::default(),
        }
    }
}
impl Interpreter {
    fn is_equal(&self, left_value: &LiteralValue, right_value: &LiteralValue) -> bool {
        if left_value == right_value {
            return true;
        }
        return false;
    }
    fn is_truthy(&self, literal_value: LiteralValue) -> bool {
        match literal_value {
            LiteralValue::None => false,
            LiteralValue::Bool(value) => value,
            LiteralValue::Integer(value) => {
                if value == 0 {
                    false
                } else {
                    true
                }
            }
            LiteralValue::String(value) => {
                if value == "" {
                    false
                } else {
                    true
                }
            }
            LiteralValue::Float(value) => {
                if value == 0.0 {
                    false
                } else {
                    true
                }
            }
        }
    }
    fn return_error_string(&mut self, operator: TokenType) -> String {
        return format!("Invalid data type for {operator}");
    }

    fn evaluate_binary(
        &mut self,
        left_literal_val: &LiteralValue,
        operator_token_type: &TokenType,
        right_literal_val: &LiteralValue,
    ) -> Result<LiteralValue, RuntimeError> {
        match (left_literal_val, operator_token_type, right_literal_val) {
            (
                LiteralValue::Float(left_value),
                TokenType::Minus,
                LiteralValue::Float(right_value),
            ) => Ok(LiteralValue::Float(left_value - right_value)),
            (
                LiteralValue::Integer(left_value),
                TokenType::Minus,
                LiteralValue::Integer(right_value),
            ) => Ok(LiteralValue::Integer(left_value - right_value)),
            (
                LiteralValue::Float(left_value),
                TokenType::Plus,
                LiteralValue::Float(right_value),
            ) => Ok(LiteralValue::Float(left_value + right_value)),
            (
                LiteralValue::Integer(left_value),
                TokenType::Plus,
                LiteralValue::Integer(right_value),
            ) => Ok(LiteralValue::Integer(left_value + right_value)),
            (
                LiteralValue::String(left_value),
                TokenType::Plus,
                LiteralValue::String(right_value),
            ) => {
                let concat_string: String = format!("{left_value}{right_value}");
                Ok(LiteralValue::String(concat_string))
            }
            (
                LiteralValue::Float(left_value),
                TokenType::Star,
                LiteralValue::Float(right_value),
            ) => Ok(LiteralValue::Float(left_value * right_value)),
            (
                LiteralValue::Integer(left_value),
                TokenType::Star,
                LiteralValue::Integer(right_value),
            ) => Ok(LiteralValue::Integer(left_value * right_value)),
            (
                LiteralValue::Float(left_value),
                TokenType::Slash,
                LiteralValue::Float(right_value),
            ) => Ok(LiteralValue::Float(left_value / right_value)),
            (
                LiteralValue::Integer(left_value),
                TokenType::Slash,
                LiteralValue::Integer(right_value),
            ) => Ok(LiteralValue::Integer(left_value / right_value)),
            (
                LiteralValue::Float(left_value),
                TokenType::Greater,
                LiteralValue::Float(right_value),
            ) => Ok(LiteralValue::Bool(left_value > right_value)),
            (
                LiteralValue::Integer(left_value),
                TokenType::Greater,
                LiteralValue::Integer(right_value),
            ) => Ok(LiteralValue::Bool(left_value > right_value)),
            (
                LiteralValue::Float(left_value),
                TokenType::GreaterEqual,
                LiteralValue::Float(right_value),
            ) => Ok(LiteralValue::Bool(left_value >= right_value)),
            (
                LiteralValue::Integer(left_value),
                TokenType::GreaterEqual,
                LiteralValue::Integer(right_value),
            ) => Ok(LiteralValue::Bool(left_value >= right_value)),
            (
                LiteralValue::Float(left_value),
                TokenType::Less,
                LiteralValue::Float(right_value),
            ) => Ok(LiteralValue::Bool(left_value < right_value)),
            (
                LiteralValue::Integer(left_value),
                TokenType::Less,
                LiteralValue::Integer(right_value),
            ) => Ok(LiteralValue::Bool(left_value < right_value)),
            (
                LiteralValue::Float(left_value),
                TokenType::LessEqual,
                LiteralValue::Float(right_value),
            ) => Ok(LiteralValue::Bool(left_value <= right_value)),
            (
                LiteralValue::Integer(left_value),
                TokenType::LessEqual,
                LiteralValue::Integer(right_value),
            ) => Ok(LiteralValue::Bool(left_value <= right_value)),

            (_, TokenType::BangEqual, _) => Ok(LiteralValue::Bool(
                !self.is_equal(left_literal_val, right_literal_val),
            )),
            (_, TokenType::EqualEqual, _) => Ok(LiteralValue::Bool(
                self.is_equal(left_literal_val, right_literal_val),
            )),
            _ => Err(RuntimeError {
                message: "Invalid operator".to_owned(),
            }),
        }
    }
    fn evaluate_unary(
        &mut self,
        operator_token_type: &TokenType,
        right_literal_val: &LiteralValue,
    ) -> Result<LiteralValue, RuntimeError> {
        match operator_token_type {
            TokenType::Minus => match right_literal_val {
                LiteralValue::Float(value) => Ok(LiteralValue::Float(-value)),
                LiteralValue::Integer(value) => Ok(LiteralValue::Integer(-value)),
                _ => Err(RuntimeError {
                    message: self.return_error_string(TokenType::Minus),
                }),
            },
            TokenType::Bang => Ok(LiteralValue::Bool(
                !self.is_truthy(right_literal_val.clone()),
            )),
            _ => Ok(LiteralValue::None),
        }
    }

    fn evaluate(&mut self, expr: Box<Expr>) -> Result<LiteralValue, RuntimeError> {
        match *expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_literal_val = self.evaluate(left).unwrap();
                let right_literal_val = self.evaluate(right).unwrap();
                self.evaluate_binary(&left_literal_val, &operator.token_type, &right_literal_val)
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => Ok(value),
            Expr::Unary { operator, right } => {
                let right_literal_val: LiteralValue = self.evaluate(right).unwrap();
                self.evaluate_unary(&operator.token_type, &right_literal_val)
            }
            Expr::Var { name } => self.environment.get(name),
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;
                let _ = self.environment.assign(name, value.clone());
                Ok(value)
            }
        }
    }
    fn execute(&mut self, stmt: Statement) {
        match stmt {
            Statement::Print { expr } => {
                let val = self.evaluate(Box::new(expr)).unwrap();
                println!("{}", val);
            }
            Statement::Expr { expr } => {
                self.evaluate(Box::new(expr)).unwrap();
            }
            Statement::Var { name, expr } => {
                let mut val: LiteralValue = LiteralValue::None;
                if let Expr::Literal {
                    value: LiteralValue::None,
                } = expr
                {
                    return;
                }
                val = self.evaluate(Box::new(expr)).unwrap();
                self.environment.define(name, val);
            }
            Statement::Block { statements } => self.execute_block(statements),
        }
    }
    fn execute_block(&mut self, stmts: Vec<Statement>) {
        let previous = self.environment.clone();
        for stmt in stmts {
            self.execute(stmt);
        }
        self.environment = previous
    }
    pub fn interpret(&mut self, stmts: Vec<Statement>) {
        for s in stmts {
            self.execute(s);
        }
    }
}
