use crate::ast::expr::BlockExpr;
use crate::ast::expr::Expr::Lit;
use crate::ast::expr::LitExpr;
use crate::ast::item::ItemFn;
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
                BlockExpr {
                    exprs: vec![Lit(LitExpr {
                        ret_type: "i32".into(),
                        value: "0".into(),
                    })],
                },
            )),
            Ok(ItemFn::new("oops".into(), Type::unit(), BlockExpr::new())),
        ],
    );
}
