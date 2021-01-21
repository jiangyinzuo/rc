use crate::ast::expr::Expr;
use crate::ast::expr::{BlockExpr, LitExpr};
use crate::ast::file::File;
use crate::ast::item::{Item, ItemFn};
use crate::ir::{BasicBlock, Data, IRGen, IRGenContext, Quad};

impl<'a> IRGen for File<'a> {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String>{
        for item in self.items.as_slice() {
            item.generate(cxt)?;
        }
        Ok(())
    }
}

impl<'a> IRGen for Item<'a> {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        match self {
            Self::Fn(item_fn) => item_fn.generate(cxt)?,
            _ => unimplemented!(),
        };
        Ok(())
    }
}

impl<'a> IRGen for ItemFn<'a> {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        let name = self.ident;
        if let Some(block_expr) = self.fn_block.as_ref() {
            block_expr.generate(cxt);
            if let Some(ret_data) = cxt.pop_data() {
                if self.ret_type != ret_data._type {
                    return Err(format!(
                        "invalid type: expect {}, found {}",
                        self.ret_type, ret_data._type
                    ));
                }
                let quad = Quad::ret(ret_data);
                cxt.push_quad(quad);
            } else {
                unreachable!("no data at data_stack");
            }

            let base_block = BasicBlock {
                name: name.to_string(),
                quads: cxt.swap_and_get_quads(),
            };
            cxt.add_basic_blocks(base_block);
        }
        Ok(())
    }
}

impl<'a> IRGen for BlockExpr<'a> {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        for expr in self.exprs.as_slice() {
            expr.generate(cxt)?;
        }
        Ok(())
    }
}

impl<'a> IRGen for LitExpr<'a> {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        cxt.push_data(self.to_data());
        Ok(())
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
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        match self {
            Self::Lit(lit_expr) => lit_expr.generate(cxt)?,
            _ => unimplemented!(),
        }
        Ok(())
    }
}
