use crate::ast::AST;
use crate::ir::{IRInst, IR, Operand, IRType};
use crate::rcc::RccError;
use crate::ast::expr::{LitNumExpr, ExprVisit};
use crate::ast::types::TypeLitNum;

pub struct IRBuilder {
    ast: AST,
    ir_output: IR
}

impl IRBuilder {
    pub fn new(ast: AST) -> IRBuilder {
        IRBuilder {
            ast,
            ir_output: IR::new()
        }
    }

    pub(super) fn generate_ir(&mut self) -> IR {
        let mut output = IR::new();
        std::mem::swap(&mut self.ir_output, &mut output);
        output
    }

    fn visit_bin_op(&mut self) {}

    fn visit_lit_num_expr(&mut self, lit_num_expr: &mut LitNumExpr) -> Result<Operand, RccError> {
        let t = lit_num_expr.get_lit_type();
        Ok(match t {
            TypeLitNum::I8 => Operand::Imm(IRType::I8(lit_num_expr.value.parse::<i8>()?)),
            _ => todo!()
        })
    }
}
