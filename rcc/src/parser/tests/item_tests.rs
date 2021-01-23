use crate::ast::expr::BlockExpr;
use crate::ast::expr::Expr::Lit;
use crate::ast::expr::LitExpr;
use crate::ast::item::ItemFn;

use super::parse_input;
use crate::ast::types::Type;

#[test]
fn item_fn_test() {
    let result = parse_input::<ItemFn>("fn main() -> i32 {0}");
    assert_eq!(
        Ok(ItemFn {
            name: "main".into(),
            ret_type: Type::Identifier("i32".into()),
            fn_block: Some(BlockExpr {
                exprs: vec![Lit(LitExpr {
                    ret_type: "i32".into(),
                    value: "0".into()
                })]
            }),
        }),
        result
    );
}
