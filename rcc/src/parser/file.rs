//! File -> Item File | Item
use crate::ast::file::File;
use crate::ast::item::VisItem;
use crate::parser::Parse;
use crate::parser::ParseCursor;
use crate::rcc::RccError;

impl Parse for File {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        let mut file = File { items: vec![] };
        while !cursor.is_eof() {
            let item = VisItem::parse(cursor)?;
            file.items.push(item);
        }
        Ok(file)
    }
}
