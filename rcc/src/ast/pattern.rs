#[derive(Debug, PartialEq)]
pub enum Pattern {
    Identifier(IdentifierPattern),
}

#[derive(Debug, PartialEq)]
pub struct IdentifierPattern {
    ident: String,
    is_mut: bool,
}

impl IdentifierPattern {
    pub fn new_mut(ident: String) -> Self {
        IdentifierPattern {
            ident,
            is_mut: true
        }
    }

    pub fn new_const(ident: String) -> Self {
        IdentifierPattern {
            ident,
            is_mut: false
        }
    }
}