use crate::ast::expr::Expr::{BinOp, LitNum};
use crate::ast::expr::{BinOpExpr, BinOperator, BlockExpr};
use crate::ast::item::{FnParam, FnParams, Item, ItemExternalBlock, ItemFn};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::types::TypeAnnotation;
use crate::ast::Visibility::Priv;
use crate::parser::tests::{parse_input, parse_validate};
use crate::tests::{assert_pretty_fmt_eq, read_from_file};

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
                BlockExpr::new(0).expr_without_block(LitNum(0.into())),
            ))),
            Ok(Item::Fn(ItemFn::new(
                Priv,
                "oops".into(),
                FnParams::new(),
                TypeAnnotation::Unit,
                BlockExpr::new(0),
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
                ]
                .into(),
                "i32".into(),
                BlockExpr::new(0).expr_without_block(BinOp(BinOpExpr::new(
                    "a".into(),
                    BinOperator::Plus,
                    "b".into(),
                ))),
            ))),
        ],
    );
}

fn expected_from_file(file_name: &str) -> String {
    read_from_file(file_name, "./src/parser/tests")
}

#[test]
fn item_external_block_test() {
    let result = parse_input::<ItemExternalBlock>(
        r#"
        extern "C" {
            pub fn foo();
            fn bar(a: i32, b: i32);
        }
    "#,
    );
    let expected = expected_from_file("item_external_block.txt");
    assert_pretty_fmt_eq(&expected, &result.unwrap());
}