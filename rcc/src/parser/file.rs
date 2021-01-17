use crate::parser::item::Item;

struct File<'a> {
    items: Vec<Item<'a>>,
}
