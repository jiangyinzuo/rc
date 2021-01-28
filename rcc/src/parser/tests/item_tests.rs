use crate::ast::expr::{BlockExpr, BinOpExpr, BinOperator};
use crate::ast::expr::Expr::{Lit, BinOp};
use crate::ast::expr::LitExpr;
use crate::ast::item::{FnParam, ItemFn};
use crate::ast::pattern::{IdentifierPattern, Pattern};
use crate::ast::stmt::Stmt;
use crate::ast::types::Type;
use crate::parser::tests::parse_validate;

#[test]
fn item_fn_test() {
    parse_validate(
        vec![
            "fn main() -> i32 {0}",
            "fn oops() {}",
            r##"
                fn add(a: i32, b: i32) -> i32 {
                    a+b
                }
            "##,
        ],
        vec![
            Ok(ItemFn::new(
                "main".into(),
                vec![],
                "i32".into(),
                vec![Stmt::ExprStmt(Lit(LitExpr {
                    ret_type: LitExpr::EMPTY_INT_TYPE.into(),
                    value: "0".into(),
                }))]
                .into(),
            )),
            Ok(ItemFn::new(
                "oops".into(),
                vec![],
                Type::unit(),
                BlockExpr::new(),
            )),
            Ok(ItemFn::new(
                "add".into(),
                vec![
                    FnParam::new(
                        Pattern::Identifier(IdentifierPattern::new_const("a".into())),
                        "i32".into(),
                    ),
                    FnParam::new(
                        Pattern::Identifier(IdentifierPattern::new_const("b".into())),
                        "i32".into(),
                    ),
                ],
                "i32".into(),
                vec![Stmt::ExprStmt(BinOp(BinOpExpr::new(
                    "a".into(), BinOperator::Plus, "b".into()
                )))].into(),
            )),
        ],
    );
}
