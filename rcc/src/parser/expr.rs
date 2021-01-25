use crate::ast::expr::Expr::{Block, Borrow, Lit, Nothing, Path, Unary, Assign};
use crate::ast::expr::*;
use crate::lexer::token::LiteralKind::*;
use crate::lexer::token::Token;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;

impl<'a> Parse<'a> for Expr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        let lhs = match cursor.next_token()? {
            Token::Not | Token::Star | Token::Minus => Unary(UnAryExpr::parse(cursor)?),
            Token::Identifier(_) | Token::PathSep => Path(PathExpr::parse(cursor)?),
            Token::Literal { .. } => Lit(LitExpr::parse(cursor)?),
            Token::LeftCurlyBraces => Block(BlockExpr::parse(cursor)?),
            Token::And | Token::AndAnd => Borrow(BorrowExpr::parse(cursor)?),
            Token::Semi => {
                cursor.bump_token()?;
                Nothing
            }
            Token::Return => Expr::Return(ReturnExpr::parse(cursor)?),
            _ => unimplemented!(),
        };

        // AssignExpr -> Expr AssignOp Expr
        // (Associativity: right to left)
        if let Some(assign_op) = cursor.eat_assign_op() {
            let rhs = Expr::parse(cursor)?;
            return Ok(Assign(AssignExpr::new(lhs, assign_op, rhs)));
        }

        Ok(lhs)
    }
}

/// PathExpr -> identifier (:: identifier)*
/// # Examples
/// `a::b::c`, `a`
impl<'a> Parse<'a> for PathExpr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        #[derive(PartialEq)]
        enum State {
            Init,
            PathSep,
            Segment,
        }

        let mut path_expr = Self::new();
        let mut state = State::Init;
        while let Ok(tk) = cursor.next_token() {
            match tk {
                Token::PathSep => {
                    if state == State::PathSep || state == State::Init {
                        return Err("invalid path".into());
                    }
                    state = State::PathSep;
                }
                Token::Identifier(s) => {
                    if state == State::Segment {
                        return Err("invalid path".into());
                    }
                    state = State::Segment;
                    path_expr.segments.push(s.to_string());
                }
                _ => break,
            }
            cursor.bump_token()?;
        }
        if state == State::Segment {
            Ok(path_expr)
        } else {
            Err("invalid path".into())
        }
    }
}

/// LitExpr -> literal
impl<'a> Parse<'a> for LitExpr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        let (literal_kind, value) = cursor.eat_literal()?;
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
                    Float { suffix } => {
                        if suffix.is_empty() {
                            "f64"
                        } else {
                            suffix
                        }
                    }
                }
                .into()
            },
            value,
        })
    }
}

/// UnAryExpr -> (`!` | `*` | `-`) Expr
impl<'a> Parse<'a> for UnAryExpr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        let tk = cursor.eat_token_in(&[Token::Not, Token::Star, Token::Minus])?;
        let op = UnOp::from_token(tk).unwrap();
        let expr = Expr::parse(cursor)?;
        Ok(UnAryExpr {
            op,
            expr: Box::new(expr),
        })
    }
}

/// BlockExpr -> `{` Expr* `}`
impl<'a> Parse<'a> for BlockExpr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        cursor.eat_token(Token::LeftCurlyBraces)?;
        let mut block_expr = BlockExpr { exprs: vec![] };
        while cursor.next_token()? != &Token::RightCurlyBraces {
            block_expr.exprs.push(Expr::parse(cursor)?);
        }
        cursor.eat_token(Token::RightCurlyBraces)?;
        Ok(block_expr)
    }
}

/// BorrowExpr -> `&`+ `mut`? Expr
impl<'a> Parse<'a> for BorrowExpr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        let mut borrow_cnt = 0u32;
        loop {
            match cursor.next_token()? {
                Token::And => {
                    cursor.bump_token()?;
                    borrow_cnt += 1;
                }
                Token::AndAnd => {
                    cursor.bump_token()?;
                    borrow_cnt += 2;
                }
                _ => break,
            }
        }

        let is_mut = cursor.eat_token_if(Token::Mut);
        let expr = Expr::parse(cursor)?;
        Ok(BorrowExpr {
            borrow_cnt,
            is_mut,
            expr: Box::new(expr),
        })
    }
}

impl<'a> Parse<'a> for BinOpExpr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        unimplemented!()
    }
}

/// ReturnExpr -> `return` Expr
impl<'a> Parse<'a> for ReturnExpr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        cursor.eat_token(Token::Return)?;
        let expr = Expr::parse(cursor)?;
        Ok(ReturnExpr(Box::new(expr)))
    }
}
