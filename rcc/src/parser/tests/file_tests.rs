use crate::ast::expr::BlockExpr;
use crate::ast::expr::Expr::LitNum;
use crate::ast::expr::LitNumExpr;
use crate::ast::file::File;
use crate::ast::item::{FnParams, Item, ItemFn};
use crate::ast::types::{TypeLitNum, TypeAnnotation};
use crate::ast::Visibility::Priv;

use super::parse_input;

#[test]
fn file_test() {
    let result = parse_input::<File>("fn pi() -> f64 {3.14f64}");
    let excepted = Ok(File::new().items(vec![Item::Fn(ItemFn::new(
        Priv,
        "pi".into(),
        FnParams::new(),
        TypeAnnotation::Identifier("f64".into()),
        BlockExpr::new().expr_without_block(LitNum(LitNumExpr {
            ret_type: TypeLitNum::F64,
            value: "3.14".into(),
        })),
    ))]));
    assert_eq!(excepted, result);
}
