//! LitExpr -> literal

use lexer::token::LiteralKind;
use lexer::token::Token::Literal;

use crate::ast::expr::LitExpr;
use crate::parser::Parse;
use crate::parser::ParseContext;

impl<'a> Parse<'a> for LitExpr<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        if let Literal { literal_kind, value } = cxt.bump_token()? {
            Ok(LitExpr {
                literal_kind: literal_kind.clone(),
                value,
            })
        } else {
            Err("invalid lit expr")
        }
    }
}
