//! LitExpr -> literal

use crate::lexer::token::LiteralKind::*;
use crate::lexer::token::Token::Literal;

use crate::ast::expr::LitExpr;
use crate::parser::Parse;
use crate::parser::ParseCursor;
use crate::rcc::RccError;
use crate::ast::types::Type;

impl<'a> Parse<'a> for LitExpr {
    fn parse(cxt: &mut ParseCursor<'a>) -> Result<Self, RccError> {
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
                    }.into()
                },
                value: value.to_string(),
            })
        } else {
            Err("invalid lit expr".into())
        }
    }
}
