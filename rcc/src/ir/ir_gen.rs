use crate::ast::expr::Expr;
use crate::ast::expr::{BlockExpr, LitNumExpr};
use crate::ast::file::File;
use crate::ast::item::{Item, ItemFn};
use crate::ast::stmt::Stmt;
use crate::ast::types::TypeAnnotation;
use crate::ir::{BasicBlock, Data, IRGen, IRGenContext, Quad};

impl IRGen for File {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        for item in self.items.as_slice() {
            item.generate(cxt)?;
        }
        Ok(())
    }
}

impl IRGen for Item {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        match self {
            Self::Fn(item_fn) => item_fn.generate(cxt)?,
            _ => unimplemented!(),
        };
        Ok(())
    }
}

impl IRGen for ItemFn {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        let name = self.name.clone();
        self.fn_block.generate(cxt);
        if let Some(ret_data) = cxt.pop_data() {
            if self.ret_type != ret_data._type {
                return Err(format!(
                    "invalid type: expect {:#?}, found {:#?}",
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
        Ok(())
    }
}

impl IRGen for Stmt {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        match self {
            Self::ExprStmt(expr) => expr.generate(cxt),
            _ => unimplemented!(),
        }
    }
}

impl IRGen for BlockExpr {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        for expr in self.stmts.as_slice() {
            expr.generate(cxt)?;
        }
        Ok(())
    }
}

impl IRGen for LitNumExpr {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        cxt.push_data(self.to_data());
        Ok(())
    }
}

impl LitNumExpr {
    fn to_data(&self) -> Data {
        Data {
            // FIXME
            _type: TypeAnnotation::Identifier(self.ret_type.to_string()),
            value: self.value.to_string(),
        }
    }
}

impl IRGen for Expr {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String> {
        match self {
            Self::LitNum(lit_expr) => lit_expr.generate(cxt)?,
            _ => unimplemented!(),
        }
        Ok(())
    }
}
