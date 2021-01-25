use crate::ast::expr::{Expr, LitExpr};
use crate::parser::tests::parse_validate;

#[test]
#[should_panic]
fn should_panic() {
    parse_validate(vec![";"], vec![Ok(Expr::Lit(LitExpr::lit_i32("0")))]);
}
