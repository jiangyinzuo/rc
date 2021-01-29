use crate::ast::expr::Expr;
use crate::ast::item::VisItem;
use crate::ast::pattern::Pattern;
use crate::ast::stmt::{LetStmt, Stmt};
use crate::ast::types::Type;
use crate::ast::TokenStart;
use crate::lexer::token::Token;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;

/// Stmt -> Semi
///       | LetStmt
///       | Item
///       | ExprStmt
impl Parse for Stmt {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        Ok(match cursor.next_token()? {
            Token::Semi => {
                cursor.bump_token()?;
                Self::Semi
            }
            Token::Let => Self::Let(LetStmt::parse(cursor)?),
            tk if VisItem::is_token_start(tk) => Self::Item(VisItem::parse(cursor)?),
            tk if Expr::is_token_start(tk) => {
                let expr = Expr::parse(cursor)?;
                if !expr.with_block() {
                    cursor.eat_token_eq(Token::Semi)?;
                }
                Self::ExprStmt(expr)
            },
            _ => unimplemented!(),
        })
    }
}

/// LetStmt -> `let` Pattern (: Type)? ( = Expr)? ;
impl Parse for LetStmt {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        cursor.eat_token_eq(Token::Let)?;
        let pattern = Pattern::parse(cursor)?;
        let mut let_stmt = LetStmt::new(pattern);
        if cursor.eat_token_if_eq(Token::Colon) {
            let_stmt = let_stmt._type(Type::parse(cursor)?);
        }
        if cursor.eat_token_if_eq(Token::Eq) {
            let_stmt = let_stmt.expr(Expr::parse(cursor)?);
        }
        cursor.eat_token_eq(Token::Semi)?;
        Ok(let_stmt)
    }
}
