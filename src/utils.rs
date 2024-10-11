use crate::common::Expr;
pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

pub fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

pub fn ast_print(ast_expr_str: &mut String, expr: Box<Expr>) {
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
        Expr::Var { name } => {
            ast_expr_str.push_str("var");
            ast_expr_str.push_str("(");
            ast_expr_str.push_str(&name.lexeme);
            ast_expr_str.push_str(")");
        }
        Expr::Assign { name, value } => {
            ast_expr_str.push_str("var");
            ast_expr_str.push_str("(");
            ast_expr_str.push_str(&name.lexeme);
            ast_expr_str.push_str("=");
            ast_print(ast_expr_str, value);
            ast_expr_str.push_str(")");
        }
    }
}
