use crate::ast::file::File;
use crate::ir::{IRGen, IRGenContext, BasicBlock, Data, Quad};
use crate::ast::item::{Item, ItemFn};
use crate::ast::expr::{BlockExpr, LitExpr};
use crate::ast::expr::Expr;
use crate::ir::Opcode::Ret;

impl<'a> IRGen for File<'a> {
    fn generate(&self, cxt: &mut IRGenContext) {
        for item in self.items.as_slice() {
            item.generate(cxt);
        }
    }
}

impl<'a> IRGen for Item<'a> {
    fn generate(&self, cxt: &mut IRGenContext) {
        match self {
            Self::Fn(item_fn) => item_fn.generate(cxt),
            _ => unimplemented!()
        }
    }
}

impl<'a> IRGen for ItemFn<'a> {
    fn generate(&self, cxt: &mut IRGenContext) {
        let name = self.ident;
        if let Some(mut block_expr) = self.fn_block.as_ref() {
            block_expr.generate(cxt);
            if let Some(ret_data) = cxt.pop_data() {
                let quad = Quad::ret(ret_data);
                cxt.push_quad(quad);
            }
            let base_block = BasicBlock { name: name.to_string(), quads: cxt.swap_and_get_quads() };
            cxt.add_basic_blocks(base_block);
        }
    }
}

impl<'a> IRGen for BlockExpr<'a> {
    fn generate(&self, cxt: &mut IRGenContext) {
        for expr in self.exprs.as_slice() {
            expr.generate(cxt);
        }
    }
}

impl<'a> IRGen for LitExpr<'a> {
    fn generate(&self, cxt: &mut IRGenContext) {
        cxt.push_data(self.to_data())
    }
}

impl<'a> LitExpr<'a> {
    fn to_data(&self) -> Data {
        Data {
            _type: self.ret_type.to_string(),
            value: self.value.to_string(),
        }
    }
}

impl<'a> IRGen for Expr<'a> {
    fn generate(&self, cxt: &mut IRGenContext) {
        match self {
            Self::Lit(lit_expr) => lit_expr.generate(cxt),
            _ => unimplemented!()
        }
    }
}

