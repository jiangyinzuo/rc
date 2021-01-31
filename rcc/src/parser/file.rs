//! File -> Item*
use crate::ast::file::File;
use crate::ast::item::Item;
use crate::parser::Parse;
use crate::parser::ParseCursor;
use crate::rcc::RccError;

impl Parse for File {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        let mut file = File::new();
        while !cursor.is_eof() {
            let item = Item::parse(cursor)?;
            file.scope.add_typedef(&item);
            file.items.push(item);
        }
        Ok(file)
    }
}
