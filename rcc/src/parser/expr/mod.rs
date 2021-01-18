use crate::parser::expr::lit_expr::LitExpr;
use crate::parser::expr::path_expr::PathExpr;

mod lit_expr;
mod path_expr;
mod tests;

pub enum Expr<'a> {
    Path(PathExpr<'a>),
    Lit(LitExpr<'a>),
    UnaryE,
}
