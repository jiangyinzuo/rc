use crate::ast::pattern::{IdentifierPattern, Pattern};
use crate::lexer::token::Token;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;

/// Pattern -> IdentifierPattern
///
/// IdentifierPattern -> `mut`? identifier
impl Parse for Pattern {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        match cursor.next_token()? {
            Token::Mut => {
                cursor.bump_token()?;
                if let Token::Identifier(s) = cursor.bump_token()? {
                    Ok(Self::Identifier(IdentifierPattern::new_mut(
                        s.to_string(),
                    )))
                } else {
                    Err("expect identifier".into())
                }
            }
            Token::Identifier(s) => {
                let s = s.to_string();
                cursor.bump_token()?;
                Ok(Self::Identifier(IdentifierPattern::new_const(s)))
            }
            _ => Err("invalid pattern".into()),
        }
    }
}
