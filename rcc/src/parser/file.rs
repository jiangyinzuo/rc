//! File -> Item File | Item
use crate::parser::item::Item;
use crate::{Parse, ParseContext};

struct File<'a> {
    items: Vec<Item<'a>>,
}

// impl<'a> Parse for File<'a> {
//     fn parse(cxt: ParseContext<'a>) -> Result<Self, ()> {}
// }
