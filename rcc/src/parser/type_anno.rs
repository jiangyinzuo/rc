use crate::{Parse, ParseContext};
use lexer::token::*;
use crate::parser::type_anno::TypeAnno::Origin;

#[derive(Debug, PartialEq)]
pub enum  TypeAnno<'a> {
    Origin(&'a str),
    Ref(Box<TypeAnno<'a>>),
    Ptr(Box<TypeAnno<'a>>)
}

impl <'a> Parse<'a> for TypeAnno<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        let tk = cxt.bump_token()?;
        match tk {
            Token::Identifier(s) => Ok(Origin(s)),
            _ => Err("invalid type anno")
        }
    }
}