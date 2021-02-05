use crate::ast::expr::Expr::{Block, For, If, Loop, While};
use crate::ast::expr::{BlockExpr, Expr, IfExpr, LoopExpr, WhileExpr};
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

impl Expr {
    pub fn parse_with_block(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        debug_assert!(Self::is_with_block_token_start(cursor.next_token()?));
        match cursor.next_token()? {
            Token::LeftCurlyBraces => Ok(Block(BlockExpr::parse(cursor)?)),
            Token::While => Ok(While(WhileExpr::parse(cursor)?)),
            Token::Loop => Ok(Loop(LoopExpr::parse(cursor)?)),
            Token::For => todo!("parse for expr"),
            Token::If => Ok(If(IfExpr::parse(cursor)?)),
            Token::Match => todo!("parse match expr"),
            _ => unreachable!(),
        }
    }
}

/// Stmt -> Semi
///       | LetStmt
///       | Item
///       | ExprStmt
pub(super) fn parse_stmt_or_expr_without_block(
    cursor: &mut ParseCursor,
) -> Result<StmtOrExpr, RccError> {
    Ok(StmtOrExpr::Stmt(match cursor.next_token()? {
        Token::Semi => {
            cursor.bump_token()?;
            Stmt::Semi
        }
        Token::Let => Stmt::Let(LetStmt::parse(cursor)?),
        tk if Item::is_token_start(tk) => Stmt::Item(Item::parse(cursor)?),
        tk if Expr::is_with_block_token_start(tk) => {
            Stmt::ExprStmt(Expr::parse_with_block(cursor)?)
        }
        tk if Expr::is_token_start(tk) => {
            let expr = Expr::parse(cursor)?;
            debug_assert!(!expr.with_block());
            if !cursor.eat_token_if_eq(Token::Semi) {
                return Ok(StmtOrExpr::Expr(expr));
            }
            Stmt::ExprStmt(expr)
        }
        tk => unimplemented!("{}", tk),
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
