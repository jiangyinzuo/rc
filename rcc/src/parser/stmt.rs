use crate::ast::stmt::Stmt;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;
use crate::lexer::token::Token;
use crate::ast::item::VisItem;
use crate::ast::TokenStart;
use crate::ast::expr::Expr;

impl Parse for Stmt  {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        Ok(match cursor.next_token()? {
            Token::Semi => {
                cursor.bump_token()?;
                Self::Semi
            },
            Token::Let => unimplemented!(),
            tk if VisItem::is_token_start(tk) => Self::Item(VisItem::parse(cursor)?),
            tk if Expr::is_token_start(tk) => Self::ExprStmt(Expr::parse(cursor)?),
            _ => unimplemented!()
        })
    }
}