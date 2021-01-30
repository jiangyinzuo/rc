use crate::ast::expr::BlockExpr;
use crate::ast::expr::Expr::Lit;
use crate::ast::expr::LitExpr;
use crate::ast::file::File;
use crate::ast::item::{Item, ItemFn, FnParams};
use super::parse_input;
use crate::ast::types::TypeAnnotation;
use crate::ast::Visibility::Priv;

#[test]
fn file_test() {
    let result = parse_input::<File>("fn pi() -> f64 {3.14f64}");
    let excepted = Ok(File {
        items: vec![Item::Fn(ItemFn::new(
            Priv,
            "pi".into(),
            FnParams::new(),
            TypeAnnotation::Identifier("f64".into()),
            BlockExpr::new().expr_without_block(Lit(LitExpr {
                ret_type: "f64".into(),
                value: "3.14".into(),
            })),
        ))],
    });
    assert_eq!(excepted, result);
}
