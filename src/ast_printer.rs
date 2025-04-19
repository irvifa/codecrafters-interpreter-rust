use crate::ast::Expr;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(expr: &Expr) -> String {
        match expr {
            Expr::Literal(value) => format!("{}", value),
            Expr::Grouping(inner) => format!("(group {})", Self::print(inner)),
            Expr::Unary(operator, right) => format!(
                "({} {})",
                operator.to_string_for_parse(),
                Self::print(right)
            ),
            Expr::Binary(left, operator, right) => {
                format!("({} {} {})", operator.to_string_for_parse(), left, right)
            }
        }
    }
}
