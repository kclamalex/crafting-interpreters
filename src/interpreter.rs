use crate::common::{Expr, LiteralValue, TokenType};
use crate::error::InterpreterError;

pub struct Interpreter {}

impl Interpreter {
    fn is_equal(&self, left_value: LiteralValue, right_value: LiteralValue) -> bool {
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
    pub fn evaluate(&mut self, expr: Box<Expr>) -> Result<LiteralValue, InterpreterError> {
        match *expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_literal_val = self.evaluate(left).unwrap();
                let right_literal_val = self.evaluate(right).unwrap();
                match operator.token_type {
                    TokenType::Minus => match (left_literal_val, right_literal_val) {
                        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
                            Ok(LiteralValue::Float(left_value - right_value))
                        }
                        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
                            Ok(LiteralValue::Integer(left_value - right_value))
                        }
                        _ => Err(InterpreterError {
                            message: self.return_error_string(TokenType::Minus),
                        }),
                    },
                    TokenType::Plus => match (left_literal_val, right_literal_val) {
                        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
                            Ok(LiteralValue::Float(left_value + right_value))
                        }
                        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
                            Ok(LiteralValue::Integer(left_value + right_value))
                        }
                        (LiteralValue::String(left_value), LiteralValue::String(right_value)) => {
                            let concat_string: String = format!("{left_value}{right_value}");
                            Ok(LiteralValue::String(concat_string))
                        }
                        _ => Err(InterpreterError {
                            message: self.return_error_string(TokenType::Plus),
                        }),
                    },
                    TokenType::Star => match (left_literal_val, right_literal_val) {
                        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
                            Ok(LiteralValue::Float(left_value * right_value))
                        }
                        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
                            Ok(LiteralValue::Integer(left_value * right_value))
                        }
                        _ => Err(InterpreterError {
                            message: self.return_error_string(TokenType::Star),
                        }),
                    },
                    TokenType::Slash => match (left_literal_val, right_literal_val) {
                        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
                            Ok(LiteralValue::Float(left_value / right_value))
                        }
                        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
                            Ok(LiteralValue::Integer(left_value / right_value))
                        }
                        _ => Err(InterpreterError {
                            message: self.return_error_string(TokenType::Slash),
                        }),
                    },
                    TokenType::Greater => match (left_literal_val, right_literal_val) {
                        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
                            Ok(LiteralValue::Bool(left_value > right_value))
                        }
                        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
                            Ok(LiteralValue::Bool(left_value > right_value))
                        }
                        _ => Err(InterpreterError {
                            message: self.return_error_string(TokenType::Greater),
                        }),
                    },
                    TokenType::GreaterEqual => match (left_literal_val, right_literal_val) {
                        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
                            Ok(LiteralValue::Bool(left_value >= right_value))
                        }
                        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
                            Ok(LiteralValue::Bool(left_value >= right_value))
                        }
                        _ => Err(InterpreterError {
                            message: self.return_error_string(TokenType::GreaterEqual),
                        }),
                    },
                    TokenType::Less => match (left_literal_val, right_literal_val) {
                        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
                            Ok(LiteralValue::Bool(left_value < right_value))
                        }
                        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
                            Ok(LiteralValue::Bool(left_value < right_value))
                        }
                        _ => Err(InterpreterError {
                            message: self.return_error_string(TokenType::Less),
                        }),
                    },
                    TokenType::LessEqual => match (left_literal_val, right_literal_val) {
                        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
                            Ok(LiteralValue::Bool(left_value <= right_value))
                        }
                        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
                            Ok(LiteralValue::Bool(left_value <= right_value))
                        }

                        _ => Err(InterpreterError {
                            message: self.return_error_string(TokenType::LessEqual),
                        }),
                    },
                    TokenType::BangEqual => Ok(LiteralValue::Bool(
                        !self.is_equal(left_literal_val, right_literal_val),
                    )),
                    TokenType::EqualEqual => Ok(LiteralValue::Bool(
                        self.is_equal(left_literal_val, right_literal_val),
                    )),

                    _ => Err(InterpreterError {
                        message: "Invalid operator".to_owned(),
                    }),
                }
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => Ok(value),
            Expr::Unary { operator, right } => {
                let right_literal_val: LiteralValue = self.evaluate(right).unwrap();
                match operator.token_type {
                    TokenType::Minus => match right_literal_val {
                        LiteralValue::Float(value) => Ok(LiteralValue::Float(-value)),
                        LiteralValue::Integer(value) => Ok(LiteralValue::Integer(-value)),
                        _ => Err(InterpreterError {
                            message: self.return_error_string(TokenType::Minus),
                        }),
                    },
                    TokenType::Bang => Ok(LiteralValue::Bool(!self.is_truthy(right_literal_val))),
                    _ => Ok(LiteralValue::None),
                }
            }
        }
    }
}
