//! File -> Item File | Item
use crate::ast::file::File;
use crate::parser::ParseCursor;
use crate::ast::item::VisItem;
use crate::parser::Parse;
use crate::rcc::RccError;

impl<'a> Parse<'a> for File {
    fn parse(cxt: &mut ParseCursor<'a>) -> Result<Self,RccError> {
        let mut file = File {items: vec![]};
        while !cxt.is_eof() {
            let item = VisItem::parse(cxt)?;
            file.items.push(item);
        }
        Ok(file)
    }
}
