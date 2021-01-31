use crate::ast::TokenStart;
use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Identifier(IdentPattern),
}

impl TokenStart for Pattern {
    fn is_token_start(tk: &Token) -> bool {
        IdentPattern::is_token_start(tk)
    }
}

#[derive(Debug, PartialEq)]
pub struct IdentPattern {
    ident: String,
    is_mut: bool,
}

impl IdentPattern {
    pub fn new_mut(ident: String) -> Self {
        IdentPattern {
            ident,
            is_mut: true
        }
    }

    pub fn new_const(ident: String) -> Self {
        IdentPattern {
            ident,
            is_mut: false
        }
    }

    pub fn is_mut(&self) -> bool {
        self.is_mut
    }

    pub fn ident(&self) -> &str {
        &self.ident
    }
}

impl TokenStart for IdentPattern {
    fn is_token_start(tk: &Token) -> bool {
        matches!(tk, Token::Identifier(_)) 
    }
}