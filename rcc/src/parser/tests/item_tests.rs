use crate::ast::expr::BlockExpr;
use crate::ast::expr::Expr::Lit;
use crate::ast::expr::LitExpr;
use crate::ast::item::ItemFn;
use crate::ast::stmt::Stmt;
use crate::ast::types::Type;
use crate::parser::tests::parse_validate;

#[test]
fn item_fn_test() {
    parse_validate(
        vec!["fn main() -> i32 {0}", "fn oops() {}"],
        vec![
            Ok(ItemFn::new(
                "main".into(),
                "i32".into(),
                vec![Stmt::ExprStmt(Lit(LitExpr {
                    ret_type: LitExpr::EMPTY_INT_TYPE.into(),
                    value: "0".into(),
                }))]
                .into(),
            )),
            Ok(ItemFn::new("oops".into(), Type::unit(), BlockExpr::new())),
        ],
    );
}
