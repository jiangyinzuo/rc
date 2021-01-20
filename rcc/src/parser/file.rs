//! File -> Item File | Item
use crate::parser::item::Item;
use crate::{Parse, ParseContext};

#[derive(Debug, PartialEq)]
pub struct File<'a> {
    pub items: Vec<Item<'a>>,
}


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
