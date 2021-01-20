#[derive(Debug, PartialEq)]
pub enum  TypeAnno<'a> {
    Origin(&'a str),
    Ref(Box<TypeAnno<'a>>),
    Ptr(Box<TypeAnno<'a>>)
}
