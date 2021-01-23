//! PathExpr -> ::? Identifier (:: Identifier)*
//! ## Examples:
//! `a::b::c`, `::b`
use crate::lexer::token::Token::*;

use crate::ast::expr::PathExpr;
use crate::parser::Parse;
use crate::parser::ParseCursor;
use crate::rcc::RccError;

impl<'a> Parse<'a> for PathExpr {
    fn parse(cxt: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        #[derive(PartialEq)]
        enum State {
            Init,
            PathSep,
            Segment,
        }

        let mut path_expr = Self::new();
        let mut state = State::Init;
        while let Ok(tk) = cxt.next_token() {
            match tk {
                PathSep => {
                    if state == State::PathSep || state == State::Init {
                        return Err("invalid path".into());
                    }
                    state = State::PathSep;
                }
                Identifier(s) => {
                    if state == State::Segment {
                        return Err("invalid path".into());
                    }
                    state = State::Segment;
                    path_expr.segments.push(s.to_string());
                }
                _ => break,
            }
            cxt.bump_token()?;
        }
        if state == State::Segment {
            Ok(path_expr)
        } else {
            Err("invalid path".into())
        }
    }
}
