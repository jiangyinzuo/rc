use crate::ast::expr::Expr;
use crate::ast::item::Item;
use crate::ast::pattern::Pattern;
use crate::ast::stmt::{LetStmt, Stmt};
use crate::ast::types::TypeAnnotation;
use crate::ast::TokenStart;
use crate::lexer::token::Token;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;

#[derive(Debug, PartialEq)]
pub(super) enum StmtOrExpr {
    Stmt(Stmt),
    Expr(Expr),
}

/// Stmt -> Semi
///       | LetStmt
///       | Item
///       | ExprStmt
pub(super) fn parse_stmt_or_expr_without_block(cursor: &mut ParseCursor) -> Result<StmtOrExpr, RccError> {
    Ok(StmtOrExpr::Stmt(match cursor.next_token()? {
        Token::Semi => {
            cursor.bump_token()?;
            Stmt::Semi
        }
        Token::Let => Stmt::Let(LetStmt::parse(cursor)?),
        tk if Item::is_token_start(tk) => Stmt::Item(Item::parse(cursor)?),
        tk if Expr::is_token_start(tk) => {
            let expr = Expr::parse(cursor)?;
            if !expr.with_block() && !cursor.eat_token_if_eq(Token::Semi) {
                return Ok(StmtOrExpr::Expr(expr));
            }
            Stmt::ExprStmt(expr)
        }
        _ => unimplemented!(),
    }))
}

/// LetStmt -> `let` Pattern (: TypeAnnotation)? ( = Expr)? ;
impl Parse for LetStmt {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        cursor.eat_token_eq(Token::Let)?;
        let pattern = Pattern::parse(cursor)?;
        let mut let_stmt = LetStmt::new(pattern);
        if cursor.eat_token_if_eq(Token::Colon) {
            let_stmt = let_stmt._type(TypeAnnotation::parse(cursor)?);
        }
        if cursor.eat_token_if_eq(Token::Eq) {
            let_stmt = let_stmt.expr(Expr::parse(cursor)?);
        }
        cursor.eat_token_eq(Token::Semi)?;
        Ok(let_stmt)
    }
}
