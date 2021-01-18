//! LitExpr -> literal

use crate::{Parse, ParseContext};
use lexer::token::LiteralKind;
use lexer::token::Token::Literal;

#[derive(PartialEq, Debug)]
pub struct LitExpr<'a> {
    pub literal_kind: LiteralKind,
    pub value: &'a str,
}

impl<'a> Parse<'a> for LitExpr<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        if let Some(Literal {
            literal_kind,
            value,
        }) = cxt.bump_token()
        {
            Ok(LitExpr {
                literal_kind: literal_kind.clone(),
                value,
            })
        } else {
            Err("invalid literal")
        }
    }
}
