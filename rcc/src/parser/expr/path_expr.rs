//! PathExpr -> ::? Identifier (:: Identifier)*
//! ## Examples:
//! `a::b::c`, `::b`
use crate::{Parse, ParseContext};
use lexer::token::Token::*;

#[derive(PartialEq, Debug)]
pub struct PathExpr<'a> {
    pub segments: Vec<&'a str>,
}

impl<'a> PathExpr<'a> {
    pub fn new() -> Self {
        PathExpr { segments: vec![] }
    }
}

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
        while let Some(tk) = cxt.next_token() {
            match tk {
                PathSep => {
                    if state == State::PathSep {
                        return ERR_INVALID_PATH;
                    } else if state == State::Init {
                        path_expr.segments.push("::");
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
