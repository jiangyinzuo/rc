use crate::ast::expr::{BinOperator, BinOpExpr, BlockExpr};
use crate::ast::expr::Expr::{BinOp, Lit};
use crate::ast::expr::LitExpr;
use crate::ast::item::{FnParam, FnParams, Item, ItemFn};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::stmt::Stmt;
use crate::ast::types::{TypeLit, TypeAnnotation};
use crate::ast::Visibility::Priv;
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
            Ok(Item::Fn(ItemFn::new(
                Priv,
                "main".into(),
                FnParams::new(),
                "i32".into(),
                BlockExpr::new().expr_without_block(Lit(LitExpr {
                    ret_type: TypeLit::I,
                    value: "0".into(),
                })),
            ))),
            Ok(Item::Fn(ItemFn::new(
                Priv,
                "oops".into(),
                FnParams::new(),
                TypeAnnotation::unit(),
                BlockExpr::new(),
            ))),
            Ok(Item::Fn(ItemFn::new(
                Priv,
                "add".into(),
                vec![
                    FnParam::new(
                        Pattern::Identifier(IdentPattern::new_const("a".into())),
                        "i32".into(),
                    ),
                    FnParam::new(
                        Pattern::Identifier(IdentPattern::new_const("b".into())),
                        "i32".into(),
                    ),
                ].into(),
                "i32".into(),
                BlockExpr::new().expr_without_block(BinOp(BinOpExpr::new(
                    "a".into(),
                    BinOperator::Plus,
                    "b".into(),
                ))),
            ))),
        ],
    );
}
