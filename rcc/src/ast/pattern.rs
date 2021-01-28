use crate::ast::TokenStart;
use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Identifier(IdentifierPattern),
}

impl TokenStart for Pattern {
    fn is_token_start(tk: &Token) -> bool {
        IdentifierPattern::is_token_start(tk)
    }
}

#[derive(Debug, PartialEq)]
pub struct IdentifierPattern {
    ident: String,
    is_mut: bool,
}

impl IdentifierPattern {
    pub fn new_mut(ident: String) -> Self {
        IdentifierPattern {
            ident,
            is_mut: true
        }
    }

    pub fn new_const(ident: String) -> Self {
        IdentifierPattern {
            ident,
            is_mut: false
        }
    }
}

impl TokenStart for IdentifierPattern {
    fn is_token_start(tk: &Token) -> bool {
        matches!(tk, Token::Identifier(_)) 
    }
}