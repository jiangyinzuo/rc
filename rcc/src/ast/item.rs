use crate::ast::expr::BlockExpr;

#[derive(Debug, PartialEq)]
pub struct ItemFn<'a> {
    pub ident: &'a str,
    pub ret_type: &'a str,
    pub fn_block: Option<BlockExpr<'a>>
}
