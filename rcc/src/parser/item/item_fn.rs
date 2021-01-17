use crate::parser::Visibility;

pub struct ItemFn<'a> {
    visibility: Visibility,
    ident: &'a str,
}
