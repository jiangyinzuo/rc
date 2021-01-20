use crate::ast::file::File;
use crate::ir::{IRGen, IRGenContext, BaseBlock};
use crate::ast::item::ItemFn;

impl<'a, T> IRGen<T> for File<'a> {
    fn generate(&mut self, cxt: &mut IRGenContext) -> Result<T, &str> {
        Err("")
    }
}

impl<'a> IRGen<BaseBlock> for ItemFn<'a> {
    fn generate(&mut self, cxt: &mut IRGenContext) -> Result<BaseBlock, &str> {
        let name = self.ident;
        let base_block = BaseBlock { name: name.to_string(), quads: vec![] };
        Err("")
    }
}

#[test]
fn test() {
    let mut file = File { items: vec![] };
    let res: Result<i8, &str> = file.generate(&mut IRGenContext {});
}