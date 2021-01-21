//! LitExpr -> literal

use lexer::token::LiteralKind::*;
use lexer::token::Token::Literal;

use crate::ast::expr::LitExpr;
use crate::parser::Parse;
use crate::parser::ParseContext;

impl<'a> Parse<'a> for LitExpr<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        if let Literal { literal_kind, value } = cxt.bump_token()? {
            Ok(LitExpr {
                ret_type: {
                    match literal_kind {
                        Char => "char",
                        String => "&str",
                        Integer { suffix } => {
                            if suffix.is_empty() {
                                if value.parse::<i32>().is_ok() {
                                    "i32"
                                } else if value.parse::<i64>().is_ok() {
                                    "i64"
                                } else {
                                     "i128"
                                }
                            } else {
                                suffix
                            }
                        }
                        Float {suffix} => {
                            if suffix.is_empty() {
                               "f64"
                            } else {
                                suffix
                            }
                        }
                    }
                },
                value,
            })
        } else {
            Err("invalid lit expr")
        }
    }
}
