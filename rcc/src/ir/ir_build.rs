use crate::ast::AST;

pub struct IRBuilder {
    ast: AST,
}

impl IRBuilder {
    pub fn new(ast: AST) -> IRBuilder {
        IRBuilder {
            ast
        }
    }

    pub fn generate_ir(&self) {

    }
}