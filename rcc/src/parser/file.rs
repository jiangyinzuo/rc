//! File -> Item File | Item
use crate::ast::file::File;
use crate::parser::ParseContext;
use crate::ast::item::Item;
use crate::parser::Parse;

impl<'a> Parse<'a> for File<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        let mut file = File {items: vec![]};
        while !cxt.is_eof() {
            let item = Item::parse(cxt)?;
            file.items.push(item);
        }
        Ok(file)
    }
}
