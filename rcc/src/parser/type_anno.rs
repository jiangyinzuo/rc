use lexer::token::*;

use crate::ast::type_anno::TypeAnno;
use crate::ast::type_anno::TypeAnno::Origin;
use crate::parser::Parse;
use crate::parser::ParseContext;

impl <'a> Parse<'a> for TypeAnno<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        let tk = cxt.bump_token()?;
        match tk {
            Token::Identifier(s) => Ok(Origin(s)),
            _ => Err("invalid type anno")
        }
    }
}