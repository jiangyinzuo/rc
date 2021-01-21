//! PathExpr -> ::? Identifier (:: Identifier)*
//! ## Examples:
//! `a::b::c`, `::b`
use crate::lexer::token::Token::*;

use crate::ast::expr::PathExpr;
use crate::parser::Parse;
use crate::parser::ParseContext;

const ERR_INVALID_PATH: Result<PathExpr, &'static str> = Err("invalid path");

impl<'a> Parse<'a> for PathExpr<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
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
                        return ERR_INVALID_PATH;
                    }
                    state = State::PathSep;
                }
                Identifier(s) => {
                    if state == State::Segment {
                        return ERR_INVALID_PATH;
                    }
                    state = State::Segment;
                    path_expr.segments.push(s);
                }
                _ => break,
            }
            cxt.bump_token();
        }
        if state == State::Segment {
            Ok(path_expr)
        } else {
            ERR_INVALID_PATH
        }
    }
}
