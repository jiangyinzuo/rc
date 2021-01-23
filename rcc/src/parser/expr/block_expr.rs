use crate::lexer::token::Token::{LeftCurlyBraces, RightCurlyBraces};

use crate::ast::expr::BlockExpr;
use crate::ast::expr::Expr;
use crate::parser::Parse;
use crate::parser::ParseCursor;
use crate::rcc::RccError;

impl<'a> Parse<'a> for BlockExpr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        cursor.eat_token(LeftCurlyBraces)?;
        let mut block_expr = BlockExpr { exprs: vec![] };
        while cursor.next_token()? != &RightCurlyBraces {
            block_expr.exprs.push(Expr::parse(cursor)?);
        }
        cursor.eat_token(RightCurlyBraces)?;
        Ok(block_expr)
    }
}
