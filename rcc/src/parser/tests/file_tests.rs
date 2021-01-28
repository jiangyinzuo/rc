use crate::ast::expr::BlockExpr;
use crate::ast::expr::Expr::Lit;
use crate::ast::expr::LitExpr;
use crate::ast::file::File;
use crate::ast::item::VisItem;
use crate::ast::item::{InnerItem, ItemFn};

use super::parse_input;
use crate::ast::stmt::Stmt;
use crate::ast::types::Type;
use crate::ast::Visibility::Priv;

#[test]
fn file_test() {
    let result = parse_input::<File>("fn pi() -> f64 {3.14f64}");
    let excepted = Ok(File {
        items: vec![VisItem::new(
            Priv,
            InnerItem::Fn(ItemFn::new(
                "pi".into(),
                vec![],
                Type::Identifier("f64".into()),
                vec![Stmt::ExprStmt(Lit(LitExpr {
                    ret_type: "f64".into(),
                    value: "3.14".into(),
                }))]
                .into(),
            )),
        )],
    });
    assert_eq!(excepted, result);
}
