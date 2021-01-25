use crate::ast::expr::Expr::{Assign, Block, Borrow, Lit, Path, Range, Unary};
use crate::ast::expr::*;
use crate::ast::stmt::Stmt;
use crate::lexer::token::LiteralKind::*;
use crate::lexer::token::Token;
use crate::lexer::token::Token::Unknown;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;

impl Parse for Expr {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        range_expr(cursor)
    }
}

/// RangeExpr -> AssignExpr
///            | AssignExpr? RangeOp AssignExpr?
/// (Associativity: Require parentheses)
fn range_expr(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
    let mut lhs = assign_expr(cursor)?;

    if let Some(range_op) = cursor.eat_if_from_token() {
        if let Ok(_assign_expr) = assign_expr(cursor) {
            if cursor.next_token().unwrap_or(&Unknown).is_range_op() {
                return Err("range operators require parentheses".into());
            }
            lhs = Range(RangeExpr::new(range_op).lhs(lhs).rhs(_assign_expr));
        } else {
            lhs = Range(RangeExpr::new(range_op).lhs(lhs));
        }
    }
    Ok(lhs)
}

/// AssignExpr -> PrimitiveExpr
///             | PrimitiveExpr AssignOp AssignExpr
/// (Associativity: right to left)
fn assign_expr(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
    let mut lhs = primitive_expr(cursor)?;

    if let Some(assign_op) = cursor.eat_if_from_token() {
        let rhs = Expr::parse(cursor)?;
        lhs = Assign(AssignExpr::new(lhs, assign_op, rhs));
    }
    Ok(lhs)
}

fn primitive_expr(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
    let expr = match cursor.next_token()? {
        Token::Not | Token::Star | Token::Minus => Unary(UnAryExpr::parse(cursor)?),
        Token::Identifier(_) | Token::PathSep => Path(PathExpr::parse(cursor)?),
        Token::Literal { .. } => Lit(LitExpr::parse(cursor)?),
        Token::LeftCurlyBraces => Block(BlockExpr::parse(cursor)?),
        Token::LeftParen => parse_left_paren(cursor)?,
        Token::And | Token::AndAnd => Borrow(BorrowExpr::parse(cursor)?),
        Token::DotDot | Token::DotDotEq => Range(RangeExpr::parse_without_lhs(cursor)?),
        Token::Return => Expr::Return(ReturnExpr::parse(cursor)?),
        _ => unreachable!(),
    };
    // TODO: call expr
    Ok(expr)
}

/// CallExpr ->
///    Expr ( CallParams? )
///
/// CallParams ->
///    Expr ( , Expression )* ,?
impl CallExpr {
    fn parse_call_expr(cursor: &mut ParseCursor, expr: Expr) -> Result<Expr, RccError> {
        let call_expr = Self::new(expr);
        todo!()
    }
}

fn parse_left_paren(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
    cursor.eat_token(Token::LeftParen)?;
    let expr = Expr::parse(cursor)?;
    match cursor.next_token()? {
        Token::RightParen => {
            cursor.bump_token()?;
            Ok(Expr::Grouped(GroupedExpr::new(expr)))
        }
        Token::Comma => Ok(Expr::Tuple(TupleExpr::parse_from_second(cursor, expr)?)),
        _ => Err("expected ',' or ')'".into()),
    }
}

///  TupleExpr -> `(` ( Expr , )+ Expr? `)`
impl TupleExpr {
    fn parse_from_second(cursor: &mut ParseCursor, first_expr: Expr) -> Result<Self, RccError> {
        cursor.eat_token(Token::Comma)?;
        let mut tuple_expr = TupleExpr(vec![first_expr]);
        while cursor.next_token()? != &Token::RightParen {
            tuple_expr.0.push(Expr::parse(cursor)?);
            if !cursor.eat_token_if(Token::Comma) {
                break;
            }
        }
        if cursor.eat_token_if(Token::RightParen) {
            Ok(tuple_expr)
        } else {
            Err("invalid tuple expression".into())
        }
    }
}

impl RangeExpr {
    pub fn parse_without_lhs(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        if let Some(op) = cursor.eat_if_from_token::<RangeOp>() {
            let _assign_expr = assign_expr(cursor)?;
            Ok(Self::new(op).rhs(_assign_expr))
        } else {
            Err("expect '..' or '..='".into())
        }
    }
}

/// PathExpr -> identifier (:: identifier)*
/// # Examples
/// `a::b::c`, `a`
impl Parse for PathExpr {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
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
impl Parse for LitExpr {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        let (literal_kind, value) = cursor.eat_literal()?;
        Ok(LitExpr {
            ret_type: {
                match literal_kind {
                    Char => "char",
                    String => "&str",
                    Integer { suffix } => {
                        if suffix.is_empty() {
                            Self::EMPTY_INT_TYPE
                        } else {
                            suffix
                        }
                    },
                    Float { suffix } => {
                        if suffix.is_empty() {
                            Self::EMPTY_FLOAT_TYPE
                        } else {
                            suffix
                        }
                    },
                }
                .into()
            },
            value,
        })
    }
}

/// UnAryExpr -> (`!` | `*` | `-`) Expr
impl Parse for UnAryExpr {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
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
impl Parse for BlockExpr {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        cursor.eat_token(Token::LeftCurlyBraces)?;
        let mut block_expr = BlockExpr { stmts: vec![] };
        while cursor.next_token()? != &Token::RightCurlyBraces {
            block_expr.stmts.push(Stmt::parse(cursor)?);
        }
        cursor.eat_token(Token::RightCurlyBraces)?;
        Ok(block_expr)
    }
}

/// BorrowExpr -> `&`+ `mut`? Expr
impl Parse for BorrowExpr {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
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

impl Parse for BinOpExpr {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        unimplemented!()
    }
}

/// ReturnExpr -> `return` Expr
impl Parse for ReturnExpr {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        cursor.eat_token(Token::Return)?;
        let expr = Expr::parse(cursor)?;
        Ok(ReturnExpr(Box::new(expr)))
    }
}
